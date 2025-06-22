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
    pub(crate) offsets: Vec<OffsetSize>,
}

impl CookiesOffset {
    pub(crate) fn new(page_offset: u64, cookies_size: Vec<u32>) -> Self {
        let mut offset = page_offset + 4 + 4 + 4 * cookies_size.len() as u64 + 4;
        let offsets = cookies_size
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
pub struct CookieDecoder {
    offsets: OffsetSize,
    buffer: Buffer,
}

impl CookieDecoder {
    /// Just cookie size
    const BUF_SIZE: usize = 4;

    pub fn new(offsets: OffsetSize) -> Self {
        let buffer = Buffer::with_capacity(Self::BUF_SIZE);
        Self { offsets, buffer }
    }

    pub fn wants_read(&self) -> u64 {
        self.offsets.offset + self.buffer.available_data() as u64
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
