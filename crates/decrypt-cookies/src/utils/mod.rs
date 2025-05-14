use std::path::Path;

pub mod binary_cookies;

pub fn need_sep(path: &Path) -> bool {
    let buf = path.as_os_str().as_encoded_bytes();
    buf.last()
        .is_some_and(|&c| char::from(c) != std::path::MAIN_SEPARATOR)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;

    #[test]
    fn need_sep_test() {
        let Ok(path) = PathBuf::from_str("/abc/");
        assert!(!need_sep(&path));

        let Ok(path) = PathBuf::from_str("/abc");
        assert!(need_sep(&path));

        let Ok(path) = PathBuf::from_str("/a/b/c");
        assert!(need_sep(&path));

        let Ok(path) = PathBuf::from_str("/a/b/c/");
        assert!(!need_sep(&path));
    }
}
