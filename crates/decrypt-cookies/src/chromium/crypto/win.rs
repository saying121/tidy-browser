use std::{
    ffi::c_void,
    path::{Path, PathBuf},
    ptr, slice,
};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};
use base64::{engine::general_purpose, Engine};
use tokio::{fs, task::spawn_blocking};
use windows::Win32::{Foundation, Security::Cryptography};

use crate::chromium::local_state::LocalState;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum CryptoError {
    #[error("{source}, path: {path}")]
    IO {
        path: PathBuf,
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
    Aead(String),
    #[error(transparent)]
    CryptUnprotectData(#[from] windows::core::Error),
    #[error("CryptUnprotectData returned a null pointer")]
    CryptUnprotectDataNull,
}

type Result<T> = std::result::Result<T, CryptoError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass: Vec<u8>,
}

impl Decrypter {
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=36
    /// AEAD key length in bytes.
    // const K_KEY_LENGTH: u32 = 256 / 8;

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=39
    /// AEAD nonce length in bytes.
    const K_NONCE_LENGTH: usize = 96 / 8;

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=41
    /// Version prefix for data encrypted with profile bound key.
    const K_ENCRYPTION_VERSION_PREFIX: &'static [u8] = b"v10";

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=45
    /// Key prefix for a key encrypted with DPAPI.
    const K_DPAPIKEY_PREFIX: &'static [u8] = b"DPAPI";
}

impl Decrypter {
    /// the method will use default `LocalState` path,
    /// custom that path use `DecrypterBuilder`
    pub async fn build<A: AsRef<Path> + Send + Sync>(key_path: A) -> Result<Self> {
        let pass = Self::get_pass(key_path).await?;
        Ok(Self { pass })
    }
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=108
    async fn get_pass<A: AsRef<Path> + Send + Sync>(key_path: A) -> Result<Vec<u8>> {
        let string_str = fs::read_to_string(&key_path)
            .await
            .map_err(|e| CryptoError::IO {
                path: key_path.as_ref().to_owned(),
                source: e,
            })?;
        let local_state: LocalState = serde_json::from_str(&string_str)?;
        let encrypted_key = general_purpose::STANDARD.decode(local_state.os_crypt.encrypted_key)?;
        let mut key = encrypted_key[Self::K_DPAPIKEY_PREFIX.len()..].to_vec();

        let key = spawn_blocking(move || decrypt_with_dpapi(&mut key)).await??;

        Ok(key)
    }

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=213
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        let pass = if ciphertext.starts_with(Self::K_ENCRYPTION_VERSION_PREFIX) {
            self.pass.as_slice()
        }
        else {
            return Ok(String::from_utf8_lossy(&decrypt_with_dpapi(ciphertext)?).to_string());
        };
        let prefix_len = Self::K_ENCRYPTION_VERSION_PREFIX.len();
        let nonce_len = Self::K_NONCE_LENGTH;

        let nonce = &ciphertext[prefix_len..nonce_len + prefix_len];
        let raw_ciphertext = &ciphertext[nonce_len + prefix_len..];

        let cipher = Aes256Gcm::new(GenericArray::from_slice(pass));

        match cipher.decrypt(nonce.into(), raw_ciphertext) {
            Ok(decrypted) => return Ok(String::from_utf8_lossy(&decrypted).to_string()),
            Err(e) => Err(CryptoError::Aead(e.to_string())),
        }
    }
}

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=81
pub fn decrypt_with_dpapi(ciphertext: &mut [u8]) -> Result<Vec<u8>> {
    let input = Cryptography::CRYPT_INTEGER_BLOB {
        cbData: ciphertext.len() as u32,
        pbData: ciphertext.as_mut_ptr(),
    };
    let mut output = Cryptography::CRYPT_INTEGER_BLOB { cbData: 0, pbData: ptr::null_mut() };

    unsafe {
        Cryptography::CryptUnprotectData(
            &input,
            Some(ptr::null_mut()),
            Some(ptr::null()),
            Some(ptr::null()),
            Some(ptr::null()),
            0,
            &mut output,
        )?;
    };
    if output.pbData.is_null() {
        return Err(CryptoError::CryptUnprotectDataNull);
    }

    let decrypted_data =
        unsafe { slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    let pbdata_hlocal = Foundation::HLOCAL(output.pbData.cast::<c_void>());
    unsafe {
        _ = Foundation::LocalFree(pbdata_hlocal);
    };
    Ok(decrypted_data)
}
