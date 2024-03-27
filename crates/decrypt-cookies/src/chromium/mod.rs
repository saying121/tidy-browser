mod items;
mod utils;

use std::path::PathBuf;

use items::cookie::{dao::CookiesQuery, entities::cookies};
pub use items::cookie::{
    entities::cookies::{Column as ChromCkColumn, ColumnIter as ChromCkColumnIter},
    DecryptedCookies,
};
use miette::{IntoDiagnostic, Result};
use rayon::prelude::*;
use sea_orm::sea_query::IntoCondition;
use tokio::task;
use utils::crypto::BrowserDecrypt;

cfg_if::cfg_if!(
    if #[cfg(target_os = "linux")] {
        use utils::linux::crypto::Decrypter;
        use utils::linux::path::LinuxChromiumBase;
    } else if #[cfg(target_os = "macos")] {
        use utils::macos::crypto::Decrypter;
        use utils::macos::path::MacChromiumBase;
    } else if #[cfg(target_os = "windows")] {
        use utils::win::crypto::DecrypterBuilder;
        use utils::win::crypto::Decrypter;
        use utils::win::path::WinChromiumBase;
    }
);

use crate::{chromium::utils::path::ChromiumPath, Browser, LeetCodeCookies};

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
    query:   CookiesQuery,
    crypto:  Decrypter,

    /// generate Default paths
    #[cfg(target_os = "linux")]
    path: LinuxChromiumBase,
    /// generate Default paths
    #[cfg(target_os = "macos")]
    path: MacChromiumBase,
    /// generate Default paths
    #[cfg(target_os = "windows")]
    path: WinChromiumBase,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ChromiumBuilder {
    browser:          Browser,
    cookies_path:     Option<PathBuf>,
    local_state_path: Option<PathBuf>,
}

impl ChromiumBuilder {
    pub fn new(browser: Browser) -> Self {
        Self { browser, ..Default::default() }
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
    pub async fn build(&mut self) -> Result<ChromiumGetter> {
        cfg_if::cfg_if!(
            if #[cfg(target_os = "linux")] {
                let crypto = Decrypter::build(self.browser).await?;
                let path = LinuxChromiumBase::new(self.browser);
            } else if #[cfg(target_os = "macos")] {
                let crypto = Decrypter::build(self.browser).await?;
                let path = MacChromiumBase::new(self.browser);
            } else if #[cfg(target_os = "windows")] {
                let mut crypto = DecrypterBuilder::new(self.browser);
                if let Some(path) = self.local_state_path.take() {
                    crypto.local_state_path(path);
                }
                let crypto = crypto.build().await?;
                let path = WinChromiumBase::new(self.browser);
            }
        );
        let query = CookiesQuery::new(
            self.cookies_path
                .take()
                .unwrap_or_else(|| path.cookies()),
        )
        .await?;

        Ok(ChromiumGetter {
            browser: self.browser,
            query,
            crypto,
            path,
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
            .query
            .query_cookie_filter(filter)
            .await?;
        self.par_decrypt(raw_ck).await
    }
    /// decrypt Cookies
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<DecryptedCookies>> {
        let raw_ck = self
            .query
            .query_cookie_by_host(host)
            .await?;
        self.par_decrypt(raw_ck).await
    }

    /// return all cookies
    pub async fn get_cookies_all(&self) -> Result<Vec<DecryptedCookies>> {
        let raw_ck = self
            .query
            .query_all_cookie()
            .await?;
        self.par_decrypt(raw_ck).await
    }

    /// get `LEETCODE_SESSION` and `csrftoken` for leetcode
    pub async fn get_cookies_session_csrf(&self, host: &str) -> Result<LeetCodeCookies> {
        let cookies = self
            .query
            .query_cookie_by_host(host)
            .await?;

        let mut csrf_token = LeetCodeCookies::default();
        let mut hds = Vec::with_capacity(2);

        #[derive(Clone, Copy)]
        #[derive(Debug)]
        #[derive(PartialEq, Eq)]
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
}

impl ChromiumGetter {
    /// generate Default paths
    #[cfg(target_os = "linux")]
    pub const fn path(&self) -> &LinuxChromiumBase {
        &self.path
    }
    /// generate Default paths
    #[cfg(target_os = "macos")]
    pub const fn path(&self) -> &MacChromiumBase {
        &self.path
    }
    /// generate Default paths
    #[cfg(target_os = "windows")]
    pub const fn path(&self) -> &WinChromiumBase {
        &self.path
    }

    pub const fn browser(&self) -> Browser {
        self.browser
    }
}
