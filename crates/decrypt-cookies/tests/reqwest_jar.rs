use std::sync::Arc;

use decrypt_cookies::{
    browser::{cookies::CookiesInfo, Edge},
    prelude::*,
};
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, Url,
};

#[ignore]
#[tokio::test]
async fn to_jar() {
    let chrmo = ChromiumBuilder::new(Edge::new())
        .build()
        .await
        .unwrap();

    let all_cookies = chrmo
        .get_cookies_all()
        .await
        .unwrap();
    let a = all_cookies
        .iter()
        .find(|v| v.host_key.contains("leetcode.cn"))
        .unwrap();
    dbg!(a);
    let hd = all_cookies[1].get_set_cookie_header();
    dbg!(&hd);
    let jar: reqwest::cookie::Jar = all_cookies.into_iter().collect();
    let a = jar
        .cookies(&Url::parse("https://leetcode.cn/").unwrap())
        .unwrap();
    let s = a.to_str().unwrap();
    dbg!(s);
}

#[ignore]
#[tokio::test]
async fn jars() {
    // Create a cookie jar we can share with the HTTP client
    let jar = Arc::new(Jar::default());

    // create the HTTP client
    let _client = Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .build()
        .unwrap();
}
