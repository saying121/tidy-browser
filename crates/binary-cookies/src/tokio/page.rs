use snafu::ResultExt;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{cookie::CookieHandle, cursor::CookieCursor};
use crate::{
    decode::{
        cookies::CookiesOffset,
        pages::{PageFsm, PagesOffset},
        DecodeResult, OffsetSize,
    },
    error::{self, Result},
};

#[derive(Clone)]
#[derive(Debug)]
pub struct PagesHandle<'a, F: CookieCursor> {
    pub(crate) file: &'a F,
    pub(crate) offset: PagesOffset,
}

impl<'a, F: CookieCursor> PagesHandle<'a, F> {
    pub fn decoders(
        &self,
    ) -> impl Iterator<Item = PageDecoder<'_, <F as CookieCursor>::Cursor<'_>, F>> {
        self.offset
            .offset_sizes
            .iter()
            .map(|&OffsetSize { offset, size }| PageDecoder {
                file: self.file,
                rd: self.file.cursor_at(offset),
                offset,
                size,
            })
    }

    pub fn into_decoders(
        self,
    ) -> impl Iterator<Item = PageDecoder<'a, <F as CookieCursor>::Cursor<'a>, F>> {
        self.offset
            .offset_sizes
            .into_iter()
            .map(|OffsetSize { offset, size }| PageDecoder {
                file: self.file,
                rd: self.file.cursor_at(offset),
                offset,
                size,
            })
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PageDecoder<'a, R: AsyncRead, F: CookieCursor> {
    pub(crate) file: &'a F,
    pub(crate) rd: R,
    pub(crate) offset: u64,
    pub(crate) size: u32,
}

impl<'a, R: AsyncRead + Unpin + Send, F: CookieCursor + Sync> PageDecoder<'a, R, F> {
    pub async fn decode(&mut self) -> Result<CookieHandle<'a, F>> {
        let mut fsm = PageFsm::new();
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
                DecodeResult::Done((cookie_offset_in_page, _)) => {
                    let cookies_offset =
                        CookiesOffset::new(self.offset, self.size, &cookie_offset_in_page);
                    return Ok(CookieHandle::new(self.file, cookies_offset));
                },
            }
        }
    }
}
