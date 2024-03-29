pub mod items;
mod utils;

use std::path::PathBuf;

pub use items::cookie::CookiesGetter;
use miette::Result;
pub use utils::binary_cookies::*;

use crate::LeetCodeCookies;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SafariGetter {
    pub cookie_getter: CookiesGetter,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SafariBuilder {
    cookies_path: Option<PathBuf>,
}

impl SafariBuilder {
    pub const fn new() -> Self {
        Self { cookies_path: None }
    }
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
    pub fn all_cookies(&self) -> Vec<&SafariCookie> {
        self.cookie_getter.all_cookies()
    }
    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        self.cookie_getter.get_session_csrf(host)
    }
    pub const fn binary_cookies(&self) -> &BinaryCookies {
        self.cookie_getter.binary_cookies()
    }
}
