use std::{collections::HashSet, path::PathBuf, str::FromStr};

use snafu::ResultExt;
use strum::IntoEnumIterator;

use crate::{
    args::{self, ChromiumArgs, ChromiumName, FirefoxArgs, SafariArgs},
    chromium::ChromiumBased,
    error::{self, Result},
    firefox::FirefoxBased,
};

pub async fn run_cli(args: crate::args::Args) -> Result<()> {
    let output_dir = args
        .output_dir
        .unwrap_or_else(|| PathBuf::from_str("results").unwrap());

    if args.all_browsers {
        let chromium = tokio::spawn({
            let output_dir = output_dir.clone();
            let sep = args.sep.clone();
            async move { ChromiumBased::multi_data(ChromiumName::iter(), output_dir, sep).await }
        });
        let firefox = tokio::spawn(async {
            FirefoxBased::multi_data(FirefoxName::iter(), output_dir, args.sep).await
        });
        let (c, f) = tokio::join!(chromium, firefox);
        c.context(error::TokioTaskSnafu)??;
        f.context(error::TokioTaskSnafu)??;

        return Ok(());
    }
    match args.core {
        args::Core::Chromium(ChromiumArgs { name, user_data_dir, host, values }) => {
            ChromiumBased::write_data(
                name,
                user_data_dir,
                host,
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
            host,
            values,
        }) => {
            FirefoxBased::write_data(
                name,
                base,
                profile,
                profile_path,
                host,
                HashSet::from_iter(values.into_iter()),
                output_dir,
                args.sep,
            )
            .await?
        },
        #[cfg(target_os = "macos")]
        args::Core::Safari(SafariArgs { values }) => {},
    }

    Ok(())
}
