use std::path::PathBuf;

use super::super::path::ChromiumPath;
use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct MacChromiumBase {
    base: PathBuf,
}

impl MacChromiumBase {
    const EDGE_MAC: &'static str = "Microsoft Edge/Default";
    const CHROME_MAC: &'static str = "Google/Chrome/Default";

    pub fn new(browser: Browser) -> Self {
        let mut cookie_dir = dirs::config_local_dir().expect("get config dir failed");
        let v = match browser {
            Browser::Chrome => Self::CHROME_MAC,
            _ => Self::EDGE_MAC,
        };
        cookie_dir.push(v);
        Self { base: cookie_dir }
    }
}

impl ChromiumPath for MacChromiumBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}
