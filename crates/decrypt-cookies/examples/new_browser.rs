use decrypt_cookies::prelude::*;

#[expect(clippy::exhaustive_structs, reason = "example")]
pub struct NewBrowserBasedChromium;

impl ChromiumPath for NewBrowserBasedChromium {
    const BASE: &'static str = ".config/NewBrowserBasedChromium"; // See `../src/browser/mod.rs`

    const NAME: &'static str = "NewBrowserBasedChromium";

    #[cfg(not(target_os = "windows"))]
    const SAFE_STORAGE: &str = "New Safe Storage";

    #[cfg(target_os = "macos")]
    const SAFE_NAME: &str = "New";
}

#[expect(clippy::exhaustive_structs, reason = "example")]
pub struct NewBrowserBasedFirefox;

impl FirefoxPath for NewBrowserBasedFirefox {
    const BASE: &'static str = ".config/NewBrowserBasedFirefox"; // See `../src/browser/mod.rs`

    const NAME: &'static str = "NewBrowserBasedFirefox";
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chromium = ChromiumBuilder::<NewBrowserBasedChromium>::new()
        .build()
        .await?;
    let all_cookies = chromium.cookies_all().await?;

    dbg!(&all_cookies.first());

    let filtered_cookies = chromium
        .cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies.first());

    let firefox = FirefoxBuilder::<NewBrowserBasedFirefox>::new()
        .build()
        .await?;
    let all_cookies = firefox.cookies_all().await?;

    dbg!(&all_cookies.first());

    let filtered_cookies = firefox
        .cookies_filter(MozCookiesCol::Host.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies.first());

    Ok(())
}
