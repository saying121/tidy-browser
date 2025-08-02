use decrypt_cookies::prelude::*;
use snafu::{ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await
        .whatever_context("Chrome build failed")?;
    let all_cookies = chromium
        .cookies_all()
        .await
        .whatever_context("Chrome get all cookies failed")?;

    dbg!(&all_cookies.first());

    let filtered_cookies = chromium
        .cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await
        .whatever_context("Chrome filter cookies failed")?;

    dbg!(&filtered_cookies.first());

    let firefox = FirefoxBuilder::<Firefox>::new()
        .build()
        .await
        .whatever_context("Firefox build failed")?;
    let all_cookies = firefox
        .cookies_all()
        .await
        .whatever_context("Firefox get all cookies failed")?;

    dbg!(&all_cookies.first());

    let filtered_cookies = firefox
        .cookies_filter(MozCookiesCol::Host.contains("google.com"))
        .await
        .whatever_context("Firefox filter cookies failed")?;

    dbg!(&filtered_cookies.first());

    Ok(())
}
