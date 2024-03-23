use std::path::PathBuf;

use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::fs;
use tracing::debug;

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};
use crate::{browser::BrowserFile, firefox::path::file_path, Browser};

/// query firefox based cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
    temp_path: Option<PathBuf>,
}

impl Drop for CookiesQuery {
    fn drop(&mut self) {
        if let Some(path) = self.temp_path.take() {
            std::thread::spawn(|| std::fs::remove_file(path));
        }
    }
}

impl CookiesQuery {
    pub async fn new(browser: Browser) -> miette::Result<Self> {
        let cookie_path = file_path(browser, BrowserFile::Cookies).await?;

        let mut temp_path = dirs::cache_dir().expect("get cache dir failed");
        temp_path.push(format!("browser_temp/{browser}"));
        fs::create_dir_all(&temp_path)
            .await
            .into_diagnostic()?;
        temp_path.push(
            cookie_path
                .file_name()
                .expect("get filename falied"),
        );

        fs::copy(&cookie_path, &temp_path)
            .await
            .into_diagnostic()?;

        let db_conn_str = format!("sqlite:{}?mode=rwc", temp_path.to_string_lossy());

        debug!("database dir: {}", &db_conn_str);

        let db = Database::connect(db_conn_str)
            .await
            .into_diagnostic()?;
        Ok(Self {
            conn: db,
            temp_path: Some(temp_path),
        })
    }
    pub async fn query_cookie(&self, host: &str) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .filter(moz_cookies::Column::Host.contains(host))
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
    pub async fn all_cookie(&self) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
}
