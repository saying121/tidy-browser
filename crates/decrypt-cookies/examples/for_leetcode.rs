use std::io::Write;

use decrypt_cookies::{get_cookie, Browser, LeetCodeCookies};
use miette::{IntoDiagnostic, Result};
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

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
        writeln!(lock, "{:#?}", edge).into_diagnostic()?;
    }

    Ok(())
}
