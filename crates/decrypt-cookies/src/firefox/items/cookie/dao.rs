use std::path::Path;

use miette::{IntoDiagnostic, Result};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
};
use tracing::debug;

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};

/// query firefox based cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    pub async fn new<P>(path: P) -> miette::Result<Self>
    where
        P: AsRef<Path>,
    {
        let db_conn_str = format!("sqlite:{}?mode=rwc", path.as_ref().to_string_lossy());

        debug!("database dir: {}", &db_conn_str);

        let db = Database::connect(db_conn_str)
            .await
            .into_diagnostic()?;
        Ok(Self { conn: db })
    }

    pub async fn query_cookie_filter<F>(&self, filter: F) -> Result<Vec<Model>>
    where
        F: IntoCondition,
    {
        let res = MozCookies::find()
            .filter(filter)
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
    pub async fn query_cookie_by_host(&self, host: &str) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .filter(moz_cookies::Column::Host.contains(host))
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
    pub async fn query_all_cookie(&self) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .all(&self.conn)
            .await
            .into_diagnostic()?;

        Ok(res)
    }
}
