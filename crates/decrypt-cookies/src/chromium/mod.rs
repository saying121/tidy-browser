pub mod crypto;
pub(crate) mod items;
#[cfg(target_os = "windows")]
pub mod local_state;
use std::{marker::PhantomData, path::PathBuf};

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
use tokio::task::{self, JoinError};

#[cfg(target_os = "linux")]
use crate::chromium::crypto::linux::Decrypter;
#[cfg(target_os = "macos")]
use crate::chromium::crypto::macos::Decrypter;
#[cfg(target_os = "windows")]
use crate::chromium::crypto::win::Decrypter;
use crate::{
    browser::cookies::LeetCodeCookies,
    chromium::items::{
        cookie::cookie_dao::CookiesQuery,
        passwd::{login_data_dao::LoginDataQuery, login_data_entities::logins},
        I64ToChromiumDateTime,
    },
};

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ChromiumError {
    #[error(transparent)]
    Task(#[from] JoinError),
    #[error(transparent)]
    Db(#[from] DbErr),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    Decrypt(#[from] crate::chromium::crypto::linux::CryptoError),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Decrypt(#[from] crate::chromium::crypto::win::CryptoError),
    #[cfg(target_os = "macos")]
    #[error(transparent)]
    Decrypt(#[from] crate::chromium::crypto::macos::CryptoError),
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
    pub(crate) crypto: Decrypter,
    pub(crate) __browser: PhantomData<T>,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct ChromiumBuilder<T> {
    pub(crate) base: PathBuf,
    pub(crate) __browser: PhantomData<T>,
}

impl<T: Send + Sync> ChromiumGetter<T> {
    async fn par_decrypt_logins(&self, raw: Vec<logins::Model>) -> Result<Vec<LoginData>> {
        let crypto = self.crypto.clone();

        let login_data = task::spawn_blocking(move || {
            raw.into_par_iter()
                .map(|mut v| {
                    let res = v
                        .password_value
                        .as_mut()
                        .map_or_else(String::new, |passwd| {
                            crypto
                                .decrypt(passwd)
                                .unwrap_or_default()
                        });
                    let mut cookies = LoginData::from(v);
                    cookies.set_password_value(res);
                    cookies
                })
                .collect()
        })
        .await;
        Ok(login_data?)
    }
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
    ///         .get_logins_filter(ChromiumLoginCol::OriginUrl.contains("google.com"))
    ///         .await
    ///         .unwrap_or_default();
    ///     dbg!(res);
    /// }
    /// ```
    pub async fn get_logins_filter<F>(&self, filter: F) -> Result<Vec<LoginData>>
    where
        F: IntoCondition + Send,
    {
        let raw_login = self
            .login_data_query
            .query_login_dt_filter(filter)
            .await?;
        self.par_decrypt_logins(raw_login)
            .await
    }
    pub async fn get_logins_by_host<F>(&self, host: F) -> Result<Vec<LoginData>>
    where
        F: AsRef<str> + Send,
    {
        let raw_login = self
            .login_data_query
            .query_login_dt_filter(ChromiumLoginCol::OriginUrl.contains(host.as_ref()))
            .await?;
        self.par_decrypt_logins(raw_login)
            .await
    }
    /// contains passwords
    pub async fn get_logins_all(&self) -> Result<Vec<LoginData>> {
        let raw_login = self
            .login_data_query
            .query_all_login_dt()
            .await?;
        self.par_decrypt_logins(raw_login)
            .await
    }
}

impl<T: Send + Sync> ChromiumGetter<T> {
    /// filter cookies
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
    ///         .get_cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
    ///         .await
    ///         .unwrap_or_default();
    ///     dbg!(res);
    /// }
    /// ```
    pub async fn get_cookies_filter<F>(&self, filter: F) -> Result<Vec<ChromiumCookie>>
    where
        F: IntoCondition + Send,
    {
        let raw_ck = self
            .cookies_query
            .query_cookie_filter(filter)
            .await?;
        self.par_decrypt_ck(raw_ck).await
    }
    /// decrypt Cookies
    pub async fn get_cookies_by_host<A: AsRef<str> + Send>(
        &self,
        host: A,
    ) -> Result<Vec<ChromiumCookie>> {
        let raw_ck = self
            .cookies_query
            .query_cookie_by_host(host.as_ref())
            .await?;
        self.par_decrypt_ck(raw_ck).await
    }

    /// return all cookies
    pub async fn get_cookies_all(&self) -> Result<Vec<ChromiumCookie>> {
        let raw_ck = self
            .cookies_query
            .query_all_cookie()
            .await?;
        self.par_decrypt_ck(raw_ck).await
    }

    /// get `LEETCODE_SESSION` and `csrftoken` for leetcode
    pub async fn get_session_csrf<A: AsRef<str> + Send>(
        &self,
        host: A,
    ) -> Result<LeetCodeCookies> {
        let cookies = self
            .cookies_query
            .query_cookie_filter(
                ChromiumCookieCol::HostKey
                    .contains(host.as_ref())
                    .and(
                        ChromiumCookieCol::Name
                            .eq("csrftoken")
                            .or(ChromiumCookieCol::Name.eq("LEETCODE_SESSION")),
                    ),
            )
            .await?;

        let mut csrf_token = LeetCodeCookies::default();
        let mut hds = Vec::with_capacity(2);

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum CsrfSession {
            Csrf,
            Session,
        }
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

                let cy = self.crypto.clone();
                let csrf_hd =
                    task::spawn_blocking(move || match cy.decrypt(&mut cookie.encrypted_value) {
                        Ok(it) => it,
                        Err(err) => {
                            tracing::warn!("decrypt csrf failed: {err}");
                            String::new()
                        },
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

                let cy = self.crypto.clone();
                let session_hd =
                    task::spawn_blocking(move || match cy.decrypt(&mut cookie.encrypted_value) {
                        Ok(it) => it,
                        Err(err) => {
                            tracing::warn!("decrypt session failed: {err}");
                            String::new()
                        },
                    });
                hds.push((session_hd, CsrfSession::Session));
            }
        }
        for (handle, flag) in hds {
            let res = handle.await?;
            match flag {
                CsrfSession::Csrf => csrf_token.csrf = res,
                CsrfSession::Session => csrf_token.session = res,
            }
        }
        Ok(csrf_token)
    }
    /// parallel decrypt cookies
    pub fn par_decrypt_cookies(&self, raw: Vec<cookies::Model>) -> Vec<ChromiumCookie> {
        raw.into_par_iter()
            .map(|mut v| {
                let res = self
                    .crypto
                    .decrypt(&mut v.encrypted_value)
                    .unwrap_or_default();
                let mut cookies = ChromiumCookie::from(v);
                cookies.set_encrypted_value(res);
                cookies
            })
            .collect()
    }

    /// parallel decrypt cookies
    /// and not blocking scheduling
    async fn par_decrypt_ck(&self, raw: Vec<cookies::Model>) -> Result<Vec<ChromiumCookie>> {
        let crypto = self.crypto.clone();

        let decrypted_ck = task::spawn_blocking(move || {
            raw.into_par_iter()
                .map(|mut v| {
                    let res = crypto
                        .decrypt(&mut v.encrypted_value)
                        .unwrap_or_default();
                    let mut cookies = ChromiumCookie::from(v);
                    cookies.set_encrypted_value(res);
                    cookies
                })
                .collect()
        })
        .await?;
        Ok(decrypted_ck)
    }
}

impl<T: Send + Sync> ChromiumGetter<T> {
    /// the browser's decrypt
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        Ok(self.crypto.decrypt(ciphertext)?)
    }
}
