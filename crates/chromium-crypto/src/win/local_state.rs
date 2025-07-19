use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct LocalState {
    pub os_crypt: OsCrypt,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct OsCrypt {
    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=33
    // const K_OS_CRYPT_AUDIT_ENABLED_PREF_NAME: &[u8] = b"os_crypt.audit_enabled";
    /// Whether or not an attempt has been made to enable audit for the DPAPI
    /// encryption backing the random key.
    #[serde(default)]
    pub audit_enabled: bool,

    // https://source.chromium.org/chromium/chromium/src/+/main:components/os_crypt/sync/os_crypt_win.cc;l=29
    // const K_OS_CRYPT_ENCRYPTED_KEY_PREF_NAME: &[u8] = b"os_crypt.encrypted_key";
    /// Contains base64 random key encrypted with DPAPI.
    pub encrypted_key: String,
    /// option for prev chromium
    pub app_bound_encrypted_key: Option<String>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct YandexLocalState {
    #[serde(default)]
    pub audit_enabled: bool,
    pub os_crypt: YandexOsCrypt,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct YandexOsCrypt {
    pub checker_state: CheckerState,
    /// DPAPI
    #[serde(default)]
    pub encrypted_key: String,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct CheckerState {
    pub counter: i32,
    pub encrypted_data: String,
}
