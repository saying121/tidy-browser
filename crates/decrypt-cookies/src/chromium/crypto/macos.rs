use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum CryptoError {
    #[error("Get keyring failed")]
    Keyring(#[from] keyring::Error),
    #[error("Unpad error: {0}")]
    Unpadding(block_padding::UnpadError),
}
type Result<T> = std::result::Result<T, CryptoError>;

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
    pub fn build(safe_storage: &str, safe_name: &str) -> Result<Self> {
        let pass_v10 = Self::get_pass(safe_storage, safe_name)?;
        Ok(Self { pass_v10 })
    }
    fn get_pass(safe_storage: &str, safe_name: &str) -> Result<Vec<u8>> {
        let entry = match keyring::Entry::new(safe_storage, safe_name) {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };
        match entry
            .get_password()
            .map(String::into_bytes)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }

    pub fn decrypt(&self, be_decrypte: &mut [u8]) -> Result<String> {
        if !be_decrypte.starts_with(Self::K_ENCRYPTION_VERSION_PREFIX) {
            return Ok(String::from_utf8_lossy(be_decrypte).to_string());
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

        match decrypter.decrypt_padded_mut::<block_padding::Pkcs7>(&mut be_decrypte[prefix_len..]) {
            Ok(res) => Ok(String::from_utf8_lossy(res).to_string()),
            Err(e) => Err(CryptoError::Unpadding(e)),
        }
    }
}
