pub mod cookies;

use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use const_format::concatcp;

const CACHE_PATH: &str = "decrypt-cookies";

macro_rules! push_exact {
    ($base:ident, $val:path) => {
        let mut additional = $val.len();
        if crate::utils::need_sep(&$base) {
            additional += 1;
        }
        $base.reserve_exact(additional);

        $base.push($val);
    };
}

macro_rules! push_temp {
    ($cache:ident, $val:path) => {
        let mut $cache = dirs::cache_dir()?;
        $cache.reserve_exact(CACHE_PATH.len() + Self::NAME.len() + $val.len() + 3);
        $cache.push(CACHE_PATH);
        $cache.push(Self::NAME);
        $cache.push($val);
    };
}

pub trait ChromiumPath {
    /// Suffix for browser data path
    const BASE: &'static str;
    /// Browser name for [`std::fmt::Display`]
    const NAME: &'static str;
    #[cfg(not(target_os = "windows"))]
    /// Suffix for cookies data path (sqlite3 database)
    const COOKIES: &str = "Default/Cookies";
    #[cfg(target_os = "windows")]
    /// Suffix for cookies data path (sqlite3 database)
    const COOKIES: &str = r"Default\Network\Cookies";
    /// Suffix for login data path (sqlite3 database)
    const LOGIN_DATA: &str = "Default/Login Data";
    /// Another login data (sqlite3)
    const LOGIN_DATA_FOR_ACCOUNT: &str = "Default/Login Data For Account";
    /// Suffix for decryption key path (json)
    const KEY: &str = "Local State";
    #[cfg(not(target_os = "windows"))]
    /// Safe keyring Storage name
    const SAFE_STORAGE: &str;
    #[cfg(target_os = "macos")]
    /// Safe keyring name
    const SAFE_NAME: &str;

    /// Decryption key path (json)
    fn key(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::KEY);

        base
    }
    /// Copy the decryption key file to a location to avoid conflicts with the browser over access to it.
    fn key_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::KEY);

        cache.into()
    }

    /// Cookies path (sqlite3 database)
    fn cookies(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::COOKIES);

        base
    }
    /// Copy the cookies file to a location to avoid conflicts with the browser over access to it.
    fn cookies_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::COOKIES);

        cache.into()
    }

    /// Login data file (sqlite3 database)
    fn login_data(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::LOGIN_DATA);
        base
    }
    /// Copy the Login data file to a location to avoid conflicts with the browser over access to it.
    fn login_data_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::LOGIN_DATA);

        cache.into()
    }

    /// Login data file (sqlite3 database)
    fn login_data_for_account(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::LOGIN_DATA_FOR_ACCOUNT);
        base
    }
    /// Copy the Login data file to a location to avoid conflicts with the browser over access to it.
    fn login_data_for_account_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::LOGIN_DATA_FOR_ACCOUNT);

        cache.into()
    }
}

pub trait FirefoxPath {
    /// Suffix for data path
    const BASE: &'static str;
    /// Name for [`std::fmt::Display`]
    const NAME: &'static str;
    /// Suffix for cookies data path (sqlite3 database)
    const COOKIES: &str = "cookies.sqlite";
    /// Suffix for login data path (json)
    const LOGIN_DATA: &str = "logins.json";
    /// Suffix for decryption key path (sqlite3 database)
    const KEY: &str = "key4.db";

    /// Decryption key path (sqlite3 database)
    fn key(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::KEY);

        base
    }
    /// Copy the decryption key file to a location to avoid conflicts with the browser over access to it.
    fn key_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::KEY);

        cache.into()
    }

    /// Cookies path (sqlite3 database)
    fn cookies(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::COOKIES);

        base
    }
    /// Copy the cookies file to a location to avoid conflicts with the browser over access to it.
    fn cookies_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::COOKIES);

        cache.into()
    }

    /// Login data path (json)
    fn login_data(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::LOGIN_DATA);

        base
    }
    /// Copy the login data file to a location to avoid conflicts with the browser over access to it.
    fn login_data_temp() -> Option<PathBuf> {
        push_temp!(cache, Self::LOGIN_DATA);

        cache.into()
    }
}

macro_rules! chromium {
    ($platform:literal, $browser:ident, base: $base:literal $(, cookies: $cookies:literal)? $(, login_data: $login_data:literal)? $(, login_data_fa: $login_data_fa:literal)? $(, key: $key:literal)? $(, safe_name: $safe_name:literal)? ) => {
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
            $(const COOKIES: &str = $cookies;)?
            $(const LOGIN_DATA: &str = $login_data;)?
            $(const LOGIN_DATA_FOR_ACCOUNT: &str = $login_data_fa;)?
            $(const KEY: &str = $key;)?
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

macro_rules! firefox {
    (
        $platform:literal,
        $browser:ident,base:
        $base:literal
        $(,cookies: $cookies:literal)?
        $(,login_data: $login_data:literal)?
        $(,key = $key:literal)?
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
            $(const COOKIES: &str = $cookies;)?
            $(const LOGIN_DATA: &str = $login_data;)?
            $(const KEY: &str = $key;)?
        }

        #[cfg(target_os = $platform)]
        impl std::fmt::Display for $browser {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(Self::NAME)
            }
        }
    };
}

chromium!("linux", Chrome  , base: ".config/google-chrome"              , safe_name: "Chrome"         );
chromium!("linux", Edge    , base: ".config/microsoft-edge"             , safe_name: "Microsoft Edge" );
chromium!("linux", Chromium, base: ".config/chromium"                   , safe_name: "Chromium"       );
chromium!("linux", Brave   , base: ".config/BraveSoftware/Brave-Browser", safe_name: "Brave"          );
chromium!("linux", Vivaldi , base: ".config/vivaldi"                    , safe_name: "Vivaldi"        );
chromium!("linux", Opera   , base: ".config/opera"                      , safe_name: "Opera"          );
chromium!("linux", Yandex  , base: ".config/yandex-browser"             , login_data: "Default/Ya Passman Data", safe_name: "Yandex");

macro_rules! cach_it {
    ($($browser:ident,)*) => {
        #[cfg(target_os = "linux")]
        pub fn need_safe_storage(lab: &str) -> bool {
            matches!(
                lab,
                $(| $browser::SAFE_STORAGE)*
            )
        }
    };
}
cach_it!(Chrome, Edge, Chromium, Brave, Vivaldi, Opera, Yandex,);

chromium!("macos", Chrome  , base: "Library/Application Support/Google/Chrome"              , safe_name: "Chrome"        );
chromium!("macos", Edge    , base: "Library/Application Support/Microsoft Edge"             , safe_name: "Microsoft Edge");
chromium!("macos", Chromium, base: "Library/Application Support/Chromium"                   , safe_name: "Chromium"      );
chromium!("macos", Brave   , base: "Library/Application Support/BraveSoftware/Brave-Browser", safe_name: "Brave"         );
chromium!("macos", Vivaldi , base: "Library/Application Support/Vivaldi"                    , safe_name: "Vivaldi"       );
chromium!("macos", CocCoc  , base: "Library/Application Support/CocCoc/Browser"             , safe_name: "CocCoc"        );
chromium!("macos", Arc     , base: "Library/Application Support/Arc/User Data"              , safe_name: "Arc"           );
chromium!("macos", Opera   , base: "Library/Application Support/com.operasoftware.Opera"    , safe_name: "Opera"         );
chromium!("macos", OperaGX , base: "Library/Application Support/com.operasoftware.OperaGX"  , cookies: "Cookies", login_data: "Login Data", safe_name: "Opera");
chromium!("macos", Yandex  , base: "Library/Application Support/Yandex/YandexBrowser"       , login_data: "Default/Ya Passman Data", login_data_fa: "Default/Ya Passman Data", safe_name: "Yandex");

chromium!("windows", Chrome  , base: r"AppData\Local\Google\Chrome\User Data"               );
chromium!("windows", Edge    , base: r"AppData\Local\Microsoft\Edge\User Data"              );
chromium!("windows", Chromium, base: r"AppData\Local\Chromium\User Data"                    );
chromium!("windows", Brave   , base: r"AppData\Local\BraveSoftware\Brave-Browser\User Data" );
chromium!("windows", Vivaldi , base: r"AppData\Local\Vivaldi\User Data"                     );
chromium!("windows", Opera   , base: r"AppData\Roaming\Opera Software\Opera Stable"         );
chromium!("windows", OperaGX , base: r"AppData\Roaming\Opera Software\Opera GX Stable"      , cookies: r"Network\Cookies", login_data: r"Login Data", login_data_fa: r"Login Data For Account" );
chromium!("windows", CocCoc  , base: r"AppData\Local\CocCoc\Browser\User Data"              );
chromium!("windows", Arc     , base: r"AppData\Local\Packages\TheBrowserCompany.Arc_ttt1ap7aakyb4\LocalCache\Local\Arc\User Data" );
chromium!("windows", Yandex  , base: r"AppData\Local\Yandex\YandexBrowser\User Data"       , login_data: r"Default\Ya Passman Data" );

firefox!("linux", Firefox, base: ".mozilla/firefox");
firefox!("linux", Librewolf, base: ".librewolf");
firefox!("linux", Floorp, base: ".floorp");
firefox!("linux", Zen, base: ".zen");

firefox!("macos", Firefox, base: "Library/Application Support/Firefox");
firefox!("macos", Librewolf, base: "Library/Application Support/librewolf");
firefox!("macos", Floorp, base: "Library/Application Support/Floorp");
firefox!("macos", Zen, base: "Library/Application Support/zen");

firefox!("windows", Firefox, base: r"AppData\Roaming\Mozilla\Firefox");
firefox!("windows", Librewolf, base: r"AppData\Roaming\librewolf");
firefox!("windows", Floorp, base: r"AppData\Roaming\Floorp");
firefox!("windows", Zen, base: r"AppData\Roaming\zen");
