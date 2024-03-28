pub mod items;
pub mod path;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod win;

use std::path::PathBuf;

pub use items::cookie::entities::moz_cookies::{
    Column as MozCookiesColumn, ColumnIter as MozCookiesColumnIter,
};
use miette::Result;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition};

#[cfg(target_os = "linux")]
use self::linux::path::LinuxFFBase;
#[cfg(target_os = "macos")]
use self::macos::path::MacFFBase;
#[cfg(target_os = "windows")]
use self::win::path::WinFFBase;
use self::{
    items::cookie::{dao::CookiesQuery, MozCookies},
    path::FFPath,
};
use crate::{Browser, LeetCodeCookies};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxGetter {
    browser:       Browser,
    cookies_query: CookiesQuery,

    #[cfg(target_os = "linux")]
    path: linux::path::LinuxFFBase,
    #[cfg(target_os = "macos")]
    path: macos::path::MacFFBase,
    #[cfg(target_os = "windows")]
    path: win::path::WinFFBase,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder {
    browser:      Browser,
    cookies_path: Option<PathBuf>,
}

impl FirefoxBuilder {
    pub fn new(browser: Browser) -> Self {
        Self { browser, ..Default::default() }
    }
    /// set `cookies_path`
    pub fn cookies_path<P>(&mut self, ck_path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.cookies_path = Some(ck_path.into());
        self
    }
    pub async fn build(&mut self) -> Result<FirefoxGetter> {
        #[cfg(target_os = "linux")]
        let path = LinuxFFBase::new(self.browser).await?;
        #[cfg(target_os = "macos")]
        let path = MacFFBase::new(self.browser).await?;
        #[cfg(target_os = "windows")]
        let path = WinFFBase::new(self.browser).await?;

        let query = CookiesQuery::new(
            self.cookies_path
                .take()
                .unwrap_or_else(|| path.cookies()),
        )
        .await?;
        Ok(FirefoxGetter {
            browser: self.browser,
            cookies_query: query,
            path,
        })
    }
}

impl FirefoxGetter {
    /// filter by condition
    ///
    /// # Example
    /// ```rust
    /// use decrypt_cookies::{firefox::MozCookiesColumn, Browser, FirefoxGetter};
    /// use sea_orm::ColumnTrait;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let ffget = FirefoxGetter::build(Browser::Firefox)
    ///         .await
    ///         .unwrap();
    ///     let res = ffget
    ///         .get_cookies_filter(MozCookiesColumn::Host.contains("mozilla.com"))
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn get_cookies_filter<F>(&self, filter: F) -> Result<Vec<MozCookies>>
    where
        F: IntoCondition,
    {
        let res = self
            .cookies_query
            .query_cookie_filter(filter)
            .await?;
        let res = res
            .into_par_iter()
            .map(MozCookies::from)
            .collect();
        Ok(res)
    }

    pub async fn get_cookies_all(&self) -> Result<Vec<MozCookies>> {
        let res = self
            .cookies_query
            .query_all_cookie()
            .await?;
        let res = res
            .into_par_iter()
            .map(MozCookies::from)
            .collect();
        Ok(res)
    }
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<MozCookies>> {
        let res = self
            .cookies_query
            .query_cookie_by_host(host)
            .await?;
        let res = res
            .into_par_iter()
            .map(MozCookies::from)
            .collect();
        Ok(res)
    }

    /// get session csrf for leetcode
    pub async fn get_session_csrf(&self, host: &str) -> Result<LeetCodeCookies> {
        let cookies = self
            .cookies_query
            .query_cookie_filter(
                MozCookiesColumn::Host
                    .contains(host)
                    .and(
                        MozCookiesColumn::Name
                            .eq("csrftoken")
                            .or(MozCookiesColumn::Name.eq("LEETCODE_SESSION")),
                    ),
            )
            .await?;

        let mut res = LeetCodeCookies::default();

        for cookie in cookies {
            if let Some(s) = cookie.name {
                if s == "csrftoken" {
                    res.csrf = cookie.value.unwrap_or_default();
                }
                else if s == "LEETCODE_SESSION" {
                    res.session = cookie.value.unwrap_or_default();
                }
            }
        }
        Ok(res)
    }
}

impl FirefoxGetter {
    pub const fn browser(&self) -> Browser {
        self.browser
    }

    #[cfg(target_os = "linux")]
    pub const fn path(&self) -> &LinuxFFBase {
        &self.path
    }
    #[cfg(target_os = "macos")]
    pub const fn path(&self) -> &MacFFBase {
        &self.path
    }
    #[cfg(target_os = "windows")]
    pub const fn path(&self) -> &WinFFBase {
        &self.path
    }
}
