//! L3 digest assembler module (chunk #81 — Epoch 9 Foundation v0.2.0).
//!
//! Composes a structured digest per dist-arch v3 §Appendix C from L1a
//! Q1-Q7 SQL outputs (chunk #79), L2 attention cues (chunk #62), corpus
//! retrieval top-3 similar past incidents (chunk #68 substrate), and
//! project context (workspace + git activity via chunk #43 + new
//! ProjectContextProvider trait). Token-budget bounded 500-2000 soft /
//! 3000 hard cap via Hugging Face `tokenizers` (Llama-3 fixture
//! downloaded at build time per `crates/triage/build.rs` Phase 2).
//!
//! Output channels:
//! - `pulse://stream/digests` broadcast topic (LWW queue for L4)
//! - Corpus `digest_archive` append (all digests; no LWW)
//!
//! Queue behavior:
//! - LWW for cadence-mode digests by default
//! - Active-incident exception bypasses LWW when L5 has unresolved
//!   incident with severity ≥ Suggested (capability P-059)
//! - Tier-1 hard signals never LWW-replaced (capped at 3)
//! - Active-incident bypass queue capped at 5 per workspace
//!
//! Capabilities enabled: P-031 foundation (Report Structure — digest is
//! precursor), P-032 (Project Context Grounding), P-044 (Retrieval-
//! Augmented Interpretation), P-059 (Active-Incident Interpretation
//! Continuity).

pub(crate) mod assembler;
pub(crate) mod broadcast;
pub(crate) mod damper;
pub(crate) mod queue;
pub(crate) mod retrieval;

pub use assembler::{Assembler, DigestAssembler, DigestFuture};
#[doc(hidden)]
pub use assembler::{
    CORPUS_MATCHES_FRAMING_NOTE, TRIGGER_LINE_PREFIX, cue_summary, render_payload,
};
pub use broadcast::{BROADCAST_CAPACITY, DigestBroadcast, STREAM_NAME_DIGESTS};
pub use damper::{
    DAMPER_CUE_EVICTION_SECONDS, DAMPER_INTERVAL_EVICTION_SECONDS, DamperVerdict, GenerateReason,
    GenerationDamper, generate_reason_label,
};
pub use queue::{ACTIVE_INCIDENT_QUEUE_CAP, LwwQueue, QueueAction, TIER1_QUEUE_CAP};
pub use retrieval::{
    CORPUS_RETRIEVAL_WINDOW_SECONDS, CorpusIncidentSource, NoopCorpusIncidentSource,
    RetrievalError, RetrievalFuture, format_corpus_match_line, select_corpus_matches,
    select_previously_seen,
};

use thiserror::Error;

/// Token budget — soft minimum per dist-arch v3 §L3 ("Target 500-2000").
pub const DIGEST_TOKEN_BUDGET_SOFT_MIN: usize = 500;
/// Token budget — soft maximum per dist-arch v3 §L3 ("Target 500-2000").
pub const DIGEST_TOKEN_BUDGET_SOFT_MAX: usize = 2000;
/// Token budget — hard cap per dist-arch v3 §L3 ("Hard cap 3000").
/// Exceeding triggers truncation strategy (drop CORPUS MATCHES last,
/// then TOP EXCEPTIONS lowest-severity, etc.) + emits an `error`-level
/// `digest.token.count.validate` event.
pub const DIGEST_TOKEN_BUDGET_HARD_CAP: usize = 3000;

/// Default ranking depth for corpus retrieval top-N similar past
/// incidents — top-5 per capability spec P-044 §Boundary ("up to N most
/// relevant entries (default 5)"; governs over dist-arch v3 §L3's
/// earlier top-3 sketch per the 2026-06-12 capability-audit remediation
/// defaults).
pub const DIGEST_CORPUS_RETRIEVAL_LIMIT: usize = 5;

/// Tracing target — top-level digest assembly span.
pub const TARGET_DIGEST_ASSEMBLE: &str = "digest.assemble.request";
/// Tracing target — LWW drop event (with `drop_reason`, `cadence_tier`).
pub const TARGET_DIGEST_LWW_DROP: &str = "digest.lww.drop";
/// Tracing target — LWW replace event (with `active_incident_bypass`).
pub const TARGET_DIGEST_LWW_REPLACE: &str = "digest.lww.replace";
/// Tracing target — token-count budget validation event.
pub const TARGET_DIGEST_TOKEN_COUNT_VALIDATE: &str = "digest.token.count.validate";
/// Tracing target — corpus retrieval boundary call (top-N by fingerprint).
pub const TARGET_DIGEST_CORPUS_RETRIEVE: &str = "digest.corpus.retrieve";

/// Metric target — per-event token-count distribution (raw events;
/// agent computes p99 via `jq` post-test aggregation per obs plan §5).
pub const TARGET_METRIC_DIGEST_TOKEN_COUNT_MS: &str = "metric.pipeline.l3.digest_token_count_ms";
/// Metric target — LWW drop counter (cumulative count increment).
pub const TARGET_METRIC_LWW_DROP_COUNT_TOTAL: &str = "metric.pipeline.l3.lww_drop_count_total";
/// Metric target — active-incident queue depth gauge.
pub const TARGET_METRIC_ACTIVE_INCIDENT_QUEUE_DEPTH: &str =
    "metric.pipeline.l3.active_incident_queue_depth";

/// Errors raised during digest assembly. Sanitized at the binary boundary
/// adapter (`pulse-app/src/digest_runtime.rs`) into `AppError` via
/// free-fn `map_err` pattern (CLAUDE.md §Session Learnings 2026-05-18) if
/// a future TauRPC procedure exposes digest operations.
#[derive(Debug, Error)]
pub enum DigestError {
    /// L1a SQL query failed (chunk #79 `SqlAggregationError`).
    #[error("L1a SQL aggregation failed")]
    SqlAggregation,
    /// Corpus read or write operation failed.
    #[error("corpus operation failed")]
    Corpus,
    /// Tokenizer initialization failed (Llama-3 fixture load failure
    /// at chunk #81 substrate). Build.rs downloads the fixture; runtime
    /// loads via `include_bytes!` so this is only surfaced if the
    /// embedded bytes deserialize fails (e.g., tokenizers crate upgrade
    /// broke schema).
    #[error("tokenizer initialization failed: {0}")]
    Tokenizer(String),
    /// Workspace-detector failed to provide project context (chunk #43
    /// `workspace_detector::Error`).
    #[error("workspace-detector failed")]
    WorkspaceDetector,
    /// Token budget exceeded after truncation strategy applied. Surfaced
    /// for observability but the digest is still emitted with truncation.
    /// Hard panics on producer side are avoided; consumers handle
    /// over-budget digests gracefully.
    #[error("token budget exceeded after truncation: {actual} > {limit}")]
    TokenBudgetExceeded { actual: usize, limit: usize },
}
