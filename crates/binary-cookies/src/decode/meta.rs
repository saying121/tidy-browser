use oval::Buffer;
use winnow::Parser;

use super::{DecodeResult, StreamIn};
use crate::{
    cookie::{BinaryCookies, Checksum, Metadata},
    error::Result,
};

/// The meta relative file start offset
pub type MetaOffset = u64;

#[derive(Clone)]
#[derive(Debug)]
pub struct MetaFsm {
    pub(crate) buffer: Buffer,
}

impl Default for MetaFsm {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaFsm {
    const BUF_SIZE: usize = 4 + 8 + 75;

    pub fn new() -> Self {
        Self {
            buffer: Buffer::with_capacity(Self::BUF_SIZE),
        }
    }

    pub fn process(mut self) -> Result<DecodeResult<Self, (Checksum, Option<Metadata>)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());

        let e = match BinaryCookies::decode_tail.parse_next(&mut input) {
            Ok((checksum, meta)) => {
                return Ok(DecodeResult::Done((checksum, meta)));
            },
            Err(e) => e,
        };
        mode_err! {self, e, self.buffer}
    }
}
