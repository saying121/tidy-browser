//! reference
//! <https://github.com/cixtor/binarycookies>
//! <https://github.com/libyal/dtformats/blob/main/documentation/Safari%20Cookies.asciidoc>
//! <https://github.com/interstateone/BinaryCookies>
//! <http://justsolve.archiveteam.org/wiki/Safari_cookies>

use bytes::Buf;
use chrono::{offset::LocalResult, prelude::*, Utc};
use miette::{bail, IntoDiagnostic, Result};

use crate::browser::{
    cookies::{CookiesInfo, SameSite},
    info::BrowserTime,
};

trait I64ToSafariTime: BrowserTime {
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

/// raw file information, with pages
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct BinaryCookies {
    signature:    Vec<u8>,
    num_pages:    u32,      // be
    pages_offset: Vec<u32>, // be
    pub pages:    Vec<Page>,
    checksum:     Vec<u8>, // 8 byte
}

impl BinaryCookies {
    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }
    /// iter all pages cookies
    pub fn iter_cookies(&self) -> impl Iterator<Item = &SafariCookie> {
        self.pages
            .iter()
            .flat_map(Page::iter_cookies)
    }

    pub fn signature(&self) -> &[u8] {
        self.signature.as_ref()
    }

    pub const fn num_pages(&self) -> u32 {
        self.num_pages
    }

    pub fn pages_offset(&self) -> &[u32] {
        self.pages_offset.as_ref()
    }

    pub fn checksum(&self) -> &[u8] {
        self.checksum.as_ref()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Page {
    pages_start:     Vec<u8>,
    num_cookies:     u32,      // le
    cookies_offsets: Vec<u32>, // le, N * `self.num_cookies`
    page_end:        Vec<u8>,  // Must be equal to []byte{0x00_00_00_00}
    pub cookies:     Vec<SafariCookie>,
}

impl Page {
    pub fn iter_cookies(&self) -> impl Iterator<Item = &SafariCookie> {
        self.cookies.iter()
    }

    pub fn pages_start(&self) -> &[u8] {
        self.pages_start.as_ref()
    }

    pub const fn num_cookies(&self) -> u32 {
        self.num_cookies
    }

    pub fn cookies_offsets(&self) -> &[u32] {
        self.cookies_offsets.as_ref()
    }

    pub fn page_end(&self) -> &[u8] {
        self.page_end.as_ref()
    }
}

impl BinaryCookies {
    const SIGNATURE: &'static [u8] = b"cook"; // 0 offset, 4 size
    const PAGE_START_HEADER: [u8; 4] = [0x00, 0x00, 0x01, 0x00];
    const END_HEADER: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

    // raw data and some unnecessary data
    pub fn parse(file: &[u8]) -> Result<Self> {
        let mut entry = file;

        if entry.len() < 4 || &entry[..4] != Self::SIGNATURE {
            bail!("wrong SIGNATURE")
        }
        entry.advance(4);

        assert!(entry.len() > 7);

        let num_pages = entry.get_u32();

        let mut pages_offsets = Vec::new();
        for _ in 0..num_pages {
            let pages_offset = entry.get_u32();
            pages_offsets.push(pages_offset);
        }
        let page_size = pages_offsets.iter().sum::<u32>() as usize;
        if entry.len() < page_size {
            bail!("wrong data")
        }
        let mut pages = vec![];

        for _ in 0..num_pages {
            let page = Self::parse_page(&mut entry)?;
            pages.push(page);
        }

        Ok(Self {
            signature: Self::SIGNATURE.to_vec(),
            num_pages,
            pages_offset: pages_offsets,
            pages,
            checksum: entry[..8].to_vec(),
        })
    }
    fn parse_page(entry: &mut &[u8]) -> Result<Page> {
        // page start
        if entry.len() < 4 || entry[..4] != Self::PAGE_START_HEADER {
            bail!("wrong cookie header")
        }
        assert!(entry.len() > 3);
        entry.advance(4);
        let pages_start = Self::PAGE_START_HEADER.to_vec();

        let num_cookies = entry.get_u32_le();

        let mut cookies_offsets = vec![];
        for _ in 0..num_cookies {
            cookies_offsets.push(entry.get_u32_le());
        }

        // page end
        if entry[..4] != Self::END_HEADER {
            bail!("wrong cookie end")
        }

        entry.advance(4);
        let page_end = Self::END_HEADER.to_vec();

        let mut raw_cookies = vec![];
        for _ in 0..num_cookies {
            let cook = Self::parse_cookie(entry)?;
            raw_cookies.push(cook);
        }

        let page = Page {
            pages_start,
            num_cookies,
            cookies_offsets,
            page_end,
            cookies: raw_cookies,
        };
        Ok(page)
    }
    fn parse_cookie(entry: &mut &[u8]) -> Result<SafariCookie> {
        let cookie_size = entry.get_u32_le();

        if entry.len() < 8 {
            bail!("wrong")
        }
        assert!(entry.len() > 7);

        let version = entry[..4].to_vec();
        entry.advance(4);

        let cookie_flags = entry.get_u32_le();

        let has_port = entry[..4]
            .try_into()
            .into_diagnostic()?;
        entry.advance(4);

        let domain_offset = entry.get_u32_le();
        let name_offset = entry.get_u32_le();
        let path_offset = entry.get_u32_le();
        let value_offset = entry.get_u32_le();
        let comment_offset = entry.get_u32_le();

        let end_header = &entry[..4];
        if end_header != Self::END_HEADER {
            bail!("wrong end header")
        }
        entry.advance(4);

        let expires = f64::from_le_bytes(
            entry[..8]
                .try_into()
                .into_diagnostic()?,
        );
        entry.advance(8);
        let expires = (expires as i64).to_utc();

        let creation = f64::from_le_bytes(
            entry[..8]
                .try_into()
                .into_diagnostic()?,
        );
        entry.advance(8);
        let creation = (creation as i64).to_utc();

        let comment = if comment_offset > 0 {
            let comment_len = (domain_offset - comment_offset) as usize;
            let comment = &entry[..comment_len - 1]; // c-string, end with 0
            entry.advance(comment_len);
            String::from_utf8_lossy(comment).to_string()
        }
        else {
            String::new()
        };

        let domin_len = (name_offset - domain_offset) as usize;
        let domain = &entry[..domin_len - 1]; // c-string, end with 0
        entry.advance(domin_len);
        let domain = String::from_utf8_lossy(domain).to_string();

        let name_len = (path_offset - name_offset) as usize;
        let name = &entry[..name_len - 1]; // c-string, end with 0
        entry.advance(name_len);
        let name = String::from_utf8_lossy(name).to_string();

        let path_len = (value_offset - path_offset) as usize;
        let path = &entry[..path_len - 1]; // c-string, end with 0
        entry.advance(path_len);
        let path = String::from_utf8_lossy(path).to_string();

        let value_len = (cookie_size - value_offset) as usize;
        let value = &entry[..value_len - 1]; // c-string, end with 0
        entry.advance(value_len);
        let value = String::from_utf8_lossy(value).to_string();

        let same_site = match cookieflag & 56 {
            40 => SameSite::Lax,
            56 => SameSite::Strict,
            32 | _ => SameSite::None,
        };

        Ok(SafariCookie {
            // cookie_size,
            version,
            cookie_flags,
            same_site,
            is_secure: cookie_flags & 0x1 == 0x1,
            is_httponly: cookie_flags & 0x4 == 0x4,
            has_port,
            domain_offset,
            name_offset,
            path_offset,
            value_offset,
            comment_offset,
            // end_header,
            expires,
            creation,
            comment,
            domain,
            name,
            path,
            value,
        })
    }
}

/// alone cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct SafariCookie {
    // cookie_size:    u32, // LE_uint32	Cookie size. Number of bytes associated to the cookie
    pub version:       Vec<u8>, // byte    Unknown field possibly related to the cookie flags
    cookie_flags:      u32, /* LE_uint32    0x0:None , 0x1:Secure , 0x4:HttpOnly , 0x5:Secure+HttpOnly */
    pub same_site:     SameSite,
    pub is_secure:     bool,
    pub is_httponly:   bool,
    pub has_port:      [u8; 4], // size:  4    byte    0 or 1
    pub domain_offset: u32,     // LE_uint32    Cookie domain offset
    name_offset:       u32,     // LE_uint32    Cookie name offset
    path_offset:       u32,     // LE_uint32    Cookie path offset
    value_offset:      u32,     // LE_uint32    Cookie value offset
    comment_offset:    u32,     // LE_uint32    Cookie comment offset
    // end_header:     Vec<u8>, /* 4    byte    Marks the end of a header. Must be equal to []byte{0x00000000} */
    pub expires:       Option<DateTime<Utc>>, /* float64    Cookie expiration time in Mac epoch time. Add 978307200 to turn into Unix */
    pub creation:      Option<DateTime<Utc>>, /* float64    Cookie creation time in Mac epoch time. Add 978307200 to turn into Unix */
    pub comment:       String, /* N    LE_uint32    Cookie comment string. N = `self.domain_offset` - `self.comment_offset` */
    pub domain:        String, /* N    LE_uint32    Cookie domain string. N = `self.name_offset` - `self.domain_offset` */
    pub name:          String, /* N    LE_uint32    Cookie name string. N = `self.path_offset` - `self.name_offset` */
    pub path:          String, /* N    LE_uint32    Cookie path string. N = `self.value_offset` - `self.path_offset` */
    pub value:         String, /* N    LE_uint32    Cookie value string. N = `self.cookie_size` - `self.value_offset` */
}
#[cfg(feature = "reqwest")]
impl TryFrom<SafariCookie> for reqwest::header::HeaderValue {
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(value: SafariCookie) -> Result<Self, Self::Error> {
        Self::from_str(&value.get_set_cookie_header())
    }
}
#[cfg(feature = "reqwest")]
impl FromIterator<SafariCookie> for reqwest::cookie::Jar {
    fn from_iter<T: IntoIterator<Item = SafariCookie>>(iter: T) -> Self {
        let jar = Self::default();
        for cookie in iter {
            let set_cookie = cookie.get_set_cookie_header();
            if let Ok(url) = reqwest::Url::parse(&cookie.get_url()) {
                jar.add_cookie_str(&set_cookie, &url);
            }
        }
        jar
    }
}

impl CookiesInfo for SafariCookie {
    fn name(&self) -> &str {
        &self.name
    }
    fn path(&self) -> &str {
        &self.path
    }
    fn domain(&self) -> &str {
        &self.domain
    }
    fn value(&self) -> &str {
        &self.value
    }
    fn expiry(&self) -> Option<String> {
        match self.expires {
            Some(expiry) => Some(expiry.to_rfc2822()),
            None => None,
        }
    }
    fn is_secure(&self) -> bool {
        self.is_secure
    }
    fn is_http_only(&self) -> bool {
        self.is_httponly
    }
    fn same_site(&self) -> SameSite {
        self.same_site
    }
}

impl SafariCookie {
    pub const fn creation(&self) -> DateTime<Utc> {
        self.creation
    }
}

impl SafariCookie {
    pub fn version(&self) -> &[u8] {
        self.version.as_ref()
    }

    pub const fn cookie_flags(&self) -> u32 {
        self.cookie_flags
    }

    pub fn has_port(&self) -> &[u8] {
        self.has_port.as_ref()
    }

    pub const fn domain_offset(&self) -> u32 {
        self.domain_offset
    }

    pub const fn name_offset(&self) -> u32 {
        self.name_offset
    }

    pub const fn path_offset(&self) -> u32 {
        self.path_offset
    }

    pub const fn value_offset(&self) -> u32 {
        self.value_offset
    }

    pub const fn comment_offset(&self) -> u32 {
        self.comment_offset
    }

    pub fn comment(&self) -> &str {
        self.comment.as_ref()
    }
}
