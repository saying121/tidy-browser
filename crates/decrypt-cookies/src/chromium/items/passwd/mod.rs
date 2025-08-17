use std::fmt::Display;

use chrono::{DateTime, Utc};

use self::login_data_entities::logins;
use super::I64ToChromiumDateTime;

pub mod login_data_dao;
pub mod login_data_entities;

#[non_exhaustive]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct LoginData {
    pub origin_url: String,
    pub action_url: Option<String>,
    pub username_element: Option<String>,
    pub username_value: Option<String>,
    pub password_element: Option<String>,
    pub password_value: Option<String>,
    pub submit_element: String,
    pub signon_realm: String,
    pub date_created: Option<DateTime<Utc>>,
    pub blacklisted_by_user: i32,
    pub scheme: i32,
    pub password_type: i32,
    pub times_used: i64,
    pub form_data: Option<Vec<u8>>,
    pub display_name: String,
    pub icon_url: String,
    pub federation_url: String,
    pub skip_zero_click: i32,
    pub generation_upload_status: i32,
    pub possible_username_pairs: Option<Vec<u8>>,
    pub id: i32,
    pub date_last_used: Option<DateTime<Utc>>,
    // pub moving_blocked_for:             Option<Vec<u8>>,
    pub date_password_modified: Option<DateTime<Utc>>,
    // pub sender_email:                   String,
    // pub sender_name:                    String,
    // NOTE: I'm not sure what it do
    // pub date_received:                  i64,
    // pub sharing_notification_displayed: i32,
    // pub keychain_identifier:            Vec<u8>,
    // pub sender_profile_image_url:       Option<String>,
}

impl LoginData {
    pub fn to_csv<D: Display>(&self, sep: D) -> String {
        format!(
            "{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}{sep}{}",
            self.origin_url,
            self.username_value
                .as_deref()
                .unwrap_or_default(),
            self.display_name,
            self.password_value
                .as_deref()
                .unwrap_or_default(),
            self.date_created
                .unwrap_or_default(),
            self.date_last_used
                .unwrap_or_default(),
            self.date_password_modified
                .unwrap_or_default(),
        )
    }
}

impl From<logins::Model> for LoginData {
    fn from(v: logins::Model) -> Self {
        Self {
            origin_url: v.origin_url,
            action_url: v.action_url,
            username_element: v.username_element,
            username_value: v.username_value,
            password_element: v.password_element,
            password_value: None,
            submit_element: v
                .submit_element
                .unwrap_or_default(),
            signon_realm: v.signon_realm,
            date_created: v
                .date_created
                .micros_to_chromium_utc(),
            blacklisted_by_user: v.blacklisted_by_user,
            scheme: v.scheme,
            password_type: v.password_type.unwrap_or_default(),
            times_used: v.times_used.unwrap_or_default(),
            form_data: v.form_data,
            display_name: v.display_name.unwrap_or_default(),
            icon_url: v.icon_url.unwrap_or_default(),
            federation_url: v
                .federation_url
                .unwrap_or_default(),
            skip_zero_click: v
                .skip_zero_click
                .unwrap_or_default(),
            generation_upload_status: v
                .generation_upload_status
                .unwrap_or_default(),
            possible_username_pairs: v.possible_username_pairs,
            id: v.id,
            date_last_used: v
                .date_last_used
                .micros_to_chromium_utc(),
            // moving_blocked_for:             v.moving_blocked_for,
            date_password_modified: v
                .date_password_modified
                .micros_to_chromium_utc(),
            // sender_email:                   v.sender_email.unwrap_or_default(),
            // sender_name:                    v.sender_name.unwrap_or_default(),
            // date_received:                  v.date_received.unwrap_or_default(),
            // sharing_notification_displayed: v.sharing_notification_displayed,
            // keychain_identifier:            v
            //     .keychain_identifier
            //     .unwrap_or_default(),
            // sender_profile_image_url:       v.sender_profile_image_url,
        }
    }
}
