use aes::cipher::{block_padding, BlockDecryptMut, BlockSizeUser, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;
use tokio::process::Command;

use crate::{chromium::utils::*, Browser};

use self::safe_storage::SafeBrowser;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=33
/// Salt for Symmetric key derivation.
const K_SALT: &[u8] = b"saltysalt";

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=35
/// Key size required for 128 bit AES.
const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=39
/// Constant for Symmetic key derivation.
const K_ENCRYPTION_ITERATIONS: u32 = 1003;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=44
/// Prefix for cypher text returned by current encryption version.  We prefix
/// the cypher text with this string so that future data migration can detect
/// this and migrate to different encryption without data loss.
const K_ENCRYPTION_VERSION_PREFIX: &[u8] = b"v10";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    browser: Browser,
    pass: String,
}

impl Decrypter {
    pub async fn new(browser: Browser) -> Result<Self> {
        let pass = Self::get_pass(browser).await?;
        Ok(Self { browser, pass })
    }
    pub async fn get_pass(browser: Browser) -> Result<String> {
        let entry =
            keyring::Entry::new(browser.safe_name(), browser.storage()).into_diagnostic()?;
        entry
            .get_password()
            .into_diagnostic()?
    }

    pub fn decrypt(&self, be_decrypte: &[u8]) -> Result<String> {
        if !be_decrypte.starts_with(K_ENCRYPTION_VERSION_PREFIX) {
            return String::from_utf8(be_decrypte.to_vec());
        }

        let mut key = [0_u8; 16];
        let iv = [b' '; 16];

        pbkdf2_hmac::<sha1::Sha1>(
            self.pass.as_bytes(),
            K_SALT,
            K_ENCRYPTION_ITERATIONS,
            &mut key,
        );

        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        if let Ok(res) = decrypter.decrypt_padded_mut::<block_padding::Pkcs7>(&mut be_decrypte[3..])
        {
            return String::from_utf8(res.to_vec());
        }

        miette::bail!("decrypt error")
    }
}
