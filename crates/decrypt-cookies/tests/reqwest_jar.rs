use std::sync::Arc;

use decrypt_cookies::{Browser, ChromiumBuilder};
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, Url,
};

#[tokio::test]
async fn to_jar() {
    let chrmo = ChromiumBuilder::new(Browser::Edge)
        .build()
        .await
        .unwrap();

    let jar: Jar = chrmo
        .get_cookies_all()
        .await
        .unwrap()
        .into_iter()
        .collect();
    let a = jar
        .cookies(&Url::parse("http://leetcode.cn/").unwrap())
        .unwrap();
    let s = a.to_str().unwrap();
    dbg!(s);
}

#[tokio::test]
async fn jars() {
    // Create a cookie jar we can share with the HTTP client
    let jar = Arc::new(Jar::default());

    // create the HTTP client
    let client = Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .build()
        .unwrap();
}
