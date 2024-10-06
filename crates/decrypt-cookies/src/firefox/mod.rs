pub mod items;

use std::path::PathBuf;

use chrono::Utc;
use miette::{IntoDiagnostic, Result};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition};

pub use self::items::cookie::entities::moz_cookies::{
    Column as MozCookiesCol, ColumnIter as MozCookiesColIter,
};
use self::items::{
    cookie::{dao::CookiesQuery, MozCookies},
    I64ToMozTime,
};
use crate::browser::{cookies::LeetCodeCookies, info::FirefoxInfo};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxGetter<T: FirefoxInfo + Send + Sync> {
    browser: T,
    cookies_query: CookiesQuery,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder<T: FirefoxInfo> {
    browser: T,
    cookies_path: Option<PathBuf>,
}

impl<T: FirefoxInfo + Send + Sync> FirefoxBuilder<T> {
    pub const fn new(browser: T) -> Self {
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
    pub async fn build(mut self) -> Result<FirefoxGetter<T>> {
        let temp_cookies_path = self.browser.cookies_temp();
        tokio::fs::copy(
            self.cookies_path
                .take()
                .unwrap_or_else(|| self.browser.cookies()),
            &temp_cookies_path,
        )
        .await
        .into_diagnostic()?;

        let query = CookiesQuery::new(temp_cookies_path).await?;

        Ok(FirefoxGetter {
            browser: self.browser,
            cookies_query: query,
        })
    }
}

impl<T: FirefoxInfo + Send + Sync> FirefoxGetter<T> {
    /// filter by condition
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use decrypt_cookies::{firefox::MozCookiesCol, Browser, FirefoxBuilder,ColumnTrait};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let ffget = FirefoxBuilder::new(Firefox::new().unwrap())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = ffget
    ///         .get_cookies_filter(MozCookiesCol::Host.contains("mozilla.com"))
    ///         .await
    ///         .unwrap_or_default();
    /// }
    /// ```
    pub async fn get_cookies_filter<F>(&self, filter: F) -> Result<Vec<MozCookies>>
    where
        F: IntoCondition + Send,
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
                MozCookiesCol::Host
                    .contains(host)
                    .and(
                        MozCookiesCol::Name
                            .eq("csrftoken")
                            .or(MozCookiesCol::Name.eq("LEETCODE_SESSION")),
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

    pub fn browser(&self) -> &str {
        self.browser.browser()
    }

    pub fn info(&self) -> &impl crate::browser::info::FirefoxInfo {
        &self.browser
    }
}
