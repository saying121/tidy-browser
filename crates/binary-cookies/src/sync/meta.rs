use std::io::Read;

use snafu::ResultExt;

use crate::{
    cookie::{Checksum, Metadata},
    decode::{meta::MetaFsm, DecodeResult},
    error::{self, Result},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaDecoder<R: Read> {
    pub(crate) rd: R,
}

impl<R: Read> MetaDecoder<R> {
    pub fn decode(&mut self) -> Result<(Checksum, Option<Metadata>)> {
        let mut fsm = MetaFsm::new();
        loop {
            self.rd
                .read_exact(fsm.buffer.space())
                .context(error::ReadSnafu)?;
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
