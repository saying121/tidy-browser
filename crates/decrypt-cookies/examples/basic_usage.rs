use decrypt_cookies::{browser::Chrome, prelude::*};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let chromium = ChromiumBuilder::new(Chrome::new())
        .build()
        .await?;
    let all_cookies = chromium.get_cookies_all().await?;

    dbg!(&all_cookies[0]);

    let filtered_cookies = chromium
        .get_cookies_filter(ChromiumCookieCol::HostKey.contains("google.com"))
        .await?;

    dbg!(&filtered_cookies[0]);

    Ok(())
}
