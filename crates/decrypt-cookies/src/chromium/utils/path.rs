use std::path::PathBuf;

use crate::{Browser, BrowserFile};

/// just impl the `base` method
pub trait ChromiumPath {
    #[cfg(target_os = "windows")]
    const COOKIES: &'static str = r#"Network\Cookies"#; // sqlite3
    #[cfg(not(target_os = "windows"))]
    const COOKIES: &'static str = "Cookies"; // sqlite3
    const BOOKMARKS: &'static str = "Bookmarks"; // json
    const LOGINDATA: &'static str = "Login Data"; // sqlite3
    const HISTORY: &'static str = "History"; // sqlite3
    const LOCAL_STORAGE: &'static str = "Local Storage/leveldb";
    const EXTENSIONS: &'static str = "Extensions"; // a directory
    const SESSION_STORAGE: &'static str = "Session Storage"; // leveldb
    const CREDIT: &'static str = "Web Data"; // sqlite3
    const LOCAL_STATE: &'static str = "Local State"; // key, json

    fn base(&self) -> &PathBuf;

    // json, for windows
    fn key(&self) -> PathBuf {
        let mut path = self.base().clone();
        path.pop();
        path.push(Self::LOCAL_STATE);
        path
    }
    /// sqlite3
    fn credit(&self) -> PathBuf {
        self.base().join(Self::CREDIT)
    }
    /// leveldb
    fn session(&self) -> PathBuf {
        self.base()
            .join(Self::SESSION_STORAGE)
    }
    /// a directory
    fn extensions(&self) -> PathBuf {
        self.base().join(Self::EXTENSIONS)
    }
    /// sqlite3
    fn logindata(&self) -> PathBuf {
        self.base().join(Self::LOGINDATA)
    }
    /// leveldb
    fn storage(&self) -> PathBuf {
        self.base()
            .join(Self::LOCAL_STORAGE)
    }
    /// json
    fn bookmarks(&self) -> PathBuf {
        self.base().join(Self::BOOKMARKS)
    }
    /// sqlite3
    fn history(&self) -> PathBuf {
        self.base().join(Self::HISTORY)
    }
    /// sqlite3
    fn cookies(&self) -> PathBuf {
        self.base().join(Self::COOKIES)
    }
}

pub fn file_path(browser: Browser, file: BrowserFile) -> PathBuf {
    #[cfg(target_os = "linux")]
    let base = super::linux::path::LinuxChromiumBase::new(browser);
    #[cfg(target_os = "macos")]
    let base = super::macos::path::MacChromiumBase::new(browser);
    #[cfg(target_os = "windows")]
    let base = super::win::path::WinChromiumBase::new(browser);

    match file {
        BrowserFile::Cookies => base.cookies(),
        BrowserFile::Storage => base.storage(),
        BrowserFile::Passwd => base.logindata(),
        BrowserFile::Extensions => base.extensions(),
        BrowserFile::Bookmarks => base.bookmarks(),
        BrowserFile::Credit => base.credit(),
        BrowserFile::Session => base.session(),
        BrowserFile::History => base.history(),
        BrowserFile::Key => base.key(),
    }
}
