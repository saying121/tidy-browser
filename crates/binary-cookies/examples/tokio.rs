use std::sync::Arc;

use binary_cookies::{error::Result, tokio::DecodeBinaryCookie};
use positioned_io::RandomAccessFile;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Need a path");

    let file = Arc::new(RandomAccessFile::open(path)?);

    let a = file.decode().await?;
    let (pages_handle, _meta_decoder) = a.into_handles();
    let mut var = vec![];
    for mut pd in pages_handle.decoders() {
        let ch = pd.decode().await?;
        for mut c in ch.decoders() {
            var.push(c.decode().await?);
        }
    }
    dbg!(var);

    Ok(())
}
