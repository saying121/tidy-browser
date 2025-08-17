use std::{fmt::Display, mem, path::PathBuf};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::{
    chromium::{
        ChromiumCookie as ChromiumCookieRs, GetCookies, GetLogins, LoginData as LoginDataRs,
    },
    prelude::{
        ChromiumCookieGetter as ChromiumCookieGetterRs, ChromiumGetter as ChromiumGetterRs, *,
    },
};
use pyo3::{exceptions::PyValueError, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::SameSite;

macro_rules! chromiums {
    ($($browser:ident),* $(,)?) => {
        pastey::paste! {
            $(
                #[gen_stub_pyclass]
                #[pyclass(frozen, str)]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser Getter>](ChromiumGetterRs<$browser>);

                impl Display for [<$browser Getter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[gen_stub_pymethods]
                #[pymethods]
                impl [<$browser Getter>] {
                    /// base: When browser start with `--user-data-dir=DIR` or special other channel
                    #[new]
                    #[pyo3(signature = (base=None))]
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[ChromiumGetter]", imports=("typing")))]
                    pub fn new(py: Python<'_>, base: Option<PathBuf>) -> PyResult<Bound<'_, Self>> {
                        let b = base.map_or_else(
                            ChromiumBuilder::<$browser>::new,
                            ChromiumBuilder::<$browser>::with_user_data_dir,
                        );
                        future_into_py(py, async move {
                            b.build()
                                .await
                                .map([<$browser Getter>])
                                .map_err(|e| PyValueError::new_err(e.to_string()))
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Return all cookies
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[ChromiumCookie]]", imports=("typing")))]
                    pub fn cookies_all<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'_, Vec<ChromiumCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_all()
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Filter by host
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[ChromiumCookie]]", imports=("typing")))]
                    pub fn cookies_by_host<'a>(
                        &'a self,
                        py: Python<'a>,
                        host: String,
                    ) -> PyResult<Bound<'_, Vec<ChromiumCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_by_host(host)
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Return all login data
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[LoginData]]", imports=("typing")))]
                    pub fn logins_all<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'_, Vec<LoginData>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .logins_all()
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Filter by host
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[LoginData]]", imports=("typing")))]
                    pub fn logins_by_host<'a>(
                        &'a self,
                        py: Python<'a>,
                        host: String,
                    ) -> PyResult<Bound<'_, Vec<LoginData>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .logins_by_host(host)
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<LoginDataRs>, Vec<LoginData>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }
                }

                #[gen_stub_pyclass]
                #[pyclass(frozen, str)]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser CookieGetter>](ChromiumCookieGetterRs<$browser>);

                impl Display for [<$browser CookieGetter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[gen_stub_pymethods]
                #[pymethods]
                impl [<$browser CookieGetter>] {
                    /// base: When browser start with `--user-data-dir=DIR` or special other channel
                    #[new]
                    #[pyo3(signature = (base=None))]
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[ChromiumCookieGetter]", imports=("typing")))]
                    pub fn new(py: Python<'_>, base: Option<PathBuf>) -> PyResult<Bound<'_, Self>> {
                        let b = base.map_or_else(
                            ChromiumBuilder::<$browser>::new,
                            ChromiumBuilder::<$browser>::with_user_data_dir,
                        );
                        future_into_py(py, async move {
                            b.build_cookie()
                                .await
                                .map([<$browser CookieGetter>])
                                .map_err(|e| PyValueError::new_err(e.to_string()))
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Return all cookies
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[ChromiumCookie]]", imports=("typing")))]
                    pub fn cookies_all<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'_, Vec<ChromiumCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_all()
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Filter by host
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[ChromiumCookie]]", imports=("typing")))]
                    pub fn cookies_by_host<'a>(
                        &'a self,
                        py: Python<'a>,
                        host: String,
                    ) -> PyResult<Bound<'_, Vec<ChromiumCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_by_host(host)
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<ChromiumCookieRs>, Vec<ChromiumCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }
                }
            )*
        }
    };
}

chromiums![Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera];
#[cfg(not(target_os = "linux"))]
chromiums![Arc, OperaGX, CocCoc];

#[gen_stub_pyclass]
#[pyclass(get_all, set_all, eq, ord)]
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

#[gen_stub_pyclass]
#[pyclass(get_all, set_all, eq, ord)]
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
