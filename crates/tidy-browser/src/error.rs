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
    #[snafu(display("{source}, @:{location}"))]
    Json {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    BinaryCookies {
        source: binary_cookies::error::ParseError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Build Chromium: {source}, @:{location}"))]
    ChromiumBuilder {
        source: ChromiumBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Chromium: {source}, @:{location}"))]
    Chromium {
        source: ChromiumError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Build Firefox: {source}, @:{location}"))]
    FirefoxBuilder {
        source: FirefoxBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Firefox: {source}, @:{location}"))]
    Firefox {
        source: FirefoxError,
        #[snafu(implicit)]
        location: Location,
    },
    #[cfg(target_os = "macos")]
    #[snafu(display("Firefox: {source}, @:{location}"))]
    Safari {
        source: SafariError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source} path: {}, @:{location}", path.display()))]
    Io {
        source: std::io::Error,
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    TokioTask {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
