use std::path::PathBuf;

use super::super::path::ChromiumPath;
use crate::Browser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct WinChromiumBase {
    base: PathBuf,
}

impl WinChromiumBase {
    const EDGE_WIN: &'static str = r#"Microsoft\Edge\User Data\Default"#;
    const CHROME_WIN: &'static str = r#"Google\Chrome\User Data\Default"#;

    pub fn new(browser: Browser) -> Self {
        let mut cookie_dir = dirs::data_local_dir().expect("get config dir failed");
        let v = match browser {
            Browser::Edge => Self::EDGE_WIN,
            _ => Self::CHROME_WIN,
        };
        cookie_dir.push(v);
        Self { base: cookie_dir }
    }
}

impl ChromiumPath for WinChromiumBase {
    fn base(&self) -> &PathBuf {
        &self.base
    }
}
