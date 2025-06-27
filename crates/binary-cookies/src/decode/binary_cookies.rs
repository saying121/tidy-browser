use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{meta::MetaOffset, pages::PagesOffset, DecodeResult};
use crate::{
    cookie::BinaryCookies,
    decode::StreamIn,
    error::{ParseError, Result},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BinaryCookieFsm {
    pub(crate) buffer: Buffer,
}

impl Default for BinaryCookieFsm {
    fn default() -> Self {
        Self::new()
    }
}

// binary cookies metadata
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Offsets {
    pub(crate) page_sizes: Vec<u32>, // be
    pub(crate) tail_offset: u64,
}

impl BinaryCookieFsm {
    /// 4(`magic`) + 4(`num_pages`) + 4 * `num_pages`
    /// assume `num_pages` = 2 round up to a power of 2
    const BUF_SIZE: usize = 4 + 4 + 4 * 2;

    pub fn new() -> Self {
        Self {
            buffer: Buffer::with_capacity(Self::BUF_SIZE),
        }
    }

    pub fn process(mut self) -> Result<DecodeResult<Self, (MetaOffset, PagesOffset)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let e = match BinaryCookies::decode_head.parse_next(&mut input) {
            Ok(offsets) => {
                return Ok(DecodeResult::Done((
                    offsets.tail_offset,
                    PagesOffset::new(offsets.page_sizes),
                )));
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
