pub mod builder;
pub mod items;

use std::{
    fmt::Display,
    future::Future,
    marker::{PhantomData, Sync},
};

use chrono::Utc;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition, DbErr};
use snafu::{Location, ResultExt, Snafu};

pub use self::items::cookie::{
    entities::moz_cookies::{Column as MozCookiesCol, ColumnIter as MozCookiesColIter},
    MozCookie,
};
use self::items::{cookie::dao::CookiesQuery, I64ToMozTime};
use crate::browser::{cookies::LeetCodeCookies, FirefoxPath};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum FirefoxError {
    #[snafu(display("{source}\n@:{location}"))]
    Db {
        source: DbErr,
        #[snafu(implicit)]
        location: Location,
    },
}

type Result<T> = std::result::Result<T, FirefoxError>;

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
pub struct FirefoxCookieGetter<T> {
    pub(crate) cookies_query: CookiesQuery,
    pub(crate) __browser: PhantomData<T>,
}

macro_rules! impl_display {
    ($($s:ident),* $(,)?) => {
        $(
            impl<B: FirefoxPath> Display for $s<B> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(B::NAME)
                }
            }
        )*
    };
}
impl_display![FirefoxGetter, FirefoxCookieGetter,];

impl<B> SealedCookies for FirefoxGetter<B> {
    fn cookies_query(&self) -> &CookiesQuery {
        &self.cookies_query
    }
}
impl<B> SealedCookies for FirefoxCookieGetter<B> {
    fn cookies_query(&self) -> &CookiesQuery {
        &self.cookies_query
    }
}
impl<B> GetCookies for FirefoxGetter<B> {}
impl<B> GetCookies for FirefoxCookieGetter<B> {}

trait SealedCookies {
    fn cookies_query(&self) -> &CookiesQuery;
}

#[expect(private_bounds, reason = "impl details")]
pub trait GetCookies: SealedCookies {
    /// filter by condition
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use decrypt_cookies::{firefox::MozCookiesCol, Browser, FirefoxBuilder, ColumnTrait};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let ffget = FirefoxBuilder::new(Firefox::new().unwrap())
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = ffget
    ///         .cookies_filter(MozCookiesCol::Host.contains("mozilla.com"))
    ///         .await
    ///         .unwrap_or_default();
    /// }
    /// ```
    fn cookies_filter<F>(&self, filter: F) -> impl Future<Output = Result<Vec<MozCookie>>> + Send
    where
        F: IntoCondition + Send,
        Self: Sync,
    {
        async {
            let res = self
                .cookies_query()
                .query_cookie_filter(filter)
                .await
                .context(DbSnafu)?;
            let res = res
                .into_par_iter()
                .map(MozCookie::from)
                .collect();
            Ok(res)
        }
    }

    fn cookies_all(&self) -> impl Future<Output = Result<Vec<MozCookie>>> + Send
    where
        Self: Sync,
    {
        async {
            let res = self
                .cookies_query()
                .query_all_cookie()
                .await
                .context(DbSnafu)?;
            let res = res
                .into_par_iter()
                .map(MozCookie::from)
                .collect();
            Ok(res)
        }
    }

    /// Filter cookies by host
    #[doc(alias = "cookies_by_domain", alias = "cookies_by_url")]
    fn cookies_by_host<H>(&self, host: H) -> impl Future<Output = Result<Vec<MozCookie>>> + Send
    where
        Self: Sync,
        H: AsRef<str> + Send + Sync,
    {
        async move {
            let res = self
                .cookies_query()
                .query_cookie_by_host(host.as_ref())
                .await
                .context(DbSnafu)?;
            let res = res
                .into_par_iter()
                .map(MozCookie::from)
                .collect();
            Ok(res)
        }
    }

    /// get session csrf for leetcode
    fn get_session_csrf<H>(&self, host: H) -> impl Future<Output = Result<LeetCodeCookies>> + Send
    where
        Self: Sync,
        H: AsRef<str> + Send + Sync,
    {
        async move {
            let cookies = self
                .cookies_query()
                .query_cookie_filter(
                    MozCookiesCol::Host
                        .contains(host.as_ref())
                        .and(
                            MozCookiesCol::Name
                                .eq("csrftoken")
                                .or(MozCookiesCol::Name.eq("LEETCODE_SESSION")),
                        ),
                )
                .await
                .context(DbSnafu)?;

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
}
