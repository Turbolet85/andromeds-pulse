//! Chunk #92 integration test: an incident produced from an L4Output by
//! `create_incident_from_l4_output` persists across a simulated app restart
//! with its originating cue `scope_id` intact — the activation evidence for
//! the chunk #91 per-service-severity join (the produced incident attributes
//! to its service, so `services.list_with_states` resolves a non-baseline
//! hue rather than the runtime-inert baseline).
//!
//! Mirrors chunk #87 `integration_findings_counter_persists_across_restart.rs`.
//! Lives in pulse-app/tests/ per testing.md 2026-05-20 (`[lib] test = false`).

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::inference_runtime::create_incident_from_l4_output;
use tempfile::TempDir;
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    IncidentPersistence, IncidentRegistry, IncidentStatus, PriorityTier,
    Severity as IncidentSeverity,
};

const WORKSPACE: &str = "ws-producer-restart";
const KEYCHAIN_KEY: [u8; 32] = [0xA3u8; 32];

fn digest_with_service_cue(service: &str) -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 512,
        payload_summary: "WINDOW ... SERVICES ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: WORKSPACE.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::ErrorRateSpike,
            priority_tier: PriorityTier::Autonomous,
            summary: "error_rate_spike".to_string(),
            scope: CueScope::Service,
            fingerprint: None,
            scope_id: Some(service.to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn surface_output() -> L4Output {
    L4Output {
        schema_version: "2.0".into(),
        prompt_version: "v2.1".into(),
        decision: Decision::Surface,
        severity: L4Severity::Autonomous,
        title: "[summary] elevated error rate".into(),
        symptom: "[summary] 12% errors vs 0.8% baseline".into(),
        timeline: "[summary] onset at T-3m".into(),
        hypotheses: vec![],
        investigation_steps: vec![],
        evidence_refs: vec!["fp-x".into()],
        fingerprint: "producer-fp".into(),
        model_tier: "primary".into(),
        hardware_profile: "gpu_primary".into(),
        is_resolution_summary: false,
    }
}

#[test]
fn produced_incident_persists_across_restart_with_scope_id() {
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("producer-restart.db");
    let service = "checkout-service";

    // First session: run the producer against a real corpus persistence.
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

        let digest = digest_with_service_cue(service);
        let output = surface_output();
        create_incident_from_l4_output(
            registry.as_ref(),
            persistence.as_ref(),
            &digest,
            &output,
            7_000,
        );

        // Live registry already carries the incident (the activation path:
        // ServicesApiImpl reads this registry for the per-service join).
        let active = registry.list_active(WORKSPACE);
        assert_eq!(active.len(), 1, "producer inserted one active incident");
        assert_eq!(active[0].scope_id.as_deref(), Some(service));
        assert_eq!(active[0].severity, IncidentSeverity::Error);
    }

    // Second session (simulated restart): reopen corpus, rehydrate registry.
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
    assert_eq!(loaded.len(), 1, "produced incident survived restart");
    let inc = &loaded[0];
    assert_eq!(
        inc.scope_id.as_deref(),
        Some(service),
        "scope_id round-trips through corpus serde — chunk #91 per-service-severity activation evidence",
    );
    assert_eq!(inc.status, IncidentStatus::Active);
    assert_eq!(inc.kind, CueKind::ErrorRateSpike);
    assert_eq!(inc.severity, IncidentSeverity::Error);
}
