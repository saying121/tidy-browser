mod items;
mod utils;

pub use items::cookie::entities::cookies::{
    Column as ChromCkColumn, ColumnIter as ChromCkColumnIter,
};
use items::cookie::{dao::CookiesQuery, entities::cookies, DecryptedCookies};
use miette::{IntoDiagnostic, Result};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sea_orm::sea_query::IntoCondition;
use utils::crypto::BrowserDecrypt;

cfg_if::cfg_if!(
    if #[cfg(target_os = "linux")] {
        use utils::linux::crypto::Decrypter;
        use utils::linux::path::LinuxChromiumBase;
    } else if #[cfg(target_os = "macos")] {
        use utils::macos::crypto::Decrypter;
        use utils::macos::path::MacChromiumBase;
    } else if #[cfg(target_os = "windows")] {
        use utils::win::crypto::Decrypter;
        use utils::win::path::WinChromiumBase;
    }
);

use crate::{chromium::utils::path::ChromiumPath, Browser, LeetCodeCookies};

/// Chromium based, get cookies, etc. and decrypt
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumGetter {
    browser: Browser,
    query:   CookiesQuery,
    crypto:  Decrypter,

    #[cfg(target_os = "linux")]
    path: LinuxChromiumBase,
    #[cfg(target_os = "macos")]
    path: MacChromiumBase,
    #[cfg(target_os = "windows")]
    path: WinChromiumBase,
}

impl ChromiumGetter {
    pub async fn build(browser: Browser) -> Result<Self> {
        cfg_if::cfg_if!(
            if #[cfg(target_os = "linux")] {
                let crypto = Decrypter::build(browser).await?;
                let path = LinuxChromiumBase::new(browser);
            } else if #[cfg(target_os = "macos")] {
                let crypto = Decrypter::build(browser).await?;
                let path = WinChromiumBase::new(browser);
            } else if #[cfg(target_os = "windows")] {
                let crypto = Decrypter::build(browser).await?;
                let path = MacChromiumBase::new(browser);
            }
        );

        let query = CookiesQuery::new(path.cookies()).await?;

        Ok(Self { browser, query, crypto, path })
    }

    /// the browser's decrypt
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        self.crypto.decrypt(ciphertext)
    }
    /// use rayon decrypt cookies
    fn par_decrypt_cookies(&self, raw: Vec<cookies::Model>) -> Vec<DecryptedCookies> {
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
        Ok(self.par_decrypt_cookies(raw_ck))
    }
    /// decrypt Cookies
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<DecryptedCookies>> {
        let raw_cookies = self
            .query
            .query_cookie_by_host(host)
            .await?;
        Ok(self.par_decrypt_cookies(raw_cookies))
    }

    /// return all cookies
    pub async fn get_cookies_all(&self) -> Result<Vec<DecryptedCookies>> {
        let raw_cookies = self
            .query
            .query_all_cookie()
            .await?;
        Ok(self.par_decrypt_cookies(raw_cookies))
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
                let csrf_hd = tokio::task::spawn_blocking(move || {
                    match cy.decrypt(&mut cookie.encrypted_value) {
                        Ok(it) => it,
                        Err(err) => {
                            tracing::warn!("decrypt csrf failed: {err}");
                            String::new()
                        },
                    }
                });
                hds.push((csrf_hd, CsrfSession::Csrf));
            }
            else if cookie.name == "LEETCODE_SESSION" {
                let cy = self.crypto.clone();
                let session_hd = tokio::task::spawn_blocking(move || {
                    match cy.decrypt(&mut cookie.encrypted_value) {
                        Ok(it) => it,
                        Err(err) => {
                            tracing::warn!("decrypt session failed: {err}");
                            String::new()
                        },
                    }
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
    #[cfg(target_os = "linux")]
    pub const fn path(&self) -> &LinuxChromiumBase {
        &self.path
    }
    #[cfg(target_os = "macos")]
    pub const fn path(&self) -> &MacChromiumBase {
        &self.path
    }
    #[cfg(target_os = "windows")]
    pub const fn path(&self) -> &WinChromiumBase {
        &self.path
    }

    pub const fn browser(&self) -> Browser {
        self.browser
    }
}
