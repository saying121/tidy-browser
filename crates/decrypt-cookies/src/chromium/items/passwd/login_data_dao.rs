use std::path::Path;

use sea_orm::{
    prelude::{DatabaseConnection, EntityTrait, QueryFilter},
    sea_query::IntoCondition,
    Database, DbErr,
};

use super::login_data_entities::{logins, prelude::Logins};

type Result<T> = std::result::Result<T, DbErr>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct LoginDataQuery {
    conn: DatabaseConnection,
}

impl LoginDataQuery {
    pub async fn new<P: AsRef<Path> + Send>(path: P) -> Result<Self> {
        let db_conn_str = format!("sqlite:{}?mode=rwc", path.as_ref().to_string_lossy());

        let db = Database::connect(db_conn_str).await?;
        Ok(Self { conn: db })
    }

    /// filter login data
    pub async fn query_login_dt_filter<F>(&self, filter: F) -> Result<Vec<logins::Model>>
    where
        F: IntoCondition + Send,
    {
        Logins::find()
            .filter(filter)
            .all(&self.conn)
            .await
    }
    /// query all login data
    pub async fn query_all_login_dt(&self) -> Result<Vec<logins::Model>> {
        Logins::find()
            .all(&self.conn)
            .await
    }
}
