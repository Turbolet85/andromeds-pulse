//! Unit tests for `pulse_app::training_export` (chunk #95). Integration-test
//! crate per `[lib] test = false` (CLAUDE.md testing 2026-05-13/2026-05-20).

use corpus::contract::IncidentRowRaw;
use pulse_app::training_export::{
    assemble_records, build_preview_summary, count_redactions, incident_row_to_export_record,
    resolve_default_target, serialize_jsonl, validate_export_target,
};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, Incident, IncidentStatus, PriorityTier, Severity,
};

fn sample_incident(
    title: &str,
    detail: &str,
    scope_id: Option<&str>,
    resolution: Option<&str>,
    fingerprints: Vec<&str>,
    status: IncidentStatus,
    severity: Severity,
) -> Incident {
    Incident {
        id: 0,
        workspace: "/home/dev/project".to_string(),
        fingerprint: "fp-uuid".to_string(),
        title: title.to_string(),
        detail: detail.to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: scope_id.map(|s| s.to_string()),
        status,
        severity,
        priority_tier: PriorityTier::Autonomous,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: fingerprints.iter().map(|s| s.to_string()).collect(),
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: 1_000,
        updated_at_unix_nano: 2_000,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: resolution.map(|_| 2_000),
        read_at_unix_nano: None,
        resolution_summary_text: resolution.map(|s| s.to_string()),
    }
}

fn row_for(incident: &Incident, id: i64, workspace: &str, status: &str) -> IncidentRowRaw {
    IncidentRowRaw {
        id,
        workspace: workspace.to_string(),
        status: status.to_string(),
        created_unix_nano: incident.opened_at_unix_nano,
        updated_unix_nano: incident.updated_at_unix_nano,
        resolved_unix_nano: incident.resolved_at_unix_nano,
        read_unix_nano: incident.read_at_unix_nano,
        payload: bincode::serialize(incident).expect("bincode serialize"),
    }
}

#[test]
fn incident_row_to_export_record_maps_fields_and_restamps_metadata() {
    let incident = sample_incident(
        "checkout latency regression",
        "p95 exceeded baseline",
        Some("checkout-service"),
        Some("{\"decision\":\"surface\"}"),
        vec!["abc123"],
        IncidentStatus::Resolved,
        Severity::Error,
    );
    let row = row_for(&incident, 42, "/ws-x", "resolved");
    let record = incident_row_to_export_record(&row).expect("decode");

    // Re-stamped from the SQL columns.
    assert_eq!(record.id, 42);
    assert_eq!(record.workspace, "/ws-x");
    // Non-PII text passes through unchanged.
    assert_eq!(record.title, "checkout latency regression");
    assert_eq!(record.detail, "p95 exceeded baseline");
    assert_eq!(record.scope_id.as_deref(), Some("checkout-service"));
    assert_eq!(
        record.interpretation.as_deref(),
        Some("{\"decision\":\"surface\"}")
    );
    assert_eq!(record.fingerprint_hashes, vec!["abc123".to_string()]);
    assert_eq!(record.status, IncidentStatus::Resolved);
    assert_eq!(record.severity, Severity::Error);
}

#[test]
fn scrub_redacts_pii_in_exported_text_fields() {
    let canary_email = "alice@example.com";
    let canary_jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV1adQssw5c";
    let incident = sample_incident(
        canary_email,
        canary_jwt,
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Warn,
    );
    let row = row_for(&incident, 1, "/ws", "active");
    let record = incident_row_to_export_record(&row).expect("decode");

    assert!(record.title.starts_with("[redacted:"));
    assert!(record.detail.starts_with("[redacted:"));
    // Negative-canary: the raw secrets MUST NOT survive into the record.
    assert!(!record.title.contains(canary_email));
    assert!(!record.detail.contains(canary_jwt));

    // ... and the redaction must be absent from the serialized JSONL.
    let jsonl = serialize_jsonl(&[record]).expect("jsonl");
    assert!(!jsonl.contains(canary_email));
    assert!(!jsonl.contains(canary_jwt));
}

#[test]
fn serialize_jsonl_produces_one_valid_json_object_per_line() {
    let a = sample_incident(
        "a",
        "d-a",
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Info,
    );
    let b = sample_incident(
        "b",
        "d-b",
        None,
        None,
        vec![],
        IncidentStatus::Resolved,
        Severity::Critical,
    );
    let rows = [
        row_for(&a, 1, "/ws", "active"),
        row_for(&b, 2, "/ws", "resolved"),
    ];
    let records = assemble_records(&rows).expect("assemble");
    let jsonl = serialize_jsonl(&records).expect("jsonl");

    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 2);
    for line in lines {
        let value: serde_json::Value = serde_json::from_str(line).expect("each line is valid JSON");
        assert!(value.get("id").is_some());
        assert!(value.get("status").is_some());
        assert!(value.get("severity").is_some());
    }
}

#[test]
fn build_preview_summary_counts_by_status_and_severity_with_date_range() {
    let a = sample_incident(
        "a",
        "d",
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Error,
    );
    let mut b = sample_incident(
        "b",
        "d",
        None,
        None,
        vec![],
        IncidentStatus::Resolved,
        Severity::Error,
    );
    b.opened_at_unix_nano = 5_000;
    let rows = [
        row_for(&a, 1, "/ws", "active"),
        row_for(&b, 2, "/ws", "resolved"),
    ];
    let records = assemble_records(&rows).expect("assemble");
    let summary = build_preview_summary(&records, false, None);

    assert_eq!(summary.total_records, 2);
    assert!(!summary.written);
    assert!(summary.anonymization_confirmed);
    assert_eq!(summary.date_range_start_unix_nano, Some(1_000));
    assert_eq!(summary.date_range_end_unix_nano, Some(5_000));

    let active = summary
        .categories
        .iter()
        .find(|c| c.dimension == "status" && c.label == "active");
    assert_eq!(active.map(|c| c.count), Some(1));
    let resolved = summary
        .categories
        .iter()
        .find(|c| c.dimension == "status" && c.label == "resolved");
    assert_eq!(resolved.map(|c| c.count), Some(1));
    let error_sev = summary
        .categories
        .iter()
        .find(|c| c.dimension == "severity" && c.label == "error");
    assert_eq!(error_sev.map(|c| c.count), Some(2));
}

#[test]
fn build_preview_summary_handles_empty_corpus() {
    let summary = build_preview_summary(&[], false, None);
    assert_eq!(summary.total_records, 0);
    assert!(summary.categories.is_empty());
    assert_eq!(summary.date_range_start_unix_nano, None);
    assert_eq!(summary.date_range_end_unix_nano, None);
}

#[test]
fn count_redactions_counts_redacted_fields() {
    let clean = sample_incident(
        "ok",
        "fine",
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Info,
    );
    let dirty = sample_incident(
        "user bob@example.com",
        "fine",
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Info,
    );
    let rows = [
        row_for(&clean, 1, "/ws", "active"),
        row_for(&dirty, 2, "/ws", "active"),
    ];
    let records = assemble_records(&rows).expect("assemble");
    assert_eq!(count_redactions(&records), 1);
}

#[test]
fn count_redactions_span_mask_counts_a_mid_value_placeholder() {
    let masked = sample_incident(
        "user [redacted: email] signed in",
        "fine",
        None,
        None,
        vec![],
        IncidentStatus::Active,
        Severity::Info,
    );
    let rows = [row_for(&masked, 1, "/ws", "active")];
    let records = assemble_records(&rows).expect("assemble");
    assert_eq!(count_redactions(&records), 1);
}

#[test]
fn export_interpretation_span_mask_keeps_json_parseable_across_delimiters() {
    // The `symptom` key match runs to the end of its line, and the whole
    // summary is one compact-JSON line; `timeline` ends in a key word whose
    // value would start at the next JSON delimiter.
    let summary = r#"{"symptom":"login password=hunter2 failed","timeline":"rotate the secret:","title":"auth errors"}"#;
    let incident = sample_incident(
        "ok",
        "fine",
        None,
        Some(summary),
        vec![],
        IncidentStatus::Resolved,
        Severity::Info,
    );
    let rows = [row_for(&incident, 1, "/ws", "resolved")];
    let records = assemble_records(&rows).expect("assemble");
    let exported = records[0]
        .interpretation
        .as_deref()
        .expect("interpretation exported");

    assert!(
        !exported.contains("hunter2"),
        "the keyed value must not export"
    );
    let parsed: serde_json::Value =
        serde_json::from_str(exported).expect("the exported interpretation stays parseable JSON");
    assert_eq!(
        parsed,
        serde_json::json!({
            "symptom": "login [redacted: secret_kv]",
            "timeline": "rotate the secret:",
            "title": "auth errors",
        })
    );
    assert_eq!(count_redactions(&records), 1);
}

#[test]
fn resolve_default_target_uses_injected_timestamp_under_downloads() {
    let path = resolve_default_target(987_654_321).expect("home dir available in test env");
    assert_eq!(
        path.file_name().and_then(|s| s.to_str()),
        Some("pulse-corpus-export-987654321.jsonl"),
    );
    assert!(path.to_string_lossy().contains("Downloads"));
}

#[test]
fn validate_export_target_rejects_parent_dir_traversal() {
    let bad = std::path::Path::new("/some/dir/../etc/passwd.jsonl");
    let err = validate_export_target(bad).expect_err("traversal rejected");
    match err {
        ui_bridge::contract::AppError::Validation { field, .. } => {
            assert_eq!(field, "target_path");
        }
        other => panic!("expected Validation, got {other:?}"),
    }
}

#[test]
fn validate_export_target_rejects_nonexistent_parent() {
    let bad = std::path::Path::new("/nonexistent-pulse-xyz-9931/export.jsonl");
    let err = validate_export_target(bad).expect_err("missing parent rejected");
    assert!(matches!(
        err,
        ui_bridge::contract::AppError::Validation { .. }
    ));
}

#[test]
fn validate_export_target_accepts_existing_parent() {
    let dir = std::env::temp_dir();
    let target = dir.join("pulse-export-validate-test.jsonl");
    let validated = validate_export_target(&target).expect("temp_dir parent exists");
    assert_eq!(validated, target);
}
