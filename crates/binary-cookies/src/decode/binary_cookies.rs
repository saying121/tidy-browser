use std::io::Read;

use oval::Buffer;
use winnow::{
    error::{ErrMode, Needed},
    Parser,
};

use super::{
    meta::{MetaDecoder, MetaOffset},
    pages::PagesOffset,
    DecodeResult,
};
use crate::{
    cookie::BinaryCookies,
    cursor::CookieCursor,
    decode::{pages::PagesHandle, StreamIn},
    error::{ParseError, Result},
};

pub trait DecodeBinaryCookie {
    type File: CookieCursor;
    fn decode(&self) -> Result<BinaryCookiesHandle<'_, Self::File>>;
}

impl<F> DecodeBinaryCookie for F
where
    F: CookieCursor,
{
    type File = F;

    fn decode(&self) -> Result<BinaryCookiesHandle<'_, F>> {
        let mut fsm = BinaryCookieFsm::new();
        let mut rd = self.cursor_at(0);
        loop {
            rd.read_exact(fsm.buffer.space())?;
            let count = fsm.buffer.available_space();
            fsm.buffer.fill(count);
            match fsm.process()? {
                DecodeResult::Continue(fsm_) => {
                    fsm = fsm_;
                    continue;
                },
                DecodeResult::Done((meta_offset, pages_offset)) => {
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
#[derive(PartialEq)]
pub struct BinaryCookieFsm {
    buffer: Buffer,
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

    pub fn process(mut self) -> Result<DecodeResult<Self, (MetaOffset, PagesOffset)>> {
        let mut input: StreamIn = StreamIn::new(self.buffer.data());
        let e = match BinaryCookies::parse_head.parse_next(&mut input) {
            Ok(offsets) => {
                return Ok(DecodeResult::Done((
                    offsets.tail_offset,
                    PagesOffset::new(offsets.page_sizes),
                )));
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

impl Default for BinaryCookieFsm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct BinaryCookiesHandle<'a, F: CookieCursor> {
    file: &'a F,
    meta_offset: MetaOffset,
    pages_offset: PagesOffset,
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use crate::{cookie::Metadata, decode::OffsetSize};

    #[test]
    fn test_name() {
        let f = File::open("./test-resource/BinaryCookies.binarycookies").unwrap();
        let bch = f.decode().unwrap();
        assert_eq!(bch.meta_offset, 408);
        assert_eq!(
            bch.pages_offset,
            PagesOffset {
                offset_sizes: vec![
                    OffsetSize { offset: 16, size: 196 },
                    OffsetSize { offset: 212, size: 196 },
                ],
            }
        );
        let (pages_handle, mut meta_h) = bch.into_handles();
        let meta = meta_h.decode().unwrap();
        assert_eq!(
            meta,
            (5672, Some(Metadata { nshttp_cookie_accept_policy: 2 }))
        );

        for mut page_decoder in pages_handle.decoders() {
            let cookie_handle = page_decoder.decode().unwrap();
            for mut cookie_decoder in cookie_handle.decoders() {
                let _cookie = cookie_decoder.decode().unwrap();
            }
        }
    }
}
