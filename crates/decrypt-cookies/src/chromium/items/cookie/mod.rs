use miette::Result;

use self::dao::CookiesQuery;
use self::entities::cookies;
use crate::{chromium::utils::crypto::decrypt_cookies, Browser, LeetCodeCookies};

pub mod dao;
pub mod entities;

/// get `LEETCODE_SESSION` and `csrftoken` for leetcode
pub async fn get_session_csrf(browser: Browser, host: &str) -> Result<LeetCodeCookies> {
    let query = CookiesQuery::new(browser).await?;
    let mut cookies = query.query_cookie(host).await?;

    let mut res = LeetCodeCookies::default();
    for cookie in &mut cookies {
        if cookie.name == "csrftoken" {
            res.csrf = match decrypt_cookies(&mut cookie.encrypted_value, browser).await {
                Ok(it) => it,
                Err(err) => {
                    tracing::warn!("decrypt csrf failed: {err}");
                    String::new()
                },
            };
            tracing::trace!("{:?}", &cookie.encrypted_value);
        } else if cookie.name == "LEETCODE_SESSION" {
            res.session = match decrypt_cookies(&mut cookie.encrypted_value.clone(), browser).await
            {
                Ok(it) => it,
                Err(err) => {
                    tracing::warn!("decrypt session failed: {err}");
                    String::new()
                },
            };
            tracing::trace!("{:?}", &cookie.encrypted_value);
        }
    }
    Ok(res)
}

/// Chromium based, get cookies and decrypt
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesGetter {
    query: CookiesQuery,
    browser: Browser,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct DecryptedCookies {
    pub creation_utc: i32,
    pub host_key: String,
    pub top_frame_site_key: String,
    pub name: String,
    pub value: String,
    pub encrypted_value: Option<String>,
    pub path: String,
    pub expires_utc: i32,
    pub is_secure: i32,
    pub is_httponly: i32,
    pub last_access_utc: i32,
    pub has_expires: i32,
    pub is_persistent: i32,
    pub priority: i32,
    pub samesite: i32,
    pub source_scheme: i32,
    pub source_port: i32,
    pub last_update_utc: i32,
}

impl DecryptedCookies {
    pub fn set_encrypted_value(&mut self, encrypted_value: String) {
        self.encrypted_value = Some(encrypted_value);
    }
}

impl From<cookies::Model> for DecryptedCookies {
    fn from(value: cookies::Model) -> Self {
        Self {
            creation_utc: value.creation_utc,
            host_key: value.host_key,
            top_frame_site_key: value.top_frame_site_key,
            name: value.name,
            value: value.value,
            encrypted_value: None,
            path: value.path,
            expires_utc: value.expires_utc,
            is_secure: value.is_secure,
            is_httponly: value.is_httponly,
            last_access_utc: value.last_access_utc,
            has_expires: value.has_expires,
            is_persistent: value.is_persistent,
            priority: value.priority,
            samesite: value.samesite,
            source_scheme: value.source_scheme,
            source_port: value.source_port,
            last_update_utc: value.last_update_utc,
        }
    }
}

impl CookiesGetter {
    pub async fn build(browser: Browser) -> Result<Self> {
        let query = CookiesQuery::new(browser).await?;
        Ok(Self { query, browser })
    }
    pub async fn get_cookies_by_host(&self, host: &str) -> Result<Vec<DecryptedCookies>> {
        let cookies = self
            .query
            .query_cookie(host)
            .await?;
        let mut decrypted_cookies = vec![];
        for mut cookie in cookies {
            let res = decrypt_cookies(&mut cookie.encrypted_value, self.browser).await?;
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
            let res = decrypt_cookies(&mut cookie.encrypted_value, self.browser).await?;
            let mut decrypted = DecryptedCookies::from(cookie);
            decrypted.set_encrypted_value(res);
            decrypted_cookies.push(decrypted);
        }
        Ok(decrypted_cookies)
    }
}
