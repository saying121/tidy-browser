use std::path::PathBuf;

use super::super::path::ChromiumPath;
use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct LinuxChromiumBase {
    base: PathBuf,
}

impl ChromiumPath for LinuxChromiumBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}

impl LinuxChromiumBase {
    const EDGE_BASE: &'static str = "microsoft-edge/Default";
    const CHROME_BASE: &'static str = "google-chrome/Default";
    // const CHROME_BASE_P1: &'static str = "google-chrome/Profile 1";

    pub fn new(browser: Browser) -> Self {
        let base = match browser {
            Browser::Edge => Self::EDGE_BASE,
            _ => Self::CHROME_BASE,
        };
        let mut res = dirs::config_dir().expect("get config dir failed");
        res.push(base);
        // if !res.exists() && browser == Browser::Chrome {
        //     res = dirs::config_dir().expect("get config dir failed");
        //     res.push(Self::CHROME_BASE_P1)
        // }

        Self { base: res }
    }
}
