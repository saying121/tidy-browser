use std::{collections::HashSet, fmt::Display, fs::File, io::IoSlice, path::PathBuf};

use decrypt_cookies::{chromium::ChromiumCookie, prelude::*};
use snafu::ResultExt;
use strum::IntoEnumIterator;
use tokio::task;

use self::cookies::CookiesInfo;
use crate::{
    args::{FirefoxName, Value},
    error::{self, Result},
    utils::write_all_vectored,
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct FirefoxBased;

impl FirefoxBased {
    pub async fn multi_data(
        names: impl Iterator<Item = FirefoxName>,
        output_dir: PathBuf,
        sep: String,
    ) -> Result<()> {
        for task in names.map(|name| {
            let output_dir = output_dir.clone();
            let sep = sep.clone();
            let values = HashSet::from_iter(Value::iter());

            tokio::task::spawn(async move {
                Self::write_data(name, None, None, None, None, values, output_dir, sep).await
            })
        }) {
            task.await
                .context(error::TokioTaskSnafu)??;
        }

        Ok(())
    }

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
                                        firefox.all_cookies().await
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

        let (ff, name) = firefoxes![Firefox, Librewolf, Floorp,];

        output_dir.push(name);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .context(error::IoSnafu { path: output_dir.clone() })?;

        if let Some(ff) = ff {
            let cookies = ff
                .await
                .context(error::TokioTaskSnafu)??;
            let out_file = output_dir.join("cookies.csv");
            let sep = sep.clone();

            task::spawn_blocking(move || {
                let mut file = File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&out_file)
                    .context(error::IoSnafu { path: out_file.clone() })?;

                let header = <ChromiumCookie as CookiesInfo>::csv_header(sep.clone());

                let mut slices = Vec::with_capacity(2 + cookies.len() * 2);
                slices.push(IoSlice::new(header.as_bytes()));
                slices.push(IoSlice::new(b"\n"));

                let csvs: Vec<_> = cookies
                    .into_iter()
                    .map(|v| v.to_csv(sep.clone()))
                    .collect();

                for csv in &csvs {
                    slices.push(IoSlice::new(csv.as_bytes()));
                    slices.push(IoSlice::new(b"\n"));
                }

                write_all_vectored(&mut file, &mut slices)
                    .context(error::IoSnafu { path: out_file.clone() })?;

                Ok::<(), error::Error>(())
            })
            .await
            .context(error::TokioTaskSnafu)??;
        }

        Ok(())
    }
}
