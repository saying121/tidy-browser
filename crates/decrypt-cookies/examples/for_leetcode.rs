use std::io::Write;

use decrypt_cookies::prelude::*;
use miette::{IntoDiagnostic, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let leetcode_cn = "leetcode.cn";

    let getter = ChromiumBuilder::<Chrome>::new()
        .build()
        .await?;
    let ck = getter
        .get_cookies_session_csrf(leetcode_cn)
        .await?;
    writeln!(lock, "{:#?}", ck).into_diagnostic()?;

    Ok(())
}
