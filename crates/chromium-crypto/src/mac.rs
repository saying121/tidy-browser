use std::convert::Into;

use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;

use crate::error::{CryptoError, Result};

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=35
/// Key size required for 128 bit AES.
// const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass_v10: Vec<u8>,
}

impl Decrypter {
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=33
    /// Salt for Symmetric key derivation.
    const K_SALT: &'static [u8] = b"saltysalt";

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=39
    /// Constant for Symmetic key derivation.
    const K_ENCRYPTION_ITERATIONS: u32 = 1003;

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=44
    /// Prefix for cypher text returned by current encryption version.  We prefix
    /// the cypher text with this string so that future data migration can detect
    /// this and migrate to different encryption without data loss.
    const K_ENCRYPTION_VERSION_PREFIX: &'static [u8] = b"v10";
}

impl Decrypter {
    pub async fn build(safe_storage: &str, safe_name: &str) -> Result<Self> {
        let pass_v10 = Self::get_pass(safe_storage, safe_name).await?;
        Ok(Self { pass_v10 })
    }

    async fn get_pass(safe_storage: &str, safe_name: &str) -> Result<Vec<u8>> {
        // # Safety
        //
        // Already `.await` in the function.
        // See: `std::thread::Builder::spawn_unchecked`.
        let safe_storage: &'static str =
            unsafe { std::mem::transmute::<&str, &'static str>(safe_storage) };
        let safe_name: &'static str =
            unsafe { std::mem::transmute::<&str, &'static str>(safe_name) };

        let entry = tokio::task::spawn_blocking(|| {
            keyring::Entry::new(safe_storage, safe_name).map_err(|e| -> CryptoError { e.into() })
        })
        .await??;
        entry
            .get_password()
            .map(String::into_bytes)
            .map_err(Into::into)
    }

    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
        if !ciphertext.starts_with(Self::K_ENCRYPTION_VERSION_PREFIX) {
            return Ok(String::from_utf8_lossy(ciphertext).to_string());
        }
        let prefix_len = Self::K_ENCRYPTION_VERSION_PREFIX.len();

        let mut key = [0_u8; 16];
        let iv = [b' '; 16];

        pbkdf2_hmac::<sha1::Sha1>(
            &self.pass_v10,
            Self::K_SALT,
            Self::K_ENCRYPTION_ITERATIONS,
            &mut key,
        );

        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        decrypter
            .decrypt_padded_mut::<block_padding::Pkcs7>(&mut ciphertext[prefix_len..])
            .map_err(CryptoError::Unpadding)
            .map(|res| {
                String::from_utf8(res.to_vec()).unwrap_or_else(|_| {
                    tracing::info!("Decoding for chromium >= 130.x");
                    String::from_utf8_lossy(&res[32..]).to_string()
                })
            })
    }
}
