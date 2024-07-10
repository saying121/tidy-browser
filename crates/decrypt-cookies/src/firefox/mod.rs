pub mod items;
use std::path::PathBuf;

use chrono::Utc;
pub use items::cookie::entities::moz_cookies::{
    Column as MozCookiesColumn, ColumnIter as MozCookiesColumnIter,
};
use miette::{IntoDiagnostic, Result};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition};

use self::items::{
    cookie::{dao::CookiesQuery, MozCookies},
    I64ToMozTime,
};
use crate::{browser::info::FfInfo, Browser, LeetCodeCookies};

cfg_if::cfg_if!(
    if #[cfg(target_os = "linux")] {
        use crate::browser::info::linux::LinuxFFBase;
    } else if #[cfg(target_os = "macos")] {
        use crate::browser::info::macos::MacFFBase;
    } else if #[cfg(target_os = "windows")] {
        use crate::browser::info::win::WinFFBase;
    }
);

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxGetter {
    browser:       Browser,
    cookies_query: CookiesQuery,

    #[cfg(target_os = "linux")]
    info: LinuxFFBase,
    #[cfg(target_os = "windows")]
    info: WinFFBase,
    #[cfg(target_os = "macos")]
    info: MacFFBase,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder {
    browser:      Browser,
    cookies_path: Option<PathBuf>,
}

impl FirefoxBuilder {
    /// # Panics
    ///
    /// When you use not Firefox based browser
    pub fn new(browser: Browser) -> Self {
        assert!(
            browser.is_firefox_base(),
            "Firefox based not support: {browser}"
        );
        Self { browser, cookies_path: None }
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
        let info = LinuxFFBase::new(self.browser).await?;
        #[cfg(target_os = "macos")]
        let info = MacFFBase::new(self.browser).await?;
        #[cfg(target_os = "windows")]
        let info = WinFFBase::new(self.browser).await?;

        let temp_cookies_path = info.cookies_temp();
        tokio::fs::copy(
            self.cookies_path
                .take()
                .unwrap_or_else(|| info.cookies()),
            &temp_cookies_path,
        )
        .await
        .into_diagnostic()?;

        let query = CookiesQuery::new(temp_cookies_path).await?;

        Ok(FirefoxGetter {
            browser: self.browser,
            cookies_query: query,
            info,
        })
    }
}

impl FirefoxGetter {
    /// filter by condition
    ///
    /// # Example
    /// ```rust,ignore
    /// use decrypt_cookies::{firefox::MozCookiesColumn, Browser, FirefoxBuilder};
    /// use sea_orm::ColumnTrait;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let ffget = FirefoxBuilder::new(Browser::Firefox)
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = ffget
    ///         .get_cookies_filter(MozCookiesColumn::Host.contains("mozilla.com"))
    ///         .await
    ///         .unwrap_or_default();
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
                    let expir = cookie
                        .expiry
                        .unwrap_or_default()
                        .secs_to_moz_utc();
                    if let Some(expir) = expir {
                        if Utc::now() > expir {
                            res.expiry = true;
                            break;
                        }
                    }

                    res.csrf = cookie.value.unwrap_or_default();
                }
                else if s == "LEETCODE_SESSION" {
                    let expir = cookie
                        .expiry
                        .unwrap_or_default()
                        .secs_to_moz_utc();
                    if let Some(expir) = expir {
                        if Utc::now() > expir {
                            res.expiry = true;
                            break;
                        }
                    }

                    res.session = cookie.value.unwrap_or_default();
                }
            }
        }
        Ok(res)
    }

    pub const fn browser(&self) -> Browser {
        self.browser
    }

    pub fn info(&self) -> &impl crate::browser::info::FfInfo {
        &self.info
    }
}
