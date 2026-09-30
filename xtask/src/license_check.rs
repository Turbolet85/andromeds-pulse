use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const DUAL: &str = "MIT OR Apache-2.0";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest has no workspace parent")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

// TOML is read as text on purpose: the assertions are line-level and a parser
// would add a dependency to xtask for no extra enforcement.
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

#[test]
fn license_texts_present_at_root() {
    let mit = read("LICENSE-MIT");
    for needle in [
        "Copyright (c) 2026 Turbolet85",
        "Permission is hereby granted, free of charge",
    ] {
        assert!(mit.contains(needle), "LICENSE-MIT lacks {needle:?}");
    }
    let apache = read("LICENSE-APACHE");
    for needle in [
        "Apache License",
        "Version 2.0, January 2004",
        "TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION",
    ] {
        assert!(apache.contains(needle), "LICENSE-APACHE lacks {needle:?}");
    }
}

#[test]
fn license_workspace_value_is_dual_and_inherited() {
    let cargo_toml = read("Cargo.toml");
    let license_lines: Vec<&str> = section_lines(&cargo_toml, "[workspace.package]")
        .into_iter()
        .filter(|line| line.starts_with("license"))
        .collect();
    assert_eq!(license_lines, [format!("license = \"{DUAL}\"")]);

    let members = workspace_members(&cargo_toml);
    assert!(!members.is_empty(), "no [workspace] members parsed");
    for expected in ["pulse-app", "xtask"] {
        assert!(
            members.iter().any(|m| m == expected),
            "members {members:?} lack {expected}"
        );
    }
    for member in &members {
        let manifest = read(&format!("{member}/Cargo.toml"));
        let lines: Vec<&str> = manifest.lines().map(str::trim).collect();
        assert!(
            lines.contains(&"license.workspace = true"),
            "{member} does not inherit the workspace license"
        );
        assert!(
            !lines.iter().any(|line| line.starts_with("license =")),
            "{member} overrides the workspace license"
        );
    }
}

#[test]
fn license_npm_manifest_and_lock_root_agree() {
    let manifest: Value =
        serde_json::from_str(&read("pulse-app/ui/package.json")).expect("package.json parses");
    assert_eq!(manifest["license"], DUAL, "package.json license");
    let lock: Value = serde_json::from_str(&read("pulse-app/ui/package-lock.json"))
        .expect("package-lock.json parses");
    assert_eq!(
        lock["packages"][""]["license"], DUAL,
        "package-lock.json root entry license"
    );
}

#[test]
fn license_channel_manifests_carry_dual_license() {
    let workflow = read(".github/workflows/update-channels.yml");
    for needle in [
        r#"license any_of: ["MIT", "Apache-2.0"]"#,
        r#""license": "MIT|Apache-2.0""#,
    ] {
        assert!(
            workflow.contains(needle),
            "update-channels.yml lacks {needle:?}"
        );
    }
    for stale in [r#"license "MIT""#, r#""license": "MIT","#] {
        assert!(
            !workflow.contains(stale),
            "update-channels.yml still carries {stale:?}"
        );
    }
}
