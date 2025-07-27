use std::{collections::HashSet, fmt::Display, path::PathBuf};

use decrypt_cookies::prelude::{SafariBuilder, SafariGetter};
use snafu::ResultExt;

use crate::{
    args::Value,
    error::{self, Result},
    utils,
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SafariBased;

impl SafariBased {
    pub async fn write_data<C, H, S>(
        values: HashSet<Value>,
        cookies_path: C,
        host: H,
        sep: S,
        mut output_dir: PathBuf,
    ) -> Result<()>
    where
        C: Into<Option<PathBuf>>,
        H: Into<Option<String>>,
        S: Display + Send + Clone + 'static,
    {
        let safari = if let Some(path) = cookies_path.into() {
            let mut safari_builder = SafariBuilder::new();
            safari_builder.cookies_path(path);
            safari_builder
        }
        else {
            SafariBuilder::new()
        }
        .build()
        .await
        .context(error::SafariSnafu)?;
        let host: Option<String> = host.into();

        if values.contains(&Value::Cookie) {
            let cookies = if let Some(host) = &host {
                safari
                    .cookies_by_host(host)
                    .cloned()
                    .collect()
            }
            else {
                safari.cookies_all().to_vec()
            };

            output_dir.push(SafariGetter::NAME);

            tokio::fs::create_dir_all(&output_dir)
                .await
                .context(error::IoSnafu { path: output_dir.clone() })?;

            let out_file = output_dir.join(crate::COOKIES_FILE);

            utils::write_cookies(out_file, cookies, sep)
                .await
                .context(error::TokioTaskSnafu)??;
        }

        Ok(())
    }
}
