use std::sync::Arc;

use decrypt_cookies::prelude::*;
use reqwest::cookie::Jar;
use snafu::{ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await
        .whatever_context("Chromium build failed")?;
    let all_cookies: Jar = chromium
        .cookies_all()
        .await
        .whatever_context("Get cookies failed")?
        .into_iter()
        .collect();

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(all_cookies))
        .build()
        .whatever_context("reqwest Client build failed")?;

    let resp = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .whatever_context("Get send failed")?
        .text()
        .await
        .whatever_context("get text failed")?;
    println!("{resp}");

    Ok(())
}
