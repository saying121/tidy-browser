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
    const CHROMIUM_MAC: &'static str = "Chromium/Default";
    const BRAVE_MAC: &'static str = "BraveSoftware/Brave-Browser/Default";
    const YANDEX_MAC: &'static str = "Yandex/YandexBrowser/Default";
    const VIVALDI_MAC: &'static str = "Vivaldi/Default";
    const OPERA_MAC: &'static str = "com.operasoftware.Opera/Default";
    const OPERAGX_MAC: &'static str = "com.operasoftware.OperaGX";
    const COCCOC_MAC: &'static str = "Coccoc/Default";
    const ARC_MAC: &'static str = "Arc/User Data/Default";

    pub fn new(browser: Browser) -> Self {
        let mut cookie_dir = dirs::config_local_dir().expect("get config dir failed");
        let v = match browser {
            Browser::Chrome => Self::CHROME_MAC,
            Browser::Edge => Self::EDGE_MAC,
            Browser::Chromium => Self::CHROMIUM_MAC,
            Browser::Brave => Self::BRAVE_MAC,
            Browser::Yandex => Self::YANDEX_MAC,
            Browser::Vivaldi => Self::VIVALDI_MAC,
            Browser::Opera => Self::OPERA_MAC,
            Browser::OperaGX => Self::OPERAGX_MAC,
            Browser::CocCoc => Self::COCCOC_MAC,
            Browser::Arc => Self::ARC_MAC,
            _ => {
                tracing::warn!("linux Chromium base: {browser} not support fallback Chrome");
                Self::CHROME_MAC
            },
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
