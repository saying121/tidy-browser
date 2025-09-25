use decrypt_cookies::{
    chromium, chromium::GetCookies, firefox, firefox::GetCookies as _, prelude::*,
};
use snafu::{ResultExt, Whatever};

chromium!("linux",   NewBrowserBasedChromium, base: ".config/chromiumbased", safe_name: "Chromiumbased");
chromium!("macos",   NewBrowserBasedChromium, base: "Library/Application Support/chromiumbased", safe_name: "Chromiumbased");
chromium!("windows", NewBrowserBasedChromium, base: r"AppData\Local\Google\chromiumbased\User Data");

firefox!("linux",   NewBrowserBasedFirefox, base: ".NewBrowserBasedFirefox");
firefox!("macos",   NewBrowserBasedFirefox, base: "Library/Application Support/NewBrowserBasedFirefox");
firefox!("windows", NewBrowserBasedFirefox, base: r"AppData\Roaming\Mozilla\NewBrowserBasedFirefox");

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    assert!(BROWSERS.contains(&"NewBrowserBasedChromium"));
    assert!(BROWSERS.contains(&"NewBrowserBasedFirefox"));

    let chromium = ChromiumBuilder::<NewBrowserBasedChromium>::new()
        .build()
        .await
        .whatever_context("New browser Chromium based build failed")?;
    let all_cookies = chromium
        .cookies_all()
        .await
        .whatever_context("New browser Chromium based get all cookies failed")?;

    dbg!(&all_cookies.first());

    let filtered_cookies = chromium
        .cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await
        .whatever_context("New browser Chromium based filter cookies failed")?;

    dbg!(&filtered_cookies.first());

    let firefox = FirefoxBuilder::<NewBrowserBasedFirefox>::new()
        .build()
        .await
        .whatever_context("New browser Firefox based build failed")?;
    let all_cookies = firefox
        .cookies_all()
        .await
        .whatever_context("New browser Firefox based get all cookies failed")?;

    dbg!(&all_cookies.first());

    let filtered_cookies = firefox
        .cookies_filter(MozCookiesCol::Host.contains("google.com"))
        .await
        .whatever_context("New browser Firefox based filter cookies failed")?;

    dbg!(&filtered_cookies.first());

    Ok(())
}
