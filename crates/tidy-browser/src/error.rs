use std::path::PathBuf;

use decrypt_cookies::chromium::{builder::ChromiumBuilderError, ChromiumError};
use decrypt_cookies::firefox::builder::FirefoxBuilderError;
use decrypt_cookies::firefox::FirefoxError;
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[derive(Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Build Chromium: {source}{location}"))]
    ChromiumBuilder {
        source: ChromiumBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Chromium: {source}{location}"))]
    Chromium {
        source: ChromiumError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Build Firefox: {source}{location}"))]
    FirefoxBuilder {
        source: FirefoxBuilderError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Firefox: {source}{location}"))]
    Firefox {
        source: FirefoxError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source} path: {}{location}", path.display()))]
    Io {
        source: std::io::Error,
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}"))]
    TokioTask {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
