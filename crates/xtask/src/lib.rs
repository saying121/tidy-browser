#![expect(
    clippy::exhaustive_structs,
    reason = "Not publish, and inherit workspace lint"
)]

use std::{convert::Into, ffi::OsString};

use anyhow::Result;

pub mod args;

/// --lib --bins
pub fn check_lb() -> Result<()> {
    duct::cmd!(
        "cargo",
        "hack",
        "clippy",
        "--lib",
        "--bins",
        "--workspace",
        "--locked",
        "--each-feature",
        "--",
        "-D",
        "warnings"
    )
    .run()?;
    Ok(())
}

/// --tests --benches --examples
pub fn check_tbe() -> Result<()> {
    duct::cmd!(
        "cargo",
        "hack",
        "clippy",
        "--tests",
        "--benches",
        "--examples",
        "--all-features",
        "--workspace",
        "--locked",
        "--",
        "-D",
        "warnings"
    )
    .run()?;
    Ok(())
}

pub fn fmt_task<I: IntoIterator<Item = OsString>>(list: I) -> Result<()> {
    let args = ["+nightly", "fmt", "--all"]
        .into_iter()
        .map(Into::into)
        .chain(list);
    duct::cmd("cargo", args).run()?;
    Ok(())
}

pub fn test_task() -> Result<()> {
    if which::which("cargo-nextest").is_err() {
        duct::cmd!("cargo", "test", "--all-features").run()?;
        return Ok(());
    }

    duct::cmd!("cargo", "nextest", "run", "--all-features").run()?;
    Ok(())
}
