#![doc = include_str!("../README.md")]
pub mod prelude;

pub mod browser;
#[cfg(feature = "chromium")]
pub mod chromium;
#[cfg(feature = "firefox")]
pub mod firefox;
#[cfg(feature = "Safari")]
pub mod safari;

pub(crate) mod utils;

pub use pastey;

#[cfg(feature = "reqwest")]
impl<'a> FromIterator<(&'a str, &'a reqwest::Url)> for reqwest::cookie::Jar {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, &'a reqwest::Url)>,
    {
        let jar = Self::default();
        for (cookie, url) in iter {
            jar.add_cookie_str(cookie, url);
        }
        jar
    }
}
