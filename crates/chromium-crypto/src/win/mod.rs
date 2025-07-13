pub mod local_state;

use std::{ffi::c_void, path::Path, ptr, slice};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, Key, KeyInit,
};
use base64::{engine::general_purpose, Engine};
use chacha20poly1305::ChaCha20Poly1305;
use local_state::LocalState;
use tokio::{fs, task::spawn_blocking};
use windows::{
    core::w,
    Win32::{
        Foundation,
        Security::Cryptography::{
            self, NCryptOpenKey, NCryptOpenStorageProvider, NCRYPT_KEY_HANDLE, NCRYPT_PROV_HANDLE,
        },
    },
};
use winnow::{binary::le_u32, error::StrContext, token::take, Parser};

use crate::error::{CryptError, Result};

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
    pub async fn build_v20<A: AsRef<Path> + Send + Sync>(key_path: A) -> Result<Self> {
        let pass = Self::get_pass_v20(key_path).await?;
        Ok(Self { pass })
    }

    async fn get_pass_v20<A: AsRef<Path> + Send + Sync>(key_path: A) -> Result<Vec<u8>> {
        let string_str = fs::read_to_string(&key_path)
            .await
            .map_err(|e| CryptError::IO {
                path: key_path.as_ref().to_owned(),
                source: e,
            })?;
        let local_state: LocalState = serde_json::from_str(&string_str)?;
        let encrypted_key = general_purpose::STANDARD.decode(
            local_state
                .os_crypt
                .app_bound_encrypted_key
                .expect("todo handle"),
        )?;

        debug_assert!(encrypted_key.starts_with(Self::K_DPAPIKEY_PREFIX));

        let mut key = encrypted_key[Self::K_DPAPIKEY_PREFIX.len()..].to_vec();

        let key = spawn_blocking(move || decrypt_with_dpapi(&mut key)).await??;

        Ok(key)
    }

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
            .map_err(|e| CryptError::IO {
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

        cipher
            .decrypt(nonce.into(), raw_ciphertext)
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .map_err(CryptError::AesGcm)
    }
}

enum KeyData<'k> {
    One {
        iv: &'k [u8],
        ciphertext: &'k [u8],
        // tag: &'k [u8],
    },
    Two {
        iv: &'k [u8],
        ciphertext: &'k [u8],
        // tag: &'k [u8],
    },
    Three {
        enctypted_aes_key: &'k [u8],
        iv: &'k [u8],
        ciphertext: &'k [u8],
        // tag: &'k [u8],
    },
}

fn parse_key_blob<'k>(blob_data: &mut &'k [u8]) -> winnow::Result<KeyData<'k>> {
    let header_len = le_u32(blob_data)? as usize;
    let _header = take(header_len).parse_next(blob_data)?;
    let _content_len = le_u32(blob_data)? as usize;

    let flag = take(1_usize).parse_next(blob_data)?[0];
    match flag {
        1 => Ok(KeyData::One {
            iv: take(12_usize).parse_next(blob_data)?,
            ciphertext: take(32_usize).parse_next(blob_data)?,
            // tag: take(16_usize).parse_next(blob_data)?,
        }),
        2 => Ok(KeyData::Two {
            iv: take(12_usize).parse_next(blob_data)?,
            ciphertext: take(32_usize).parse_next(blob_data)?,
            // tag: take(16_usize).parse_next(blob_data)?,
        }),
        3 => Ok(KeyData::Three {
            enctypted_aes_key: take(32_usize).parse_next(blob_data)?,
            iv: take(12_usize).parse_next(blob_data)?,
            ciphertext: take(32_usize).parse_next(blob_data)?,
            // tag: take(16_usize).parse_next(blob_data)?,
        }),
        _ => {
            let mut err = winnow::error::ContextError::new();
            err.push(StrContext::Label("Bad key flag"));
            Err(err)
        },
    }
}

fn decrypt_with_cng(keydpapi: &[u8]) -> Result<Vec<u8>> {
    let mut phprovider = NCRYPT_PROV_HANDLE::default();
    unsafe {
        let pszprovidername = w!("Microsoft Software Key Storage Provider");
        NCryptOpenStorageProvider(&mut phprovider, pszprovidername, 0)?;
    };
    let mut hkey = NCRYPT_KEY_HANDLE::default();
    unsafe {
        NCryptOpenKey(
            phprovider,
            &mut hkey,
            w!("Google Chromekey1"),
            Cryptography::CERT_KEY_SPEC::default(),
            Cryptography::NCRYPT_FLAGS::default(),
        )?;
    };

    let mut output_buffer = vec![0; keydpapi.len()];
    let mut output_len = 0;
    unsafe {
        Cryptography::NCryptDecrypt(
            hkey,
            keydpapi.into(),
            None,
            Some(&mut output_buffer),
            &mut output_len,
            Cryptography::NCRYPT_SILENT_FLAG,
        )?;
    };

    unsafe {
        Cryptography::NCryptFreeObject(hkey.into())?;
    };
    unsafe {
        Cryptography::NCryptFreeObject(phprovider.into())?;
    };
    output_buffer.truncate(output_len as usize);

    Ok(output_buffer)
}

fn derive_v20_master_key(key_data: &KeyData) -> Result<Vec<u8>> {
    match key_data {
        KeyData::One { iv, ciphertext, .. } => {
            let aes_key = b"\xB3\x1C\x6E\x24\x1A\xC8\x46\x72\x8D\xA9\xC1\xFA\xC4\x93\x66\x51\xCF\xFB\x94\x4D\x14\x3A\xB8\x16\x27\x6B\xCC\x6D\xA0\x28\x47\x87";
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(aes_key));
            let nonce = GenericArray::from_slice(iv);
            cipher
                .decrypt(nonce, *ciphertext)
                .map_err(CryptError::AesGcm)
        },
        KeyData::Two { iv, ciphertext, .. } => {
            let chacha_key = b"\xE9\x8F\x37\xD7\xF4\xE1\xFA\x43\x3D\x19\x30\x4D\xC2\x25\x80\x42\x09\x0E\x2D\x1D\x7E\xEA\x76\x70\xD4\x1F\x73\x8D\x08\x72\x96\x60";
            let cipher = ChaCha20Poly1305::new(Key::<ChaCha20Poly1305>::from_slice(chacha_key));
            cipher
                .decrypt(GenericArray::from_slice(iv), *ciphertext)
                .map_err(CryptError::ChaCha)
        },
        KeyData::Three {
            enctypted_aes_key, iv, ciphertext, ..
        } => {
            let xor_key = b"\xCC\xF8\xA1\xCE\xC5\x66\x05\xB8\x51\x75\x52\xBA\x1A\x2D\x06\x1C\x03\xA2\x9E\x90\x27\x4F\xB2\xFC\xF5\x9B\xA4\xB7\x5C\x39\x23\x90";
            let mut plain_aes_key = decrypt_with_cng(enctypted_aes_key)?;
            plain_aes_key
                .iter_mut()
                .zip(xor_key)
                .for_each(|(a, b)| *a ^= b);
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&plain_aes_key));
            cipher
                .decrypt(GenericArray::from_slice(iv), *ciphertext)
                .map_err(CryptError::AesGcm)
        },
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
        return Err(CryptError::CryptUnprotectDataNull);
    }

    let decrypted_data =
        unsafe { slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    let pbdata_hlocal = Foundation::HLOCAL(output.pbData.cast::<c_void>());
    unsafe {
        _ = Foundation::LocalFree(Some(pbdata_hlocal));
    };
    Ok(decrypted_data)
}
