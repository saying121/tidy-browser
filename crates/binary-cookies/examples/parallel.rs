use std::fs::File;

use binary_cookies::{
    error::Result,
    sync::{self, DecodeBinaryCookie},
};
use rayon::prelude::{ParallelBridge, ParallelIterator};

// Unfortunately, parallelism does not always improve performance.
// In a simple test, it only brought about a 10% improvement when processing a 25MB file.
fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Need a path");

    let file = File::open(path)?;

    let a = file.decode()?;
    let (pages_handle, _meta_decoder) = a.into_handles();
    let a = pages_handle
        .decoders()
        .par_bridge()
        .filter_map(|mut v| v.decode().ok())
        .map(sync::CookieHandle::into_decoders)
        .map(|v| {
            v.par_bridge()
                .filter_map(|mut v| v.decode().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    dbg!(a);

    Ok(())
}
