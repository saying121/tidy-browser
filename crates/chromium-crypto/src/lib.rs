cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux::Decrypter;
    } else if #[cfg(target_os = "macos")] {
        pub mod mac;
        pub use mac::Decrypter;
    } else if #[cfg(target_os = "windows")] {
        pub mod win;
        pub use win::Decrypter;
    }
}

pub mod error;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Which {
    Cookie,
    Login,
}

use std::str;

#[cfg(not(target_os = "windows"))]
/// Maybe use [`std::hint::unlikely`]
#[cold]
#[inline(never)]
const fn from_utf8_cold(arg: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    str::from_utf8(arg)
}

#[cfg(target_os = "windows")]
/// Maybe use [`std::hint::unlikely`]
#[cold]
#[inline(never)]
fn from_utf8_cold(arg: Vec<u8>) -> std::result::Result<String, std::str::Utf8Error> {
    spec_from_utf8(arg)
}

#[cfg(target_os = "windows")]
/// Unified error types
#[inline(always)]
fn spec_from_utf8(arg: Vec<u8>) -> std::result::Result<String, std::str::Utf8Error> {
    match str::from_utf8(&arg) {
        Ok(..) => Ok(unsafe { String::from_utf8_unchecked(arg) }),
        Err(e) => Err(e),
    }
}
