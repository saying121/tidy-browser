use std::{ffi::c_void, path::Path, ptr};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};
use base64::{engine::general_purpose, Engine};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;
use windows::Win32::{Foundation, Security::Cryptography};

use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass:    Vec<u8>,
    browser: Browser,
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
    pub async fn build(browser: Browser, key_path: impl AsRef<Path>) -> Result<Self> {
        let pass = Self::get_pass(key_path).await?;
        Ok(Self { pass, browser })
    }
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=108
    async fn get_pass(key_path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let string_str = read_to_string(key_path)
            .await
            .into_diagnostic()?;
        let local_state: LocalState = serde_json::from_str(&string_str).into_diagnostic()?;
        let encrypted_key = general_purpose::STANDARD
            .decode(local_state.os_crypt.encrypted_key)
            .into_diagnostic()?;
        let mut key = encrypted_key[Self::K_DPAPIKEY_PREFIX.len()..].to_vec();

        let key = tokio::task::spawn_blocking(move || decrypt_with_dpapi(&mut key))
            .await
            .into_diagnostic()??;

        Ok(key)
    }

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=213
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        let pass = if ciphertext.starts_with(Self::K_ENCRYPTION_VERSION_PREFIX) {
            self.pass.as_slice()
        }
        else {
            return Ok(String::from_utf8_lossy(decrypt_with_dpapi(ciphertext)?).to_string());
        };
        let prefix_len = Self::K_ENCRYPTION_VERSION_PREFIX.len();
        let nonce_len = Self::K_NONCE_LENGTH;

        let nonce = &ciphertext[prefix_len..nonce_len + prefix_len];
        let raw_ciphertext = &ciphertext[nonce_len + prefix_len..];

        let cipher = Aes256Gcm::new(GenericArray::from_slice(pass));

        if let Ok(decrypted) = cipher.decrypt(nonce.into(), raw_ciphertext) {
            return Ok(String::from_utf8_lossy(decrypted).to_string());
        };

        miette::bail!("decrypt failed");
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct LocalState {
    pub os_crypt: OsCrypt,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct OsCrypt {
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=33
    // const K_OS_CRYPT_AUDIT_ENABLED_PREF_NAME: &[u8] = b"os_crypt.audit_enabled";
    /// Whether or not an attempt has been made to enable audit for the DPAPI
    /// encryption backing the random key.
    pub audit_enabled: bool,

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=29
    // const K_OS_CRYPT_ENCRYPTED_KEY_PREF_NAME: &[u8] = b"os_crypt.encrypted_key";
    /// Contains base64 random key encrypted with DPAPI.
    pub encrypted_key: String,
}

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=81
pub fn decrypt_with_dpapi(ciphertext: &mut [u8]) -> Result<Vec<u8>> {
    let input = Cryptography::CRYPT_INTEGER_BLOB {
        cbData: ciphertext.len() as u32,
        pbData: ciphertext.as_mut_ptr(),
    };
    let mut output = Cryptography::CRYPT_INTEGER_BLOB { cbData: 0, pbData: ptr::null_mut() };

    unsafe {
        let _: Result<_, miette::Report> = match Cryptography::CryptUnprotectData(
            &input,
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            0,
            &mut output,
        ) {
            Ok(()) => Ok(()),
            Err(_) => miette::bail!("CryptUnprotectData failed"),
        };
    }
    if output.pbData.is_null() {
        miette::bail!("CryptUnprotectData returned a null pointer");
    }

    let decrypted_data =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    let pbdata_hlocal = Foundation::HLOCAL(output.pbData.cast::<c_void>());
    unsafe {
        _ = Foundation::LocalFree(pbdata_hlocal);
    };
    Ok(decrypted_data)
}
