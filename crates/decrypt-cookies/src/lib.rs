pub mod prelude;

pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;

#[cfg(any(target_os = "macos", feature = "binary_cookies"))]
pub(crate) mod utils;

#[cfg(feature = "binary_cookies")]
pub use utils::binary_cookies;
