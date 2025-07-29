use std::path::Path;

pub fn need_sep(path: &Path) -> bool {
    let buf = path.as_os_str().as_encoded_bytes();
    buf.last()
        .is_some_and(|&c| char::from(c) != std::path::MAIN_SEPARATOR)
}

#[cfg(target_os = "windows")]
pub fn shadow_copy(from: &Path, to: &Path) -> crate::chromium::builder::Result<()> {
    // shadow copy `to` must is dir

    use snafu::ResultExt;

    use crate::chromium::builder::{IoSnafu, RawcopySnafu};
    if !to.is_dir() && to.exists() {
        std::fs::remove_file(to).context(IoSnafu { path: to.to_owned() })?;
    }

    let to = if to.is_dir() {
        to
    }
    else {
        to.parent()
            .expect("Get shadow copy dir failed")
    };
    rawcopy_rs_next::rawcopy(
        from.to_str()
            .expect("`from` path to str failed"),
        to.to_str()
            .expect("`to` path to str failed"),
    )
    .context(RawcopySnafu)?;

    Ok(())
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
