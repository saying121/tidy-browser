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

#[cfg(not(target_os = "windows"))]
/// Maybe use [`std::hint::unlikely`]
#[cold]
#[inline(never)]
fn from_utf8_cold(arg: &[u8]) -> std::result::Result<String, std::string::FromUtf8Error> {
    String::from_utf8(arg.to_vec())
}

#[cfg(target_os = "windows")]
/// Maybe use [`std::hint::unlikely`]
#[cold]
#[inline(never)]
fn from_utf8_cold(arg: Vec<u8>) -> std::result::Result<String, std::string::FromUtf8Error> {
    String::from_utf8(arg)
}
