use oval::Buffer;
use winnow::{
    stream::{Offset, Stream},
    Parser,
};

use super::{meta::MetaOffset, pages::PagesOffset, DecodeResult};
use crate::{cookie::BinaryCookies, decode::StreamIn, error::Result};

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

    pub fn process(mut self) -> Result<DecodeResult<Self, (MetaOffset, PagesOffset, Buffer)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let start = input.checkpoint();

        let e = match BinaryCookies::decode_head.parse_next(&mut input) {
            Ok(offsets) => {
                let consumed = input.offset_from(&start);
                self.buffer.consume(consumed);
                return Ok(DecodeResult::Done((
                    offsets.tail_offset,
                    PagesOffset::new(offsets.page_sizes),
                    self.buffer,
                )));
            },
            Err(e) => e,
        };
        mode_err! {self, e, self.buffer}
    }
}
