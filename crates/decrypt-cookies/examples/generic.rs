use decrypt_cookies::{
    chromium::GetCookies as ChromiumGetCookies,
    firefox::GetCookies as FirefoxGetCookies,
    prelude::{ChromiumBuilder, *},
};
use snafu::{ResultExt, Whatever};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Client<C, F>
where
    C: ChromiumGetCookies, // + chromium::GetLogins // if require
    F: FirefoxGetCookies,
{
    chromium_cookies: C,
    firefox_cookies: F,
}

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await
        .whatever_context("Chrome build failed")?;

    let firefox = FirefoxBuilder::<Firefox>::new()
        .build()
        .await
        .whatever_context("Firefox build failed")?;

    let client = Client {
        chromium_cookies: chromium,
        firefox_cookies: firefox,
    };
    let _c = client
        .chromium_cookies
        .cookies_all()
        .await
        .whatever_context("Chrome get all cookies failed")?;
    let _c = client
        .firefox_cookies
        .cookies_all()
        .await
        .whatever_context("Firefox get all cookies failed")?;

    Ok(())
}
