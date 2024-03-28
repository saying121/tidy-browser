use std::path::PathBuf;

/// just impl the `base` method
pub trait ChromiumPath {
    const BOOKMARKS: &'static str = "Bookmarks"; // json
    const COOKIES: &'static str = "Cookies"; // sqlite3
                                             // const PROFILE_PICTURE: &'static str = "Edge Profile Picture.png";
    const EXTENSION_COOKIES: &'static str = "Extension Cookies";
    // const FAVICONS: &'static str = "Favicons"; // sqlite3
    const HISTORY: &'static str = "History"; // sqlite3
    const LOAD_STATISTICS: &'static str = "load_statistics.db"; // sqlite3
    const LOGIN_DATA: &'static str = "Login Data"; // sqlite3
    const MEDIA_DEVICE_SALTS: &'static str = "MediaDeviceSalts"; // sqlite3, https://source.chromium.org/chromium/chromium/src/+/main:components/media_device_salt/README.md
    const NETWORK_ACTION_PREDICTOR: &'static str = "Network Action Predictor"; // sqlite3
    const LOCAL_STORAGE: &'static str = "Local Storage/leveldb";
    const EXTENSIONS: &'static str = "Extensions"; // a directory
    const SESSION_STORAGE: &'static str = "Session Storage"; // leveldb
    /// The webdata component manages the "web database", a `SQLite` database stored in the user's profile containing various webpage-related metadata such as autofill and web search engine data.
    const WEB_DATA: &'static str = "Web Data"; // sqlite3, https://source.chromium.org/chromium/chromium/src/+/main:components/webdata/README.md
    /// This directory contains shared files for the implementation of the <chrome://local-state> `WebUI` page.
    const LOCAL_STATE: &'static str = "Local State"; // key, json, https://source.chromium.org/chromium/chromium/src/+/main:components/local_state/README.md

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
        self.base().join(Self::WEB_DATA)
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
        self.base().join(Self::LOGIN_DATA)
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
