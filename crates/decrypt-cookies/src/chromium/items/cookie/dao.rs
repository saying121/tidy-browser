use std::path::Path;

use miette::{IntoDiagnostic, Result};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
};

use super::entities::{
    cookies::{self, Model},
    prelude::*,
};

#[derive(Clone, Debug, Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    /// * `browser`: `edge`, `chrome`
    pub async fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let db_conn_str = format!("sqlite:{}?mode=rwc", path.as_ref().to_string_lossy());

        let db = Database::connect(db_conn_str).await.into_diagnostic()?;
        Ok(Self { conn: db })
    }

    /// get raw Cookies
    pub async fn query_cookie_filter<F>(&self, filter: F) -> Result<Vec<Model>>
    where
        F: IntoCondition,
    {
        let res = CookiesDB::find()
            .filter(filter)
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }

    /// get raw Cookies
    pub async fn query_cookie_by_host(&self, host: &str) -> Result<Vec<Model>> {
        let res = CookiesDB::find()
            .filter(cookies::Column::HostKey.contains(host))
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
    /// get raw Cookies
    pub async fn query_all_cookie(&self) -> Result<Vec<Model>> {
        let res = CookiesDB::find().all(&self.conn).await.into_diagnostic()?;

        Ok(res)
    }
}
