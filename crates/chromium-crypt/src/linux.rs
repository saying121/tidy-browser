use std::sync::LazyLock;

use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use secret_service::{EncryptionType, SecretService};
use tinyufo::TinyUfo;

use crate::error::{CryptError, Result};

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
    pub async fn build(safe_storage: &str) -> Result<Self> {
        let pass_v11 = Self::get_pass(safe_storage)
            .await
            .unwrap_or(Self::PASSWORD_V10);
        Ok(Self { pass_v11 })
    }

    async fn get_pass(safe_storage: &str) -> Result<&'static [u8]> {
        if let Some(v) = CACHE_PASSWD.get(&safe_storage) {
            return Ok(v);
        }

        // initialize secret service (dbus connection and encryption session)
        let ss = SecretService::connect(EncryptionType::Dh).await?;
        // get default collection
        let collection = ss.get_default_collection().await?;

        if collection.is_locked().await? {
            collection.unlock().await?;
        }
        let coll = collection.get_all_items().await?;

        for item in coll {
            let Ok(label) = item.get_label().await
            else {
                continue;
            };
            if label == safe_storage {
                let Ok(s) = item.get_secret().await
                else {
                    continue;
                };
                let s = s.leak();
                CACHE_PASSWD.put(&label, s, 1);
                return Ok(s);
            }
        }

        Ok(Self::PASSWORD_V10)
    }
    // pub fn decrypt_yandex_password(&self, ciphertext: &mut [u8]) -> Result<String> {
    //     use aes_gcm::{
    //         aead::{generic_array::GenericArray, Aead},
    //         Aes256Gcm, KeyInit,
    //     };
    //
    //     let cipher = Aes256Gcm::new(GenericArray::from_slice(Self::PASSWORD_V10));
    //
    //     miette::bail!("decrypt failed")
    // }

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=72
    pub fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String> {
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
            .map(|res| {
                String::from_utf8(res.to_vec()).unwrap_or_else(|_| {
                    tracing::info!("Decoding for chromium 130.x");
                    String::from_utf8_lossy(&res[32..]).to_string()
                })
            })
            .map_err(CryptError::Unpadding)
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

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;
//
//     use base64::{engine::general_purpose, Engine};
//     use tokio::fs::read_to_string;
//
//     use super::*;
//     use crate::chromium::local_state::YandexLocalState;
//     async fn yandex_passwd(path: PathBuf) -> Result<Vec<u8>> {
//         let string_str = read_to_string(path)
//             .await
//             ?;
//         let local_state: YandexLocalState = serde_json::from_str(&string_str)?;
//         let encrypted_key = general_purpose::STANDARD
//             .decode(
//                 local_state
//                     .os_crypt
//                     .checker_state
//                     .encrypted_data,
//             )
//             ?;
//         Ok(encrypted_key)
//     }
//     #[ignore = "need realy environment"]
//     #[tokio::test]
//     async fn yandex_passwd_work() {
//         use crate::{browser::info::ChromiumInfo, ChromiumBuilder};
//         let yandex_getter = ChromiumBuilder::new(Browser::Yandex)
//             .build()
//             .await
//             .unwrap();
//         let mut pss = yandex_passwd(yandex_getter.info().local_state())
//             .await
//             .unwrap();
//         let pass = yandex_getter
//             .decrypt(&mut pss)
//             .unwrap();
//
//         assert_eq!(&pass, "0");
//     }
// }
