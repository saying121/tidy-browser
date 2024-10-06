use std::{
    fs::{create_dir_all, read_to_string},
    path::{Path, PathBuf},
};

use miette::{IntoDiagnostic, Result};

use super::*;

/// just impl `browser` method
pub trait TempPath {
    const NAME: &'static str = "Temp";

    fn browser(&self) -> &'static str {
        Self::NAME
    }

    /// for gen temp path
    fn temp_path_prefix(&self) -> PathBuf {
        let mut temp_path = dirs::cache_dir().expect("get cache_dir failed");
        temp_path.push(format!("tidy_browser/{}", self.browser()));
        create_dir_all(&temp_path).expect("create temp path failed");

        temp_path
    }
}

/// just impl the `base` method
pub trait ChromiumInfo: TempPath {
    const BOOKMARKS: &'static str = "Bookmarks"; // json
    #[cfg(not(target_os = "windows"))]
    const COOKIES: &'static str = "Cookies"; // sqlite3
    #[cfg(target_os = "windows")]
    const COOKIES: &'static str = "Network/Cookies"; // sqlite3

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

    fn base(&self) -> &Path;

    /// json, for windows fetch password
    #[cfg(target_os = "windows")]
    fn local_state(&self) -> PathBuf {
        // shit, quirky
        if self.browser() == "OperaGX" {
            self.base().join(Self::LOCAL_STATE)
        }
        else {
            let mut path = self.base().to_owned();
            path.pop();
            path.push(Self::LOCAL_STATE);
            path
        }
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
    fn login_data(&self) -> PathBuf {
        self.base().join(Self::LOGIN_DATA)
    }
    fn login_data_temp(&self) -> PathBuf {
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
        #[cfg(not(target_os = "windows"))]
        let cookies = Self::COOKIES;
        #[cfg(target_os = "windows")]
        let cookies = "Cookies";

        self.temp_path_prefix()
            .join(cookies)
    }

    /// for fetch password
    #[cfg(target_os = "macos")]
    fn safe_name(&self) -> &str;

    /// for fetch password
    #[cfg(not(target_os = "windows"))]
    fn safe_storage(&self) -> &str;
}

/// on Linux cache this
#[cfg(target_os = "linux")]
pub(crate) fn need_safe_storage(lab: &str) -> bool {
    matches!(
        lab,
        Chromium::SAFE_STORAGE
            | Chrome::SAFE_STORAGE
            | Edge::SAFE_STORAGE
            | Brave::SAFE_STORAGE
            | Yandex::SAFE_STORAGE
            | Vivaldi::SAFE_STORAGE
            | Opera::SAFE_STORAGE
    )
}

pub trait FirefoxInfo: TempPath {
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

    // fn base_str(&self) -> &str;
    fn base(&self) -> &Path;

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

    fn helper(init_path: PathBuf, base: &str) -> Result<PathBuf> {
        let mut ini_path = init_path.clone();
        ini_path.push(format!("{}/profiles.ini", base));

        if !ini_path.exists() {
            miette::bail!(
                "{} not exists",
                ini_path
                    .to_str()
                    .unwrap_or_default()
            );
        }
        let str = read_to_string(ini_path).into_diagnostic()?;
        let ini_file = ini::Ini::load_from_str(&str).into_diagnostic()?;
        let mut section = base.to_owned();
        for (sec, prop) in ini_file {
            let Some(sec) = sec
            else {
                continue;
            };
            if sec.starts_with("Install") {
                let default = prop
                    .get("Default")
                    .unwrap_or_default();
                section.push_str(default);
                break;
            }
        }

        tracing::debug!("section: {}", section);

        let mut res = init_path;
        res.push(section);
        tracing::debug!("path: {:?}", res);

        Ok(res)
    }
}

macro_rules! chromium_temp_path_impl {
    ($($browser:ident), *) => {
        $(
            impl TempPath for $browser {
                fn browser(&self) -> &'static str {
                    Self::NAME
                }
            }
        )*
    };
}

macro_rules! chromium_impl {
    ($($browser:ident), *) => {
        $(
            impl $browser {
                pub fn new() -> Self {
                    #[cfg(target_os = "linux")]
                    let mut base = dirs::config_dir().expect("Get config dir failed");

                    #[cfg(target_os = "windows")]
                    let mut base = dirs::data_local_dir().expect("Get data local dir failed");

                    #[cfg(target_os = "macos")]
                    let mut base = dirs::config_local_dir().expect("Get config dir failed");

                    base.push(Self::BASE);

                    Self { base }
                }
            }
        )*
    };
}

macro_rules! chromium_opera_impl {
    ($($browser:ident), *) => {
        $(
            impl $browser {
                pub fn new() -> Self {
                    #[cfg(target_os = "linux")]
                    let mut base = dirs::config_dir().expect("Get config dir failed");

                    #[cfg(target_os = "windows")]
                    let mut base = dirs::data_dir().expect("Get data dir failed");

                    #[cfg(target_os = "macos")]
                    let mut base = dirs::config_local_dir().expect("Get config dir failed");

                    base.push(Self::BASE);

                    Self { base }
                }
            }
        )*
    };
}

macro_rules! chromium_info_impl {
    ($($browser:ident), *) => {
        $(
            impl ChromiumInfo for $browser {
                fn base(&self) -> &Path {
                    &self.base
                }

                #[cfg(target_os = "macos")]
                fn safe_name(&self) -> &str {
                    Self::SAFE_NAME
                }

                #[cfg(not(target_os = "windows"))]
                fn safe_storage(&self) -> &str {
                    Self::SAFE_STORAGE
                }
            }
        )*
    };
}

macro_rules! chromium_info_yandex_impl {
    ($($browser:ident), *) => {
        $(
            impl ChromiumInfo for $browser {
                const LOGIN_DATA: &'static str = "Ya Passman Data"; // sqlite3

                fn base(&self) -> &Path {
                    &self.base
                }

                #[cfg(target_os = "macos")]
                fn safe_name(&self) -> &str {
                    Self::SAFE_NAME
                }

                #[cfg(not(target_os = "windows"))]
                fn safe_storage(&self) -> &str {
                    Self::SAFE_STORAGE
                }
            }
        )*
    };
}

chromium_temp_path_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
chromium_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi);
chromium_opera_impl!(Opera);
chromium_info_impl!(Chrome, Edge, Chromium, Brave, Vivaldi, Opera);
chromium_info_yandex_impl!(Yandex);

#[cfg(not(target_os = "linux"))]
chromium_temp_path_impl!(OperaGX, CocCoc, Arc);
#[cfg(not(target_os = "linux"))]
chromium_impl!(CocCoc, Arc);
#[cfg(not(target_os = "linux"))]
chromium_opera_impl!(OperaGX);

macro_rules! firefox_impl {
    ($($browser:ident), *) => {
        $(
            impl TempPath for $browser {
                const NAME: &'static str = Self::NAME;

                fn browser(&self) -> &'static str {
                    Self::NAME
                }
            }

            impl $browser {
                pub fn new() -> Result<Self> {
                    #[cfg(target_os = "linux")]
                    let init = dirs::home_dir().expect("Get home dir failed");
                    #[cfg(target_os = "macos")]
                    let init = dirs::config_local_dir().expect("get config local dir failed");
                    #[cfg(target_os = "windows")]
                    let init = dirs::data_dir().expect("get data local dir failed");

                    let base = Self::helper(init, Self::BASE)?;
                    Ok(Self { base })
                }
            }

            impl FirefoxInfo for $browser {
                fn base(&self) -> &Path {
                    &self.base
                }
            }

            impl std::fmt::Display for $browser {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(Self::NAME)
                }
            }
        )*
    };
}

firefox_impl!(Firefox, Librewolf);
