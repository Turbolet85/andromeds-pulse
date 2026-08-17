//! Regression proof for constellation severity live-wiring (P-079): the
//! incident FILTER key and the incident STAMP (`digest.workspace`) are
//! single-sourced through `resolve_workspace_for_incidents`, so a live
//! storm's incidents are found by `list_active`. Before the fix the filter
//! used `data_dir` while the producer stamped the detected project root, so
//! `list_active` returned zero and every constellation dot read healthy and
//! the incidents panel stayed empty. (verification-matrix.json#P-079)
//!
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20 ([lib] test = false).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use interpretation::contract::ModelTier;
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::digest_runtime::{resolve_workspace_for_incidents, workspace_to_digest_context};
use pulse_app::inference_runtime::{
    L4DigestOutcome, create_incident_from_l4_output, handle_digest_outcome,
};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    Incident, IncidentError, IncidentPersistence, IncidentRegistry, PriorityTier,
    Severity as IncidentSeverity,
};
use workspace_detector::contract::WorkspaceContext;

fn sample_ctx(root: &str) -> WorkspaceContext {
    WorkspaceContext {
        root: PathBuf::from(root),
        project_name: Some("payments".to_string()),
        vcs: None,
        has_andromeda_marker: true,
    }
}

/// Parity: on a detected workspace the filter key and the producer's
/// stamped path are ONE derivation, and the detected root wins over
/// `data_dir` (the pre-fix filter source).
#[test]
fn resolver_key_equals_producer_workspace_on_detected() {
    let ctx = sample_ctx("/home/dev/payments");
    let data_dir = Path::new("/var/data/andromeda-pulse");
    let (key, context) = resolve_workspace_for_incidents(Some(&ctx), data_dir);
    assert_eq!(key, context.workspace_canonical_path);
    assert_eq!(
        key,
        workspace_to_digest_context(&ctx).workspace_canonical_path
    );
    assert_ne!(key, data_dir.to_string_lossy());
    assert_eq!(key, "/home/dev/payments");
}

/// Parity holds for a Windows extended-length (`\\?\`) canonical root — the
/// exact string-equality trap where a re-canonicalized side would diverge
/// (session-learnings 2026-06-04). Both halves use `root.to_string_lossy()`.
#[test]
fn resolver_preserves_windows_extended_length_prefix() {
    let ctx = sample_ctx(r"\\?\C:\dev\payments");
    let data_dir = Path::new(r"C:\Users\dev\AppData\Roaming\andromeda-pulse");
    let (key, context) = resolve_workspace_for_incidents(Some(&ctx), data_dir);
    assert_eq!(key, context.workspace_canonical_path);
    assert_eq!(key, r"\\?\C:\dev\payments");
}

/// Fallback parity: when detection fails, BOTH halves fall back to
/// `data_dir` so the filter key still equals the stamped workspace.
#[test]
fn resolver_falls_back_to_data_dir_on_both_halves() {
    let data_dir = Path::new("/var/data/andromeda-pulse");
    let (key, context) = resolve_workspace_for_incidents(None, data_dir);
    assert_eq!(key, data_dir.to_string_lossy());
    assert_eq!(key, context.workspace_canonical_path);
}

// ---- end-to-end storm wired through the resolver ----

const STORM_SERVICE: &str = "payment-service";
const STORM_EVENTS: usize = 150;

/// Recording `IncidentPersistence` double — assigns rowids, captures saves.
#[derive(Default)]
struct RecordingPersistence {
    saved: Mutex<Vec<Incident>>,
    next_id: Mutex<i64>,
}

impl IncidentPersistence for RecordingPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        let mut next = self.next_id.lock().expect("lock");
        *next += 1;
        self.saved.lock().expect("lock").push(incident.clone());
        Ok(*next)
    }
    fn update_incident_status(&self, _id: i64, _payload: &Incident) -> Result<(), IncidentError> {
        Ok(())
    }
    fn mark_read(&self, _id: i64, _read_unix_nano: i64) -> Result<(), IncidentError> {
        Ok(())
    }
    fn load_active_incidents(&self, _workspace: &str) -> Result<Vec<Incident>, IncidentError> {
        Ok(vec![])
    }
    fn load_incidents_for_workspace_since(
        &self,
        _workspace: &str,
        _since_unix_nano: i64,
    ) -> Result<Vec<Incident>, IncidentError> {
        Ok(vec![])
    }
    fn count_active_unread(&self, _workspace: &str) -> Result<u64, IncidentError> {
        Ok(0)
    }
    fn save_incident_event(
        &self,
        _incident_id: i64,
        _event_kind: &str,
        _occurred_at: i64,
    ) -> Result<(), IncidentError> {
        Ok(())
    }
}

/// One storm digest whose `workspace` is stamped from the producer context
/// (mirrors the P-074 cue-bearing Tier-1 storm digest shape).
fn storm_digest(workspace: &str, seq: usize) -> Digest {
    Digest {
        kind: DigestKind::CadenceTier1,
        token_count: 512,
        payload_summary: "WINDOW 60s, tier1 cadence\nSERVICES payment-service".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000 + seq as i64,
        workspace: workspace.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::RetryStorm,
            priority_tier: PriorityTier::Autonomous,
            summary: "retry storm".to_string(),
            scope: CueScope::Service,
            fingerprint: None,
            scope_id: Some(STORM_SERVICE.to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Tier1NeverLww,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

/// End-to-end: derive BOTH the producer workspace and the filter key from
/// the ONE resolver (exactly the single-source wiring main.rs uses at boot),
/// drive a storm, and assert the FILTER key finds the incident. Zero here
/// would be the pre-fix data_dir-vs-detected-root mismatch.
#[tokio::test]
async fn storm_incident_is_found_by_the_resolver_derived_filter_key() {
    let ctx = sample_ctx("/home/dev/payments");
    let data_dir = Path::new("/var/data/andromeda-pulse");
    let (filter_key, project_context) = resolve_workspace_for_incidents(Some(&ctx), data_dir);

    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    let registry = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    for seq in 0..STORM_EVENTS {
        let digest = storm_digest(&project_context.workspace_canonical_path, seq);
        let parsed = match handle_digest_outcome(&runner, &digest).await {
            L4DigestOutcome::Success(p) => p,
            other => panic!("expected Success, got {other:?}"),
        };
        create_incident_from_l4_output(
            registry.as_ref(),
            persistence.as_ref(),
            &digest,
            &parsed,
            5_000 + seq as i64,
        );
    }

    let active = registry.list_active(&filter_key);
    assert_eq!(
        active.len(),
        1,
        "the storm incident must be found by the resolver-derived filter key (zero = the P-079 mismatch)",
    );
    assert_eq!(active[0].scope_id.as_deref(), Some(STORM_SERVICE));
    assert_eq!(
        active[0].severity,
        IncidentSeverity::Error,
        "autonomous storm → non-healthy (red-dot) severity",
    );
}

/// No-regression (P-067): with no incidents the resolver-derived key finds
/// nothing (all-healthy), never a false positive.
#[tokio::test]
async fn zero_incident_state_stays_empty() {
    let ctx = sample_ctx("/home/dev/payments");
    let data_dir = Path::new("/var/data/andromeda-pulse");
    let (filter_key, _context) = resolve_workspace_for_incidents(Some(&ctx), data_dir);
    let registry = InMemoryIncidentRegistry::new();
    assert!(registry.list_active(&filter_key).is_empty());
}

// ---- cross-process parity: what the app stamps is what the sidecar filters ----

/// The alignment contract. The app's resolver key, published at boot, is the
/// byte-identical string the sidecar reads back — so a `query_incident_list`
/// against a shared data dir sees the rows the app stamped. Before this,
/// the sidecar derived `data_dir` while the app stamped the detected root,
/// and every sidecar incident query came back empty.
#[test]
fn published_key_read_by_the_sidecar_equals_the_app_resolver_key() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let data_dir = tmp.path();
    let ctx = sample_ctx(r"\\?\C:\dev\payments");

    let (app_key, context) = resolve_workspace_for_incidents(Some(&ctx), data_dir);
    workspace_detector::contract::publish_workspace_key(data_dir, &app_key).expect("publish");

    let sidecar_key = workspace_detector::contract::read_published_workspace_key(data_dir)
        .expect("sidecar reads the published key");

    assert_eq!(sidecar_key, app_key);
    assert_eq!(
        sidecar_key, context.workspace_canonical_path,
        "the sidecar's filter key must equal the value the producer stamps",
    );
    assert_ne!(
        sidecar_key,
        data_dir.to_string_lossy(),
        "a detected workspace must not collapse back to the data dir",
    );
}

/// Fallback parity: with nothing published (no app has run against this data
/// dir), the sidecar's key is `data_dir` — which is exactly what the app's
/// resolver yields when detection fails, so the two still agree.
#[test]
fn absent_published_key_falls_back_to_data_dir_on_both_ends() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let data_dir = tmp.path();

    assert_eq!(
        workspace_detector::contract::read_published_workspace_key(data_dir),
        None,
    );
    let sidecar_key = workspace_detector::contract::read_published_workspace_key(data_dir)
        .unwrap_or_else(|| data_dir.to_string_lossy().to_string());

    let (app_key, _context) = resolve_workspace_for_incidents(None, data_dir);
    assert_eq!(sidecar_key, app_key);
}
