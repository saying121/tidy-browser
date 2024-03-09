use miette::{IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::debug;

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};
use crate::{firefox::path::file_path, Browser, BrowserFile};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    pub async fn new(browser: Browser) -> miette::Result<Self> {
        let cookie_path = file_path(browser, BrowserFile::Cookies).await?;

        let db_conn_str = format!("sqlite:{}?mode=rwc", cookie_path.to_string_lossy());

        debug!("database dir: {}", &db_conn_str);

        let db = Database::connect(db_conn_str)
            .await
            .into_diagnostic()?;
        Ok(Self { conn: db })
    }
    pub async fn query_cookie(&self, host: &str) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .filter(moz_cookies::Column::Host.contains(host))
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
}
