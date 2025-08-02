use std::io::Write;

use decrypt_cookies::prelude::*;
use snafu::{ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let stdout = std::io::stdout();

    let leetcode_cn = "leetcode.cn";

    let getter = ChromiumBuilder::<Chrome>::new()
        .build()
        .await
        .whatever_context("Chrome build failed")?;
    let ck = getter
        .get_session_csrf(leetcode_cn)
        .await
        .whatever_context("Chrome get session csrf failed")?;

    let mut lock = stdout.lock();
    writeln!(lock, "{:#?}", ck).whatever_context("Write stdout failed")?;

    Ok(())
}
