use std::num::NonZeroUsize;

use bstr::{BString, ByteSlice as _};
use chrono::{DateTime, TimeZone as _, Utc};
use serde::{Deserialize, Serialize};
use winnow::{
    binary::{be_i8, be_u32, be_u64, be_u8},
    combinator::repeat,
    error::{ContextError, ErrMode, FromExternalError, StrContext, StrContextValue},
    token::take,
    ModalResult, Parser,
};

use crate::{
    decode::{binary_cookies::Offsets, CookieDecoder, ExpectErr, StreamIn},
    error::BplistErr,
};

/// raw file information, with pages
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[non_exhaustive]
pub struct BinaryCookies {
    // pub magic: [u8: 4],
    // pub num_pages: u32,         // be
    #[cfg(test)]
    pub page_sizes: Vec<u32>, // be
    pub pages: Vec<Page>,
    pub checksum: Checksum, // 4 byte
    pub metadata: Option<Metadata>,
}

pub type Checksum = u32;

impl BinaryCookies {
    pub fn parser_head(input: &mut StreamIn) -> ModalResult<Offsets> {
        if input.len() < 8 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(8 - input.len())
            })));
        }
        let magic = take(4_usize).parse_next(input)?;
        if magic != Self::MAGIC {
            let mut context_error = ContextError::new();
            context_error.extend([
                StrContext::Label("BinaryCookies magic broken"),
                StrContext::Expected(StrContextValue::Description(
                    r#"Expected magic: `b"cook"`"#,
                )),
            ]);
            return Err(ErrMode::Cut(context_error));
        }
        let num_pages = be_u32(input)? as usize;
        let page_size = num_pages * 4;

        if input.len() < page_size {
            let size = unsafe { NonZeroUsize::new_unchecked(page_size - input.len()) };
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(size)));
        }

        let page_sizes: Vec<u32> = repeat(num_pages..num_pages + 1, be_u32).parse_next(input)?;

        let tail_offset = 4
            + 4
            + num_pages * 4
            + page_sizes
                .iter()
                .map(|&v| v as usize)
                .sum::<usize>();
        Ok(Offsets { page_sizes, tail_offset })
    }

    pub fn parser_tail(input: &mut StreamIn) -> ModalResult<(Checksum, Option<Metadata>)> {
        if input.len() < 4 + 8 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(4 + 8 - input.len())
            })));
        }
        let checksum = be_u32(input)?;
        let footer = be_u64(input)?;
        if footer != Self::FOOTER {
            let mut ctx_err = ContextError::from_external_error(input, ExpectErr::U64(footer));
            ctx_err.extend([
                StrContext::Label("BinaryCookies footer broken"),
                StrContext::Expected(StrContextValue::Description(
                    r#"Expected : `0x071720050000004b_u64`"#,
                )),
            ]);
            return Err(ErrMode::Cut(ctx_err));
        }

        let metadata = Metadata::decode(input).ok();
        Ok((checksum, metadata))
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
#[expect(clippy::exhaustive_structs, reason = "allow")]
pub struct Metadata {
    #[serde(rename = "NSHTTPCookieAcceptPolicy")]
    pub nshttp_cookie_accept_policy: i8,
}

impl Metadata {
    // See apple opensource CFBinaryPList.c
    // This is a very specialized decoder that needs to be updated with the BinaryCookies format
    pub(crate) fn decode(input: &mut StreamIn) -> Result<Self, ErrMode<ContextError>> {
        if input.len() < 75 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(75 - input.len())
            })));
        }
        let bplist = take(8_usize).parse_next(input)?;
        if bplist != b"bplist00" {
            let ctx_err = ContextError::from_external_error(input, BplistErr::Magic);
            return Err(ErrMode::Cut(ctx_err));
        }
        let dict = be_u8(input)?;
        if dict != 0xD1 {
            let ctx_err = ContextError::from_external_error(input, BplistErr::NotDict);
            return Err(ErrMode::Cut(ctx_err));
        }
        let _length = take(5_usize).parse_next(input)?;
        let key = take(24_usize).parse_next(input)?;
        if b"NSHTTPCookieAcceptPolicy" != key {
            let ctx_err = ContextError::from_external_error(input, BplistErr::BadKey);
            return Err(ErrMode::Cut(ctx_err));
        }
        let int_flags = be_u8(input)?;
        if int_flags != 0x10 {
            let ctx_err = ContextError::from_external_error(input, BplistErr::OneByteInt);
            return Err(ErrMode::Cut(ctx_err));
        }
        let int_val = be_i8(input)?;
        take(32 + 3_usize).parse_next(input)?;
        let metadata = Self {
            nshttp_cookie_accept_policy: int_val,
        };
        Ok(metadata)
    }
}

impl BinaryCookies {
    pub const MAGIC: &'static [u8] = b"cook"; // 0 offset, 4 size
    pub const FOOTER: u64 = 0x071720050000004B;

    pub fn new(pages: Vec<Page>) -> Self {
        let mut self_ = Self {
            #[cfg(test)]
            page_sizes: vec![],
            pages,
            checksum: 0,
            metadata: None,
        };
        #[cfg(test)]
        {
            self_.page_sizes = self_.page_sizes();
        };
        self_.checksum = self_.checksum();
        self_
    }

    pub fn push(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn page_sizes(&self) -> Vec<u32> {
        self.pages
            .iter()
            .map(Page::size)
            .collect()
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }

    /// iter all pages cookies
    pub fn iter_cookies(&self) -> impl Iterator<Item = &Cookie> {
        self.pages()
            .flat_map(Page::iter_cookies)
    }

    /// FIXME: checksum impl not correct
    pub fn checksum(&self) -> u32 {
        self.pages
            .iter()
            .fold(0_u32, |i, v| v.encode().1.wrapping_add(i))
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(Self::MAGIC);
        raw.extend_from_slice(&(self.pages.len() as u32).to_be_bytes());
        for ele in self.pages() {
            raw.extend_from_slice(&ele.size().to_be_bytes());
        }

        // FIXME: checksum impl not correct
        let checksum = self
            .pages
            .iter()
            .fold(0_u32, |i, v| {
                let (data, sum) = v.encode();
                raw.extend_from_slice(&data);
                i.wrapping_add(sum)
            });

        raw.extend_from_slice(&checksum.to_be_bytes());
        raw.extend_from_slice(&Self::FOOTER.to_be_bytes());
        if let Some(meta) = &self.metadata {
            // optional data ignore error
            plist::to_writer_binary(&mut raw, meta).ok();
        }
        raw
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[non_exhaustive]
pub struct Page {
    // pub pages_start: [u8; 4],
    // pub num_cookies: u32,          // le
    #[cfg(test)]
    /// Offset in the page
    pub cookie_offsets: Vec<u32>, // le, N * `self.num_cookies`
    // pub page_end: [u8],            // Must be equal to []byte{0x00_00_00_00}
    pub cookies: Vec<Cookie>,
}

impl Page {
    pub const HEADER: u32 = 0x00000100;
    pub const FOOTER: u32 = 0x00000000;

    pub fn push(&mut self, cookie: Cookie) {
        self.cookies.push(cookie);
    }

    /// Dynamic calculation
    pub fn cookie_offsets(&self) -> Vec<u32> {
        let mut offset = 4 + 4 + 4 * self.cookies.len() as u32 + 4;
        let mut offsets = Vec::with_capacity(self.cookies.len());
        for ele in &self.cookies {
            offsets.push(offset);
            offset += ele.size();
        }
        offsets
    }

    pub fn size(&self) -> u32 {
        4 * 3
            + self.cookies.len() as u32 * 4
            + self
                .cookies
                .iter()
                .map(Cookie::size)
                .sum::<u32>()
    }

    pub fn encode(&self) -> (Vec<u8>, u32) {
        let data = self._encode();
        // FIXME: checksum impl not correct
        let checksum = data
            .iter()
            .step_by(4)
            .fold(0_u32, |i, &v| i.wrapping_add(v as u32));

        (data, checksum)
    }

    fn _encode(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(&Self::HEADER.to_be_bytes());
        raw.extend_from_slice(&(self.cookies.len() as u32).to_le_bytes());
        for ele in self.cookie_offsets() {
            raw.extend_from_slice(&ele.to_le_bytes());
        }
        raw.extend_from_slice(&Self::FOOTER.to_be_bytes());
        for ele in &self.cookies {
            raw.extend_from_slice(&ele.encode());
        }

        raw
    }

    pub fn iter_cookies(&self) -> impl Iterator<Item = &Cookie> {
        self.cookies.iter()
    }
}

/// alone cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[cfg_attr(not(test), derive(Eq))]
#[expect(clippy::exhaustive_structs, reason = "allow")]
pub struct Cookie {
    // pub cookie_size: u32, // LE Cookie size. Number of bytes associated to the cookie
    pub version: u32, // LE Unknown field possibly related to the cookie flags

    pub flags: u32, // LE 0x0:None , 0x1:Secure , 0x4:HttpOnly , 0x5:Secure+HttpOnly
    // pub has_port: u32,       // LE 0 or 1
    #[cfg(test)]
    pub domain_offset: u32, // LE Cookie domain offset in the cookie
    #[cfg(test)]
    pub name_offset: u32, // LE Cookie name offset in the cookie
    #[cfg(test)]
    pub path_offset: u32, // LE Cookie path offset in the cookie
    #[cfg(test)]
    pub value_offset: u32, // LE Cookie value offset in the cookie
    #[cfg(test)]
    pub comment_offset: u32, // LE Cookie comment offset in the cookie

    // pub end_header: [u8; 4], /* 4    byte    Marks the end of a header. Must be equal to []byte{0x00000000} */
    #[cfg(test)]
    pub raw_expires: f64, /* f64    Cookie expiration time in Mac epoch time. Add 978307200 to turn into Unix */
    #[cfg(test)]
    pub raw_creation: f64, /* f64    Cookie creation time in Mac epoch time. Add 978307200 to turn into Unix */

    pub port: Option<u16>, // LE  Only present if the "Has port" field is 1
    pub comment: Option<BString>, /* Cookie comment string. N = `self.domain_offset` - `self.comment_offset` when `comment_offset` > 0 */
    pub domain: BString, // Cookie domain string. N = `self.name_offset` - `self.domain_offset`
    pub name: BString,   // Cookie name string. N = `self.path_offset` - `self.name_offset`
    pub path: BString,   // Cookie path string. N = `self.value_offset` - `self.path_offset`
    pub value: BString,  // Cookie value string. N = `self.cookie_size` - `self.value_offset`

    pub expires: Option<DateTime<Utc>>,
    pub creation: Option<DateTime<Utc>>,
    pub same_site: SameSite,
    pub is_secure: bool,
    pub is_http_only: bool,
}

impl Cookie {
    pub const fn flags(&self) -> u32 {
        let mut flags = self.flags;

        if self.is_secure {
            flags |= CookieDecoder::IS_SECURE;
        }

        if self.is_http_only {
            flags |= CookieDecoder::IS_HTTP_ONLY;
        }

        match self.same_site {
            SameSite::None => {},
            SameSite::Lax => flags |= CookieDecoder::SS_LAX,
            SameSite::Strict => flags |= CookieDecoder::SS_STRICT,
        }

        flags
    }

    pub(crate) fn time_to_f64(time: DateTime<Utc>) -> f64 {
        let timestamp = time
            - Utc
                .with_ymd_and_hms(2001, 1, 1, 0, 0, 0)
                .unwrap();
        timestamp.num_seconds() as f64
    }

    pub fn encode(&self) -> Vec<u8> {
        let size = self.size();
        let mut raw = Vec::with_capacity(size as usize);
        raw.extend_from_slice(&size.to_le_bytes());
        raw.extend_from_slice(&self.version.to_le_bytes());
        raw.extend_from_slice(&self.flags().to_le_bytes());
        raw.extend_from_slice(&(self.has_port() as u32).to_le_bytes());
        raw.extend_from_slice(&self.domain_offset().to_le_bytes());
        raw.extend_from_slice(&self.name_offset().to_le_bytes());
        raw.extend_from_slice(&self.path_offset().to_le_bytes());
        raw.extend_from_slice(&self.value_offset().to_le_bytes());
        raw.extend_from_slice(&self.comment_offset().to_le_bytes());
        raw.extend_from_slice(&Self::END_HEADER);
        raw.extend_from_slice(&Self::time_to_f64(self.expires.unwrap_or_default()).to_le_bytes());
        raw.extend_from_slice(&Self::time_to_f64(self.creation.unwrap_or_default()).to_le_bytes());
        if let Some(port) = self.port {
            raw.extend_from_slice(&port.to_le_bytes());
        }
        if let Some(s) = &self.comment {
            Self::encode_string(&mut raw, s.as_bstr());
        }
        Self::encode_string(&mut raw, self.domain.as_bstr());
        Self::encode_string(&mut raw, self.name.as_bstr());
        Self::encode_string(&mut raw, self.path.as_bstr());
        Self::encode_string(&mut raw, self.value.as_bstr());

        raw
    }

    pub const fn has_port(&self) -> bool {
        self.port.is_some()
    }

    /// Dynamic calculation
    const fn prefix_offset(&self) -> u32 {
        4 * 10 + 8 * 2 + if self.has_port() { 2 } else { 0 }
    }
    /// Dynamic calculation
    pub fn domain_offset(&self) -> u32 {
        self.prefix_offset()
            + self
                .comment
                .as_ref()
                .map_or(0, |v| v.len() as u32 + 1)
    }
    /// Dynamic calculation
    pub fn name_offset(&self) -> u32 {
        self.domain_offset() + self.domain.len() as u32 + 1
    }
    /// Dynamic calculation
    pub fn path_offset(&self) -> u32 {
        self.name_offset() + self.name.len() as u32 + 1
    }
    /// Dynamic calculation
    pub fn value_offset(&self) -> u32 {
        self.path_offset() + self.path.len() as u32 + 1
    }
    /// Dynamic calculation
    pub const fn comment_offset(&self) -> u32 {
        if self.comment.is_none() {
            0
        }
        else {
            self.prefix_offset()
        }
    }

    pub fn size(&self) -> u32 {
        4 * 10
            + 8 * 2
            + self.port.map_or(0, |_| 2)
            + self
                .comment
                .as_ref()
                .map_or(0, |v| v.len() as u32 + 1)
            + (self.domain.len() as u32 + 1)
            + (self.name.len() as u32 + 1)
            + (self.path.len() as u32 + 1)
            + (self.value.len() as u32 + 1)
    }

    fn encode_string(raw: &mut Vec<u8>, s: &bstr::BStr) {
        raw.extend(s.bytes());
        raw.push(0);
    }

    pub const END_HEADER: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SameSite {
    #[default]
    None,
    Lax,
    Strict,
}

impl From<i32> for SameSite {
    fn from(value: i32) -> Self {
        #[expect(clippy::wildcard_in_or_patterns, reason = "this is more clear")]
        match value {
            1 => Self::Lax,
            2 => Self::Strict,
            0 | _ => Self::None,
        }
    }
}

impl From<Option<i32>> for SameSite {
    fn from(value: Option<i32>) -> Self {
        value.unwrap_or_default().into()
    }
}

impl std::fmt::Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => "None",
            Self::Lax => "Lax",
            Self::Strict => "Strict",
        }
        .fmt(f)
    }
}
