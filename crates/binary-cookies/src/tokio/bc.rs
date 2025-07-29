//! binarycookies mod

use snafu::ResultExt;
use tokio::io::AsyncReadExt;

use super::{cursor::CookieCursor, meta::MetaDecoder};
use crate::{
    decode::{binary_cookies::BinaryCookieFsm, meta::MetaOffset, pages::PagesOffset, DecodeResult},
    error::{self, Result},
    tokio::page::PagesHandle,
};

pub trait DecodeBinaryCookie {
    type File: CookieCursor;

    #[expect(async_fn_in_trait, reason = "allow")]
    async fn decode(&self) -> Result<BinaryCookiesHandle<'_, Self::File>>;
}

impl<F> DecodeBinaryCookie for F
where
    F: CookieCursor,
{
    type File = F;

    #[expect(clippy::future_not_send, reason = "allow")]
    async fn decode(&self) -> Result<BinaryCookiesHandle<'_, F>> {
        let mut fsm = BinaryCookieFsm::new();
        let mut rd = self.cursor_at(0);
        loop {
            rd.read_exact(fsm.buffer.space())
                .await
                .context(error::ReadSnafu)?;
            let count = fsm.buffer.available_space();
            fsm.buffer.fill(count);
            match fsm.process()? {
                DecodeResult::Continue(fsm_) => {
                    fsm = fsm_;
                    continue;
                },
                DecodeResult::Done((meta_offset, pages_offset, _)) => {
                    return Ok(BinaryCookiesHandle {
                        file: self,
                        meta_offset,
                        pages_offset,
                    })
                },
            }
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct BinaryCookiesHandle<'a, F: CookieCursor> {
    pub(crate) file: &'a F,
    pub(crate) meta_offset: MetaOffset,
    pub(crate) pages_offset: PagesOffset,
}

impl<'a, F: CookieCursor> BinaryCookiesHandle<'a, F> {
    pub fn into_handles(
        self,
    ) -> (
        PagesHandle<'a, F>,
        MetaDecoder<<F as CookieCursor>::Cursor<'a>>,
    ) {
        let pages_handle = PagesHandle {
            file: self.file,
            offset: self.pages_offset,
        };
        let meta_handle = MetaDecoder {
            rd: self
                .file
                .cursor_at(self.meta_offset),
        };
        (pages_handle, meta_handle)
    }
}
