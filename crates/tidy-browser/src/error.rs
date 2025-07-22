use decrypt_cookies::chromium::{builder::ChromiumBuilderError, ChromiumError};
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
    #[snafu(display("Not support {name}{location}"))]
    ChromiumNotSupport {
        name: String,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
