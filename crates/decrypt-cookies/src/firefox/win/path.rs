use std::path::PathBuf;

use crate::{firefox::path::FFPath, Browser};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct WinFFBase {
    base: PathBuf,
}

impl WinFFBase {
    const FIREFOX_BASE: &'static str = r"Mozilla\Firefox";
    const LIBREWOLF_BASE: &'static str = "librewolf";

    pub async fn new(browser: Browser) -> miette::Result<Self> {
        let base = match browser {
            Browser::Librewolf => Self::LIBREWOLF_BASE,
            _ => Self::FIREFOX_BASE,
        };
        let init =
            dirs::data_dir().ok_or_else(|| miette::miette!("get data local dir failed"))?;
        let base = Self::helper(init, base).await?;

        Ok(Self { base })
    }
}

impl FFPath for WinFFBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}
