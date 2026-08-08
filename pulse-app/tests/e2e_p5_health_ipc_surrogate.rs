//! P5 partial coverage — `health` Standard Contract envelope shape.
//! Per test-plan §6 P5 Status: full window/tray UI exercise DEFERRED to
//! chunk #51 tauri-driver headful matrix. Chunk #50 covers the agent-driven
//! IPC contract surrogate per arch §Standard Contracts (health envelope
//! locked shape).

use ui_bridge::health::{HealthStatus, current_health};

#[test]
fn p5_health_envelope_shape_matches_standard_contract() {
    let envelope = current_health();

    // status must be one of the locked enum variants.
    let is_ok_or_degraded = matches!(envelope.status, HealthStatus::Ok | HealthStatus::Degraded);
    assert!(is_ok_or_degraded, "status must be Ok or Degraded");

    // subsystems object must include all 6 named subsystems per
    // .claude/rules/verification-harness.md §Status endpoint shape.
    let s = &envelope.subsystems;
    // Each subsystem field is non-empty by virtue of the struct shape
    // (SubsystemStatus has a required `status: String`). Verify status
    // field is non-empty for each.
    assert!(
        !s.otlp_grpc_receiver.status.is_empty(),
        "otlp_grpc_receiver.status non-empty"
    );
    assert!(
        !s.otlp_http_receiver.status.is_empty(),
        "otlp_http_receiver.status non-empty"
    );
    assert!(!s.buffer.status.is_empty(), "buffer.status non-empty");
    assert!(
        !s.ingest_channel.status.is_empty(),
        "ingest_channel.status non-empty"
    );
    assert!(!s.viz.status.is_empty(), "viz.status non-empty");
    assert!(!s.plugins.status.is_empty(), "plugins.status non-empty");

    // pid + uptime_ms required per Standard Contract.
    assert!(envelope.pid > 0, "pid must be non-zero");
    let _: u64 = envelope.uptime_ms; // type assertion only — value may be 0 in test context
}

#[test]
fn p5_health_envelope_serializes_to_json_with_required_keys() {
    let envelope = current_health();
    let json = serde_json::to_string(&envelope).expect("health envelope serializes");

    // Required top-level keys per arch §Standard Contracts + verification-harness.md.
    assert!(json.contains("\"status\""), "json contains status key");
    assert!(
        json.contains("\"subsystems\""),
        "json contains subsystems key"
    );
    assert!(json.contains("\"pid\""), "json contains pid key");
    assert!(
        json.contains("\"uptime_ms\""),
        "json contains uptime_ms key"
    );
    assert!(
        json.contains("\"checked_at\""),
        "json contains checked_at key"
    );

    // Required subsystem keys.
    assert!(
        json.contains("otlp_grpc_receiver"),
        "json contains otlp_grpc_receiver"
    );
    assert!(
        json.contains("otlp_http_receiver"),
        "json contains otlp_http_receiver"
    );
    assert!(json.contains("\"buffer\""), "json contains buffer");
    assert!(
        json.contains("\"ingest_channel\""),
        "json contains ingest_channel"
    );
    assert!(json.contains("\"viz\""), "json contains viz");
    assert!(json.contains("\"plugins\""), "json contains plugins");
}

#[test]
fn p5_health_serialized_message_does_not_leak_internal_paths() {
    // Per security plan §Error Handling + §Logging hygiene: serialized
    // envelope must NOT contain file paths, library names, struct names.
    let envelope = current_health();
    let json = serde_json::to_string(&envelope).expect("serializes");

    assert!(
        !json.contains(".rs:"),
        "no .rs file paths in serialized envelope; got: {}",
        &json[..json.len().min(200)]
    );
    assert!(
        !json.contains("thiserror"),
        "no thiserror struct names in envelope"
    );
    assert!(
        !json.contains("/home/") && !json.contains("/Users/") && !json.contains("C:\\Users"),
        "no user-home paths in envelope"
    );
}
