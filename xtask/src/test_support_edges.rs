//! The `test-support` member is reachable through `[dev-dependencies]` alone,
//! and it depends on nothing but the crate whose types it builds.
//!
//! A dev-only edge to a test-support member is the one edge between workspace
//! members that the dependency-direction rule allows beside the declared ones.
//! These pins redden when a member names `test-support` anywhere else, or when
//! `test-support` gains a dependency of its own.

use std::fs;
use std::path::{Path, PathBuf};

const MEMBER: &str = "crates/test-support";
const NAME: &str = "test-support";
const DEV_TABLE: &str = "[dev-dependencies]";

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

fn table_headers(manifest: &str) -> Vec<&str> {
    manifest
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('['))
        .collect()
}

/// Every line of `manifest` that names the member, with the table it stands
/// in. A table header that names it is listed under itself. Comment lines are
/// not read.
fn lines_naming_the_member(manifest: &str) -> Vec<(String, String)> {
    let mut table = String::new();
    let mut found = Vec::new();
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            table = line.to_string();
        }
        if line.contains(NAME) {
            found.push((table.clone(), line.to_string()));
        }
    }
    found
}

/// The dependency names a table declares, one per `name = …` or `name.key = …`
/// line.
fn declared_names(manifest: &str, header: &str) -> Vec<String> {
    section_lines(manifest, header)
        .into_iter()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split(['=', '.', ' ']).next())
        .map(str::to_string)
        .collect()
}

fn other_members() -> Vec<String> {
    let members = workspace_members(&read("Cargo.toml"));
    assert!(!members.is_empty(), "no [workspace] members parsed");
    members.into_iter().filter(|m| m != MEMBER).collect()
}

#[test]
fn no_member_names_test_support_outside_its_dev_dependencies() {
    for member in other_members() {
        let manifest = read(&format!("{member}/Cargo.toml"));
        for (table, line) in lines_naming_the_member(&manifest) {
            assert_eq!(
                table, DEV_TABLE,
                "{member} names {NAME} under {table:?}: {line:?}"
            );
        }
    }
}

#[test]
fn test_support_depends_on_ingest_alone() {
    let manifest = read(&format!("{MEMBER}/Cargo.toml"));
    assert_eq!(
        declared_names(&manifest, "[dependencies]"),
        ["ingest"],
        "{MEMBER} depends on the crate whose types it builds, and on nothing else"
    );
    for header in table_headers(&manifest) {
        assert!(
            !header.contains("dependencies") || header == "[dependencies]",
            "{MEMBER} declares {header}"
        );
        assert!(!header.contains("features"), "{MEMBER} declares {header}");
    }
}

#[test]
fn test_support_is_a_member_that_some_member_uses() {
    assert!(
        workspace_members(&read("Cargo.toml")).contains(&MEMBER.to_string()),
        "the root member list does not hold {MEMBER}"
    );
    let users: Vec<String> = other_members()
        .into_iter()
        .filter(|member| {
            declared_names(&read(&format!("{member}/Cargo.toml")), DEV_TABLE)
                .iter()
                .any(|name| name == NAME)
        })
        .collect();
    assert!(
        !users.is_empty(),
        "no member names {NAME} under {DEV_TABLE}: the pins above would pass over a set nobody uses"
    );
}

#[test]
fn a_line_naming_the_member_is_listed_with_its_table() {
    let manifest = "[package]\nname = \"x\"\n# test-support in a comment\n[dependencies]\n\
                    test-support = { path = \"../test-support\" }\n[dev-dependencies]\n\
                    test-support = { path = \"../test-support\" }\n[features]\n\
                    extra = [\"dep:test-support\"]\n[dependencies.test-support]\npath = \"x\"\n";
    let tables: Vec<String> = lines_naming_the_member(manifest)
        .into_iter()
        .map(|(table, _)| table)
        .collect();
    assert_eq!(
        tables,
        [
            "[dependencies]",
            DEV_TABLE,
            "[features]",
            "[dependencies.test-support]"
        ]
    );
}

#[test]
fn declared_names_reads_both_spellings_of_a_dependency_line() {
    let manifest = "[dependencies]\n# a comment\ningest = { path = \"../ingest\" }\n\n\
                    tokio.workspace = true\n[dev-dependencies]\nrstest = \"1\"\n";
    assert_eq!(
        declared_names(manifest, "[dependencies]"),
        ["ingest", "tokio"]
    );
    assert_eq!(declared_names(manifest, DEV_TABLE), ["rstest"]);
}
