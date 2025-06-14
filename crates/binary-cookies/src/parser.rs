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
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpectErr {
    found: u32,
}

impl std::fmt::Debug for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpectErr")
            .field("found", &format_args!("{:#b}", self.found))
            .finish()
    }
}

impl Error for ExpectErr {}

impl Display for ExpectErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#04b}", self.found))
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
pub struct CookieParser;

impl CookieParser {
    pub fn parse(input: &mut Stream) -> ModalResult<BinaryCookies> {
        if take(4_usize).parse_next(input)? != BinaryCookies::SIGNATURE {
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
        let page_sizes: Vec<u32> = repeat(num_pages..num_pages + 1, be_u32).parse_next(input)?;
        let pages = repeat(num_pages..num_pages + 1, Self::page).parse_next(input)?;

        let checksum = be_u32(input)?;
        if be_u64(input)? != BinaryCookies::FOOTER {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("BinaryCookies footer broken"),
                StrContext::Expected(StrContextValue::Description(
                    r#"Expected signature: `0x071720050000004b_u64`"#,
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let other = &input[..];

        let metadata = plist::from_bytes::<Metadata>(other).ok();

        Ok(BinaryCookies {
            page_sizes,
            pages,
            checksum,
            metadata,
        })
    }

    pub fn page(input: &mut Stream) -> ModalResult<Page> {
        if Page::HEADER != be_u32(input)? {
            let mut context_error =
                ContextError::from_external_error(input, ExpectErr { found: Page::HEADER });
            context_error.extend([
                StrContext::Label("Page header broken"),
                StrContext::Expected(StrContextValue::Description(
                    "Expected page start header: `0010`",
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let num_cookies = le_u32(input)? as usize;
        let cookie_offsets: Vec<u32> =
            repeat(num_cookies..num_cookies + 1, le_u32).parse_next(input)?;

        if be_u32(input)? != Page::FOOTER {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("Page page footer broken"),
                StrContext::Expected(StrContextValue::Description("Expected page footer: `0000`")),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let cookies: Vec<Cookie> =
            repeat(num_cookies..num_cookies + 1, Self::cookie).parse_next(input)?;

        Ok(Page { cookie_offsets, cookies })
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

        #[expect(clippy::wildcard_in_or_patterns, reason = "this is more clear")]
        let same_site = match flags & 0b111000 {
            0b101000 => SameSite::Lax,
            0b111000 => SameSite::Strict,
            0b100000 | _ => SameSite::None,
        };

        let is_secure = flags & 0x1 == 0x1;
        let is_http_only = flags & 0x4 == 0x4;

        Ok(Cookie {
            version,
            flags,
            domain_offset,
            name_offset,
            path_offset,
            value_offset,
            comment_offset,
            raw_expires,
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
