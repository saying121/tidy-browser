use decrypt_cookies_rs::browser::cookies::SameSite as SameSiteRs;
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyclass_enum};

use self::{
    chromium::{ChromeGetter, ChromiumCookie},
    firefox::{FirefoxGetter, MozCookie},
};

mod chromium;
mod firefox;

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, ord)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SameSite {
    #[default]
    Non = 0,
    Lax = 1,
    Strict = 2,
}

impl From<SameSiteRs> for SameSite {
    fn from(value: SameSiteRs) -> Self {
        match value {
            SameSiteRs::None => Self::Non,
            SameSiteRs::Lax => Self::Lax,
            SameSiteRs::Strict => Self::Strict,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
pub fn decrypt_cookies_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SameSite>()?;

    m.add_class::<ChromeGetter>()?;
    m.add_class::<ChromiumCookie>()?;

    m.add_class::<FirefoxGetter>()?;
    m.add_class::<MozCookie>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
