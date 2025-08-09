use std::string::FromUtf8Error;

use snafu::{Location, Snafu};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[cfg(target_os = "linux")]
pub enum CryptoError {
    #[snafu(display("{source}\n@:{location}"))]
    Utf8 {
        source: FromUtf8Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    GetPass {
        source: secret_service::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Unpadding {
        source: aes::cipher::block_padding::UnpadError,
        #[snafu(implicit)]
        location: Location,
    },
}

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[cfg(target_os = "macos")]
pub enum CryptoError {
    #[snafu(display("{source}\n@:{location}"))]
    Utf8 {
        source: FromUtf8Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Keyring {
        source: keyring::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Unpadding {
        source: aes::cipher::block_padding::UnpadError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Task {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
}

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[cfg(target_os = "windows")]
pub enum CryptoError {
    #[snafu(display("{source}\n@:{location}"))]
    Utf8 {
        source: FromUtf8Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, path: {}\n@:{location}",path.display()))]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Base64 {
        source: base64::DecodeError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Task {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    AesGcm {
        source: aes_gcm::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    CryptUnprotectData {
        source: windows::core::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("CryptUnprotectData returned a null pointer\n@:{location}"))]
    CryptUnprotectDataNull {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    ChaCha {
        source: chacha20poly1305::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{render}\n@:{location}"))]
    Context {
        render: winnow::error::ContextError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display(r#"app_bound_encrypted_key not start with "APPB"\n@:{location}"#))]
    Appb {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Get process path failed\n@:{location}"))]
    ProcessPath {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Invalid status from `RtlAdjustPrivilege`\n@:{location}"))]
    Privilege {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("No such Process lsass.exe or winlogon.exe\n@:{location}"))]
    NotFoundProcess {
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, CryptoError>;
