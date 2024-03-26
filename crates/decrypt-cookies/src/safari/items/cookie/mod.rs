use std::path::Path;

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
    pub binary_cookies: BinaryCookies,
}

impl CookiesGetter {
    /// `MacOs12` cookies path
    const COOKIES: &'static str =
        "Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies";
    /// < `MacOs12` cookies path
    const COOKIES_OLD: &'static str = "Library/Cookies/Cookies.binarycookies";

    pub async fn build<T>(cookies_path: Option<T>) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        let mut cookie_path = dirs::home_dir().expect("get home dir failed");
        if let Some(path) = cookies_path {
            cookie_path.push(path);
            println!("{:?}", cookie_path);
        }
        else {
            cookie_path.push(Self::COOKIES);
        }
        if !cookie_path.exists() {
            cookie_path = dirs::home_dir().expect("get home dir failed");
            cookie_path.push(Self::COOKIES_OLD);
        }
        let content = tokio::fs::read(cookie_path)
            .await
            .into_diagnostic()?;
        let binary_cookies = BinaryCookies::parse(&content)?;

        Ok(Self { binary_cookies })
    }
    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        let mut lc_cookies = LeetCodeCookies::default();
        let filtered = self
            .binary_cookies
            .filter_by(|v| v.domain().contains(host));
        for ck in filtered {
            if ck.name() == "csrftoken" {
                lc_cookies.csrf = ck.value().to_owned();
            }
            else if ck.name() == "LEETCODE_SESSION" {
                lc_cookies.session = ck.value().to_owned();
            }
        }
        lc_cookies
    }
    pub fn all_cookies(&self) -> Vec<&SafariCookie> {
        self.binary_cookies
            .iter_cookies()
            .collect()
    }
}
