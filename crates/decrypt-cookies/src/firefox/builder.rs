use std::{fmt::Display, marker::PhantomData, path::PathBuf};

use tokio::{fs, join};

use crate::{
    firefox::{items::cookie::dao::CookiesQuery, FirefoxGetter},
    prelude::FirefoxPath,
};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum FirefoxBuilderError {
    #[error(transparent)]
    Ini(#[from] ini::Error),
    #[error(transparent)]
    IniParser(#[from] ini::ParseError),
    #[error("Profile {0} missing `Name` properties")]
    ProfilePath(String),
    #[error("Install {0} missing `Default` properties")]
    InstallPath(String),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("Io: {source}, path: {path}")]
    Io {
        source: std::io::Error,
        path: PathBuf,
    },
}

pub type Result<T> = std::result::Result<T, FirefoxBuilderError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TempPaths {
    pub(crate) cookies_temp: PathBuf,
    pub(crate) login_data_temp: PathBuf,
    pub(crate) key_temp: PathBuf,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct FirefoxBuilder<'a, T> {
    pub(crate) init: Option<PathBuf>,
    pub(crate) profile: Option<&'a str>,
    pub(crate) profile_path: Option<&'a str>,
    pub(crate) __browser: PhantomData<T>,
}

impl<B: FirefoxPath> Display for FirefoxGetter<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(B::NAME)
    }
}

impl<B: FirefoxPath> Display for FirefoxBuilder<'_, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}Builder", B::NAME))
    }
}

impl<'b, B: FirefoxPath> FirefoxBuilder<'b, B> {
    pub const fn new() -> Self {
        Self {
            init: None,
            profile: None,
            profile_path: None,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// Get firefox data dir
    pub fn init() -> PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(B::BASE)
    }

    /// `profile_path`: when browser start with `-profile <profile_path>`
    pub fn with_profile_path<P>(profile_path: P) -> Result<Self>
    where
        P: Into<Option<&'b str>>,
    {
        Ok(Self {
            init: None,
            profile: None,
            profile_path: profile_path.into(),
            __browser: core::marker::PhantomData::<B>,
        })
    }

    /// `init`: When firefox init path changed
    /// `profile`: When start with `-P <profile>`
    pub fn with_base_profile<I, P>(init: I, profile: P) -> Self
    where
        I: Into<Option<PathBuf>>,
        P: Into<Option<&'b str>>,
    {
        Self {
            init: init.into(),
            profile: profile.into(),
            profile_path: None,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    async fn cache_data(profile_path: PathBuf) -> Result<TempPaths> {
        let cookies = B::cookies(profile_path.clone());
        let cookies_temp = B::cookies_temp();

        let login_data = B::login_data(profile_path.clone());
        let login_data_temp = B::login_data_temp();

        let key = B::key(profile_path.clone());
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
        cd_ck.map_err(|e| FirefoxBuilderError::Io {
            source: e,
            path: ck_temp_p.to_owned(),
        })?;
        cd_lg.map_err(|e| FirefoxBuilderError::Io {
            source: e,
            path: lg_temp_p.to_owned(),
        })?;
        cd_k.map_err(|e| FirefoxBuilderError::Io {
            source: e,
            path: k_temp_p.to_owned(),
        })?;

        let cookies_cp = fs::copy(&cookies, &cookies_temp);
        let login_cp = fs::copy(&login_data, &login_data_temp);
        let key_cp = fs::copy(&key, &key_temp);

        let (ck, lg, k) = join!(cookies_cp, login_cp, key_cp);
        ck.map_err(|e| FirefoxBuilderError::Io { source: e, path: cookies })?;
        lg.map_err(|e| FirefoxBuilderError::Io { source: e, path: login_data })?;
        k.map_err(|e| FirefoxBuilderError::Io { source: e, path: key })?;

        Ok(TempPaths {
            cookies_temp,
            login_data_temp,
            key_temp,
        })
    }
}

impl<'b, B: FirefoxPath + Send + Sync> FirefoxBuilder<'b, B> {
    /// Get user specify profile path
    pub async fn get_profile_path(&self) -> Result<PathBuf> {
        let mut base = self
            .init
            .clone()
            .unwrap_or_else(Self::init);
        let ini_path = base.join("profiles.ini");

        let ini_str = fs::read_to_string(&ini_path)
            .await
            .map_err(|e| FirefoxBuilderError::Io { source: e, path: ini_path })?;

        let ini_file = ini::Ini::load_from_str(&ini_str)?;
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
                        return Err(FirefoxBuilderError::ProfilePath(profile_name.to_owned()));
                    };
                    base.push(var);
                    break;
                }
            }
            else {
                if !section.starts_with("Install") {
                    continue;
                }
                let Some(default) = prop.get("Default")
                else {
                    return Err(FirefoxBuilderError::InstallPath(section));
                };
                base.push(default);
                break;
            }
        }

        tracing::debug!("path: {:?}", base);

        Ok(base)
    }

    pub async fn build(self) -> Result<FirefoxGetter<B>> {
        let profile_path = if let Some(path) = self.profile_path {
            path.into()
        }
        else {
            self.get_profile_path().await?
        };
        let temp_paths = Self::cache_data(profile_path).await?;

        let query = CookiesQuery::new(temp_paths.cookies_temp).await?;

        Ok(FirefoxGetter {
            cookies_query: query,
            __browser: core::marker::PhantomData::<B>,
        })
    }
}
