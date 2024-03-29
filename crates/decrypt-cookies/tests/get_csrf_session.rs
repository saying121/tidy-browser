use decrypt_cookies::{get_cookie, Browser, LeetCodeCookies};
use miette::Result;
use strum::IntoEnumIterator;

#[ignore = "need realy environment"]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn get_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let leetcode_cn = "leetcode.cn";
    for browser in Browser::iter() {
        dbg!(browser);
        let edge = match get_cookie(browser, leetcode_cn).await {
            Ok(v) => v,
            Err(err) => {
                println!("{err}");
                LeetCodeCookies::default()
            },
        };
        println!(r##"(| {leetcode_cn} |) -> {edge:#?}"##);
    }

    Ok(())
}