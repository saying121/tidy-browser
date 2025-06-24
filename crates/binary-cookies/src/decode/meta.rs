use std::io::Read;

use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser as _,
};

use super::{DecodeResult, StreamIn};
use crate::{
    cookie::{BinaryCookies, Checksum, Metadata},
    error::{ParseError, Result},
};

/// The meta relative file start offset
pub type MetaOffset = u64;

#[derive(Clone)]
#[derive(Debug)]
pub struct MetaFsm {
    buffer: Buffer,
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
        let e = match BinaryCookies::parse_tail.parse_next(&mut input) {
            Ok((checksum, meta)) => {
                return Ok(DecodeResult::Done((checksum, meta)));
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

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaDecoder<R: Read> {
    pub(crate) rd: R,
}

impl Default for MetaFsm {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Read> MetaDecoder<R> {
    pub fn decode(&mut self) -> Result<(Checksum, Option<Metadata>)> {
        let mut fsm = MetaFsm::new();
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
                DecodeResult::Done(done) => return Ok(done),
            }
        }
    }
}
