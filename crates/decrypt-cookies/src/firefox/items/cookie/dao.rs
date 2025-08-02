use std::path::Path;

use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

use super::entities::{
    moz_cookies::{self, Model},
    prelude::*,
};
use crate::utils::connect_db;

type Result<T> = std::result::Result<T, DbErr>;

/// query Firefox based cookies
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct CookiesQuery {
    conn: DatabaseConnection,
}

impl CookiesQuery {
    pub async fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path> + Send,
    {
        let db = connect_db(&path).await?;
        Ok(Self { conn: db })
    }

    pub async fn query_cookie_filter<F>(&self, filter: F) -> Result<Vec<Model>>
    where
        F: IntoCondition + Send,
    {
        let res = MozCookies::find()
            .filter(filter)
            .all(&self.conn)
            .await?;

        Ok(res)
    }

    pub async fn query_cookie_by_host(&self, host: &str) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .filter(moz_cookies::Column::Host.contains(host))
            .all(&self.conn)
            .await?;

        Ok(res)
    }
    pub async fn query_all_cookie(&self) -> Result<Vec<Model>> {
        let res = MozCookies::find()
            .all(&self.conn)
            .await?;

        Ok(res)
    }
}
