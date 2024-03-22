use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;
use secret_service::{EncryptionType, SecretService};

use crate::Browser;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=29
/// Salt for Symmetric key derivation.
const K_SALT: &[u8] = b"saltysalt";
// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=35
/// Constant for Symmetric key derivation.
const K_ENCRYPTION_ITERATIONS: u32 = 1;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=38
/// Size of initialization vector for AES 128-bit.
const K_IVBLOCK_SIZE_AES128: usize = 16;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=40
/// Prefixes for cypher text returned by obfuscation version.
/// We prefix the ciphertext with this string so that future data migration can detect
/// this and migrate to full encryption without data loss.
/// `K_OBFUSCATION_PREFIX_V10` means that the hardcoded password will be used.
/// `K_OBFUSCATION_PREFIX_V11` means that a password is/will be stored using an OS-level library (e.g Libsecret).
/// V11 will not be used if such a library is not available.
const K_OBFUSCATION_PREFIX_V10: &[u8] = b"v10";
const K_OBFUSCATION_PREFIX_V11: &[u8] = b"v11";

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=313
/// v10: hardcoded/default password
const PASSWORD_V10: &[u8] = b"peanuts";

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=50
/// The UMA metric name for whether the false was decryptable with an empty key.
// const K_METRIC_DECRYPTED_WITH_EMPTY_KEY: &[u8] = b"OSCrypt.Linux.DecryptedWithEmptyKey";

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=32
/// Key size required for 128 bit AES.
// const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    pass_v11: Vec<u8>,
    browser: Browser,
}

// OPTIMIZE: lazy initialize pass_v11?
impl Decrypter {
    pub async fn new(browser: Browser) -> Result<Self> {
        let pass_v11 = Self::get_pass(browser)
            .await
            .unwrap_or_else(|_| PASSWORD_V10.to_vec());
        Ok(Self { pass_v11, browser })
    }

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_linux.cc;l=72
    pub fn decrypt(&self, be_decrypte: &[u8]) -> Result<String> {
        let (pass, prefix_len) = if be_decrypte.starts_with(K_OBFUSCATION_PREFIX_V11) {
            (self.pass_v11.as_slice(), K_OBFUSCATION_PREFIX_V11.len())
        } else if be_decrypte.starts_with(K_OBFUSCATION_PREFIX_V10) {
            (PASSWORD_V10, K_OBFUSCATION_PREFIX_V10.len())
        } else {
            return String::from_utf8(be_decrypte.to_vec()).into_diagnostic();
        };

        let mut key = [0_u8; 16];
        let iv = [b' '; K_IVBLOCK_SIZE_AES128];

        pbkdf2_hmac::<sha1::Sha1>(pass, K_SALT, K_ENCRYPTION_ITERATIONS, &mut key);
        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        let mut be_decrypte = be_decrypte[prefix_len..].to_vec();

        if let Ok(res) = decrypter.decrypt_padded_mut::<block_padding::Pkcs7>(&mut be_decrypte) {
            return String::from_utf8(res.to_vec()).into_diagnostic();
        }

        miette::bail!("decrypt failed")
    }

    /// from `secret_service` get password
    pub async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
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

        let mut res = vec![];
        for item in coll {
            let Ok(l) = item.get_label().await else {
                continue;
            };
            if l.as_str() == browser.storage() {
                res = item
                    .get_secret()
                    .await
                    .into_diagnostic()?;
            }
        }
        tracing::debug!("res: {}", String::from_utf8_lossy(&res).to_string());

        Ok(res)
    }
}
