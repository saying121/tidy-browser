use std::{
    mem::replace,
    path::{Path, PathBuf},
};

use miette::{IntoDiagnostic, Result};

use crate::{
    safari::utils::binary_cookies::{BinaryCookies, SafariCookie},
    LeetCodeCookies,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CookiesGetter {
    pub binary_cookies: Option<BinaryCookies>,
}

impl CookiesGetter {
    pub fn into_inner(&mut self) -> Option<BinaryCookies> {
        self.binary_cookies.take()
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

        Ok(Self {
            binary_cookies: Some(binary_cookies),
        })
    }
    pub fn get_session_csrf(&self, host: &str) -> Option<LeetCodeCookies> {
        let mut lc_cookies = LeetCodeCookies::default();
        for ck in self
            .binary_cookies
            .as_ref()?
            .iter_cookies()
        {
            if ck.domain().contains(host) {
                if ck.name() == "csrftoken" {
                    lc_cookies.csrf = ck.value().to_owned();
                }
                else if ck.name() == "LEETCODE_SESSION" {
                    lc_cookies.session = ck.value().to_owned();
                }
            }
        }
        Some(lc_cookies)
    }
    pub fn ref_binary_cookies(&self) -> Option<&BinaryCookies> {
        self.binary_cookies.as_ref()
    }
    pub fn all_cookies(&self) -> Option<Vec<&SafariCookie>> {
        Some(
            self.ref_binary_cookies()?
                .iter_cookies()
                .collect(),
        )
    }
    pub fn iter_cookies(&self) -> Option<impl Iterator<Item = &SafariCookie>> {
        Some(
            self.ref_binary_cookies()?
                .iter_cookies(),
        )
    }
}
