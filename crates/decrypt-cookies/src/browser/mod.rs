use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Default)]
#[derive(Debug)]
#[derive(EnumIter, Display, EnumString, AsRefStr)]
pub enum Browser {
    /// win, mac, linux
    #[default]
    Firefox,
    /// win, mac, linux
    Librewolf,

    /// win, mac, linux
    Chromium,
    /// win, mac, linux
    Chrome,
    /// win, mac, linux
    Edge,
    /// win, mac, linux
    Brave,
    /// win, mac, linux
    Yandex,
    /// win, mac, linux
    Vivaldi,
    /// win, mac, linux
    Opera,
    /// win, mac
    #[cfg(not(target_os = "linux"))]
    OperaGX,
    /// win, mac
    #[cfg(not(target_os = "linux"))]
    CocCoc,
    /// win, mac, ?
    #[cfg(not(target_os = "linux"))]
    Arc,

    /// mac
    #[cfg(target_os = "macos")]
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

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum BrowserFile {
    #[default]
    Cookies,
    Key,
    Storage,
    Passwd,
    Extensions,
    Bookmarks,
    Credit,
    Session,
    History,
}

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

impl Cookies {
    pub fn is_completion(&self) -> bool {
        !(self.csrf.is_empty() || self.session.is_empty())
    }
}

impl std::fmt::Display for Cookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf).fmt(f)
    }
}
