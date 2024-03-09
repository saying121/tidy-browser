use aes::cipher::{block_padding, BlockDecryptMut, BlockSizeUser, KeyIvInit};
use miette::{IntoDiagnostic, Result};
use pbkdf2::pbkdf2_hmac;
use tokio::process::Command;

use crate::{
    chromium::utils::{S_CHROME, S_EDGE},
    Browser,
};

const CHROME_STORAGE_NAME: &str = "Chrome Safe Storage";
const EDGE_STORAGE_NAME: &str = "Microsoft Edge Safe Storage";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Decrypter {
    browser: Browser,
    pass:    String,
}

impl Decrypter {
    pub async fn new(browser: Browser) -> Result<Self> {
        let pass = Self::get_pass(browser).await?;
        Ok(Self { browser, pass })
    }
    pub async fn get_pass(browser: Browser) -> Result<String> {
        let storage = match browser {
            Browser::Edge => S_EDGE,
            _ => S_CHROME,
        };
        let entry = keyring::Entry::new(storage[0], storage[1]).into_diagnostic()?;
        entry.get_password().into_diagnostic()?
    }

    pub fn decrypt<'res>(&self, be_decrypte: &'res mut Vec<u8>) -> Result<&'res Vec<u8>> {
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

        let mut key = [0_u8; 16];
        let iv = [32_u8; 16];

        pbkdf2_hmac::<sha1::Sha1>(self.pass.as_bytes(), b"saltysalt", 1003, &mut key);

        let block_size = Aes128CbcDec::block_size();
        if be_decrypte.len() < block_size {
            miette::bail!("can't decrypt");
        }
        let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

        decrypter
            .decrypt_padded_mut::<block_padding::NoPadding>(&mut be_decrypte[3..])
            .expect("decrypt failed");

        be_decrypte.retain(|v| v >= &32);

        Ok(be_decrypte)
    }
}
