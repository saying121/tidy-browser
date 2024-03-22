use self::dao::Key4Query;
use crate::Browser;

pub mod dao;
pub mod key4db;

pub async fn get_firefox_decrypt_key(browser: Browser) -> miette::Result<()> {
    let query = Key4Query::new(browser).await?;
    let nss_private = query.query_nssprivate().await?;
    let meta_data = query.query_metadata().await?;

    let (globalSalt, metaBytes, nssA11, nssA102) = (
        meta_data.item1,
        meta_data.item2,
        nss_private.a11,
        nss_private.a102,
    );
    Ok(())
}
