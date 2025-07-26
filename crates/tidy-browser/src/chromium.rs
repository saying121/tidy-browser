use std::path::PathBuf;

use decrypt_cookies::{chromium::ChromiumCookie, prelude::*};
use snafu::ResultExt;

use crate::{
    args::ChromiumName,
    error::{self, Result},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ChromiumBased {}

impl ChromiumBased {
    pub async fn cookies<D, H>(
        name: ChromiumName,
        data_dir: D,
        host: H,
    ) -> Result<Vec<ChromiumCookie>>
    where
        D: Into<Option<PathBuf>>,
        H: for<'a> Into<Option<&'a str>>,
    {
        let data_dir = data_dir.into();
        let host: Option<&str> = host.into();

        macro_rules! chromiums {
            ($($browser:ident,) *) => {
                match name {
                    $(
                    ChromiumName::$browser => {
                        let chromium = if let Some(dir) = data_dir {
                            ChromiumBuilder::<$browser>::with_user_data_dir(dir)
                        }
                        else {
                            ChromiumBuilder::new()
                        }
                        .build()
                        .await
                        .context(error::ChromiumBuilderSnafu)?;

                        let cookies = if let Some(host) = host {
                            chromium.cookies_by_host(host).await
                        } else {
                            chromium.all_cookies().await
                        }.context(error::ChromiumSnafu)?;

                        Ok(cookies)
                    },
                    )*
                }
            };
        }

        #[cfg(target_os = "linux")]
        {
            chromiums![Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera,]
        }
        #[cfg(not(target_os = "linux"))]
        {
            chromiums![Chrome, Edge, Chromium, Brave, Vivaldi, Yandex, Opera, Arc, OperaGX, CocCoc,]
        }
    }
}
