use std::{
    fmt::Display,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use snafu::{Location, OptionExt, ResultExt, Snafu};
use tokio::fs;

use super::FirefoxCookieGetter;
use crate::{
    firefox::{items::cookie::dao::CookiesQuery, FirefoxGetter},
    prelude::FirefoxPath,
};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum FirefoxBuilderError {
    #[snafu(display(r#"Not found {}
The browser is not installed or started with `-P`/`-profile` arg
@:{location}"#, path.display()))]
    NotFoundBase {
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Ini {
        source: ini::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    IniParser {
        source: ini::ParseError,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Profile {profile} missing `Name` properties\n@:{location}"))]
    ProfilePath {
        profile: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Install {install} missing `Default` properties\n@:{location}"))]
    InstallPath {
        install: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{source}\n@:{location}"))]
    Db {
        source: sea_orm::DbErr,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Io: {source}, path: {}\n@:{location}",path.display()))]
    Io {
        source: std::io::Error,
        path: PathBuf,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Can not found home dir\n@:{location}"))]
    Home {
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T> = std::result::Result<T, FirefoxBuilderError>;

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

    let from = from.as_ref();
    fs::copy(from, to.as_ref())
        .await
        .with_context(|_| IoSnafu { path: from.to_owned() })?;

    Ok(())
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder<'a, T> {
    pub(crate) base: Option<PathBuf>,
    pub(crate) profile: Option<&'a str>,
    pub(crate) profile_path: Option<PathBuf>,
    pub(crate) __browser: PhantomData<T>,
}

impl<B: FirefoxPath> Display for FirefoxBuilder<'_, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}Builder", B::NAME))
    }
}

impl<'b, B: FirefoxPath> FirefoxBuilder<'b, B> {
    pub const fn new() -> Self {
        Self {
            base: None,
            profile: None,
            profile_path: None,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// `profile_path`: when browser started with `-profile <profile_path>`
    /// When set `profile_path` ignore other parameters like `base`, `profile`.
    pub fn with_profile_path(profile_path: PathBuf) -> Self {
        Self {
            base: None,
            profile: None,
            profile_path: profile_path.into(),
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// `base`: When Firefox data path changed
    pub fn base(&mut self, base: PathBuf) -> &mut Self {
        self.base = base.into();
        self
    }

    /// `profile`: When started with `-P <profile>`
    pub fn profile(&mut self, profile: &'b str) -> &mut Self {
        self.profile = profile.into();
        self
    }

    // async fn cache_data(profile_path: PathBuf) -> Result<TempPaths> {
    //     let cookies = B::cookies(profile_path.clone());
    //     let cookies_temp = B::cookies_temp().context(HomeSnafu)?;
    //
    //     let login_data = B::login_data(profile_path.clone());
    //     let login_data_temp = B::login_data_temp().context(HomeSnafu)?;
    //
    //     let key = B::key(profile_path.clone());
    //     let key_temp = B::key_temp().context(HomeSnafu)?;
    //
    //     let (ck, lg, k) = join!(
    //         copy(&cookies, &cookies_temp),
    //         copy(&login_data, &login_data_temp),
    //         copy(&key, &key_temp)
    //     );
    //     ck?;
    //     lg?;
    //     k?;
    //
    //     Ok(TempPaths {
    //         cookies_temp,
    //         login_data_temp,
    //         key_temp,
    //     })
    // }

    async fn cache_cookies(profile_path: PathBuf) -> Result<CookiesQuery> {
        let cookies = B::cookies(profile_path.clone());
        let cookies_temp = B::cookies_temp().context(HomeSnafu)?;

        copy(&cookies, &cookies_temp).await?;
        CookiesQuery::new(cookies_temp)
            .await
            .context(DbSnafu)
    }
}

impl<'b, B: FirefoxPath + Send + Sync> FirefoxBuilder<'b, B> {
    /// Get user specify profile path
    pub async fn get_profile_path(self) -> Result<PathBuf> {
        let mut base = if let Some(base) = self.base {
            base
        }
        else {
            let mut home = dirs::home_dir().context(HomeSnafu)?;
            home.push(B::BASE);
            home
        };
        let ini_path = base.join("profiles.ini");

        let ini_str = fs::read_to_string(&ini_path)
            .await
            .context(IoSnafu { path: ini_path })?;

        let ini_file = ini::Ini::load_from_str(&ini_str).context(IniParserSnafu)?;
        for (section, prop) in ini_file {
            let Some(section) = section
            else {
                continue;
            };
            if let Some(profile) = self.profile {
                if !section.starts_with("Profile") {
                    continue;
                }
                let Some(profile_name) = prop.get("Name")
                else {
                    continue;
                };
                if profile_name == profile {
                    let Some(var) = prop.get("Path")
                    else {
                        return Err(ProfilePathSnafu { profile: profile_name.to_owned() }.build());
                    };
                    base.push(var);
                    break;
                }
            }
            else if section.starts_with("Install") {
                let Some(default) = prop.get("Default")
                else {
                    return Err(InstallPathSnafu { install: section }.build());
                };
                base.push(default);
                break;
            }
        }

        Ok(base)
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "Firefox build", skip(self), fields(browser), level = "debug")
    )]
    pub async fn build(self) -> Result<FirefoxGetter<B>> {
        let profile_path = if let Some(path) = self.profile_path {
            path
        }
        else {
            self.get_profile_path().await?
        };

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(profile_path = %profile_path.display());
        };

        let cookies_query = Self::cache_cookies(profile_path).await?;

        Ok(FirefoxGetter {
            cookies_query,
            __browser: core::marker::PhantomData::<B>,
        })
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            name = "Firefox Cookie build",
            skip(self),
            fields(browser),
            level = "debug"
        )
    )]
    pub async fn build_cookie(self) -> Result<FirefoxCookieGetter<B>> {
        let profile_path = if let Some(path) = self.profile_path {
            path
        }
        else {
            self.get_profile_path().await?
        };

        #[cfg(feature = "tracing")]
        {
            tracing::Span::current().record("browser", B::NAME);
            tracing::debug!(profile_path = %profile_path.display());
        };

        let cookies_query = Self::cache_cookies(profile_path).await?;

        Ok(FirefoxCookieGetter {
            cookies_query,
            __browser: core::marker::PhantomData::<B>,
        })
    }

    #[deprecated(note = "use build_cookie")]
    pub async fn build_cookies(self) -> Result<FirefoxCookieGetter<B>> {
        self.build_cookie().await
    }
}
