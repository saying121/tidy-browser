use std::{collections::HashSet, fmt::Display, fs::File, io::IoSlice, path::PathBuf};

use decrypt_cookies::{
    chromium::{builder::ChromiumBuilderError, GetCookies, GetLogins},
    prelude::*,
};
use snafu::ResultExt;
use strum::IntoEnumIterator;
use tokio::task;

use crate::{
    args::{ChromiumName, Format, Value},
    error::{self, IoSnafu, JsonSnafu, Result},
    utils::{self, write_all_vectored},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ChromiumBased;

fn login_csv_header<D: Display>(sep: D) -> String {
    format!("url{sep}username{sep}display_name{sep}password{sep}date_created{sep}date_last_used{sep}modified")
}

impl ChromiumBased {
    pub(crate) async fn multi_data<H>(
        names: impl Iterator<Item = ChromiumName>,
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
                Self::write_data(name, None, host, values, output_dir, sep, format).await
            })
        }) {
            if let Err(e) = task
                .await
                .context(error::TokioTaskSnafu)?
            {
                match e {
                    error::Error::ChromiumBuilder {
                        source: source @ ChromiumBuilderError::NotFoundBase { .. },
                        ..
                    } => {
                        #[cfg(not(target_os = "windows"))]
                        tracing::info!(r#"{source}"#,);
                        #[cfg(target_os = "windows")]
                        tracing::info!(
                            r#"{source}
When you use scoop on Windows, the data path is located at `~\scoop\persisst\<name>\<xxx>`"#,
                        );
                    },
                    e => tracing::error!("{e}"),
                }
            }
        }

        Ok(())
    }

    pub async fn write_data<D, H, S>(
        name: ChromiumName,
        data_dir: D,
        host: H,
        values: HashSet<Value>,
        mut output_dir: PathBuf,
        sep: S,
        format: Format,
    ) -> Result<()>
    where
        D: Into<Option<PathBuf>>,
        H: Into<Option<String>>,
        S: Display + Send + Clone + 'static,
    {
        let data_dir = data_dir.into();
        let host: Option<String> = host.into();

        macro_rules! chromiums {
            ($($browser:ident,) *) => {
                match name {
                    $(
                    ChromiumName::$browser => {
                        let chromium = if let Some(dir) = data_dir {
                            ChromiumBuilder::<$browser>::with_user_data_dir(dir)
                        }
                        else {
                            ChromiumBuilder::new()
                        }
                        .build()
                        .await
                        .context(error::ChromiumBuilderSnafu)?;

                        let cookies = if values.contains(&Value::Cookie) {
                            let host = host.clone();
                            let chromium = chromium.clone();
                            let task = task::spawn(async move {
                                let cookies = if let Some(host) = host {
                                    chromium
                                        .cookies_by_host(host)
                                        .await
                                }
                                else {
                                    chromium.cookies_all().await
                                }
                                .context(error::ChromiumSnafu)?;
                                Ok::<_, error::Error>(cookies)
                            });
                            Some(task)
                        }
                        else {
                            None
                        };

                        let logins = if values.contains(&Value::Login) {
                            let host = host.clone();
                            let task = task::spawn(async move {
                                let logins = if let Some(host) = host {
                                    chromium.logins_by_host(host).await
                                }
                                else {
                                    chromium.all_logins().await
                                }
                                .context(error::ChromiumSnafu)?;
                                Ok::<_, error::Error>(logins)
                            });
                            Some(task)
                        }
                        else {
                            None
                        };
                        (cookies, logins, $browser::NAME)
                    },
                    )*
                }
            };
        }

        #[cfg(target_os = "linux")]
        let (cookies, logins, name) =
            chromiums![Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera,];
        #[cfg(not(target_os = "linux"))]
        let (cookies, logins, name) = chromiums![
            Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera, Arc, OperaGX, CocCoc,
        ];
        let (cookies, logins, cap) = match (cookies, logins) {
            (None, None) => (None, None, 0),
            (None, Some(logins)) => {
                let l = logins
                    .await
                    .context(error::TokioTaskSnafu)??;
                (None, Some(l), 1)
            },
            (Some(cookies), None) => {
                let c = cookies
                    .await
                    .context(error::TokioTaskSnafu)??;
                (Some(c), None, 1)
            },
            (Some(cookies), Some(logins)) => {
                let (c, l) = tokio::join!(cookies, logins);
                let c = c.context(error::TokioTaskSnafu)??;
                let l = l.context(error::TokioTaskSnafu)??;
                (Some(c), Some(l), 2)
            },
        };

        output_dir.push(name);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .with_context(|_| error::IoSnafu { path: output_dir.clone() })?;

        let mut tasks = Vec::with_capacity(cap);

        if let Some(cookies) = cookies {
            let out_file = output_dir.join({
                match format {
                    Format::Csv => crate::COOKIES_FILE_CSV,
                    Format::Json => crate::COOKIES_FILE_JSON,
                    Format::JsonLines => crate::COOKIES_FILE_JSONL,
                }
            });
            let sep = sep.clone();

            let handle = utils::write_cookies(out_file, cookies, sep, format);
            tasks.push(handle);
        }

        if let Some(logins) = logins {
            let sep = sep.clone();

            let handle = task::spawn_blocking(move || {
                let out_file = output_dir.join(match format {
                    Format::Csv => crate::LOGINS_FILE_CSV,
                    Format::Json => crate::LOGINS_FILE_JSON,
                    Format::JsonLines => crate::LOGINS_FILE_JSONL,
                });

                let mut file = File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&out_file)
                    .with_context(|_| error::IoSnafu { path: out_file.clone() })?;

                match format {
                    Format::Csv => {
                        let mut slices = Vec::with_capacity(2 + logins.len() * 2);

                        let header = login_csv_header(sep.clone());
                        slices.push(IoSlice::new(header.as_bytes()));
                        slices.push(IoSlice::new(b"\n"));

                        let csvs: Vec<_> = logins
                            .into_iter()
                            .map(|v| v.to_csv(sep.clone()))
                            .collect();

                        for csv in &csvs {
                            slices.push(IoSlice::new(csv.as_bytes()));
                            slices.push(IoSlice::new(b"\n"));
                        }
                        write_all_vectored(&mut file, &mut slices)
                            .with_context(|_| error::IoSnafu { path: out_file })
                    },
                    Format::Json => serde_json::to_writer(file, &logins).context(JsonSnafu),
                    Format::JsonLines => serde_jsonlines::write_json_lines(&out_file, &logins)
                        .context(IoSnafu { path: out_file }),
                }
            });
            tasks.push(handle);
        }

        for ele in tasks {
            ele.await
                .context(error::TokioTaskSnafu)??;
        }

        Ok(())
    }
}
