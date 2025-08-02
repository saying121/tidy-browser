#[cfg(any(feature = "chromium", feature = "firefox"))]
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

#[cfg(any(feature = "chromium", feature = "firefox"))]
macro_rules! push_temp {
    ($cache:ident, $val:path) => {
        let mut $cache = dirs::cache_dir()?;
        $cache.reserve_exact(CACHE_PATH.len() + Self::NAME.len() + $val.len() + 3);
        $cache.push(CACHE_PATH);
        $cache.push(Self::NAME);
        $cache.push($val);
    };
}

pub mod cookies;

#[cfg(feature = "chromium")]
pub mod chromium;
#[cfg(feature = "chromium")]
pub use chromium::*;

#[cfg(feature = "firefox")]
pub mod firefox;
#[cfg(feature = "firefox")]
pub use firefox::*;

#[cfg(any(feature = "chromium", feature = "firefox"))]
const CACHE_PATH: &str = "decrypt-cookies";
