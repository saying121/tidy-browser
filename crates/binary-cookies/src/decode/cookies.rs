use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{DecodeResult, OffsetSize};
use crate::{
    cookie::Cookie,
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
    // NOTE: The offset_sizes order is reverse
    pub(crate) fn new(page_offset: u64, page_size: u32, cookies_offset: &[u32]) -> Self {
        let mut prev_cookie_end = page_size;

        let offset_sizes: Vec<OffsetSize> = cookies_offset
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
}

#[derive(Clone)]
#[derive(Debug)]
pub struct CookieFsm {
    pub(crate) buffer: Buffer,
}

impl Default for CookieFsm {
    fn default() -> Self {
        Self::new()
    }
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

        let e = match Cookie::decode.parse_next(&mut input) {
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
