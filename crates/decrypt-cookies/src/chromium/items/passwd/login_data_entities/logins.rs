//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

// NOTE: the comment field Yandex browser not have
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "logins")]
pub struct Model {
    pub origin_url:                     String,
    pub action_url:                     Option<String>,
    pub username_element:               Option<String>,
    pub username_value:                 Option<String>,
    pub password_element:               Option<String>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub password_value:                 Option<Vec<u8>>,
    pub submit_element:                 Option<String>,
    pub signon_realm:                   String,
    pub date_created:                   i64,
    pub blacklisted_by_user:            i32,
    pub scheme:                         i32,
    pub password_type:                  Option<i32>,
    pub times_used:                     Option<i64>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub form_data:                      Option<Vec<u8>>,
    pub display_name:                   Option<String>,
    pub icon_url:                       Option<String>,
    pub federation_url:                 Option<String>,
    pub skip_zero_click:                Option<i32>,
    pub generation_upload_status:       Option<i32>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub possible_username_pairs:        Option<Vec<u8>>,
    #[sea_orm(primary_key,auto_increment=true)]
    pub id:                             i32,
    pub date_last_used:                 i64,
    // #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    // pub moving_blocked_for:             Option<Vec<u8>>,
    pub date_password_modified:         i64,
    // pub sender_email:                   Option<String>,
    // pub sender_name:                    Option<String>,
    // pub date_received:                  Option<i64>,
    // pub sharing_notification_displayed: i32,
    // #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    // pub keychain_identifier:            Option<Vec<u8>>,
    // pub sender_profile_image_url:       Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::insecure_credentials::Entity")]
    InsecureCredentials,
    #[sea_orm(has_many = "super::password_notes::Entity")]
    PasswordNotes,
}

impl Related<super::insecure_credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InsecureCredentials.def()
    }
}

impl Related<super::password_notes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PasswordNotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
