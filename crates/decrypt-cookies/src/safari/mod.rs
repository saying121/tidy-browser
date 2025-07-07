pub mod items;

use std::path::PathBuf;

pub use self::items::cookie::CookiesGetter;
use self::items::cookie::SafariCookie;
use crate::browser::cookies::LeetCodeCookies;

type Result<T> = std::result::Result<T, crate::safari::items::cookie::CookiesGetterError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub struct SafariGetter {
    pub cookie_getter: CookiesGetter,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SafariBuilder {
    cookies_path: Option<PathBuf>,
}

impl SafariBuilder {
    pub const fn new() -> Self {
        Self { cookies_path: None }
    }

    /// If the Cookies file is not in specified location
    pub fn cookies_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.cookies_path = Some(path.into());
        self
    }

    pub async fn build(&mut self) -> Result<SafariGetter> {
        let cookie_getter = CookiesGetter::build(self.cookies_path.take()).await?;
        Ok(SafariGetter { cookie_getter })
    }
}

impl SafariGetter {
    const NAME: &'static str = "Safari";

    pub fn all_cookies(&self) -> Vec<&SafariCookie> {
        self.cookie_getter.all_cookies()
    }

    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        self.cookie_getter
            .get_session_csrf(host)
    }

    pub const fn browser() -> &'static str {
        Self::NAME
    }
}
