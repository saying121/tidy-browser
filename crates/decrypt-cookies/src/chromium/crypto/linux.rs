use std::collections::HashMap;

use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;
use secret_service::{EncryptionType, SecretService};
use tokio::sync::OnceCell;

use crate::browser::info::need_safe_storage;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=32
/// Key size required for 128 bit AES.
// const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[expect(clippy::empty_line_after_doc_comments)]
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

static ALL_PASSWD: OnceCell<HashMap<&'static str, &'static [u8]>> = OnceCell::const_new();

async fn get_pass_once() -> &'static HashMap<&'static str, &'static [u8]> {
    ALL_PASSWD
        .get_or_init(|| async {
            get_all_pass()
                .await
                .unwrap_or_default()
        })
        .await
}

// lab: Brave Safe Storage
/// from `secret_service` get all password
async fn get_all_pass() -> Result<HashMap<&'static str, &'static [u8]>> {
    // initialize secret service (dbus connection and encryption session)
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .into_diagnostic()?;
    // get default collection
    let collection = ss
        .get_default_collection()
        .await
        .into_diagnostic()?;

    if collection
        .is_locked()
        .await
        .into_diagnostic()?
    {
        collection
            .unlock()
            .await
            .into_diagnostic()?;
    }
    let coll = collection
        .get_all_items()
        .await
        .into_diagnostic()?;

    let mut res = HashMap::new();
    for item in coll {
        let Ok(label) = item.get_label().await
        else {
            continue;
        };

        if !need_safe_storage(&label) {
            continue;
        }

        let Ok(s) = item.get_secret().await
        else {
            continue;
        };
        let l: &'static str = label.leak();
        let s: &'static [u8] = s.leak();
        res.insert(l, s);
    }

    Ok(res)
}

impl Decrypter {
    pub async fn build(safe_storage: &str) -> Result<Self> {
        let pass_v11 = get_pass_once()
            .await
            .get(safe_storage)
            .map_or_else(|| Self::PASSWORD_V10, |v| *v);
        Ok(Self { pass_v11 })
    }
    // pub fn decrypt_yandex_password(&self, be_decrypte: &mut [u8]) -> Result<String> {
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
    pub fn decrypt(&self, be_decrypte: &mut [u8]) -> Result<String> {
        let (pass, prefix_len) = if be_decrypte.starts_with(Self::K_OBFUSCATION_PREFIX_V11) {
            (self.pass_v11, Self::K_OBFUSCATION_PREFIX_V11.len())
        }
        else if be_decrypte.starts_with(Self::K_OBFUSCATION_PREFIX_V10) {
            (Self::PASSWORD_V10, Self::K_OBFUSCATION_PREFIX_V10.len())
        }
        else {
            return Ok(String::from_utf8_lossy(be_decrypte).to_string());
        };

        let mut key = [0_u8; 16];
        let iv = [b' '; Self::K_IVBLOCK_SIZE_AES128];

        pbkdf2_hmac::<sha1::Sha1>(pass, Self::K_SALT, Self::K_ENCRYPTION_ITERATIONS, &mut key);
        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        if let Ok(res) =
            decrypter.decrypt_padded_mut::<block_padding::Pkcs7>(&mut be_decrypte[prefix_len..])
        {
            return Ok(String::from_utf8_lossy(res).to_string());
        }

        miette::bail!("decrypt failed")
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
//             .into_diagnostic()?;
//         let local_state: YandexLocalState = serde_json::from_str(&string_str).into_diagnostic()?;
//         let encrypted_key = general_purpose::STANDARD
//             .decode(
//                 local_state
//                     .os_crypt
//                     .checker_state
//                     .encrypted_data,
//             )
//             .into_diagnostic()?;
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
