pub mod args;
pub mod binary_cookies;
pub mod chromium;
pub mod cli;
pub mod error;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;
pub mod utils;

const BINARY_COOKIES_FILE_CSV: &str = "binary_cookies.csv";
const BINARY_COOKIES_FILE_JSON: &str = "binary_cookies.json";
const BINARY_COOKIES_FILE_JSONL: &str = "binary_cookies.jsonl";

const COOKIES_FILE_CSV: &str = "cookies.csv";
const COOKIES_FILE_JSON: &str = "cookies.json";
const COOKIES_FILE_JSONL: &str = "cookies.jsonl";

const LOGINS_FILE_CSV: &str = "logins.csv";
const LOGINS_FILE_JSON: &str = "logins.json";
const LOGINS_FILE_JSONL: &str = "logins.jsonl";
