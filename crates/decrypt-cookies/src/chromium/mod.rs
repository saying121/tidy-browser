pub mod crypto;
mod items;
pub mod local_state;
use std::path::PathBuf;

use chrono::prelude::Utc;
use items::cookie::{cookie_dao::CookiesQuery, cookie_entities::cookies};
pub use items::{
    cookie::{
        cookie_entities::cookies::{Column as ChromCkColumn, ColumnIter as ChromCkColumnIter},
        ChromiumCookie,
    },
    passwd::{
        login_data_entities::logins::{Column as ChromLoginColumn, Column as ChromLoginColumnIter},
        LoginData,
    },
};
use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition};
use tokio::{fs, task};

use self::items::passwd::{login_data_dao::LoginDataQuery, login_data_entities::logins};
use crate::{
    browser::info::ChromiumInfo, chromium::items::I64ToChromiumDateTime, Browser, LeetCodeCookies,
};

cfg_if::cfg_if!(
    if #[cfg(target_os="linux")] {
        use crypto::linux::Decrypter;
        use crate::browser::info::linux::LinuxChromiumBase;
    } else if #[cfg(target_os="macos")] {
        use crypto::macos::Decrypter;
        use crate::browser::info::macos::MacChromiumBase;
    } else if #[cfg(target_os="windows")] {
        use crypto::win::Decrypter;
        use crate::browser::info::win::WinChromiumBase;
    }
);

/// Chromium based, get cookies, etc. and decrypt
///
/// Initialize it with `ChromiumBuilder`
///
/// # Example
/// ```rust, ignore
/// let getter = ChromiumBuilder::new(Browser::Chromium)
///     .build()
///     .await?;
/// getter
///     .get_cookies_session_csrf(host)
///     .await?
/// ```
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumGetter {
    browser: Browser,
    cookies_query: CookiesQuery,
    login_data_query: LoginDataQuery,
    crypto: Decrypter,

    #[cfg(target_os = "linux")]
    pub info: LinuxChromiumBase,
    #[cfg(target_os = "windows")]
    pub info: WinChromiumBase,
    #[cfg(target_os = "macos")]
    pub info: MacChromiumBase,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct ChromiumBuilder {
    browser: Browser,
    cookies_path: Option<PathBuf>,
    /// in windows, it store passwd
    local_state_path: Option<PathBuf>,

    login_data_path: Option<PathBuf>,
}

impl ChromiumBuilder {
    /// # Panics
    ///
    /// When you use not Chromium based browser
    pub fn new(browser: Browser) -> Self {
        assert!(
            browser.is_chromium_base(),
            "Chromium based not support: {browser}"
        );
        Self {
            browser,
            cookies_path: None,
            local_state_path: None,
            login_data_path: None,
        }
    }
    pub fn login_data_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.login_data_path = Some(path.into());
        self
    }
    /// set cookies path
    pub fn cookies_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.cookies_path = Some(path.into());
        self
    }
    /// set `local_state` path
    pub fn local_state_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.local_state_path = Some(path.into());
        self
    }

    pub async fn build(self) -> Result<ChromiumGetter> {
        cfg_if::cfg_if!(
            if #[cfg(target_os = "linux")] {
                let info = LinuxChromiumBase::new(self.browser);
                let crypto = Decrypter::build(self.browser, info.safe_storage()).await?;
            } else if #[cfg(target_os = "macos")] {
                let info = MacChromiumBase::new(self.browser);
                let crypto = Decrypter::build(
                    self.browser,
                    info.safe_storage(),
                    info.safe_name(),
                )?;
            } else if #[cfg(target_os = "windows")] {
                let info = WinChromiumBase::new(self.browser);

                let temp_key_path = info.local_state_temp();
                fs::copy(
                    self.local_state_path
                        .unwrap_or_else(|| info.local_state()),
                    &temp_key_path,
                )
                .await
                .into_diagnostic()?;
                let crypto = Decrypter::build(self.browser, temp_key_path).await?;
            }
        );
        let (temp_cookies_path, temp_login_data_path) =
            (info.cookies_temp(), info.logindata_temp());
        let cp_login = fs::copy(
            self.login_data_path
                .unwrap_or_else(|| info.logindata()),
            &temp_login_data_path,
        );

        let cp_cookies = fs::copy(
            self.cookies_path
                .unwrap_or_else(|| info.cookies()),
            &temp_cookies_path,
        );
        let (login, cookies) = tokio::join!(cp_login, cp_cookies);
        login.into_diagnostic()?;
        cookies.into_diagnostic()?;

        let (cookies_query, login_data_query) = (
            CookiesQuery::new(temp_cookies_path),
            LoginDataQuery::new(temp_login_data_path),
        );
        let (cookies_query, login_data_query) = tokio::join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (cookies_query?, login_data_query?);

        Ok(ChromiumGetter {
            browser: self.browser,
            cookies_query,
            login_data_query,
            crypto,
            info,
        })
    }
}

impl ChromiumGetter {
    async fn par_decrypt_logins(&self, raw: Vec<logins::Model>) -> Result<Vec<LoginData>> {
        let crypto = self.crypto.clone();

        task::spawn_blocking(move || {
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
        .await
        .into_diagnostic()
    }
    /// contains passwords
    ///
    /// # Example:
    ///
    /// ```rust
    /// use decrypt_cookies::{chromium::ChromLoginColumn, Browser, ChromiumBuilder, ColumnTrait};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let edge_getter = ChromiumBuilder::new(Browser::Edge)
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = edge_getter
    ///         .get_logins_filter(ChromLoginColumn::OriginUrl.contains("google.com"))
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
    /// contains passwords
    pub async fn get_logins_by_host<F>(&self, host: F) -> Result<Vec<LoginData>>
    where
        F: AsRef<str> + Send,
    {
        let raw_login = self
            .login_data_query
            .query_login_dt_filter(ChromLoginColumn::OriginUrl.contains(host.as_ref()))
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

impl ChromiumGetter {
    /// filter cookies
    ///
    /// # Example:
    ///
    /// ```rust
    /// use decrypt_cookies::{chromium::ChromCkColumn, Browser, ChromiumBuilder, ColumnTrait};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let edge_getter = ChromiumBuilder::new(Browser::Edge)
    ///         .build()
    ///         .await
    ///         .unwrap();
    ///     let res = edge_getter
    ///         .get_cookies_filter(ChromCkColumn::HostKey.contains("google.com"))
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
    pub async fn get_cookies_session_csrf<A: AsRef<str> + Send>(
        &self,
        host: A,
    ) -> Result<LeetCodeCookies> {
        let cookies = self
            .cookies_query
            .query_cookie_filter(
                ChromCkColumn::HostKey
                    .contains(host.as_ref())
                    .and(
                        ChromCkColumn::Name
                            .eq("csrftoken")
                            .or(ChromCkColumn::Name.eq("LEETCODE_SESSION")),
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
            let res = handle.await.into_diagnostic()?;
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

        task::spawn_blocking(move || {
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
        .await
        .into_diagnostic()
    }
}

impl ChromiumGetter {
    /// the browser's decrypt
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        self.crypto.decrypt(ciphertext)
    }

    pub const fn browser(&self) -> Browser {
        self.browser
    }

    pub fn info(&self) -> &impl crate::browser::info::ChromiumInfo {
        &self.info
    }
}
