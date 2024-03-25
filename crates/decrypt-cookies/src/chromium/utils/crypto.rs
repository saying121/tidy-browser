use miette::Result;

use crate::Browser;

pub trait BrowserDecrypt
where
    Self: std::marker::Sized,
{
    fn build(browser: Browser) -> impl std::future::Future<Output = Result<Self>> + Send;

    fn decrypt(&self, ciphertext: &mut [u8]) -> Result<String>;

    fn get_pass(browser: Browser) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
}
