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

// TODO: add dev,nightly .etc channel
impl LinuxChromiumBase {
    const EDGE_BASE: &'static str = "microsoft-edge/Default";
    const CHROME_BASE: &'static str = "google-chrome/Default";
    // const CHROME_BASE_P1: &'static str = "google-chrome/Profile 1";
    const OPERA_BASE: &'static str = "opera/Default";
    const BRAVE_BASE: &'static str = "BraveSoftware/Brave-Browser/Default";
    const CHROMIUM_BASE: &'static str = "chromium/Default";
    const YANDEX_BASE: &'static str = "yandex-browser/Default";
    const VIVALDI_BASE: &'static str = "vivaldi/Default";

    pub fn new(browser: Browser) -> Self {
        let base = match browser {
            Browser::Edge => Self::EDGE_BASE,
            Browser::Chromium => Self::CHROMIUM_BASE,
            Browser::Chrome => Self::CHROME_BASE,
            Browser::Brave => Self::BRAVE_BASE,
            Browser::Yandex => Self::YANDEX_BASE,
            Browser::Vivaldi => Self::VIVALDI_BASE,
            Browser::Opera => Self::OPERA_BASE,
            _ => {
                tracing::warn!("linux Chromium base: {browser} not support fallback Chrome");
                Self::CHROME_BASE
            },
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
