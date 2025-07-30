use std::num::NonZeroUsize;

use bstr::{BString, ByteSlice as _};
use chrono::{DateTime, TimeZone as _, Utc};
use winnow::{
    binary::{be_u32, be_u64, be_u8, le_f64, le_u16, le_u32},
    combinator::repeat,
    error::{ContextError, ErrMode, FromExternalError, Needed, StrContext, StrContextValue},
    token::take,
    ModalResult, Parser,
};

use crate::{
    decode::{
        binary_cookies::Offsets, cookies::CookiesOffsetInPage, F64ToSafariTime as _, StreamIn,
    },
    error::{BadKeySnafu, ExpectErr, MagicSnafu, NotDictSnafu, OneByteIntSnafu},
};

/// raw file information, with pages
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[cfg_attr(not(test), derive(Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(
    clippy::exhaustive_structs,
    reason = "Breaking change with Binarycookies format"
)]
pub struct BinaryCookies {
    pub pages: Vec<Page>,
    pub metadata: Option<Metadata>,
}

pub type Checksum = u32;

impl BinaryCookies {
    pub(crate) fn decode_head(input: &mut StreamIn) -> ModalResult<Offsets> {
        if input.len() < 8 {
            return Err(ErrMode::Incomplete(Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(8 - input.len())
            })));
        }
        let magic = take(4_usize).parse_next(input)?;
        if magic != Self::MAGIC {
            #[expect(clippy::unwrap_used, reason = "magic len is 4")]
            let arr: [u8; 4] = magic.try_into().unwrap();
            let mut context_error = ContextError::from_external_error(input, ExpectErr::Magic(arr));
            context_error.extend([
                StrContext::Label("BinaryCookies magic broken"),
                StrContext::Expected(StrContextValue::Description(r#"Expected magic: `b"cook"`"#)),
            ]);
            return Err(ErrMode::Cut(context_error));
        }
        let num_pages = be_u32(input)? as usize;
        let pages_size = num_pages * 4;

        if input.len() < pages_size {
            let size = unsafe { NonZeroUsize::new_unchecked(pages_size - input.len()) };
            return Err(ErrMode::Incomplete(Needed::Size(size)));
        }

        let page_sizes: Vec<u32> = repeat(num_pages..num_pages + 1, be_u32).parse_next(input)?;

        let tail_offset = 4
            + 4
            + num_pages as u64 * 4
            + page_sizes
                .iter()
                .map(|&v| v as u64)
                .sum::<u64>();
        Ok(Offsets { page_sizes, tail_offset })
    }

    pub(crate) fn decode_tail(input: &mut StreamIn) -> ModalResult<(Checksum, Option<Metadata>)> {
        if input.len() < 4 + 8 {
            return Err(ErrMode::Incomplete(Needed::Size(unsafe {
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
                    r#"Expected big endian: `0x071720050000004b_u64`"#,
                )),
            ]);
            return Err(ErrMode::Cut(ctx_err));
        }

        let metadata = Metadata::decode(input).ok();
        Ok((checksum, metadata))
    }
}

impl BinaryCookies {
    pub const MAGIC: &'static [u8] = b"cook"; // 0 offset, 4 size
    pub const FOOTER: u64 = 0x071720050000004B;

    pub fn push(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn page_sizes(&self) -> Vec<u32> {
        self.pages
            .iter()
            .map(Page::size)
            .collect()
    }

    pub fn iter_pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }

    /// iter all pages cookies
    pub fn iter_cookies(&self) -> impl Iterator<Item = &Cookie> {
        self.iter_pages()
            .flat_map(Page::iter_cookies)
    }

    /// FIXME: checksum impl not correct
    pub fn checksum(&self) -> u32 {
        self.pages
            .iter()
            .fold(0_u32, |i, v| v.encode().1.wrapping_add(i))
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut raw = Self::MAGIC.to_vec();
        raw.extend_from_slice(&(self.pages.len() as u32).to_be_bytes());
        for ele in self.iter_pages() {
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
            raw.extend_from_slice(&meta.encode());
        }
        raw
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[expect(
    clippy::exhaustive_structs,
    reason = "Breaking change with Binarycookies format"
)]
pub struct Metadata {
    #[cfg_attr(test, serde(rename = "NSHTTPCookieAcceptPolicy"))]
    pub nshttp_cookie_accept_policy: u8,
}

impl Metadata {
    #[rustfmt::skip]
    // This is a very specialized decoder that needs to be updated with the BinaryCookies format
    pub const fn encode(&self) -> [u8; 75] {
        [
            98, 112, 108, 105, 115, 116, 48, 48, 209, 1, 2, 95, 16, 24, 78, 83, 72, 84, 84, 80, 67,
            111, 111, 107, 105, 101, 65, 99, 99, 101, 112, 116, 80, 111, 108, 105, 99, 121, 16,
            self.nshttp_cookie_accept_policy,
            8, 11, 38, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 3,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40,
        ]
    }
    // See apple opensource CFBinaryPList.c
    // This is a very specialized decoder that needs to be updated with the BinaryCookies format
    pub(crate) fn decode(input: &mut StreamIn) -> Result<Self, ErrMode<ContextError>> {
        if input.len() < 75 {
            return Err(ErrMode::Incomplete(Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(75 - input.len())
            })));
        }
        let bplist = take(8_usize).parse_next(input)?;
        if bplist != b"bplist00" {
            let ctx_err = ContextError::from_external_error(input, MagicSnafu.build());
            return Err(ErrMode::Cut(ctx_err));
        }
        let dict = be_u8(input)?;
        if dict != 0xD1 {
            let ctx_err = ContextError::from_external_error(input, NotDictSnafu.build());
            return Err(ErrMode::Cut(ctx_err));
        }
        let _length = take(5_usize).parse_next(input)?;
        let key = take(24_usize).parse_next(input)?;
        if b"NSHTTPCookieAcceptPolicy" != key {
            let ctx_err = ContextError::from_external_error(input, BadKeySnafu.build());
            return Err(ErrMode::Cut(ctx_err));
        }
        let int_flags = be_u8(input)?;
        if int_flags != 0x10 {
            let ctx_err = ContextError::from_external_error(input, OneByteIntSnafu.build());
            return Err(ErrMode::Cut(ctx_err));
        }
        let int_val = be_u8(input)?;
        take(32 + 3_usize).parse_next(input)?;
        let metadata = Self {
            nshttp_cookie_accept_policy: int_val,
        };
        Ok(metadata)
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
#[cfg_attr(not(test), derive(Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(
    clippy::exhaustive_structs,
    reason = "Breaking change with Binarycookies format"
)]
pub struct Page {
    pub cookies: Vec<Cookie>,
}

impl Page {
    /// Return cookie offsets
    pub(crate) fn decode_head(input: &mut StreamIn) -> ModalResult<CookiesOffsetInPage> {
        if input.len() < 8 {
            return Err(ErrMode::Incomplete(Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(8 - input.len())
            })));
        }
        let header = be_u32(input)?;
        if Self::HEADER != header {
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
        // cookies size and footer
        let need_size = num_cookies * 4 + 4;
        if input.len() < need_size {
            return Err(ErrMode::Incomplete(Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(need_size - input.len())
            })));
        }
        let cookie_offsets: Vec<u32> =
            repeat(num_cookies..num_cookies + 1, le_u32).parse_next(input)?;

        let footer = be_u32(input)?;
        if footer != Self::FOOTER {
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

        Ok(CookiesOffsetInPage(cookie_offsets))
    }
}

impl Page {
    pub const HEADER: u32 = 0x00000100;
    pub const FOOTER: u32 = 0x00000000;

    pub fn push(&mut self, cookie: Cookie) {
        self.cookies.push(cookie);
    }

    /// Dynamic calculation offset in the page
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(
    clippy::exhaustive_structs,
    reason = "Breaking change with Binarycookies format"
)]
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

#[cfg(feature = "csv")]
impl Cookie {
    pub fn csv_header<D: std::fmt::Display>(sep: D) -> String {
        format!("domain{sep}name{sep}path{sep}value{sep}creation{sep}expires{sep}is_secure{sep}is_http_only")
    }

    pub fn to_csv<D: std::fmt::Display>(&self, sep: D) -> String {
        format!(
            "{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}",
            self.domain,
            self.name,
            self.path,
            self.value,
            self.creation.unwrap_or_default(),
            self.expires.unwrap_or_default(),
            self.is_secure,
            self.is_http_only,
        )
    }
}

#[rustfmt::skip]
impl Cookie {
    pub const IS_SECURE:     u32 = 0b000001;
    pub const IS_HTTP_ONLY:  u32 = 0b000100;
    pub const SAME_SITE_BIT: u32 = 0b111000;
    pub const SS_STRICT:     u32 = 0b111000;
    pub const SS_LAX:        u32 = 0b101000;
    pub const SS_NONE:       u32 = 0b100000;
}

impl Cookie {
    pub(crate) const fn same_site(flags: u32) -> SameSite {
        #[expect(clippy::wildcard_in_or_patterns, reason = "this is more clear")]
        match flags & Self::SAME_SITE_BIT {
            Self::SS_STRICT => SameSite::Strict,
            Self::SS_LAX => SameSite::Lax,
            Self::SS_NONE | _ => SameSite::None,
        }
    }

    pub(crate) const fn is_secure(flags: u32) -> bool {
        flags & Self::IS_SECURE == Self::IS_SECURE
    }

    pub(crate) const fn is_http_only(flags: u32) -> bool {
        flags & Self::IS_HTTP_ONLY == Self::IS_HTTP_ONLY
    }

    pub(crate) fn decode(input: &mut StreamIn) -> ModalResult<Self> {
        let cookie_size = le_u32(input)?;

        let need_size = cookie_size as usize - 4;
        if input.len() < need_size {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(need_size - input.len())
            })));
        }

        // NOTE: No accurate explanation of `version` was found
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

        let end_header = take(4_usize).parse_next(input)?;
        if end_header != Self::END_HEADER {
            #[expect(clippy::unwrap_used, reason = "end_header len is 4")]
            let arr: [u8; 4] = end_header.try_into().unwrap();
            let mut context_error =
                ContextError::from_external_error(input, ExpectErr::EndHeader(arr));
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

        Ok(Self {
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
    fn get_string(input: &mut StreamIn, len: usize) -> ModalResult<bstr::BString> {
        let str = take(len)
            .map(|c: &[u8]| bstr::BString::new(c[..len - 1].to_vec())) // c-string, end with 0
            .parse_next(input)?;
        Ok(str)
    }
}

impl Cookie {
    pub const fn flags(&self) -> u32 {
        let mut flags = self.flags;

        if self.is_secure {
            flags |= Self::IS_SECURE;
        }

        if self.is_http_only {
            flags |= Self::IS_HTTP_ONLY;
        }

        match self.same_site {
            SameSite::None => {},
            SameSite::Lax => flags |= Self::SS_LAX,
            SameSite::Strict => flags |= Self::SS_STRICT,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[test]
fn test_encode_metadata() {
    let meta = Metadata { nshttp_cookie_accept_policy: 1 };
    let mut res = vec![];
    plist::to_writer_binary(&mut res, &meta).unwrap();
    assert_eq!(&meta.encode(), res.as_slice());
}
