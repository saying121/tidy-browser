use std::{fmt::Display, path::PathBuf};

use tokio::{fs, join};

use crate::{
    firefox::{FirefoxBuilder, FirefoxGetter},
    prelude::FirefoxPath,
};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum BuilderError {
    #[error(transparent)]
    Ini(#[from] ini::Error),
    #[error(transparent)]
    IniParser(#[from] ini::ParseError),
    #[error("Profile {0} missing `Name` properties")]
    ProfilePath(String),
    #[error("Install {0} missing `Default` properties")]
    InstallPath(String),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    Decrypter(#[from] crate::chromium::crypto::linux::CryptoError),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Decrypter(#[from] crate::chromium::crypto::win::CryptoError),
    #[cfg(target_os = "macos")]
    #[error(transparent)]
    Decrypt(#[from] crate::chromium::crypto::macos::CryptoError),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("Io: {source}, path: {path}")]
    Io {
        source: std::io::Error,
        path: PathBuf,
    },
}

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
    pub fn new() -> Result<Self> {
        Ok(Self {
            init: Some(Self::init()),
            profile: None,
            __browser: core::marker::PhantomData::<B>,
        })
    }

    pub fn init() -> PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(B::BASE)
    }

    /// `base`: When firefox base path changed
    /// `profile`: When start with `-P <profile>`
    pub fn with_path_profile<I, P>(init: I, profile: P) -> Result<Self>
    where
        I: Into<Option<PathBuf>>,
        P: Into<Option<&'b str>>,
    {
        Ok(Self {
            init: init.into(),
            profile: profile.into(),
            __browser: core::marker::PhantomData::<B>,
        })
    }

    /// get user profile
    pub async fn firefox_profile(mut base: PathBuf, profile: Option<&str>) -> Result<PathBuf> {
        let ini_path = base.join("profiles.ini");

        let ini_str = fs::read_to_string(&ini_path)
            .await
            .map_err(|e| BuilderError::Io { source: e, path: ini_path.clone() })?;

        let ini_file = ini::Ini::load_from_str(&ini_str)?;
        for (sec, prop) in ini_file {
            let Some(sec) = sec
            else {
                continue;
            };
            if let Some(profile) = profile {
                if !sec.starts_with("Profile") {
                    continue;
                }
                let Some(profile_name) = prop.get("Name")
                else {
                    continue;
                };
                if profile_name == profile {
                    let Some(var) = prop.get("Path")
                    else {
                        return Err(BuilderError::ProfilePath(profile_name.to_owned()));
                    };
                    base.push(var);
                    break;
                }
            }
            else {
                if !sec.starts_with("Install") {
                    continue;
                }
                let Some(default) = prop.get("Default")
                else {
                    return Err(BuilderError::InstallPath(sec));
                };
                base.push(default);
                break;
            }
        }

        tracing::debug!("path: {:?}", base);

        Ok(base)
    }

    async fn copy_temp_firefox(base: PathBuf, profile: Option<&str>) -> Result<TempPaths> {
        let base = Self::firefox_profile(base, profile).await?;
        let cookies = B::cookies(&base);
        let cookies_temp = B::cookies_temp();

        let login_data = B::login_data(&base);
        let login_data_temp = B::login_data_temp();

        let key = B::key(&base);
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

impl<'b, B: FirefoxPath + Send> FirefoxBuilder<'b, B> {
    pub async fn build(self) -> Result<FirefoxGetter<B>> {
        let temp_paths = Self::copy_temp_firefox(
            self.init
                .unwrap_or_else(Self::init),
            self.profile,
        )
        .await?;

        let query =
            crate::firefox::items::cookie::dao::CookiesQuery::new(temp_paths.cookies_temp).await?;

        Ok(FirefoxGetter {
            cookies_query: query,
            __browser: core::marker::PhantomData::<B>,
        })
    }
}
