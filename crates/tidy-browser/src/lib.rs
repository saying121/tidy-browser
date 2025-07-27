pub mod args;
pub mod chromium;
pub mod cli;
pub mod error;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;
pub mod utils;

const COOKIES_FILE: &str = "cookies.csv";
const LOGINS_FILE: &str = "logins.csv";
