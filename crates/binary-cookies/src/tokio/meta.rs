use snafu::ResultExt;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{
    cookie::{Checksum, Metadata},
    decode::{meta::MetaFsm, DecodeResult},
    error::{self, Result},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaDecoder<R: AsyncRead> {
    pub(crate) rd: R,
}

impl<R: AsyncRead + Unpin + Send> MetaDecoder<R> {
    pub async fn decode(&mut self) -> Result<(Checksum, Option<Metadata>)> {
        let mut fsm = MetaFsm::new();
        loop {
            self.rd
                .read_exact(fsm.buffer.space())
                .await
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
