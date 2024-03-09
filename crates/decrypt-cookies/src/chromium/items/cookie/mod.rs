use miette::Result;

use self::dao::CookiesQuery;
use self::entities::cookies;
use crate::{chromium::utils::crypto::decrypt_cookies, Browser, Cookies};

pub mod dao;
pub mod entities;

/// get `LEETCODE_SESSION` and `csrftoken` for leetcode
pub async fn get_session_csrf(browser: Browser, host: &str) -> Result<Cookies> {
    let query = CookiesQuery::new(browser).await?;
    let mut cookies = query.query_cookie(host).await?;

    let mut res = Cookies::default();
    // the `encrypted_value` start with `v10`, so use `[3..]`
    for cookie in &mut cookies {
        if cookie.name == "csrftoken" {
            res.csrf = decrypt_cookies(&mut cookie.encrypted_value[3..].to_vec(), browser).await?;
            tracing::trace!("{:?}", &cookie.encrypted_value);
        }
        else if cookie.name == "LEETCODE_SESSION" {
            res.session =
                decrypt_cookies(&mut cookie.encrypted_value[3..].to_vec(), browser).await?;
            tracing::trace!("{:?}", &cookie.encrypted_value);
        }
    }
    Ok(res)
}
pub async fn get_all_cookies(browser: Browser, host: &str) -> Result<Vec<cookies::Model>> {
    let query = CookiesQuery::new(browser).await?;
    let mut cookies = query.query_cookie(host).await?;

    for cookie in &mut cookies {
        decrypt_cookies(&mut cookie.encrypted_value[3..].to_vec(), browser).await?;
        tracing::trace!(decrypted_value = ?cookie.encrypted_value);
    }
    Ok(cookies)
}
