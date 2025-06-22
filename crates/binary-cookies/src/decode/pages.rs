use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{cookies::CookiesOffset, DecodeResult, OffsetSize};
use crate::{
    cookie::Page,
    decode::StreamIn,
    error::{ParseError, Result},
};

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PagesOffset {
    pub(crate) offsets: Vec<OffsetSize>,
}

impl PagesOffset {
    pub(crate) fn new(page_sizes: Vec<u32>) -> Self {
        let head_len = 4 * 2 + page_sizes.len() as u64 * 4;
        let mut offset = head_len;
        let offsets = page_sizes
            .into_iter()
            .map(|size| {
                let prev = offset;
                offset += size as u64;
                OffsetSize { offset: prev, size }
            })
            .collect();
        Self { offsets }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct PageDecoder {
    offset: u64,
    buffer: Buffer,
}

impl PageDecoder {
    /// 4(`header`) + 4(`num_cookies`) + 4 * `num_cookies`
    /// assume `num_cookies` = 3 round up to a power of 2
    const BUF_SIZE: usize = 4 + 4 + 4 * 2;

    pub fn new(offset: u64) -> Self {
        let buffer = Buffer::with_capacity(Self::BUF_SIZE);
        Self { offset, buffer }
    }

    /// return read offset
    pub fn wants_read(&self) -> usize {
        self.offset as usize + self.buffer.available_data()
    }

    pub fn process(mut self) -> Result<DecodeResult<Self, CookiesOffset>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let e = match Page::parse_head.parse_next(&mut input) {
            Ok(o) => {
                let page_offset = self.offset;
                return Ok(DecodeResult::Done(CookiesOffset::new(page_offset, o)));
            },
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
