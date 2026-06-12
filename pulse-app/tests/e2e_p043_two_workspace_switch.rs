//! Chunk #100 — P-043 two-workspace switch e2e (capability-audit F3
//! gap-fill): incidents are attributed to the workspace they originated
//! from, and every workspace-scoped surface — active list, P-045 unread
//! counter, P-036 previously-seen matching — reflects the CURRENT
//! workspace only when the observed workspace switches. Workspace
//! identities come from REAL `workspace_detector::detect` over two
//! marker-carrying temp dirs (the e2e_p7 template); persistence runs
//! over one shared on-disk corpus, exactly the production topology.
//!
//! Lives in pulse-app/tests/ per testing.md 2026-05-20.

use std::fs;
use std::sync::Arc;

use assert_fs::TempDir;
use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use triage::contract::{
    CueKind, CueScope, DIGEST_CORPUS_RETRIEVAL_LIMIT, EvidenceRefs, Incident, IncidentPersistence,
    IncidentStatus, PriorityTier, Severity, select_previously_seen,
};
use workspace_detector::detect::detect;

const KEYCHAIN_KEY: [u8; 32] = [0x77u8; 32];
const NOW: i64 = 1_700_000_000_000_000_000;
const DAY_NANOS: i64 = 86_400 * 1_000_000_000;

fn workspace_dir() -> TempDir {
    let tmp = TempDir::new().expect("tempdir");
    fs::create_dir(tmp.path().join(".andromeda")).expect("create .andromeda/ marker");
    tmp
}

fn incident(workspace: &str, fingerprint: &str, title: &str, opened_at: i64) -> Incident {
    Incident {
        id: 0,
        workspace: workspace.to_string(),
        fingerprint: fingerprint.to_string(),
        title: title.to_string(),
        detail: "[redacted] detail".to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("svc-shared-name".to_string()),
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: opened_at,
        updated_at_unix_nano: opened_at,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

#[test]
fn p043_workspace_switch_scopes_list_counter_and_previously_seen() {
    // Two REAL detected workspaces (canonicalized roots are the
    // attribution strings, exactly what production boot derives).
    let dir_a = workspace_dir();
    let dir_b = workspace_dir();
    let ws_a = detect(dir_a.path())
        .expect("detect A")
        .root
        .to_string_lossy()
        .into_owned();
    let ws_b = detect(dir_b.path())
        .expect("detect B")
        .root
        .to_string_lossy()
        .into_owned();
    assert_ne!(ws_a, ws_b, "two distinct workspace identities");

    let corpus_dir = TempDir::new().expect("corpus tmp");
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        KEYCHAIN_KEY,
    ));
    let corpus = Corpus::open(corpus_dir.path().join("p043.db"), backend).expect("open corpus");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus) as Arc<dyn CorpusWriter>;
    let persistence = CorpusIncidentPersistence::new(writer);

    // Session observing workspace A: two incidents (same fingerprint
    // family) land attributed to A.
    persistence
        .save_new_incident(&incident(
            &ws_a,
            "fp-shared",
            "[redacted] a-first",
            NOW - 2 * DAY_NANOS,
        ))
        .expect("save a-first");
    persistence
        .save_new_incident(&incident(
            &ws_a,
            "fp-shared",
            "[redacted] a-second",
            NOW - DAY_NANOS,
        ))
        .expect("save a-second");
    // Session observing workspace B: one incident with the SAME
    // fingerprint + the same service scope_id — the conflation bait.
    persistence
        .save_new_incident(&incident(
            &ws_b,
            "fp-shared",
            "[redacted] b-only",
            NOW - DAY_NANOS,
        ))
        .expect("save b-only");

    // Surface 1 — active list scoped to the current workspace.
    let active_a = persistence.load_active_incidents(&ws_a).expect("list A");
    assert_eq!(active_a.len(), 2, "workspace A sees exactly its incidents");
    assert!(active_a.iter().all(|i| i.workspace == ws_a));
    let active_b = persistence.load_active_incidents(&ws_b).expect("list B");
    assert_eq!(active_b.len(), 1, "switching to B reflects B's state only");
    assert!(active_b[0].title.contains("b-only"));

    // Surface 2 — P-045 unread counter scoped per workspace.
    assert_eq!(persistence.count_active_unread(&ws_a).expect("count A"), 2);
    assert_eq!(persistence.count_active_unread(&ws_b).expect("count B"), 1);

    // Surface 3 — P-036 previously-seen matching never conflates
    // cross-workspace incidents despite identical fingerprint + scope.
    let current = &active_a[1];
    let candidates_a = persistence
        .load_incidents_for_workspace_since(&ws_a, NOW - 30 * DAY_NANOS)
        .expect("candidates A");
    assert_eq!(
        candidates_a.len(),
        2,
        "candidate query is workspace-bounded at SQL level"
    );
    let matches = select_previously_seen(current, candidates_a, DIGEST_CORPUS_RETRIEVAL_LIMIT);
    assert_eq!(
        matches.len(),
        1,
        "self excluded; the prior A incident matches"
    );
    assert!(matches[0].title.contains("a-first"));
    assert!(
        matches.iter().all(|m| m.workspace == ws_a),
        "no cross-workspace candidate can appear (P-043 boundary)"
    );
}
