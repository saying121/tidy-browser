#[macro_use]
pub mod builder;
pub mod cookies;

use const_format::concatcp;

pub trait ChromiumPath {
    /// Browser data path
    const BASE: &'static str;
    /// Browser name for [`std::fmt::Display`]
    const NAME: &'static str;
    /// Cookies data path (sqlite3 database)
    const COOKIES: &str;
    /// Login data path (sqlite3 database)
    const LOGIN_DATA: &str;
    /// Decryption key path
    const KEY: &str;
    #[cfg(not(target_os = "windows"))]
    /// Safe keyring Storage name
    const SAFE_STORAGE: &str;
    #[cfg(target_os = "macos")]
    /// Safe keyring name
    const SAFE_NAME: &str;

    fn key() -> std::path::PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(Self::KEY)
    }
    fn key_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::KEY));
        cache
    }

    fn cookies() -> std::path::PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(Self::COOKIES)
    }
    fn cookies_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::COOKIES));
        cache
    }

    fn login_data() -> std::path::PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(Self::LOGIN_DATA)
    }
    fn login_data_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::LOGIN_DATA));
        cache
    }
}

pub trait FirefoxPath {
    /// Data path
    const BASE: &'static str;
    /// Name for [`std::fmt::Display`]
    const NAME: &'static str;
    /// Cookies data path (sqlite3 database)
    const COOKIES: &str = "cookies.sqlite";
    /// Login data path (json)
    const LOGIN_DATA: &str = "logins.json";
    /// Decryption key path
    const KEY: &str = "key4.db";

    fn key(base: &std::path::Path) -> std::path::PathBuf {
        base.join(Self::KEY)
    }
    fn key_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::KEY));
        cache
    }

    fn cookies(base: &std::path::Path) -> std::path::PathBuf {
        base.join(Self::COOKIES)
    }
    fn cookies_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::COOKIES));
        cache
    }

    fn login_data(base: &std::path::Path) -> std::path::PathBuf {
        base.join(Self::LOGIN_DATA)
    }
    fn login_data_temp() -> std::path::PathBuf {
        let mut cache = dirs::cache_dir().expect("Get cache dir failed");
        cache.push(format!("decrypt-cookies/{}", Self::LOGIN_DATA));
        cache
    }
}

macro_rules! chromium_common {
    ($platform:literal, $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal $(, $safe_name:literal)? ) => {
        #[cfg(target_os = $platform)]
        #[derive(Clone, Copy)]
        #[derive(Debug)]
        #[derive(Default)]
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        #[expect(clippy::exhaustive_structs, reason = "unit struct")]
        pub struct $browser;

        #[cfg(target_os = $platform)]
        impl ChromiumPath for $browser {
            const BASE: &'static str = $base;
            const NAME: &'static str = stringify!($browser);
            const COOKIES: &str = concatcp!($browser::BASE, "/", $cookies);
            const LOGIN_DATA: &str = concatcp!($browser::BASE, "/", $login_data);
            const KEY: &str = concatcp!($browser::BASE, "/", $key);
            $(
                const SAFE_STORAGE: &str = concatcp!($safe_name, " Safe Storage");
                #[cfg(target_os = "macos")]
                const SAFE_NAME: &str = $safe_name;
            )?
        }

        #[cfg(target_os = $platform)]
        impl std::fmt::Display for $browser {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(Self::NAME)
            }
        }
    };
}

macro_rules! chromium_base_linux {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal $(, safe_name = $safe_name:literal)? },) *) => {
        $(
            chromium_common!("linux", $browser, $base, $cookies, $login_data, $key $(, $safe_name)?);
        )*

        /// on Linux cache this
        #[cfg(target_os = "linux")]
        pub(crate) fn need_safe_storage(lab: &str) -> bool {
            matches!(
                lab,
                $(| $browser::SAFE_STORAGE)*
            )
        }
    };
}

macro_rules! chromium_base_win {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal },) *) => {
        $(
            chromium_common!("windows", $browser, $base, $cookies, $login_data, $key);
        )*
    };
}

macro_rules! chromium_base_macos {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal $(, safe_name = $safe_name:literal)? },) *) => {
        $(
            chromium_common!("macos", $browser, $base, $cookies, $login_data, $key $(, $safe_name)?);
        )*
    };
}

macro_rules! firefox_common {
    (
        $platform:literal,
        $browser:ident,
        $base:literal,
        $cookies:literal,
        $login_data:literal,
        $key:literal
    ) => {
        #[cfg(target_os = $platform)]
        #[derive(Clone, Copy)]
        #[derive(Debug)]
        #[derive(Default)]
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        #[expect(clippy::exhaustive_structs, reason = "unit struct")]
        pub struct $browser;

        #[cfg(target_os = $platform)]
        impl FirefoxPath for $browser {
            const BASE: &'static str = $base;
            const NAME: &'static str = stringify!($browser);
            const COOKIES: &str = $cookies;
            const LOGIN_DATA: &str = $login_data;
            const KEY: &str = $key;
        }

        #[cfg(target_os = $platform)]
        impl std::fmt::Display for $browser {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(Self::NAME)
            }
        }
    };
}

macro_rules! firefox_base_linux {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal $(, safe_name = $safe_name:literal)? },) *) => {
        $(
            firefox_common!("linux", $browser, $base, $cookies, $login_data, $key $(, $safe_name)?);
        )*
    };
}

macro_rules! firefox_base_win {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal },) *) => {
        $(
            firefox_common!("windows", $browser, $base, $cookies, $login_data, $key);
        )*
    };
}

macro_rules! firefox_base_macos {
    ($({ $browser:ident, $base:literal, $cookies:literal, $login_data:literal, $key:literal $(, safe_name = $safe_name:literal)? },) *) => {
        $(
            firefox_common!("macos", $browser, $base, $cookies, $login_data, $key $(, $safe_name)?);
        )*
    };
}

chromium_base_linux! {
    { Chrome  , ".config/google-chrome"              , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Chrome"         },
    { Edge    , ".config/microsoft-edge"             , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Microsoft Edge" },
    { Chromium, ".config/chromium"                   , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Chromium"       },
    { Brave   , ".config/BraveSoftware/Brave-Browser", "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Brave"          },
    { Yandex  , ".config/yandex-browser"             , "Default/Cookies", "Ya Passman Data"   , "Local State", safe_name = "Yandex"         },
    { Vivaldi , ".config/vivaldi"                    , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Vivaldi"        },
    { Opera   , ".config/opera"                      , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Opera"          },
}

chromium_base_macos! {
    { Chrome  , "Library/Application Support/Google/Chrome"              , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Chrome"         },
    { Edge    , "Library/Application Support/Microsoft Edge"             , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Microsoft Edge" },
    { Chromium, "Library/Application Support/Chromium"                   , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Chromium"       },
    { Brave   , "Library/Application Support/BraveSoftware/Brave-Browser", "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Brave"          },
    { Yandex  , "Library/Application Support/Yandex/YandexBrowser"       , "Default/Cookies", "Ya Passman Data"   , "Local State", safe_name = "Yandex"         },
    { Vivaldi , "Library/Application Support/Vivaldi"                    , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Vivaldi"        },
    { Opera   , "Library/Application Support/com.operasoftware.Opera"    , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Opera"          },
    { OperaGX , "Library/Application Support/com.operasoftware.OperaGX"  , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Opera"          },
    { CocCoc  , "Library/Application Support/Coccoc"                     , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "CocCoc"         },
    { Arc     , "Library/Application Support/Arc/User Data"              , "Default/Cookies", "Default/Login Data", "Local State", safe_name = "Arc"            },
}

chromium_base_win! {
    { Chrome  , r"AppData\Local\Google\Chrome\User Data"              , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Edge    , r"AppData\Local\Microsoft\Edge\User Data"             , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Chromium, r"AppData\Local\Chromium\User Data"                   , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Brave   , r"AppData\Local\BraveSoftware\Brave-Browser\User Data", r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Yandex  , r"AppData\Local\Yandex\YandexBrowser\User Data"       , r"Default\Network\Cookies", "Ya Passman Data"   , "Local State" },
    { Vivaldi , r"AppData\Local\Vivaldi\User Data"                    , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Opera   , r"AppData\Roaming\Opera Software\Opera Stable"        , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { OperaGX , r"AppData\Roaming\Opera Software\Opera GX Stable"     , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { CocCoc  , r"AppData\Local\CocCoc\Browser\User Data"             , r"Default\Network\Cookies", "Default/Login Data", "Local State" },
    { Arc     , r"AppData\Local\Packages\TheBrowserCompany.Arc_ttt1ap7aakyb4\LocalCache\Local\Arc\User Data", r"Default\Network\Cookies", "Default/Login Data", "Local State" },
}

firefox_base_linux! {
    { Firefox  , ".mozilla/firefox", "cookies.sqlite", "logins.json", "key4.db" },
    { Librewolf, ".librewolf"      , "cookies.sqlite", "logins.json", "key4.db" },
}
firefox_base_macos! {
    { Firefox  , "Library/Application Support/Firefox"  , "cookies.sqlite", "logins.json", "key4.db" },
    { Librewolf, "Library/Application Support/librewolf", "cookies.sqlite", "logins.json", "key4.db" },
}

firefox_base_win! {
    { Firefox  , r"AppData\Roaming\Mozilla\Firefox", "cookies.sqlite", "logins.json", "key4.db" },
    { Librewolf, r"AppData\Roaming\librewolf"      , "cookies.sqlite", "logins.json", "key4.db" },
}
