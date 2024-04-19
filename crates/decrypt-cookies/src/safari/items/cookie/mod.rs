use std::path::PathBuf;

use chrono::prelude::Utc;
use miette::{IntoDiagnostic, Result};

use crate::{
    safari::utils::binary_cookies::{BinaryCookies, SafariCookie},
    LeetCodeCookies,
};

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
        T: Into<PathBuf>,
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
        let content = tokio::fs::read(cookie_path)
            .await
            .into_diagnostic()?;
        let binary_cookies = BinaryCookies::parse(&content)?;

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
                if Utc::now() > ck.expires {
                    lc_cookies.expiry = true;
                    break;
                }

                lc_cookies.csrf = ck.value().to_owned();
            }
            else if ck.name() == "LEETCODE_SESSION" {
                if Utc::now() > ck.expires {
                    lc_cookies.expiry = true;
                    break;
                }

                lc_cookies.session = ck.value().to_owned();
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
