use std::fs::File;

use binary_cookies::{
    error::Result,
    sync::{self, DecodeBinaryCookie},
};

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Need a path");

    let file = File::open(path)?;

    let a = file.decode()?;
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
