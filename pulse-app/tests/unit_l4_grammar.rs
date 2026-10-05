//! The committed L4 grammar is the b9305 converter's output for the shipped
//! schema. These tests PERFORM the conversion with the vendored converter
//! (`pulse-app/vendor/llama-cpp/json_schema_to_grammar.py`) rather than
//! compare hashes, so a schema edit without a regenerated grammar fails here.
//!
//! No Python 3 interpreter is a FAILURE, never a skip: a skip reads as a pass
//! in nextest's counts.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use interpretation::schema::L4_OUTPUT_JSON_SCHEMA;
use pulse_app::llamacli_inference::L4_OUTPUT_GBNF;

fn converter() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor/llama-cpp/json_schema_to_grammar.py")
}

/// The first of `python3`, `python` whose `--version` prints `Python 3`.
fn python3() -> String {
    for candidate in ["python3", "python"] {
        let Ok(out) = Command::new(candidate).arg("--version").output() else {
            continue;
        };
        let version = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        if out.status.success() && version.trim_start().starts_with("Python 3") {
            return candidate.to_string();
        }
    }
    panic!(
        "no Python 3 interpreter found (tried python3, python): the grammar equality cannot be checked"
    );
}

/// The converter's stdout for `schema`, fed on stdin, with CRLF normalised
/// to LF (Windows Python writes CRLF; the committed grammar is LF).
fn convert(schema: &str) -> String {
    let mut child = Command::new(python3())
        .arg(converter())
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the converter spawns");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(schema.as_bytes())
        .expect("the schema reaches the converter");
    let out = child.wait_with_output().expect("the converter exits");
    assert!(
        out.status.success(),
        "the converter failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("the converter prints UTF-8")
        .replace("\r\n", "\n")
}

#[test]
fn committed_grammar_is_the_converter_output_for_the_shipped_schema() {
    assert_eq!(convert(L4_OUTPUT_JSON_SCHEMA), L4_OUTPUT_GBNF);
}

#[test]
fn a_one_enum_value_schema_change_changes_the_converter_output() {
    assert_eq!(
        L4_OUTPUT_JSON_SCHEMA.matches("\"surface\"").count(),
        1,
        "the discriminating edit needs exactly one \"surface\" in the schema"
    );
    let edited = L4_OUTPUT_JSON_SCHEMA.replacen("\"surface\"", "\"surfaced\"", 1);
    assert_ne!(convert(&edited), L4_OUTPUT_GBNF);
}
