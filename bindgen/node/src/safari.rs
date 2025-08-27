#![expect(
    clippy::trailing_empty_array,
    clippy::needless_pass_by_value,
    reason = "bindgen code"
)]

use std::{fmt::Display, mem, path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::prelude::{
    SafariBuilder, SafariCookie as SafariCookieRs, SafariGetter as SafariGetterRs,
};
use napi_derive::napi;

use crate::SameSite;

#[napi]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct SafariGetter(SafariGetterRs);

impl Display for SafariGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[napi]
impl SafariGetter {
    #[napi]
    pub async fn new(cookies_path: Option<String>) -> napi::Result<Self> {
        let mut b = SafariBuilder::new();
        if let Some(p) = cookies_path {
            let Ok(p) = PathBuf::from_str(&p);
            b.cookies_path(p);
        }
        b.build()
            .await
            .map(Self)
            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
    }

    #[napi]
    pub fn cookies_all(&self) -> Vec<SafariCookie> {
        let all = self.0.cookies_all().to_vec();
        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
        unsafe {
            mem::transmute::<Vec<SafariCookieRs>, Vec<SafariCookie>>(all)
        }
    }

    #[napi]
    pub fn cookies_by_host(&self, host: String) -> Vec<SafariCookie> {
        let all = self
            .0
            .cookies_by_host(&host)
            .cloned()
            .collect();
        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
        unsafe {
            mem::transmute::<Vec<SafariCookieRs>, Vec<SafariCookie>>(all)
        }
    }
}

#[napi(object)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct SafariCookie {
    pub version: u32,

    pub flags: u32,
    pub port: Option<u16>,
    pub comment: Option<String>,
    pub domain: String,
    pub name: String,
    pub path: String,
    pub value: String,

    pub expires: Option<DateTime<Utc>>,
    pub creation: Option<DateTime<Utc>>,
    pub same_site: SameSite,
    pub is_secure: bool,
    pub is_http_only: bool,
}
