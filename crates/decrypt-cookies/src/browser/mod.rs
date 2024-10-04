pub mod cookies;
pub mod info;

use std::path::PathBuf;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Firefox {
    base: PathBuf,
}

impl Firefox {
    pub const NAME: &'static str = "Firefox";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = ".mozilla/firefox";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = r"Mozilla\Firefox";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Firefox";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Librewolf {
    base: PathBuf,
}

impl Librewolf {
    pub const NAME: &'static str = "Librewolf";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = ".librewolf";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "librewolf";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "librewolf";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Chrome {
    base: PathBuf,
}

impl Chrome {
    pub const NAME: &'static str = "Chrome";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "google-chrome/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Google/Chrome/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Google/Chrome/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Chrome Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Chrome";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    base: PathBuf,
}

impl Edge {
    pub const NAME: &'static str = "Edge";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "microsoft-edge/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Microsoft/Edge/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Microsoft Edge/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Microsoft Edge Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Microsoft Edge";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Chromium {
    base: PathBuf,
}

impl Chromium {
    pub const NAME: &'static str = "Chromium";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "chromium/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Chromium/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Chromium/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Chromium Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Chromium";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Brave {
    base: PathBuf,
}

impl Brave {
    pub const NAME: &'static str = "Brave";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "BraveSoftware/Brave-Browser/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "BraveSoftware/Brave-Browser/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "BraveSoftware/Brave-Browser/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Brave Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Brave";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Yandex {
    base: PathBuf,
}

impl Yandex {
    pub const NAME: &'static str = "Yandex";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "yandex-browser/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Yandex/YandexBrowser/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Yandex/YandexBrowser/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Yandex Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Yandex";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Vivaldi {
    base: PathBuf,
}

impl Vivaldi {
    pub const NAME: &'static str = "Vivaldi";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "vivaldi/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Vivaldi/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Vivaldi/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Vivaldi Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Vivaldi";
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Opera {
    base: PathBuf,
}

impl Opera {
    pub const NAME: &'static str = "Opera";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "opera/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Opera Software/Opera Stable/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "com.operasoftware.Opera/Default";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const SAFE_STORAGE: &str = "Opera Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Opera";
}

#[cfg(not(target_os = "linux"))]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct OperaGX {
    base: PathBuf,
}

#[cfg(not(target_os = "linux"))]
impl OperaGX {
    pub const NAME: &'static str = "OperaGX";

    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Opera Software/Opera GX Stable";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "com.operasoftware.OperaGX";

    #[cfg(target_os = "macos")]
    pub const SAFE_STORAGE: &str = "Opera Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Opera";
}

#[cfg(not(target_os = "linux"))]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CocCoc {
    base: PathBuf,
}

#[cfg(not(target_os = "linux"))]
impl CocCoc {
    pub const NAME: &'static str = "CocCoc";

    #[cfg(target_os = "linux")]
    pub const BASE: &'static str = "vivaldi/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "CocCoc/Browser/User Data/Default";
    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Coccoc/Default";

    #[cfg(target_os = "macos")]
    pub const SAFE_STORAGE: &str = "CocCoc Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "CocCoc";
}

#[cfg(not(target_os = "linux"))]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Arc {
    base: PathBuf,
}

// WARN: test it
#[cfg(not(target_os = "linux"))]
impl Arc {
    pub const NAME: &'static str = "Arc";

    #[cfg(target_os = "macos")]
    pub const BASE: &'static str = "Arc/User Data/Default";
    #[cfg(target_os = "windows")]
    pub const BASE: &'static str = "Arc/User Data/Default";

    #[cfg(target_os = "macos")]
    pub const SAFE_STORAGE: &str = "Arc Safe Storage";
    #[cfg(target_os = "macos")]
    pub const SAFE_NAME: &str = "Arc";
}
