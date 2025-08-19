use std::{fmt::Display, mem, path::PathBuf};

use chrono::{DateTime, Utc};
use decrypt_cookies_rs::prelude::{
    SafariBuilder, SafariCookie as SafariCookieRs, SafariGetter as SafariGetterRs,
};
use pyo3::{
    exceptions::PyValueError, prelude::PyAnyMethods as _, pyclass, pymethods, Bound, PyResult,
    Python,
};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::SameSite;

#[gen_stub_pyclass]
#[pyclass(frozen, str)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct SafariGetter(SafariGetterRs);

impl Display for SafariGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl SafariGetter {
    #[new]
    #[pyo3(signature = (cookies_path=None))]
    #[gen_stub(override_return_type(type_repr="typing.Awaitable[SafariGetter]", imports=("typing")))]
    pub fn new(py: Python<'_>, cookies_path: Option<PathBuf>) -> PyResult<Bound<'_, Self>> {
        future_into_py(py, async move {
            let mut b = SafariBuilder::new();
            if let Some(p) = cookies_path {
                b.cookies_path(p);
            }
            b.build()
                .await
                .map(SafariGetter)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
        .map(|v| unsafe { v.downcast_into_unchecked() })
    }

    pub fn cookies_all(&self) -> Vec<SafariCookie> {
        let all = self.0.cookies_all().to_vec();
        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
        unsafe {
            mem::transmute::<Vec<SafariCookieRs>, Vec<SafariCookie>>(all)
        }
    }

    pub fn cookies_by_host(&self, host: &str) -> Vec<SafariCookie> {
        let all = self
            .0
            .cookies_by_host(host)
            .cloned()
            .collect();
        #[expect(clippy::transmute_undefined_repr, reason = "already repr(C)")]
        unsafe {
            mem::transmute::<Vec<SafariCookieRs>, Vec<SafariCookie>>(all)
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(get_all, set_all, eq, ord)]
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
