pub mod items;

use std::{marker::PhantomData, path::PathBuf};

use chrono::Utc;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition, DbErr};

pub use self::items::cookie::entities::moz_cookies::{
    Column as MozCookiesCol, ColumnIter as MozCookiesColIter,
};
use self::items::{
    cookie::{dao::CookiesQuery, MozCookies},
    I64ToMozTime,
};
use crate::browser::cookies::LeetCodeCookies;

type Result<T> = std::result::Result<T, DbErr>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxGetter<T> {
    pub(crate) cookies_query: CookiesQuery,
    pub(crate) __browser: PhantomData<T>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder<'a, T> {
    pub(crate) init: Option<PathBuf>,
    pub(crate) profile: Option<&'a str>,
    pub(crate) __browser: PhantomData<T>,
}

impl<T: Send + Sync> FirefoxGetter<T> {
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
}
