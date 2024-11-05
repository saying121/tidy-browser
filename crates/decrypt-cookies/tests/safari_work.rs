#[ignore]
#[cfg(target_os = "macos")]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn safari_binary_cookies() {
    use decrypt_cookies::prelude::*;

    let safari = SafariBuilder::new()
        .build()
        .await
        .unwrap();
    let all = safari.all_cookies();
    dbg!(all[0]);
}
