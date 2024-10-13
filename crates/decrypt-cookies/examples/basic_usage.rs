use decrypt_cookies::prelude::*;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let chromium = ChromiumBuilder::<Chrome>::new()
        .build()
        .await?;
    let all_cookies = chromium.get_cookies_all().await?;

    dbg!(&all_cookies[0]);

    let filtered_cookies = chromium
        .get_cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies[0]);

    let firefox = FirefoxBuilder::<Firefox>::new()?
        .build()
        .await?;
    let all_cookies = firefox.get_cookies_all().await?;

    dbg!(&all_cookies[0]);

    let filtered_cookies = firefox
        .get_cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies[0]);

    Ok(())
}
