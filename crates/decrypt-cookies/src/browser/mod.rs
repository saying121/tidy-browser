pub mod cookies;
pub mod info;

use std::fmt::Display;

macro_rules! browser_base {
    ($({ $browser:ident, $linux_base:literal, $win_base:literal, $mac_base:literal }), *) => {
        $(
            #[derive(Clone, Copy)]
            #[derive(Debug)]
            #[derive(Default)]
            #[derive(PartialEq, Eq, PartialOrd, Ord)]
            #[expect(clippy::exhaustive_structs)]
            pub struct $browser;

            impl $browser {
                pub const NAME: &'static str = stringify!($browser);

                #[cfg(target_os = "linux")]
                pub const BASE: &'static str = $linux_base;
                #[cfg(target_os = "windows")]
                pub const BASE: &'static str = $win_base;
                #[cfg(target_os = "macos")]
                pub const BASE: &'static str = $mac_base;
            }

            impl Display for $browser {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(Self::NAME)
                }
            }
        )*
    };
    ($({ $browser:ident, $win_base:literal, $mac_base:literal }), *) => {
        $(
            #[derive(Clone, Copy)]
            #[derive(Debug)]
            #[derive(Default)]
            #[derive(PartialEq, Eq, PartialOrd, Ord)]
            #[expect(clippy::exhaustive_structs)]
            pub struct $browser;

            impl $browser {
                pub const NAME: &'static str = stringify!($browser);

                #[cfg(target_os = "windows")]
                pub const BASE: &'static str = $win_base;
                #[cfg(target_os = "macos")]
                pub const BASE: &'static str = $mac_base;
            }

            impl Display for $browser {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(Self::NAME)
                }
            }
        )*
    };
}

macro_rules! chromium_safe {
    ($({ $browser:ident, $safe_storage:literal, $safe_name:literal }), *) => {
        $(
            impl $browser {
                #[cfg(not(target_os = "windows"))]
                pub const SAFE_STORAGE: &str = $safe_storage;
                #[cfg(target_os = "macos")]
                pub const SAFE_NAME: &str = $safe_name;
            }
        )*
    };
}

// TODO: Add more browser
browser_base!(
    { Firefox,   ".mozilla/firefox", "Mozilla/Firefox", "Firefox" },
    { Librewolf, ".librewolf",       "librewolf",       "librewolf" },

    { Chrome,   "google-chrome",               "Google/Chrome/User Data",               "Google/Chrome" },
    { Edge,     "microsoft-edge",              "Microsoft/Edge/User Data",              "Microsoft Edge" },
    { Chromium, "chromium",                    "Chromium/User Data",                    "Chromium" },
    { Brave,    "BraveSoftware/Brave-Browser", "BraveSoftware/Brave-Browser/User Data", "BraveSoftware/Brave-Browser" },
    { Yandex,   "yandex-browser",              "Yandex/YandexBrowser/User Data",        "Yandex/YandexBrowser" },
    { Vivaldi,  "vivaldi",                     "Vivaldi/User Data",                     "Vivaldi"},
    { Opera,    "opera",                       "Opera Software/Opera Stable",           "com.operasoftware.Opera"}
);
// WARN: `Arc` is no test
// TODO: Test `Arc`
#[cfg(not(target_os = "linux"))]
browser_base!(
    { OperaGX, "Opera Software/Opera GX Stable", "com.operasoftware.OperaGX" },
    { CocCoc,  "CocCoc/Browser/User Data",       "Coccoc"},
    { Arc,     "Packages/TheBrowserCompany.Arc_ttt1ap7aakyb4/LocalCache/Local/Arc/User Data",                  "Arc/User Data"}
);
chromium_safe!(
    { Chrome,   "Chrome Safe Storage",         "Chrome" },
    { Edge,     "Microsoft Edge Safe Storage", "Microsoft Edge" },
    { Chromium, "Chromium Safe Storage",       "Chromium" },
    { Brave,    "Brave Safe Storage",          "Brave" },
    { Yandex,   "Yandex Safe Storage",         "Yandex" },
    { Vivaldi,  "Vivaldi Safe Storage",        "Vivaldi" },
    { Opera,    "Opera Safe Storage",          "Opera" }
);
#[cfg(not(target_os = "linux"))]
chromium_safe!(
    { OperaGX, "Opera Safe Storage",  "Opera" },
    { CocCoc,  "CocCoc Safe Storage", "CocCoc" },
    { Arc,     "Arc Safe Storage",    "Arc" }
);
