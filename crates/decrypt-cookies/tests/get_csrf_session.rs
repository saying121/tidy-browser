use std::io::Write;

use decrypt_cookies::prelude::*;
use miette::{IntoDiagnostic, Result};

#[ignore = "need realy environment"]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn get_cookie_work() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let stdout = std::io::stdout();
    let mut std_lock = stdout.lock();

    let leetcode_cn = "leetcode.cn";
    let edge = ChromiumBuilder::<Edge>::new()
        .build()
        .await?;
    let ck = edge
        .get_cookies_session_csrf(leetcode_cn)
        .await?;
    writeln!(std_lock, "{:#?}", ck).into_diagnostic()?;

    Ok(())
}
