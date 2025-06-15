#[cfg(test)]
mod tests;

use std::{error::Error, fmt::Display, num::NonZeroUsize};

use chrono::{offset::LocalResult, DateTime, TimeZone as _, Utc};
use winnow::{
    binary::{be_u32, be_u64, le_f64, le_u16, le_u32},
    combinator::repeat,
    error::{ContextError, ErrMode, FromExternalError, StrContext, StrContextValue},
    token::take,
    ModalResult, Parser, Partial,
};

use crate::cookie::{BinaryCookies, Cookie, Metadata, Page, SameSite};

pub(crate) type Stream<'i> = Partial<&'i [u8]>;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpectErr {
    U32(u32),
    U64(u64),
    Array([u8; 4]),
}

impl std::fmt::Debug for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Error for ExpectErr {}

impl Display for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U32(binary) => f.write_fmt(format_args!("{:#>06x}", binary)),
            Self::U64(binary) => f.write_fmt(format_args!("{:#>010x}", binary)),
            Self::Array(array) => f.write_fmt(format_args!("{:?}", array)),
        }
    }
}

trait F64ToSafariTime {
    fn to_utc(&self) -> Option<DateTime<Utc>>;
}
impl F64ToSafariTime for f64 {
    #[expect(clippy::cast_sign_loss, reason = "Don't worry")]
    fn to_utc(&self) -> Option<DateTime<Utc>> {
        let seconds = self.trunc() as i64 + 978_307_200;
        let nanos = ((self.fract()) * 1_000_000_000_f64) as u32;

        match Utc.timestamp_opt(seconds, nanos) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[expect(clippy::exhaustive_structs, reason = "allow")]
pub struct CookieDecoder;

#[rustfmt::skip]
impl CookieDecoder {
    pub const IS_SECURE: u32 = 0x1;
    pub const IS_HTTP_ONLY: u32 = 0x4;

    pub const NONE:   u32 = 0b100000;
    pub const LAX:    u32 = 0b101000;
    pub const STRICT: u32 = 0b111000;
}

impl CookieDecoder {
    pub(crate) const fn same_site(flags: u32) -> SameSite {
        #[expect(clippy::wildcard_in_or_patterns, reason = "this is more clear")]
        match flags & 0b111000 {
            Self::LAX => SameSite::Lax,
            Self::STRICT => SameSite::Strict,
            Self::NONE | _ => SameSite::None,
        }
    }

    pub(crate) const fn is_secure(flags: u32) -> bool {
        flags & Self::IS_SECURE == Self::IS_SECURE
    }

    pub(crate) const fn is_http_only(flags: u32) -> bool {
        flags & Self::IS_HTTP_ONLY == Self::IS_HTTP_ONLY
    }

    pub fn binary_cookies(input: &mut Stream) -> ModalResult<BinaryCookies> {
        let signature = take(4_usize).parse_next(input)?;
        if signature != BinaryCookies::SIGNATURE {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("BinaryCookies signature broken"),
                StrContext::Expected(StrContextValue::Description(
                    r#"Expected signature: `b"cook"`"#,
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }
        let num_pages = be_u32(input)? as usize;
        let _page_sizes: Vec<u32> = repeat(num_pages..num_pages + 1, be_u32).parse_next(input)?;
        let pages = repeat(num_pages..num_pages + 1, Self::page).parse_next(input)?;

        let checksum = be_u32(input)?;
        let footer = be_u64(input)?;
        if footer != BinaryCookies::FOOTER {
            let mut context_error =
                ContextError::from_external_error(input, ExpectErr::U64(footer));
            context_error.extend([
                StrContext::Label("BinaryCookies footer broken"),
                StrContext::Expected(StrContextValue::Description(
                    r#"Expected signature: `0x071720050000004b_u64`"#,
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        // TODO: get the remaining size
        let bytes = input.into_inner();
        let metadata = plist::from_bytes::<Metadata>(bytes).ok();

        Ok(BinaryCookies {
            #[cfg(test)]
            page_sizes: _page_sizes,
            pages,
            checksum,
            metadata,
        })
    }

    pub fn page(input: &mut Stream) -> ModalResult<Page> {
        let header = be_u32(input)?;
        if Page::HEADER != header {
            let mut context_error =
                ContextError::from_external_error(input, ExpectErr::U32(header));
            context_error.extend([
                StrContext::Label("Page header broken"),
                StrContext::Expected(StrContextValue::Description(
                    "Expected page start header: `0x0010`",
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let num_cookies = le_u32(input)? as usize;
        let _cookie_offsets: Vec<u32> =
            repeat(num_cookies..num_cookies + 1, le_u32).parse_next(input)?;

        let footer = be_u32(input)?;
        if footer != Page::FOOTER {
            let mut context_error =
                ContextError::from_external_error(input, ExpectErr::U32(footer));
            context_error.extend([
                StrContext::Label("Page page footer broken"),
                StrContext::Expected(StrContextValue::Description(
                    "Expected page footer: `0x0000`",
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let cookies: Vec<Cookie> =
            repeat(num_cookies..num_cookies + 1, Self::cookie).parse_next(input)?;

        Ok(Page {
            #[cfg(test)]
            cookie_offsets: _cookie_offsets,
            cookies,
        })
    }

    pub fn cookie(input: &mut Stream) -> ModalResult<Cookie> {
        let cookie_size = le_u32(input)?;

        let need_size = cookie_size - 4;
        if input.len() < need_size as usize && need_size > 0 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(cookie_size as usize)
            })));
        }

        // NOTE: No accurate explanation of `version` and `has_port` was found
        let (
            version,
            flags,
            has_port,
            domain_offset,
            name_offset,
            path_offset,
            value_offset,
            comment_offset,
        ) = (
            le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32,
        )
            .parse_next(input)?;

        if take(4_usize).parse_next(input)? != Cookie::END_HEADER {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("Cookies end header broken"),
                StrContext::Expected(StrContextValue::Description("Expected end header: `0000`")),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let (raw_expires, raw_creation) = (le_f64, le_f64).parse_next(input)?;
        let (expires, creation) = ((raw_expires).to_utc(), (raw_creation).to_utc());
        let port = if has_port > 0 {
            let port = le_u16(input)?;
            Some(port)
        }
        else {
            None
        };

        let comment = if comment_offset > 0 {
            let comment = Self::get_string(input, (domain_offset - comment_offset) as usize)?;
            Some(comment)
        }
        else {
            None
        };

        let domain = Self::get_string(input, (name_offset - domain_offset) as usize)?;
        let name = Self::get_string(input, (path_offset - name_offset) as usize)?;
        let path = Self::get_string(input, (value_offset - path_offset) as usize)?;
        let value = Self::get_string(input, (cookie_size - value_offset) as usize)?;

        let same_site = Self::same_site(flags);

        let is_secure = Self::is_secure(flags);
        let is_http_only = Self::is_http_only(flags);

        Ok(Cookie {
            version,
            flags,
            #[cfg(test)]
            domain_offset,
            #[cfg(test)]
            name_offset,
            #[cfg(test)]
            path_offset,
            #[cfg(test)]
            value_offset,
            #[cfg(test)]
            comment_offset,
            #[cfg(test)]
            raw_expires,
            #[cfg(test)]
            raw_creation,
            port,
            comment,
            domain,
            name,
            path,
            value,
            expires,
            creation,
            same_site,
            is_secure,
            is_http_only,
        })
    }

    #[inline(always)]
    fn get_string(input: &mut Stream, len: usize) -> ModalResult<bstr::BString> {
        let str = take(len)
            .map(|c: &[u8]| bstr::BString::new(c[..len - 1].to_vec())) // c-string, end with 0
            .parse_next(input)?;
        Ok(str)
    }
}
