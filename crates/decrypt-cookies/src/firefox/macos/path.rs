use std::path::PathBuf;

use miette::Result;

use crate::{firefox::path::FFPath, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct MacFFBase {
    pub base: PathBuf,
}

impl FFPath for MacFFBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}

impl MacFFBase {
    const FIREFOX_BASE: &'static str = "Firefox";
    const LIBREWOLF_BASE: &'static str = "librewolf";

    pub async fn new(browser: Browser) -> Result<Self> {
        let init = dirs::config_local_dir()
            .ok_or_else(|| miette::miette!("get config local dir failed"))?;
        let base = match browser {
            Browser::Librewolf => Self::LIBREWOLF_BASE,
            _ => Self::FIREFOX_BASE,
        };
        let base = Self::helper(init, base).await?;

        Ok(Self { base })
    }
}
