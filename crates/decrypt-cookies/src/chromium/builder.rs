#![expect(clippy::exhaustive_structs, reason = "Allow error")]

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use chromium_crypto::Decrypter;
use snafu::{Location, OptionExt, ResultExt, Snafu};
use tokio::{fs, join};

use super::ChromiumGetter;
use crate::{
    browser::ChromiumPath,
    chromium::items::{cookie::cookie_dao::CookiesQuery, passwd::login_data_dao::LoginDataQuery},
};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum ChromiumBuilderError {
    #[snafu(display(r#"Not found {}
The browser is not installed or started with `--user-data-dir` arg
@:{location}"#, path.display()))]
    NotFoundBase {
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Decrypter {
        source: chromium_crypto::error::CryptoError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Db {
        source: sea_orm::DbErr,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}, path: {}\n@:{location}",path.display()))]
    Io {
        source: std::io::Error,
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[cfg(target_os = "windows")]
    #[snafu(display("{source}\n@:{location}"))]
    Rawcopy {
        source: anyhow::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    TokioJoin {
        source: tokio::task::JoinError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Can not found home dir\n@:{location}"))]
    Home {
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, ChromiumBuilderError>;

/// The `to` must have parent dir
async fn copy<A, A0>(from: A, to: A0) -> Result<()>
where
    A: AsRef<Path> + Send,
    A0: AsRef<Path> + Send,
{
    let parent = to
        .as_ref()
        .parent()
        .expect("Get parent dir failed");
    fs::create_dir_all(parent)
        .await
        .with_context(|_| IoSnafu { path: parent.to_owned() })?;

    #[cfg(not(target_os = "windows"))]
    fs::copy(from.as_ref(), to.as_ref())
        .await
        .with_context(|_| IoSnafu { path: from.as_ref().to_owned() })?;
    #[cfg(target_os = "windows")]
    {
        let from_ = from.as_ref().to_owned();
        let to = to.as_ref().to_owned();
        tokio::task::spawn_blocking(move || crate::utils::shadow_copy(&from_, &to))
            .await
            .context(TokioJoinSnafu)??;
    };

    Ok(())
}

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

impl<B: ChromiumPath> Display for ChromiumBuilder<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}Builder", B::NAME))
    }
}

impl<B: ChromiumPath> ChromiumBuilder<B> {
    pub const fn new() -> Self {
        Self {
            base: None,
            __browser: PhantomData::<B>,
        }
    }

    /// When browser start with `--user-data-dir=DIR` or special other channel
    pub const fn with_user_data_dir(base: PathBuf) -> Self {
        Self {
            base: Some(base),
            __browser: PhantomData::<B>,
        }
    }
}

impl<B: ChromiumPath + Send + Sync> ChromiumBuilder<B> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "Chromium build", skip(self), fields(browser), level = "debug")
    )]
    pub async fn build(self) -> Result<ChromiumGetter<B>> {
        let base = if let Some(base) = self.base {
            base
        }
        else {
            let mut base = dirs::home_dir().context(HomeSnafu)?;

            base.push(B::BASE);
            base
        };

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(base = %base.display());
        };

        if !base.exists() {
            return Err(NotFoundBaseSnafu { path: base }.build());
        }

        let temp_paths = Self::cache_data(base).await?;

        #[cfg(target_os = "linux")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, crate::browser::need_safe_storage)
            .await
            .context(DecrypterSnafu)?;

        #[cfg(target_os = "macos")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, B::SAFE_NAME)
            .await
            .context(DecrypterSnafu)?;

        #[cfg(target_os = "windows")]
        let crypto = Decrypter::build(temp_paths.key_temp)
            .await
            .context(DecrypterSnafu)?;

        let (cookies_query, login_data_query) = (
            CookiesQuery::new(temp_paths.cookies_temp),
            LoginDataQuery::new(temp_paths.login_data_temp),
        );
        let (cookies_query, login_data_query) = join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (
            cookies_query.context(DbSnafu)?,
            login_data_query.context(DbSnafu)?,
        );
        let login_data_for_account_query =
            if let Some(path) = temp_paths.login_data_for_account_temp {
                LoginDataQuery::new(path)
                    .await
                    .context(DbSnafu)?
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
        let cookies_temp = B::cookies_temp().context(HomeSnafu)?;

        let login_data = B::login_data(base.clone());
        let login_data_temp = B::login_data_temp().context(HomeSnafu)?;

        let login_data_for_account = B::login_data_for_account(base.clone());
        let login_data_for_account_temp = B::login_data_for_account_temp().context(HomeSnafu)?;

        let key = B::key(base.clone());
        let key_temp = B::key_temp().context(HomeSnafu)?;

        let (cookies_cp, login_cp, lfac_cp, key_cp) = {
            (
                copy(&cookies, &cookies_temp),
                copy(&login_data, &login_data_temp),
                copy(&login_data_for_account, &login_data_for_account_temp),
                copy(&key, &key_temp),
            )
        };

        let (ck, lg, lfac, k) = join!(cookies_cp, login_cp, lfac_cp, key_cp);
        ck?;
        lg?;
        k?;
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
