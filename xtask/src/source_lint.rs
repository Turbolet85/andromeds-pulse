//! English-only source gate. Russian prepositions and Cyrillic homoglyphs
//! (U+043A for "to", U+0430 for "a") had leaked into comments, error strings
//! and the L4 system prompts. Docs, rules and `.andromeda/` are out of scope:
//! they legitimately quote the operator in Russian.
//!
//! Every byte this verb prints is ASCII by construction: a Windows runner
//! pipes a step's stdout through the ANSI code page, so echoing a Cyrillic
//! line verbatim crashed the former inline python step with
//! `UnicodeEncodeError` and reported nothing readable. Non-ASCII characters
//! of a reported line are rendered as `\u{XXXX}`.
//!
//! Verdict arms (exit code in parentheses): `clean` (0) · `findings` (1) ·
//! `cannot-evaluate` (2 — a root absent or a file unreadable; never a pass).
//! One pretty-JSON verdict on stdout after the annotations, twinned at
//! `target/english-sources/report.json`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use serde_json::{Value, json};

pub(crate) const ROOTS: [&str; 5] = [
    "crates",
    "pulse-app/src",
    "pulse-app/tests",
    "pulse-app/ui/src",
    "xtask/src",
];
const EXTENSIONS: [&str; 3] = ["rs", "ts", "tsx"];
const CYRILLIC_FIRST: u32 = 0x0400;
const CYRILLIC_LAST: u32 = 0x04FF;
const LINE_CAP_CHARS: usize = 120;

pub(crate) struct Report {
    pub(crate) annotations: Vec<String>,
    pub(crate) payload: Value,
    pub(crate) exit: u8,
}

fn is_cyrillic(c: char) -> bool {
    (CYRILLIC_FIRST..=CYRILLIC_LAST).contains(&u32::from(c))
}

fn ascii_escaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_ascii() && !c.is_ascii_control() {
            out.push(c);
        } else {
            out.push_str(&format!("\\u{{{:04X}}}", u32::from(c)));
        }
    }
    out
}

// GitHub workflow-command data escaping: `%`, CR and LF in the message.
fn annotation_data(text: &str) -> String {
    text.replace('%', "%25")
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| EXTENSIONS.contains(&e))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn cannot_evaluate(detail: String) -> Report {
    Report {
        annotations: Vec::new(),
        payload: json!({
            "verdict": "cannot-evaluate",
            "hits": 0,
            "files_scanned": 0,
            "detail": ascii_escaped(&detail),
        }),
        exit: 2,
    }
}

pub(crate) fn evaluate(root: &Path) -> Report {
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for sub in ROOTS {
        let dir = root.join(sub);
        if !dir.is_dir() {
            return cannot_evaluate(format!("root {sub} is absent"));
        }
        let mut found = Vec::new();
        if let Err(err) = collect_files(&dir, &mut found) {
            return cannot_evaluate(format!("root {sub} is unreadable: {err}"));
        }
        for path in found {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            files.push((rel, path));
        }
    }
    files.sort();

    let mut annotations = Vec::new();
    for (rel, path) in &files {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return cannot_evaluate(format!("{rel} is unreadable: {err}")),
        };
        let text = String::from_utf8_lossy(&bytes);
        for (index, line) in text.lines().enumerate() {
            if !line.chars().any(is_cyrillic) {
                continue;
            }
            let n = index + 1;
            let shown: String = line.trim().chars().take(LINE_CAP_CHARS).collect();
            let rel_ascii = annotation_data(&ascii_escaped(rel));
            annotations.push(format!(
                "::error file={rel_ascii},line={n}::Cyrillic character in source - {rel_ascii}:{n}: {}",
                annotation_data(&ascii_escaped(&shown))
            ));
        }
    }

    let hits = annotations.len();
    let (verdict, exit) = if hits == 0 {
        ("clean", 0)
    } else {
        ("findings", 1)
    };
    Report {
        annotations,
        payload: json!({
            "verdict": verdict,
            "hits": hits,
            "files_scanned": files.len(),
        }),
        exit,
    }
}

pub(crate) fn render_stdout(report: &Report) -> Result<String> {
    let mut out = String::new();
    for line in &report.annotations {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(&serde_json::to_string_pretty(&report.payload)?);
    out.push('\n');
    Ok(out)
}

fn workspace_root() -> Result<PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf())
}

pub(crate) fn run() -> Result<ExitCode> {
    let root = workspace_root()?;
    let report = evaluate(&root);
    print!("{}", render_stdout(&report)?);
    let report_dir = root.join("target").join("english-sources");
    fs::create_dir_all(&report_dir).context("create english-sources report dir")?;
    fs::write(
        report_dir.join("report.json"),
        serde_json::to_string_pretty(&report.payload)?,
    )
    .context("write english-sources report")?;
    let verdict = report.payload["verdict"].as_str().unwrap_or("unknown");
    eprintln!("check:english-sources: {verdict} (exit {})", report.exit);
    if report.exit == 1 {
        eprintln!(
            "{} line(s) carry Cyrillic characters; sources are English-only.",
            report.annotations.len()
        );
    }
    Ok(ExitCode::from(report.exit))
}

#[cfg(test)]
mod tests {
    // andromeda:walks-tree
    use super::*;

    fn cyrillic(code: u32) -> char {
        char::from_u32(code).expect("valid scalar")
    }

    fn fixture_tree() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        for sub in ROOTS {
            fs::create_dir_all(dir.path().join(sub)).expect("root");
        }
        fs::write(dir.path().join("crates/clean.rs"), "fn main() {}\n").expect("write");
        dir
    }

    #[test]
    fn english_sources_hit_renders_file_line_and_is_ascii() {
        let dir = fixture_tree();
        let planted = format!("// line one\n// send {} the model\n", cyrillic(0x043A));
        fs::create_dir_all(dir.path().join("crates/a/src")).expect("dir");
        fs::write(dir.path().join("crates/a/src/lib.rs"), planted).expect("write");

        let report = evaluate(dir.path());
        assert_eq!(report.exit, 1);
        assert_eq!(report.payload["verdict"], "findings");
        assert_eq!(report.payload["hits"], 1);
        let stdout = render_stdout(&report).expect("render");
        assert!(stdout.is_ascii(), "stdout must be pure ASCII: {stdout}");
        assert!(
            stdout.contains(
                "::error file=crates/a/src/lib.rs,line=2::Cyrillic character in source - crates/a/src/lib.rs:2: // send \\u{043A} the model"
            ),
            "annotation must name file:line with the escaped codepoint: {stdout}"
        );
    }

    #[test]
    fn english_sources_clean_tree_exits_zero() {
        let dir = fixture_tree();
        let report = evaluate(dir.path());
        assert_eq!(report.exit, 0);
        assert_eq!(report.payload["verdict"], "clean");
        assert_eq!(report.payload["files_scanned"], 1);
        let stdout = render_stdout(&report).expect("render");
        assert!(stdout.contains("\"verdict\": \"clean\""));
    }

    #[test]
    fn english_sources_missing_root_exits_two() {
        let dir = fixture_tree();
        fs::remove_dir_all(dir.path().join("xtask/src")).expect("remove");
        let report = evaluate(dir.path());
        assert_eq!(report.exit, 2);
        assert_eq!(report.payload["verdict"], "cannot-evaluate");
    }

    #[test]
    fn english_sources_hit_outside_roots_is_ignored() {
        let dir = fixture_tree();
        fs::create_dir_all(dir.path().join("docs")).expect("dir");
        fs::write(
            dir.path().join("docs/notes.rs"),
            format!("// {}\n", cyrillic(0x0430)),
        )
        .expect("write");
        fs::create_dir_all(dir.path().join("pulse-app/scripts")).expect("dir");
        fs::write(
            dir.path().join("pulse-app/scripts/x.ts"),
            format!("// {}\n", cyrillic(0x0430)),
        )
        .expect("write");
        let report = evaluate(dir.path());
        assert_eq!(report.exit, 0);
        assert_eq!(report.payload["hits"], 0);
    }

    #[test]
    fn english_sources_md_under_root_is_ignored() {
        let dir = fixture_tree();
        fs::write(
            dir.path().join("crates/README.md"),
            format!("{}\n", cyrillic(0x0431)),
        )
        .expect("write");
        let report = evaluate(dir.path());
        assert_eq!(report.exit, 0);
        assert_eq!(report.payload["verdict"], "clean");
    }
}
