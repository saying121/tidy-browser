use std::fs::File;

use binary_cookies::sync::{self, DecodeBinaryCookie};
use snafu::{OptionExt, ResultExt, Whatever};

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let mut args = std::env::args();
    args.next();
    let path = args
        .next()
        .whatever_context("Need a path")?;

    let file = File::open(path).whatever_context("Open file failed")?;

    let a = file
        .decode()
        .whatever_context("Bad file")?;
    let (pages_handle, _meta_decoder) = a.into_handles();
    let a = pages_handle
        .decoders()
        .filter_map(|mut v| v.decode().ok())
        .map(sync::CookieHandle::into_decoders)
        .flat_map(|v| v.filter_map(|mut v| v.decode().ok()))
        .collect::<Vec<_>>();
    dbg!(a);

    Ok(())
}
