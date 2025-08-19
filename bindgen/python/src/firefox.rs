use std::{fmt::Display, mem, path::PathBuf};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::{
    firefox::{GetCookies as _, MozCookie as MozCookieRs},
    prelude::{
        FirefoxBuilder as FirefoxBuilderRs, FirefoxCookieGetter as FirefoxCookieGetterRs,
        FirefoxGetter as FirefoxGetterRs, *,
    },
};
use pyo3::{
    exceptions::PyValueError, prelude::PyAnyMethods, pyclass, pymethods, Bound, PyResult, Python,
};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::SameSite;

macro_rules! firefoxs {
    ($($browser:ident),* $(,)?) => {
        pastey::paste! {
            $(
                #[gen_stub_pyclass]
                #[pyclass(frozen, str)]
                #[derive(Clone)]
                #[derive(Debug)]
                #[derive(Default)]
                pub struct [<$browser Getter>](FirefoxGetterRs<$browser>);

                impl Display for [<$browser Getter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[gen_stub_pymethods]
                #[pymethods]
                impl [<$browser Getter>] {
                    /// `base`: When Firefox data path changed
                    /// `profile`: When started with `-P <profile>`
                    /// `profile_path`: when browser started with `-profile <profile_path>`
                    ///
                    /// When set `profile_path` ignore other parameters like `base`, `profile`.
                    #[new]
                    #[pyo3(signature = (base=None, profile=None, profile_path=None))]
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[FirefoxGetter]", imports=("typing")))]
                    pub fn new(
                        py: Python<'_>,
                        base: Option<PathBuf>,
                        profile: Option<String>,
                        profile_path: Option<PathBuf>,
                    ) -> PyResult<Bound<'_, Self>> {
                        future_into_py(py, async move {
                            profile_path
                                .map_or_else(
                                    || {
                                        let mut ffb = FirefoxBuilderRs::new();
                                        if let Some(b) = base {
                                            ffb.base(b);
                                        }
                                        if let Some(p) = &profile {
                                            ffb.profile(p);
                                        }
                                        ffb.build()
                                    },
                                    |p| {
                                        let with_profile_path = FirefoxBuilderRs::<$browser>::with_profile_path(p);
                                        with_profile_path.build()
                                    },
                                )
                                .await
                                .map(Self)
                                .map_err(|e| PyValueError::new_err(e.to_string()))
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Return all cookies
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[MozCookie]]", imports=("typing")))]
                    pub fn cookies_all<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'_, Vec<MozCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_all()
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Filter by host
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[MozCookie]]", imports=("typing")))]
                    pub fn cookies_by_host<'a>(
                        &'a self,
                        py: Python<'a>,
                        host: String,
                    ) -> PyResult<Bound<'_, Vec<MozCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_by_host(host)
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
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
                pub struct [<$browser CookieGetter>](FirefoxCookieGetterRs<$browser>);

                impl Display for [<$browser CookieGetter>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                #[gen_stub_pymethods]
                #[pymethods]
                impl [<$browser CookieGetter>] {
                    /// `base`: When Firefox data path changed
                    /// `profile`: When started with `-P <profile>`
                    /// `profile_path`: when browser started with `-profile <profile_path>`
                    ///
                    /// When set `profile_path` ignore other parameters like `base`, `profile`.
                    #[new]
                    #[pyo3(signature = (base=None, profile=None, profile_path=None))]
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[FirefoxCookieGetter]", imports=("typing")))]
                    pub fn new(
                        py: Python<'_>,
                        base: Option<PathBuf>,
                        profile: Option<String>,
                        profile_path: Option<PathBuf>,
                    ) -> PyResult<Bound<'_, Self>> {
                        future_into_py(py, async move {
                            profile_path
                                .map_or_else(
                                    || {
                                        let mut ffb = FirefoxBuilderRs::new();
                                        if let Some(b) = base {
                                            ffb.base(b);
                                        }
                                        if let Some(p) = &profile {
                                            ffb.profile(p);
                                        }
                                        ffb.build_cookie()
                                    },
                                    |p| {
                                        let with_profile_path = FirefoxBuilderRs::<$browser>::with_profile_path(p);
                                        with_profile_path.build_cookie()
                                    },
                                )
                                .await
                                .map(Self)
                                .map_err(|e| PyValueError::new_err(e.to_string()))
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Return all cookies
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[MozCookie]]", imports=("typing")))]
                    pub fn cookies_all<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'_, Vec<MozCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_all()
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }

                    /// Filter by host
                    #[gen_stub(override_return_type(type_repr="typing.Awaitable[list[MozCookie]]", imports=("typing")))]
                    pub fn cookies_by_host<'a>(
                        &'a self,
                        py: Python<'a>,
                        host: String,
                    ) -> PyResult<Bound<'_, Vec<MozCookie>>> {
                        let self_ = self.clone();
                        future_into_py(py, async move {
                            let all = self_
                                .0
                                .cookies_by_host(host)
                                .await
                                .map_err(|e| PyValueError::new_err(e.to_string()))?;
                            #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
                            let all = unsafe { mem::transmute::<Vec<MozCookieRs>, Vec<MozCookie>>(all) };
                            Ok(all)
                        })
                        .map(|v| unsafe { v.downcast_into_unchecked() })
                    }
                }
            )*
        }
    };
}

firefoxs![Firefox, Librewolf, Floorp, Zen];

#[gen_stub_pyclass]
#[pyclass(get_all, set_all, eq, ord)]
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
