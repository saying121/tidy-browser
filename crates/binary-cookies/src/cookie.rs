use chrono::{DateTime, Utc};

/// raw file information, with pages
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub struct BinaryCookies {
    // pub signature: [u8],
    // pub num_pages: u32,         // be
    pub pages_offsets: Vec<u32>, // be
    pub pages: Vec<Page>,
    pub checksum: [u8; 8], // 8 byte
}

impl BinaryCookies {
    /// 0 offset, 4 size
    pub const SIGNATURE: &'static [u8] = b"cook";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub struct Page {
    // pub pages_start: [u8],
    // pub num_cookies: u32,          // le
    pub cookies_offsets: Vec<u32>, // le, N * `self.num_cookies`
    // pub page_end: [u8],            // Must be equal to []byte{0x00_00_00_00}
    pub cookies: Vec<Cookie>,
}

impl Page {
    pub const PAGE_HEADER: [u8; 4] = [0x00, 0x00, 0x01, 0x00];
    pub const PAGE_FOOTER: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
}

/// alone cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub struct Cookie {
    pub cookie_size: u32, // LE_uint32	Cookie size. Number of bytes associated to the cookie
    /// NOTE: No accurate explanation for this field was found
    pub version: [u8; 4], // byte    Unknown field possibly related to the cookie flags
    pub cookie_flags: u32, /* LE_uint32    0x0:None , 0x1:Secure , 0x4:HttpOnly , 0x5:Secure+HttpOnly */
    pub same_site: SameSite,
    pub is_secure: bool,
    pub is_http_only: bool,
    pub has_port: [u8; 4],   // size:  4    byte    0 or 1
    pub domain_offset: u32,  // LE_uint32    Cookie domain offset
    pub name_offset: u32,    // LE_uint32    Cookie name offset
    pub path_offset: u32,    // LE_uint32    Cookie path offset
    pub value_offset: u32,   // LE_uint32    Cookie value offset
    pub comment_offset: u32, // LE_uint32    Cookie comment offset
    // pub end_header: [u8; 4], /* 4    byte    Marks the end of a header. Must be equal to []byte{0x00000000} */
    pub expires: Option<DateTime<Utc>>, /* float64    Cookie expiration time in Mac epoch time. Add 978307200 to turn into Unix */
    pub creation: Option<DateTime<Utc>>, /* float64    Cookie creation time in Mac epoch time. Add 978307200 to turn into Unix */
    pub comment: Option<String>, /* N    LE_uint32    Cookie comment string. N = `self.domain_offset` - `self.comment_offset` */
    pub domain: String, /* N    LE_uint32    Cookie domain string. N = `self.name_offset` - `self.domain_offset` */
    pub name: String, /* N    LE_uint32    Cookie name string. N = `self.path_offset` - `self.name_offset` */
    pub path: String, /* N    LE_uint32    Cookie path string. N = `self.value_offset` - `self.path_offset` */
    pub value: String, /* N    LE_uint32    Cookie value string. N = `self.cookie_size` - `self.value_offset` */
}

impl Cookie {
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
