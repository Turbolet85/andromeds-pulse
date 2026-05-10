use std::fs;
use std::path::Path;

use crate::contract::{VcsMetadata, VcsType};

/// Detects VCS metadata for `root` by walking up the directory tree
/// looking for `.git/HEAD`. Returns None if no VCS marker found within
/// 32 ancestors (sanity bound to avoid pathological filesystem walks).
///
/// Reads `.git/HEAD` text only to extract a short commit SHA basename
/// (8-character prefix). NEVER reads commit message body, NEVER follows
/// pack-refs across files (per Open Q5 resolution: filesystem-only,
/// `gix` upgrade deferred).
pub(crate) fn detect_vcs(root: &Path) -> Option<VcsMetadata> {
    const MAX_ANCESTORS: usize = 32;

    for ancestor in root.ancestors().take(MAX_ANCESTORS) {
        let git_dir = ancestor.join(".git");
        if !git_dir.is_dir() {
            continue;
        }
        let head_path = git_dir.join("HEAD");
        let head_commit_basename = read_head_commit_basename(&git_dir, &head_path);
        return Some(VcsMetadata {
            vcs_type: VcsType::Git,
            vcs_root: ancestor.to_path_buf(),
            head_commit_basename,
        });
    }
    None
}

fn read_head_commit_basename(git_dir: &Path, head_path: &Path) -> Option<String> {
    let head_text = fs::read_to_string(head_path).ok()?;
    let trimmed = head_text.trim();

    // Detached HEAD: HEAD contains the SHA directly.
    if let Some(basename) = sha_basename(trimmed) {
        return Some(basename);
    }

    // Symbolic ref: "ref: refs/heads/main" → resolve loose ref file.
    let target = trimmed.strip_prefix("ref: ")?;
    let ref_path = git_dir.join(target);
    let ref_text = fs::read_to_string(&ref_path).ok()?;
    sha_basename(ref_text.trim())
}

fn sha_basename(s: &str) -> Option<String> {
    if s.len() < 8 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(s[..8].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn returns_none_when_no_git_dir() {
        let tmp = TempDir::new().unwrap();
        assert!(detect_vcs(tmp.path()).is_none());
    }

    #[test]
    fn returns_metadata_with_short_sha_for_loose_ref() {
        let tmp = TempDir::new().unwrap();
        let git = tmp.path().join(".git");
        fs::create_dir_all(git.join("refs/heads")).unwrap();
        fs::write(git.join("HEAD"), "ref: refs/heads/main\n").unwrap();
        fs::write(
            git.join("refs/heads/main"),
            "abcdef1234567890abcdef1234567890abcdef12\n",
        )
        .unwrap();

        let metadata = detect_vcs(tmp.path()).expect("vcs detected");
        assert_eq!(metadata.vcs_type, VcsType::Git);
        assert_eq!(metadata.vcs_root, tmp.path());
        assert_eq!(metadata.head_commit_basename, Some("abcdef12".to_string()));
    }

    #[test]
    fn returns_metadata_with_short_sha_for_detached_head() {
        let tmp = TempDir::new().unwrap();
        let git = tmp.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(
            git.join("HEAD"),
            "1234567890abcdef1234567890abcdef12345678\n",
        )
        .unwrap();

        let metadata = detect_vcs(tmp.path()).expect("vcs detected");
        assert_eq!(metadata.head_commit_basename, Some("12345678".to_string()));
    }

    #[test]
    fn returns_metadata_without_basename_when_ref_unresolvable() {
        let tmp = TempDir::new().unwrap();
        let git = tmp.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(git.join("HEAD"), "ref: refs/heads/missing\n").unwrap();

        let metadata = detect_vcs(tmp.path()).expect("vcs detected");
        assert_eq!(metadata.head_commit_basename, None);
    }

    #[test]
    fn walks_up_from_subdirectory() {
        let tmp = TempDir::new().unwrap();
        let git = tmp.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(
            git.join("HEAD"),
            "1234567890abcdef1234567890abcdef12345678\n",
        )
        .unwrap();
        let nested = tmp.path().join("crates").join("inner");
        fs::create_dir_all(&nested).unwrap();

        let metadata = detect_vcs(&nested).expect("vcs detected from subdir");
        assert_eq!(metadata.vcs_root, tmp.path());
    }
}
