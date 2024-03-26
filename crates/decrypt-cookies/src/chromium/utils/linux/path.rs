use std::path::PathBuf;

use super::super::path::ChromiumPath;
use crate::Browser;

/// Get every paths
/// platform: Linux
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct LinuxChromiumBase {
    base:    PathBuf,
    browser: Browser,
}

impl ChromiumPath for LinuxChromiumBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}

// TODO: add dev,nightly .etc channel
impl LinuxChromiumBase {
    const EDGE_LINUX: &'static str = "microsoft-edge/Default";
    const CHROME_LINUX: &'static str = "google-chrome/Default";
    // const CHROME_BASE_P1: &'static str = "google-chrome/Profile 1";
    const OPERA_LINUX: &'static str = "opera/Default";
    const BRAVE_LINUX: &'static str = "BraveSoftware/Brave-Browser/Default";
    const CHROMIUM_LINUX: &'static str = "chromium/Default";
    const YANDEX_LINUX: &'static str = "yandex-browser/Default";
    const VIVALDI_LINUX: &'static str = "vivaldi/Default";

    pub fn new(browser: Browser) -> Self {
        let base = match browser {
            Browser::Edge => Self::EDGE_LINUX,
            Browser::Chromium => Self::CHROMIUM_LINUX,
            Browser::Chrome => Self::CHROME_LINUX,
            Browser::Brave => Self::BRAVE_LINUX,
            Browser::Yandex => Self::YANDEX_LINUX,
            Browser::Vivaldi => Self::VIVALDI_LINUX,
            Browser::Opera => Self::OPERA_LINUX,
            _ => {
                tracing::warn!("linux Chromium base: {browser} not support fallback Chrome");
                Self::CHROME_LINUX
            },
        };
        let mut res = dirs::config_dir().expect("get config dir failed");
        res.push(base);
        // if !res.exists() && browser == Browser::Chrome {
        //     res = dirs::config_dir().expect("get config dir failed");
        //     res.push(Self::CHROME_BASE_P1)
        // }

        Self { base: res, browser }
    }
}
