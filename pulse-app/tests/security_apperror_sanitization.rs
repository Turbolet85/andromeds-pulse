//! Chunk #77 — Test (a) AppError sanitization PII vector test.
//!
//! Per pulse-v0_2_0-route §77 Part 2 first sub-bullet + acceptance criterion
//! #6 item (a) + security plan §Anti-Pattern Logging row 4 ("NEVER expose
//! stack traces, Rust struct names, file paths, or library versions in
//! `AppError::Internal { message }` content surfaced to the webview —
//! sanitize at the `From<thiserror::Error> for AppError` impl"):
//!
//! Exercise the `From<E> for AppError` impls in `crates/ui-bridge/src/contract.rs`
//! (chunks #26 / #44 / #68 + downstream) and assert the serialized AppError JSON
//! across the TauRPC bridge contains no PII-vector substrings (Rust struct
//! names, file paths, stack traces, library versions, source-chain markers).
//!
//! The From impls already produce sanitized constant strings per the security
//! plan discipline; this test is the negative-canary contract verification
//! that the discipline holds across all 6 AppError variants.

use buffer::Error as BufferError;
use plugins::contract::Error as PluginsError;
use snapshot::contract::Error as SnapshotError;
use ui_bridge::contract::AppError;
use viz::Error as VizError;
use workspace_detector::contract::Error as WorkspaceDetectorError;

/// PII-vector substrings that MUST NOT appear in any serialized AppError per
/// security plan §Anti-Pattern Logging row 4 + §Logging "NEVER expose stack
/// traces, Rust struct names, file paths, or library versions". Each substring
/// represents a class of leak the From impls must suppress.
const NEVER_IN_APPERROR_JSON: &[&str] = &[
    // File-system paths (Linux / macOS / Windows)
    "at /home/",
    "at /Users/",
    "at C:\\",
    "/src/",
    "\\src\\",
    "crates/",
    // Rust struct / module names (qualified paths)
    "::Error::",
    "anyhow::",
    "thiserror::",
    "::source(",
    "BoxedError",
    "ErrorImpl",
    // Stack-trace markers
    "stack backtrace:",
    "stack trace:",
    "at line ",
    "RUST_BACKTRACE",
    "SpanTrace",
    // Library version leakage (typical Cargo.toml-style strings)
    "version ",
    " v0.",
    // Source-chain leakage (Display chain of source()-walking)
    "caused by:",
    "Caused by:",
    // Specific known sensitive substrings that have appeared in past chains
    ".rs:",
];

/// Assert the serialized AppError JSON contains none of the NEVER strings.
fn assert_no_pii_leak(err: &AppError, variant_name: &str) {
    let json = serde_json::to_string(err).expect("AppError serializes");
    for forbidden in NEVER_IN_APPERROR_JSON {
        assert!(
            !json.contains(forbidden),
            "AppError::{variant_name} JSON leaked PII-vector substring `{forbidden}` per security plan §Anti-Pattern Logging row 4. Full JSON: {json}"
        );
    }
}

#[test]
fn apperror_validation_does_not_leak_pii_vectors() {
    let err = AppError::Validation {
        field: "retention_seconds".into(),
        reason: "out of range".into(),
    };
    assert_no_pii_leak(&err, "Validation");
}

#[test]
fn apperror_not_found_does_not_leak_pii_vectors() {
    let err = AppError::NotFound {
        resource: "snapshot:nonexistent".into(),
    };
    assert_no_pii_leak(&err, "NotFound");
}

#[test]
fn apperror_internal_from_constant_message_does_not_leak_pii_vectors() {
    // The `AppError::internal()` constructor accepts any string. Exercise
    // both a short summary AND a longer sanitized message; both must serialize
    // without PII markers.
    let short = AppError::internal("operation failed");
    assert_no_pii_leak(&short, "Internal(short)");

    let longer = AppError::internal(
        "retention sweep failed during periodic interval; investigate buffer pressure",
    );
    assert_no_pii_leak(&longer, "Internal(longer)");
}

#[test]
fn apperror_plugin_does_not_leak_pii_vectors() {
    let err = AppError::Plugin {
        plugin_id: "example-plugin".into(),
        message: "guest exceeded epoch deadline".into(),
    };
    assert_no_pii_leak(&err, "Plugin");
}

#[test]
fn apperror_storage_does_not_leak_pii_vectors() {
    let err = AppError::Storage {
        message: "buffer connection lost".into(),
    };
    assert_no_pii_leak(&err, "Storage");
}

#[test]
fn apperror_ingest_does_not_leak_pii_vectors() {
    let err = AppError::Ingest {
        message: "OTLP payload rejected for invariant violation".into(),
    };
    assert_no_pii_leak(&err, "Ingest");
}

#[test]
fn from_buffer_error_init_produces_sanitized_storage_variant() {
    // Construct a BufferError::Init carrying a file path + library name in its
    // `reason` field — the From impl MUST collapse to the constant "buffer
    // init failed" message per `crates/ui-bridge/src/contract.rs:382-415`
    // impl block. Verify by serializing the resulting AppError.
    let buffer_err = BufferError::Init {
        reason: "failed to open database at /home/secret/path/database.db with anyhow::Error chain"
            .into(),
    };
    let app_err: AppError = buffer_err.into();
    assert_no_pii_leak(&app_err, "Storage(from BufferError::Init)");
    // Additionally assert the source-chain string was discarded — message is
    // the sanitized constant only.
    if let AppError::Storage { message } = &app_err {
        assert!(
            !message.contains("/home/"),
            "AppError::Storage leaked source-chain path from BufferError::Init: {message}"
        );
        assert!(
            !message.contains("anyhow"),
            "AppError::Storage leaked library name from BufferError::Init: {message}"
        );
    } else {
        panic!("expected AppError::Storage, got {app_err:?}");
    }
}

#[test]
fn from_viz_error_query_failed_produces_sanitized_storage_variant() {
    let viz_err = VizError::QueryFailed {
        reason: "SQL execution failed at line 47 of /src/viz/query.rs".into(),
    };
    let app_err: AppError = viz_err.into();
    assert_no_pii_leak(&app_err, "Storage(from VizError::QueryFailed)");
}

#[test]
fn from_snapshot_error_propagates_sanitized_message_only() {
    // Construct a SnapshotError variant carrying potentially leaky reason;
    // From impl should sanitize per `crates/ui-bridge/src/contract.rs:474+`.
    let snapshot_err = SnapshotError::InvalidSpanRecord {
        kind: "missing trace_id",
    };
    let app_err: AppError = snapshot_err.into();
    assert_no_pii_leak(&app_err, "from SnapshotError::InvalidSpanRecord");
}

#[test]
fn from_plugins_error_produces_sanitized_plugin_or_internal_variant() {
    let plugins_err = PluginsError::NotFound {
        plugin_id: "example".into(),
    };
    let app_err: AppError = plugins_err.into();
    assert_no_pii_leak(&app_err, "from PluginsError::NotFound");
}

#[test]
fn from_workspace_detector_error_produces_sanitized_variant() {
    let ws_err = WorkspaceDetectorError::PathTraversalRejected {
        reason: "resolved path escapes /Users/secret/data dir".into(),
    };
    let app_err: AppError = ws_err.into();
    assert_no_pii_leak(
        &app_err,
        "from WorkspaceDetectorError::PathTraversalRejected",
    );
}

#[test]
fn apperror_internal_constructor_does_not_inject_caller_context() {
    // Sanity: the `AppError::internal(msg)` constructor stores `msg` verbatim
    // in the Internal variant's `message` field. The CALLER is responsible
    // for passing only sanitized strings. This test documents that contract
    // by asserting the constructor does NOT add file:line / module path /
    // SpanTrace context from calling site automatically — the message is taken
    // verbatim.
    let msg = "deliberately-checked-marker-string";
    let err = AppError::internal(msg);
    let json = serde_json::to_string(&err).expect("serialize");
    assert!(
        json.contains(msg),
        "AppError::internal must preserve caller's message verbatim; got: {json}"
    );
    // And NO injected context surfaces beyond the caller-provided message —
    // this is the discipline boundary that lets all callers safely use this
    // constructor without worrying about Rust-internal context leakage.
    assert_no_pii_leak(&err, "Internal(verbatim)");
}
