//! The workspace's declared Rust floor equals the pinned toolchain channel at
//! major.minor, and no member declares a floor of its own.
//!
//! A declared floor equal to the pin is proven by every pinned-toolchain build,
//! covering syntax, std APIs and the dependency graph's own `rust-version`
//! declarations alike. A lower declared floor would need a witness that builds
//! with that older toolchain, and clippy's MSRV check covers APIs only.

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest has no workspace parent")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

// TOML is read as text, as in `license_check`: the assertions are line-level.
fn section_lines<'a>(text: &'a str, header: &str) -> Vec<&'a str> {
    text.lines()
        .map(str::trim)
        .skip_while(|line| *line != header)
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .collect()
}

fn workspace_members(cargo_toml: &str) -> Vec<String> {
    let workspace = section_lines(cargo_toml, "[workspace]").join("\n");
    let Some(start) = workspace.find("members") else {
        return Vec::new();
    };
    let rest = &workspace[start..];
    let (Some(open), Some(close)) = (rest.find('['), rest.find(']')) else {
        return Vec::new();
    };
    rest[open + 1..close]
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

fn major_minor(version: &str) -> Option<(u32, u32)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    Some((major, minor))
}

#[test]
fn declared_floor_equals_the_pinned_channel() {
    let cargo_toml = read("Cargo.toml");
    let declared: Vec<&str> = section_lines(&cargo_toml, "[workspace.package]")
        .into_iter()
        .filter_map(|line| line.strip_prefix("rust-version"))
        .filter_map(|rest| rest.trim_start().strip_prefix('='))
        .map(|value| value.trim().trim_matches('"'))
        .collect();
    let [declared] = declared.as_slice() else {
        panic!("[workspace.package] must declare rust-version exactly once, found {declared:?}");
    };

    let channel = crate::pre_push::rust_channel(&read("rust-toolchain.toml"))
        .expect("rust-toolchain.toml declares a [toolchain] channel");
    assert_eq!(
        major_minor(declared),
        major_minor(&channel),
        "declared rust-version {declared:?} vs pinned channel {channel:?} (major.minor)"
    );
    assert!(
        major_minor(declared).is_some(),
        "declared rust-version {declared:?} is not major.minor"
    );

    let members = workspace_members(&cargo_toml);
    assert!(!members.is_empty(), "no [workspace] members parsed");
    for member in &members {
        let manifest = read(&format!("{member}/Cargo.toml"));
        let lines: Vec<&str> = manifest.lines().map(str::trim).collect();
        assert!(
            lines.contains(&"rust-version.workspace = true"),
            "{member} does not inherit the workspace rust-version"
        );
        assert!(
            !lines
                .iter()
                .any(|line| line.starts_with("rust-version")
                    && *line != "rust-version.workspace = true"),
            "{member} declares its own rust-version"
        );
    }
}
