use miette::{IntoDiagnostic, Result};
use sea_orm::{prelude::*, Database, FromQueryResult, QuerySelect};

use super::key4db::{meta_data, prelude::*};
use crate::{browser::BrowserFile, firefox::path::file_path, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Key4Query {
    conn: DatabaseConnection,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "NssPrivate")]
pub struct NssPrivPart {
    pub a11:  Vec<u8>,
    pub a102: Vec<u8>,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "MetaData")]
pub struct Items {
    pub item1: Vec<u8>,
    pub item2: Vec<u8>,
}

impl Key4Query {
    /// * `browser`: `edge`, `chrome`
    pub async fn new(browser: Browser) -> Result<Self> {
        let cookie_path = file_path(browser, BrowserFile::Key).await?;

        let db_url = format!("sqlite:{}?mode=ro", cookie_path.to_string_lossy());

        let db = Database::connect(db_url)
            .await
            .into_diagnostic()?;
        Ok(Self { conn: db })
    }

    pub async fn query_metadata(&self) -> Result<Items> {
        let res = MetaData::find()
            .filter(meta_data::Column::Id.eq("password"))
            .into_partial_model::<Items>()
            .one(&self.conn)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette::miette!("not found id=password"))?;

        Ok(res)
    }
    pub async fn query_nssprivate(&self) -> Result<NssPrivPart> {
        let res = NssPrivate::find()
            .select_only()
            .into_partial_model::<NssPrivPart>()
            .one(&self.conn)
            .await
            .into_diagnostic()?
            .ok_or_else(|| miette::miette!("not found nss_private a11, a102"))?;
        Ok(res)
    }
}
