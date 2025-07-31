pub mod items;

use std::{fmt::Display, path::PathBuf};

use snafu::{Location, Snafu};

pub use self::items::cookie::CookiesGetter;
use self::items::cookie::SafariCookie;
use crate::browser::cookies::LeetCodeCookies;

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum SafariError {
    #[snafu(display("{source}\n@:{location}"))]
    Parse {
        source: binary_cookies::error::ParseError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, path: {}\n@:{location}",path.display()))]
    Io {
        path: PathBuf,
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Task {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Can not found home dir\n@:{location}"))]
    Home {
        #[snafu(implicit)]
        location: Location,
    },
}

type Result<T> = std::result::Result<T, SafariError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub struct SafariGetter {
    pub cookie_getter: CookiesGetter,
}

impl Display for SafariGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Self::NAME)
    }
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
    pub const NAME: &'static str = "Safari";

    pub fn cookies_all(&self) -> &[SafariCookie] {
        self.cookie_getter.cookies_all()
    }

    pub fn cookies_by_host<'a>(&'a self, host: &'a str) -> impl Iterator<Item = &'a SafariCookie> {
        self.cookie_getter
            .iter_cookies()
            .filter(move |v| v.domain.contains(host))
    }

    pub fn get_session_csrf(&self, host: &str) -> LeetCodeCookies {
        self.cookie_getter
            .get_session_csrf(host)
    }

    pub const fn browser() -> &'static str {
        Self::NAME
    }
}
