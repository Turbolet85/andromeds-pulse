//! E2E P95 — `storage.export_for_training` resolver (chunk #95). Seeds an
//! in-memory corpus with incidents (via the corpus write path), then drives
//! the resolver's preview + confirm + traversal-rejection paths. Integration
//! test crate per `[lib] test = false` (CLAUDE.md testing 2026-05-13).

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusReader, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::storage_router::{StorageApi, StorageApiImpl};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, Incident, IncidentStatus, PriorityTier, Severity,
};
use ui_bridge::contract::AppError;

fn incident(
    title: &str,
    detail: &str,
    status: IncidentStatus,
    severity: Severity,
    resolution: Option<&str>,
) -> Incident {
    Incident {
        id: 0,
        workspace: "/ws".to_string(),
        fingerprint: "fp".to_string(),
        title: title.to_string(),
        detail: detail.to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("svc".to_string()),
        status,
        severity,
        priority_tier: PriorityTier::Autonomous,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: 1_000,
        updated_at_unix_nano: 1_000,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: resolution.map(|_| 2_000),
        read_at_unix_nano: None,
        resolution_summary_text: resolution.map(|s| s.to_string()),
    }
}

fn seeded_impl(incidents: &[(Incident, &str, &str)]) -> StorageApiImpl {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    let corpus_arc = Arc::new(corpus);
    let reader: Arc<dyn CorpusReader> = Arc::clone(&corpus_arc) as Arc<dyn CorpusReader>;
    let writer: Arc<dyn CorpusWriter> = Arc::clone(&corpus_arc) as Arc<dyn CorpusWriter>;
    for (inc, workspace, status) in incidents {
        let payload = bincode::serialize(inc).expect("bincode serialize");
        writer
            .save_incident(
                workspace,
                status,
                inc.opened_at_unix_nano,
                inc.updated_at_unix_nano,
                inc.resolved_at_unix_nano,
                None,
                &payload,
            )
            .expect("seed incident");
    }
    StorageApiImpl::new(reader, writer)
}

#[tokio::test]
async fn preview_returns_summary_without_writing_file() {
    let api = seeded_impl(&[
        (
            incident("a", "d", IncidentStatus::Active, Severity::Error, None),
            "/ws",
            "active",
        ),
        (
            incident(
                "b",
                "d",
                IncidentStatus::Resolved,
                Severity::Warn,
                Some("{}"),
            ),
            "/ws",
            "resolved",
        ),
    ]);
    let summary = api
        .export_for_training(None, false)
        .await
        .expect("preview ok");
    assert_eq!(summary.total_records, 2);
    assert!(!summary.written);
    assert_eq!(summary.written_path_basename, None);
    assert!(summary.anonymization_confirmed);
}

#[tokio::test]
async fn confirm_writes_expected_jsonl_with_canary_absent() {
    let canary = "secret@example.com";
    let api = seeded_impl(&[
        (
            incident(
                canary,
                "detail one",
                IncidentStatus::Active,
                Severity::Error,
                None,
            ),
            "/ws",
            "active",
        ),
        (
            incident(
                "clean title",
                "detail two",
                IncidentStatus::Resolved,
                Severity::Info,
                Some("{}"),
            ),
            "/ws",
            "resolved",
        ),
    ]);
    let target =
        std::env::temp_dir().join(format!("pulse-export-e2e-{}.jsonl", std::process::id()));
    let _ = std::fs::remove_file(&target);

    let summary = api
        .export_for_training(Some(target.to_string_lossy().to_string()), true)
        .await
        .expect("export ok");
    assert!(summary.written);
    assert_eq!(summary.total_records, 2);
    assert!(summary.written_path_basename.is_some());

    let contents = std::fs::read_to_string(&target).expect("export file written");
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2);
    for line in &lines {
        let _: serde_json::Value = serde_json::from_str(line).expect("valid JSON line");
    }
    // Negative canary: seeded PII MUST NOT appear verbatim in the export.
    assert!(
        !contents.contains(canary),
        "PII canary leaked into the export file"
    );

    let _ = std::fs::remove_file(&target);
}

#[tokio::test]
async fn traversal_target_rejected_with_validation_error() {
    let api = seeded_impl(&[(
        incident("a", "d", IncidentStatus::Active, Severity::Error, None),
        "/ws",
        "active",
    )]);
    let result = api
        .export_for_training(Some("/tmp/../etc/pulse-export.jsonl".to_string()), true)
        .await;
    match result {
        Err(AppError::Validation { field, .. }) => assert_eq!(field, "target_path"),
        other => panic!("expected AppError::Validation, got {other:?}"),
    }
}

#[tokio::test]
async fn export_empty_corpus_returns_zero_records() {
    let api = seeded_impl(&[]);
    let summary = api
        .export_for_training(None, false)
        .await
        .expect("preview ok");
    assert_eq!(summary.total_records, 0);
    assert!(summary.categories.is_empty());
    assert!(!summary.written);
}
