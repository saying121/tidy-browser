pub mod items;
mod utils;

use std::path::Path;

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

impl SafariGetter {
    pub async fn new<T>(cookie_path: Option<T>) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        let cookie_getter = CookiesGetter::build(cookie_path).await?;
        Ok(Self { cookie_getter })
    }
}
