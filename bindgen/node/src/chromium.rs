#![expect(clippy::trailing_empty_array, reason = "bindgen code")]

use std::{fmt::Display, mem, path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::{
    chromium::{
        ChromiumCookie as ChromiumCookieRs, ChromiumCookieGetter as ChromiumCookieGetterRs,
        ChromiumLoginGetter as ChromiumLoginGetterRs, GetCookies, GetLogins,
        LoginData as LoginDataRs,
    },
    prelude::{ChromiumBuilder as ChromiumBuilderRs, ChromiumGetter as ChromiumGetterRs, *},
};
use napi_derive::napi;

use crate::SameSite;

macro_rules! chromiums {
    ($($browser:ident),* $(,)?) => {
        pastey::paste! {
            $(
                #[napi]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser Getter>](ChromiumGetterRs<$browser>);

                #[napi]
                impl Display for [<$browser Getter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[napi]
                impl [<$browser Getter>] {
                    #[napi(factory)]
                    pub async fn new(base: Option<String>) -> napi::Result<Self> {
                        let b = base
                            .map(|v| {
                                let Ok(p) = PathBuf::from_str(&v);
                                p
                            })
                        .map_or_else(
                            ChromiumBuilderRs::<$browser>::new,
                            ChromiumBuilderRs::<$browser>::with_user_data_dir,
                        );
                        b.build()
                            .await
                            .map(Self)
                            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
                    }

                    #[napi]
                    pub async fn cookies_all(&self) -> napi::Result<Vec<ChromiumCookie>> {
                        let all = self
                            .0
                            .cookies_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn cookies_by_host(&self, host: String) -> napi::Result<Vec<ChromiumCookie>> {
                        let all = self
                            .0
                            .cookies_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn logins_all(&self) -> napi::Result<Vec<LoginData>> {
                        let all = self
                            .0
                            .logins_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn logins_by_host(&self, host: String) -> napi::Result<Vec<LoginData>> {
                        let all = self
                            .0
                            .logins_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                        Ok(all)
                    }
                }

                #[napi]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser CookieGetter>](ChromiumCookieGetterRs<$browser>);

                impl Display for [<$browser CookieGetter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[napi]
                impl [<$browser CookieGetter>] {
                    #[napi(factory)]
                    pub async fn new(base: Option<String>) -> napi::Result<Self> {
                        let b = base
                            .map(|v| {
                                let Ok(p) = PathBuf::from_str(&v);
                                p
                            })
                        .map_or_else(
                            ChromiumBuilderRs::<$browser>::new,
                            ChromiumBuilderRs::<$browser>::with_user_data_dir,
                        );
                        b.build_cookie()
                            .await
                            .map(Self)
                            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
                    }

                    #[napi]
                    pub async fn cookies_all(&self) -> napi::Result<Vec<ChromiumCookie>> {
                        let all = self
                            .0
                            .cookies_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn cookies_by_host(&self, host: String) -> napi::Result<Vec<ChromiumCookie>> {
                        let all = self
                            .0
                            .cookies_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                        Ok(all)
                    }
                }

                #[napi]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser LoginGetter>](ChromiumLoginGetterRs<$browser>);

                impl Display for [<$browser LoginGetter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[napi]
                impl [<$browser LoginGetter>] {
                    #[napi(factory)]
                    pub async fn new(base: Option<String>) -> napi::Result<Self> {
                        let b = base
                            .map(|v| {
                                let Ok(p) = PathBuf::from_str(&v);
                                p
                            })
                        .map_or_else(
                            ChromiumBuilderRs::<$browser>::new,
                            ChromiumBuilderRs::<$browser>::with_user_data_dir,
                        );
                        b.build_login()
                            .await
                            .map(Self)
                            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
                    }

                    #[napi]
                    pub async fn logins_all(&self) -> napi::Result<Vec<LoginData>> {
                        let all = self
                            .0
                            .logins_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn logins_by_host(&self, host: String) -> napi::Result<Vec<LoginData>> {
                        let all = self
                            .0
                            .logins_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                        Ok(all)
                    }
                }
            )*
        }
    };
}

chromiums![Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera];
#[cfg(not(target_os = "linux"))]
chromiums![Arc, OperaGX, CocCoc];

#[napi(object)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct ChromiumCookie {
    pub creation_utc: Option<DateTime<Utc>>,
    pub host_key: String,
    pub top_frame_site_key: String,
    pub name: String,
    pub value: String,
    pub decrypted_value: Option<String>,
    pub path: String,
    pub expires_utc: Option<DateTime<Utc>>,
    pub is_secure: bool,
    pub is_httponly: bool,
    pub last_access_utc: Option<DateTime<Utc>>,
    pub has_expires: bool,
    pub is_persistent: bool,
    pub priority: i32,
    pub same_site: SameSite,
    pub source_scheme: i32,
    pub source_port: i32,
    pub last_update_utc: Option<DateTime<Utc>>,
}

#[napi(object)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
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
    pub date_password_modified: Option<DateTime<Utc>>,
}
