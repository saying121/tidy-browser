use std::io::Read;

use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{DecodeResult, OffsetSize};
use crate::{
    cookie::Cookie,
    cursor::CookieCursor,
    decode::StreamIn,
    error::{ParseError, Result},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookiesOffset {
    pub(crate) offset_sizes: Vec<OffsetSize>,
}

impl CookiesOffset {
    pub(crate) fn new(page_offset: u64, page_size: u32, cookies_offset: &[u32]) -> Self {
        let mut prev_cookie_end = page_size;

        let offset_sizes = cookies_offset
            .iter()
            .rev()
            .map(|&offset_in_page| {
                let res = OffsetSize {
                    offset: offset_in_page as u64 + page_offset,
                    size: prev_cookie_end - offset_in_page,
                };
                prev_cookie_end = offset_in_page;
                res
            })
            .collect();
        Self { offset_sizes }
    }

    pub fn offset_sizes(&self) -> &[OffsetSize] {
        &self.offset_sizes
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct CookieFsm {
    buffer: Buffer,
}

impl CookieFsm {
    /// Just cookie size
    const BUF_SIZE: usize = 4;

    pub fn new() -> Self {
        let buffer = Buffer::with_capacity(Self::BUF_SIZE);
        Self { buffer }
    }

    pub fn with_capacity(size: usize) -> Self {
        let buffer = Buffer::with_capacity(size);
        Self { buffer }
    }

    pub fn process(mut self) -> Result<DecodeResult<Self, Cookie>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());

        let e = match Cookie::parse.parse_next(&mut input) {
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

impl Default for CookieFsm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookieHandle<'a, F: CookieCursor> {
    file: &'a F,
    offset_size: CookiesOffset,
}

impl<'a, F: CookieCursor> CookieHandle<'a, F> {
    pub const fn new(file: &'a F, offset_size: CookiesOffset) -> Self {
        Self { file, offset_size }
    }

    pub fn decoders(&self) -> impl Iterator<Item = CookieDecoder<<F as CookieCursor>::Cursor<'_>>> {
        self.offset_size
            .offset_sizes
            .iter()
            .map(|&OffsetSize { offset, size }| CookieDecoder {
                rd: self.file.cursor_at(offset),
                size,
            })
    }

    pub fn into_decoders(
        self,
    ) -> impl Iterator<Item = CookieDecoder<<F as CookieCursor>::Cursor<'a>>> {
        self.offset_size
            .offset_sizes
            .into_iter()
            .map(|OffsetSize { offset, size }| CookieDecoder {
                rd: self.file.cursor_at(offset),
                size,
            })
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookieDecoder<R: Read> {
    rd: R,
    /// The cookie size
    size: u32,
}

impl<R: Read> CookieDecoder<R> {
    pub fn decode(&mut self) -> Result<Cookie> {
        let mut fsm = CookieFsm::with_capacity(self.size as usize);
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
                DecodeResult::Done(r) => return Ok(r),
            }
        }
    }
}
