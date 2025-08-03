use std::path::Path;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

use super::cookie_entities::{
    cookies::{self, Model},
    prelude::*,
};
use crate::utils::connect_db;

type Result<T> = std::result::Result<T, DbErr>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    pub async fn new<P: AsRef<Path> + Send>(path: P) -> Result<Self> {
        let db = connect_db(&path).await?;
        Ok(Self { conn: db })
    }

    /// get raw Cookies
    pub async fn cookies_filter<F>(&self, filter: F) -> Result<Vec<Model>>
    where
        F: IntoCondition + Send,
    {
        Cookies::find()
            .filter(filter)
            .all(&self.conn)
            .await
    }

    /// get raw Cookies
    pub async fn cookies_by_host(&self, host: &str) -> Result<Vec<Model>> {
        Cookies::find()
            .filter(cookies::Column::HostKey.contains(host))
            .all(&self.conn)
            .await
    }

    /// get raw Cookies
    pub async fn cookies_all(&self) -> Result<Vec<Model>> {
        Cookies::find()
            .all(&self.conn)
            .await
    }
}
