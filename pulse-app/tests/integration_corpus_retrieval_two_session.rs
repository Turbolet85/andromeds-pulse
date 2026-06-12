//! Chunk #100 — P-044 two-session recurrence (the capability-audit F1
//! matrix gap): an incident persisted in session A surfaces in session
//! B's digest `corpus_matches` after a full corpus drop + reopen, with
//! all four retrieval filters asserted — same-workspace, ≤30 days,
//! fingerprint/scope match, top-5 cap — including negative exclusions
//! (cross-workspace + >30-day candidates). Also covers the egress scrub
//! (a legacy-style unscrubbed canary never reaches the digest) + the
//! aggregate-only retrieval event (no fingerprint / workspace / incident
//! text in tracing fields) + the P-041 boot purge wiring.
//!
//! Drives the REAL `Assembler` + `CorpusBackedIncidentSource` + the real
//! PII scrub closure over an on-disk corpus, mirroring
//! `integration_incident_producer_persists_across_restart.rs`. Lives in
//! pulse-app/tests/ per testing.md 2026-05-20 (`[lib] test = false`).

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::corpus_retrieval::{CorpusBackedIncidentSource, run_pipeline_metrics_purge};
use pulse_app::digest_runtime::pii_scrub_closure;
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use tempfile::TempDir;
use triage::contract::{
    Assembler, CadenceMode, CueKind, CueScope, DigestAssembler, DigestBroadcast,
    DigestProjectContext, EvidenceRefs, InMemoryIncidentRegistry, Incident, IncidentPersistence,
    IncidentStatus, LwwQueue, PriorityTier, Q1RedRow, Q2OperationRow, Q3FingerprintRow,
    Q4InteractionRow, Q5CardinalityRow, Q6LogRow, Q7CriticalPathRow, Severity, SqlAggregationError,
    SqlQueryRunner,
};

const WORKSPACE: &str = "ws-recurrence";
const OTHER_WORKSPACE: &str = "ws-other";
const KEYCHAIN_KEY: [u8; 32] = [0x5Au8; 32];
const NOW: i64 = 1_700_000_000_000_000_000;
const DAY_NANOS: i64 = 86_400 * 1_000_000_000;

fn fp_hex() -> String {
    "aa".repeat(16)
}

fn incident(fingerprint: &str, workspace: &str, title: &str, opened_at: i64) -> Incident {
    Incident {
        id: 0,
        workspace: workspace.to_string(),
        fingerprint: fingerprint.to_string(),
        title: title.to_string(),
        detail: "[redacted] detail".to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("svc-checkout".to_string()),
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

type SqlFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, SqlAggregationError>> + Send + 'a>>;

/// Canned L1a runner: Q3 carries the recurrence fingerprint bytes so the
/// assembler's match inputs come from the same surface production uses.
struct CannedSqlRunner {
    q3: Vec<Q3FingerprintRow>,
}

impl SqlQueryRunner for CannedSqlRunner {
    fn run_q1<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q1RedRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
    fn run_q2<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q2OperationRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
    fn run_q3<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q3FingerprintRow>> {
        let rows = self.q3.clone();
        Box::pin(async move { Ok(rows) })
    }
    fn run_q4<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q4InteractionRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
    fn run_q5<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q5CardinalityRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
    fn run_q6<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q6LogRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
    fn run_q7<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q7CriticalPathRow>> {
        Box::pin(async move { Ok(Vec::new()) })
    }
}

fn q3_row_for_recurrence_fp() -> Q3FingerprintRow {
    Q3FingerprintRow {
        fingerprint: vec![0xAAu8; 16],
        occurrences: 4,
        first_seen: NOW - 60_000_000_000,
        last_seen: NOW,
    }
}

fn open_writer(db_path: std::path::PathBuf) -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        KEYCHAIN_KEY,
    ));
    let corpus = Corpus::open(db_path, backend).expect("corpus open");
    Arc::new(corpus) as Arc<dyn CorpusWriter>
}

fn build_session_b_assembler(writer: Arc<dyn CorpusWriter>) -> Arc<Assembler> {
    Arc::new(
        Assembler::new(
            Arc::new(CannedSqlRunner {
                q3: vec![q3_row_for_recurrence_fp()],
            }),
            Arc::new(InMemoryIncidentRegistry::new()),
            Arc::new(DigestBroadcast::new()),
            Arc::new(Mutex::new(LwwQueue::new())),
            pii_scrub_closure(),
            Arc::new(CorpusBackedIncidentSource::new(writer)),
        )
        .expect("assembler init"),
    )
}

fn project_context() -> DigestProjectContext {
    DigestProjectContext {
        workspace_canonical_path: WORKSPACE.to_string(),
        project_name: Some("recurrence".to_string()),
        vcs_type: Some("git"),
        recent_commits: Vec::new(),
        framework_signals: Vec::new(),
    }
}

#[tokio::test]
async fn incident_from_session_a_surfaces_in_session_b_corpus_matches() {
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("recurrence.db");

    // Session A: persist the recurrence incident + the two negatives.
    {
        let writer = open_writer(db_path.clone());
        let persistence = CorpusIncidentPersistence::new(writer);
        persistence
            .save_new_incident(&incident(
                &fp_hex(),
                WORKSPACE,
                "[redacted] checkout error spike",
                NOW - DAY_NANOS,
            ))
            .expect("save in-window incident");
        persistence
            .save_new_incident(&incident(
                &fp_hex(),
                OTHER_WORKSPACE,
                "[redacted] other-workspace occurrence",
                NOW - DAY_NANOS,
            ))
            .expect("save cross-workspace negative");
        persistence
            .save_new_incident(&incident(
                &fp_hex(),
                WORKSPACE,
                "[redacted] stale occurrence",
                NOW - 40 * DAY_NANOS,
            ))
            .expect("save >30-day negative");
    }
    // Corpus dropped here — session boundary.

    // Session B: reopen + assemble through the real retrieval path.
    let writer = open_writer(db_path);
    let assembler = build_session_b_assembler(writer);
    let digest = assembler
        .assemble(
            CadenceMode::Tier2,
            None,
            &project_context(),
            NOW,
            Duration::from_secs(60),
        )
        .await
        .expect("assemble succeeds");

    assert_eq!(
        digest.corpus_matches.len(),
        1,
        "exactly the in-window same-workspace fingerprint match; got {:?}",
        digest.corpus_matches
    );
    let line = &digest.corpus_matches[0];
    assert!(line.contains(&fp_hex()), "fingerprint identity: {line}");
    assert!(
        line.contains("checkout error spike"),
        "title carried: {line}"
    );
    assert!(
        !line.contains("other-workspace"),
        "cross-workspace candidate excluded: {line}"
    );
    assert!(
        !digest
            .corpus_matches
            .iter()
            .any(|l| l.contains("stale occurrence")),
        "31+ day candidate excluded"
    );
    assert!(
        digest.payload_summary.contains("CORPUS MATCHES:"),
        "matches render into the LLM-facing payload"
    );
}

#[tokio::test]
async fn retrieval_caps_at_top_five_newest_matches() {
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("topfive.db");
    {
        let writer = open_writer(db_path.clone());
        let persistence = CorpusIncidentPersistence::new(writer);
        for i in 0..7 {
            persistence
                .save_new_incident(&incident(
                    &fp_hex(),
                    WORKSPACE,
                    &format!("[redacted] occurrence-{i}"),
                    NOW - (i + 1) * DAY_NANOS,
                ))
                .expect("save candidate");
        }
    }
    let writer = open_writer(db_path);
    let assembler = build_session_b_assembler(writer);
    let digest = assembler
        .assemble(
            CadenceMode::Tier2,
            None,
            &project_context(),
            NOW,
            Duration::from_secs(60),
        )
        .await
        .expect("assemble succeeds");
    assert_eq!(digest.corpus_matches.len(), 5, "top-5 cap (spec default)");
    assert!(
        digest.corpus_matches[0].contains("occurrence-0"),
        "newest first: {:?}",
        digest.corpus_matches
    );
    assert!(
        !digest
            .corpus_matches
            .iter()
            .any(|l| l.contains("occurrence-5") || l.contains("occurrence-6")),
        "oldest two candidates fall off the cap"
    );
}

#[tokio::test]
async fn legacy_unscrubbed_canary_never_reaches_the_digest() {
    // Simulates a pre-chunk-#72 legacy BLOB whose title was persisted
    // WITHOUT producer-side scrubbing — the assembler's egress scrub is
    // the last line of defense (chunk #88 precedent).
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("canary.db");
    let canary = "leak-canary@example.com";
    {
        let writer = open_writer(db_path.clone());
        let persistence = CorpusIncidentPersistence::new(writer);
        persistence
            .save_new_incident(&incident(
                &fp_hex(),
                WORKSPACE,
                &format!("contact {canary} for help"),
                NOW - DAY_NANOS,
            ))
            .expect("save canary incident");
    }
    let writer = open_writer(db_path);
    let assembler = build_session_b_assembler(writer);
    let digest = assembler
        .assemble(
            CadenceMode::Tier2,
            None,
            &project_context(),
            NOW,
            Duration::from_secs(60),
        )
        .await
        .expect("assemble succeeds");
    assert_eq!(digest.corpus_matches.len(), 1);
    assert!(
        !digest.corpus_matches[0].contains(canary),
        "egress scrub must redact the canary: {}",
        digest.corpus_matches[0]
    );
    assert!(
        !digest.payload_summary.contains(canary),
        "canary must not reach the LLM-facing payload"
    );
}

/// Minimal capturing subscriber (testing.md 2026-05-07 + 2026-05-11
/// field-collector extension) — captures (target, concatenated field
/// values) tuples for the negative log-grep assertion.
mod capture {
    use std::sync::{Arc, Mutex};
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    pub type Captured = Arc<Mutex<Vec<(String, String)>>>;

    pub struct CapturingSubscriber {
        pub events: Captured,
    }

    struct FieldCollector {
        sink: String,
    }

    impl Visit for FieldCollector {
        fn record_debug(&mut self, _field: &Field, value: &dyn std::fmt::Debug) {
            self.sink.push_str(&format!("{value:?} "));
        }
        fn record_str(&mut self, _field: &Field, value: &str) {
            self.sink.push_str(value);
            self.sink.push(' ');
        }
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _span: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _span: &Id, _values: &Record<'_>) {}
        fn record_follows_from(&self, _span: &Id, _follows: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let mut collector = FieldCollector {
                sink: String::new(),
            };
            event.record(&mut collector);
            self.events
                .lock()
                .expect("lock")
                .push((event.metadata().target().to_string(), collector.sink));
        }
        fn enter(&self, _span: &Id) {}
        fn exit(&self, _span: &Id) {}
    }
}

#[tokio::test]
async fn retrieval_events_carry_no_fingerprints_workspaces_or_incident_text() {
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("loggrep.db");
    let secret_title = "[redacted] unique-incident-title-canary";
    {
        let writer = open_writer(db_path.clone());
        let persistence = CorpusIncidentPersistence::new(writer);
        persistence
            .save_new_incident(&incident(
                &fp_hex(),
                WORKSPACE,
                secret_title,
                NOW - DAY_NANOS,
            ))
            .expect("save incident");
    }
    let writer = open_writer(db_path);
    let assembler = build_session_b_assembler(writer);

    let events: capture::Captured = Arc::new(Mutex::new(Vec::new()));
    let subscriber = capture::CapturingSubscriber {
        events: Arc::clone(&events),
    };
    let guard = tracing::subscriber::set_default(subscriber);
    let digest = assembler
        .assemble(
            CadenceMode::Tier2,
            None,
            &project_context(),
            NOW,
            Duration::from_secs(60),
        )
        .await
        .expect("assemble succeeds");
    drop(guard);
    assert_eq!(digest.corpus_matches.len(), 1, "retrieval exercised");

    let captured = events.lock().expect("lock");
    let retrieve_events: Vec<&(String, String)> = captured
        .iter()
        .filter(|(t, _)| t == "digest.corpus.retrieve")
        .collect();
    assert!(
        !retrieve_events.is_empty(),
        "retrieval event must fire with aggregate metadata"
    );
    for (target, fields) in captured.iter() {
        assert!(
            !fields.contains(&fp_hex()),
            "fingerprint leaked into tracing fields ({target}): {fields}"
        );
        assert!(
            !fields.contains("unique-incident-title-canary"),
            "incident title leaked into tracing fields ({target}): {fields}"
        );
        if target == "digest.corpus.retrieve" {
            assert!(
                !fields.contains(WORKSPACE),
                "workspace filter string leaked into the retrieval event: {fields}"
            );
        }
    }
}

#[test]
fn boot_purge_wiring_reports_zero_on_fresh_corpus() {
    let tmp = TempDir::new().expect("tmp");
    let writer = open_writer(tmp.path().join("purge.db"));
    let purged = run_pipeline_metrics_purge(writer.as_ref(), NOW);
    assert_eq!(purged, 0, "fresh corpus has nothing to purge");
}
