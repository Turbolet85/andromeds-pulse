//! Chunk #87 integration test: Findings counter derivation persists
//! across simulated app restart. Seeds N unread incidents, marks all
//! read via `incidents.mark_all_read()`, drops corpus, reopens with same
//! keychain seed, rehydrates registry from persisted state, asserts
//! `read_at_unix_nano` survived the restart (counter would derive to 0).
//!
//! Mirrors chunk #78 `unit_incident_persistence.rs::p042_cross_session_continuity`
//! cross-restart pattern. Lives in `pulse-app/tests/` per session-learnings
//! 2026-05-13 (`[lib] test = false` makes source-level `mod tests` dead
//! in pulse-app).

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::incidents_router::{IncidentsApi, IncidentsApiImpl};
use tempfile::TempDir;
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, InMemoryIncidentRegistry, Incident,
    IncidentLifecycleBroadcast, IncidentPersistence, IncidentRegistry, IncidentStatus,
    PriorityTier, Severity,
};

const WORKSPACE: &str = "ws-findings-restart";
const KEYCHAIN_KEY: [u8; 32] = [0xC7u8; 32];

fn sample_incident(workspace: &str, kind: CueKind, ts: i64) -> Incident {
    Incident {
        id: 0,
        workspace: workspace.to_string(),
        fingerprint: format!("fp-{kind:?}-{ts}"),
        title: "[redacted] sample incident".to_string(),
        detail: "[redacted] sample detail".to_string(),
        kind,
        scope: CueScope::Service,
        scope_id: None,
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: ts,
        updated_at_unix_nano: ts,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

#[tokio::test]
async fn findings_counter_state_persists_across_app_restart() {
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("findings-restart.db");

    // First session: seed 3 unread incidents + mark_all_read via resolver.
    {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
            "corpus-key",
            KEYCHAIN_KEY,
        ));
        let corpus = Corpus::open(db_path.clone(), backend).expect("first open");
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus) as Arc<dyn CorpusWriter>;
        let persistence: Arc<dyn IncidentPersistence> =
            Arc::new(CorpusIncidentPersistence::new(writer));
        let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());

        // Seed corpus + rehydrate registry from corpus rowids.
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RetryStorm,
        ] {
            let inc = sample_incident(WORKSPACE, kind, 1_000);
            let row_id = persistence.save_new_incident(&inc).expect("seed save");
            let mut hydrated = inc.clone();
            hydrated.id = row_id;
            registry.insert(hydrated);
        }

        // Sanity: P-045 SQL counts 3 unread before mark_all_read.
        let count_before = persistence
            .count_active_unread(WORKSPACE)
            .expect("count_active_unread before");
        assert_eq!(count_before, 3);

        // Invoke resolver-level mark_all_read (the bulk action surface).
        let api = IncidentsApiImpl::new(
            Arc::clone(&registry),
            Arc::new(IncidentLifecycleBroadcast::new()),
            Arc::clone(&persistence),
            WORKSPACE.to_string(),
        );
        let payload = api.mark_all_read().await.expect("infallible");
        assert_eq!(payload.affected_count, 3);

        // Verify P-045 SQL now counts zero unread (persistence captured).
        let count_after = persistence
            .count_active_unread(WORKSPACE)
            .expect("count_active_unread after");
        assert_eq!(
            count_after, 0,
            "P-045 unread-count SQL reflects in-corpus read_at_unix_nano post mark_all_read"
        );
    }

    // Second session (simulated restart): reopen corpus with same seed,
    // rehydrate registry via load_active_incidents, assert read_at
    // survived.
    let backend2: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        KEYCHAIN_KEY,
    ));
    let corpus2 = Corpus::open(db_path, backend2).expect("reopen");
    let writer2: Arc<dyn CorpusWriter> = Arc::new(corpus2) as Arc<dyn CorpusWriter>;
    let persistence2: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(writer2));

    let loaded = persistence2
        .load_active_incidents(WORKSPACE)
        .expect("rehydrate");
    assert_eq!(loaded.len(), 3, "all 3 incidents survived restart");
    for inc in &loaded {
        assert!(
            inc.read_at_unix_nano.is_some(),
            "incident {} must carry read_at after restart (got None)",
            inc.id,
        );
    }

    // Counter-derivation simulation: P-045 SQL count_active_unread
    // returns 0 across the restart boundary (the property the chunk #87
    // Findings counter relies on for "no separate state file"
    // semantics per project-doc §86).
    let count_after_restart = persistence2
        .count_active_unread(WORKSPACE)
        .expect("count_active_unread after restart");
    assert_eq!(
        count_after_restart, 0,
        "Findings counter derivation across restart MUST be 0 after mark_all_read"
    );
}
