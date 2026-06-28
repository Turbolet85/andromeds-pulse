//! L3 digest runtime adapter (chunk #81).
//!
//! Constructs the `triage::digest::Assembler` from injected deps + spawns
//! two tasks:
//! - **Cadence subscriber:** subscribes to `DigestTriggerBroadcast` and
//!   invokes `assembler.assemble(...)` per trigger, passing the triggering
//!   cue THROUGH so the digest carries `attention_cues` and the L4 producer
//!   can create a cue-derived incident (P-074). The PII-free
//!   `pulse://stream/cadence-events` L6 topic stays a separate channel.
//! - **Digest persister:** subscribes to `DigestBroadcast` (chunk #81)
//!   AND persists each emitted digest к corpus via
//!   `CorpusWriter::save_digest` (chunk #81 trait extension). Bincode-
//!   serializes the Digest struct; AES-256-GCM cell-level encryption
//!   happens inside the CorpusWriter impl.
//!
//! Project context construction: maps `workspace_detector::detect()`
//! output к `triage::contract::DigestProjectContext` (recent commits +
//! framework signals are empty Vec at chunk #81 substrate; future
//! chunk enriches via filesystem `.git/refs/heads/{branch}` walk +
//! framework signal detection from `Cargo.toml` / `package.json`).
//!
//! PII scrubbing: injects а closure wrapping
//! `security::scrubber::scrub_attribute` per CLAUDE.md §Session Learnings
//! 2026-05-20 cross-crate `scrubbed_clone` pattern.
//!
//! Error mapping: free-fn `corpus_error_to_digest_runtime_error` follows
//! the chunk #68 `corpus_error_to_app_error` precedent (CLAUDE.md §Session
//! Learnings 2026-05-18).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use security::scrubber::{ScrubbedValue, scrub_attribute};
use tokio::sync::broadcast::error::RecvError;
use triage::contract::{
    Assembler, CadenceMode, CorpusIncidentSource, Digest, DigestAssembler, DigestBroadcast,
    DigestProjectContext, DigestRecentCommit, DigestTriggerBroadcast, IncidentRegistry, LwwQueue,
    SqlQueryRunner, mode_label,
};
use workspace_detector::contract::WorkspaceContext;

use corpus::contract::{CorpusReader, CorpusWriter, Error as CorpusError};

/// Tracing target — digest persister boundary log.
const TARGET_DIGEST_RUNTIME_PERSIST: &str = "digest.runtime.persist";
/// Tracing target — cadence subscriber tick log.
const TARGET_DIGEST_RUNTIME_CADENCE_TICK: &str = "digest.runtime.cadence_tick";

/// Construct а PII-scrubbing closure suitable for
/// `Assembler::new`'s `Arc<dyn Fn(&str) -> String + Send + Sync>` slot.
/// Wraps `security::scrubber::scrub_attribute` per CLAUDE.md §Session
/// Learnings 2026-05-20.
pub fn pii_scrub_closure() -> Arc<dyn Fn(&str) -> String + Send + Sync> {
    Arc::new(|val: &str| match scrub_attribute(val) {
        ScrubbedValue::Allowed(s) => s,
        ScrubbedValue::Redacted { category } => format!("[redacted:{category}]"),
    })
}

/// Build the digest assembler. Caller injects all dependencies; this fn
/// pairs the tokenizer load (from build-time fixture) с the scrub
/// closure construction. `corpus_source` supplies P-044 retrieval
/// candidates (`NoopCorpusIncidentSource` when the corpus is absent at
/// boot — digest assembly degrades to empty `corpus_matches`). Returns
/// `Arc<dyn DigestAssembler>` ready for trait-injection elsewhere.
pub fn build_assembler(
    sql_runner: Arc<dyn SqlQueryRunner>,
    incident_registry: Arc<dyn IncidentRegistry>,
    broadcast: Arc<DigestBroadcast>,
    queue: Arc<Mutex<LwwQueue>>,
    corpus_source: Arc<dyn CorpusIncidentSource>,
) -> Result<Arc<dyn DigestAssembler>, String> {
    let scrub = pii_scrub_closure();
    let assembler = Assembler::new(
        sql_runner,
        incident_registry,
        broadcast,
        queue,
        scrub,
        corpus_source,
    )
    .map_err(|e| format!("digest assembler init failed: {e}"))?;
    Ok(Arc::new(assembler))
}

/// Map а `workspace_detector::WorkspaceContext` к the L3 assembler's
/// expected `DigestProjectContext` value type. Chunk #81 substrate:
/// recent_commits + framework_signals empty (deferred к future chunk
/// per chunk #81 implementation notes).
pub fn workspace_to_digest_context(ctx: &WorkspaceContext) -> DigestProjectContext {
    DigestProjectContext {
        workspace_canonical_path: ctx.root.to_string_lossy().into_owned(),
        project_name: ctx.project_name.clone(),
        vcs_type: ctx.vcs.as_ref().map(|_| "git"),
        recent_commits: Vec::<DigestRecentCommit>::new(),
        framework_signals: Vec::new(),
    }
}

/// Spawn the cadence subscriber task. Listens on `cadence_broadcast`,
/// invokes assembler on each event. Project context derived once per
/// task invocation from а static `WorkspaceContext` (chunk #81 substrate
/// uses а fixed context; future chunk re-detects per event if workspace
/// changes mid-session, currently rare).
pub fn spawn_cadence_subscriber(
    digest_trigger: Arc<DigestTriggerBroadcast>,
    assembler: Arc<dyn DigestAssembler>,
    project_context: DigestProjectContext,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut rx = digest_trigger.subscribe();
        loop {
            match rx.recv().await {
                Ok(trigger) => {
                    let mode = trigger.mode;
                    let window = cycle_window_for_mode(mode);
                    let now_nanos = trigger.executed_at_unix_nano;
                    tracing::info!(
                        target: TARGET_DIGEST_RUNTIME_CADENCE_TICK,
                        mode = mode_label(mode),
                        cue_present = trigger.triggering_cue.is_some(),
                        "digest trigger observed; invoking digest assembler",
                    );
                    let result = assembler
                        .assemble(
                            mode,
                            trigger.triggering_cue.as_ref(),
                            &project_context,
                            now_nanos,
                            window,
                        )
                        .await;
                    if let Err(e) = result {
                        tracing::warn!(
                            target: TARGET_DIGEST_RUNTIME_CADENCE_TICK,
                            error_category = digest_error_label(&e),
                            "digest assembly failed",
                        );
                    }
                }
                Err(RecvError::Lagged(skipped)) => {
                    tracing::warn!(
                        target: TARGET_DIGEST_RUNTIME_CADENCE_TICK,
                        skipped_events = skipped,
                        "digest trigger subscriber lagged; resubscribing",
                    );
                    rx = digest_trigger.subscribe();
                }
                Err(RecvError::Closed) => return,
            }
        }
    })
}

/// Spawn the digest persister task. Listens on `digest_broadcast`,
/// serializes each digest к bincode, persists к corpus via
/// `CorpusWriter::save_digest`. Errors from corpus writes are logged
/// and dropped (digest is still broadcast к L4 consumers).
pub fn spawn_digest_persister(
    digest_broadcast: Arc<DigestBroadcast>,
    corpus_writer: Arc<dyn CorpusWriter>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut rx = digest_broadcast.subscribe();
        loop {
            match rx.recv().await {
                Ok(digest) => {
                    let payload = match bincode::serialize(&digest) {
                        Ok(b) => b,
                        Err(e) => {
                            tracing::warn!(
                                target: TARGET_DIGEST_RUNTIME_PERSIST,
                                error_category = "bincode_serialize_failed",
                                error_message = %e,
                                "digest bincode serialize failed",
                            );
                            continue;
                        }
                    };
                    let kind_label = digest_kind_label(&digest);
                    match corpus_writer.save_digest(
                        kind_label,
                        digest.generated_at_unix_nano,
                        digest.token_count as i64,
                        &payload,
                    ) {
                        Ok(rowid) => {
                            tracing::info!(
                                target: TARGET_DIGEST_RUNTIME_PERSIST,
                                rowid,
                                token_count = digest.token_count as u64,
                                digest_kind = kind_label,
                                "digest persisted к corpus",
                            );
                        }
                        Err(e) => {
                            tracing::warn!(
                                target: TARGET_DIGEST_RUNTIME_PERSIST,
                                error_category = corpus_error_label(&e),
                                digest_kind = kind_label,
                                "digest persist к corpus failed",
                            );
                        }
                    }
                }
                Err(RecvError::Lagged(skipped)) => {
                    tracing::warn!(
                        target: TARGET_DIGEST_RUNTIME_PERSIST,
                        skipped_events = skipped,
                        "digest persister lagged; resubscribing",
                    );
                    rx = digest_broadcast.subscribe();
                }
                Err(RecvError::Closed) => return,
            }
        }
    })
}

fn cycle_window_for_mode(mode: CadenceMode) -> Duration {
    match mode {
        CadenceMode::Tier1 | CadenceMode::Tier2 | CadenceMode::Tier3 => Duration::from_secs(60),
        CadenceMode::Reflection => Duration::from_secs(1800),
    }
}

fn digest_kind_label(d: &Digest) -> &'static str {
    use triage::contract::DigestKind;
    match d.kind {
        DigestKind::Snapshot => "snapshot",
        DigestKind::IncidentSummary => "incident_summary",
        DigestKind::BaselineState => "baseline_state",
        DigestKind::AttentionCueDigest => "attention_cue_digest",
        DigestKind::CadenceTier1 => "cadence_tier1",
        DigestKind::CadenceTier2 => "cadence_tier2",
        DigestKind::CadenceTier3 => "cadence_tier3",
        DigestKind::Reflection => "reflection",
        DigestKind::ResolutionSummary => "resolution_summary",
    }
}

fn digest_error_label(e: &triage::contract::DigestError) -> &'static str {
    use triage::contract::DigestError;
    match e {
        DigestError::SqlAggregation => "sql_aggregation_failed",
        DigestError::Corpus => "corpus_op_failed",
        DigestError::Tokenizer(_) => "tokenizer_init_failed",
        DigestError::WorkspaceDetector => "workspace_detector_failed",
        DigestError::TokenBudgetExceeded { .. } => "token_budget_exceeded",
    }
}

fn corpus_error_label(e: &CorpusError) -> &'static str {
    match e {
        CorpusError::KeyringUnavailable => "keyring_unavailable",
        CorpusError::MigrationFailed => "migration_failed",
        CorpusError::SchemaVersionMismatch => "schema_version_mismatch",
        CorpusError::QueryFailed => "query_failed",
        CorpusError::EncryptionFailed => "encryption_failed",
        CorpusError::DecryptionFailed => "decryption_failed",
        _ => "corpus_other",
    }
}

/// Suppress dead-code lint for `CorpusReader` import — kept available
/// because future retrieval chunks will surface а `load_recent_digests_for_workspace`
/// method here per plan step 9 deferred scope.
#[allow(dead_code)]
fn _hold_corpus_reader_ref<R: CorpusReader>(_r: &R) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pii_scrub_closure_redacts_jwt() {
        let scrub = pii_scrub_closure();
        let input = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.signaturedata123456";
        let out = scrub(input);
        assert!(out.starts_with("[redacted:"), "expected redaction: {out}");
    }

    #[test]
    fn pii_scrub_closure_allows_clean_text() {
        let scrub = pii_scrub_closure();
        let out = scrub("hello world");
        assert_eq!(out, "hello world");
    }
}
