//! Corpus-backed incident source (capability P-044) + pipeline-metrics
//! retention purge (capability P-041) — chunk #100.
//!
//! `CorpusBackedIncidentSource` implements the triage-declared
//! `CorpusIncidentSource` seam over `corpus::contract::CorpusWriter` at
//! the binary boundary, preserving the arch §Module dependency direction
//! DAG (triage stays corpus-free). Mirrors the chunk #78
//! `CorpusIncidentPersistence` adapter shape, including the
//! column-over-BLOB re-stamping discipline.
//!
//! The purge runner enforces the P-041 30-day pipeline-metrics retention
//! at boot. The corpus-side DELETE preserves the newest row per
//! `(metric_name, layer)` series (see `CorpusWriter::
//! purge_pipeline_metrics_older_than`) so baseline / Drain / storm
//! snapshots survive idle gaps longer than the window.

use std::sync::Arc;
use std::time::Instant;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use triage::contract::{
    CorpusIncidentSource, Incident, RetrievalError, RetrievalFuture, TARGET_DIGEST_CORPUS_RETRIEVE,
};

/// P-041 pipeline-metrics retention window — 30 days per capability spec
/// ("pipeline operational metrics per P-058 with 30-day default
/// retention"); matches the Diagnostics read-window assumption. Code
/// constant by design — the env-var namespace is registry-closed per
/// arch §Occupied Resources.
pub const PIPELINE_METRICS_RETENTION_SECONDS: i64 = 30 * 24 * 60 * 60;

/// Tracing target for the boot-time purge event (aggregate-only fields:
/// `purged_row_count` / `retention_window_days` / `duration_ms` /
/// `error_category`).
pub const TARGET_PIPELINE_METRICS_PURGE: &str = "corpus.pipeline_metrics.purge";

/// Production `CorpusIncidentSource` over the corpus writer. Cheap to
/// clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusBackedIncidentSource {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusBackedIncidentSource {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl CorpusIncidentSource for CorpusBackedIncidentSource {
    fn load_candidates<'a>(
        &'a self,
        workspace: &'a str,
        since_unix_nano: i64,
    ) -> RetrievalFuture<'a, Vec<Incident>> {
        Box::pin(async move {
            // Synchronous rusqlite read inside the async seam — same shape
            // as the incident persistence calls the TauRPC resolvers make;
            // the candidate set is bounded by the 30-day workspace window.
            let started = Instant::now();
            let rows = self
                .writer
                .load_incidents_for_workspace_since(workspace, since_unix_nano)
                .map_err(|err| {
                    tracing::warn!(
                        target: TARGET_DIGEST_CORPUS_RETRIEVE,
                        query_id = "incidents_for_workspace_since",
                        error_category = corpus_error_label(&err),
                        duration_ms = started.elapsed().as_millis() as u64,
                        "corpus candidate query failed",
                    );
                    RetrievalError::SourceFailed
                })?;
            let mut candidates = Vec::with_capacity(rows.len());
            for row in rows {
                let mut incident: Incident = bincode::deserialize::<Incident>(&row.payload)
                    .map_err(|_| {
                        tracing::warn!(
                            target: TARGET_DIGEST_CORPUS_RETRIEVE,
                            query_id = "incidents_for_workspace_since",
                            error_category = "decode_failed",
                            duration_ms = started.elapsed().as_millis() as u64,
                            "corpus candidate decode failed",
                        );
                        RetrievalError::SourceFailed
                    })?;
                incident.id = row.id;
                incident.workspace = row.workspace;
                incident.opened_at_unix_nano = row.created_unix_nano;
                incident.updated_at_unix_nano = row.updated_unix_nano;
                incident.resolved_at_unix_nano = row.resolved_unix_nano;
                incident.read_at_unix_nano = row.read_unix_nano;
                candidates.push(incident);
            }
            Ok(candidates)
        })
    }
}

/// Run the boot-time P-041 retention purge. Failure is non-fatal (the
/// purge re-runs at next boot); the event is the observable signal either
/// way. Returns the purged row count.
pub fn run_pipeline_metrics_purge(writer: &dyn CorpusWriter, now_unix_nano: i64) -> u64 {
    let cutoff = now_unix_nano
        .saturating_sub(PIPELINE_METRICS_RETENTION_SECONDS.saturating_mul(1_000_000_000));
    let started = Instant::now();
    match writer.purge_pipeline_metrics_older_than(cutoff) {
        Ok(purged_row_count) => {
            tracing::info!(
                target: TARGET_PIPELINE_METRICS_PURGE,
                purged_row_count,
                retention_window_days = 30_u64,
                duration_ms = started.elapsed().as_millis() as u64,
                "pipeline metrics retention purge complete",
            );
            purged_row_count
        }
        Err(err) => {
            tracing::warn!(
                target: TARGET_PIPELINE_METRICS_PURGE,
                error_category = corpus_error_label(&err),
                retention_window_days = 30_u64,
                duration_ms = started.elapsed().as_millis() as u64,
                "pipeline metrics retention purge failed",
            );
            0
        }
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

// Unit + integration coverage lives at
// `pulse-app/tests/integration_corpus_retrieval_two_session.rs` per
// session-learnings 2026-05-13/2026-05-20 — `[lib] test = false` makes
// source-level `mod tests` dead weight in this crate.
