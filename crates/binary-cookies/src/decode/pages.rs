use std::io::Read;

use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{
    cookies::{CookieHandle, CookiesOffset},
    DecodeResult, OffsetSize,
};
use crate::{
    cookie::Page,
    cursor::CookieCursor,
    decode::StreamIn,
    error::{ParseError, Result},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PagesOffset {
    pub(crate) offset_sizes: Vec<OffsetSize>,
}

impl PagesOffset {
    pub(crate) fn new(page_sizes: Vec<u32>) -> Self {
        let head_len = 4 * 2 + page_sizes.len() as u64 * 4;
        let mut offset = head_len;
        let offset_sizes = page_sizes
            .into_iter()
            .map(|size| {
                let prev = offset;
                offset += size as u64;
                OffsetSize { offset: prev, size }
            })
            .collect();
        Self { offset_sizes }
    }

    /// Returns the offset and size of each page in binarycookies
    pub fn offset_sizes(&self) -> &[OffsetSize] {
        &self.offset_sizes
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct PageFsm {
    buffer: Buffer,
}

impl PageFsm {
    /// 4(`header`) + 4(`num_cookies`) + 4 * `num_cookies`
    /// assume `num_cookies` = 3 round up to a power of 2
    const BUF_SIZE: usize = 4 + 4 + 4 * 2;

    pub fn new() -> Self {
        let buffer = Buffer::with_capacity(Self::BUF_SIZE);
        Self { buffer }
    }

    pub fn with_capacity(size: usize) -> Self {
        let buffer = Buffer::with_capacity(size);
        Self { buffer }
    }

    pub fn process(mut self) -> Result<DecodeResult<Self, Vec<u32>>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let e = match Page::parse_head.parse_next(&mut input) {
            Ok(o) => return Ok(DecodeResult::Done(o)),
            Err(e) => e,
        };

        match e {
            ErrMode::Backtrack(e) | ErrMode::Cut(e) => Err(ParseError::WinnowCtx(e)),
            ErrMode::Incomplete(Needed::Unknown) => {
                // The branch is unreachable?
                let new_cap = self.buffer.capacity() * 2;
                self.buffer.grow(new_cap);
                Ok(DecodeResult::Continue(self))
            },
            ErrMode::Incomplete(Needed::Size(size)) => {
                let need_size = size.get();
                self.buffer
                    .grow(self.buffer.available_data() + need_size);

                if self.buffer.available_space() < need_size {
                    self.buffer.shift();
                }
                Ok(DecodeResult::Continue(self))
            },
        }
    }
}

impl Default for PageFsm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct PagesHandle<'a, F: CookieCursor> {
    pub(crate) file: &'a F,
    pub(crate) offset: PagesOffset,
}

impl<'a, F: CookieCursor> PagesHandle<'a, F> {
    pub fn decoders(
        &self,
    ) -> impl Iterator<Item = PageDecoder<'_, <F as CookieCursor>::Cursor<'_>, F>> {
        self.offset
            .offset_sizes
            .iter()
            .map(|&OffsetSize { offset, size }| PageDecoder {
                file: self.file,
                rd: self.file.cursor_at(offset),
                offset,
                size,
            })
    }

    pub fn into_decoders(
        self,
    ) -> impl Iterator<Item = PageDecoder<'a, <F as CookieCursor>::Cursor<'a>, F>> {
        self.offset
            .offset_sizes
            .into_iter()
            .map(|OffsetSize { offset, size }| PageDecoder {
                file: self.file,
                rd: self.file.cursor_at(offset),
                offset,
                size,
            })
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PageDecoder<'a, R: Read, F: CookieCursor> {
    file: &'a F,
    rd: R,
    offset: u64,
    size: u32,
}

impl<'a, R: Read, F: CookieCursor> PageDecoder<'a, R, F> {
    pub fn decode(&mut self) -> Result<CookieHandle<'a, F>> {
        let mut fsm = PageFsm::new();
        loop {
            self.rd
                .read_exact(fsm.buffer.space())?;
            let count = fsm.buffer.available_space();
            fsm.buffer.fill(count);
            match fsm.process()? {
                DecodeResult::Continue(fsm_) => {
                    fsm = fsm_;
                    continue;
                },
                DecodeResult::Done(cookie_offset_in_page) => {
                    let cookies_offset =
                        CookiesOffset::new(self.offset, self.size, &cookie_offset_in_page);
                    return Ok(CookieHandle::new(self.file, cookies_offset));
                },
            }
        }
    }
}
