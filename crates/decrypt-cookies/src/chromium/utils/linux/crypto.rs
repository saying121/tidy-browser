use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;
use secret_service::{EncryptionType, SecretService};
use tracing::debug;

use crate::Browser;

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Microsoft Edge Safe Storage";
// const CHROMIUM_STORAGE_NAME: &str = "Chromium Safe Storage";

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

    pub fn decrypt<'res>(&self, be_decrypte: &'res mut Vec<u8>) -> Result<&'res Vec<u8>> {
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

        let mut key = [0_u8; 16];
        let iv = [32_u8; 16];

        pbkdf2_hmac::<sha1::Sha1>(&self.pass, b"saltysalt", 1, &mut key);

        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        if decrypter
            .decrypt_padded_mut::<block_padding::NoPadding>(&mut *be_decrypte)
            .is_err()
        {
            miette::bail!("decrypt failed")
        }

        be_decrypte.retain(|v| v >= &32);

        Ok(be_decrypte)
    }

    /// from `secret_service` get password
    pub async fn get_pass(browser: Browser) -> Result<Vec<u8>> {
        let default_pass = Ok(b"peanuts".to_vec());
        // initialize secret service (dbus connection and encryption session)
        let Ok(ss) = SecretService::connect(EncryptionType::Dh).await
        else {
            return default_pass;
        };
        // get default collection
        let Ok(collection) = ss.get_default_collection().await
        else {
            return default_pass;
        };
        let coll = collection
            .get_all_items()
            .await
            .into_diagnostic()?;
        let label = match browser {
            Browser::Edge => EDGE_STORAGE_NAME,
            _ => CHROME_STORAGE_NAME,
        };
        let mut res = vec![];
        for i in coll {
            let Ok(l) = i.get_label().await
            else {
                continue;
            };
            if l == label {
                res = i
                    .get_secret()
                    .await
                    .into_diagnostic()?;
            }
        }
        debug!("res: {}", String::from_utf8_lossy(&res).to_string());
        if res.is_empty() {
            return default_pass;
        }

        Ok(res)
    }
}
