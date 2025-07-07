use std::path::Path;

pub fn need_sep(path: &Path) -> bool {
    let buf = path.as_os_str().as_encoded_bytes();
    buf.last()
        .is_some_and(|&c| char::from(c) != std::path::MAIN_SEPARATOR)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;

    #[cfg(not(target_os = "windows"))]
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

    #[cfg(target_os = "windows")]
    #[test]
    fn need_sep_test() {
        let Ok(path) = PathBuf::from_str(r"\abc\");
        assert!(!need_sep(&path));

        let Ok(path) = PathBuf::from_str(r"\abc");
        assert!(need_sep(&path));

        let Ok(path) = PathBuf::from_str(r"\a\b\c");
        assert!(need_sep(&path));

        let Ok(path) = PathBuf::from_str(r"\a\b\c\");
        assert!(!need_sep(&path));
    }
}
