use std::{fmt::Display, marker::PhantomData, path::PathBuf};

use chromium_crypto::Decrypter;
use tokio::{fs, join};

use super::ChromiumGetter;
use crate::{
    browser::ChromiumPath,
    chromium::items::{cookie::cookie_dao::CookiesQuery, passwd::login_data_dao::LoginDataQuery},
};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ChromiumBuilderError {
    #[error(transparent)]
    Decrypter(#[from] chromium_crypto::error::CryptoError),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("Io: {source}, path: {path}")]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error(transparent)]
    Rawcopy(#[from] anyhow::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
    #[error("Can not found home dir")]
    HOME,
}

pub type Result<T> = std::result::Result<T, ChromiumBuilderError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TempPaths {
    pub(crate) cookies_temp: PathBuf,
    pub(crate) login_data_temp: PathBuf,
    pub(crate) login_data_for_account_temp: Option<PathBuf>,
    pub(crate) key_temp: PathBuf,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct ChromiumBuilder<T> {
    pub(crate) base: Option<PathBuf>,
    pub(crate) __browser: PhantomData<T>,
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
    pub const fn new() -> Self {
        Self {
            base: None,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// When browser start with `--user-data-dir=DIR` or special other channel
    pub const fn with_user_data_dir(base: PathBuf) -> Self {
        Self {
            base: Some(base),
            __browser: core::marker::PhantomData::<B>,
        }
    }
}

impl<B: ChromiumPath + Send + Sync> ChromiumBuilder<B> {
    pub async fn build(self) -> Result<ChromiumGetter<B>> {
        let base = if let Some(base) = self.base {
            base
        }
        else {
            let Some(mut base) = dirs::home_dir()
            else {
                return Err(ChromiumBuilderError::HOME);
            };

            base.push(B::BASE);
            base
        };

        let temp_paths = Self::cache_data(base).await?;

        #[cfg(target_os = "linux")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, crate::browser::need_safe_storage).await?;

        #[cfg(target_os = "macos")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, B::SAFE_NAME).await?;

        #[cfg(target_os = "windows")]
        let crypto = Decrypter::build(temp_paths.key_temp).await?;

        let (cookies_query, login_data_query) = (
            CookiesQuery::new(temp_paths.cookies_temp),
            LoginDataQuery::new(temp_paths.login_data_temp),
        );
        let (cookies_query, login_data_query) = join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (cookies_query?, login_data_query?);
        let login_data_for_account_query =
            if let Some(path) = temp_paths.login_data_for_account_temp {
                LoginDataQuery::new(path)
                    .await?
                    .into()
            }
            else {
                None
            };

        Ok(ChromiumGetter {
            cookies_query,
            login_data_query,
            login_data_for_account_query,
            crypto,
            __browser: self.__browser,
        })
    }

    async fn cache_data(base: PathBuf) -> Result<TempPaths> {
        let cookies = B::cookies(base.clone());
        let cookies_temp = B::cookies_temp();

        let login_data = B::login_data(base.clone());
        let login_data_temp = B::login_data_temp();

        let login_data_for_account = B::login_data_for_account(base.clone());
        let login_data_for_account_temp = B::login_data_for_account_temp();

        let key = B::key(base.clone());
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
        cd_ck.map_err(|e| ChromiumBuilderError::Io {
            source: e,
            path: ck_temp_p.to_owned(),
        })?;
        cd_lg.map_err(|e| ChromiumBuilderError::Io {
            source: e,
            path: lg_temp_p.to_owned(),
        })?;
        cd_k.map_err(|e| ChromiumBuilderError::Io {
            source: e,
            path: k_temp_p.to_owned(),
        })?;

        #[cfg(target_os = "windows")]
        let (cookies_cp, login_cp, lfac_cp, key_cp) = {
            let cookies = cookies.clone();
            let cookies_temp = cookies_temp.clone();
            let cc = tokio::task::spawn_blocking(move || {
                crate::utils::shadow_copy(&cookies, &cookies_temp)
            });

            let login = login_data.clone();
            let login_temp = login_data_temp.clone();
            let lc =
                tokio::task::spawn_blocking(move || crate::utils::shadow_copy(&login, &login_temp));

            let login_for_account = login_data_for_account.clone();
            let login_for_account_temp = login_data_for_account_temp.clone();
            let lfac = tokio::task::spawn_blocking(move || {
                crate::utils::shadow_copy(&login_for_account, &login_for_account_temp)
            });

            let key = key.clone();
            let key_temp = key_temp.clone();
            let kc =
                tokio::task::spawn_blocking(move || crate::utils::shadow_copy(&key, &key_temp));

            (cc, lc, lfac, kc)
        };

        #[cfg(not(target_os = "windows"))]
        let (cookies_cp, login_cp, lfac_cp, key_cp) = {
            (
                fs::copy(&cookies, &cookies_temp),
                fs::copy(&login_data, &login_data_temp),
                fs::copy(&login_data_for_account, &login_data_for_account_temp),
                fs::copy(&key, &key_temp),
            )
        };

        let (ck, lg, lfac, k) = join!(cookies_cp, login_cp, lfac_cp, key_cp);
        #[cfg(target_os = "windows")]
        {
            ck??;
            lg??;
            k??;
        };
        #[cfg(not(target_os = "windows"))]
        {
            ck.map_err(|e| ChromiumBuilderError::Io { source: e, path: cookies })?;
            lg.map_err(|e| ChromiumBuilderError::Io { source: e, path: login_data })?;
            k.map_err(|e| ChromiumBuilderError::Io { source: e, path: key })?;
        };
        Ok(TempPaths {
            cookies_temp,
            login_data_temp,
            login_data_for_account_temp: lfac
                .map(|_| login_data_for_account_temp)
                .ok(),
            key_temp,
        })
    }
}
