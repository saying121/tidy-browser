use std::ops::Deref;

use oval::Buffer;
use winnow::{
    stream::{Offset, Stream},
    Parser,
};

use super::{DecodeResult, OffsetSize};
use crate::{cookie::Cookie, decode::StreamIn, error::Result};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookiesOffsetInPage(pub(crate) Vec<u32>);

impl Deref for CookiesOffsetInPage {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookiesOffset {
    pub(crate) offset_sizes: Vec<OffsetSize>,
}

impl CookiesOffset {
    #[cfg(any(feature = "sync", feature = "tokio"))]
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

    pub fn process(mut self) -> Result<DecodeResult<Self, (Cookie, Buffer)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let start = input.checkpoint();

        let e = match Cookie::decode.parse_next(&mut input) {
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
