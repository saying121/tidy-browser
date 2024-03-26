pub mod cookies;

use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq,PartialOrd, Ord)]
#[derive(Default)]
#[derive(Debug)]
#[derive(EnumIter, Display, EnumString, AsRefStr)]
pub enum Browser {
    /// win, mac, linux
    #[default]
    #[strum(ascii_case_insensitive)]
    Firefox,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Librewolf,

    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Chrome,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Edge,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Chromium,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Brave,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Yandex,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Vivaldi,
    /// win, mac, linux
    #[strum(ascii_case_insensitive)]
    Opera,
    /// win, mac
    #[cfg(not(target_os = "linux"))]
    #[strum(ascii_case_insensitive)]
    OperaGX,
    /// win, mac
    #[cfg(not(target_os = "linux"))]
    #[strum(ascii_case_insensitive)]
    CocCoc,
    /// win, mac, ?
    // #[cfg(not(target_os = "linux"))]
    #[cfg(target_os = "macos")]
    #[strum(ascii_case_insensitive)]
    Arc,

    /// mac
    #[cfg(target_os = "macos")]
    #[strum(ascii_case_insensitive)]
    Safari,
}

impl Browser {
    #[cfg(target_os = "macos")]
    pub const fn safe_name(&self) -> &str {
        match self {
            Self::Chromium => "Chromium",
            Self::Chrome => "Chrome",
            Self::Edge => "Microsoft Edge",
            Self::Brave => "Brave",
            Self::Yandex => "Yandex",
            Self::Vivaldi => "Vivaldi",
            Self::Opera => "Opera",
            #[cfg(not(target_os = "linux"))]
            Self::OperaGX => "Opera",
            #[cfg(not(target_os = "linux"))]
            Self::CocCoc => "CocCoc",
            #[cfg(not(target_os = "linux"))]
            Self::Arc => "Arc",
            _ => "Chrome",
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub const fn storage(&self) -> &str {
        match self {
            Self::Chromium => concat!("Chromium", " Safe Storage"),
            Self::Chrome => concat!("Chrome", " Safe Storage"),
            Self::Edge => concat!("Microsoft Edge", " Safe Storage"),
            Self::Brave => concat!("Brave", " Safe Storage"),
            Self::Yandex => concat!("Yandex", " Safe Storage"),
            Self::Vivaldi => concat!("Vivaldi", " Safe Storage"),
            Self::Opera => concat!("Opera", " Safe Storage"),
            #[cfg(not(target_os = "linux"))]
            Self::OperaGX => concat!("Opera", " Safe Storage"),
            #[cfg(not(target_os = "linux"))]
            Self::CocCoc => concat!("CocCoc", " Safe Storage"),
            #[cfg(not(target_os = "linux"))]
            Self::Arc => concat!("Arc", " Safe Storage"),
            _ => concat!("Chrome", " Safe Storage"),
        }
    }
}
