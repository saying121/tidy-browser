//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone)]
#[derive(Debug)]
#[derive(DeriveEntityModel)]
#[derive(PartialEq, Eq)]
#[sea_orm(table_name = "cookies")]
pub struct Model {
    pub creation_utc:       i64,
    pub host_key:           String,
    pub top_frame_site_key: String,
    pub name:               String,
    pub value:              String,
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub encrypted_value:    Vec<u8>,
    pub path:               String,
    pub expires_utc:        i64,
    pub is_secure:          i32,
    pub is_httponly:        i32,
    pub last_access_utc:    i64,
    pub has_expires:        i32,
    pub is_persistent:      i32,
    pub priority:           i32,
    pub samesite:           i32,
    pub source_scheme:      i32,
    pub source_port:        i32,
    pub last_update_utc:    i64,
    // pub is_same_party:      i32,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
