use snafu::ResultExt;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::cursor::CookieCursor;
use crate::{
    cookie::Cookie,
    decode::{
        cookies::{CookieFsm, CookiesOffset},
        DecodeResult, OffsetSize,
    },
    error::{self, Result},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookieHandle<'a, F: CookieCursor> {
    file: &'a F,
    offset_size: CookiesOffset,
}

impl<'a, F: CookieCursor> CookieHandle<'a, F> {
    pub const fn new(file: &'a F, offset_size: CookiesOffset) -> Self {
        Self { file, offset_size }
    }

    pub fn decoders(&self) -> impl Iterator<Item = CookieDecoder<<F as CookieCursor>::Cursor<'_>>> {
        self.offset_size
            .offset_sizes
            .iter()
            .rev() // put it in order
            .map(|&OffsetSize { offset, size }| CookieDecoder {
                rd: self.file.cursor_at(offset),
                size,
            })
    }

    pub fn into_decoders(
        self,
    ) -> impl Iterator<Item = CookieDecoder<<F as CookieCursor>::Cursor<'a>>> {
        self.offset_size
            .offset_sizes
            .into_iter()
            .rev() // put it in order
            .map(|OffsetSize { offset, size }| CookieDecoder {
                rd: self.file.cursor_at(offset),
                size,
            })
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookieDecoder<R: AsyncRead> {
    pub(crate) rd: R,
    // The cookie size
    pub(crate) size: u32,
}

impl<R: AsyncRead + Unpin + Send> CookieDecoder<R> {
    pub async fn decode(&mut self) -> Result<Cookie> {
        let mut fsm = CookieFsm::with_capacity(self.size as usize);
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
                DecodeResult::Done((r, _)) => return Ok(r),
            }
        }
    }
}
