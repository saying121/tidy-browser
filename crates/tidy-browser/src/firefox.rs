use std::{collections::HashSet, fmt::Display, path::PathBuf};

use decrypt_cookies::{
    firefox::{builder::FirefoxBuilderError, GetCookies as _},
    prelude::*,
};
use snafu::ResultExt;
use strum::IntoEnumIterator;
use tokio::task;

use crate::{
    args::{FirefoxName, Format, Value},
    error::{self, Result},
    utils::{self},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct FirefoxBased;

impl FirefoxBased {
    pub async fn multi_data<H>(
        names: impl Iterator<Item = FirefoxName>,
        output_dir: PathBuf,
        sep: String,
        host: H,
        format: Format,
    ) -> Result<()>
    where
        H: Into<Option<String>>,
    {
        let host = host.into();
        for task in names.map(|name| {
            let host = host.clone();
            let output_dir = output_dir.clone();
            let sep = sep.clone();
            let values = HashSet::from_iter(Value::iter());

            tokio::task::spawn(async move {
                Self::write_data(
                    name, None, None, None, host, values, output_dir, sep, format,
                )
                .await
            })
        }) {
            if let Err(e) = task
                .await
                .context(error::TokioTaskSnafu)?
            {
                match e {
                    error::Error::FirefoxBuilder {
                        source: FirefoxBuilderError::NotFoundBase { path, .. },
                        ..
                    } => {
                        tracing::info!(
                            r#"Not found {},
The browser is not installed or started with `-P`/`-profile` arg."#,
                            path.display()
                        );
                        todo!()
                    },
                    e => tracing::error!("{e}"),
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip_all, fields(name), level = "info")]
    #[expect(clippy::too_many_arguments, reason = "bin not lib")]
    pub async fn write_data<B, P, PP, H, S>(
        name: FirefoxName,
        base: B,
        profile: P,
        profile_path: PP,
        host: H,
        values: HashSet<Value>,
        mut output_dir: PathBuf,
        sep: S,
        format: Format,
    ) -> Result<()>
    where
        B: Into<Option<PathBuf>>,
        P: Into<Option<String>>,
        PP: Into<Option<PathBuf>>,
        H: Into<Option<String>>,
        S: Display + Send + Clone + 'static,
    {
        let base: Option<PathBuf> = base.into();
        let profile: Option<String> = profile.into();
        let profile_path: Option<PathBuf> = profile_path.into();
        let host: Option<String> = host.into();

        macro_rules! firefoxes {
            ($($browser:ident,) *) => {
                match name {
                    $(
                        FirefoxName::$browser => {
                            let firefox = if let Some(pp) = profile_path {
                                FirefoxBuilder::<$browser>::with_profile_path(pp)
                            }
                            else {
                                let mut builder = FirefoxBuilder::<$browser>::new();
                                if let Some(base) = base {
                                    builder.base(base);
                                }
                                if let Some(profile) = &profile {
                                    builder.profile(profile);
                                }
                                builder
                            }
                            .build()
                                .await
                                .context(error::FirefoxBuilderSnafu)?;

                            let task = if values.contains(&Value::Cookie) {
                                let host = host.clone();
                                let firefox = firefox.clone();
                                let task = task::spawn(async move {
                                    let cookies = if let Some(host) = host {
                                        firefox.cookies_by_host(host).await
                                    }
                                    else {
                                        firefox.cookies_all().await
                                    }
                                    .context(error::FirefoxSnafu)?;
                                    Ok::<_, error::Error>(cookies)
                                });
                                Some(task)
                            }
                            else {
                                None
                            };

                            if values.contains(&Value::Login) {
                                // TODO:
                            }
                            (task, $browser::NAME)
                        },
                    )*
                }
            };
        }

        let (ff, name) = firefoxes![Firefox, Librewolf, Floorp, Zen,];

        output_dir.push(name);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .with_context(|_| error::IoSnafu { path: output_dir.clone() })?;

        if let Some(ff) = ff {
            let cookies = ff
                .await
                .context(error::TokioTaskSnafu)??;
            let out_file = output_dir.join(crate::COOKIES_FILE_CSV);
            let sep = sep.clone();

            utils::write_cookies(out_file, cookies, sep, format)
                .await
                .context(error::TokioTaskSnafu)??;
        }

        Ok(())
    }
}
