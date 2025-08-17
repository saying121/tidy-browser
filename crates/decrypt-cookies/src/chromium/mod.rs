pub mod builder;
pub(crate) mod items;
use std::{
    fmt::Display,
    marker::{PhantomData, Sync},
};

use chromium_crypto::{Decrypter, Which};
use chrono::prelude::Utc;
use items::cookie::cookie_entities::cookies;
pub use items::{
    cookie::{
        cookie_entities::cookies::{
            Column as ChromiumCookieCol, ColumnIter as ChromiumCookieColIter,
        },
        ChromiumCookie,
    },
    passwd::{
        login_data_entities::logins::{Column as ChromiumLoginCol, Column as ChromiumLoginColIter},
        LoginData,
    },
};
use rayon::prelude::*;
use sea_orm::{sea_query::IntoCondition, ColumnTrait, DbErr};
use snafu::{Location, ResultExt, Snafu};
use tokio::task::{self, JoinError};

use crate::{
    browser::{cookies::LeetCodeCookies, ChromiumPath},
    chromium::items::{
        cookie::cookie_dao::CookiesQuery,
        passwd::{login_data_dao::LoginDataQuery, login_data_entities::logins},
        I64ToChromiumDateTime,
    },
};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum ChromiumError {
    #[snafu(display("{source}\n@:{location}"))]
    Task {
        source: JoinError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Db {
        source: DbErr,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Decrypt {
        source: chromium_crypto::error::CryptoError,
        #[snafu(implicit)]
        location: Location,
    },
}

type Result<T> = std::result::Result<T, ChromiumError>;

/// Chromium based, get cookies, etc. and decrypt
///
/// Initialize it with `ChromiumBuilder`
///
/// # Example
/// ```rust, ignore
/// let getter = ChromiumBuilder::new(Chromium::new())
///     .build()
///     .await?;
/// getter
///     .get_cookies_session_csrf(host)
///     .await?
/// ```
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumGetter<T> {
    pub(crate) cookies_query: CookiesQuery,
    pub(crate) login_data_query: LoginDataQuery,
    pub(crate) login_data_for_account_query: Option<LoginDataQuery>,
    pub(crate) crypto: Decrypter,
    pub(crate) __browser: PhantomData<T>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumCookieGetter<T> {
    pub(crate) cookies_query: CookiesQuery,
    pub(crate) crypto: Decrypter,
    pub(crate) __browser: PhantomData<T>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumLoginGetter<T> {
    pub(crate) login_data_query: LoginDataQuery,
    pub(crate) login_data_for_account_query: Option<LoginDataQuery>,
    pub(crate) crypto: Decrypter,
    pub(crate) __browser: PhantomData<T>,
}

macro_rules! impl_display {
    ($($browser:ident),* $(,)?) => {
        $(
            impl<B: ChromiumPath> Display for $browser<B> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(B::NAME)
                }
            }
        )*
    };
}
impl_display![ChromiumGetter, ChromiumCookieGetter, ChromiumLoginGetter,];

impl<B> SealedCrypto for ChromiumGetter<B> {
    fn crypto(&self) -> &Decrypter {
        &self.crypto
    }
}
impl<B> SealedCrypto for ChromiumCookieGetter<B> {
    fn crypto(&self) -> &Decrypter {
        &self.crypto
    }
}
impl<B> SealedCrypto for ChromiumLoginGetter<B> {
    fn crypto(&self) -> &Decrypter {
        &self.crypto
    }
}

impl<B> SealedCookies for ChromiumCookieGetter<B> {
    fn cookies_query(&self) -> &CookiesQuery {
        &self.cookies_query
    }
}

impl<B> SealedCookies for ChromiumGetter<B> {
    fn cookies_query(&self) -> &CookiesQuery {
        &self.cookies_query
    }
}

impl<B> SealedLogins for ChromiumGetter<B> {
    fn login_data_query(&self) -> &LoginDataQuery {
        &self.login_data_query
    }

    fn login_data_for_account_query(&self) -> Option<&LoginDataQuery> {
        self.login_data_for_account_query
            .as_ref()
    }
}

impl<B> SealedLogins for ChromiumLoginGetter<B> {
    fn login_data_query(&self) -> &LoginDataQuery {
        &self.login_data_query
    }

    fn login_data_for_account_query(&self) -> Option<&LoginDataQuery> {
        self.login_data_for_account_query
            .as_ref()
    }
}

impl<B> GetCookies for ChromiumGetter<B> {}
impl<B> GetCookies for ChromiumCookieGetter<B> {}

impl<B> GetLogins for ChromiumGetter<B> {}
impl<B> GetLogins for ChromiumLoginGetter<B> {}

trait SealedCrypto {
    fn crypto(&self) -> &Decrypter;
    fn par_decrypt_logins(
        &self,
        raw: Vec<logins::Model>,
    ) -> impl std::future::Future<Output = Result<Vec<LoginData>>> + std::marker::Send
    where
        Self: Sync,
    {
        async {
            let crypto = self.crypto().clone();

            task::spawn_blocking(move || {
                raw.into_par_iter()
                    .map(|mut v| {
                        let res = v
                            .password_value
                            .as_mut()
                            .and_then(|v| {
                                crypto
                                    .decrypt(v, Which::Login)
                                    .ok()
                            });

                        let mut login_data = LoginData::from(v);
                        login_data.password_value = res;
                        login_data
                    })
                    .collect()
            })
            .await
            .context(TaskSnafu)
        }
    }

    /// parallel decrypt cookies
    /// and not blocking scheduling
    fn par_decrypt_ck(
        &self,
        raw: Vec<cookies::Model>,
    ) -> impl std::future::Future<Output = Result<Vec<ChromiumCookie>>> + Send + Sync
    where
        Self: Sync,
    {
        async {
            let crypto = self.crypto().clone();

            let decrypted_ck = task::spawn_blocking(move || {
                raw.into_par_iter()
                    .map(|mut v| {
                        let res = crypto
                            .decrypt(&mut v.encrypted_value, Which::Cookie)
                            .ok();
                        let mut cookies = ChromiumCookie::from(v);
                        cookies.decrypted_value = res;
                        cookies
                    })
                    .collect()
            })
            .await
            .context(TaskSnafu)?;
            Ok(decrypted_ck)
        }
    }
}

trait SealedCookies {
    fn cookies_query(&self) -> &CookiesQuery;
}

trait SealedLogins {
    fn login_data_query(&self) -> &LoginDataQuery;
    fn login_data_for_account_query(&self) -> Option<&LoginDataQuery>;
}

#[expect(private_bounds, reason = "impl details")]
pub trait GetLogins: SealedCrypto + SealedLogins {
    /// contains passwords
    ///
    /// # Example:
    ///
    /// ```rust
    /// use decrypt_cookies::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let edge_getter = ChromiumBuilder::<Chrome>::new()
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = edge_getter
    ///         .logins_filter(ChromiumLoginCol::OriginUrl.contains("google.com"))
    ///         .await
    ///         .unwrap_or_default();
    ///     dbg!(res);
    /// }
    /// ```
    fn logins_filter<F>(
        &self,
        filter: F,
    ) -> impl std::future::Future<Output = Result<Vec<LoginData>>> + Send
    where
        F: IntoCondition + Send + Clone,
        Self: Sync,
    {
        async {
            let mut raw_login = self
                .login_data_query()
                .query_login_dt_filter(filter.clone())
                .await
                .context(DbSnafu)?;
            if raw_login.is_empty() {
                if let Some(query) = &self.login_data_for_account_query() {
                    raw_login = query
                        .query_login_dt_filter(filter)
                        .await
                        .context(DbSnafu)?;
                }
            }
            self.par_decrypt_logins(raw_login)
                .await
        }
    }

    /// Filter by host
    fn logins_by_host<H>(
        &self,
        host: H,
    ) -> impl std::future::Future<Output = Result<Vec<LoginData>>> + Send
    where
        Self: Sync,
        H: AsRef<str> + Send + Sync,
    {
        async move {
            let mut raw_login = self
                .login_data_query()
                .query_login_dt_filter(ChromiumLoginCol::OriginUrl.contains(host.as_ref()))
                .await
                .context(DbSnafu)?;
            if raw_login.is_empty() {
                if let Some(query) = &self.login_data_for_account_query() {
                    raw_login = query
                        .query_login_dt_filter(ChromiumLoginCol::OriginUrl.contains(host.as_ref()))
                        .await
                        .context(DbSnafu)?;
                }
            }
            self.par_decrypt_logins(raw_login)
                .await
        }
    }

    /// Return all login data
    fn logins_all(&self) -> impl std::future::Future<Output = Result<Vec<LoginData>>> + Send
    where
        Self: Sync,
    {
        async {
            let mut raw_login = self
                .login_data_query()
                .query_all_login_dt()
                .await
                .context(DbSnafu)?;
            if raw_login.is_empty() {
                if let Some(query) = &self.login_data_for_account_query() {
                    raw_login = query
                        .query_all_login_dt()
                        .await
                        .context(DbSnafu)?;
                }
            }
            self.par_decrypt_logins(raw_login)
                .await
        }
    }
}

#[expect(private_bounds, reason = "impl details")]
pub trait GetCookies: SealedCrypto + SealedCookies {
    /// filter cookies
    ///
    /// # Example:
    ///
    /// ```rust
    /// use decrypt_cookies::{chromium::GetCookies, prelude::*};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let edge_getter = ChromiumBuilder::<Chrome>::new()
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = edge_getter
    ///         .cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
    ///         .await
    ///         .unwrap_or_default();
    ///     dbg!(res);
    /// }
    /// ```
    fn cookies_filter<F>(
        &self,
        filter: F,
    ) -> impl std::future::Future<Output = Result<Vec<ChromiumCookie>>> + Send
    where
        F: IntoCondition + Send,
        Self: Sync,
    {
        async {
            let raw_ck = self
                .cookies_query()
                .cookies_filter(filter)
                .await
                .context(DbSnafu)?;
            self.par_decrypt_ck(raw_ck).await
        }
    }

    /// Filter by host
    #[doc(alias = "cookies_by_domain", alias = "cookies_by_url")]
    fn cookies_by_host<H>(
        &self,
        host: H,
    ) -> impl std::future::Future<Output = Result<Vec<ChromiumCookie>>> + Send
    where
        Self: Sync,
        H: AsRef<str> + Send + Sync,
    {
        async move {
            let raw_ck = self
                .cookies_query()
                .cookies_by_host(host.as_ref())
                .await
                .context(DbSnafu)?;
            self.par_decrypt_ck(raw_ck).await
        }
    }

    /// Return all cookies
    fn cookies_all(&self) -> impl std::future::Future<Output = Result<Vec<ChromiumCookie>>> + Send
    where
        Self: Sync,
    {
        async {
            let raw_ck = self
                .cookies_query()
                .cookies_all()
                .await
                .context(DbSnafu)?;
            self.par_decrypt_ck(raw_ck).await
        }
    }

    /// get `LEETCODE_SESSION` and `csrftoken` for leetcode
    fn get_session_csrf<H>(
        &self,
        host: H,
    ) -> impl std::future::Future<Output = Result<LeetCodeCookies>> + Send
    where
        Self: Sync,
        H: AsRef<str> + Send + Sync,
    {
        async move {
            let cookies = self
                .cookies_query()
                .cookies_filter(
                    ChromiumCookieCol::HostKey
                        .contains(host.as_ref())
                        .and(
                            ChromiumCookieCol::Name
                                .eq("csrftoken")
                                .or(ChromiumCookieCol::Name.eq("LEETCODE_SESSION")),
                        ),
                )
                .await
                .context(DbSnafu)?;

            let mut csrf_token = LeetCodeCookies::default();
            let mut hds = Vec::with_capacity(2);

            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            enum CsrfSession {
                Csrf,
                Session,
            }

            // # Safety: scope task
            let cy =
                unsafe { std::mem::transmute::<&Decrypter, &'static Decrypter>(self.crypto()) };

            for mut cookie in cookies {
                if cookie.name == "csrftoken" {
                    let expir = cookie
                        .expires_utc
                        .micros_to_chromium_utc();
                    if let Some(expir) = expir {
                        if Utc::now() > expir {
                            csrf_token.expiry = true;
                            break;
                        }
                    }

                    let csrf_hd = task::spawn_blocking(move || {
                        cy.decrypt(&mut cookie.encrypted_value, Which::Cookie)
                            .inspect_err(|_e| {
                                #[cfg(feature = "tracing")]
                                tracing::warn!("decrypt csrf failed: {_e}");
                            })
                            .unwrap_or_default()
                    });
                    hds.push((csrf_hd, CsrfSession::Csrf));
                }
                else if cookie.name == "LEETCODE_SESSION" {
                    let expir = cookie
                        .expires_utc
                        .micros_to_chromium_utc();
                    if let Some(expir) = expir {
                        if Utc::now() > expir {
                            csrf_token.expiry = true;
                            break;
                        }
                    }

                    let session_hd = task::spawn_blocking(move || {
                        cy.decrypt(&mut cookie.encrypted_value, Which::Cookie)
                            .inspect_err(|_e| {
                                #[cfg(feature = "tracing")]
                                tracing::warn!("decrypt session failed: {_e}");
                            })
                            .unwrap_or_default()
                    });
                    hds.push((session_hd, CsrfSession::Session));
                }
            }

            for (handle, flag) in hds {
                let res = handle.await.context(TaskSnafu)?;
                match flag {
                    CsrfSession::Csrf => csrf_token.csrf = res,
                    CsrfSession::Session => csrf_token.session = res,
                }
            }
            Ok(csrf_token)
        }
    }
}
