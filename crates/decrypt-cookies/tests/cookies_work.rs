#![allow(clippy::string_slice)]

use std::str::FromStr;

use decrypt_cookies::{browser::Browser, get_cookie, ChromiumGetter, FirefoxGetter};
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
        let chrmo = ChromiumGetter::build(browser).await?;
        let a = chrmo.get_cookies_all().await?;
        for i in a.iter().take(6) {
            println!("{}, {},{}", i.name, i.expires_utc, i.creation_utc);
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

    let chrmo = FirefoxGetter::build(Browser::Firefox).await?;
    let a = chrmo.get_cookies_all().await?;
    for i in a.iter().take(6) {
        println!(
            "{}, {},{},{}",
            i.name,
            i.last_accessed.unwrap_or_default(),
            i.expiry.unwrap_or_default(),
            i.creation_time.unwrap_or_default(),
        );
    }

    Ok(())
}

#[test]
fn browsers() {
    let b = Browser::Edge;
    assert_eq!(&b.to_string(), "Edge");
    let b = Browser::from_str("Edge").unwrap();
    assert_eq!(b, Browser::Edge);
    let b = Browser::from_str("eDgE").unwrap();
    assert_eq!(b, Browser::Edge);
}

#[ignore = "just inspect"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[cfg(target_os = "linux")]
async fn all_pass() {
    use secret_service::{EncryptionType, SecretService};
    // initialize secret service (dbus connection and encryption session)
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .unwrap();
    // get default collection
    let collection = ss
        // .get_all_collections()
        .get_default_collection()
        .await
        .unwrap();
    if collection
        .is_locked()
        .await
        .unwrap()
    {
        collection.unlock().await.unwrap();
    }
    let coll = collection
        .get_all_items()
        .await
        .unwrap();
    for i in coll {
        let lab = i.get_label().await.unwrap();
        dbg!(lab);
        let res = i.get_secret().await.unwrap();
        let pass = String::from_utf8_lossy(&res).to_string();
        println!(r##"(| pass |) -> {}"##, &pass[..50.min(pass.len())]);
    }
}
