use std::path::PathBuf;

#[cfg(target_os = "macos")]
use decrypt_cookies::safari::SafariError;
use decrypt_cookies::{
    chromium::{builder::ChromiumBuilderError, ChromiumError},
    firefox::{builder::FirefoxBuilderError, FirefoxError},
};
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[derive(Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}\n@:{location}"))]
    Json {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    BinaryCookies {
        source: binary_cookies::error::ParseError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Build Chromium: {source}\n@:{location}"))]
    ChromiumBuilder {
        source: ChromiumBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Chromium: {source}\n@:{location}"))]
    Chromium {
        source: ChromiumError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Build Firefox: {source}\n@:{location}"))]
    FirefoxBuilder {
        source: FirefoxBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Firefox: {source}\n@:{location}"))]
    Firefox {
        source: FirefoxError,
        #[snafu(implicit)]
        location: Location,
    },
    #[cfg(target_os = "macos")]
    #[snafu(display("Firefox: {source}\n@:{location}"))]
    Safari {
        source: SafariError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source} path: {}\n@:{location}", path.display()))]
    Io {
        source: std::io::Error,
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    TokioTask {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
