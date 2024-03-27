pub mod items;
mod utils;

use std::path::{Path, PathBuf};

pub use items::cookie::CookiesGetter;
use miette::Result;
pub use utils::binary_cookies::*;

use self::items::cookie::CookiesGetter;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn new() -> Self {
        Self { cookies_path: cookie_path }
    }
    pub fn cookies_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.cookies_path = path.into();
        self
    }
    pub fn build(&mut self) -> Result<SafariGetter> {
        let cookie_getter = CookiesGetter::build(self.cookies_path.take()).await?;
        Ok(SafariGetter { cookie_getter })
    }
}

impl SafariGetter {
    pub fn all_cookies(&self) -> Option<Vec<&SafariCookie>> {
        self.cookie_getter.all_cookies()
    }
    pub fn get_session_csrf(&self, host: &str) -> Option<LeetCodeCookies> {
        self.cookie_getter
            .get_session_csrf(host)
    }
    pub fn ref_binary_cookies(&self) -> Option<&BinaryCookies> {
        self.cookie_getter.ref_binary_cookies()
    }
    /// consume inner `BinaryCookies`
    pub fn get_binarycookies(&mut self) -> Option<BinaryCookies> {
        self.cookie_getter.into_inner()
    }
}
