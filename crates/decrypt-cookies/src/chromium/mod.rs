pub mod crypto;
mod items;
use std::path::PathBuf;

use items::cookie::{dao::CookiesQuery, entities::cookies};
pub use items::cookie::{
    entities::cookies::{Column as ChromCkColumn, ColumnIter as ChromCkColumnIter},
    DecryptedCookies,
};
use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use sea_orm::{prelude::ColumnTrait, sea_query::IntoCondition};
use tokio::{fs, task};

use crate::{
    browser::info::ChromiumInfo,
    Browser, LeetCodeCookies,
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
    browser:       Browser,
    cookies_query: CookiesQuery,
    crypto:        Decrypter,

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
    browser:          Browser,
    cookies_path:     Option<PathBuf>,
    /// in windows, it store passwd
    local_state_path: Option<PathBuf>,
}

impl ChromiumBuilder {
    pub const fn new(browser: Browser) -> Self {
        Self {
            browser,
            cookies_path: None,
            local_state_path: None,
        }
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
            if #[cfg(target_os="linux")] {
                let info = LinuxChromiumBase::new(self.browser);
                let crypto = Decrypter::build(info.browser(), info.safe_storage()).await?;
            } else if #[cfg(target_os="macos")] {
                let info = MacChromiumBase::new(self.browser);
                let crypto = Decrypter::build(
                    self.browser,
                    info.safe_storage(),
                    info.safe_name(),
                )?;
            } else if #[cfg(target_os="windows")] {
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

        let temp_cookies_path = info.cookies_temp();

        fs::copy(
            self.cookies_path
                .unwrap_or_else(|| info.cookies()),
            &temp_cookies_path,
        )
        .await
        .into_diagnostic()?;

        let query = CookiesQuery::new(temp_cookies_path).await?;

        Ok(ChromiumGetter {
            browser: self.browser,
            cookies_query: query,
            crypto,
            info,
        })
    }
}

impl ChromiumGetter {
    /// the browser's decrypt
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        self.crypto.decrypt(ciphertext)
    }
    /// parallel decrypt cookies
    pub fn par_decrypt_cookies(&self, raw: Vec<cookies::Model>) -> Vec<DecryptedCookies> {
        raw.into_par_iter()
            .map(|mut v| {
                let res = self
                    .crypto
                    .decrypt(&mut v.encrypted_value)
                    .unwrap_or_default();
                let mut cookies = DecryptedCookies::from(v);
                cookies.set_encrypted_value(res);
                cookies
            })
            .collect()
    }

    /// parallel decrypt cookies
    /// and not blocking scheduling
    pub async fn par_decrypt(&self, raw: Vec<cookies::Model>) -> Result<Vec<DecryptedCookies>> {
        let crypto = self.crypto.clone();

        task::spawn_blocking(move || {
            raw.into_par_iter()
                .map(|mut v| {
                    let res = crypto
                        .decrypt(&mut v.encrypted_value)
                        .unwrap_or_default();
                    let mut cookies = DecryptedCookies::from(v);
                    cookies.set_encrypted_value(res);
                    cookies
                })
                .collect()
        })
        .await
        .into_diagnostic()
    }

    /// filter cookies
    ///
    /// # Example:
    ///
    /// ```rust
    /// use decrypt_cookies::{chromium::ChromCkColumn, Browser, ChromiumGetter};
    /// use sea_orm::prelude::ColumnTrait;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let edge_getter = ChromiumGetter::build(Browser::Edge)
    ///         .await
    ///         .unwrap();
    ///     let res = edge_getter
    ///         .get_cookies_filter(ChromCkColumn::HostKey.contains("google.com"))
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn get_cookies_filter<F>(&self, filter: F) -> Result<Vec<DecryptedCookies>>
    where
        F: IntoCondition,
    {
        let raw_ck = self
            .cookies_query
            .query_cookie_filter(filter)
            .await?;
        self.par_decrypt(raw_ck).await
    }
    /// decrypt Cookies
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<DecryptedCookies>> {
        let raw_ck = self
            .cookies_query
            .query_cookie_by_host(host)
            .await?;
        self.par_decrypt(raw_ck).await
    }

    /// return all cookies
    pub async fn get_cookies_all(&self) -> Result<Vec<DecryptedCookies>> {
        let raw_ck = self
            .cookies_query
            .query_all_cookie()
            .await?;
        self.par_decrypt(raw_ck).await
    }

    /// get `LEETCODE_SESSION` and `csrftoken` for leetcode
    pub async fn get_cookies_session_csrf(&self, host: &str) -> Result<LeetCodeCookies> {
        let cookies = self
            .cookies_query
            .query_cookie_filter(
                ChromCkColumn::HostKey
                    .contains(host)
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

    pub const fn browser(&self) -> Browser {
        self.browser
    }

    pub fn info(&self) -> &impl crate::browser::info::ChromiumInfo {
        &self.info
    }
}
