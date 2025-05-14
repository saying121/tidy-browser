use std::{fmt::Display, path::PathBuf};

use tokio::{fs, join};

use super::{ChromiumBuilder, ChromiumGetter};
use crate::{
    browser::ChromiumPath,
    chromium::items::{cookie::cookie_dao::CookiesQuery, passwd::login_data_dao::LoginDataQuery},
    BuilderError,
};

pub type Result<T> = std::result::Result<T, BuilderError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TempPaths {
    pub(crate) cookies_temp: PathBuf,
    pub(crate) login_data_temp: PathBuf,
    pub(crate) key_temp: PathBuf,
}

impl<B: ChromiumPath> Display for ChromiumGetter<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(B::NAME)
    }
}

impl<B: ChromiumPath> Display for ChromiumBuilder<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}Builder", B::NAME))
    }
}

impl<B: ChromiumPath> ChromiumBuilder<B> {
    pub fn new() -> Self {
        let mut base = dirs::home_dir().expect("Get home dir failed");

        base.push(B::BASE);

        Self {
            base,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// When browser start with `--user-data-dir=DIR` or special other channel
    pub const fn with_user_data_dir(base: PathBuf) -> Self {
        Self {
            base,
            __browser: core::marker::PhantomData::<B>,
        }
    }
}

impl<B: ChromiumPath + Send + Sync> ChromiumBuilder<B> {
    pub async fn build(self) -> Result<ChromiumGetter<B>> {
        let temp_paths = self.cache_data().await?;

        #[cfg(target_os = "linux")]
        let crypto = crate::chromium::crypto::linux::Decrypter::build(B::SAFE_STORAGE).await?;

        #[cfg(target_os = "macos")]
        let crypto =
            crate::chromium::crypto::macos::Decrypter::build(B::SAFE_STORAGE, B::SAFE_NAME)?;

        #[cfg(target_os = "windows")]
        let crypto = { crate::chromium::crypto::win::Decrypter::build(temp_paths.key_temp).await? };

        let (cookies_query, login_data_query) = (
            CookiesQuery::new(temp_paths.cookies_temp),
            LoginDataQuery::new(temp_paths.login_data_temp),
        );
        let (cookies_query, login_data_query) = join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (cookies_query?, login_data_query?);

        Ok(ChromiumGetter {
            cookies_query,
            login_data_query,
            crypto,
            __browser: self.__browser,
        })
    }

    async fn cache_data(&self) -> Result<TempPaths> {
        let cookies = B::cookies(self.base.clone());
        let cookies_temp = B::cookies_temp();

        let login_data = B::login_data(self.base.clone());
        let login_data_temp = B::login_data_temp();

        let key = B::key(self.base.clone());
        let key_temp = B::key_temp();

        let ck_temp_p = cookies_temp
            .parent()
            .expect("Get parent dir failed");
        let cd_ck = fs::create_dir_all(ck_temp_p);
        let lg_temp_p = login_data_temp
            .parent()
            .expect("Get parent dir failed");
        let cd_lg = fs::create_dir_all(lg_temp_p);
        let k_temp_p = key_temp
            .parent()
            .expect("Get parent dir failed");
        let cd_k = fs::create_dir_all(k_temp_p);
        let (cd_ck, cd_lg, cd_k) = join!(cd_ck, cd_lg, cd_k);
        cd_ck.map_err(|e| BuilderError::Io {
            source: e,
            path: ck_temp_p.to_owned(),
        })?;
        cd_lg.map_err(|e| BuilderError::Io {
            source: e,
            path: lg_temp_p.to_owned(),
        })?;
        cd_k.map_err(|e| BuilderError::Io {
            source: e,
            path: k_temp_p.to_owned(),
        })?;

        let cookies_cp = fs::copy(&cookies, &cookies_temp);
        let login_cp = fs::copy(&login_data, &login_data_temp);
        let key_cp = fs::copy(&key, &key_temp);

        let (ck, lg, k) = join!(cookies_cp, login_cp, key_cp);
        ck.map_err(|e| BuilderError::Io { source: e, path: cookies })?;
        lg.map_err(|e| BuilderError::Io { source: e, path: login_data })?;
        k.map_err(|e| BuilderError::Io { source: e, path: key })?;

        Ok(TempPaths {
            cookies_temp,
            login_data_temp,
            key_temp,
        })
    }
}
