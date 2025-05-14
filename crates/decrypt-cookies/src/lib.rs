pub mod prelude;

pub mod browser;
pub mod chromium;
pub mod firefox;
#[cfg(target_os = "macos")]
pub mod safari;

pub(crate) mod utils;

#[cfg(feature = "binary_cookies")]
pub use utils::binary_cookies;

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
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    Decrypter(#[from] crate::chromium::crypto::linux::CryptoError),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Decrypter(#[from] crate::chromium::crypto::win::CryptoError),
    #[cfg(target_os = "macos")]
    #[error(transparent)]
    Decrypt(#[from] crate::chromium::crypto::macos::CryptoError),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("Io: {source}, path: {path}")]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
}
