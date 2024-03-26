use std::path::PathBuf;

/// just impl the `base` method
pub trait ChromiumPath {
    #[cfg(target_os = "windows")]
    const COOKIES: &'static str = "Network/Cookies"; // sqlite3

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
