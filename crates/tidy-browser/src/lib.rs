pub mod args;
pub mod binary_cookies;
pub mod chromium;
pub mod cli;
pub mod error;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;
pub mod utils;

const BINARY_COOKIES_FILE: &str = "binary_cookies.csv";
const COOKIES_FILE: &str = "cookies.csv";
const LOGINS_FILE: &str = "logins.csv";
