#[ignore]
#[cfg(target_os = "macos")]
#[cfg(feature = "Safari")]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn safari_binary_cookies() {
    use decrypt_cookies::prelude::*;

    let safari = SafariBuilder::new()
        .build()
        .await
        .unwrap();
    let all = safari.cookies_all();
    dbg!(&all[0]);
}
