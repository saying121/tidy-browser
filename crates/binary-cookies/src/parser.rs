use std::num::NonZeroUsize;

use chrono::{offset::LocalResult, DateTime, TimeZone as _, Utc};
use winnow::{
    binary::{be_u32, le_f64, le_u32},
    combinator::repeat,
    error::{ContextError, ErrMode, StrContext, StrContextValue},
    token::take,
    ModalResult, Parser, Partial,
};

use crate::cookie::{BinaryCookies, Cookie, Page, SameSite};

pub(crate) type Stream<'i> = Partial<&'i [u8]>;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[non_exhaustive]
pub struct CookieParser;

trait I64ToSafariTime {
    fn to_utc(&self) -> Option<DateTime<Utc>>;
}
impl I64ToSafariTime for i64 {
    fn to_utc(&self) -> Option<DateTime<Utc>> {
        let time = self + 978_307_200;

        match Utc.timestamp_opt(time, 0) {
            LocalResult::Single(time) => Some(time),
            LocalResult::Ambiguous(..) | LocalResult::None => None,
        }
    }
}

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
        let pages_offsets: Vec<u32> = repeat(num_pages..num_pages + 1, be_u32).parse_next(input)?;
        let pages = repeat(num_pages..num_pages + 1, Self::page).parse_next(input)?;

        let checksum = Self::get_array(input)?;

        Ok(BinaryCookies { pages_offsets, pages, checksum })
    }

    pub fn page(input: &mut Stream) -> ModalResult<Page> {
        if Page::PAGE_HEADER == take(4_usize).parse_next(input)? {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("Page header broken"),
                StrContext::Expected(StrContextValue::Description(
                    "Expected page start header: `0010`",
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let num_cookies = le_u32(input)? as usize;
        let cookies_offsets: Vec<u32> =
            repeat(num_cookies..num_cookies + 1, le_u32).parse_next(input)?;

        if take(4_usize).parse_next(input)? != Page::PAGE_FOOTER {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("Page page footer broken"),
                StrContext::Expected(StrContextValue::Description("Expected page footer: `0000`")),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let cookies: Vec<Cookie> =
            repeat(num_cookies..num_cookies + 1, Self::cookie).parse_next(input)?;

        Ok(Page { cookies_offsets, cookies })
    }

    pub fn cookie(input: &mut Stream) -> ModalResult<Cookie> {
        let cookie_size = le_u32(input)?;

        if input.len() < cookie_size as usize && cookie_size > 0 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(cookie_size as usize)
            })));
        }

        // NOTE: No accurate explanation found
        let version = Self::get_array(input)?;

        let cookie_flags = le_u32(input)?;

        let has_port = Self::get_array(input)?;

        let domain_offset = le_u32(input)?;
        let name_offset = le_u32(input)?;
        let path_offset = le_u32(input)?;
        let value_offset = le_u32(input)?;
        let comment_offset = le_u32(input)?;

        if take(4_usize).parse_next(input)? != Cookie::END_HEADER {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("Cookies end header broken"),
                StrContext::Expected(StrContextValue::Description("Expected end header: `0000`")),
            ]);
            return Err(ErrMode::Cut(context_error));
        }

        let expires = (le_f64(input)? as i64).to_utc();
        let creation = (le_f64(input)? as i64).to_utc();

        let comment = if comment_offset > 0 {
            let comment_len = (domain_offset - comment_offset) as usize;
            let comment = Self::get_string(input, comment_len)?;
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
        let same_site = match cookie_flags & 0b111000 {
            0b101000 => SameSite::Lax,
            0b111000 => SameSite::Strict,
            0b100000 | _ => SameSite::None,
        };

        let is_secure = cookie_flags & 0x1 == 0x1;
        let is_http_only = cookie_flags & 0x4 == 0x4;

        Ok(Cookie {
            cookie_size,
            version,
            cookie_flags,
            same_site,
            is_secure,
            is_http_only,
            has_port,
            domain_offset,
            name_offset,
            path_offset,
            value_offset,
            comment_offset,
            expires,
            creation,
            comment,
            domain,
            name,
            path,
            value,
        })
    }

    #[inline(always)]
    fn get_array<const N: usize>(input: &mut Stream) -> ModalResult<[u8; N]> {
        let slice = take(N).parse_next(input)?;
        let mut array: [u8; N] = [0; N];

        array[..N].copy_from_slice(&slice[..N]);

        Ok(array)
    }

    #[inline(always)]
    fn get_string(input: &mut Stream, len: usize) -> ModalResult<String> {
        let comment = take(len).parse_next(input)?;
        Ok(String::from_utf8_lossy(&comment[..len - 1]).to_string()) // c-string, end with 0
    }
}
