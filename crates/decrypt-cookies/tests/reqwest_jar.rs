use std::sync::Arc;

use decrypt_cookies::{Browser, ChromiumBuilder};
use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Client, Url,
};

#[tokio::test]
async fn to_jar() {
    let chrmo = ChromiumBuilder::new(Browser::Edge)
        .build()
        .await?;
    let a: Jar = chrmo
        .get_cookies_all()
        .await
        .into_iter()
        .collect()?;
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
