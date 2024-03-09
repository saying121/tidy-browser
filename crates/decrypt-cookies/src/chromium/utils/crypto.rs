use miette::IntoDiagnostic;

use crate::Browser;

pub async fn decrypt_cookies(encrypted: &mut Vec<u8>, browser: Browser) -> miette::Result<String> {
    #[cfg(target_os = "linux")]
    let res = {
        let decrypter = super::linux::crypto::Decrypter::new(browser).await?;
        decrypter
            .decrypt(encrypted)?
            .clone()
    };
    #[cfg(target_os = "macos")]
    let res = {
        let decrypter = super::macos::crypto::Decrypter::new(browser).await?;
        decrypter
            .decrypt(encrypted)?
            .clone()
    };
    #[cfg(target_os = "windows")]
    let res = {
        let decrypter = super::win::crypto::Decrypter::new(browser).await?;
        decrypter.decrypt(encrypted)?
    };

    String::from_utf8(res).into_diagnostic()
}
