//! Cross-path security negative canaries — plan.md Step 10.
//!
//! Per security extract Acceptance criteria: SQL-injection canaries on
//! all 3 query routers; AppError sanitization round-trip per variant
//! (no stack traces / file paths / library names в serialized message);
//! capability-drift exit-0 precondition gate.

use std::sync::{Arc, Mutex};

use buffer::create_schema;
use duckdb::Connection;
use ui_bridge::contract::AppError;
use viz::VizState;
use viz::query::{LogsQueryArgs, MetricsQueryArgs, TracesQueryArgs};

fn ephemeral_conn() -> Arc<Mutex<Connection>> {
    let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
    create_schema(&raw).expect("schema creates");
    Arc::new(Mutex::new(raw))
}

#[test]
fn security_sql_injection_canary_on_traces_query_router() {
    let conn = ephemeral_conn();
    let viz_state = Arc::new(VizState::new());

    let args = TracesQueryArgs {
        time_window_seconds: 60,
        limit: 50,
        cursor: Some("'; DROP TABLE spans; --".to_string()),
    };
    let result = viz::query::query_traces(&conn, &viz_state, &args);

    // Either rejected at parse OR returns empty — both prove injection
    // didn't execute. Re-check spans table integrity afterward.
    let _ = result;
    let recheck = viz::query::query_traces(
        &conn,
        &viz_state,
        &TracesQueryArgs {
            time_window_seconds: 60,
            limit: 1,
            cursor: None,
        },
    );
    assert!(
        recheck.is_ok(),
        "spans table must remain intact after injection attempt; got {:?}",
        recheck.err()
    );
}

#[test]
fn security_sql_injection_canary_on_metrics_query_router() {
    let conn = ephemeral_conn();
    let viz_state = Arc::new(VizState::new());

    let args = MetricsQueryArgs {
        time_window_seconds: 60,
        limit: 50,
        cursor: Some("'; DROP TABLE metrics_points; --".to_string()),
    };
    let _ = viz::query::query_metrics(&conn, &viz_state, &args);

    let recheck = viz::query::query_metrics(
        &conn,
        &viz_state,
        &MetricsQueryArgs {
            time_window_seconds: 60,
            limit: 1,
            cursor: None,
        },
    );
    assert!(
        recheck.is_ok(),
        "metrics_points table must remain intact; got {:?}",
        recheck.err()
    );
}

#[test]
fn security_sql_injection_canary_on_logs_query_router() {
    let conn = ephemeral_conn();
    let viz_state = Arc::new(VizState::new());

    let args = LogsQueryArgs {
        time_window_seconds: 60,
        limit: 50,
        cursor: Some("'; DROP TABLE log_records; --".to_string()),
    };
    let _ = viz::query::query_logs(&conn, &viz_state, &args);

    let recheck = viz::query::query_logs(
        &conn,
        &viz_state,
        &LogsQueryArgs {
            time_window_seconds: 60,
            limit: 1,
            cursor: None,
        },
    );
    assert!(
        recheck.is_ok(),
        "log_records table must remain intact; got {:?}",
        recheck.err()
    );
}

#[test]
fn security_app_error_validation_serialized_message_no_internal_leaks() {
    let err = AppError::Validation {
        field: "test_field".to_string(),
        reason: "value out of range".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(
        !json.contains(".rs:"),
        "no .rs paths in Validation; got: {json}"
    );
    assert!(!json.contains("thiserror"), "no thiserror crate name");
    assert!(!json.contains("anyhow"), "no anyhow crate name");
    assert!(!json.contains("wasmtime"), "no wasmtime crate name");
    assert!(!json.contains("duckdb"), "no duckdb crate name");
}

#[test]
fn security_app_error_internal_serialized_message_no_internal_leaks() {
    let err = AppError::Internal {
        message: "boot failed".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(!json.contains(".rs:"), "no .rs paths");
    assert!(!json.contains("/home/"), "no user home paths");
    assert!(!json.contains("C:\\Users"), "no Windows user home");
    assert!(!json.contains("thiserror"), "no thiserror");
}

#[test]
fn security_app_error_not_found_serialized_message_no_internal_leaks() {
    let err = AppError::NotFound {
        resource: "plugin-foo".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(!json.contains(".rs:"));
    assert!(!json.contains("anyhow"));
}

#[test]
fn security_app_error_plugin_serialized_message_no_internal_leaks() {
    let err = AppError::Plugin {
        plugin_id: "test-plugin".to_string(),
        message: "WIT capability mismatch".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(!json.contains(".rs:"));
    assert!(!json.contains("wasmtime::"), "no wasmtime:: paths");
}

#[test]
fn security_app_error_storage_serialized_message_no_internal_leaks() {
    let err = AppError::Storage {
        message: "query failed".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(!json.contains(".rs:"));
    assert!(!json.contains("duckdb::"), "no duckdb:: paths");
}

#[test]
fn security_app_error_ingest_serialized_message_no_internal_leaks() {
    let err = AppError::Ingest {
        message: "bind failed".to_string(),
    };
    let json = serde_json::to_string(&err).expect("serializes");
    assert!(!json.contains(".rs:"));
    assert!(!json.contains("tonic::"), "no tonic:: paths");
    assert!(!json.contains("axum::"), "no axum:: paths");
}

#[test]
fn security_workspace_wide_loopback_only_no_non_loopback_bind_literals() {
    // Workspace-wide grep gate per plan.md Implementation Step 14 —
    // walks crates/{ingest,buffer,viz,ui-bridge,snapshot,workspace-detector,
    // plugins,mcp-server}/src/ + pulse-app/src/ + pulse-app/tests/
    // searching for non-loopback bind literals. Asserts zero matches
    // outside this test file itself.
    use std::fs;
    use std::path::Path;

    // Files that legitimately contain the forbidden literals because they
    // ARE grep gates (they use the literals as needles in their own scans).
    // Skip these to avoid self-referential failure.
    const GREP_GATE_FILES: &[&str] = &[
        "e2e_security_negative_canaries.rs",
        "grpc_loopback.rs", // chunk #18 ingest-crate-scoped grep gate
    ];

    fn walk(dir: &Path, hits: &mut Vec<String>, skip_files: &[&str]) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, hits, skip_files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let path_str = path.to_string_lossy();
                if skip_files.iter().any(|f| path_str.contains(f)) {
                    continue;
                }
                if let Ok(contents) = fs::read_to_string(&path) {
                    for (lineno, line) in contents.lines().enumerate() {
                        let code = line.split("//").next().unwrap_or("");
                        for needle in [
                            "0.0.0.0",
                            "Ipv4Addr::UNSPECIFIED",
                            "Ipv6Addr::UNSPECIFIED",
                            "([0, 0, 0, 0]",
                        ] {
                            if code.contains(needle) {
                                hits.push(format!(
                                    "{}:{}: {}",
                                    path.display(),
                                    lineno + 1,
                                    line.trim()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    // workspace root is two levels up from pulse-app/tests/
    // (CARGO_MANIFEST_DIR is pulse-app/).
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().expect("workspace root");
    let scan_dirs = [
        workspace_root.join("crates"),
        workspace_root.join("pulse-app").join("src"),
        workspace_root.join("pulse-app").join("tests"),
    ];
    let mut hits = Vec::new();
    for dir in &scan_dirs {
        walk(dir, &mut hits, GREP_GATE_FILES);
    }
    assert!(
        hits.is_empty(),
        "non-loopback bind literals found in workspace:\n{}",
        hits.join("\n")
    );
}
