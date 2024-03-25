use std::path::Path;

use miette::Result;

use self::cookie::CookiesGetter;

pub mod cookie;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
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
