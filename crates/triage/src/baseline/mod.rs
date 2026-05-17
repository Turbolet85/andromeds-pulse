//! Streaming baseline trackers + corpus persistence (chunk #61).
//!
//! Implements the L1b streaming distillation foundation per
//! `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 2 (capabilities P-009 +
//! P-011). Three algorithm primitives — `EwmaTracker` (per-service error
//! rate baseline), `TDigestPair` (per-operation streaming p50/p95/p99
//! latency baseline with swap-on-tick rotation), and `RollingWindow<u32>`
//! (per-service activity tracking) — feed `BaselineState`, a DashMap-backed
//! aggregator that persists to a self-versioned bincode corpus file under
//! `<data-dir>/triage/baseline-corpus.bin`.
//!
//! Boot wiring (spawn `run_persist_loop`, tap the ingest span stream) is
//! deferred to chunk #62; this chunk delivers callable + testable primitives
//! only.

mod corpus;
mod error;
mod ewma;
mod rolling_window;
mod tdigest_pair;

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::time::Duration;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

#[cfg(test)]
pub(crate) use corpus::load_state;
pub use corpus::{
    BootstrapResult, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP, PersistStats,
    bootstrap_from_corpus, persist_state, resolve_corpus_path,
};
pub use error::BaselineError;
pub use ewma::EwmaTracker;
pub use rolling_window::RollingWindow;
pub use tdigest_pair::TDigestPair;

pub const SCHEMA_VERSION: u32 = 1;

pub(crate) const TARGET_BASELINE_TICK: &str = "triage.baseline.tick";
pub(crate) const TARGET_BASELINE_PERSIST: &str = "triage.baseline.persist";
pub(crate) const TARGET_BASELINE_PERSIST_ERROR: &str = "triage.baseline.persist.error";
pub(crate) const TARGET_SERVICE_ID_MISSING: &str = "triage.service_id_missing";
pub(crate) const TARGET_METRIC_EWMA_SHORT_WINDOW: &str = "metric.baseline.ewma_short_window_size";
pub(crate) const TARGET_PIPELINE_L1B_PERSIST_TOTAL: &str = "pipeline.l1b.persist_count_total";
pub(crate) const TARGET_PIPELINE_L1B_BOOTSTRAP_TOTAL: &str = "pipeline.l1b.bootstrap_count_total";

pub const DEFAULT_ALPHA_5MIN_WINDOW: f64 = 0.00333;
pub const DEFAULT_PERSIST_INTERVAL_NANOS: i64 = 60_000_000_000;
pub const STATE_AGE_THRESHOLD_NANOS: i64 = 3_600_000_000_000;
pub(crate) const DEFAULT_ACTIVITY_WINDOW_CAPACITY: usize = 300;

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

#[derive(Debug, Default, Serialize, Deserialize)]
struct ServiceBaseline {
    error_rate_ewma: EwmaTracker,
    activity_window: RollingWindow<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct OperationBaseline {
    latency_tdigest: TDigestPair,
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

    /// Per-spec entry point. Drops spans с empty service.name (increments
    /// `drops_since_last_tick`; aggregate warn fires from per-tick emitter).
    /// Updates the service's error-rate EWMA + per-second activity bucket +
    /// the (service, operation) tuple's latency t-digest.
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

        let error_observation = if status_code == STATUS_CODE_ERROR {
            1.0
        } else {
            0.0
        };
        let mut svc = self.services.entry(service_name.to_string()).or_default();
        svc.error_rate_ewma.observe(error_observation, now_nanos);
        svc.activity_window.push(1);
        drop(svc);

        if !operation_name.is_empty() {
            let key = operation_key(service_name, operation_name);
            let mut op = self.operations.entry(key).or_default();
            op.latency_tdigest.insert(latency_ms as f64);
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
/// drain service-identity drops + emit aggregate warn, persist corpus,
/// emit persist outcome metric. Returns the persist result for callers
/// that want explicit error handling. Designed for unit-testable invocation
/// independent of the `tokio::time::interval`-driven outer loop.
pub fn run_persist_cycle(
    state: &BaselineState,
    corpus_path: &Path,
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

    let persist_start = std::time::Instant::now();
    let corpus_basename = corpus_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("baseline-corpus.bin")
        .to_string();

    // Set persisted_at BEFORE serialize so the round-trip includes it.
    // If persist fails, the in-memory atomic is "ahead" of the disk
    // version; next successful persist re-syncs. Acceptable trade-off.
    let prior_persisted_at = state.persisted_at_unix_nanos();
    state.set_persisted_at_unix_nanos(now_nanos);

    match persist_state(corpus_path, state) {
        Ok(stats) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            tracing::info!(
                target: TARGET_BASELINE_PERSIST,
                service_id_count = stats.service_count as u64,
                state_size_bytes = stats.bytes_written,
                duration_ms = duration_ms,
                persist_kind = persist_kind,
                corpus_basename = %corpus_basename,
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

/// Long-running future spawned at boot (chunk #62 territory) that fires
/// `run_persist_cycle` on the supplied `interval`. Wall-clock time is read
/// from `SystemTime::UNIX_EPOCH`; tests should exercise `run_persist_cycle`
/// directly with injected `now_nanos` for deterministic timing.
pub async fn run_persist_loop(
    state: Arc<BaselineState>,
    corpus_path: std::path::PathBuf,
    interval: Duration,
) {
    let mut ticker = tokio::time::interval(interval);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let now = current_unix_nanos();
        let _ = run_persist_cycle(&state, &corpus_path, now, "periodic");
    }
}

/// Graceful-shutdown persist helper. Synchronous so it can be called from a
/// Tauri shutdown hook (chunk #62 boot wiring). Emits the persist counter
/// with `persist_kind = "shutdown"`.
pub fn persist_on_shutdown(
    state: &BaselineState,
    corpus_path: &Path,
) -> Result<PersistStats, BaselineError> {
    let now = current_unix_nanos();
    run_persist_cycle(state, corpus_path, now, "shutdown")
}

/// Bootstrap entry point used at boot to attempt load-from-corpus, falling
/// through to fresh state on any failure. Emits the
/// `pipeline.l1b.bootstrap_count_total{kind}` metric describing the path
/// taken. Returns a ready-to-use `BaselineState`.
pub fn bootstrap_state(
    corpus_path: &Path,
    max_size_bytes: u64,
    service_count_cap: usize,
    now_nanos: i64,
) -> BaselineState {
    let kind: &'static str;
    let state =
        match bootstrap_from_corpus(corpus_path, max_size_bytes, service_count_cap, now_nanos) {
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
    use tempfile::TempDir;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

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
    fn run_persist_cycle_writes_corpus_and_advances_persisted_at() {
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-1", 0, 100, 1_000);
        assert_eq!(state.persisted_at_unix_nanos(), 0);

        let stats = run_persist_cycle(&state, &path, 5_000, "periodic").expect("persist");
        assert!(stats.bytes_written > 0);
        assert_eq!(state.persisted_at_unix_nanos(), 5_000);
        assert!(path.exists());
    }

    #[test]
    fn run_persist_cycle_round_trip_preserves_service_state() {
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let original = BaselineState::new();
        for i in 0..50 {
            original.observe_span("svc-a", "op-x", 0, 100, i * 1_000_000);
        }
        run_persist_cycle(&original, &path, 5_000, "periodic").expect("persist");

        let loaded =
            load_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP).expect("load");
        assert_eq!(loaded.service_count(), original.service_count());
        assert_eq!(loaded.persisted_at_unix_nanos(), 5_000);
        assert_eq!(loaded.error_rate("svc-a"), original.error_rate("svc-a"));
    }

    #[test]
    fn bootstrap_state_returns_fresh_when_corpus_missing() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("nonexistent-corpus.bin");
        let state = bootstrap_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP, 0);
        assert_eq!(state.service_count(), 0);
        assert_eq!(state.schema_version(), SCHEMA_VERSION);
    }

    #[test]
    fn bootstrap_state_returns_loaded_when_corpus_fresh() {
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let original = BaselineState::new();
        original.observe_span("svc-a", "op-x", 0, 100, 1_000);
        original.set_persisted_at_unix_nanos(1_000);
        persist_state(&path, &original).expect("persist");

        let loaded = bootstrap_state(
            &path,
            DEFAULT_MAX_SIZE_BYTES,
            DEFAULT_SERVICE_COUNT_CAP,
            2_000,
        );
        assert_eq!(loaded.service_count(), 1);
    }

    #[test]
    fn bootstrap_state_resets_when_corpus_stale_beyond_threshold() {
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let original = BaselineState::new();
        original.observe_span("svc-a", "op-x", 0, 100, 1_000);
        original.set_persisted_at_unix_nanos(0);
        persist_state(&path, &original).expect("persist");

        let now = STATE_AGE_THRESHOLD_NANOS + 1;
        let loaded = bootstrap_state(
            &path,
            DEFAULT_MAX_SIZE_BYTES,
            DEFAULT_SERVICE_COUNT_CAP,
            now,
        );
        assert_eq!(loaded.service_count(), 0, "stale corpus should reset");
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
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        for _ in 0..10 {
            state.observe_span("", "op", 0, 100, 1_000);
        }
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &path, 5_000, "periodic").expect("persist");
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
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-x", 0, 100, 1_000);
        tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &path, 5_000, "periodic").expect("persist");
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
    fn persist_on_shutdown_emits_shutdown_kind_metric() {
        let (sub, events) = CapturingSubscriber::new();
        let tmp = TempDir::new().expect("tmp");
        let path = resolve_corpus_path(tmp.path()).expect("resolve");
        let state = BaselineState::new();
        state.observe_span("svc-a", "op-x", 0, 100, 1_000);
        tracing::subscriber::with_default(sub, || {
            persist_on_shutdown(&state, &path).expect("shutdown persist");
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
    fn run_persist_cycle_emits_error_event_on_io_failure() {
        let (sub, events) = CapturingSubscriber::new();
        let tmp = TempDir::new().expect("tmp");
        // Point at a path whose parent is a regular file — fs::write fails.
        let blocker = tmp.path().join("blocker");
        std::fs::write(&blocker, b"x").expect("write blocker");
        let bad_path = blocker.join("corpus.bin");

        let state = BaselineState::new();
        let result = tracing::subscriber::with_default(sub, || {
            run_persist_cycle(&state, &bad_path, 1_000, "periodic")
        });
        assert!(result.is_err(), "expected persist failure");
        assert_eq!(state.persisted_at_unix_nanos(), 0, "rollback on failure");

        let captured = events.lock().unwrap();
        let errs: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_BASELINE_PERSIST_ERROR)
            .collect();
        assert_eq!(errs.len(), 1, "exactly one persist error event");
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
    fn bootstrap_state_emits_cold_start_metric() {
        let (sub, events) = CapturingSubscriber::new();
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("nonexistent.bin");
        tracing::subscriber::with_default(sub, || {
            let _ = bootstrap_state(&path, DEFAULT_MAX_SIZE_BYTES, DEFAULT_SERVICE_COUNT_CAP, 0);
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
}
