use std::path::PathBuf;

use miette::Result;

use crate::{firefox::path::FFPath, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct LinuxFFBase {
    base: PathBuf,
}

impl FFPath for LinuxFFBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}

impl LinuxFFBase {
    const FF_BASE: &'static str = ".mozilla/firefox";
    const LIBREWOLF_BASE: &'static str = ".librewolf";

    pub async fn new(browser: Browser) -> Result<Self> {
        let init = dirs::home_dir().ok_or_else(|| miette::miette!("get home dir failed"))?;
        let base = match browser {
            Browser::Librewolf => Self::LIBREWOLF_BASE,
            _ => Self::FF_BASE,
        };
        let base = Self::helper(init, base).await?;

        Ok(Self { base })
    }
}
