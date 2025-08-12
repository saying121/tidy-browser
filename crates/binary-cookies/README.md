# BinaryCookies

A BinaryCookies decode and encode crate.

## Highlights

- Sans-io support sync and async.
- [Parallel](./examples/parallel.rs) for large file decoding.

## Usage

See: [examples](./examples)

```rust
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

    let file = File::open(path).with_whatever_context(|_e| "Open file failed")?;

    let a = file
        .decode()
        .with_whatever_context(|_| "Bad file")?;
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
```

## Reference

[BinaryCookies File Format](https://github.com/interstateone/BinaryCookies?tab=readme-ov-file#file-format)
