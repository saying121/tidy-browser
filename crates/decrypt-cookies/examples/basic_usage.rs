use decrypt_cookies::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await?;
    let all_cookies = chromium.cookies_all().await?;

    dbg!(&all_cookies.first());

    let filtered_cookies = chromium
        .cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies.first());

    let firefox = FirefoxBuilder::<Firefox>::new()
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
