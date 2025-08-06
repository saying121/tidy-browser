#[cfg(target_os = "linux")]
#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    use decrypt_cookies::{chromium::GetCookies as _, firefox::GetCookies, prelude::*};
    use snafu::{OptionExt, ResultExt};

    let mut p = dirs::config_dir().whatever_context("get config dir failed")?;
    p.push("google-chrome-beta");

    let chromium = ChromiumBuilder::<Chrome>::with_user_data_dir(p)
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

    let mut p = dirs::home_dir().expect("Get home dir failed");
    p.push(".mozilla/firefox-esr");

    let mut firefox = FirefoxBuilder::<Firefox>::new();
    firefox.base(p).profile("default");
    let firefox = firefox
        .build()
        .await
        .whatever_context("firefox build failed")?;

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

#[cfg(not(target_os = "linux"))]
fn main() {
    unimplemented!();
}
