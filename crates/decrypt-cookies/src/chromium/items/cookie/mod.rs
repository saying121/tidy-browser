use self::entities::cookies;

pub mod dao;
pub mod entities;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct DecryptedCookies {
    pub creation_utc: i32,
    pub host_key: String,
    pub top_frame_site_key: String,
    pub name: String,
    pub value: String,
    pub encrypted_value: Option<String>,
    pub path: String,
    pub expires_utc: i32,
    pub is_secure: i32,
    pub is_httponly: i32,
    pub last_access_utc: i32,
    pub has_expires: i32,
    pub is_persistent: i32,
    pub priority: i32,
    pub samesite: i32,
    pub source_scheme: i32,
    pub source_port: i32,
    pub last_update_utc: i32,
}

impl DecryptedCookies {
    pub fn set_encrypted_value(&mut self, encrypted_value: String) {
        self.encrypted_value = Some(encrypted_value);
    }
}

impl From<cookies::Model> for DecryptedCookies {
    fn from(value: cookies::Model) -> Self {
        Self {
            creation_utc: value.creation_utc,
            host_key: value.host_key,
            top_frame_site_key: value.top_frame_site_key,
            name: value.name,
            value: value.value,
            encrypted_value: None,
            path: value.path,
            expires_utc: value.expires_utc,
            is_secure: value.is_secure,
            is_httponly: value.is_httponly,
            last_access_utc: value.last_access_utc,
            has_expires: value.has_expires,
            is_persistent: value.is_persistent,
            priority: value.priority,
            samesite: value.samesite,
            source_scheme: value.source_scheme,
            source_port: value.source_port,
            last_update_utc: value.last_update_utc,
        }
    }
}
