use decrypt_cookies_rs::browser::cookies::SameSite as SameSiteRs;
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyclass_enum};

use self::{chromium::*, firefox::*};

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

    macro_rules! browsers {
        ($($browser:ident),* $(,)?) => {
            pastey::paste! {
                $(
                    m.add_class::<[<$browser Getter>]>()?;
                    m.add_class::<[<$browser CookieGetter>]>()?;
                )*
            }
        };
    }

    browsers![
        Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera, Firefox, Librewolf, Floorp, Zen
    ];
    #[cfg(not(target_os = "linux"))]
    browsers![Arc, OperaGX, CocCoc];

    m.add_class::<ChromiumCookie>()?;
    m.add_class::<LoginData>()?;

    m.add_class::<MozCookie>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
