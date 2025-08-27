#![expect(clippy::trailing_empty_array, reason = "bindgen code")]

use std::{fmt::Display, mem, path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::{
    firefox::{GetCookies, MozCookie as MozCookieRs},
    prelude::{
        FirefoxBuilder as FirefoxBuilderRs, FirefoxCookieGetter as FirefoxCookieGetterRs,
        FirefoxGetter as FirefoxGetterRs, *,
    },
};
use napi_derive::napi;

use crate::SameSite;

macro_rules! firefoxs {
    ($($browser:ident),* $(,)?) => {
        pastey::paste! {
            $(
                #[napi]
                #[derive(Clone)] #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser Getter>](FirefoxGetterRs<$browser>);

                #[napi]
                impl Display for [<$browser Getter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[napi]
                impl [<$browser Getter>] {
                    #[napi(factory)]
                    pub async fn new(
                        base: Option<String>,
                        profile: Option<String>,
                        profile_path: Option<String>,
                    ) -> napi::Result<Self> {
                        profile_path
                            .map_or_else(
                                || {
                                    let mut ffb = FirefoxBuilderRs::new();
                                    if let Some(b) = base {
                                        let Ok(b) = PathBuf::from_str(&b);
                                        ffb.base(b);
                                    }
                                    if let Some(p) = &profile {
                                        ffb.profile(p);
                                    }
                                    ffb.build()
                                },
                                |p| {
                                    let Ok(p) = PathBuf::from_str(&p);
                                    let with_profile_path = FirefoxBuilderRs::<$browser>::with_profile_path(p);
                                    with_profile_path.build()
                                },
                            )
                            .await
                            .map(Self)
                            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
                    }

                    #[napi]
                    pub async fn cookies_all(&self) -> napi::Result<Vec<MozCookie>> {
                        let all = self
                            .0
                            .cookies_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn cookies_by_host(&self, host: String) -> napi::Result<Vec<MozCookie>> {
                        let all = self
                            .0
                            .cookies_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                        Ok(all)
                    }
                }

                #[napi]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser CookieGetter>](FirefoxCookieGetterRs<$browser>);

                impl Display for [<$browser CookieGetter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[napi]
                impl [<$browser CookieGetter>] {
                    #[napi(factory)]
                    pub async fn new(
                        base: Option<String>,
                        profile: Option<String>,
                        profile_path: Option<String>,
                    ) -> napi::Result<Self> {
                        profile_path
                            .map_or_else(
                                || {
                                    let mut ffb = FirefoxBuilderRs::new();
                                    if let Some(b) = base {
                                        let Ok(b) = PathBuf::from_str(&b);
                                        ffb.base(b);
                                    }
                                    if let Some(p) = &profile {
                                        ffb.profile(p);
                                    }
                                    ffb.build_cookie()
                                },
                                |p| {
                                    let Ok(p) = PathBuf::from_str(&p);
                                    let with_profile_path = FirefoxBuilderRs::<$browser>::with_profile_path(p);
                                    with_profile_path.build_cookie()
                                },
                            )
                            .await
                            .map(Self)
                            .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e.to_string()))
                    }

                    #[napi]
                    pub async fn cookies_all(&self) -> napi::Result<Vec<MozCookie>> {
                        let all = self
                            .0
                            .cookies_all()
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                        Ok(all)
                    }

                    #[napi]
                    pub async fn cookies_by_host(&self, host: String) -> napi::Result<Vec<MozCookie>> {
                        let all = self
                            .0
                            .cookies_by_host(host)
                            .await
                            .map_err(|e| napi::Error::new(napi::Status::ObjectExpected, e.to_string()))?;

                        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                        let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                        Ok(all)
                    }
                }
            )*
        }
    };
}

firefoxs![Firefox, Librewolf, Floorp, Zen];

#[napi(object)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct MozCookie {
    pub id: i32,
    pub origin_attributes: String,
    pub name: String,
    pub value: String,
    pub host: String,
    pub path: String,
    pub expiry: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub creation_time: Option<DateTime<Utc>>,
    pub is_secure: bool,
    pub is_http_only: bool,
    pub in_browser_element: i32,
    pub same_site: SameSite,
    pub scheme_map: i32,
}
