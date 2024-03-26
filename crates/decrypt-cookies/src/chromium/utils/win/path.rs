use std::path::PathBuf;

use super::super::path::ChromiumPath;
use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct WinChromiumBase {
    base:    PathBuf,
    browser: Browser,
}

impl WinChromiumBase {
    /// consume self
    pub fn into_key(mut self) -> PathBuf {
        self.base.push(Self::LOCAL_STATE);
        self.base
    }
}

impl WinChromiumBase {
    const EDGE_WIN: &'static str = "Microsoft/Edge/User Data/Default";
    const CHROME_WIN: &'static str = "Google/Chrome/User Data/Default";
    const CHROMIUM_WIN: &'static str = "Chromium/User Data/Default";
    const BRAVE_WIN: &'static str = "BraveSoftware/Brave-Browser/User Data/Default";
    const VIVALDI_WIN: &'static str = "Vivaldi/User Data/Default";
    const COCCOC_WIN: &'static str = "CocCoc/Browser/User Data/Default";
    const YANDEX_WIN: &'static str = "Yandex/YandexBrowser/User Data/Default";
    const OPERA_WIN: &'static str = "Opera Software/Opera Stable/Default";
    const OPERAGX_WIN: &'static str = "Opera Software/Opera GX Stable";
    // const ARC_WIN: &'static str = r#"Yandex/YandexBrowser/User Data/Default"#;

    pub fn new(browser: Browser) -> Self {
        let mut cookie_dir = if matches!(browser, Browser::Opera | Browser::OperaGX) {
            dirs::data_dir().expect("get config dir failed")
        }
        else {
            dirs::data_local_dir().expect("get config dir failed")
        };

        let path_base = match browser {
            Browser::Edge => Self::EDGE_WIN,
            Browser::Chromium => Self::CHROMIUM_WIN,
            Browser::Chrome => Self::CHROME_WIN,
            Browser::Brave => Self::BRAVE_WIN,
            Browser::Yandex => Self::YANDEX_WIN,
            Browser::Vivaldi => Self::VIVALDI_WIN,
            Browser::Opera => Self::OPERA_WIN,
            Browser::OperaGX => Self::OPERAGX_WIN,
            Browser::CocCoc => Self::COCCOC_WIN,
            // Browser::Arc => Self::ARC_WIN,
            _ => {
                tracing::warn!("{browser} not support fallback Chrome.");
                Self::CHROME_WIN
            },
        };
        cookie_dir.push(path_base);

        Self { base: cookie_dir, browser }
    }
}

impl ChromiumPath for WinChromiumBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
    fn key(&self) -> PathBuf {
        // shit, quirky
        if self.browser == Browser::OperaGX {
            self.base().join(Self::LOCAL_STATE)
        }
        else {
            let mut path = self.base().clone();
            path.pop();
            path.push(Self::LOCAL_STATE);
            path
        }
    }
}
