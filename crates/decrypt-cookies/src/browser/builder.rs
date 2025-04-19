use std::path::PathBuf;

use super::{ChromiumPath, FirefoxPath};

// TODO: add browser name in error
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum BuilderError {
    #[error("No such file: {0:?}")]
    NoFile(PathBuf),
    #[error("Create dir failed: {0:?}")]
    CreateDir(PathBuf),
    #[error(transparent)]
    Ini(#[from] ini::Error),
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

async fn copy_temp_chromium<B: ChromiumPath>(
) -> crate::browser::builder::Result<crate::browser::builder::TempPaths> {
    use tokio::{fs, join};

    use crate::browser::builder::{BuilderError, TempPaths};

    let cookies = B::cookies();
    let cookies_temp = B::cookies_temp();

    let login_data = B::login_data();
    let login_data_temp = B::login_data_temp();

    let key = B::key();
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
    if cd_ck.is_err() {
        return Err(BuilderError::CreateDir(ck_temp_p.to_owned()));
    }
    if cd_lg.is_err() {
        return Err(BuilderError::CreateDir(lg_temp_p.to_owned()));
    }
    if cd_k.is_err() {
        return Err(BuilderError::CreateDir(k_temp_p.to_owned()));
    }

    let cookies_cp = fs::copy(&cookies, &cookies_temp);
    let login_cp = fs::copy(&login_data, &login_data_temp);
    let key_cp = fs::copy(&key, &key_temp);

    let (ck, lg, k) = join!(cookies_cp, login_cp, key_cp);
    if ck.is_err() {
        return Err(BuilderError::NoFile(cookies));
    }
    if lg.is_err() {
        return Err(BuilderError::NoFile(login_data));
    }
    if k.is_err() {
        return Err(BuilderError::NoFile(key));
    }

    Ok(TempPaths {
        cookies_temp,
        login_data_temp,
        key_temp,
    })
}

async fn copy_temp_firefox<B: FirefoxPath>(
    base: std::path::PathBuf,
    profile: Option<&str>,
) -> crate::browser::builder::Result<crate::browser::builder::TempPaths> {
    use tokio::{fs, join};

    let base = crate::browser::builder::firefox_profile(base, profile)?;
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
    if cd_ck.is_err() {
        return Err(crate::browser::builder::BuilderError::CreateDir(
            ck_temp_p.to_owned(),
        ));
    }
    if cd_lg.is_err() {
        return Err(crate::browser::builder::BuilderError::CreateDir(
            lg_temp_p.to_owned(),
        ));
    }
    if cd_k.is_err() {
        return Err(crate::browser::builder::BuilderError::CreateDir(
            k_temp_p.to_owned(),
        ));
    }

    let cookies_cp = fs::copy(&cookies, &cookies_temp);
    let login_cp = fs::copy(&login_data, &login_data_temp);
    let key_cp = fs::copy(&key, &key_temp);

    let (ck, lg, k) = join!(cookies_cp, login_cp, key_cp);
    if ck.is_err() {
        return Err(crate::browser::builder::BuilderError::NoFile(cookies));
    }
    if lg.is_err() {
        return Err(crate::browser::builder::BuilderError::NoFile(login_data));
    }
    if k.is_err() {
        return Err(crate::browser::builder::BuilderError::NoFile(key));
    }

    Ok(crate::browser::builder::TempPaths {
        cookies_temp,
        login_data_temp,
        key_temp,
    })
}

pub(crate) fn firefox_profile(mut base: PathBuf, profile: Option<&str>) -> Result<PathBuf> {
    let ini_path = base.join("profiles.ini");

    let Ok(ini_file) = ini::Ini::load_from_file(&ini_path)
    else {
        return Err(BuilderError::NoFile(ini_path));
    };
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

impl<B: ChromiumPath> std::fmt::Display for crate::chromium::ChromiumGetter<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(B::NAME)
    }
}

impl<B: ChromiumPath> crate::chromium::ChromiumBuilder<B> {
    pub fn new() -> Self {
        let mut base = dirs::home_dir().expect("Get home dir failed");

        base.push(B::BASE);

        Self {
            base,
            __browser: core::marker::PhantomData::<B>,
        }
    }

    /// When browser start with `--user-data-dir=DIR` or special other channel
    pub const fn with_user_data_dir(base: std::path::PathBuf) -> Self {
        Self {
            base,
            __browser: core::marker::PhantomData::<B>,
        }
    }
}

impl<B: ChromiumPath + Send> crate::chromium::ChromiumBuilder<B> {
    pub async fn build(
        self,
    ) -> crate::browser::builder::Result<crate::chromium::ChromiumGetter<B>> {
        use tokio::join;

        let temp_paths = copy_temp_chromium::<B>().await?;

        #[cfg(target_os = "linux")]
        let crypto = crate::chromium::crypto::linux::Decrypter::build(B::SAFE_STORAGE).await?;

        #[cfg(target_os = "macos")]
        let crypto =
            crate::chromium::crypto::macos::Decrypter::build(B::SAFE_STORAGE, B::SAFE_NAME)?;

        #[cfg(target_os = "windows")]
        let crypto = { crate::chromium::crypto::win::Decrypter::build(temp_paths.key_temp).await? };

        let (cookies_query, login_data_query) = (
            crate::chromium::items::cookie::cookie_dao::CookiesQuery::new(temp_paths.cookies_temp),
            crate::chromium::items::passwd::login_data_dao::LoginDataQuery::new(
                temp_paths.login_data_temp,
            ),
        );
        let (cookies_query, login_data_query) = join!(cookies_query, login_data_query);
        let (cookies_query, login_data_query) = (cookies_query?, login_data_query?);

        Ok(crate::chromium::ChromiumGetter {
            cookies_query,
            login_data_query,
            crypto,
            __browser: self.__browser,
        })
    }
}

impl<'b, B: FirefoxPath + Send> crate::firefox::FirefoxBuilder<'b, B> {
    pub fn new() -> crate::browser::builder::Result<Self> {
        Ok(Self {
            init: Some(Self::init()),
            profile: None,
            __browser: core::marker::PhantomData::<B>,
        })
    }

    /// `base`: When firefox base path changed
    /// `profile`: When start with `-P <profile>`
    pub fn with_path_profile<I, P>(init: I, profile: P) -> crate::browser::builder::Result<Self>
    where
        I: Into<Option<std::path::PathBuf>>,
        P: Into<Option<&'b str>>,
    {
        Ok(Self {
            init: init.into(),
            profile: profile.into(),
            __browser: core::marker::PhantomData::<B>,
        })
    }

    pub fn init() -> std::path::PathBuf {
        dirs::home_dir()
            .expect("Get home dir failed")
            .join(B::BASE)
    }

    pub async fn build(self) -> crate::browser::builder::Result<crate::firefox::FirefoxGetter<B>> {
        let temp_paths = copy_temp_firefox::<B>(
            self.init
                .unwrap_or_else(Self::init),
            self.profile,
        )
        .await?;

        let query =
            crate::firefox::items::cookie::dao::CookiesQuery::new(temp_paths.cookies_temp).await?;

        Ok(crate::firefox::FirefoxGetter {
            cookies_query: query,
            __browser: core::marker::PhantomData::<B>,
        })
    }
}

impl<B: FirefoxPath> std::fmt::Display for crate::firefox::FirefoxGetter<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(B::NAME)
    }
}
