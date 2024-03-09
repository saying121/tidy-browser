use std::{ffi::c_void, ptr};

use aes::cipher::generic_array::GenericArray;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;
use windows::Win32::{Foundation, Security::Cryptography};

use crate::{chromium::utils::path::ChromiumPath, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass:    Vec<u8>,
    browser: Browser,
}

impl Decrypter {
    pub async fn new(browser: Browser) -> Result<Self> {
        let pass = Self::get_pass(browser).await?;
        Ok(Self { pass, browser })
    }

    pub async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
        let base = super::path::WinChromiumBase::new(browser);
        let path = base.key();
        let string_str = read_to_string(path)
            .await
            .into_diagnostic()?;
        let local_state: LocalState = serde_json::from_str(&string_str).into_diagnostic()?;
        let encrypted_key = general_purpose::STANDARD
            .decode(local_state.os_crypt.encrypted_key)
            .into_diagnostic()?;
        let mut key = encrypted_key[5..].to_vec();

        let key = tokio::task::spawn_blocking(move || decrypt_data_key(&mut key))
            .await
            .into_diagnostic()??;

        Ok(key)
    }

    pub fn decrypt(&self, encrypted_value: &mut Vec<u8>) -> Result<Vec<u8>> {
        let iv = &encrypted_value[3..15];
        let encrypted_value = &encrypted_value[15..];

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.pass));

        let Ok(decrypted) = cipher.decrypt(iv.into(), encrypted_value)
        else {
            miette::bail!("decrypt failed");
        };

        Ok(decrypted)
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
    pub audit_enabled: bool,
    pub encrypted_key: String,
}

pub fn decrypt_data_key(keydpapi: &mut [u8]) -> Result<Vec<u8>> {
    let data_in = Cryptography::CRYPT_INTEGER_BLOB {
        cbData: keydpapi.len() as u32,
        pbData: keydpapi.as_mut_ptr(),
    };
    let mut data_out = Cryptography::CRYPT_INTEGER_BLOB { cbData: 0, pbData: ptr::null_mut() };

    unsafe {
        let _: Result<_, miette::Report> = match Cryptography::CryptUnprotectData(
            &data_in,
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            0,
            &mut data_out,
        ) {
            Ok(()) => Ok(()),
            Err(_) => miette::bail!("CryptUnprotectData failed"),
        };
    }
    if data_out.pbData.is_null() {
        miette::bail!("CryptUnprotectData returned a null pointer");
    }

    let decrypted_data =
        unsafe { std::slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).to_vec() };
    let pbdata_hlocal = Foundation::HLOCAL(data_out.pbData.cast::<c_void>());
    unsafe {
        _ = Foundation::LocalFree(pbdata_hlocal);
    };
    Ok(decrypted_data)
}
