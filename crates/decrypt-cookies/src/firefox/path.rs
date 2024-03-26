use std::path::PathBuf;

use miette::{IntoDiagnostic, Result};
use tokio::fs::read_to_string;

/// just impl the `base` method
pub trait FFPath {
    const COOKIES: &'static str = "cookies.sqlite";
    const DATAS: &'static str = "places.sqlite"; // bookmarks etc.
    const KEY: &'static str = "key4.db"; // key sqlite3
    const STORAGE: &'static str = "webappsstore.sqlite"; // web storage data
    const PASSWD: &'static str = "logins.json"; // passwd
    const EXTENSIONS: &'static str = "extensions.json";

    fn base(&self) -> &PathBuf;

    /// json
    fn extensions(&self) -> PathBuf {
        self.base().join(Self::EXTENSIONS)
    }
    /// json
    fn passwd(&self) -> PathBuf {
        self.base().join(Self::PASSWD)
    }
    /// sqlite3
    fn storage(&self) -> PathBuf {
        self.base().join(Self::STORAGE)
    }
    /// sqlite3
    fn key(&self) -> PathBuf {
        self.base().join(Self::KEY)
    }
    /// sqlite3
    fn datas(&self) -> PathBuf {
        self.base().join(Self::DATAS)
    }
    /// sqlite3
    fn cookies(&self) -> PathBuf {
        self.base().join(Self::COOKIES)
    }

    fn helper(
        init_path: PathBuf,
        base: &str,
    ) -> impl std::future::Future<Output = Result<PathBuf>> + Send {
        let mut ini_path = init_path.clone();
        ini_path.push(format!("{}/profiles.ini", base));
        async move {
            if !ini_path.exists() {
                miette::bail!(
                    "{} not exists",
                    ini_path
                        .to_str()
                        .unwrap_or_default()
                );
            }
            let str = read_to_string(ini_path)
                .await
                .into_diagnostic()?;
            let ini_file = ini::Ini::load_from_str(&str).into_diagnostic()?;
            let mut section = String::new();
            for (sec, prop) in ini_file {
                let Some(sec) = sec
                else {
                    continue;
                };
                if sec.starts_with("Install") {
                    prop.get("Default")
                        .unwrap_or_default()
                        .clone_into(&mut section);
                    break;
                }
            }

            tracing::debug!("section: {}", section);

            let mut res = init_path;
            res.push(format!("{}/{}", base, section));
            tracing::debug!("path: {:?}", res);

            Ok(res)
        }
    }
}
