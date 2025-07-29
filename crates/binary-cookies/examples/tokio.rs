use std::sync::Arc;

use binary_cookies::tokio::DecodeBinaryCookie;
use positioned_io::RandomAccessFile;
use snafu::{OptionExt, ResultExt, Whatever};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    let mut args = std::env::args();
    args.next();
    let path = args
        .next()
        .whatever_context("Need a path")?;

    let file = Arc::new(RandomAccessFile::open(path).with_whatever_context(|_| "Open file")?);

    let a = file
        .decode()
        .await
        .with_whatever_context(|_| "Bad file")?;
    let (pages_handle, _meta_decoder) = a.into_handles();
    let mut var = vec![];
    for mut pd in pages_handle.decoders() {
        let ch = pd
            .decode()
            .await
            .with_whatever_context(|e| e.to_string())?;
        for mut c in ch.decoders() {
            var.push(
                c.decode()
                    .await
                    .with_whatever_context(|e| e.to_string())?,
            );
        }
    }
    dbg!(var);

    Ok(())
}
