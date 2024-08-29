use std::{fs::create_dir_all, path::PathBuf};

use chrono::{DateTime, Utc};
use miette::{IntoDiagnostic, Result};
use tokio::fs::read_to_string;

use crate::Browser;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CHROMIUM_SAFE: &str = "Chromium Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const CHROME_SAFE: &str = "Chrome Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const EDGE_SAFE: &str = "Microsoft Edge Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const BRAVE_SAFE: &str = "Brave Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const YANDEX_SAFE: &str = "Yandex Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const VIVALDI_SAFE: &str = "Vivaldi Safe Storage";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const OPERA_SAFE: &str = "Opera Safe Storage";
#[cfg(target_os = "macos")]
const OPERAGX_SAFE: &str = "Opera Safe Storage";
#[cfg(target_os = "macos")]
const COCCOC_SAFE: &str = "CocCoc Safe Storage";
#[cfg(target_os = "macos")]
const ARC_SAFE: &str = "Arc Safe Storage";

pub trait BrowserTime {
    const MAX_TIME: DateTime<Utc> = chrono::DateTime::<Utc>::MAX_UTC;
    const MIN_TIME: DateTime<Utc> = chrono::DateTime::<Utc>::MIN_UTC;
}

impl BrowserTime for i64 {}

/// just impl `browser` method
pub trait TempPath {
    fn browser(&self) -> Browser;

    /// for gen temp path
    fn temp_path_prefix(&self) -> PathBuf {
        let mut temp_path = dirs::cache_dir().expect("get cache_dir failed");
        temp_path.push(format!("tidy_browser/{}", self.browser(),));
        create_dir_all(&temp_path).expect("create temp path failed");

        temp_path
    }
}

/// just impl the `base` method
pub trait ChromiumInfo: TempPath {
    const BOOKMARKS: &'static str = "Bookmarks"; // json
    const COOKIES: &'static str = "Cookies"; // sqlite3

    // const PROFILE_PICTURE: &'static str = "Edge Profile Picture.png";
    const EXTENSION_COOKIES: &'static str = "Extension Cookies";
    // const FAVICONS: &'static str = "Favicons"; // sqlite3
    const HISTORY: &'static str = "History"; // sqlite3
    const LOAD_STATISTICS: &'static str = "load_statistics.db"; // sqlite3
    const LOGIN_DATA: &'static str = "Login Data"; // sqlite3
    const MEDIA_DEVICE_SALTS: &'static str = "MediaDeviceSalts"; // sqlite3, https://source.chromium.org/chromium/chromium/src/+/main:components/media_device_salt/README.md
    const NETWORK_ACTION_PREDICTOR: &'static str = "Network Action Predictor"; // sqlite3
    const LOCAL_STORAGE: &'static str = "Local Storage/leveldb";
    const EXTENSIONS: &'static str = "Extensions"; // a directory
    const SESSION_STORAGE: &'static str = "Session Storage"; // leveldb
    /// The webdata component manages the "web database", a `SQLite` database stored in the user's profile
    /// containing various webpage-related metadata such as autofill and web search engine data.
    const WEB_DATA: &'static str = "Web Data"; // sqlite3, https://source.chromium.org/chromium/chromium/src/+/main:components/webdata/README.md
    /// This directory contains shared files for the implementation of the <chrome://local-state> `WebUI` page.
    const LOCAL_STATE: &'static str = "Local State"; // key, json, https://source.chromium.org/chromium/chromium/src/+/main:components/local_state/README.md

    fn base(&self) -> &PathBuf;

    /// json, for windows fetch password
    fn local_state(&self) -> PathBuf {
        let mut path = self.base().clone();
        path.pop();
        path.push(Self::LOCAL_STATE);
        path
    }
    fn local_state_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::LOCAL_STATE)
    }

    /// sqlite3
    fn credit(&self) -> PathBuf {
        self.base().join(Self::WEB_DATA)
    }
    fn credit_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::WEB_DATA)
    }

    /// leveldb
    fn session(&self) -> PathBuf {
        self.base()
            .join(Self::SESSION_STORAGE)
    }
    fn session_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::SESSION_STORAGE)
    }

    /// a directory
    fn extensions(&self) -> PathBuf {
        self.base().join(Self::EXTENSIONS)
    }
    fn extensions_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::EXTENSIONS)
    }

    /// sqlite3
    fn logindata(&self) -> PathBuf {
        match self.browser() {
            Browser::Yandex => self.base().join("Ya Passman Data"),
            _ => self.base().join(Self::LOGIN_DATA),
        }
    }
    fn logindata_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::LOGIN_DATA)
    }

    /// leveldb
    fn storage(&self) -> PathBuf {
        self.base()
            .join(Self::LOCAL_STORAGE)
    }
    fn storage_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::LOCAL_STORAGE)
    }

    /// json
    fn bookmarks(&self) -> PathBuf {
        self.base().join(Self::BOOKMARKS)
    }
    fn bookmarks_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::BOOKMARKS)
    }

    /// sqlite3
    fn history(&self) -> PathBuf {
        self.base().join(Self::HISTORY)
    }
    fn history_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::HISTORY)
    }

    /// sqlite3
    fn cookies(&self) -> PathBuf {
        self.base().join(Self::COOKIES)
    }
    fn cookies_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::COOKIES)
    }

    /// for fetch password
    #[cfg(target_os = "macos")]
    fn safe_name(&self) -> &str {
        match self.browser() {
            Browser::Chromium => "Chromium",
            Browser::Chrome => "Chrome",
            Browser::Edge => "Microsoft Edge",
            Browser::Brave => "Brave",
            Browser::Yandex => "Yandex",
            Browser::Vivaldi => "Vivaldi",
            Browser::Opera => "Opera",
            #[cfg(not(target_os = "linux"))]
            Browser::OperaGX => "Opera",
            #[cfg(not(target_os = "linux"))]
            Browser::CocCoc => "CocCoc",
            #[cfg(not(target_os = "linux"))]
            Browser::Arc => "Arc",
            _ => panic!("safe storage not support: {}", self.browser()),
        }
    }

    /// for fetch password
    #[cfg(not(target_os = "windows"))]
    fn safe_storage(&self) -> &str {
        match self.browser() {
            Browser::Chromium => CHROMIUM_SAFE,
            Browser::Chrome => CHROME_SAFE,
            Browser::Edge => EDGE_SAFE,
            Browser::Brave => BRAVE_SAFE,
            Browser::Yandex => YANDEX_SAFE,
            Browser::Vivaldi => VIVALDI_SAFE,
            Browser::Opera => OPERA_SAFE,
            #[cfg(not(target_os = "linux"))]
            Browser::OperaGX => OPERAGX_SAFE,
            #[cfg(not(target_os = "linux"))]
            Browser::CocCoc => COCCOC_SAFE,
            #[cfg(not(target_os = "linux"))]
            Browser::Arc => ARC_SAFE,
            _ => panic!("safe storage not support: {}", self.browser()),
        }
    }
}

/// on Linux cache this
#[cfg(target_os = "linux")]
pub(crate) fn need_safe_storage(lab: &str) -> bool {
    matches!(
        lab,
        CHROMIUM_SAFE
            | CHROME_SAFE
            | EDGE_SAFE
            | BRAVE_SAFE
            | YANDEX_SAFE
            | VIVALDI_SAFE
            | OPERA_SAFE
    )
}

pub trait FfInfo: TempPath {
    const COOKIES: &'static str = "cookies.sqlite";
    const DATAS: &'static str = "places.sqlite"; // Bookmarks, Downloads and Browsing History:
    const BOOKMARKBACKUPS: &'static str = "bookmarkbackups/bookmarks-date.jsonlz4";
    const FAVICONS: &'static str = "favicons.sqlite"; // sqlite3, This file contains all of the favicons for your Firefox bookmarks.
    const KEY: &'static str = "key4.db"; // key sqlite3
    const PASSWD: &'static str = "logins.json"; // passwd
    const SEARCH: &'static str = "search.json.mozlz4"; // This file stores user-installed search engines.
    const STORAGE: &'static str = "webappsstore.sqlite"; // web storage data
    const EXTENSIONS: &'static str = "extensions.json";
    const CERT9: &'static str = "cert9.db"; // This file stores all your security certificate settings and any SSL certificates you have imported into Firefox.

    fn base(&self) -> &PathBuf;

    /// json
    fn extensions(&self) -> PathBuf {
        self.base().join(Self::EXTENSIONS)
    }
    fn extensions_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::EXTENSIONS)
    }

    /// json
    fn passwd(&self) -> PathBuf {
        self.base().join(Self::PASSWD)
    }
    fn passwd_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::PASSWD)
    }

    /// sqlite3
    fn storage(&self) -> PathBuf {
        self.base().join(Self::STORAGE)
    }
    fn storage_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::STORAGE)
    }

    /// sqlite3
    fn key(&self) -> PathBuf {
        self.base().join(Self::KEY)
    }
    fn key_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::KEY)
    }

    /// sqlite3
    fn datas(&self) -> PathBuf {
        self.base().join(Self::DATAS)
    }
    fn datas_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::DATAS)
    }

    /// sqlite3
    fn cookies(&self) -> PathBuf {
        self.base().join(Self::COOKIES)
    }
    fn cookies_temp(&self) -> PathBuf {
        self.temp_path_prefix()
            .join(Self::COOKIES)
    }

    fn helper(
        init_path: PathBuf,
        base: &str,
    ) -> impl std::future::Future<Output = Result<PathBuf>> + Send {
        let mut ini_path = init_path.clone();
        ini_path.push(format!("{}/profiles.ini", base));
        async move {
            if !ini_path.exists() {
                miette::bail!(
                    "{} not exists",
                    ini_path
                        .to_str()
                        .unwrap_or_default()
                );
            }
            let str = read_to_string(ini_path)
                .await
                .into_diagnostic()?;
            let ini_file = ini::Ini::load_from_str(&str).into_diagnostic()?;
            let mut section = String::new();
            for (sec, prop) in ini_file {
                let Some(sec) = sec
                else {
                    continue;
                };
                if sec.starts_with("Install") {
                    prop.get("Default")
                        .unwrap_or_default()
                        .clone_into(&mut section);
                    break;
                }
            }

            tracing::debug!("section: {}", section);

            let mut res = init_path;
            res.push(format!("{}/{}", base, section));
            tracing::debug!("path: {:?}", res);

            Ok(res)
        }
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use std::path::PathBuf;

    use miette::Result;

    use super::{ChromiumInfo, FfInfo, TempPath};
    use crate::Browser;

    #[allow(clippy::exhaustive_structs)]
    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct LinuxChromiumBase {
        pub base: PathBuf,
        pub browser: Browser,
    }

    impl TempPath for LinuxChromiumBase {
        fn browser(&self) -> Browser {
            self.browser
        }
    }

    impl ChromiumInfo for LinuxChromiumBase {
        fn base(&self) -> &PathBuf {
            &self.base
        }
    }

    // TODO: add dev,nightly .etc channel
    impl LinuxChromiumBase {
        pub const EDGE_LINUX: &'static str = "microsoft-edge/Default";
        pub const CHROME_LINUX: &'static str = "google-chrome/Default";
        pub const OPERA_LINUX: &'static str = "opera/Default";
        pub const BRAVE_LINUX: &'static str = "BraveSoftware/Brave-Browser/Default";
        pub const CHROMIUM_LINUX: &'static str = "chromium/Default";
        pub const YANDEX_LINUX: &'static str = "yandex-browser/Default";
        pub const VIVALDI_LINUX: &'static str = "vivaldi/Default";

        pub fn new(browser: Browser) -> Self {
            let base = match browser {
                Browser::Edge => Self::EDGE_LINUX,
                Browser::Chromium => Self::CHROMIUM_LINUX,
                Browser::Chrome => Self::CHROME_LINUX,
                Browser::Brave => Self::BRAVE_LINUX,
                Browser::Yandex => Self::YANDEX_LINUX,
                Browser::Vivaldi => Self::VIVALDI_LINUX,
                Browser::Opera => Self::OPERA_LINUX,
                _ => panic!("Linux Chromium base not support: {browser}"),
            };
            let mut res = dirs::config_dir().expect("get config dir failed");
            res.push(base);

            Self { base: res, browser }
        }
    }

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct LinuxFFBase {
        base: PathBuf,
        browser: Browser,
    }

    impl TempPath for LinuxFFBase {
        fn browser(&self) -> Browser {
            self.browser
        }
    }

    impl FfInfo for LinuxFFBase {
        fn base(&self) -> &PathBuf {
            &self.base
        }
    }

    impl LinuxFFBase {
        const FF_BASE: &'static str = ".mozilla/firefox";
        const LIBREWOLF_BASE: &'static str = ".librewolf";

        pub async fn new(browser: Browser) -> Result<Self> {
            let init = dirs::home_dir().ok_or_else(|| miette::miette!("get home dir failed"))?;
            let base = match browser {
                Browser::Librewolf => Self::LIBREWOLF_BASE,
                Browser::Firefox => Self::FF_BASE,
                _ => panic!("Linux Firefox base not support: {browser}"),
            };
            let base = Self::helper(init, base).await?;

            Ok(Self { base, browser })
        }
    }
}

#[cfg(target_os = "macos")]
pub mod macos {
    use std::path::PathBuf;

    use miette::Result;

    use super::{ChromiumInfo, FfInfo, TempPath};
    use crate::Browser;

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct MacChromiumBase {
        pub base: PathBuf,
        pub browser: Browser,
    }

    impl TempPath for MacChromiumBase {
        fn browser(&self) -> Browser {
            self.browser
        }
    }

    impl MacChromiumBase {
        pub const EDGE_MAC: &'static str = "Microsoft Edge/Default";
        pub const CHROME_MAC: &'static str = "Google/Chrome/Default";
        pub const CHROMIUM_MAC: &'static str = "Chromium/Default";
        pub const BRAVE_MAC: &'static str = "BraveSoftware/Brave-Browser/Default";
        pub const YANDEX_MAC: &'static str = "Yandex/YandexBrowser/Default";
        pub const VIVALDI_MAC: &'static str = "Vivaldi/Default";
        pub const OPERA_MAC: &'static str = "com.operasoftware.Opera/Default";
        pub const OPERAGX_MAC: &'static str = "com.operasoftware.OperaGX";
        pub const COCCOC_MAC: &'static str = "Coccoc/Default";
        pub const ARC_MAC: &'static str = "Arc/User Data/Default";

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
                _ => panic!("MacOs Chromium base not support: {browser}"),
            };
            cookie_dir.push(v);
            Self { base: cookie_dir, browser }
        }
    }

    impl ChromiumInfo for MacChromiumBase {
        fn base(&self) -> &PathBuf {
            &self.base
        }
    }

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct MacFFBase {
        pub base: PathBuf,
        browser: Browser,
    }

    impl TempPath for MacFFBase {
        fn browser(&self) -> Browser {
            self.browser
        }
    }

    impl FfInfo for MacFFBase {
        fn base(&self) -> &PathBuf {
            &self.base
        }
    }

    impl MacFFBase {
        const FIREFOX_BASE: &'static str = "Firefox";
        const LIBREWOLF_BASE: &'static str = "librewolf";

        pub async fn new(browser: Browser) -> Result<Self> {
            let init = dirs::config_local_dir()
                .ok_or_else(|| miette::miette!("get config local dir failed"))?;
            let base = match browser {
                Browser::Librewolf => Self::LIBREWOLF_BASE,
                Browser::Firefox => Self::FIREFOX_BASE,
                _ => panic!("MacOs Firefox base not support: {browser}"),
            };
            let base = Self::helper(init, base).await?;

            Ok(Self { base, browser })
        }
    }
}

#[cfg(target_os = "windows")]
pub mod win {
    use std::path::PathBuf;

    use miette::Result;

    use super::{ChromiumInfo, FfInfo, TempPath};
    use crate::Browser;

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct WinChromiumBase {
        base: PathBuf,
        browser: Browser,
    }

    impl TempPath for WinChromiumBase {
        fn browser(&self) -> Browser {
            self.browser
        }
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
                _ => panic!("Windows Chromium base not support: {browser}."),
            };
            cookie_dir.push(path_base);

            Self { base: cookie_dir, browser }
        }
    }

    impl ChromiumInfo for WinChromiumBase {
        const COOKIES: &'static str = "Network/Cookies"; // sqlite3

        fn base(&self) -> &PathBuf {
            &self.base
        }
        fn local_state(&self) -> PathBuf {
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
        fn cookies_temp(&self) -> PathBuf {
            self.temp_path_prefix()
                .join("Cookies")
        }
    }

    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(Default)]
    #[derive(PartialEq, Eq)]
    pub struct WinFFBase {
        base: PathBuf,
        browser: Browser,
    }

    impl TempPath for WinFFBase {
        fn browser(&self) -> Browser {
            self.browser
        }
    }

    impl WinFFBase {
        const FIREFOX_BASE: &'static str = r"Mozilla\Firefox";
        const LIBREWOLF_BASE: &'static str = "librewolf";

        pub async fn new(browser: Browser) -> Result<Self> {
            let base = match browser {
                Browser::Librewolf => Self::LIBREWOLF_BASE,
                Browser::Firefox => Self::FIREFOX_BASE,
                _ => panic!("Windows Firefox base not support: {browser}"),
            };
            let init =
                dirs::data_dir().ok_or_else(|| miette::miette!("get data local dir failed"))?;
            let base = Self::helper(init, base).await?;

            Ok(Self { base, browser })
        }
    }

    impl FfInfo for WinFFBase {
        fn base(&self) -> &PathBuf {
            &self.base
        }
    }
}
