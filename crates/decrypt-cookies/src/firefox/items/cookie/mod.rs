use chrono::{DateTime, Utc};

use self::entities::moz_cookies;
use super::I64ToMozTime;
use crate::browser::cookies::{CookiesInfo, SameSite};

pub mod dao;
pub mod entities;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct MozCookie {
    pub id: i32,
    pub origin_attributes: String,
    pub name: String,
    pub value: String,
    pub host: String,
    pub path: String,
    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expires>
    pub expiry: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub creation_time: Option<DateTime<Utc>>,
    pub is_secure: bool,
    pub is_http_only: bool,
    pub in_browser_element: i32,
    pub same_site: SameSite,
    // pub raw_same_site: i32,
    pub scheme_map: i32,
}
#[cfg(feature = "reqwest")]
impl TryFrom<MozCookie> for reqwest::header::HeaderValue {
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(value: MozCookie) -> Result<Self, Self::Error> {
        Self::from_str(&value.set_cookie_header())
    }
}
#[cfg(feature = "reqwest")]
impl FromIterator<MozCookie> for reqwest::cookie::Jar {
    fn from_iter<T: IntoIterator<Item = MozCookie>>(iter: T) -> Self {
        let jar = Self::default();
        for cookie in iter {
            let set_cookie = cookie.set_cookie_header();
            if let Ok(url) = reqwest::Url::parse(&cookie.url()) {
                jar.add_cookie_str(&set_cookie, &url);
            }
        }
        jar
    }
}

impl CookiesInfo for MozCookie {
    fn is_http_only(&self) -> bool {
        self.is_http_only
    }
    fn same_site(&self) -> SameSite {
        self.same_site
    }
    fn is_secure(&self) -> bool {
        self.is_secure
    }
    fn expiry(&self) -> Option<String> {
        self.expiry
            .map(|time| time.to_rfc2822())
    }
    fn domain(&self) -> &str {
        &self.host
    }
    fn value(&self) -> &str {
        &self.value
    }
    fn path(&self) -> &str {
        &self.path
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn creation(&self) -> Option<DateTime<Utc>> {
        self.creation_time
    }

    fn expires(&self) -> Option<DateTime<Utc>> {
        self.expiry
    }
}

impl From<moz_cookies::Model> for MozCookie {
    fn from(value: moz_cookies::Model) -> Self {
        Self {
            id: value.id,
            origin_attributes: value.origin_attributes,
            name: value.name.unwrap_or_default(),
            value: value.value.unwrap_or_default(),
            host: value.host.unwrap_or_default(),
            path: value.path.unwrap_or_default(),
            expiry: value
                .expiry
                .unwrap_or_default()
                .secs_to_moz_utc(),
            last_accessed: value
                .last_accessed
                .unwrap_or_default()
                .micros_to_moz_utc(),
            creation_time: value
                .creation_time
                .unwrap_or_default()
                .micros_to_moz_utc(),
            is_secure: value
                .is_secure
                .is_some_and(|v| v != 0),
            is_http_only: value
                .is_http_only
                .is_some_and(|v| v != 0),
            in_browser_element: value
                .in_browser_element
                .unwrap_or_default(),
            same_site: value.same_site.into(),
            // raw_same_site: value
            //     .raw_same_site
            //     .unwrap_or_default(),
            scheme_map: value
                .scheme_map
                .unwrap_or_default(),
        }
    }
}
