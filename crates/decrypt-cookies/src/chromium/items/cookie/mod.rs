use chrono::{prelude::*, LocalResult, TimeZone, Utc};

use self::entities::cookies;

pub mod dao;
pub mod entities;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecryptedCookies {
    pub creation_utc: LocalResult<DateTime<Utc>>,
    pub host_key: String,
    pub top_frame_site_key: String,
    pub name: String,
    pub value: String,
    pub decrypted_value: Option<String>,
    pub path: String,
    pub expires_utc: LocalResult<DateTime<Utc>>,
    pub is_secure: bool,
    pub is_httponly: bool,
    pub last_access_utc: LocalResult<DateTime<Utc>>,
    pub has_expires: bool,
    pub is_persistent: bool,
    pub priority: i32,
    pub samesite: i32,
    pub source_scheme: i32,
    pub source_port: i32,
    pub last_update_utc: LocalResult<DateTime<Utc>>,
}

impl DecryptedCookies {
    pub fn set_encrypted_value(&mut self, encrypted_value: String) {
        self.decrypted_value = Some(encrypted_value);
    }
}

trait I64ToDateTime {
    fn micros_to_chromium_utc(&self) -> LocalResult<DateTime<Utc>>;
}

// https://source.chromium.org/chromium/chromium/src/+/main:base/time/time.h;l=5;
impl I64ToDateTime for i64 {
    fn micros_to_chromium_utc(&self) -> LocalResult<DateTime<Utc>> {
        Utc.timestamp_micros(self - 11_644_473_600 * 1_000_000)
    }
}

impl From<cookies::Model> for DecryptedCookies {
    fn from(value: cookies::Model) -> Self {
        Self {
            creation_utc: value.creation_utc.micros_to_chromium_utc(),
            host_key: value.host_key,
            top_frame_site_key: value.top_frame_site_key,
            name: value.name,
            value: value.value,
            decrypted_value: None,
            path: value.path,
            expires_utc: value.expires_utc.micros_to_chromium_utc(),
            is_secure: value.is_secure != 0,
            is_httponly: value.is_httponly != 0,
            last_access_utc: value.last_access_utc.micros_to_chromium_utc(),
            has_expires: value.has_expires != 0,
            is_persistent: value.is_persistent != 0,
            priority: value.priority,
            samesite: value.samesite,
            source_scheme: value.source_scheme,
            source_port: value.source_port,
            last_update_utc: value.last_update_utc.micros_to_chromium_utc(),
        }
    }
}
