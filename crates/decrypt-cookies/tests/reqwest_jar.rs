use decrypt_cookies::{
    browser::{cookies::CookiesInfo, Edge},
    prelude::*,
};
use reqwest::{cookie::CookieStore, Url};

#[ignore]
#[tokio::test]
async fn to_jar() {
    let chrmo = ChromiumBuilder::<Edge>::new()
        .build()
        .await
        .unwrap();

    let all_cookies = chrmo.cookies_all().await.unwrap();
    let hd = all_cookies[1].set_cookie_header();
    dbg!(&hd);
    let jar: reqwest::cookie::Jar = all_cookies.into_iter().collect();
    let a = jar
        .cookies(&Url::parse("https://leetcode.cn/").unwrap())
        .unwrap();
    let s = a.to_str().unwrap();
    dbg!(s);
}
