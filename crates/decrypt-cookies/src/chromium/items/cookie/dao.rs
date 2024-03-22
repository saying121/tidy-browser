use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

use super::entities::{
    cookies::{self, Model},
    prelude::*,
};
use crate::{browser::BrowserFile, chromium::utils::path::file_path, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    /// * `browser`: `edge`, `chrome`
    pub async fn new(browser: Browser) -> Result<Self> {
        let cookie_path = file_path(browser, BrowserFile::Cookies);
        tracing::debug!(path = ?cookie_path);

        let db_conn_str = format!("sqlite:{}?mode=rwc", cookie_path.to_string_lossy());

        let db = Database::connect(db_conn_str)
            .await
            .into_diagnostic()?;
        Ok(Self { conn: db })
    }

    pub async fn query_cookie(&self, host: &str) -> Result<Vec<Model>> {
        let res = CookiesDB::find()
            .filter(cookies::Column::HostKey.contains(host))
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
}
