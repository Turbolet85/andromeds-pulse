use std::path::Path;

/// Returns true if a `.andromeda/` directory exists at `root`.
/// Existence-only check — content NEVER read per security plan §Logging
/// hygiene (workspace marker file content can be user-controlled and may
/// contain incidentally captured secrets).
pub(crate) fn check_andromeda_marker(root: &Path) -> bool {
    root.join(".andromeda").is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn returns_false_when_no_marker() {
        let tmp = TempDir::new().unwrap();
        assert!(!check_andromeda_marker(tmp.path()));
    }

    #[test]
    fn returns_true_when_marker_dir_exists() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".andromeda")).unwrap();
        assert!(check_andromeda_marker(tmp.path()));
    }

    #[test]
    fn returns_false_when_marker_is_a_file_not_dir() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".andromeda"), "not a dir").unwrap();
        assert!(!check_andromeda_marker(tmp.path()));
    }
}
