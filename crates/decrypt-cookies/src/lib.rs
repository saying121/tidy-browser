pub mod prelude;

pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(feature = "Safari")]
pub mod safari;

pub(crate) mod utils;

// TODO: add browser name in error
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum BuilderError {
    #[error(transparent)]
    Ini(#[from] ini::Error),
    #[error(transparent)]
    IniParser(#[from] ini::ParseError),
    #[error("Profile {0} missing `Name` properties")]
    ProfilePath(String),
    #[error("Install {0} missing `Default` properties")]
    InstallPath(String),
    #[error(transparent)]
    Decrypter(#[from] chromium_crypt::error::CryptError),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("Io: {source}, path: {path}")]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error(transparent)]
    Rawcopy(#[from] anyhow::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, BuilderError>;
