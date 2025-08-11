use std::sync::LazyLock;

use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use secret_service::{EncryptionType, SecretService};
use snafu::ResultExt;
use tinyufo::TinyUfo;

use crate::{
    error::{self, Result, Utf8Snafu},
    Which,
};

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=32
/// Key size required for 128 bit AES.
// const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[expect(clippy::empty_line_after_doc_comments, reason = "it not use")]
// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=50
/// The UMA metric name for whether the false was decryptable with an empty key.
// const K_METRIC_DECRYPTED_WITH_EMPTY_KEY: &[u8] = b"OSCrypt.Linux.DecryptedWithEmptyKey";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass_v11: &'static [u8],
}

impl Decrypter {
    pub const fn pass_v11(&self) -> &[u8] {
        self.pass_v11
    }
}

static CACHE_PASSWD: LazyLock<TinyUfo<&str, &'static [u8]>> =
    LazyLock::new(|| TinyUfo::new_compact(10, 10));

impl Decrypter {
    /// `safe_storage` example: Brave Safe Storage
    pub async fn build<F, N>(safe_storage: &str, need: N) -> Result<Self>
    where
        N: Into<Option<F>> + Send,
        F: Fn(&str) -> bool + Send,
    {
        let pass_v11 = Self::get_pass(safe_storage, need)
            .await
            .unwrap_or(Self::PASSWORD_V10);
        Ok(Self { pass_v11 })
    }

    async fn get_pass<F, N>(safe_storage: &str, need: N) -> Result<&'static [u8]>
    where
        N: Into<Option<F>> + Send,
        F: Fn(&str) -> bool + Send,
    {
        if let Some(v) = CACHE_PASSWD.get(&safe_storage) {
            return Ok(v);
        }

        // initialize secret service (dbus connection and encryption session)
        let ss = SecretService::connect(EncryptionType::Dh)
            .await
            .context(error::GetPassSnafu)?;
        let collection = ss
            .get_default_collection()
            .await
            .context(error::GetPassSnafu)?;

        if collection
            .is_locked()
            .await
            .context(error::GetPassSnafu)?
        {
            collection
                .unlock()
                .await
                .context(error::GetPassSnafu)?;
        }
        let coll = collection
            .get_all_items()
            .await
            .context(error::GetPassSnafu)?;

        let predicate: Option<F> = need.into();

        for item in coll {
            let Ok(label) = item.get_label().await
            else {
                continue;
            };
            // TODO: use 1.88 let_chains
            if let Some(cache_it) = &predicate {
                if cache_it(&label) || label == safe_storage {
                    let Ok(s) = item.get_secret().await
                    else {
                        continue;
                    };

                    let s = s.leak();
                    CACHE_PASSWD.put(&label, s, 1);
                }
            }
            else if label == safe_storage {
                let Ok(s) = item.get_secret().await
                else {
                    continue;
                };
                let s = s.leak();
                CACHE_PASSWD.put(&label, s, 1);
            }
        }

        if let Some(v) = CACHE_PASSWD.get(&safe_storage) {
            return Ok(v);
        }

        Ok(Self::PASSWORD_V10)
    }

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=72
    pub fn decrypt(&self, ciphertext: &mut [u8], which: Which) -> Result<String> {
        let (pass, prefix_len) = if ciphertext.starts_with(Self::K_OBFUSCATION_PREFIX_V11) {
            (self.pass_v11, Self::K_OBFUSCATION_PREFIX_V11.len())
        }
        else if ciphertext.starts_with(Self::K_OBFUSCATION_PREFIX_V10) {
            (Self::PASSWORD_V10, Self::K_OBFUSCATION_PREFIX_V10.len())
        }
        else {
            return Ok(String::from_utf8_lossy(ciphertext).to_string());
        };

        let mut key = [0_u8; 16];
        let iv = [b' '; Self::K_IVBLOCK_SIZE_AES128];

        pbkdf2_hmac::<sha1::Sha1>(pass, Self::K_SALT, Self::K_ENCRYPTION_ITERATIONS, &mut key);
        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        decrypter
            .decrypt_padded_mut::<block_padding::Pkcs7>(&mut ciphertext[prefix_len..])
            .context(error::UnpaddingSnafu)
            .map(|res| match which {
                Which::Cookie => {
                    if res.len() > 32 {
                        String::from_utf8(res[32..].to_vec())
                            .or_else(|_| crate::from_utf8_cold(res))
                    }
                    else {
                        crate::from_utf8_cold(res)
                    }
                },
                Which::Login => String::from_utf8(res.to_vec()),
            })?
            .context(Utf8Snafu)
    }
}

impl Decrypter {
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=313
    /// v10: hardcoded/default password
    const PASSWORD_V10: &'static [u8] = b"peanuts";

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=40
    /// Prefixes for cypher text returned by obfuscation version.
    /// We prefix the ciphertext with this string so that future data migration can detect
    /// this and migrate to full encryption without data loss.
    /// `K_OBFUSCATION_PREFIX_V10` means that the hardcoded password will be used.
    /// `K_OBFUSCATION_PREFIX_V11` means that a password is/will be stored using an OS-level library (e.g Libsecret).
    /// V11 will not be used if such a library is not available.
    const K_OBFUSCATION_PREFIX_V10: &'static [u8] = b"v10";
    const K_OBFUSCATION_PREFIX_V11: &'static [u8] = b"v11";

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=38
    /// Size of initialization vector for AES 128-bit.
    const K_IVBLOCK_SIZE_AES128: usize = 16;

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=29
    /// Salt for Symmetric key derivation.
    const K_SALT: &'static [u8] = b"saltysalt";
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=35
    /// Constant for Symmetric key derivation.
    const K_ENCRYPTION_ITERATIONS: u32 = 1;
}
