#![doc = include_str!("../README.md")]
pub mod prelude;

pub mod browser;
#[cfg(feature = "chromium")]
pub mod chromium;
#[cfg(feature = "firefox")]
pub mod firefox;
#[cfg(feature = "Safari")]
pub mod safari;

pub(crate) mod utils;
