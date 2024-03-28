use decrypt_cookies::{browser::Browser, get_cookie, ChromiumBuilder, FirefoxBuilder};
use miette::Result;
use strum::IntoEnumIterator;

#[ignore = "need realy environment"]
#[tokio::test]
async fn get_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let leetcode_cn = "leetcode.cn";
    for browser in Browser::iter() {
        dbg!(browser);
        let edge = get_cookie(browser, leetcode_cn)
            .await
            .unwrap_or_default();
        println!(r##"(| {leetcode_cn} |) -> {edge:#?}"##);
    }

    Ok(())
}
#[ignore = "need realy environment"]
#[tokio::test]
async fn get_all_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    for browser in Browser::iter() {
        let chrmo = ChromiumBuilder::new(browser)
            .build()
            .await?;
        let a = chrmo.get_cookies_all().await?;
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
