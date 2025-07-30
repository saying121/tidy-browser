use snafu::{Location, Snafu};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[cfg(target_os = "linux")]
pub enum CryptoError {
    #[snafu(display("{source}, @:{location}"))]
    GetPass {
        source: secret_service::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
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
    #[snafu(display("{source}, @:{location}"))]
    Keyring {
        source: keyring::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    Unpadding {
        source: aes::cipher::block_padding::UnpadError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
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
    #[snafu(display("{source}, path: {}, @:{location}",path.display()))]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    Base64 {
        source: base64::DecodeError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    Task {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    AesGcm {
        source: aes_gcm::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    CryptUnprotectData {
        source: windows::core::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("CryptUnprotectData returned a null pointer, @:{location}"))]
    CryptUnprotectDataNull {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, @:{location}"))]
    ChaCha {
        source: chacha20poly1305::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{render}, @:{location}"))]
    Context {
        render: winnow::error::ContextError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display(r#"app_bound_encrypted_key not start with "APPB", @:{location}"#))]
    Appb {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Get process path failed, @:{location}"))]
    ProcessPath {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Invalid status from `RtlAdjustPrivilege`, @:{location}"))]
    Privilege {
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("No such Process lsass.exe or winlogon.exe, @:{location}"))]
    NotFoundProcess {
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, CryptoError>;
