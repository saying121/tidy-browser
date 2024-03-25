use self::cookie::dao::CookiesQuery;

// pub mod passwd;
pub mod cookie;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxGetter {
    pub cookies_getter: CookiesQuery,
}
