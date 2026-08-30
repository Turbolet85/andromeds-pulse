//! Retry storm detector — chunk #66.
//!
//! L2 distillation layer per pulse v0.2.0 plan Phase 2 line 240. Tracks
//! per-exception-fingerprint occurrences in a 60-second rolling window;
//! when occurrences within a 30-second detection sub-window cross
//! configured thresholds, emits a `CueKind::RetryStorm` attention cue
//! through the existing chunk #62 `pulse://stream/attention-cues`
//! broadcast channel.
//!
//! Capability spec coverage: P-017 (Exception Fingerprinting — detector
//! side) + P-018 (Retry Storm Detection).
//!
//! Per arch §Cross-cutting Patterns Module dependency direction, this
//! module receives opaque `[u8; 16]` fingerprint bytes from the buffer
//! crate via the `FingerprintObserver` trait declared in
//! `crates/buffer/src/fingerprint.rs`; no hash compute happens here.
//! Cross-crate wiring lives at the `pulse-app` binary boundary
//! (`pulse-app/src/storm_observer.rs::StormObserverAdapter`).
//!
//! ## Threshold + dedup semantics (per chunk #66 plan)
//!
//! - `count >= autonomous_threshold` (default 10) within the detection
//!   sub-window → emit `RetryStorm` cue with `priority_tier: Autonomous`,
//!   ONE-SHOT per fingerprint until detection window resets via timestamp
//!   pruning. Escalation FROM Suggested TO Autonomous emits one cue at
//!   the threshold crossing.
//! - `count >= suggested_threshold` (default 5) within the detection
//!   sub-window → emit `RetryStorm` cue with `priority_tier: Suggested`,
//!   ONE-SHOT per fingerprint until detection window resets.
//! - Otherwise → no cue.
//! - Per-tier dedup avoids log-spam under sustained high-rate storms.
//!
//! ## State (corpus-backed via chunk #71)
//!
//! `DashMap<[u8; 16], FingerprintState>` — concurrent map keyed by
//! fingerprint, value tracks `service` (first-observed), `timestamps_nanos`
//! Vec (sorted by insertion order; pruned per tick + per record), and
//! `last_emitted` (tier + ts for dedup). State persisted via chunk #71
//! `StormPersistence` trait + `CorpusStormPersistence` adapter at
//! `pulse-app/src/storm_persistence.rs`. Restart preserves dedup window
//! per capability P-018 closure.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::persistence::StormStateSnapshot;
use crate::contract::{
    AttentionCue, AttentionCueBroadcast, CueKind, CueScope, PriorityTier, hex_lower,
};

use super::{
    TARGET_METRIC_FINGERPRINT_EVICTED_COUNT, TARGET_METRIC_FINGERPRINTS_TRACKED,
    TARGET_METRIC_STORM_DETECTED_COUNT, TARGET_PATTERN_STORM_DETECTED, TARGET_PATTERN_STORM_EMIT,
    TARGET_PATTERN_STORM_TICK,
};

/// Default rolling-window for fingerprint occurrence retention. Timestamps
/// older than this are pruned on every `record_occurrence` AND every
/// heartbeat tick.
pub const DEFAULT_STORM_WINDOW_SECONDS: u64 = 60;

/// Default detection sub-window: occurrences counted within this many
/// seconds before `now_nanos` are the trigger population. Smaller than the
/// retention window so storm detection responds to recent activity even when
/// older occurrences linger in the rolling state.
pub const DEFAULT_DETECTION_SUB_WINDOW_SECONDS: u64 = 30;

/// Default threshold for `Suggested` cue emission: 5 occurrences within
/// detection sub-window. One-shot per fingerprint until window resets.
pub const DEFAULT_SUGGESTED_THRESHOLD: u64 = 5;

/// Default threshold for `Autonomous` cue emission: 10 occurrences within
/// detection sub-window. Escalates from a prior `Suggested` emit OR fires
/// fresh if no prior emit; one-shot per fingerprint until window resets.
pub const DEFAULT_AUTONOMOUS_THRESHOLD: u64 = 10;

/// Per-fingerprint storm-tracking state. Public (chunk #71) so the
/// `StormStateSnapshot` exported via `crate::pattern::persistence` can
/// reference this type via its `entries: Vec<([u8; 16], FingerprintState)>`
/// field. Fields stay private — external crates hold the opaque value
/// type via bincode serialize/deserialize but cannot introspect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FingerprintState {
    /// `service.name` from first observation. Subsequent observations do
    /// NOT re-key by service; cross-service propagation of the same
    /// fingerprint contributes to one storm cue attributed to first-observed.
    pub(crate) service: String,
    /// Occurrence timestamps within the rolling window (nanoseconds).
    /// Pruned during `record_occurrence` and `run_one_storm_cycle`.
    pub(crate) timestamps_nanos: Vec<i64>,
    /// `(emit_ts_nanos, tier)` of the most recent cue emission for this
    /// fingerprint; `None` if no cue has fired in the current window OR
    /// if the window has fully expired since the last emit.
    pub(crate) last_emitted: Option<(i64, PriorityTier)>,
}

/// Retry storm detector — chunk #66 + chunk #71 corpus persistence.
///
/// State persisted via `crate::pattern::persistence::StormPersistence`
/// trait — see `pulse-app/src/storm_persistence.rs` for the
/// `CorpusStormPersistence` adapter wiring.
#[derive(Debug)]
pub struct RetryStormDetector {
    fingerprints: DashMap<[u8; 16], FingerprintState>,
    window_seconds: u64,
    detection_window_seconds: u64,
    suggested_threshold: u64,
    autonomous_threshold: u64,
    storms_detected_total: AtomicU64,
    fingerprints_evicted_total: AtomicU64,
}

impl RetryStormDetector {
    pub fn new(
        window_seconds: u64,
        detection_window_seconds: u64,
        suggested_threshold: u64,
        autonomous_threshold: u64,
    ) -> Self {
        Self {
            fingerprints: DashMap::new(),
            window_seconds,
            detection_window_seconds,
            suggested_threshold,
            autonomous_threshold,
            storms_detected_total: AtomicU64::new(0),
            fingerprints_evicted_total: AtomicU64::new(0),
        }
    }

    pub fn fingerprints_tracked(&self) -> usize {
        self.fingerprints.len()
    }

    pub fn storms_detected_total(&self) -> u64 {
        self.storms_detected_total.load(Ordering::Relaxed)
    }

    pub fn fingerprints_evicted_total(&self) -> u64 {
        self.fingerprints_evicted_total.load(Ordering::Relaxed)
    }

    /// Materialize a serializable snapshot of the detector's state +
    /// configuration knobs. Iteration over the DashMap is safe under
    /// concurrent observation because each entry returns a clone; the
    /// snapshot represents an at-time-of-call view (may be slightly
    /// stale by the time persist completes, which is acceptable for
    /// the 60s persist cadence — see chunk #71 plan §Storm persist
    /// cadence tuning deferred decision).
    pub fn snapshot(&self) -> StormStateSnapshot {
        let entries: Vec<([u8; 16], FingerprintState)> = self
            .fingerprints
            .iter()
            .map(|entry| (*entry.key(), entry.value().clone()))
            .collect();
        StormStateSnapshot {
            entries,
            window_seconds: self.window_seconds,
            detection_window_seconds: self.detection_window_seconds,
            suggested_threshold: self.suggested_threshold,
            autonomous_threshold: self.autonomous_threshold,
        }
    }

    /// Rebuild a detector from a previously-persisted snapshot. Used at
    /// boot when corpus has a non-empty `storm_state` row; preserves
    /// fingerprint dedup state across restart per capability P-018.
    /// Counters (`storms_detected_total`, `fingerprints_evicted_total`)
    /// reset to 0 on restore — they are per-session monotonic metrics
    /// not persisted across boots.
    pub fn restore_from_snapshot(snapshot: StormStateSnapshot) -> Self {
        let fingerprints = DashMap::with_capacity(snapshot.entries.len());
        for (key, state) in snapshot.entries {
            fingerprints.insert(key, state);
        }
        Self {
            fingerprints,
            window_seconds: snapshot.window_seconds,
            detection_window_seconds: snapshot.detection_window_seconds,
            suggested_threshold: snapshot.suggested_threshold,
            autonomous_threshold: snapshot.autonomous_threshold,
            storms_detected_total: AtomicU64::new(0),
            fingerprints_evicted_total: AtomicU64::new(0),
        }
    }
}

/// Stats captured during one heartbeat tick. Public for test inspection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StormCycleStats {
    pub fingerprints_tracked: usize,
    pub storms_detected_total: u64,
    pub fingerprints_evicted_total: u64,
    pub window_seconds: u64,
}

/// Record a fingerprint occurrence; return `Some(AttentionCue)` if a
/// threshold crossing fires a new cue, else `None`. Idempotent with respect
/// to dedup state (see module-level threshold + dedup semantics).
///
/// Empty `service` is dropped per chunk #61 service identity discipline
/// (mirror `RestartDetector::observe_span`).
pub fn record_occurrence(
    detector: &RetryStormDetector,
    fingerprint: [u8; 16],
    service: &str,
    now_nanos: i64,
) -> Option<AttentionCue> {
    if service.is_empty() {
        return None;
    }

    let window_nanos = (detector.window_seconds as i64).saturating_mul(1_000_000_000);
    let detection_nanos = (detector.detection_window_seconds as i64).saturating_mul(1_000_000_000);

    let mut state_ref =
        detector
            .fingerprints
            .entry(fingerprint)
            .or_insert_with(|| FingerprintState {
                service: service.to_string(),
                timestamps_nanos: Vec::new(),
                last_emitted: None,
            });

    state_ref
        .timestamps_nanos
        .retain(|ts| now_nanos.saturating_sub(*ts) <= window_nanos);

    if state_ref.timestamps_nanos.is_empty() && state_ref.last_emitted.is_some() {
        state_ref.last_emitted = None;
    }

    state_ref.timestamps_nanos.push(now_nanos);

    let count = state_ref
        .timestamps_nanos
        .iter()
        .filter(|ts| now_nanos.saturating_sub(**ts) <= detection_nanos)
        .count() as u64;

    let should_emit_autonomous = count >= detector.autonomous_threshold
        && match state_ref.last_emitted {
            None => true,
            Some((_, PriorityTier::Suggested)) => true,
            Some((_, PriorityTier::Autonomous)) => false,
            Some((_, PriorityTier::Curious)) => true,
        };

    if should_emit_autonomous {
        detector
            .storms_detected_total
            .fetch_add(1, Ordering::Relaxed);
        let cue = synthesize_cue(
            &state_ref.service,
            count,
            PriorityTier::Autonomous,
            now_nanos,
            &state_ref.timestamps_nanos,
            detector,
            &fingerprint,
        );
        state_ref.last_emitted = Some((now_nanos, PriorityTier::Autonomous));
        return Some(cue);
    }

    let should_emit_suggested =
        count >= detector.suggested_threshold && state_ref.last_emitted.is_none();

    if should_emit_suggested {
        detector
            .storms_detected_total
            .fetch_add(1, Ordering::Relaxed);
        let cue = synthesize_cue(
            &state_ref.service,
            count,
            PriorityTier::Suggested,
            now_nanos,
            &state_ref.timestamps_nanos,
            detector,
            &fingerprint,
        );
        state_ref.last_emitted = Some((now_nanos, PriorityTier::Suggested));
        return Some(cue);
    }

    None
}

fn synthesize_cue(
    service: &str,
    count: u64,
    tier: PriorityTier,
    now_nanos: i64,
    timestamps: &[i64],
    detector: &RetryStormDetector,
    fingerprint: &[u8; 16],
) -> AttentionCue {
    let suggested_threshold = detector.suggested_threshold;
    let autonomous_threshold = detector.autonomous_threshold;
    let magnitude = count as f64 / (suggested_threshold.max(1) as f64);
    let absolute_value = count as f64;
    let confidence = (count as f64 / (autonomous_threshold.max(1) as f64)).min(1.0);
    let persistence = timestamps
        .first()
        .map(|oldest| (now_nanos.saturating_sub(*oldest) / 1_000_000_000).max(0) as u64)
        .unwrap_or(0);
    AttentionCue {
        kind: CueKind::RetryStorm,
        scope: CueScope::Service,
        scope_id: Some(service.to_string()),
        magnitude,
        absolute_value,
        persistence,
        confidence,
        priority_tier: tier,
        suppression_bypassed: false,
        // FULL-width hex, not `fingerprint_to_hex_prefix` (4 bytes): the
        // corpus-retrieval arm compares against `hex_lower` of all 16 Q3
        // bytes, so a prefix here could never match.
        fingerprint: Some(hex_lower(fingerprint)),
    }
}

/// Observe one fingerprint occurrence + dispatch the resulting cue (if
/// any) via the broadcast channel + tracing emission. Mirror of chunk #63
/// `pattern::observe_and_dispatch` shape; the broadcast send is benign
/// when no subscribers are attached per chunk #62 precedent.
pub fn observe_and_dispatch_storm(
    detector: &RetryStormDetector,
    broadcast: &AttentionCueBroadcast,
    fingerprint: [u8; 16],
    service: &str,
    now_nanos: i64,
) {
    let Some(cue) = record_occurrence(detector, fingerprint, service, now_nanos) else {
        return;
    };

    let severity_label = priority_tier_label(cue.priority_tier);
    let occurrence_count = cue.absolute_value as u64;
    let fp_hex = fingerprint_to_hex_prefix(&fingerprint);

    info!(
        target: TARGET_PATTERN_STORM_DETECTED,
        cue_kind = "retry_storm",
        severity_hint = severity_label,
        occurrence_count = occurrence_count,
        window_seconds = detector.detection_window_seconds,
        fingerprint_hex = fp_hex.as_str(),
    );

    let _ = broadcast.sender().send(cue);

    info!(
        target: TARGET_PATTERN_STORM_EMIT,
        cue_kind = "retry_storm",
        severity_hint = severity_label,
        occurrence_count = occurrence_count,
    );

    info!(
        target: TARGET_METRIC_STORM_DETECTED_COUNT,
        value = 1u64,
        severity_hint = severity_label,
    );
}

/// Run one heartbeat cycle: prune expired fingerprints, emit observability
/// events with the current snapshot. Returns the snapshot for test inspection.
/// Pure synchronous + idempotent at a given `now_nanos`; safe to call from
/// `tokio::time::interval` tick loop or directly from tests.
pub fn run_one_storm_cycle(detector: &RetryStormDetector, now_nanos: i64) -> StormCycleStats {
    let window_nanos = (detector.window_seconds as i64).saturating_mul(1_000_000_000);

    let mut evicted_this_cycle = 0u64;
    detector.fingerprints.retain(|_fp, state| {
        state
            .timestamps_nanos
            .retain(|ts| now_nanos.saturating_sub(*ts) <= window_nanos);
        let keep = !state.timestamps_nanos.is_empty();
        if !keep {
            evicted_this_cycle += 1;
        }
        keep
    });
    if evicted_this_cycle > 0 {
        detector
            .fingerprints_evicted_total
            .fetch_add(evicted_this_cycle, Ordering::Relaxed);
    }

    let stats = StormCycleStats {
        fingerprints_tracked: detector.fingerprints.len(),
        storms_detected_total: detector.storms_detected_total.load(Ordering::Relaxed),
        fingerprints_evicted_total: detector.fingerprints_evicted_total.load(Ordering::Relaxed),
        window_seconds: detector.window_seconds,
    };

    info!(
        target: TARGET_PATTERN_STORM_TICK,
        value = 1u64,
        tracked_fingerprints_count = stats.fingerprints_tracked as u64,
        storms_detected_total = stats.storms_detected_total,
        fingerprints_evicted_total = stats.fingerprints_evicted_total,
        window_seconds = stats.window_seconds,
    );

    info!(
        target: TARGET_METRIC_FINGERPRINTS_TRACKED,
        value = stats.fingerprints_tracked as u64,
    );

    info!(
        target: TARGET_METRIC_FINGERPRINT_EVICTED_COUNT,
        value = stats.fingerprints_evicted_total,
    );

    stats
}

/// Long-running heartbeat task spawned at boot. Fires `run_one_storm_cycle`
/// at the supplied interval. Skips the immediate first tick to mirror
/// chunk #62 emitter + chunk #63 detector convention.
pub async fn start_storm_detector(detector: Arc<RetryStormDetector>, heartbeat_interval: Duration) {
    let mut interval = tokio::time::interval(heartbeat_interval);
    interval.tick().await;
    loop {
        interval.tick().await;
        let now_nanos = current_time_unix_nano();
        let _ = run_one_storm_cycle(&detector, now_nanos);
    }
}

fn current_time_unix_nano() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

fn priority_tier_label(tier: PriorityTier) -> &'static str {
    match tier {
        PriorityTier::Autonomous => "autonomous",
        PriorityTier::Suggested => "suggested",
        PriorityTier::Curious => "curious",
    }
}

/// Format the fingerprint's first 4 bytes as 8-char lowercase hex for
/// bounded-cardinality tracing field emission. Local helper mirroring
/// `buffer::fingerprint::fingerprint_to_hex_prefix` shape; no buffer dep.
fn fingerprint_to_hex_prefix(fingerprint: &[u8; 16]) -> String {
    let mut out = String::with_capacity(8);
    for byte in &fingerprint[..4] {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

    const NANOS_PER_SEC: i64 = 1_000_000_000;
    const FP_A: [u8; 16] = [0xAA; 16];
    const FP_B: [u8; 16] = [0xBB; 16];

    fn fresh_detector() -> RetryStormDetector {
        RetryStormDetector::new(
            DEFAULT_STORM_WINDOW_SECONDS,
            DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
            DEFAULT_SUGGESTED_THRESHOLD,
            DEFAULT_AUTONOMOUS_THRESHOLD,
        )
    }

    #[test]
    fn default_thresholds_match_spec() {
        assert_eq!(DEFAULT_STORM_WINDOW_SECONDS, 60);
        assert_eq!(DEFAULT_DETECTION_SUB_WINDOW_SECONDS, 30);
        assert_eq!(DEFAULT_SUGGESTED_THRESHOLD, 5);
        assert_eq!(DEFAULT_AUTONOMOUS_THRESHOLD, 10);
    }

    /// Emit one cue by crossing the suggested threshold, returning it.
    fn emit_cue_with_fingerprint(fp: [u8; 16]) -> AttentionCue {
        let d = fresh_detector();
        let mut cue = None;
        for i in 0..DEFAULT_SUGGESTED_THRESHOLD {
            cue = record_occurrence(&d, fp, "svc", (1_000 + i as i64) * NANOS_PER_SEC);
        }
        cue.expect("suggested threshold must emit a cue")
    }

    #[test]
    fn synthesized_cue_carries_full_width_hex_fingerprint_not_the_tracing_prefix() {
        let cue = emit_cue_with_fingerprint(FP_A);
        let fp = cue.fingerprint.expect("storm cue must carry a fingerprint");

        // Width is the load-bearing assertion. The adjacent, correctly-named
        // `fingerprint_to_hex_prefix` yields 8 chars; the corpus-retrieval arm
        // compares against `hex_lower` of all 16 bytes, so a prefix could never
        // match and a shape-only "is it hex?" check would pass either way.
        assert_eq!(fp.len(), 32, "must encode all 16 bytes, got {fp}");
        assert_eq!(fp, hex_lower(&FP_A));
        assert_ne!(
            fp,
            fingerprint_to_hex_prefix(&FP_A),
            "the 4-byte tracing prefix must never be used as the cue fingerprint"
        );
        assert!(
            fp.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
        );
    }

    #[test]
    fn distinct_fingerprints_yield_distinct_cue_fingerprints() {
        let a = emit_cue_with_fingerprint(FP_A).fingerprint;
        let b = emit_cue_with_fingerprint(FP_B).fingerprint;
        assert!(a.is_some() && b.is_some());
        assert_ne!(a, b, "distinct faults must stay distinguishable");
    }

    #[test]
    fn record_occurrence_with_empty_service_dropped() {
        let d = fresh_detector();
        assert!(record_occurrence(&d, FP_A, "", 1_000 * NANOS_PER_SEC).is_none());
        assert_eq!(d.fingerprints_tracked(), 0);
    }

    #[test]
    fn record_occurrence_first_observation_returns_none() {
        let d = fresh_detector();
        let r = record_occurrence(&d, FP_A, "svc", 1_000 * NANOS_PER_SEC);
        assert!(r.is_none());
        assert_eq!(d.fingerprints_tracked(), 1);
    }

    #[test]
    fn record_occurrence_4_occurrences_in_30s_returns_none() {
        let d = fresh_detector();
        for i in 0..4 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            assert!(record_occurrence(&d, FP_A, "svc", ts).is_none());
        }
        assert_eq!(d.storms_detected_total(), 0);
    }

    #[test]
    fn record_occurrence_5_occurrences_in_30s_returns_suggested() {
        let d = fresh_detector();
        let mut last_cue = None;
        for i in 0..5 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            last_cue = record_occurrence(&d, FP_A, "svc", ts);
        }
        let cue = last_cue.expect("5th occurrence emits cue");
        assert_eq!(cue.kind, CueKind::RetryStorm);
        assert_eq!(cue.priority_tier, PriorityTier::Suggested);
        assert_eq!(cue.scope_id.as_deref(), Some("svc"));
        assert_eq!(cue.absolute_value as u64, 5);
        assert_eq!(d.storms_detected_total(), 1);
    }

    #[test]
    fn record_occurrence_10_occurrences_emits_autonomous_at_10th_after_suggested_at_5th() {
        let d = fresh_detector();
        let mut emitted: Vec<PriorityTier> = Vec::new();
        for i in 0..10 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            if let Some(cue) = record_occurrence(&d, FP_A, "svc", ts) {
                emitted.push(cue.priority_tier);
            }
        }
        assert_eq!(
            emitted,
            vec![PriorityTier::Suggested, PriorityTier::Autonomous]
        );
        assert_eq!(d.storms_detected_total(), 2);
    }

    #[test]
    fn record_occurrence_11th_in_30s_does_not_re_emit_post_autonomous() {
        let d = fresh_detector();
        let mut emitted = 0;
        for i in 0..11 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            if record_occurrence(&d, FP_A, "svc", ts).is_some() {
                emitted += 1;
            }
        }
        assert_eq!(
            emitted, 2,
            "exactly 2 emissions: Suggested at 5th, Autonomous at 10th"
        );
        assert_eq!(d.storms_detected_total(), 2);
    }

    #[test]
    fn record_occurrence_60_spreads_with_max_4_in_any_30s_returns_none() {
        let d = fresh_detector();
        let mut emitted = 0;
        for i in 0..60 {
            let ts = (1_000 + i * 10) * NANOS_PER_SEC;
            if record_occurrence(&d, FP_A, "svc", ts).is_some() {
                emitted += 1;
            }
        }
        assert_eq!(
            emitted, 0,
            "occurrences spread 10s apart never reach 5-in-30s"
        );
    }

    #[test]
    fn record_occurrence_per_fingerprint_isolation() {
        let d = fresh_detector();
        for i in 0..5 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            record_occurrence(&d, FP_A, "svc-a", ts);
        }
        for i in 0..3 {
            let ts = (1_010 + i) * NANOS_PER_SEC;
            record_occurrence(&d, FP_B, "svc-b", ts);
        }
        assert_eq!(d.fingerprints_tracked(), 2);
        assert_eq!(d.storms_detected_total(), 1);
    }

    #[test]
    fn record_occurrence_window_reset_after_full_expiration_re_emits_suggested() {
        let d = fresh_detector();
        for i in 0..5 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            record_occurrence(&d, FP_A, "svc", ts);
        }
        assert_eq!(d.storms_detected_total(), 1);

        let later_base = (1_000 + DEFAULT_STORM_WINDOW_SECONDS as i64 + 10) * NANOS_PER_SEC;
        let mut next_emitted: Option<PriorityTier> = None;
        for i in 0..5 {
            let ts = later_base + i * NANOS_PER_SEC;
            if let Some(cue) = record_occurrence(&d, FP_A, "svc", ts) {
                next_emitted = Some(cue.priority_tier);
            }
        }
        assert_eq!(
            next_emitted,
            Some(PriorityTier::Suggested),
            "after full window expiration, fresh 5 occurrences re-emit Suggested"
        );
        assert_eq!(d.storms_detected_total(), 2);
    }

    #[test]
    fn run_one_storm_cycle_prunes_expired_fingerprints() {
        let d = fresh_detector();
        record_occurrence(&d, FP_A, "svc-a", 1_000 * NANOS_PER_SEC);
        record_occurrence(&d, FP_B, "svc-b", 1_000 * NANOS_PER_SEC);
        assert_eq!(d.fingerprints_tracked(), 2);

        let past_window = (1_000 + DEFAULT_STORM_WINDOW_SECONDS as i64 + 10) * NANOS_PER_SEC;
        let stats = run_one_storm_cycle(&d, past_window);

        assert_eq!(stats.fingerprints_tracked, 0);
        assert_eq!(stats.fingerprints_evicted_total, 2);
    }

    #[test]
    fn run_one_storm_cycle_keeps_recent_fingerprints() {
        let d = fresh_detector();
        record_occurrence(&d, FP_A, "svc-a", 1_000 * NANOS_PER_SEC);

        let within_window = 1_020 * NANOS_PER_SEC;
        let stats = run_one_storm_cycle(&d, within_window);

        assert_eq!(stats.fingerprints_tracked, 1);
        assert_eq!(stats.fingerprints_evicted_total, 0);
    }

    #[test]
    fn run_one_storm_cycle_reports_zero_on_empty_detector() {
        let d = fresh_detector();
        let stats = run_one_storm_cycle(&d, 1_000 * NANOS_PER_SEC);
        assert_eq!(stats.fingerprints_tracked, 0);
        assert_eq!(stats.storms_detected_total, 0);
        assert_eq!(stats.fingerprints_evicted_total, 0);
        assert_eq!(stats.window_seconds, DEFAULT_STORM_WINDOW_SECONDS);
    }

    #[test]
    fn observe_and_dispatch_storm_broadcasts_cue_on_threshold_crossing() {
        let d = fresh_detector();
        let b = AttentionCueBroadcast::new();
        let mut rx = b.subscribe();

        for i in 0..5 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            observe_and_dispatch_storm(&d, &b, FP_A, "svc", ts);
        }

        let cue = rx.try_recv().expect("Suggested cue broadcast");
        assert_eq!(cue.kind, CueKind::RetryStorm);
        assert_eq!(cue.priority_tier, PriorityTier::Suggested);
        assert_eq!(cue.scope_id.as_deref(), Some("svc"));
    }

    #[test]
    fn observe_and_dispatch_storm_benign_without_subscribers() {
        let d = fresh_detector();
        let b = AttentionCueBroadcast::new();
        for i in 0..5 {
            let ts = (1_000 + i) * NANOS_PER_SEC;
            observe_and_dispatch_storm(&d, &b, FP_A, "svc", ts);
        }
        assert_eq!(d.storms_detected_total(), 1);
    }

    #[test]
    fn fingerprint_to_hex_prefix_yields_8_lowercase_hex_chars() {
        let fp = [
            0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE, 0xFA, 0xCE, 0xFE, 0xED, 0xC0, 0xDE,
            0x00, 0xFF,
        ];
        assert_eq!(fingerprint_to_hex_prefix(&fp), "deadbeef");
    }

    // PII negative-canary + identifier-class field discipline tests. Mirror
    // chunk #63 `pattern::detector::tests::{tracing_emission_uses_identifier_class_fields_only,
    // pii_negative_canary_in_restart_emit}` shape.

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
        fn record_f64(&mut self, field: &Field, value: f64) {
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
            let mut fields = FieldCollector(Vec::new());
            event.record(&mut fields);
            let metadata = event.metadata();
            self.events.lock().expect("lock").push((
                metadata.target().to_string(),
                *metadata.level(),
                fields.0,
            ));
        }
        fn enter(&self, _: &tracing::Id) {}
        fn exit(&self, _: &tracing::Id) {}
    }

    #[test]
    fn tracing_emission_uses_identifier_class_fields_only() {
        let (subscriber, events) = CapturingSubscriber::new();
        tracing::subscriber::with_default(subscriber, || {
            let d = fresh_detector();
            let b = AttentionCueBroadcast::new();
            for i in 0..5 {
                let ts = (1_000 + i) * NANOS_PER_SEC;
                observe_and_dispatch_storm(&d, &b, FP_A, "svc-a", ts);
            }
            let _ = run_one_storm_cycle(&d, 1_010 * NANOS_PER_SEC);
        });
        let captured = events.lock().expect("lock");
        let targets: Vec<String> = captured.iter().map(|(t, _, _)| t.clone()).collect();
        assert!(targets.contains(&TARGET_PATTERN_STORM_DETECTED.to_string()));
        assert!(targets.contains(&TARGET_PATTERN_STORM_EMIT.to_string()));
        assert!(targets.contains(&TARGET_PATTERN_STORM_TICK.to_string()));

        // Banned PII / per-service-cardinality field substrings — none should
        // appear in field NAMES or VALUES across the captured emissions.
        for banned in [
            "service_name",
            "scope_id",
            "trace_id",
            "span_id",
            "attribute",
            "instrumentation_scope",
            "body",
            "exception_message",
            "exception_stacktrace",
        ] {
            for (target, _level, fields) in captured.iter() {
                for (name, value) in fields {
                    assert!(
                        !name.contains(banned),
                        "banned substring `{banned}` in field NAME ({target}.{name})"
                    );
                    assert!(
                        !value.contains(banned),
                        "banned substring `{banned}` in field VALUE ({target}.{name}={value})"
                    );
                }
            }
        }
    }

    #[test]
    fn pii_negative_canary_storm_emit_does_not_leak_service_canary() {
        let (subscriber, events) = CapturingSubscriber::new();
        const CANARY: &str = "service-with-secret-canary-token-99887";
        tracing::subscriber::with_default(subscriber, || {
            let d = fresh_detector();
            let b = AttentionCueBroadcast::new();
            for i in 0..5 {
                let ts = (1_000 + i) * NANOS_PER_SEC;
                observe_and_dispatch_storm(&d, &b, FP_A, CANARY, ts);
            }
        });
        let captured = events.lock().expect("lock");
        for (target, _level, fields) in captured.iter() {
            for (name, value) in fields {
                assert!(
                    !name.contains("secret-canary-token-99887"),
                    "CANARY in field NAME ({target}.{name})"
                );
                assert!(
                    !value.contains("secret-canary-token-99887"),
                    "CANARY substring leaked into field VALUE ({target}.{name}={value})"
                );
            }
        }
    }
}
