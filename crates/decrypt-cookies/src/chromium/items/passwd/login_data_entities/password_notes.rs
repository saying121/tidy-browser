//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "password_notes")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment=true)]
    pub id:           i32,
    pub parent_id:    i32,
    pub key:          String,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub value:        Option<Vec<u8>>,
    pub date_created: i64,
    pub confidential: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::logins::Entity",
        from = "Column::ParentId",
        to = "super::logins::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Logins,
}

impl Related<super::logins::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logins.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
