use crate::{
    cookie::{Checksum, Cookie, Metadata},
    decode::{
        binary_cookies::BinaryCookieFsm,
        cookies::{CookieFsm, CookiesOffsetInPage},
        meta::{MetaFsm, MetaOffset},
        pages::{PageFsm, PagesOffset},
    },
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
/// `StreamDecoder`'s state
pub(crate) enum State {
    Bc {
        fsm: BinaryCookieFsm,
    },
    Page {
        fsm: PageFsm,
        remaining_page: u32,
    },
    Cookie {
        fsm: CookieFsm,
        remaining_cookie: u32,
        remaining_page: u32,
    },
    Meta {
        fsm: MetaFsm,
    },
    Finished,
    #[default]
    Transition,
}

#[derive(Clone)]
#[derive(Debug)]
/// `StreamDecoder` return Values
pub enum Values {
    /// Some metadata
    Bc {
        meta_offset: MetaOffset,
        pages_offset: PagesOffset,
    },
    /// Some metadata
    Page(CookiesOffsetInPage),
    /// A cookie
    Cookie(Cookie),
    /// Binarycookies metadata
    Meta {
        checksum: Checksum,
        meta: Option<Metadata>,
    },
}
