use std::{fmt::Display, mem, path::PathBuf};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::{
    chromium::{ChromiumCookie as ChromiumCookieRs, GetCookies},
    prelude::{ChromiumGetter as ChromiumGetterRs, *},
};
use pyo3::{exceptions::PyValueError, prelude::*};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::SameSite;

#[gen_stub_pyclass]
#[pyclass(frozen, str)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ChromeGetter(ChromiumGetterRs<Chrome>);

#[gen_stub_pymethods]
#[pymethods]
impl ChromeGetter {
    #[new]
    #[pyo3(signature = (base=None))]
    #[gen_stub(override_return_type(type_repr="typing.Awaitable[ChromeGetter]", imports=("typing")))]
    pub fn new(py: Python<'_>, base: Option<PathBuf>) -> PyResult<Bound<'_, Self>> {
        let b = base.map_or_else(
            ChromiumBuilder::<Chrome>::new,
            ChromiumBuilder::<Chrome>::with_user_data_dir,
        );
        future_into_py(py, async move {
            b.build()
                .await
                .map(ChromeGetter)
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

impl Display for ChromeGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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
