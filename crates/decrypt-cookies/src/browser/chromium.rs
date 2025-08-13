use std::path::PathBuf;

use super::CACHE_PATH;

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
    #[cfg(target_os = "windows")]
    /// Suffix for decryption key path (json)
    const KEY: &str = "Local State";
    #[cfg(not(target_os = "windows"))]
    /// Safe keyring Storage name
    const SAFE_STORAGE: &str;
    #[cfg(target_os = "macos")]
    /// Safe keyring name
    const SAFE_NAME: &str;

    #[cfg(target_os = "windows")]
    /// Decryption key path (json)
    fn key(mut base: PathBuf) -> PathBuf {
        push_exact!(base, Self::KEY);

        base
    }
    #[cfg(target_os = "windows")]
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

/// Register a Chromium based browser info
///
/// It accept
/// - `platform`
/// - `browser`: Generate a struct
/// - `base: <path>`: A browser all data location relative to home dir.
/// - `cookies: <path>`, `login_data: <path>`, `login_data_fa: <path>`: Relative to base dir. (optional)
/// - `key: <path>`: Relative to profile dir. Require on windows.
/// - `safe_name: <name>`: Require on linux and macos
///
/// # Example:
///
/// ```rust, no_run
/// chromium!(
///     "linux",
///     Chrome,
///     base: ".config/google-chrome",
///     cookies: "Default/Cookies",
///     login_data: "Default/Login Data",
///     login_data_fa: "Default/Login Data For Account",
///     safe_name: "Chrome",
/// );
/// chromium!(
///     "windows",
///     Chrome,
///     base: r"AppData\Local\Google\Chrome\User Data",
///     cookies: r"Default\Network\Cookies",
///     login_data: r"Default\Login Data",
///     login_data_fa: r"Default\Login Data For Account",
///     key: "Local State",
/// );
/// // or omit use default value
/// chromium!("linux", Chrome, base: ".config/google-chrome", safe_name: "Chrome");
/// chromium!("macos", Chrome, base: "Library/Application Support/Google/Chrome", safe_name: "Chrome");
/// chromium!("windows", Chrome, base: r"AppData\Local\Google\Chrome\User Data");
/// ```
#[macro_export]
macro_rules! chromium {
    (
        $platform:literal,
        $browser:ident,
        base: $base:literal
        $(, cookies: $cookies:literal)?
        $(, login_data: $login_data:literal)?
        $(, login_data_fa: $login_data_fa:literal)?
        $(, key: $key:literal)?
        $(, safe_name: $safe_name:literal)?
    ) => {
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
                const SAFE_STORAGE: &str = const_format::concatcp!($safe_name, " Safe Storage");
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

chromium!("linux", Chrome  , base: ".config/google-chrome"              , safe_name: "Chrome"        );
chromium!("linux", Edge    , base: ".config/microsoft-edge"             , safe_name: "Microsoft Edge");
chromium!("linux", Chromium, base: ".config/chromium"                   , safe_name: "Chromium"      );
chromium!("linux", Brave   , base: ".config/BraveSoftware/Brave-Browser", safe_name: "Brave"         );
chromium!("linux", Vivaldi , base: ".config/vivaldi"                    , safe_name: "Vivaldi"       );
chromium!("linux", Opera   , base: ".config/opera"                      , safe_name: "Opera"         );
chromium!("linux", Yandex  , base: ".config/yandex-browser"             , login_data: "Default/Ya Passman Data", safe_name: "Yandex");

macro_rules! cache_it {
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
cache_it!(Chrome, Edge, Chromium, Brave, Vivaldi, Opera, Yandex,);

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

chromium!("windows", Chrome  , base: r"AppData\Local\Google\Chrome\User Data"              );
chromium!("windows", Edge    , base: r"AppData\Local\Microsoft\Edge\User Data"             );
chromium!("windows", Chromium, base: r"AppData\Local\Chromium\User Data"                   );
chromium!("windows", Brave   , base: r"AppData\Local\BraveSoftware\Brave-Browser\User Data");
chromium!("windows", Vivaldi , base: r"AppData\Local\Vivaldi\User Data"                    );
chromium!("windows", Opera   , base: r"AppData\Roaming\Opera Software\Opera Stable"        );
chromium!("windows", OperaGX , base: r"AppData\Roaming\Opera Software\Opera GX Stable"     , cookies: r"Network\Cookies", login_data: r"Login Data", login_data_fa: r"Login Data For Account");
chromium!("windows", CocCoc  , base: r"AppData\Local\CocCoc\Browser\User Data"             );
chromium!("windows", Arc     , base: r"AppData\Local\Packages\TheBrowserCompany.Arc_ttt1ap7aakyb4\LocalCache\Local\Arc\User Data");
chromium!("windows", Yandex  , base: r"AppData\Local\Yandex\YandexBrowser\User Data"       , login_data: r"Default\Ya Passman Data");
