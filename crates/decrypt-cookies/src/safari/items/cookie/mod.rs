use std::path::PathBuf;

use chrono::prelude::Utc;
use tokio::{fs, task::spawn_blocking};

use crate::{
    browser::cookies::{CookiesInfo, LeetCodeCookies},
    utils::binary_cookies::{BinaryCookies, SafariCookie},
};

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum CookiesGetterError {
    #[error("Parse cookies failed")]
    Parse(#[from] crate::utils::binary_cookies::ParseError),
    #[error("Io error")]
    Io(#[from] std::io::Error),
    #[error("Tokio task failed")]
    Task(#[from] tokio::task::JoinError),
}

type Result<T> = std::result::Result<T, CookiesGetterError>;

#[non_exhaustive]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct CookiesGetter {
    pub binary_cookies: BinaryCookies,
}

impl CookiesGetter {
    pub fn into_inner(self) -> BinaryCookies {
        self.binary_cookies
    }
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
        let content = fs::read(cookie_path).await?;
        let binary_cookies = spawn_blocking(move || BinaryCookies::parse(&content)).await??;

        Ok(Self { binary_cookies })
    }
    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        let mut lc_cookies = LeetCodeCookies::default();
        for ck in self
            .binary_cookies
            .iter_cookies()
            .filter(|v| {
                v.domain().contains(host)
                    && (v.name().eq("csrftoken") || v.name().eq("LEETCODE_SESSION"))
            })
        {
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
    pub const fn binary_cookies(&self) -> &BinaryCookies {
        &self.binary_cookies
    }
    pub fn all_cookies(&self) -> Vec<&SafariCookie> {
        self.binary_cookies
            .iter_cookies()
            .collect()
    }
    pub fn iter_cookies(&self) -> impl Iterator<Item = &SafariCookie> {
        self.binary_cookies.iter_cookies()
    }
}
