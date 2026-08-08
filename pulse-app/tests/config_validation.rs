//! Chunk #52 config-validation tests. Lightweight smoke checks asserting
//! the `tauri.conf.json` updater section + `capabilities/updater.json`
//! capability JSON contain the locked-by-chunk-#3-ACTIVE configuration
//! values that release.yml depends on. Full Minisign signature
//! verification (positive / negative / bypass-attempt) is deferred to a
//! follow-on testing chunk per /implement Phase 6 Open Question 3 = Path A.
//!
//! Path resolution uses env!("CARGO_MANIFEST_DIR") + relative paths per
//! testing.md Session Additions 2026-05-13 (chunk #50 cross-crate pattern,
//! adapted to same-crate file reads). Tests are pure file-read + JSON-
//! parse + value-assert; no Tauri runtime context needed.

use std::path::PathBuf;

use serde_json::Value;

fn pulse_app_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_json(relative_path: &str) -> Value {
    let full_path = pulse_app_root().join(relative_path);
    let content = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", full_path.display()));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("parse {} as JSON failed: {e}", full_path.display()))
}

#[test]
fn config_updater_pubkey_is_non_empty() {
    // Tauri updater Minisign Ed25519 public key MUST be baked into
    // tauri.conf.json plugins.updater.pubkey per security plan §Code-
    // signing key custody + chunk #3 ACTIVE scope. The private key
    // (consumed by release.yml via Environment secret MINISIGN_PRIVATE_KEY
    // per production-release-environment.md §DEFERRED scope item 3) never
    // enters the repo.
    let conf = read_json("tauri.conf.json");
    let pubkey = conf
        .get("plugins")
        .and_then(|v| v.get("updater"))
        .and_then(|v| v.get("pubkey"))
        .and_then(Value::as_str)
        .expect("tauri.conf.json plugins.updater.pubkey present");
    assert!(
        !pubkey.is_empty(),
        "updater pubkey must be non-empty (chunk #3 ACTIVE-scope keypair baked in)"
    );
    assert!(
        pubkey.starts_with("RW"),
        "Minisign Ed25519 public keys begin with 'RW' base64 prefix; got: {}",
        &pubkey[..pubkey.len().min(8)]
    );
}

#[test]
fn config_updater_endpoint_matches_github_releases_pattern() {
    // Per arch §Occupied Resources Updater channel: latest.json is
    // published to GitHub Releases under the canonical repository. The
    // tauri-plugin-updater endpoint MUST resolve there; chunk #52
    // release.yml emits the manifest via tauri-action's auto-upload.
    let conf = read_json("tauri.conf.json");
    let endpoints = conf
        .get("plugins")
        .and_then(|v| v.get("updater"))
        .and_then(|v| v.get("endpoints"))
        .and_then(Value::as_array)
        .expect("tauri.conf.json plugins.updater.endpoints array present");
    assert!(
        !endpoints.is_empty(),
        "updater endpoints array must be non-empty"
    );
    let primary = endpoints[0]
        .as_str()
        .expect("primary endpoint is a string URL");
    assert!(
        primary.starts_with("https://"),
        "updater endpoint MUST use HTTPS; got: {primary}"
    );
    assert!(
        primary.contains("github.com/") && primary.contains("/releases/"),
        "updater endpoint MUST target github.com/.../releases/...; got: {primary}"
    );
    assert!(
        primary.contains("latest.json"),
        "updater endpoint MUST reference latest.json filename; got: {primary}"
    );
}

#[test]
fn config_updater_dialog_is_false() {
    // Per arch §Cross-cutting Patterns Webview IPC capability policy:
    // pulse:updater MUST NOT be exposed to webview JavaScript — bound to
    // the tauri-plugin-updater flow that consumes latest.json. Setting
    // `dialog: false` disables the plugin's built-in confirmation dialog
    // surface; future webview UI for "update available" notifications
    // goes through OS notification (tauri-plugin-notification) NOT
    // updater dialog (which would require widening pulse:updater capability
    // to webview windows).
    let conf = read_json("tauri.conf.json");
    let dialog = conf
        .get("plugins")
        .and_then(|v| v.get("updater"))
        .and_then(|v| v.get("dialog"))
        .and_then(Value::as_bool)
        .expect("tauri.conf.json plugins.updater.dialog field present and boolean");
    assert!(
        !dialog,
        "updater dialog flag MUST be false (webview-IPC capability policy: \
         updater surface not webview-facing)"
    );
}

#[test]
fn capability_updater_permissions_scoped_to_updater_default() {
    // Per security plan §Anti-Patterns API row "pulse:updater": capability
    // permissions MUST be exactly `updater:default` (no widening); the
    // capability MUST NOT bind to any webview window (windows: []). This
    // ensures the updater flow remains plugin-internal and not invokable
    // from webview JavaScript.
    let cap = read_json("capabilities/updater.json");
    let permissions = cap
        .get("permissions")
        .and_then(Value::as_array)
        .expect("capabilities/updater.json permissions array present");
    let perm_strs: Vec<&str> = permissions.iter().filter_map(Value::as_str).collect();
    assert_eq!(
        perm_strs,
        vec!["updater:default"],
        "updater capability MUST expose exactly [updater:default]; got: {perm_strs:?}"
    );

    let windows = cap
        .get("windows")
        .and_then(Value::as_array)
        .expect("capabilities/updater.json windows array present");
    assert!(
        windows.is_empty(),
        "updater capability MUST NOT bind to any webview window \
         (security plan §Anti-Patterns API row 'pulse:updater'); got: {windows:?}"
    );

    let local = cap
        .get("local")
        .and_then(Value::as_bool)
        .expect("capabilities/updater.json local field present");
    assert!(
        local,
        "updater capability MUST be local (not remote-context)"
    );
}
