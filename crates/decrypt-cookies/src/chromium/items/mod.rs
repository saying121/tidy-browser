pub mod cookie;
pub mod passwd;

use miette::Result;

use self::cookie::{dao::CookiesQuery, DecryptedCookies};
use super::utils::crypto::BrowserDecrypt;
cfg_if::cfg_if!(
    if #[cfg(target_os = "linux")] {
        use super::utils::linux::crypto::Decrypter;
    } else if #[cfg(target_os = "macos")] {
        use super::utils::macos::crypto::Decrypter;
    } else if #[cfg(target_os = "windows")] {
        use super::utils::win::crypto::Decrypter;
    }
);
use crate::{Browser, LeetCodeCookies};

/// Chromium based, get cookies and decrypt
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromiumGetter {
    browser: Browser,
    query:   CookiesQuery,
    crypto:  Decrypter,
}

impl ChromiumGetter {
    pub async fn build(browser: Browser) -> Result<Self> {
        let query = CookiesQuery::new(browser).await?;

        #[cfg(target_os = "linux")]
        let crypto = Decrypter::build(browser).await?;
        #[cfg(target_os = "macos")]
        let crypto = Decrypter::build(browser).await?;
        #[cfg(target_os = "windows")]
        let crypto = Decrypter::build(browser).await?;

        Ok(Self { browser, query, crypto })
    }

    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        self.crypto.decrypt(ciphertext)
    }

    pub const fn brwoser(&self) -> Browser {
        self.browser
    }
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<DecryptedCookies>> {
        let cookies = self
            .query
            .query_cookie(host)
            .await?;
        let mut decrypted_cookies = vec![];
        for mut cookie in cookies {
            let res = match self
                .crypto
                .decrypt(&mut cookie.encrypted_value)
            {
                Ok(it) => it,
                Err(err) => {
                    tracing::warn!("decrypt csrf failed: {err}");
                    String::new()
                },
            };
            let mut decrypted = DecryptedCookies::from(cookie);
            decrypted.set_encrypted_value(res);
            decrypted_cookies.push(decrypted);
        }
        Ok(decrypted_cookies)
    }

    pub async fn get_all_cookies(&self) -> Result<Vec<DecryptedCookies>> {
        let cookies = self.query.all_cookie().await?;
        let mut decrypted_cookies = vec![];
        for mut cookie in cookies {
            let res = match self
                .crypto
                .decrypt(&mut cookie.encrypted_value)
            {
                Ok(it) => it,
                Err(err) => {
                    tracing::warn!("decrypt csrf failed: {err}");
                    String::new()
                },
            };
            let mut decrypted = DecryptedCookies::from(cookie);
            decrypted.set_encrypted_value(res);
            decrypted_cookies.push(decrypted);
        }
        Ok(decrypted_cookies)
    }

    /// get `LEETCODE_SESSION` and `csrftoken` for leetcode
    pub async fn get_session_csrf(&self, host: &str) -> Result<LeetCodeCookies> {
        let mut cookies = self
            .query
            .query_cookie(host)
            .await?;

        let mut res = LeetCodeCookies::default();
        for cookie in &mut cookies {
            if cookie.name == "csrftoken" {
                res.csrf = match self
                    .crypto
                    .decrypt(&mut cookie.encrypted_value)
                {
                    Ok(it) => it,
                    Err(err) => {
                        tracing::warn!("decrypt csrf failed: {err}");
                        String::new()
                    },
                };
            }
            else if cookie.name == "LEETCODE_SESSION" {
                res.session = match self
                    .crypto
                    .decrypt(&mut cookie.encrypted_value)
                {
                    Ok(it) => it,
                    Err(err) => {
                        tracing::warn!("decrypt session failed: {err}");
                        String::new()
                    },
                };
            }
        }
        Ok(res)
    }
}
