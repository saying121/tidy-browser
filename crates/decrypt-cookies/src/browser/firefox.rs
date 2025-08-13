use std::path::PathBuf;

use super::CACHE_PATH;

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

/// Register a Firefox based browser info
///
/// It accept
/// - `platform`
/// - `browser`: Generate a struct
/// - `base: <path>`: A browser all data location relative to home dir.
/// - `cookies: <path>`, `login_data: <path>`, `key: <path>`: Relative to profile dir. (optional)
///
/// # Example:
///
/// ```rust, no_run
/// firefox!(
///     "linux",
///     Firefox,
///     base: ".mozilla/firefox",
///     cookies: "cookies.sqlite",
///     login_data: "logins.json",
///     key: "key4.db"
/// );
/// // or omit use default value
/// firefox!("linux", Firefox, base: ".mozilla/firefox");
/// firefox!("macos", Firefox, base: "Library/Application Support/Firefox");
/// firefox!("windows", Firefox, base: r"AppData\Roaming\Mozilla\Firefox");
/// ```
#[macro_export]
macro_rules! firefox {
    (
        $platform:literal,
        $browser:ident,
        base: $base:literal
        $(, cookies: $cookies:literal)?
        $(, login_data: $login_data:literal)?
        $(, key = $key:literal)?
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

firefox!("linux", Firefox  , base: ".mozilla/firefox");
firefox!("linux", Librewolf, base: ".librewolf"      );
firefox!("linux", Floorp   , base: ".floorp"         );
firefox!("linux", Zen      , base: ".zen"            );

firefox!("macos", Firefox  , base: "Library/Application Support/Firefox"  );
firefox!("macos", Librewolf, base: "Library/Application Support/librewolf");
firefox!("macos", Floorp   , base: "Library/Application Support/Floorp"   );
firefox!("macos", Zen      , base: "Library/Application Support/zen"      );

firefox!("windows", Firefox  , base: r"AppData\Roaming\Mozilla\Firefox");
firefox!("windows", Librewolf, base: r"AppData\Roaming\librewolf"      );
firefox!("windows", Floorp   , base: r"AppData\Roaming\Floorp"         );
firefox!("windows", Zen      , base: r"AppData\Roaming\zen"            );
