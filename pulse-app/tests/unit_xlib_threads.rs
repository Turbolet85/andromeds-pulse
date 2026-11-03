//! Xlib's thread support at the head of `main` (`pulse_app::xlib_threads`).
//!
//! The call's arms are pinned through the hidden entry that takes the
//! library name. Its place is read from `src/main.rs` itself, since it is
//! sound only before any thread exists and before anything reaches Xlib.

use std::path::Path;

use pulse_app::xlib_threads::{XLIB_LIBRARY, XlibThreads, init_from};

const POSTURE_STATEMENT: &str = "let render_posture = render_posture::apply_linux_default();";
const INIT_STATEMENT: &str = "xlib_threads::init();";
const RUNTIME_BUILD: &str = "tokio::runtime::Builder";

#[test]
fn the_product_library_is_the_xlib_soname() {
    assert_eq!(XLIB_LIBRARY, c"libX11.so.6");
}

// A host without the library fails this pin; it never skips it.
#[cfg(target_os = "linux")]
#[test]
fn the_found_library_initialises_and_a_second_call_does_too() {
    assert_eq!(init_from(XLIB_LIBRARY), XlibThreads::Initialised);
    assert_eq!(init_from(XLIB_LIBRARY), XlibThreads::Initialised);
}

#[cfg(target_os = "linux")]
#[test]
fn a_library_that_is_not_there_is_inert() {
    assert_eq!(
        init_from(c"libandromeda-pulse-no-such-library.so.0"),
        XlibThreads::LibraryNotFound
    );
}

#[cfg(target_os = "linux")]
#[test]
fn a_library_without_the_symbol_is_inert() {
    assert_eq!(init_from(c"libc.so.6"), XlibThreads::SymbolNotFound);
}

#[cfg(not(target_os = "linux"))]
#[test]
fn other_platforms_are_not_applicable() {
    assert_eq!(init_from(XLIB_LIBRARY), XlibThreads::NotApplicable);
}

fn src_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The lines of `fn main()`'s body that are neither blank nor a comment.
fn main_body_lines() -> Vec<String> {
    let source = std::fs::read_to_string(src_dir().join("main.rs")).expect("read src/main.rs");
    let lines: Vec<&str> = source.lines().collect();
    let opening = lines
        .iter()
        .position(|line| *line == "fn main() {")
        .expect("`fn main() {` on its own line");
    lines[opening + 1..]
        .iter()
        .take_while(|line| **line != "}")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(str::to_string)
        .collect()
}

#[test]
fn main_calls_it_second_after_the_render_posture_step_and_before_the_runtime() {
    let body = main_body_lines();
    assert_eq!(body[0], POSTURE_STATEMENT, "the first statement of main");
    assert_eq!(body[1], INIT_STATEMENT, "the second statement of main");
    let runtime = body
        .iter()
        .position(|line| line.contains(RUNTIME_BUILD))
        .expect("main builds the tokio runtime");
    assert!(runtime > 1, "the runtime is built after both statements");
}

#[test]
fn the_product_entry_has_one_call_site_outside_its_module() {
    let mut call_sites = Vec::new();
    for entry in std::fs::read_dir(src_dir()).expect("read pulse-app/src") {
        let path = entry.expect("dir entry").path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        if !name.ends_with(".rs") || name == "xlib_threads.rs" {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("read src file");
        assert!(
            !source.contains("init_from"),
            "{name} reaches the test entry"
        );
        for _ in source.matches("xlib_threads::init(") {
            call_sites.push(name.clone());
        }
    }
    assert_eq!(call_sites, ["main.rs"]);
}
