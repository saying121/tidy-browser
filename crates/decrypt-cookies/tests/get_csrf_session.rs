use std::io::Write;

use decrypt_cookies::{get_cookie, Browser, LeetCodeCookies};
use miette::{IntoDiagnostic, Result};
use strum::{EnumProperty, IntoEnumIterator};

#[ignore = "need realy environment"]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn get_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let leetcode_cn = "leetcode.cn";
    for browser in Browser::iter().skip_while(|v| {
        !v.get_str("Based")
            .unwrap()
            .eq_ignore_ascii_case("chromium")
    }) {
        dbg!(browser);
        let edge = match get_cookie(browser, leetcode_cn).await {
            Ok(v) => v,
            Err(err) => {
                println!("{err}");
                LeetCodeCookies::default()
            },
        };
        writeln!(lock, "{:#?}", edge).into_diagnostic()?;
    }

    Ok(())
}
