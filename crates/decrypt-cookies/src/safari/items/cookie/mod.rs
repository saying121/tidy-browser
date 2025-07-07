use std::{path::PathBuf, sync::Arc};

use binary_cookies::{cookie::Cookie, tokio::DecodeBinaryCookie};
use chrono::{prelude::Utc, DateTime};

use crate::{
    browser::cookies::{CookiesInfo, LeetCodeCookies},
    prelude::cookies::SameSite,
};

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum CookiesGetterError {
    #[error(transparent)]
    Parse(#[from] binary_cookies::error::ParseError),
    #[error("{source}, path: {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
}

type Result<T> = std::result::Result<T, CookiesGetterError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[expect(
    clippy::exhaustive_structs,
    reason = "Breaking change with Binarycookies format"
)]
pub struct SafariCookie {
    pub version: u32,

    pub flags: u32,
    pub port: Option<u16>,
    pub comment: Option<String>,
    pub domain: String,
    pub name: String,
    pub path: String,
    pub value: String,

    pub expires: Option<DateTime<Utc>>,
    pub creation: Option<DateTime<Utc>>,
    pub same_site: SameSite,
    pub is_secure: bool,
    pub is_http_only: bool,
}

impl From<Cookie> for SafariCookie {
    fn from(value: Cookie) -> Self {
        Self {
            version: value.version,
            flags: value.flags,
            port: value.port,
            comment: value
                .comment
                .map(|v| v.to_string()),
            domain: value.domain.to_string(),
            name: value.name.to_string(),
            path: value.path.to_string(),
            value: value.value.to_string(),
            expires: value.expires,
            creation: value.creation,
            same_site: value.same_site.into(),
            is_secure: value.is_secure,
            is_http_only: value.is_http_only,
        }
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
        self.expires
            .map(|expiry| expiry.to_rfc2822())
    }
    fn is_secure(&self) -> bool {
        self.is_secure
    }
    fn is_http_only(&self) -> bool {
        self.is_http_only
    }
    fn same_site(&self) -> SameSite {
        self.same_site
    }
}

#[non_exhaustive]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct CookiesGetter {
    cookies: Vec<SafariCookie>,
}

impl CookiesGetter {
    /// `MacOs12` cookies path
    const COOKIES: &'static str =
        "Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies";
    /// < `MacOs12` cookies path
    const COOKIES_OLD: &'static str = "Library/Cookies/Cookies.binarycookies";

    pub async fn build<T>(cookies_path: Option<T>) -> Result<Self>
    where
        T: Into<PathBuf> + Send,
    {
        let mut cookie_path;
        if let Some(path) = cookies_path {
            cookie_path = path.into();
        }
        else {
            cookie_path = dirs::home_dir().expect("get home dir failed");
            cookie_path.push(Self::COOKIES);
            if !cookie_path.exists() {
                cookie_path = dirs::home_dir().expect("get home dir failed");
                cookie_path.push(Self::COOKIES_OLD);
            }
        }

        let file = binary_cookies::tokio::RandomAccessFile::open(&cookie_path)
            .map_err(|e| CookiesGetterError::Io { path: cookie_path, source: e })?;
        let file = Arc::new(file);

        let bch = file.decode().await?;
        let (pages_handle, _meta_decoder) = bch.into_handles();
        let mut cookies = vec![];
        for mut pd in pages_handle.decoders() {
            let ch = pd.decode().await?;
            for mut c in ch.decoders() {
                let cookie = c.decode().await?;
                cookies.push(cookie.into());
            }
        }

        Ok(Self { cookies })
    }

    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        let mut lc_cookies = LeetCodeCookies::default();
        for ck in self.cookies.iter().filter(|v| {
            v.domain().contains(host)
                && (v.name().eq("csrftoken") || v.name().eq("LEETCODE_SESSION"))
        }) {
            if ck.name() == "csrftoken" {
                if let Some(expires) = ck.expires {
                    if Utc::now() > expires {
                        lc_cookies.expiry = true;
                        break;
                    }
                }
                ck.value()
                    .clone_into(&mut lc_cookies.csrf);
            }
            else if ck.name() == "LEETCODE_SESSION" {
                if let Some(expires) = ck.expires {
                    if Utc::now() > expires {
                        lc_cookies.expiry = true;
                        break;
                    }
                }
                ck.value()
                    .clone_into(&mut lc_cookies.session);
            }
        }
        lc_cookies
    }
    pub fn all_cookies(&self) -> Vec<&SafariCookie> {
        self.cookies.iter().collect()
    }
    pub fn iter_cookies(&self) -> impl Iterator<Item = &SafariCookie> {
        self.cookies.iter()
    }
}
