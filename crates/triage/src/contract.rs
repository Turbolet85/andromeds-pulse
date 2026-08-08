//! Triage contract types for Pulse v0.2.0 L1 streaming distillation
//! (capability spec P-019 Three-Tier Severity Model + P-021 Algorithmic
//! Attention Cues — see `docs/v0_2_0/pulse-capability-spec.md`). Empty
//! implementations at this scaffold stage; consumed by chunks #61+.

use serde::{Deserialize, Serialize};

// Chunk #61 — streaming baseline trackers + corpus persistence.
// Re-export public-API types from `triage::baseline` so future chunks (#62
// attention cue emitter, #63 restart event detector) import one shape per
// arch §Conventions "Module visibility discipline" + chunks #58/#60 precedent.
//
// Chunk #64 extension: `ActivityFloor` + `BootstrapState` + activity-floor
// window constants + `ServiceSilenceSnapshot` + `ACTIVITY_FLOOR_SERVICE_CAP`
// + `TARGET_SERVICE_CAP_EXCEEDED` cover the new per-service activity
// histogram + ServiceWentSilent gating contract types.
pub use crate::baseline::{
    ACTIVITY_FLOOR_SERVICE_CAP, ActivityFloor, BOOTSTRAP_WINDOW_SECONDS, BUCKET_COUNT,
    BUCKET_INTERVAL_SECONDS, BaselineError, BaselinePersistence, BaselineState, BootstrapResult,
    BootstrapState, DEFAULT_ALPHA_5MIN_WINDOW, DEFAULT_MAX_SIZE_BYTES,
    DEFAULT_PERSIST_INTERVAL_NANOS, DEFAULT_SERVICE_COUNT_CAP, OperationMetricSnapshot,
    PersistStats, SCHEMA_VERSION, STATE_AGE_THRESHOLD_NANOS, ServiceMetricSnapshot,
    ServiceSilenceSnapshot, TARGET_BASELINE_MIGRATE, TARGET_BASELINE_MIGRATE_FAILED,
    TARGET_BASELINE_PERSIST_ERROR, TARGET_SERVICE_CAP_EXCEEDED, WINDOW_DURATION_SECONDS,
    bootstrap_state, persist_on_shutdown, run_persist_cycle, run_persist_loop,
};

// Chunk #62 — attention cue emitter. Re-export public-API types from
// `triage::cue` so pulse-app boot wiring + future v0.2.0 chunks import one
// shape per chunk #61 precedent.
//
// Chunk #63 extension: `DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER` +
// `DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE` + `DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS`
// + `DEFAULT_RESTART_GAP_THRESHOLD_SECONDS` +
// `DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS` +
// `DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS` cover the new
// `Thresholds` fields wired through `dual_condition_bypass` + the
// `pattern::suppression` filter.
pub use crate::cue::{
    AttentionCueBroadcast, BROADCAST_CAPACITY, CHANNEL_NAME_CADENCE_TRIGGERS,
    CadenceTriggerChannel, DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE, DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS,
    DEFAULT_BASE_ERROR_RATE, DEFAULT_BASE_LATENCY_MS, DEFAULT_BOOTSTRAP_WINDOW_SECONDS,
    DEFAULT_ERROR_RATE_MULTIPLIER, DEFAULT_LATENCY_MULTIPLIER, DEFAULT_LATENCY_PERCENTILE,
    DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER, DEFAULT_MIN_PERSISTENCE_SECONDS,
    DEFAULT_QUIET_DURATION_PERCENTILE, DEFAULT_RESTART_GAP_THRESHOLD_SECONDS,
    DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS, DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS,
    DEFAULT_TICK_INTERVAL, MIN_EWMA_SAMPLES, MIN_LATENCY_SAMPLES, STREAM_NAME_ATTENTION_CUES,
    Thresholds, ThresholdsError, classify_priority, dual_condition_bypass,
    evaluate_service_went_silent, evaluate_thresholds, run_one_emit_cycle, start_emitter,
};

// Chunk #63 — restart event detector + dual-condition bypass. Re-export
// `pattern` public-API types so pulse-app boot wiring + future v0.2.0
// chunks import one shape per chunk #61/#62 precedent. `BROADCAST_CAPACITY`
// is NOT re-exported from `pattern` here because the same const value is
// already re-exported from `cue` (both equal 32; sharing avoids ambiguity).
pub use crate::pattern::{
    BypassReason, BypassTrigger, DEFAULT_AUTONOMOUS_THRESHOLD,
    DEFAULT_DETECTION_SUB_WINDOW_SECONDS, DEFAULT_HEARTBEAT_INTERVAL,
    DEFAULT_STORM_PERSIST_INTERVAL_SECS, DEFAULT_STORM_WINDOW_SECONDS, DEFAULT_SUGGESTED_THRESHOLD,
    DetectCycleStats, FingerprintState, RestartDetector, RestartEvent, RestartEventBroadcast,
    RetryStormDetector, STORM_PERSISTENCE_KIND, STREAM_NAME_RESTART_EVENTS, StormCycleStats,
    StormError, StormPersistence, StormStateSnapshot, SuppressionOutcome, SuppressionParams,
    SuppressionState, TARGET_PATTERN_STORM_CORPUS_RESTORE, TARGET_PATTERN_STORM_PERSIST,
    TARGET_PATTERN_STORM_PERSIST_ERROR, evaluate_with_suppression, observe_and_dispatch,
    observe_and_dispatch_storm, persist_storm_on_shutdown, record_occurrence, run_one_detect_cycle,
    run_one_storm_cycle, run_storm_persist_cycle, run_storm_persist_loop, start_restart_detector,
    start_storm_detector,
};

// Chunk #78 — incident records + lifecycle persistence. Re-export
// `incident` public-API types so pulse-app boot wiring imports one shape
// per chunk #67/#71 precedent. `BROADCAST_CAPACITY` is NOT re-exported
// here because the same const value is already re-exported from `cue` +
// `pattern` + `lifecycle` (all equal 32).
pub use crate::incident::{
    DEFAULT_INCIDENT_ACK_COOLDOWN_SECS, DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS,
    DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS, INCIDENT_PERSISTENCE_KIND, InMemoryIncidentRegistry,
    IncidentError, IncidentLifecycleBroadcast, IncidentLifecycleEvent, IncidentPersistence,
    IncidentRecordPayload, IncidentRegistry, IncidentRegistryError, ResolutionTrigger,
    STREAM_NAME_INCIDENTS, TARGET_INCIDENT_PERSIST, TARGET_INCIDENT_PERSIST_ERROR,
    cooldown_expiry_unix_nano, is_valid_incident_transition, run_incident_persist_cycle,
    run_incident_persist_loop, should_auto_resolve, status_label as incident_status_label,
};

// Chunk #67 — service registry + lifecycle state machine. Re-export
// `lifecycle` public-API types so pulse-app boot wiring + future v0.2.0
// chunks import one shape per chunk #58/#61/#62/#63 precedent.
// `BROADCAST_CAPACITY` is NOT re-exported here because the same const
// value is already re-exported from `cue` + `pattern` (all equal 32).
pub use crate::lifecycle::{
    ACTIVE_TO_QUIET_THRESHOLD_SECONDS, DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL,
    DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS, InMemoryServiceRegistry, LIFECYCLE_PERSISTENCE_KIND,
    LifecycleError, LifecyclePersistence, LifecycleThresholds, QUIET_TO_SILENT_FALLBACK_SECONDS,
    STREAM_NAME_SERVICE_LIFECYCLE, ServiceLifecycleBroadcast, ServiceLifecycleEvent,
    ServiceLifecycleState, ServiceListItem, ServiceRegistry, ServiceRegistryEntry,
    TARGET_LIFECYCLE_CORPUS_RESTORE, TARGET_LIFECYCLE_PERSIST, TARGET_LIFECYCLE_PERSIST_ERROR,
    TARGET_LIFECYCLE_TICK, TARGET_LIFECYCLE_TRANSITION, TARGET_METRIC_LIFECYCLE_STATE_DISTRIBUTION,
    TARGET_PIPELINE_L1B_TRACKED_SERVICES_TOTAL, TransitionTrigger, is_valid_transition,
    persist_lifecycle_on_shutdown, reevaluate_now, run_lifecycle_persist_cycle,
    run_lifecycle_persist_loop, start_lifecycle_heartbeat, state_index, state_label,
};

// Chunk #80 — cadence coordinator + three-tier triggering. Re-export
// public-API types from `triage::cadence` so pulse-app boot wiring +
// future v0.2.0 chunks import one shape per chunk #61/#62/#63/#67/#78
// precedent. Capabilities P-052 / P-060.
pub use crate::cadence::{
    CADENCE_ACCELERATED_SECONDS_MIN, CADENCE_BASELINE_SECONDS_MIN, CADENCE_REFLECTION_SECONDS_MIN,
    CadenceConfig, CadenceConfigError, CadenceCoordinator, CadenceEvent, CadenceEventBroadcast,
    CadenceMode, CoordinatorCycleStats, DEFAULT_CADENCE_ACCELERATED_SECONDS,
    DEFAULT_CADENCE_BASELINE_SECONDS, DEFAULT_CADENCE_REFLECTION_SECONDS, DigestTrigger,
    DigestTriggerBroadcast, HardwareProfile, HardwareProfileSource, STREAM_NAME_CADENCE_EVENTS,
    SqlQueryRunner, UnknownHardwareProfile, mode_label, run_one_coordinator_cycle,
    start_cadence_coordinator,
};

// Chunk #79 — L1a SQL aggregation queries. Re-export to enable chunk #80
// Cadence Coordinator's `SqlQueryRunner` adapter at the binary boundary
// (`pulse-app/src/cadence_runner.rs`) to construct the runner over
// `TriageSqlState` without directly reaching into `triage::baseline`. Mirrors
// chunk #62 pattern of contract-as-single-import-surface for pulse-app.
pub use crate::baseline::{
    Q1RedRow, Q2OperationRow, Q3FingerprintRow, Q4InteractionRow, Q5CardinalityRow, Q6LogRow,
    Q7CriticalPathRow, SqlAggregationError, TriageSqlState, run_q1, run_q2, run_q3, run_q4, run_q5,
    run_q6, run_q7,
};

// Chunk #81 — L3 digest assembler + LWW queue + active-incident exception.
// Re-export public-API types from `triage::digest` so pulse-app boot
// wiring (digest_runtime adapter + main.rs cadence coordinator hook)
// imports one shape per chunks #61/#62/#63/#67/#78/#80 precedent.
// Capabilities P-031 / P-032 / P-044 / P-059.
pub use crate::digest::{
    ACTIVE_INCIDENT_QUEUE_CAP, Assembler, BROADCAST_CAPACITY as DIGEST_BROADCAST_CAPACITY,
    CORPUS_RETRIEVAL_WINDOW_SECONDS, CorpusIncidentSource, DIGEST_CORPUS_RETRIEVAL_LIMIT,
    DIGEST_TOKEN_BUDGET_HARD_CAP, DIGEST_TOKEN_BUDGET_SOFT_MAX, DIGEST_TOKEN_BUDGET_SOFT_MIN,
    DigestAssembler, DigestBroadcast, DigestError, DigestFuture, LwwQueue,
    NoopCorpusIncidentSource, QueueAction, RetrievalError, RetrievalFuture, STREAM_NAME_DIGESTS,
    TARGET_DIGEST_ASSEMBLE, TARGET_DIGEST_CORPUS_RETRIEVE, TARGET_DIGEST_LWW_DROP,
    TARGET_DIGEST_LWW_REPLACE, TARGET_DIGEST_TOKEN_COUNT_VALIDATE,
    TARGET_METRIC_ACTIVE_INCIDENT_QUEUE_DEPTH, TARGET_METRIC_DIGEST_TOKEN_COUNT_MS,
    TARGET_METRIC_LWW_DROP_COUNT_TOTAL, TIER1_QUEUE_CAP,
    assembler::{DigestProjectContext, DigestRecentCommit},
    format_corpus_match_line, select_corpus_matches, select_previously_seen,
};

/// Kind of detected condition emitted as an attention cue. Bounded
/// enumeration; future kinds are added explicitly (no `Other(String)`
/// catch-all). Variants serialize as snake_case strings. Chunk #78 added
/// the `Hash` derive (needed for `IncidentRegistry` cool-down map keying)
/// AND the cfg-gated `specta::Type` derive (for cross-bridge type
/// generation via `IncidentLifecycleEvent`). Chunk #98 added the synthetic
/// `ReflectionTrend` variant — the sole kind NOT emitted by a streaming
/// detector.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueKind {
    ErrorRateSpike,
    LatencyRegression,
    RestartEvent,
    ServiceWentSilent,
    RetryStorm,
    /// Cumulative-pattern signal from the background reflection cadence
    /// (chunk #98). The ONE synthetic kind NOT emitted by a streaming
    /// detector — constructed only at the L4 reflection-incident producer
    /// (`pulse-app/src/inference_runtime.rs`) to give workspace-global
    /// reflection incidents a `(kind, scope, workspace)` cool-down identity
    /// distinct from the five detector cues. Never enters the
    /// cue→suppression path (reflection digests carry no `AttentionCue`).
    ReflectionTrend,
}

/// Scope an attention cue applies to: a single service, a single operation
/// within a service, or the global pipeline. Bounded enumeration; variants
/// serialize as snake_case strings. Chunk #78 added `Hash` derive +
/// cfg-gated `specta::Type` derive (parallel to `CueKind`).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueScope {
    Service,
    Operation,
    Global,
}

/// Three-tier severity model per capability spec P-019. Autonomous = model
/// is highly confident (surfaces with prominent halo shift and counter
/// increment); Suggested = model believes likely problem with reservations
/// (quiet counter increment, minimal halo); Curious = worth recording for
/// pattern learning, no interruption (Findings dropdown collapsed section).
/// Chunk #78 added cfg-gated `specta::Type` derive (for cross-bridge
/// `IncidentRecord` resolver envelope).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorityTier {
    Autonomous,
    Suggested,
    Curious,
}

/// Severity level for an incident. Bounded enumeration; mirrors
/// `tracing::Level` ordering for future event integration. Variants
/// serialize as snake_case strings. Chunk #78 added cfg-gated
/// `specta::Type` derive (for cross-bridge `IncidentRecord` resolver
/// envelope). The TypeScript binding is renamed `IncidentSeverity`
/// to disambiguate from `ingest::connection::Severity` (same identifier,
/// distinct domain — connection severity vs incident severity); specta
/// rejects duplicate type names across the bindings.ts emission.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[cfg_attr(feature = "taurpc-runtime", specta(rename = "IncidentSeverity"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warn,
    Error,
    Critical,
}

/// Lifecycle state of an incident per capability spec P-022 auto-resolution
/// flow. Active = currently surfaced; Acknowledged = user acknowledged but
/// underlying signal still active; Resolved = signal has not re-emitted for
/// the cool-down window (120s default). Chunk #78 added cfg-gated
/// `specta::Type` derive (for cross-bridge type generation via
/// `IncidentLifecycleEvent` payload + `IncidentRecord` resolver envelope).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Active,
    Acknowledged,
    Resolved,
}

/// Kind of digest emitted at the L3/L4 layer boundary. Bounded enumeration;
/// future kinds are added explicitly. Variants serialize as snake_case
/// strings.
///
/// Chunk #81 extension: cadence-mode tier variants
/// (`CadenceTier1`/`Tier2`/`Tier3`/`Reflection`) + `ResolutionSummary`
/// per dist-arch v3 §L3 invocation modes + §Queue behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestKind {
    Snapshot,
    IncidentSummary,
    BaselineState,
    AttentionCueDigest,
    CadenceTier1,
    CadenceTier2,
    CadenceTier3,
    Reflection,
    ResolutionSummary,
}

/// References to evidence supporting an incident or attention cue. Stores
/// OTLP-native identifiers (raw bytes per OTLP spec; 16-byte trace_id,
/// 8-byte span_id) plus optional anonymized fingerprint hashes per
/// capability spec P-047 redaction-by-construction posture. Hex string
/// conversion (for tracing event field values) happens at emission site,
/// not in the contract type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRefs {
    pub trace_id: Option<[u8; 16]>,
    pub span_ids: Vec<[u8; 8]>,
    pub fingerprint_hashes: Vec<String>,
    pub timestamps_unix_nano: Vec<i64>,
}

/// Algorithmic attention cue emitted by triage detectors per capability
/// spec P-021. Structured record naming the detected condition, the
/// affected scope, and the magnitude of the deviation. Consumed by the
/// model's input context for subsequent interpretation passes. Attention
/// cues never become incidents by themselves; they are internal
/// infrastructure ensuring statistically significant patterns are not
/// overlooked.
///
/// `AttentionCue` derives `PartialEq` (not `Eq`) because the `f64`
/// fields (`magnitude`, `absolute_value`, `confidence`) do not implement
/// `Eq` per Rust's IEEE 754 floating-point semantics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttentionCue {
    pub kind: CueKind,
    pub scope: CueScope,
    /// Identifier of the cue's affected scope (`service.name` when scope =
    /// Service; `operation.name` when scope = Operation; `None` when
    /// scope = Global). Bounded by OTLP cardinality discipline at emission
    /// site.
    pub scope_id: Option<String>,
    /// Detected magnitude as a multiplier vs the streaming baseline
    /// (e.g., 3.5 = 3.5x baseline value).
    pub magnitude: f64,
    /// Absolute value of the detected metric (e.g., absolute error rate
    /// as a fraction; absolute p95 latency in milliseconds).
    pub absolute_value: f64,
    /// How long the condition has persisted, in seconds.
    pub persistence_seconds: u64,
    /// Detector confidence in (0.0, 1.0].
    pub confidence: f64,
    /// Priority tier assigned by the detector based on magnitude +
    /// confidence + persistence per capability spec P-019.
    pub priority_tier: PriorityTier,
    /// True if dual-condition bypass triggered per capability spec P-057
    /// (e.g., `magnitude > 10x` baseline OR `absolute_error_rate > 5%`
    /// overrides restart-window suppression).
    pub suppression_bypassed: bool,
}

/// User-visible incident produced when the model interprets an attention
/// cue as a real problem worth surfacing per capability spec P-020.
/// Carries narrative content (`title`, `detail`) separable from
/// agent-readable enumerated metadata (`kind`, `severity`, `status`) so
/// future emitters can apply `#[tracing::instrument(skip(title, detail))]`
/// + Layer-redaction at the formatter stage.
///
/// Chunk #78 extended the struct with persistence-related fields:
/// `workspace` (workspace-detector attribution), `kind` + `scope`
/// (reused from CueKind/CueScope as cool-down identity tuple),
/// `updated_at_unix_nano` (re-emission timestamp for 120s auto-resolve
/// window), `acknowledged_at_unix_nano` (ack timestamp), and
/// `read_at_unix_nano` (Report-opening event mutation; column exists in
/// chunk #68 schema but UI trigger lands in chunk #87+).
///
/// `id` is the corpus rowid (i64) assigned by `INSERT INTO incidents`;
/// 0 = unpersisted sentinel for in-memory drafts. The `fingerprint`
/// field is the UUID-shaped cross-incident grouping identifier (separate
/// from id).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Incident {
    /// Corpus rowid assigned by INSERT. 0 = unpersisted sentinel.
    pub id: i64,
    /// Workspace attribution; canonicalized path from workspace-detector
    /// at the producer side. Per security plan §Anti-Patterns Input row
    /// 4 (CWE-22 path traversal class defense); workspace-detector is
    /// the trust boundary; this field stores the resolved string.
    pub workspace: String,
    /// Anonymized fingerprint hash for cross-incident grouping per
    /// capability spec P-047 redaction-by-construction posture.
    pub fingerprint: String,
    /// Free-text summary content; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub title: String,
    /// Free-text incident detail; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub detail: String,
    /// Bounded incident kind enum; part of (kind, scope, workspace)
    /// cool-down identity tuple per capability spec P-023. Reuses
    /// `CueKind` since incidents derive from attention cues per dist-arch
    /// v3.
    pub kind: CueKind,
    /// Bounded incident scope enum; part of cool-down identity tuple.
    pub scope: CueScope,
    /// Service attribution: `service.name` when `scope == CueScope::Service`,
    /// `None` otherwise. Sourced from the originating `AttentionCue.scope_id`
    /// at incident construction (the production cue→incident creation path is
    /// deferred per `pulse-app/src/inference_runtime.rs` — populated `None`
    /// until that producer lands). `#[serde(default)]` keeps pre-existing
    /// corpus BLOB payloads (authored before this field) deserializable.
    /// Consumed by the per-service severity join in `services.list_with_states`.
    #[serde(default)]
    pub scope_id: Option<String>,
    pub status: IncidentStatus,
    pub severity: Severity,
    pub priority_tier: PriorityTier,
    pub evidence_refs: EvidenceRefs,
    pub opened_at_unix_nano: i64,
    /// Re-emission timestamp; bumped by `IncidentRegistry::observe_reemission`.
    /// The auto-resolve evaluator at chunk #78
    /// `pulse-app/src/incident_observer.rs` compares
    /// `now - updated_at_unix_nano >= 120s` and transitions Active /
    /// Acknowledged → Resolved per capability spec P-022.
    pub updated_at_unix_nano: i64,
    pub acknowledged_at_unix_nano: Option<i64>,
    pub resolved_at_unix_nano: Option<i64>,
    pub read_at_unix_nano: Option<i64>,
    /// L4-generated summary attached on Resolved transition per capability
    /// spec P-022 + P-059 (chunk #86). `#[serde(default)]` keeps pre-chunk-#86
    /// persisted rows deserializable (backward-compat for corpus BLOB payloads
    /// authored by chunk #78 persist cycle). MUST be populated via the
    /// `IncidentRegistry::attach_resolution_summary` API path so that
    /// `security::scrubber::scrub_attribute` runs before persistence per
    /// chunk #72 uniform-coverage invariant.
    #[serde(default)]
    pub resolution_summary_text: Option<String>,
}

/// LWW queue mode classifier for `Digest` per dist-arch v3 §Queue behavior.
/// Determines whether the digest is LWW-eligible, an active-incident
/// bypass that escapes LWW, or a Tier-1 hard signal that never gets
/// LWW-replaced. Reflection digests follow LWW as well (cadence-mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestLwwMode {
    /// Default cadence-mode digest; LWW-replaces prior cadence digest
    /// within the same workspace.
    Default,
    /// Cadence-mode digest produced while ≥1 active incident has severity
    /// ≥ Suggested in the digest's workspace. Bypasses LWW; queues
    /// independently (capped at 5 per workspace).
    ActiveIncidentBypass,
    /// Tier-1 hard signal digest. Never LWW-replaced (capped at 3
    /// regardless of workspace state).
    Tier1NeverLww,
    /// Reflection digest (30-minute background cadence). Same LWW
    /// semantics as Default cadence digests; classified separately for
    /// observability + scheduling discipline.
    Reflection,
}

/// Per-service row in the SERVICES section of an Appendix C digest
/// (`rate, error%, p99 vs baselines`). f64 fields → `PartialEq` only
/// (matches `AttentionCue` precedent at this file).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigestServiceRow {
    /// Service identifier (e.g., `service.name`). Pre-scrubbed at the
    /// producer side; chunk #81 assembler invokes
    /// `security::scrubber::scrub_attribute` BEFORE this field is
    /// populated when the source string is OTLP-attribute-derived.
    pub service: String,
    pub rate_per_sec: f64,
    pub rate_baseline_per_sec: f64,
    pub error_rate: f64,
    pub error_rate_baseline: f64,
    pub p99_latency_ms: f64,
    pub p99_baseline_ms: f64,
}

/// Backward-compat default for `DigestCueRef::scope` when deserializing
/// archived digest BLOBs authored before the chunk #92 scope-threading
/// fields existed. `CueScope` derives no `Default`, so serde needs an
/// explicit producer; Global is the neutral "no specific service" scope.
fn default_digest_cue_scope() -> CueScope {
    CueScope::Global
}

/// Compact attention-cue reference embedded in a digest's ATTENTION CUES
/// section. Carries enumerated cue kind + scope-summary string (NOT raw
/// OTLP attributes — scrubbed at producer).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestCueRef {
    pub kind: CueKind,
    pub priority_tier: PriorityTier,
    /// Free-text scope summary; pre-scrubbed at producer side.
    pub summary: String,
    /// Originating cue scope enum (chunk #92). Threaded structurally so the
    /// incident-creation producer reuses the `(kind, scope)` identity to
    /// build `Incident.{kind,scope}`. `#[serde(default)]` keeps pre-chunk-#92
    /// archived digest BLOBs deserializable.
    #[serde(default = "default_digest_cue_scope")]
    pub scope: CueScope,
    /// Originating service attribution (`AttentionCue.scope_id`; chunk #92).
    /// Threaded structurally (not only folded into `summary`) so the producer
    /// can populate `Incident.scope_id` and light up the per-service severity
    /// join. Pre-scrubbed at producer side; `#[serde(default)]` keeps older
    /// archived BLOBs deserializable.
    #[serde(default)]
    pub scope_id: Option<String>,
}

/// Output of the L3 distillation layer — a digest summarizing incidents,
/// baselines, attention cues, services, and corpus matches for downstream
/// L4 LLM consumption + corpus archival.
///
/// Chunk #81 extension: adds workspace identification, window timestamps,
/// SERVICES + ATTENTION CUES + CORPUS MATCHES structured fields, and
/// LWW metadata (lww_mode + active_incident_bypass + resolution_event)
/// per dist-arch v3 §Appendix C. f64 fields in `services` force dropping
/// the `Eq` derive (preserving `PartialEq`); existing tests use
/// `assert_eq!` which works on `PartialEq`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Digest {
    pub kind: DigestKind,
    pub token_count: usize,
    /// Free-text digest payload; MUST be populated with redacted /
    /// fingerprinted summary content only. NEVER raw OTLP attribute
    /// values, span/log/metric content payloads. Per security plan
    /// §Logging "What NEVER to log" bullet 1 + §Security Anti-Patterns
    /// §Logging bullet 1.
    pub payload_summary: String,
    /// References to incident IDs included in this digest.
    pub incident_refs: Vec<String>,
    pub generated_at_unix_nano: i64,
    /// Workspace canonical path (chunk #81). Drives corpus filter +
    /// active-incident lookup. Pre-canonicalized at workspace-detector;
    /// not scrubbed (workspace path is a first-party detected identifier,
    /// not OTLP-attribute-derived).
    pub workspace: String,
    /// Window start (chunk #81). Used for corpus retention / replay.
    pub window_start_unix_nano: i64,
    pub window_end_unix_nano: i64,
    /// SERVICES section structured rows (chunk #81). Sorted by anomaly
    /// severity per Appendix C composition rule.
    pub services: Vec<DigestServiceRow>,
    /// ATTENTION CUES section (chunk #81). Combined HIGH + MEDIUM tiers;
    /// renderer separates by `priority_tier`.
    pub attention_cues: Vec<DigestCueRef>,
    /// CORPUS MATCHES section (chunk #81). Top-N fingerprint hashes of
    /// similar past incidents retrieved from corpus (capability P-044).
    pub corpus_matches: Vec<String>,
    /// LWW queue mode (chunk #81). Drives `LwwQueue::push` decision.
    pub lww_mode: DigestLwwMode,
    /// True if active-incident exception bypassed LWW (chunk #81 / P-059).
    /// Mirrored as a discriminating field in `digest.lww.replace` events
    /// per obs plan binding.
    pub active_incident_bypass: bool,
    /// True if this digest captures a Resolved transition (chunk #81 /
    /// P-022). L4 prompt uses this flag to generate resolution summary.
    pub resolution_event: bool,
}

impl Digest {
    /// Apply a scrubbing closure to every OTLP-attribute-derived string
    /// field per chunk #72 cross-crate `scrubbed_clone` pattern (CLAUDE.md
    /// §Session Learnings 2026-05-20). Pulse-app side dep-injects
    /// `security::scrubber::scrub_attribute`-wrapping closure; this
    /// method lives on the lower crate per arch §Module dependency
    /// direction (no `security` crate dep on `triage`).
    ///
    /// Fields scrubbed: `payload_summary` (the rendered Appendix C text
    /// body), each `attention_cues[].summary`, each `services[].service`
    /// name. `workspace` is NOT scrubbed (first-party canonicalized
    /// path, not OTLP-derived). `incident_refs` + `corpus_matches`
    /// carry hash IDs, not user content.
    pub fn scrubbed_clone<F: Fn(&str) -> String>(&self, scrub: F) -> Self {
        Self {
            kind: self.kind,
            token_count: self.token_count,
            payload_summary: scrub(&self.payload_summary),
            incident_refs: self.incident_refs.clone(),
            generated_at_unix_nano: self.generated_at_unix_nano,
            workspace: self.workspace.clone(),
            window_start_unix_nano: self.window_start_unix_nano,
            window_end_unix_nano: self.window_end_unix_nano,
            services: self
                .services
                .iter()
                .map(|row| DigestServiceRow {
                    service: scrub(&row.service),
                    rate_per_sec: row.rate_per_sec,
                    rate_baseline_per_sec: row.rate_baseline_per_sec,
                    error_rate: row.error_rate,
                    error_rate_baseline: row.error_rate_baseline,
                    p99_latency_ms: row.p99_latency_ms,
                    p99_baseline_ms: row.p99_baseline_ms,
                })
                .collect(),
            attention_cues: self
                .attention_cues
                .iter()
                .map(|cue| DigestCueRef {
                    kind: cue.kind,
                    priority_tier: cue.priority_tier,
                    summary: scrub(&cue.summary),
                    scope: cue.scope,
                    scope_id: cue.scope_id.as_deref().map(&scrub),
                })
                .collect(),
            corpus_matches: self.corpus_matches.clone(),
            lww_mode: self.lww_mode,
            active_incident_bypass: self.active_incident_bypass,
            resolution_event: self.resolution_event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_evidence_refs() -> EvidenceRefs {
        EvidenceRefs {
            trace_id: Some([0xAB; 16]),
            span_ids: vec![[0xCD; 8], [0xEF; 8]],
            fingerprint_hashes: vec!["abc123".to_string(), "def456".to_string()],
            timestamps_unix_nano: vec![1_700_000_000_000, 1_700_000_001_000],
        }
    }

    fn sample_attention_cue() -> AttentionCue {
        AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("checkout".to_string()),
            magnitude: 3.5,
            absolute_value: 0.075,
            persistence_seconds: 45,
            confidence: 0.87,
            priority_tier: PriorityTier::Suggested,
            suppression_bypassed: false,
        }
    }

    fn sample_incident() -> Incident {
        Incident {
            id: 42,
            workspace: "ws-checkout".to_string(),
            fingerprint: "fp-checkout-err-spike".to_string(),
            title: "[redacted] error rate spike in checkout".to_string(),
            detail: "[redacted] sustained 3.5x baseline for 45s".to_string(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("checkout".to_string()),
            status: IncidentStatus::Active,
            severity: Severity::Warn,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: sample_evidence_refs(),
            opened_at_unix_nano: 1_700_000_000_000,
            updated_at_unix_nano: 1_700_000_000_000,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    fn sample_digest() -> Digest {
        Digest {
            kind: DigestKind::IncidentSummary,
            token_count: 1024,
            payload_summary: "[redacted] 1 active incident, 3 resolved".to_string(),
            incident_refs: vec!["42".to_string()],
            generated_at_unix_nano: 1_700_000_002_000,
            workspace: "/home/dev/example".to_string(),
            window_start_unix_nano: 1_700_000_000_000,
            window_end_unix_nano: 1_700_000_060_000,
            services: vec![DigestServiceRow {
                service: "auth-service".to_string(),
                rate_per_sec: 45.2,
                rate_baseline_per_sec: 44.0,
                error_rate: 0.123,
                error_rate_baseline: 0.008,
                p99_latency_ms: 240.0,
                p99_baseline_ms: 80.0,
            }],
            attention_cues: vec![DigestCueRef {
                kind: CueKind::ErrorRateSpike,
                priority_tier: PriorityTier::Suggested,
                summary: "auth-service error rate 12.3% vs 0.8% baseline".to_string(),
                scope: CueScope::Service,
                scope_id: Some("auth-service".to_string()),
            }],
            corpus_matches: vec!["fp-a3f9".to_string()],
            lww_mode: DigestLwwMode::Default,
            active_incident_bypass: false,
            resolution_event: false,
        }
    }

    #[test]
    fn cue_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&CueKind::ErrorRateSpike).unwrap(),
            "\"error_rate_spike\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::LatencyRegression).unwrap(),
            "\"latency_regression\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::RestartEvent).unwrap(),
            "\"restart_event\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::ServiceWentSilent).unwrap(),
            "\"service_went_silent\""
        );
        assert_eq!(
            serde_json::to_string(&CueKind::RetryStorm).unwrap(),
            "\"retry_storm\""
        );
    }

    #[test]
    fn cue_scope_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&CueScope::Service).unwrap(),
            "\"service\""
        );
        assert_eq!(
            serde_json::to_string(&CueScope::Operation).unwrap(),
            "\"operation\""
        );
        assert_eq!(
            serde_json::to_string(&CueScope::Global).unwrap(),
            "\"global\""
        );
    }

    #[test]
    fn priority_tier_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&PriorityTier::Autonomous).unwrap(),
            "\"autonomous\""
        );
        assert_eq!(
            serde_json::to_string(&PriorityTier::Suggested).unwrap(),
            "\"suggested\""
        );
        assert_eq!(
            serde_json::to_string(&PriorityTier::Curious).unwrap(),
            "\"curious\""
        );
    }

    #[test]
    fn severity_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), "\"info\"");
        assert_eq!(serde_json::to_string(&Severity::Warn).unwrap(), "\"warn\"");
        assert_eq!(
            serde_json::to_string(&Severity::Error).unwrap(),
            "\"error\""
        );
        assert_eq!(
            serde_json::to_string(&Severity::Critical).unwrap(),
            "\"critical\""
        );
    }

    #[test]
    fn incident_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Acknowledged).unwrap(),
            "\"acknowledged\""
        );
        assert_eq!(
            serde_json::to_string(&IncidentStatus::Resolved).unwrap(),
            "\"resolved\""
        );
    }

    #[test]
    fn digest_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&DigestKind::Snapshot).unwrap(),
            "\"snapshot\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::IncidentSummary).unwrap(),
            "\"incident_summary\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::BaselineState).unwrap(),
            "\"baseline_state\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::AttentionCueDigest).unwrap(),
            "\"attention_cue_digest\""
        );
        assert_eq!(
            serde_json::to_string(&DigestKind::Reflection).unwrap(),
            "\"reflection\""
        );
    }

    #[test]
    fn cue_kind_round_trips_through_serde() {
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let parsed: CueKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn cue_kind_reflection_trend_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&CueKind::ReflectionTrend).unwrap(),
            "\"reflection_trend\""
        );
    }

    #[test]
    fn cue_scope_round_trips_through_serde() {
        for scope in [CueScope::Service, CueScope::Operation, CueScope::Global] {
            let json = serde_json::to_string(&scope).expect("serialize");
            let parsed: CueScope = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, scope);
        }
    }

    #[test]
    fn priority_tier_round_trips_through_serde() {
        for tier in [
            PriorityTier::Autonomous,
            PriorityTier::Suggested,
            PriorityTier::Curious,
        ] {
            let json = serde_json::to_string(&tier).expect("serialize");
            let parsed: PriorityTier = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, tier);
        }
    }

    #[test]
    fn severity_round_trips_through_serde() {
        for sev in [
            Severity::Info,
            Severity::Warn,
            Severity::Error,
            Severity::Critical,
        ] {
            let json = serde_json::to_string(&sev).expect("serialize");
            let parsed: Severity = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, sev);
        }
    }

    #[test]
    fn incident_status_round_trips_through_serde() {
        for status in [
            IncidentStatus::Active,
            IncidentStatus::Acknowledged,
            IncidentStatus::Resolved,
        ] {
            let json = serde_json::to_string(&status).expect("serialize");
            let parsed: IncidentStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, status);
        }
    }

    #[test]
    fn digest_kind_round_trips_through_serde() {
        for kind in [
            DigestKind::Snapshot,
            DigestKind::IncidentSummary,
            DigestKind::BaselineState,
            DigestKind::AttentionCueDigest,
            DigestKind::Reflection,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let parsed: DigestKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn evidence_refs_round_trips_through_serde() {
        let e = sample_evidence_refs();
        let json = serde_json::to_string(&e).expect("serialize");
        let parsed: EvidenceRefs = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, e);
    }

    #[test]
    fn attention_cue_round_trips_through_serde() {
        let cue = sample_attention_cue();
        let json = serde_json::to_string(&cue).expect("serialize");
        let parsed: AttentionCue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, cue);
    }

    #[test]
    fn incident_round_trips_through_serde() {
        let inc = sample_incident();
        let json = serde_json::to_string(&inc).expect("serialize");
        let parsed: Incident = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, inc);
    }

    #[test]
    fn incident_deserializes_pre_chunk_86_payload_without_resolution_summary_field() {
        // BLOB payloads persisted by chunk #78 (pre-chunk-#86) lack the
        // `resolution_summary_text` field. `#[serde(default)]` MUST keep
        // those rows deserializable; chunk #86 backward-compat invariant.
        let pre_chunk_86_json = r#"{
            "id": 42,
            "workspace": "ws-checkout",
            "fingerprint": "fp-checkout-err-spike",
            "title": "[redacted] error rate spike in checkout",
            "detail": "[redacted] sustained 3.5x baseline for 45s",
            "kind": "error_rate_spike",
            "scope": "service",
            "status": "active",
            "severity": "warn",
            "priority_tier": "suggested",
            "evidence_refs": {"trace_id": null, "span_ids": [], "fingerprint_hashes": [], "timestamps_unix_nano": []},
            "opened_at_unix_nano": 1700000000000,
            "updated_at_unix_nano": 1700000000000,
            "acknowledged_at_unix_nano": null,
            "resolved_at_unix_nano": null,
            "read_at_unix_nano": null
        }"#;
        let parsed: Incident =
            serde_json::from_str(pre_chunk_86_json).expect("deserialize pre-chunk-#86 row");
        assert_eq!(parsed.resolution_summary_text, None);
        assert_eq!(parsed.id, 42);
    }

    #[test]
    fn digest_round_trips_through_serde() {
        let d = sample_digest();
        let json = serde_json::to_string(&d).expect("serialize");
        let parsed: Digest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, d);
    }
}
