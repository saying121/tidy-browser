use chrono::prelude::*;

use self::cookie_entities::cookies;
use super::I64ToChromiumDateTime;
use crate::browser::cookies::{CookiesInfo, SameSite};

pub mod cookie_dao;
pub mod cookie_entities;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct ChromiumCookie {
    pub creation_utc: Option<DateTime<Utc>>,
    pub host_key: String,
    pub top_frame_site_key: String,
    pub name: String,
    pub value: String,
    pub decrypted_value: Option<String>,
    pub path: String,
    /// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expires>
    pub expires_utc: Option<DateTime<Utc>>,
    pub is_secure: bool,
    pub is_httponly: bool,
    pub last_access_utc: Option<DateTime<Utc>>,
    pub has_expires: bool,
    pub is_persistent: bool,
    pub priority: i32,
    pub same_site: SameSite,
    pub source_scheme: i32,
    pub source_port: i32,
    pub last_update_utc: Option<DateTime<Utc>>,
}

#[cfg(feature = "reqwest")]
impl TryFrom<ChromiumCookie> for reqwest::header::HeaderValue {
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(value: ChromiumCookie) -> Result<Self, Self::Error> {
        Self::from_str(&value.set_cookie_header())
    }
}
#[cfg(feature = "reqwest")]
impl FromIterator<ChromiumCookie> for reqwest::cookie::Jar {
    fn from_iter<T: IntoIterator<Item = ChromiumCookie>>(iter: T) -> Self {
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

impl CookiesInfo for ChromiumCookie {
    fn name(&self) -> &str {
        &self.name
    }
    fn path(&self) -> &str {
        &self.path
    }
    fn value(&self) -> &str {
        self.decrypted_value
            .as_ref()
            .unwrap_or(&self.value)
    }
    fn domain(&self) -> &str {
        &self.host_key
    }
    fn expiry(&self) -> Option<String> {
        self.expires_utc
            .map(|expir| expir.to_rfc2822())
    }
    fn is_secure(&self) -> bool {
        self.is_secure
    }
    fn same_site(&self) -> SameSite {
        self.same_site
    }
    fn is_http_only(&self) -> bool {
        self.is_httponly
    }

    fn creation(&self) -> Option<DateTime<Utc>> {
        self.creation_utc
    }

    fn expires(&self) -> Option<DateTime<Utc>> {
        self.expires_utc
    }
}

impl From<cookies::Model> for ChromiumCookie {
    fn from(value: cookies::Model) -> Self {
        Self {
            creation_utc: value
                .creation_utc
                .micros_to_chromium_utc(),
            host_key: value.host_key,
            top_frame_site_key: value.top_frame_site_key,
            name: value.name,
            value: value.value,
            decrypted_value: None,
            path: value.path,
            expires_utc: value
                .expires_utc
                .micros_to_chromium_utc(),
            is_secure: value.is_secure != 0,
            is_httponly: value.is_httponly != 0,
            last_access_utc: value
                .last_access_utc
                .micros_to_chromium_utc(),
            has_expires: value.has_expires != 0,
            is_persistent: value.is_persistent != 0,
            priority: value.priority,
            same_site: value.samesite.into(),
            source_scheme: value.source_scheme,
            source_port: value.source_port,
            last_update_utc: value
                .last_update_utc
                .micros_to_chromium_utc(),
        }
    }
}
