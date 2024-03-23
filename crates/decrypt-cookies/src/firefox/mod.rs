pub mod items;
pub mod path;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod win;

use miette::Result;

use self::items::cookie::{dao::CookiesQuery, entities::moz_cookies};
use crate::{Browser, LeetCodeCookies};

pub async fn get_session_csrf(borwser: Browser, host: &str) -> Result<LeetCodeCookies> {
    let query = CookiesQuery::new(borwser).await?;
    let cookies = query.query_cookie(host).await?;

    let mut res = LeetCodeCookies::default();

    for cookie in cookies {
        if let Some(s) = cookie.name {
            if s == "csrftoken" {
                res.csrf = cookie.value.unwrap_or_default();
            }
            else if s == "LEETCODE_SESSION" {
                res.session = cookie.value.unwrap_or_default();
            }
        }
    }
    Ok(res)
}
pub async fn get_all_cookies(borwser: Browser, host: &str) -> Result<Vec<moz_cookies::Model>> {
    let query = CookiesQuery::new(borwser).await?;
    query.query_cookie(host).await
}
