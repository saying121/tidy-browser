#![expect(clippy::exhaustive_structs, reason = "Allow error")]

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use chromium_crypto::Decrypter;
use snafu::{ensure, Location, OptionExt, ResultExt, Snafu};
use tokio::{fs, join};

use super::{ChromiumCookieGetter, ChromiumGetter, ChromiumLoginGetter};
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
    fn ensure_base(self) -> Result<PathBuf> {
        let base = if let Some(base) = self.base {
            base
        }
        else {
            let mut base = dirs::home_dir().context(HomeSnafu)?;

            base.push(B::BASE);
            base
        };

        ensure!(base.exists(), NotFoundBaseSnafu { path: base });

        Ok(base)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "Chromium build", skip(self), fields(browser), level = "debug")
    )]
    pub async fn build(self) -> Result<ChromiumGetter<B>> {
        let __browser = self.__browser;
        let base = self.ensure_base()?;

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(base = %base.display());
        };

        let crypto = Self::gen_crypto(&base);

        let (crypto, cookies_query, logins) = join!(
            crypto,
            Self::cache_cookies(base.clone()),
            Self::cache_login(base.clone())
        );

        let (login_data_query, lfa) = logins?;

        Ok(ChromiumGetter {
            cookies_query: cookies_query?,
            login_data_query,
            login_data_for_account_query: lfa,
            crypto: crypto?,
            __browser,
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            name = "Chromium Login build",
            skip(self),
            fields(browser),
            level = "debug"
        )
    )]
    pub async fn build_login(self) -> Result<ChromiumLoginGetter<B>> {
        let __browser = self.__browser;
        let base = self.ensure_base()?;

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(base = %base.display());
        };

        let (crypto, logins) = join!(Self::gen_crypto(&base), Self::cache_login(base.clone()));

        let (login_data_query, lfa) = logins?;

        Ok(ChromiumLoginGetter {
            login_data_query,
            login_data_for_account_query: lfa,
            crypto: crypto?,
            __browser,
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            name = "Chromium Cookie build",
            skip(self),
            fields(browser),
            level = "debug"
        )
    )]
    pub async fn build_cookie(self) -> Result<ChromiumCookieGetter<B>> {
        let __browser = self.__browser;
        let base = self.ensure_base()?;

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(base = %base.display());
        };

        let crypto = Self::gen_crypto(&base);

        let (crypto, cookies_query) = join!(crypto, Self::cache_cookies(base.clone()));

        Ok(ChromiumCookieGetter {
            cookies_query: cookies_query?,
            crypto: crypto?,
            __browser,
        })
    }

    #[cfg_attr(
        not(target_os = "windows"),
        expect(unused_variables, reason = "for windows")
    )]
    async fn gen_crypto(base: &Path) -> Result<Decrypter> {
        #[cfg(target_os = "linux")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, crate::browser::need_safe_storage);

        #[cfg(target_os = "macos")]
        let crypto = Decrypter::build(B::SAFE_STORAGE, B::SAFE_NAME);

        #[cfg(target_os = "windows")]
        let crypto = {
            let key_path = Self::cache_key(base.to_owned()).await?;
            Decrypter::build(key_path)
        };

        crypto
            .await
            .context(DecrypterSnafu)
    }

    /// return login and login for account
    async fn cache_login(base: PathBuf) -> Result<(LoginDataQuery, Option<LoginDataQuery>)> {
        let login_data = B::login_data(base.clone());
        let login_data_temp = B::login_data_temp().context(HomeSnafu)?;

        let login_data_for_account = B::login_data_for_account(base.clone());
        let login_data_for_account_temp = B::login_data_for_account_temp().context(HomeSnafu)?;

        let (lg, lfac) = join!(
            copy(&login_data, &login_data_temp),
            copy(&login_data_for_account, &login_data_for_account_temp)
        );
        lg?;

        Ok((
            LoginDataQuery::new(login_data_temp)
                .await
                .context(DbSnafu)?,
            if lfac.is_ok() {
                LoginDataQuery::new(login_data_for_account_temp)
                    .await
                    .ok()
            }
            else {
                None
            },
        ))
    }

    async fn cache_cookies(base: PathBuf) -> Result<CookiesQuery> {
        let cookies = B::cookies(base.clone());
        let cookies_temp = B::cookies_temp().context(HomeSnafu)?;

        copy(&cookies, &cookies_temp).await?;
        CookiesQuery::new(cookies_temp)
            .await
            .context(DbSnafu)
    }

    #[cfg(target_os = "windows")]
    async fn cache_key(base: PathBuf) -> Result<PathBuf> {
        let key = B::key(base.clone());
        let key_temp = B::key_temp().context(HomeSnafu)?;

        copy(&key, &key_temp).await?;

        Ok(key_temp)
    }
}
