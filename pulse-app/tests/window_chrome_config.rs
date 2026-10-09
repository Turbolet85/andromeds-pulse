// Static guards on the window-shell config (chunk 2026-06-29-window-geometry-
// movable-shell, P-061): the capability JSON must grant the window-mutation
// permissions a frameless custom-titlebar app needs (Tauri 2 core:default
// omits them, silently rejecting data-tauri-drag-region / minimize /
// toggle-maximize), and the main dashboard must open centered.

use std::path::PathBuf;

use serde_json::Value;

fn manifest_json(rel: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

#[test]
fn default_capability_grants_window_drag_and_controls() {
    let cap = manifest_json("capabilities/default.json");
    let perms = cap["permissions"].as_array().expect("permissions array");
    let has = |p: &str| perms.iter().any(|v| v.as_str() == Some(p));
    assert!(
        has("core:window:allow-start-dragging"),
        "missing allow-start-dragging -> data-tauri-drag-region is silently rejected (F1 movability)"
    );
    assert!(has("core:window:allow-minimize"), "missing allow-minimize");
    assert!(
        has("core:window:allow-toggle-maximize"),
        "missing allow-toggle-maximize"
    );
}

#[test]
fn main_window_opens_centered() {
    let conf = manifest_json("tauri.conf.json");
    let windows = conf["app"]["windows"].as_array().expect("windows array");
    let main = windows
        .iter()
        .find(|w| w["label"].as_str() == Some("main"))
        .expect("main window definition");
    assert_eq!(
        main["center"].as_bool(),
        Some(true),
        "main dashboard must open centered (intent F1)"
    );
}
