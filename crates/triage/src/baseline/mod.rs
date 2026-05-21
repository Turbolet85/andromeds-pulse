//! Streaming baseline trackers + corpus persistence (chunk #61 trackers +
//! chunk #70 corpus migration).
//!
//! Implements the L1b streaming distillation foundation per
//! `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 2 (capabilities P-009 +
//! P-011) + Phase 6 Consolidation (capability P-013 + P-051). Three
//! algorithm primitives — `EwmaTracker` (per-service error rate baseline),
//! `TDigestPair` (per-operation streaming p50/p95/p99 latency baseline
//! with swap-on-tick rotation), and `RollingWindow<u32>` (per-service
//! activity tracking) — feed `BaselineState`, a DashMap-backed aggregator.
//!
//! Persistence routes through the [`persistence::BaselinePersistence`]
//! trait, implemented at `pulse-app/src/baseline_persistence.rs` over
//! `corpus::contract::CorpusWriter` (Schema Option A — reuses the
//! `pipeline_metrics` blob slot with `metric_name="baseline_state"` +
//! `layer="l1b"`, mirroring chunk #69 `CorpusDrainPersistence`). The flat
//! file `<data-dir>/triage/baseline-corpus.bin` (chunk #61 substrate) is
//! eliminated; legacy files migrate-and-delete on first boot via
//! `pulse-app/src/baseline_persistence.rs::migrate_legacy_baseline_if_present`.

mod activity_floor;
mod corpus;
mod error;
mod ewma;
mod persistence;
mod rolling_window;
mod tdigest_pair;

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::time::Duration;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

pub use activity_floor::{
    ActivityFloor, BOOTSTRAP_WINDOW_SECONDS, BUCKET_COUNT, BUCKET_INTERVAL_SECONDS, BootstrapState,
    WINDOW_DURATION_SECONDS,
};
pub use corpus::{
    BootstrapResult, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP, PersistStats,
    bootstrap_from_persistence,
};
pub use error::BaselineError;
pub use ewma::EwmaTracker;
pub use persistence::BaselinePersistence;
pub use rolling_window::RollingWindow;
pub use tdigest_pair::TDigestPair;

pub const SCHEMA_VERSION: u32 = 1;

/// Canonical corpus filesystem basename. Persistence writes go through
/// `corpus::contract::CorpusWriter`; this constant carries the basename
/// surfaced in tracing fields (`corpus_basename`) since the trait API
/// does not expose the underlying path. Matches arch §Occupied Resources
/// Filesystem locations corpus/corpus.db subpath.
pub(crate) const CORPUS_BASENAME: &str = "corpus.db";

pub(crate) const TARGET_BASELINE_TICK: &str = "triage.baseline.tick";
pub(crate) const TARGET_BASELINE_PERSIST: &str = "triage.baseline.persist";
pub const TARGET_BASELINE_PERSIST_ERROR: &str = "triage.baseline.persist.error";
pub const TARGET_BASELINE_MIGRATE: &str = "triage.baseline.migrate";
pub const TARGET_BASELINE_MIGRATE_FAILED: &str = "triage.baseline.migrate.failed";
pub(crate) const TARGET_SERVICE_ID_MISSING: &str = "triage.service_id_missing";
pub(crate) const TARGET_METRIC_EWMA_SHORT_WINDOW: &str = "metric.baseline.ewma_short_window_size";
pub(crate) const TARGET_PIPELINE_L1B_PERSIST_TOTAL: &str = "pipeline.l1b.persist_count_total";
pub(crate) const TARGET_PIPELINE_L1B_BOOTSTRAP_TOTAL: &str = "pipeline.l1b.bootstrap_count_total";
pub const TARGET_SERVICE_CAP_EXCEEDED: &str = "triage.baseline.service_cap_exceeded";

/// In-process cardinality cap for per-service tracking (chunk #64 service
/// cap enforcement). Mirrors the existing corpus-load-time
/// `DEFAULT_SERVICE_COUNT_CAP` value so corpus persistence (deferred to
/// chunk #69 wire-up) does not surface a separate ceiling. Per security
/// extract: bounds OTLP-attribute-derived `service.name` cardinality to
/// prevent OOM under malicious or buggy instrumented hosts.
pub const ACTIVITY_FLOOR_SERVICE_CAP: usize = DEFAULT_SERVICE_COUNT_CAP;

pub const DEFAULT_ALPHA_5MIN_WINDOW: f64 = 0.00333;
/// Per capability spec P-010: short-term (30-second window) EWMA alpha for
/// baseline-relative spike detection. The cue evaluator compares the
/// short-term EWMA against the long-term EWMA (`DEFAULT_ALPHA_5MIN_WINDOW`)
/// rather than against a fixed constant baseline. Calibrated assuming
/// ~1 observation per second; alpha ≈ 1/window_seconds.
pub const DEFAULT_ALPHA_30S_WINDOW: f64 = 0.0333;
pub const DEFAULT_PERSIST_INTERVAL_NANOS: i64 = 60_000_000_000;
/// Short-window t-digest swap interval (chunk #73 P-012). 15s swap →
/// rolling 15-30s effective window via swap-on-tick rotation (current
/// period 0-15s + previous period 0-15s spanning ~30s of observations).
/// Invoked from cue emitter tick body at 1Hz; internal age-check no-ops
/// if elapsed < interval.
pub const DEFAULT_SHORT_SWAP_INTERVAL_NANOS: i64 = 15_000_000_000;
pub const STATE_AGE_THRESHOLD_NANOS: i64 = 3_600_000_000_000;
pub(crate) const DEFAULT_ACTIVITY_WINDOW_CAPACITY: usize = 300;

const _: () = {
    assert!(ACTIVITY_FLOOR_SERVICE_CAP > 0);
};

/// OTLP Status.code = 2 means Error per OTLP Trace v1 spec.
const STATUS_CODE_ERROR: u8 = 2;

mod atomic_i64_serde {
    use std::sync::atomic::{AtomicI64, Ordering};

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &AtomicI64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(value.load(Ordering::Relaxed))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<AtomicI64, D::Error> {
        let n = i64::deserialize(deserializer)?;
        Ok(AtomicI64::new(n))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceBaseline {
    error_rate_ewma: EwmaTracker,
    /// Per capability spec P-010: short-term (30-second window) EWMA for
    /// baseline-relative spike detection. Compares against `error_rate_ewma`
    /// (5-min long-term baseline) at evaluation time. `#[serde(default)]`
    /// provides a fresh empty short tracker for pre-chunk-#73 corpus records;
    /// it re-accumulates as observations arrive.
    #[serde(default = "default_short_ewma")]
    error_rate_ewma_short: EwmaTracker,
    activity_window: RollingWindow<u32>,
    #[serde(default)]
    activity_floor: ActivityFloor,
}

impl Default for ServiceBaseline {
    fn default() -> Self {
        Self {
            error_rate_ewma: EwmaTracker::default(),
            error_rate_ewma_short: default_short_ewma(),
            activity_window: RollingWindow::default(),
            activity_floor: ActivityFloor::default(),
        }
    }
}

fn default_short_ewma() -> EwmaTracker {
    EwmaTracker::new(DEFAULT_ALPHA_30S_WINDOW)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct OperationBaseline {
    latency_tdigest: TDigestPair,
    // Per capability spec P-012: short-term (~30-second window via 15s swap
    // rotation) t-digest for baseline-relative latency regression detection
    // (compares against `latency_tdigest` long-term ~120s window at evaluation).
    // `#[serde(default)]` provides empty pair for pre-chunk-#73 corpus records.
    #[serde(default)]
    latency_tdigest_short: TDigestPair,
    // Per capability spec P-011: human-readable operation identifier (e.g.,
    // `"GET /api/foo"`) preserved alongside the hashed `operation_key`
    // (DashMap key) so downstream surfaces can show the readable name without
    // re-deriving it from a collision-resistant hash. `#[serde(default)]`
    // provides empty fallback for pre-chunk-#73 corpus records.
    #[serde(default)]
    pub(crate) operation_name: String,
}

/// Aggregator of per-service and per-operation streaming baselines.
/// `DashMap` provides lock-free reads for the hot `observe_span` path;
/// `persisted_at_unix_nanos` advances on each successful persist (used by
/// bootstrap age-check on next startup); `drops_since_last_tick` counts
/// spans with empty `service.name` (drained + emitted as aggregate warn on
/// each tick).
#[derive(Debug, Serialize, Deserialize)]
pub struct BaselineState {
    schema_version: u32,
    services: DashMap<String, ServiceBaseline>,
    operations: DashMap<String, OperationBaseline>,
    #[serde(with = "atomic_i64_serde")]
    persisted_at_unix_nanos: AtomicI64,
    #[serde(skip)]
    drops_since_last_tick: AtomicU32,
    #[serde(skip)]
    service_cap_exceeded_since_last_tick: AtomicU32,
}

impl Default for BaselineState {
    fn default() -> Self {
        Self::new()
    }
}

impl BaselineState {
    pub fn new() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            services: DashMap::new(),
            operations: DashMap::new(),
            persisted_at_unix_nanos: AtomicI64::new(0),
            drops_since_last_tick: AtomicU32::new(0),
            service_cap_exceeded_since_last_tick: AtomicU32::new(0),
        }
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    pub fn persisted_at_unix_nanos(&self) -> i64 {
        self.persisted_at_unix_nanos.load(Ordering::Relaxed)
    }

    pub fn set_persisted_at_unix_nanos(&self, now_nanos: i64) {
        self.persisted_at_unix_nanos
            .store(now_nanos, Ordering::Relaxed);
    }

    pub fn drain_drops_since_last_tick(&self) -> u32 {
        self.drops_since_last_tick.swap(0, Ordering::Relaxed)
    }

    /// Drain + return the count of spans dropped this tick due to the
    /// service-name cardinality cap (chunk #64 P-013 in-process bound on
    /// `service.name` cardinality). Mirrors `drain_drops_since_last_tick`
    /// pattern.
    pub fn drain_service_cap_exceeded_since_last_tick(&self) -> u32 {
        self.service_cap_exceeded_since_last_tick
            .swap(0, Ordering::Relaxed)
    }

    /// Per-spec entry point. Drops spans с empty service.name (increments
    /// `drops_since_last_tick`; aggregate warn fires from per-tick emitter).
    /// Updates the service's error-rate EWMA + per-second activity bucket +
    /// activity-floor histogram (chunk #64) + the (service, operation)
    /// tuple's latency t-digest. Enforces the chunk #64 per-instance
    /// `service.name` cardinality cap (`ACTIVITY_FLOOR_SERVICE_CAP`); spans
    /// for new services after the cap is reached are dropped + counted
    /// (aggregate warn fires from per-tick emitter).
    pub fn observe_span(
        &self,
        service_name: &str,
        operation_name: &str,
        status_code: u8,
        latency_ms: u64,
        now_nanos: i64,
    ) {
        if service_name.is_empty() {
            self.drops_since_last_tick.fetch_add(1, Ordering::Relaxed);
            return;
        }

        if !self.services.contains_key(service_name)
            && self.services.len() >= ACTIVITY_FLOOR_SERVICE_CAP
        {
            self.service_cap_exceeded_since_last_tick
                .fetch_add(1, Ordering::Relaxed);
            return;
        }

        let error_observation = if status_code == STATUS_CODE_ERROR {
            1.0
        } else {
            0.0
        };
        let mut svc = self.services.entry(service_name.to_string()).or_default();
        svc.error_rate_ewma.observe(error_observation, now_nanos);
        // Chunk #73 P-010: feed the short-term EWMA alongside long-term.
        svc.error_rate_ewma_short
            .observe(error_observation, now_nanos);
        svc.activity_window.push(1);
        svc.activity_floor.observe(now_nanos);
        drop(svc);

        if !operation_name.is_empty() {
            let key = operation_key(service_name, operation_name);
            let mut op = self
                .operations
                .entry(key)
                .or_insert_with(|| OperationBaseline {
                    latency_tdigest: TDigestPair::default(),
                    latency_tdigest_short: TDigestPair::default(),
                    operation_name: operation_name.to_string(),
                });
            op.latency_tdigest.insert(latency_ms as f64);
            // Chunk #73 P-012: feed the short-window t-digest alongside long.
            op.latency_tdigest_short.insert(latency_ms as f64);
        }
    }

    /// Swap all t-digest pairs whose age exceeds the swap window. Returns
    /// the count rotated. Invoked from per-tick wrapper before metric
    /// emission so percentile queries see the freshest rotation.
    pub fn swap_tdigest_pairs_on_tick(&self, now_nanos: i64, swap_window_nanos: i64) -> usize {
        let mut rotated = 0_usize;
        for mut op in self.operations.iter_mut() {
            if op
                .latency_tdigest
                .swap_on_tick(now_nanos, swap_window_nanos)
            {
                rotated += 1;
            }
        }
        rotated
    }

    /// Swap all SHORT-window t-digest pairs whose age exceeds
    /// `DEFAULT_SHORT_SWAP_INTERVAL_NANOS` (chunk #73 P-012). Invoked from
    /// cue emitter tick at 1Hz; internal age-check in `swap_on_tick` no-ops
    /// if elapsed < interval. Returns the count rotated.
    pub fn swap_short_tdigest_pairs_on_tick(&self, now_nanos: i64) -> usize {
        let mut rotated = 0_usize;
        for mut op in self.operations.iter_mut() {
            if op
                .latency_tdigest_short
                .swap_on_tick(now_nanos, DEFAULT_SHORT_SWAP_INTERVAL_NANOS)
            {
                rotated += 1;
            }
        }
        rotated
    }

    /// Total centroid count across all operation t-digests (proxy for
    /// `tdigest_centroid_count` metric field).
    pub fn total_centroid_count(&self) -> usize {
        self.operations
            .iter()
            .map(|op| op.latency_tdigest.centroid_count())
            .sum()
    }

    /// Per-service error-rate EWMA snapshot. Returns None when no spans
    /// observed for the service yet.
    pub fn error_rate(&self, service_name: &str) -> Option<f64> {
        self.services
            .get(service_name)
            .map(|s| s.error_rate_ewma.value())
    }

    /// Per-(service, operation) latency percentile snapshot. Returns None
    /// when no inserts have occurred.
    pub fn latency_percentile(
        &self,
        service_name: &str,
        operation_name: &str,
        q: f64,
    ) -> Option<f64> {
        let key = operation_key(service_name, operation_name);
        self.operations
            .get(&key)
            .and_then(|op| op.latency_tdigest.percentile(q))
    }

    /// Return a clone of the state with each `services` DashMap key and each
    /// `operations` DashMap key passed through the caller-supplied `scrub`
    /// closure (chunk #72). `scrub` is dep-injected so the triage crate stays
    /// security-crate-free per arch §Module dependency direction; the pulse-app
    /// `CorpusBaselinePersistence::save` adapter wires
    /// `security::scrubber::scrub_attribute` as the closure body. Required for
    /// at-rest persistence: `services` keys are raw `service.name` from OTLP
    /// (user-content); `operations` keys are `format!("{service}/{:016x}",
    /// service, hash)` so the prefix carries the same PII risk and is scrubbed
    /// by splitting on the first `/` and rejoining with the scrubbed prefix.
    ///
    /// Aggregation-collapse caveat (per chunk #72 plan §Implementation Note 3):
    /// PII-shaped `service.name` keys collapse into the same scrubbed bucket
    /// (e.g., two distinct misconfigured services that both look like emails
    /// merge into one `[REDACTED:email]` entry). Acceptable security > attribution
    /// trade-off for misconfigured services; typical service names
    /// (`web-api`, `auth-service`) flow through unchanged.
    pub fn scrubbed_clone<F>(&self, scrub: F) -> Self
    where
        F: Fn(&str) -> String,
    {
        let cloned = Self::new();
        cloned.set_persisted_at_unix_nanos(self.persisted_at_unix_nanos());
        for entry in self.services.iter() {
            cloned
                .services
                .insert(scrub(entry.key()), entry.value().clone());
        }
        for entry in self.operations.iter() {
            let key = entry.key();
            let scrubbed_key = match key.split_once('/') {
                Some((service_prefix, hash_suffix)) => {
                    format!("{}/{}", scrub(service_prefix), hash_suffix)
                }
                None => key.clone(),
            };
            cloned
                .operations
                .insert(scrubbed_key, entry.value().clone());
        }
        cloned
    }

    /// Snapshot all per-service baseline metrics. Called from the chunk #62
    /// attention cue emitter tick body to enumerate services for threshold
    /// evaluation. Allocates a Vec — sufficient at 1Hz tick cadence with
    /// bounded service count per `DEFAULT_SERVICE_COUNT_CAP`.
    pub fn iter_services(&self) -> Vec<ServiceMetricSnapshot> {
        self.services
            .iter()
            .map(|entry| ServiceMetricSnapshot {
                service_name: entry.key().clone(),
                error_rate: entry.error_rate_ewma.value(),
                short_term_error_rate: entry.error_rate_ewma_short.value(),
                samples: entry.error_rate_ewma.samples(),
                last_update_nanos: entry.error_rate_ewma.last_update_nanos(),
            })
            .collect()
    }

    /// Snapshot per-service activity-floor silence state for chunk #64
    /// `evaluate_service_went_silent`. Allocates a Vec — sufficient at 1Hz
    /// tick cadence with bounded service count per
    /// `ACTIVITY_FLOOR_SERVICE_CAP`. Mirrors `iter_services()` allocation
    /// shape and lock discipline (snapshot at iteration; downstream works
    /// without holding the DashMap shard lock).
    pub fn iter_service_silence_snapshots(&self, now_nanos: i64) -> Vec<ServiceSilenceSnapshot> {
        self.services
            .iter()
            .map(|entry| ServiceSilenceSnapshot {
                service_name: entry.key().clone(),
                current_quiet_duration_seconds: entry
                    .activity_floor
                    .current_quiet_duration_seconds(now_nanos),
                p95_historical_quiet_duration_seconds: entry
                    .activity_floor
                    .p95_historical_quiet_duration_seconds(),
                bootstrap_state: entry.activity_floor.bootstrap_state(now_nanos),
            })
            .collect()
    }

    /// Snapshot all per-operation latency metrics at the supplied percentile
    /// `q ∈ [0.0, 1.0]`. Service name is recovered from the operation key's
    /// prefix (everything before the first `/`); operations с malformed keys
    /// are skipped. Used by the chunk #62 cue emitter for LatencyRegression
    /// detection.
    pub fn iter_operations(&self, percentile_q: f64) -> Vec<OperationMetricSnapshot> {
        self.operations
            .iter()
            .filter_map(|entry| {
                let key = entry.key();
                let service_name = key.split('/').next()?.to_string();
                let latency = entry.latency_tdigest.percentile(percentile_q);
                // Chunk #73 P-012: SHORT t-digest queries `percentile_current_only`
                // (recent window only) so the spike signal is не diluted by
                // the prior rotation cycle. LONG t-digest stays on union for
                // smoothed long-term baseline reference.
                let short_latency = entry
                    .latency_tdigest_short
                    .percentile_current_only(percentile_q);
                let samples = entry.latency_tdigest.samples_current();
                Some(OperationMetricSnapshot {
                    operation_key: key.clone(),
                    service_name,
                    operation_name: entry.operation_name.clone(),
                    latency_at_percentile: latency,
                    short_term_latency_at_percentile: short_latency,
                    samples,
                })
            })
            .collect()
    }
}

/// Snapshot of one service's baseline metrics for chunk #62 cue evaluation
/// (long-term EWMA in `error_rate`) and chunk #73 P-010 baseline-relative
/// spike detection (30s EWMA in `short_term_error_rate`). Cloned from the
/// live DashMap shard at iteration time; downstream evaluation works on the
/// snapshot without holding the DashMap lock.
#[derive(Debug, Clone)]
pub struct ServiceMetricSnapshot {
    pub service_name: String,
    pub error_rate: f64,
    pub short_term_error_rate: f64,
    pub samples: u64,
    pub last_update_nanos: i64,
}

/// Snapshot of one operation's latency metric for chunk #62 cue evaluation
/// (long-term t-digest in `latency_at_percentile`) and chunk #73 P-012
/// baseline-relative regression detection (short-window t-digest in
/// `short_term_latency_at_percentile`). The `operation_key` is opaque
/// (`"{service}/{hash}"`); `service_name` is extracted from the prefix;
/// `operation_name` per chunk #73 P-011 is the human-readable identifier
/// preserved separately from the collision-resistant hash.
#[derive(Debug, Clone)]
pub struct OperationMetricSnapshot {
    pub operation_key: String,
    pub service_name: String,
    pub operation_name: String,
    pub latency_at_percentile: Option<f64>,
    pub short_term_latency_at_percentile: Option<f64>,
    pub samples: u64,
}

/// Snapshot of one service's activity-floor silence state for chunk #64
/// `evaluate_service_went_silent` consumption. Cloned from the live
/// `DashMap` shard at iteration time; downstream evaluation works on the
/// snapshot without holding the DashMap lock. The `p95_historical_*` field
/// is `None` when fewer than two observations have occurred for that
/// service (no gap samples yet).
#[derive(Debug, Clone)]
pub struct ServiceSilenceSnapshot {
    pub service_name: String,
    pub current_quiet_duration_seconds: u64,
    pub p95_historical_quiet_duration_seconds: Option<u64>,
    pub bootstrap_state: BootstrapState,
}

fn operation_key(service: &str, operation: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    service.hash(&mut hasher);
    operation.hash(&mut hasher);
    format!("{}/{:016x}", service, hasher.finish())
}

/// Synchronous one-pass persist cycle: swap-on-tick, emit per-tick metric,
/// drain service-identity drops + emit aggregate warn, persist via the
/// `BaselinePersistence` trait, emit persist outcome metric. Returns the
/// persist result for callers that want explicit error handling. Designed
/// for unit-testable invocation independent of the `tokio::time::interval`-
/// driven outer loop.
pub fn run_persist_cycle(
    state: &BaselineState,
    persistence: &dyn BaselinePersistence,
    now_nanos: i64,
    persist_kind: &'static str,
) -> Result<PersistStats, BaselineError> {
    state.swap_tdigest_pairs_on_tick(now_nanos, DEFAULT_PERSIST_INTERVAL_NANOS);

    let service_count = state.service_count();
    let centroid_count = state.total_centroid_count();

    tracing::info!(
        target: TARGET_BASELINE_TICK,
        value = service_count as u64,
        service_id_count = service_count as u64,
        tdigest_centroid_count = centroid_count as u64,
        dropped_count = 0_u64,
        "baseline tracker tick"
    );

    tracing::info!(
        target: TARGET_METRIC_EWMA_SHORT_WINDOW,
        value = service_count as u64,
        service_id_count = service_count as u64,
        "baseline ewma window state"
    );

    let drops = state.drain_drops_since_last_tick();
    if drops > 0 {
        tracing::warn!(
            target: TARGET_SERVICE_ID_MISSING,
            dropped_count = drops as u64,
            "service.name empty; spans dropped this tick"
        );
    }

    let cap_drops = state.drain_service_cap_exceeded_since_last_tick();
    if cap_drops > 0 {
        tracing::warn!(
            target: TARGET_SERVICE_CAP_EXCEEDED,
            dropped_count = cap_drops as u64,
            cap = ACTIVITY_FLOOR_SERVICE_CAP as u64,
            "service.name cardinality cap reached; new-service spans dropped this tick"
        );
    }

    let persist_start = std::time::Instant::now();

    // Set persisted_at BEFORE serialize so the round-trip includes it.
    // If persist fails, the in-memory atomic is "ahead" of the disk
    // version; next successful persist re-syncs. Acceptable trade-off.
    let prior_persisted_at = state.persisted_at_unix_nanos();
    state.set_persisted_at_unix_nanos(now_nanos);

    match persistence.save(state) {
        Ok(()) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            // Adapter does not surface `bytes_written` across the trait boundary;
            // synthesize from a fresh serialize. Cheap relative to the AES write
            // already performed; preserves the existing tracing field shape.
            let bytes_written = bincode::serialize(state)
                .map(|v| v.len() as u64)
                .unwrap_or(0);
            let stats = PersistStats {
                bytes_written,
                service_count,
            };
            tracing::info!(
                target: TARGET_BASELINE_PERSIST,
                service_id_count = stats.service_count as u64,
                state_size_bytes = stats.bytes_written,
                duration_ms = duration_ms,
                persist_kind = persist_kind,
                corpus_basename = CORPUS_BASENAME,
                "corpus persisted"
            );
            tracing::info!(
                target: TARGET_PIPELINE_L1B_PERSIST_TOTAL,
                value = 1_u64,
                duration_ms = duration_ms,
                persist_kind = persist_kind,
                state_size_bytes = stats.bytes_written,
                "persist count"
            );
            Ok(stats)
        }
        Err(e) => {
            // Roll the atomic back so subsequent observers don't see a
            // false-positive freshness.
            state.set_persisted_at_unix_nanos(prior_persisted_at);
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            tracing::warn!(
                target: TARGET_BASELINE_PERSIST_ERROR,
                error_category = e.error_category(),
                duration_ms = duration_ms,
                "corpus persist failed"
            );
            Err(e)
        }
    }
}

/// Long-running future spawned at boot that fires `run_persist_cycle` on
/// the supplied `interval`. Wall-clock time is read from
/// `SystemTime::UNIX_EPOCH`; tests should exercise `run_persist_cycle`
/// directly with injected `now_nanos` for deterministic timing.
pub async fn run_persist_loop(
    state: Arc<BaselineState>,
    persistence: Arc<dyn BaselinePersistence>,
    interval: Duration,
) {
    let mut ticker = tokio::time::interval(interval);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let now = current_unix_nanos();
        let _ = run_persist_cycle(&state, persistence.as_ref(), now, "periodic");
    }
}

/// Graceful-shutdown persist helper. Synchronous so it can be called from a
/// Tauri shutdown hook. Emits the persist counter with
/// `persist_kind = "shutdown"`.
pub fn persist_on_shutdown(
    state: &BaselineState,
    persistence: &dyn BaselinePersistence,
) -> Result<PersistStats, BaselineError> {
    let now = current_unix_nanos();
    run_persist_cycle(state, persistence, now, "shutdown")
}

/// Bootstrap entry point used at boot to attempt load-from-corpus, falling
/// through to fresh state on any failure. Emits the
/// `pipeline.l1b.bootstrap_count_total{kind}` metric describing the path
/// taken. Returns a ready-to-use `BaselineState`. `persistence = None`
/// (corpus unavailable at boot) → cold-start fresh state with
/// `kind = "cold_start"`.
pub fn bootstrap_state(
    persistence: Option<&dyn BaselinePersistence>,
    service_count_cap: usize,
    now_nanos: i64,
) -> BaselineState {
    let kind: &'static str;
    let state = match persistence {
        Some(p) => match bootstrap_from_persistence(p, service_count_cap, now_nanos) {
            BootstrapResult::Loaded { state, age_nanos } => {
                if age_nanos >= STATE_AGE_THRESHOLD_NANOS {
                    kind = "corpus_corrupt_reset";
                    BaselineState::new()
                } else {
                    kind = "resume_from_corpus";
                    state
                }
            }
            BootstrapResult::Fresh { reason_kind } => {
                kind = reason_kind;
                BaselineState::new()
            }
        },
        None => {
            kind = "cold_start";
            BaselineState::new()
        }
    };

    tracing::info!(
        target: TARGET_PIPELINE_L1B_BOOTSTRAP_TOTAL,
        value = 1_u64,
        kind = kind,
        "baseline bootstrap"
    );

    state
}

fn current_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

    /// In-crate test fixture implementing `BaselinePersistence` over an
    /// in-memory bincode slot. Mirrors the role of the real
    /// `pulse-app/src/baseline_persistence.rs::CorpusBaselinePersistence`
    /// adapter for crate-level tests that can't take a corpus dep
    /// (per arch §Cross-cutting Patterns Module dependency direction —
    /// `triage` MUST NOT depend on `corpus`).
    #[derive(Default)]
    struct FakeBaselinePersistence {
        bytes: Mutex<Option<Vec<u8>>>,
        next_save_error: Mutex<Option<BaselineError>>,
    }

    impl FakeBaselinePersistence {
        fn new() -> Self {
            Self::default()
        }

        /// Inject a one-shot save-error for the next call.
        fn fail_next_save_with(&self, err: BaselineError) {
            *self.next_save_error.lock().unwrap() = Some(err);
        }
    }

    impl BaselinePersistence for FakeBaselinePersistence {
        fn load(&self) -> Result<Option<BaselineState>, BaselineError> {
            let guard = self.bytes.lock().unwrap();
            match guard.as_ref() {
                Some(b) => bincode::deserialize::<BaselineState>(b)
                    .map(Some)
                    .map_err(|_| BaselineError::Deserialize),
                None => Ok(None),
            }
        }
        fn save(&self, state: &BaselineState) -> Result<(), BaselineError> {
            if let Some(err) = self.next_save_error.lock().unwrap().take() {
                return Err(err);
            }
            let bytes = bincode::serialize(state).map_err(|_| BaselineError::Serialize)?;
            *self.bytes.lock().unwrap() = Some(bytes);
            Ok(())
        }
    }

    #[test]
    fn observe_span_with_empty_service_increments_drops() {
        let state = BaselineState::new();
        state.observe_span("", "GET /x", 0, 100, 1_000);
        state.observe_span("", "GET /y", 1, 200, 2_000);
        state.observe_span("svc-a", "GET /z", 0, 50, 3_000);
        assert_eq!(state.service_count(), 1);
        let drops = state.drain_drops_since_last_tick();
        assert_eq!(drops, 2);
        assert_eq!(
            state.drain_drops_since_last_tick(),
            0,
            "drain resets counter"
        );
    }

    #[test]
    fn observe_span_populates_service_baseline() {
        let state = BaselineState::new();
        for i in 0..10 {
            state.observe_span("svc-a", "op-1", 0, 100, i * 1_000_000);
        }
        for i in 0..3 {
            state.observe_span("svc-a", "op-1", 2, 100, (i + 100) * 1_000_000);
        }
        let rate = state.error_rate("svc-a").expect("ewma populated");
        assert!(rate > 0.0 && rate < 1.0, "ewma {rate} should be a fraction");
    }

    #[test]
    fn observe_span_populates_operation_latency_tdigest() {
        let state = BaselineState::new();
        for i in 0..200 {
            state.observe_span("svc-a", "op-1", 0, i as u64 * 10, i as i64 * 1_000_000);
        }
        let p50 = state.latency_percentile("svc-a", "op-1", 0.5).expect("p50");
        assert!(
            p50 > 100.0 && p50 < 2000.0,
            "p50={p50} unreasonable for [0..2000)"
        );
    }

    #[test]
    fn observe_span_drops_empty_operation_name_from_tdigest() {
        let state = BaselineState::new();
        state.observe_span("svc-a", "", 0, 100, 1_000);
        assert_eq!(state.service_count(), 1);
        assert_eq!(state.operation_count(), 0);
    }

    #[test]
    fn run_persist_cycle_advances_persisted_at_via_trait() {
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-1", 0, 100, 1_000);
        assert_eq!(state.persisted_at_unix_nanos(), 0);

        let stats = run_persist_cycle(&state, &persistence, 5_000, "periodic").expect("persist");
        assert!(stats.bytes_written > 0);
        assert_eq!(state.persisted_at_unix_nanos(), 5_000);
        assert!(persistence.bytes.lock().unwrap().is_some());
    }

    #[test]
    fn run_persist_cycle_round_trip_preserves_service_state_via_trait() {
        let persistence = FakeBaselinePersistence::new();
        let original = BaselineState::new();
        for i in 0..50 {
            original.observe_span("svc-a", "op-x", 0, 100, i * 1_000_000);
        }
        run_persist_cycle(&original, &persistence, 5_000, "periodic").expect("persist");

        let loaded = persistence.load().expect("load ok").expect("Some(state)");
        assert_eq!(loaded.service_count(), original.service_count());
        assert_eq!(loaded.persisted_at_unix_nanos(), 5_000);
        assert_eq!(loaded.error_rate("svc-a"), original.error_rate("svc-a"));
    }

    #[test]
    fn bootstrap_state_returns_fresh_when_persistence_none() {
        let state = bootstrap_state(None, DEFAULT_SERVICE_COUNT_CAP, 0);
        assert_eq!(state.service_count(), 0);
        assert_eq!(state.schema_version(), SCHEMA_VERSION);
    }

    #[test]
    fn bootstrap_state_returns_fresh_when_persistence_empty() {
        let persistence = FakeBaselinePersistence::new();
        let state = bootstrap_state(Some(&persistence), DEFAULT_SERVICE_COUNT_CAP, 0);
        assert_eq!(state.service_count(), 0);
        assert_eq!(state.schema_version(), SCHEMA_VERSION);
    }

    #[test]
    fn bootstrap_state_returns_loaded_when_persistence_fresh() {
        let persistence = FakeBaselinePersistence::new();
        let original = BaselineState::new();
        original.observe_span("svc-a", "op-x", 0, 100, 1_000);
        original.set_persisted_at_unix_nanos(1_000);
        persistence.save(&original).expect("save");

        let loaded = bootstrap_state(Some(&persistence), DEFAULT_SERVICE_COUNT_CAP, 2_000);
        assert_eq!(loaded.service_count(), 1);
    }

    #[test]
    fn bootstrap_state_resets_when_persistence_stale_beyond_threshold() {
        let persistence = FakeBaselinePersistence::new();
        let original = BaselineState::new();
        original.observe_span("svc-a", "op-x", 0, 100, 1_000);
        original.set_persisted_at_unix_nanos(0);
        persistence.save(&original).expect("save");

        let now = STATE_AGE_THRESHOLD_NANOS + 1;
        let loaded = bootstrap_state(Some(&persistence), DEFAULT_SERVICE_COUNT_CAP, now);
        assert_eq!(loaded.service_count(), 0, "stale state should reset");
    }

    type CapturedFields = Vec<(String, String)>;
    type CapturedEvent = (String, Level, CapturedFields);
    type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

    #[derive(Default)]
    struct CapturingSubscriber {
        events: CapturedEvents,
    }

    impl CapturingSubscriber {
        fn new() -> (Self, CapturedEvents) {
            let events: CapturedEvents = Arc::new(Mutex::new(Vec::new()));
            (
                Self {
                    events: Arc::clone(&events),
                },
                events,
            )
        }
    }

    struct FieldCollector(Vec<(String, String)>);

    impl Visit for FieldCollector {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            self.0
                .push((field.name().to_string(), format!("{value:?}")));
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_u64(&mut self, field: &Field, value: u64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_i64(&mut self, field: &Field, value: i64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::Id {
            tracing::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::Id, _: &tracing::Id) {}
        fn event(&self, event: &Event<'_>) {
            let target = event.metadata().target().to_string();
            let level = *event.metadata().level();
            let mut collector = FieldCollector(Vec::new());
            event.record(&mut collector);
            self.events
                .lock()
                .unwrap()
                .push((target, level, collector.0));
        }
        fn enter(&self, _: &tracing::Id) {}
        fn exit(&self, _: &tracing::Id) {}
    }

    #[test]
    fn service_identity_missing_emits_single_aggregate_warn_per_tick() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        for _ in 0..10 {
            state.observe_span("", "op", 0, 100, 1_000);
        }
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &persistence, 5_000, "periodic").expect("persist");
        });

        let captured = events.lock().unwrap();
        let missing_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_SERVICE_ID_MISSING)
            .collect();
        assert_eq!(missing_events.len(), 1, "expected exactly 1 aggregate warn");
        let (target, level, fields) = missing_events[0];
        assert_eq!(target, TARGET_SERVICE_ID_MISSING);
        assert_eq!(*level, Level::WARN);
        let dropped = fields
            .iter()
            .find(|(k, _)| k == "dropped_count")
            .map(|(_, v)| v.clone());
        assert_eq!(dropped, Some("10".to_string()));
        let pii_keys = ["service_name", "span_id", "trace_id", "operation_name"];
        for (k, _) in fields {
            assert!(
                !pii_keys.contains(&k.as_str()),
                "PII key {k:?} leaked into warn"
            );
        }
    }

    #[test]
    fn run_persist_cycle_emits_tick_and_persist_metric_events() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-x", 0, 100, 1_000);
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &persistence, 5_000, "periodic").expect("persist");
        });

        let captured = events.lock().unwrap();
        let targets: Vec<&str> = captured.iter().map(|(t, _, _)| t.as_str()).collect();
        assert!(
            targets.contains(&TARGET_BASELINE_TICK),
            "missing tick event"
        );
        assert!(
            targets.contains(&TARGET_METRIC_EWMA_SHORT_WINDOW),
            "missing ewma metric"
        );
        assert!(
            targets.contains(&TARGET_BASELINE_PERSIST),
            "missing persist span event"
        );
        assert!(
            targets.contains(&TARGET_PIPELINE_L1B_PERSIST_TOTAL),
            "missing persist counter"
        );
    }

    #[test]
    fn run_persist_cycle_emits_corpus_basename_field() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-x", 0, 100, 1_000);
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &persistence, 5_000, "periodic").expect("persist");
        });

        let captured = events.lock().unwrap();
        let persist_event = captured
            .iter()
            .find(|(t, _, _)| t == TARGET_BASELINE_PERSIST)
            .expect("missing persist event");
        let basename = persist_event
            .2
            .iter()
            .find(|(k, _)| k == "corpus_basename")
            .map(|(_, v)| v.clone());
        assert_eq!(
            basename,
            Some(CORPUS_BASENAME.to_string()),
            "corpus_basename field must equal CORPUS_BASENAME constant"
        );
    }

    #[test]
    fn persist_on_shutdown_emits_shutdown_kind_metric() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-x", 0, 100, 1_000);
        tracing::subscriber::with_default(sub, || {
            persist_on_shutdown(&state, &persistence).expect("shutdown persist");
        });

        let captured = events.lock().unwrap();
        let persist_total: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_PIPELINE_L1B_PERSIST_TOTAL)
            .collect();
        assert_eq!(persist_total.len(), 1);
        let kind = persist_total[0]
            .2
            .iter()
            .find(|(k, _)| k == "persist_kind")
            .map(|(_, v)| v.clone());
        assert_eq!(kind, Some("shutdown".to_string()));
    }

    #[test]
    fn run_persist_cycle_emits_error_event_on_persistence_failure() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        persistence.fail_next_save_with(BaselineError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        });

        let state = BaselineState::new();
        let result = tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &persistence, 1_000, "periodic")
        });
        assert!(result.is_err(), "expected persist failure");
        assert_eq!(state.persisted_at_unix_nanos(), 0, "rollback on failure");

        let captured = events.lock().unwrap();
        let errs: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_BASELINE_PERSIST_ERROR)
            .collect();
        assert_eq!(errs.len(), 1, "exactly one persist error event");
        let (target, level, fields) = errs[0];
        assert_eq!(target, TARGET_BASELINE_PERSIST_ERROR);
        assert_eq!(*level, Level::WARN);
        let cat = fields
            .iter()
            .find(|(k, _)| k == "error_category")
            .map(|(_, v)| v.clone());
        assert_eq!(cat, Some("io".to_string()));
    }

    #[test]
    fn current_unix_nanos_returns_positive_value() {
        let n = current_unix_nanos();
        assert!(n > 0, "current_unix_nanos returned {n}");
    }

    #[test]
    fn operation_key_is_deterministic_and_includes_service_prefix() {
        let k1 = operation_key("svc-a", "GET /x");
        let k2 = operation_key("svc-a", "GET /x");
        let k3 = operation_key("svc-b", "GET /x");
        assert_eq!(k1, k2, "same inputs → same key");
        assert_ne!(k1, k3, "different service → different key");
        assert!(k1.starts_with("svc-a/"), "key prefix carries service.name");
    }

    #[test]
    fn operation_name_round_trips_through_baseline_to_snapshot() {
        // Chunk #73 P-011: human-readable operation_name is preserved on
        // OperationBaseline + flows through iter_operations into the
        // OperationMetricSnapshot. Sampled at percentile q=0.99 (chunk #73
        // post-P-012); name preserved regardless of latency value.
        let state = BaselineState::new();
        let now = 1_000_000_000_i64;
        for i in 0..100 {
            state.observe_span("svc-a", "GET /api/users", 0, 50, now + i * 1_000_000);
        }
        let ops = state.iter_operations(0.99);
        assert_eq!(ops.len(), 1, "expected one operation, got {}", ops.len());
        assert_eq!(
            ops[0].operation_name, "GET /api/users",
            "operation_name round-trip preserved through baseline → snapshot"
        );
        assert!(
            ops[0].operation_key.starts_with("svc-a/"),
            "operation_key still carries hashed identity for DashMap key"
        );
    }

    #[test]
    fn operation_name_distinguishes_two_operations_on_same_service() {
        // Chunk #73 P-011: two different operation_names on the same service
        // produce two distinct DashMap entries (full operation_key strings
        // differ); each preserves its own human-readable name.
        let state = BaselineState::new();
        let now = 1_000_000_000_i64;
        for i in 0..100 {
            state.observe_span("svc-a", "GET /api/users", 0, 50, now + i * 1_000_000);
            state.observe_span("svc-a", "POST /api/orders", 0, 75, now + i * 1_000_000);
        }
        let mut ops = state.iter_operations(0.99);
        ops.sort_by(|a, b| a.operation_name.cmp(&b.operation_name));
        assert_eq!(ops.len(), 2, "expected two operations, got {}", ops.len());
        assert_eq!(ops[0].operation_name, "GET /api/users");
        assert_eq!(ops[1].operation_name, "POST /api/orders");
        assert_ne!(
            ops[0].operation_key, ops[1].operation_key,
            "distinct operations produce distinct operation_keys"
        );
    }

    #[test]
    fn bootstrap_state_emits_cold_start_metric() {
        let (sub, events) = CapturingSubscriber::new();
        tracing::subscriber::with_default(sub, || {
            let _ = bootstrap_state(None, DEFAULT_SERVICE_COUNT_CAP, 0);
        });

        let captured = events.lock().unwrap();
        let bootstrap_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_PIPELINE_L1B_BOOTSTRAP_TOTAL)
            .collect();
        assert_eq!(bootstrap_events.len(), 1);
        let kind = bootstrap_events[0]
            .2
            .iter()
            .find(|(k, _)| k == "kind")
            .map(|(_, v)| v.clone());
        assert_eq!(kind, Some("cold_start".to_string()));
    }

    #[test]
    fn observe_span_routes_to_activity_floor_observe() {
        let state = BaselineState::new();
        let first = 1_000 * 1_000_000_000_i64;
        state.observe_span("svc-a", "op", 0, 50, first);
        let snapshots = state.iter_service_silence_snapshots(first);
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].service_name, "svc-a");
        assert_eq!(snapshots[0].current_quiet_duration_seconds, 0);
        assert_eq!(snapshots[0].bootstrap_state, BootstrapState::Learning);
        let later = first + 60 * 1_000_000_000;
        let snapshots_later = state.iter_service_silence_snapshots(later);
        assert_eq!(snapshots_later[0].current_quiet_duration_seconds, 60);
    }

    #[test]
    fn observe_span_enforces_service_cardinality_cap() {
        let state = BaselineState::new();
        // Fill exactly to the cap with distinct services.
        for i in 0..ACTIVITY_FLOOR_SERVICE_CAP {
            let name = format!("svc-{i}");
            state.observe_span(&name, "op", 0, 50, 1_000_000);
        }
        assert_eq!(state.service_count(), ACTIVITY_FLOOR_SERVICE_CAP);
        let cap_drops_before = state.drain_service_cap_exceeded_since_last_tick();
        assert_eq!(cap_drops_before, 0);
        // Emit additional 10 distinct service names → all dropped.
        for i in 0..10 {
            let name = format!("over-cap-{i}");
            state.observe_span(&name, "op", 0, 50, 2_000_000);
        }
        // Service count stays at cap; cap counter increments by 10.
        assert_eq!(state.service_count(), ACTIVITY_FLOOR_SERVICE_CAP);
        let cap_drops = state.drain_service_cap_exceeded_since_last_tick();
        assert_eq!(cap_drops, 10);
        assert_eq!(
            state.drain_service_cap_exceeded_since_last_tick(),
            0,
            "drain resets counter"
        );
        // Existing services can still observe (cap check skipped on contains_key).
        state.observe_span("svc-0", "op", 0, 50, 3_000_000);
        assert_eq!(state.service_count(), ACTIVITY_FLOOR_SERVICE_CAP);
    }

    #[test]
    fn service_cap_exceeded_emits_aggregate_warn_per_tick() {
        let (sub, events) = CapturingSubscriber::new();
        let persistence = FakeBaselinePersistence::new();
        let state = BaselineState::new();
        for i in 0..ACTIVITY_FLOOR_SERVICE_CAP {
            state.observe_span(&format!("svc-{i}"), "op", 0, 50, 1_000_000);
        }
        for i in 0..5 {
            state.observe_span(&format!("over-{i}"), "op", 0, 50, 2_000_000);
        }
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &persistence, 5_000, "periodic").expect("persist");
        });

        let captured = events.lock().unwrap();
        let cap_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_SERVICE_CAP_EXCEEDED)
            .collect();
        assert_eq!(cap_events.len(), 1, "expected exactly one aggregate warn");
        let (target, level, fields) = cap_events[0];
        assert_eq!(target, TARGET_SERVICE_CAP_EXCEEDED);
        assert_eq!(*level, Level::WARN);
        let dropped = fields
            .iter()
            .find(|(k, _)| k == "dropped_count")
            .map(|(_, v)| v.clone());
        assert_eq!(dropped, Some("5".to_string()));
        // PII guard: aggregate warn must not carry per-service identifiers.
        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ];
        for (k, _) in fields {
            assert!(
                !banned.contains(&k.as_str()),
                "PII key {k:?} leaked into cap warn"
            );
        }
    }
}
