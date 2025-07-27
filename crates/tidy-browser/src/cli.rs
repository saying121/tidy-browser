use std::{collections::HashSet, path::PathBuf, str::FromStr};

use snafu::ResultExt;
use strum::IntoEnumIterator;

use crate::{
    args::{self, ChromiumArgs, ChromiumName, FirefoxArgs, FirefoxName},
    chromium::ChromiumBased,
    error::{self, Result},
    firefox::FirefoxBased,
};
#[cfg(target_os = "macos")]
use crate::{
    args::{SafariArgs, Value},
    safari::SafariBased,
};

pub async fn run_cli(args: crate::args::Args) -> Result<()> {
    let output_dir = args
        .output_dir
        .unwrap_or_else(|| PathBuf::from_str("results").unwrap());

    if args.all_browsers {
        let chromium = tokio::spawn({
            let output_dir = output_dir.clone();
            let sep = args.sep.clone();
            let host = args.host.clone();
            async move { ChromiumBased::multi_data(ChromiumName::iter(), output_dir, sep, host).await }
        });

        let firefox = tokio::spawn({
            let host = args.host.clone();
            let output_dir = output_dir.clone();
            let sep = args.sep.clone();
            async move { FirefoxBased::multi_data(FirefoxName::iter(), output_dir, sep, host).await }
        });

        #[cfg(target_os = "macos")]
        let safari = tokio::spawn({
            let host = args.host.clone();
            async move {
                SafariBased::write_data(
                    HashSet::from_iter(Value::iter()),
                    None,
                    host,
                    args.sep,
                    output_dir,
                )
                .await
            }
        });

        #[cfg(not(target_os = "macos"))]
        let (c, f) = tokio::join!(chromium, firefox);
        #[cfg(target_os = "macos")]
        let (c, f, s) = tokio::join!(chromium, firefox, safari);

        c.context(error::TokioTaskSnafu)??;
        f.context(error::TokioTaskSnafu)??;
        #[cfg(target_os = "macos")]
        s.context(error::TokioTaskSnafu)??;

        return Ok(());
    }
    match args.core {
        args::Core::Chromium(ChromiumArgs { name, user_data_dir, values }) => {
            ChromiumBased::write_data(
                name,
                user_data_dir,
                args.host,
                HashSet::from_iter(values.into_iter()),
                output_dir,
                args.sep,
            )
            .await?;
        },
        args::Core::Firefox(FirefoxArgs {
            name,
            base,
            profile,
            profile_path,
            values,
        }) => {
            FirefoxBased::write_data(
                name,
                base,
                profile,
                profile_path,
                args.host,
                HashSet::from_iter(values.into_iter()),
                output_dir,
                args.sep,
            )
            .await?
        },
        #[cfg(target_os = "macos")]
        args::Core::Safari(SafariArgs { values, cookies_path }) => {
            SafariBased::write_data(
                HashSet::from_iter(values.into_iter()),
                cookies_path,
                args.host,
                args.sep,
                output_dir,
            )
            .await?
        },
    }

    Ok(())
}
