use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;

use crate::{chromium::utils::crypto::BrowserDecrypt, Browser};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=33
/// Salt for Symmetric key derivation.
const K_SALT: &[u8] = b"saltysalt";

// https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_mac.mm;l=35
/// Key size required for 128 bit AES.
// const K_DERIVED_KEY_SIZE_IN_BITS: u32 = 128;

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
    browser:  Browser,
    pass_v10: Vec<u8>,
}

impl BrowserDecrypt for Decrypter {
    async fn build(browser: Browser) -> Result<Self> {
        let pass_v10 = Self::get_pass(browser).await?;
        Ok(Self { browser, pass_v10 })
    }
    async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
        let entry =
            keyring::Entry::new(browser.storage(), browser.safe_name()).into_diagnostic()?;
        entry
            .get_password()
            .map(|v| v.into_bytes())
            .into_diagnostic()
    }

    fn decrypt(&self, be_decrypte: &mut [u8]) -> Result<String> {
        if !be_decrypte.starts_with(K_ENCRYPTION_VERSION_PREFIX) {
            return String::from_utf8(be_decrypte.to_vec()).into_diagnostic();
        }
        let prefix_len = K_ENCRYPTION_VERSION_PREFIX.len();

        let mut key = [0_u8; 16];
        let iv = [b' '; 16];

        pbkdf2_hmac::<sha1::Sha1>(
            &self.pass_v10,
            K_SALT,
            K_ENCRYPTION_ITERATIONS,
            &mut key,
        );

        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        if let Ok(res) =
            decrypter.decrypt_padded_mut::<block_padding::Pkcs7>(&mut be_decrypte[prefix_len..])
        {
            return String::from_utf8(res.to_vec()).into_diagnostic();
        }

        miette::bail!("decrypt error")
    }
}
