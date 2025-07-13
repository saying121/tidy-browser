#[derive(Debug)]
#[derive(thiserror::Error)]
#[non_exhaustive]
#[cfg(target_os = "linux")]
pub enum CryptError {
    #[error(transparent)]
    GetPass(#[from] secret_service::Error),
    #[error("Not exists: {0}")]
    NoPass(String),
    #[error("Crypt unpad error: {0}")]
    Unpadding(aes::cipher::block_padding::UnpadError),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
#[non_exhaustive]
#[cfg(target_os = "macos")]
pub enum CryptError {
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error("Crypt unpad error: {0}")]
    Unpadding(aes::cipher::block_padding::UnpadError),
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
#[non_exhaustive]
#[cfg(target_os = "windows")]
pub enum CryptError {
    #[error("{source}, path: {path}")]
    IO {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
    #[error("{0}")]
    AesGcm(aes_gcm::Error),
    #[error(transparent)]
    CryptUnprotectData(#[from] windows::core::Error),
    #[error("CryptUnprotectData returned a null pointer")]
    CryptUnprotectDataNull,
    #[error("{0}")]
    ChaCha(chacha20poly1305::Error),
    #[error("{0}")]
    Context(winnow::error::ContextError),
}

pub type Result<T> = std::result::Result<T, CryptError>;
