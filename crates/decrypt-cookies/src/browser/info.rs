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

    #[cfg(not(target_os = "windows"))]
    const SAFE_NAME: &'static str = "Unimplemented Safe Name";
    #[cfg(not(target_os = "windows"))]
    const SAFE_STORAGE: &'static str = "Unimplemented Safe Storage";

    fn base(&self) -> &Path;

    /// json, for windows fetch password
    #[cfg(target_os = "windows")]
    fn local_state(&self) -> PathBuf {
        let mut path = self.base().to_owned();
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
    fn safe_name(&self) -> &str {
        Self::SAFE_NAME
    }

    /// for fetch password
    #[cfg(not(target_os = "windows"))]
    fn safe_storage(&self) -> &str {
        Self::SAFE_STORAGE
    }
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
        let mut section: PathBuf = base.into();
        for (sec, prop) in ini_file {
            let Some(sec) = sec
            else {
                continue;
            };
            if sec.starts_with("Install") {
                let default = prop
                    .get("Default")
                    .unwrap_or_default();
                section.push(default);
                break;
            }
        }

        tracing::debug!("section: {:?}", section);

        let mut res = init_path;
        res.push(section);
        tracing::debug!("path: {:?}", res);

        Ok(res)
    }
}

macro_rules! chromium_builder_temp_path_impl {
    ($($browser:ident), *) => {
        $(
            impl TempPath for crate::chromium::ChromiumBuilder<$browser> {
                fn browser(&self) -> &'static str {
                    Self::NAME
                }
            }
        )*
    };
}

macro_rules! chromium_builder_new_impl {
    ($($browser:ident), *) => {
        $(
            impl crate::chromium::ChromiumBuilder<$browser> {
                pub fn new() -> Self {
                    #[cfg(target_os = "linux")]
                    let mut base = dirs::config_dir().expect("Get config dir failed");

                    #[cfg(target_os = "windows")]
                    let mut base = dirs::data_local_dir().expect("Get data local dir failed");

                    #[cfg(target_os = "macos")]
                    let mut base = dirs::config_local_dir().expect("Get config dir failed");

                    base.push($browser::BASE);

                    Self { base, __browser: core::marker::PhantomData::<$browser> }
                }
            }
        )*
    };
}

macro_rules! chromium_builder_build_impl {
    ($($browser:ident), *) => {
        $(
impl crate::chromium::ChromiumBuilder<$browser> {
    pub async fn build(self) -> Result<crate::chromium::ChromiumGetter<$browser>> {
        #[cfg(target_os = "linux")]
        let crypto = crate::chromium::crypto::linux::Decrypter::build(self.safe_storage()).await?;

        #[cfg(target_os = "macos")]
        let crypto = crate::chromium::crypto::macos::Decrypter::build(self.safe_storage(), self.safe_name())?;

        #[cfg(target_os = "windows")]
        let crypto = {
            let temp_key_path = self.local_state_temp();
            tokio::fs::copy(self.local_state(), &temp_key_path)
            .await
            .into_diagnostic()?;
            crate::chromium::crypto::win::Decrypter::build(temp_key_path).await?
        };

        let (temp_cookies_path, temp_login_data_path) =
            (self.cookies_temp(), self.login_data_temp());
        let cp_login = tokio::fs::copy(self.login_data(), &temp_login_data_path);

        let cp_cookies = tokio::fs::copy(self.cookies(), &temp_cookies_path);
        let (login, cookies) = tokio::join!(cp_login, cp_cookies);
        login.into_diagnostic()?;
        cookies.into_diagnostic()?;

        let (cookies_query, login_data_query) = (
            crate::chromium::items::cookie::cookie_dao::CookiesQuery::new(temp_cookies_path),
            crate::chromium::items::passwd::login_data_dao::LoginDataQuery::new(temp_login_data_path),
        );
        let (cookies_query, login_data_query) = tokio::join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (cookies_query?, login_data_query?);

        Ok(crate::chromium::ChromiumGetter {
            cookies_query,
            login_data_query,
            crypto,
            __browser: self.__browser,
        })
    }
}
        )*
    };
}

macro_rules! chromium_builder_new_opera_impl {
    ($($browser:ident), *) => {
        $(
            impl crate::chromium::ChromiumBuilder<$browser> {
                pub fn new() -> Self {
                    #[cfg(target_os = "linux")]
                    let mut base = dirs::config_dir().expect("Get config dir failed");

                    #[cfg(target_os = "windows")]
                    let mut base = dirs::data_dir().expect("Get data dir failed");

                    #[cfg(target_os = "macos")]
                    let mut base = dirs::config_local_dir().expect("Get config dir failed");

                    base.push($browser::BASE);

                    Self { base, __browser: core::marker::PhantomData::<$browser> }
                }
            }
        )*
    };
}

macro_rules! chromium_builder_info_impl {
    ($($browser:ident), *) => {
        $(
            impl ChromiumInfo for crate::chromium::ChromiumBuilder<$browser> {
                #[cfg(target_os = "macos")]
                const SAFE_NAME: &'static str = $browser::SAFE_NAME;
                #[cfg(not(target_os = "windows"))]
                const SAFE_STORAGE: &'static str = $browser::SAFE_STORAGE;

                fn base(&self) -> &Path {
                    &self.base
                }
            }
        )*
    };
}

macro_rules! chromium_builder_info_yandex_impl {
    ($($browser:ident), *) => {
        $(
            impl ChromiumInfo for crate::chromium::ChromiumBuilder<$browser> {
                #[cfg(target_os = "macos")]
                const SAFE_NAME: &'static str = $browser::SAFE_NAME;
                #[cfg(not(target_os = "windows"))]
                const SAFE_STORAGE: &'static str = $browser::SAFE_STORAGE;

                const LOGIN_DATA: &'static str = "Ya Passman Data"; // sqlite3

                fn base(&self) -> &Path {
                    &self.base
                }
            }
        )*
    };
}
macro_rules! chromium_getter_display_impl {
    ($($browser:ident), *) => {
        $(
impl std::fmt::Display for crate::chromium::ChromiumGetter<$browser> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str($browser::NAME)
    }
}
        )*
    }
}

chromium_builder_temp_path_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);
chromium_getter_display_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi, Opera);

chromium_builder_info_impl!(Chrome, Edge, Chromium, Brave, Vivaldi, Opera);
chromium_builder_info_yandex_impl!(Yandex);

chromium_builder_new_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi);
chromium_builder_new_opera_impl!(Opera);

chromium_builder_build_impl!(Chrome, Edge, Chromium, Brave, Yandex, Vivaldi);
chromium_builder_build_impl!(Opera);

#[cfg(not(target_os = "linux"))]
chromium_builder_temp_path_impl!(OperaGX, CocCoc, Arc);
#[cfg(not(target_os = "linux"))]
chromium_getter_display_impl!(OperaGX, CocCoc, Arc);

#[cfg(not(target_os = "linux"))]
chromium_builder_info_impl!(CocCoc, Arc);

#[cfg(not(target_os = "linux"))]
chromium_builder_new_impl!(CocCoc, Arc);
#[cfg(not(target_os = "linux"))]
chromium_builder_new_opera_impl!(OperaGX);
#[cfg(not(target_os = "linux"))]
chromium_builder_build_impl!(OperaGX, CocCoc, Arc);

#[cfg(not(target_os = "linux"))]
impl ChromiumInfo for crate::chromium::ChromiumBuilder<OperaGX> {
    #[cfg(target_os = "macos")]
    const SAFE_NAME: &'static str = OperaGX::SAFE_NAME;
    #[cfg(not(target_os = "windows"))]
    const SAFE_STORAGE: &'static str = OperaGX::SAFE_STORAGE;

    fn base(&self) -> &Path {
        &self.base
    }

    #[cfg(target_os = "windows")]
    fn local_state(&self) -> PathBuf {
        self.base().join(Self::LOCAL_STATE)
    }
}

macro_rules! firefox_impl {
    ($($browser:ident), *) => {
        $(
            impl TempPath for crate::firefox::FirefoxBuilder<$browser> {
                const NAME: &'static str = $browser::NAME;

                fn browser(&self) -> &'static str {
                    Self::NAME
                }
            }

            impl crate::firefox::FirefoxBuilder<$browser> {
                pub fn new() -> Result<Self> {
                    #[cfg(target_os = "linux")]
                    let init = dirs::home_dir().expect("Get home dir failed");
                    #[cfg(target_os = "macos")]
                    let init = dirs::config_local_dir().expect("get config local dir failed");
                    #[cfg(target_os = "windows")]
                    let init = dirs::data_dir().expect("get data local dir failed");

                    let base = Self::helper(init, $browser::BASE)?;
                    Ok(Self { base, __browser: core::marker::PhantomData::<$browser>  })
                }

                pub async fn build(self) -> Result<crate::firefox::FirefoxGetter<$browser>> {
                    let temp_cookies_path = self.cookies_temp();
                    tokio::fs::copy(self.cookies(), &temp_cookies_path)
                        .await
                        .into_diagnostic()?;

                    let query = crate::firefox::items::cookie::dao::CookiesQuery::new(temp_cookies_path).await?;

                    Ok(crate::firefox::FirefoxGetter {
                        cookies_query: query,
                        __browser: core::marker::PhantomData::<$browser>,
                    })
                }
            }

            impl FirefoxInfo for crate::firefox::FirefoxBuilder<$browser> {
                fn base(&self) -> &Path {
                    &self.base
                }
            }

        )*
    };
}

macro_rules! firefox_getter_display_impl {
    ($($browser:ident), *) => {
        $(
impl std::fmt::Display for crate::firefox::FirefoxGetter<$browser> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str($browser::NAME)
    }
}
        )*
    }
}

firefox_impl!(Firefox, Librewolf);
firefox_getter_display_impl!(Firefox, Librewolf);
