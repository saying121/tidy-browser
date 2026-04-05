use oval::Buffer;
use winnow::{
    stream::{Offset, Stream},
    Parser,
};

use super::{cookies::CookiesOffsetInPage, DecodeResult, OffsetSize};
use crate::{cookie::Page, decode::StreamIn, error::Result};

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
    pub(crate) buffer: Buffer,
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

    pub fn process(mut self) -> Result<DecodeResult<Self, (CookiesOffsetInPage, Buffer)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let start = input.checkpoint();

        let e = match Page::decode_head.parse_next(&mut input) {
            Ok(o) => {
                let consumed = input.offset_from(&start);
                self.buffer.consume(consumed);
                return Ok(DecodeResult::Done((o, self.buffer)));
            },
            Err(e) => e,
        };

        mode_err! {self, e, self.buffer}
    }
}

impl Default for PageFsm {
    fn default() -> Self {
        Self::new()
    }
}
