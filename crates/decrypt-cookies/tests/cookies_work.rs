use decrypt_cookies::{browser::Browser, ChromiumBuilder, FirefoxBuilder};
use miette::Result;
use strum::IntoEnumIterator;

#[ignore = "need realy environment"]
#[tokio::test]
async fn chromium_get_all_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    for browser in Browser::iter().skip_while(|v| !v.is_chromium_base()) {
        println!("============== {browser}");
        let chrmo = ChromiumBuilder::new(browser)
            .build()
            .await?;
        let a = match chrmo.get_cookies_all().await {
            Ok(it) => it,
            Err(e) => {
                println!("{e}");
                continue;
            },
        };
        for i in a.iter().take(6) {
            println!(
                "{}, {},{}",
                i.name,
                i.expires_utc.unwrap(),
                i.creation_utc.unwrap()
            );
        }
    }

    Ok(())
}
#[ignore = "need realy environment"]
#[tokio::test]
async fn ff_get_all_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let ff = FirefoxBuilder::new(Browser::Firefox)
        .build()
        .await?;
    let a = ff.get_cookies_all().await?;
    for i in a.iter().take(6) {
        println!(
            "name: {}, last_accessed: {}, expiry: {}, creation_time: {}",
            i.name,
            i.last_accessed.unwrap(),
            i.expiry.unwrap(),
            i.creation_time.unwrap(),
        );
    }

    Ok(())
}
