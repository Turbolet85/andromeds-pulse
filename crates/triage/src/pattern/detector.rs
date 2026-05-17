use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use dashmap::DashMap;
use tracing::info;

use super::broadcast::{RestartEvent, RestartEventBroadcast};
use super::{TARGET_PATTERN_RESTART_DETECT, TARGET_PATTERN_RESTART_EMIT, TARGET_PATTERN_TICK};

/// Default heartbeat interval for `triage.pattern.tick` events — matches
/// the `.claude/rules/observability.md` "Heartbeat ticks every 15s" rule;
/// distinct from the cue emitter's evaluation `tick_interval` (1s) since
/// the detector does its detection work inline at `observe_span` time
/// (hot path) rather than on a fixed cadence.
pub const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

/// Per-service last-seen OTLP span timestamp tracker. Detection happens
/// inline at `observe_span` time — when the gap from the previous
/// observation to the new one exceeds the configurable threshold (default
/// 20s per chunk spec), `observe_span` returns `Some(RestartEvent)` and
/// the caller dispatches via `observe_and_dispatch`. The struct holds the
/// gap threshold + an `AtomicU64` emit counter for heartbeat stats.
///
/// Concurrency: `DashMap` allows lock-free per-key updates from multiple
/// ingest tasks; counter uses `Relaxed` ordering since emit-count is a
/// monotonic observability counter, not a synchronization primitive.
#[derive(Debug)]
pub struct RestartDetector {
    last_seen: DashMap<String, i64>,
    gap_threshold_nanos: i64,
    restart_events_emitted_total: AtomicU64,
}

impl RestartDetector {
    pub fn new(gap_threshold_secs: u64) -> Self {
        Self {
            last_seen: DashMap::new(),
            gap_threshold_nanos: (gap_threshold_secs as i64).saturating_mul(1_000_000_000),
            restart_events_emitted_total: AtomicU64::new(0),
        }
    }

    /// Observe an OTLP span's `(service.name, ts_unix_nano)`. Returns
    /// `Some(RestartEvent)` if this observation completes a gap-then-resume
    /// pattern — i.e., the gap from the previous observation exceeded the
    /// detector's gap threshold. Returns `None` for first-observation,
    /// sub-threshold gaps, or empty `service.name` (dropped per chunk #61
    /// service identity discipline).
    ///
    /// Updates `last_seen[service]` to `ts_unix_nano` regardless of whether
    /// an event fires, so a sequence of sub-threshold observations keeps
    /// rolling forward.
    pub fn observe_span(&self, service: &str, ts_unix_nano: i64) -> Option<RestartEvent> {
        if service.is_empty() {
            return None;
        }
        let prior = self.last_seen.insert(service.to_string(), ts_unix_nano);
        match prior {
            Some(last_seen) if ts_unix_nano > last_seen => {
                let gap_nanos = ts_unix_nano - last_seen;
                if gap_nanos > self.gap_threshold_nanos {
                    Some(RestartEvent {
                        service: service.to_string(),
                        gap_seconds: (gap_nanos / 1_000_000_000) as u64,
                        last_seen_unix_nano: last_seen,
                        resume_unix_nano: ts_unix_nano,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn services_tracked(&self) -> usize {
        self.last_seen.len()
    }

    pub fn gap_threshold_seconds(&self) -> u64 {
        (self.gap_threshold_nanos / 1_000_000_000) as u64
    }

    pub fn restart_events_emitted_total(&self) -> u64 {
        self.restart_events_emitted_total.load(Ordering::Relaxed)
    }

    fn record_emit(&self) {
        self.restart_events_emitted_total
            .fetch_add(1, Ordering::Relaxed);
    }
}

/// Stats captured during one heartbeat tick. Public for test inspection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DetectCycleStats {
    pub services_tracked: usize,
    pub gap_threshold_seconds: u64,
    pub restart_events_emitted_total: u64,
}

/// Emit `triage.pattern.tick` heartbeat with current detector stats.
/// Returns stats for test inspection. Pure synchronous; safe to call
/// from `tokio::time::interval` tick loop or directly from tests.
pub fn run_one_detect_cycle(detector: &RestartDetector) -> DetectCycleStats {
    let stats = DetectCycleStats {
        services_tracked: detector.services_tracked(),
        gap_threshold_seconds: detector.gap_threshold_seconds(),
        restart_events_emitted_total: detector.restart_events_emitted_total(),
    };
    info!(
        target: TARGET_PATTERN_TICK,
        value = 1u64,
        gap_threshold_seconds = stats.gap_threshold_seconds,
        services_tracked = stats.services_tracked as u64,
        restart_events_emitted = stats.restart_events_emitted_total,
    );
    stats
}

/// Inline-detection helper: invoke `detector.observe_span(...)`; if a
/// `RestartEvent` returns, dispatch via the broadcast handle + emit the
/// `triage.pattern.restart_detect` + `triage.pattern.restart_emit`
/// tracing events + bump the detector's emit counter. Broadcast send is
/// non-fatal when no subscribers exist (benign per chunk #62 precedent
/// `attention_cue_broadcast_send_with_no_subscribers_is_benign`).
pub fn observe_and_dispatch(
    detector: &RestartDetector,
    broadcast: &RestartEventBroadcast,
    service: &str,
    ts_unix_nano: i64,
) {
    if let Some(event) = detector.observe_span(service, ts_unix_nano) {
        info!(
            target: TARGET_PATTERN_RESTART_DETECT,
            cue_kind = "restart_event",
            gap_seconds = event.gap_seconds,
            restart_window_active = true,
        );
        let _ = broadcast.sender().send(event.clone());
        detector.record_emit();
        info!(
            target: TARGET_PATTERN_RESTART_EMIT,
            cue_kind = "restart_event",
            gap_seconds = event.gap_seconds,
        );
    }
}

/// Long-running heartbeat task spawned at boot. Fires `run_one_detect_cycle`
/// on the supplied interval (default 15s per `DEFAULT_HEARTBEAT_INTERVAL`).
/// Skips the immediate first tick to mirror the chunk #62 emitter + chunk
/// #61 persist loop convention.
pub async fn start_restart_detector(detector: Arc<RestartDetector>, heartbeat_interval: Duration) {
    let mut interval = tokio::time::interval(heartbeat_interval);
    interval.tick().await;
    loop {
        interval.tick().await;
        let _ = run_one_detect_cycle(&detector);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

    const TEST_GAP_THRESHOLD_SECS: u64 = 20;
    const NANOS_PER_SEC: i64 = 1_000_000_000;

    #[test]
    fn observe_span_with_empty_service_dropped() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        assert!(d.observe_span("", 1_000_000_000).is_none());
        assert!(d.observe_span("", 50 * NANOS_PER_SEC).is_none());
        assert_eq!(d.services_tracked(), 0);
    }

    #[test]
    fn first_observation_never_emits() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let r = d.observe_span("svc-a", 1_000 * NANOS_PER_SEC);
        assert!(r.is_none());
        assert_eq!(d.services_tracked(), 1);
    }

    #[test]
    fn sub_threshold_gap_does_not_emit() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let _ = d.observe_span("svc-a", 1_000 * NANOS_PER_SEC);
        // 5s later — well below 20s threshold.
        let r = d.observe_span("svc-a", 1_005 * NANOS_PER_SEC);
        assert!(r.is_none());
    }

    #[test]
    fn above_threshold_gap_then_resume_emits_event() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let _ = d.observe_span("svc-a", 1_000 * NANOS_PER_SEC);
        // 25s later — above 20s threshold.
        let r = d.observe_span("svc-a", 1_025 * NANOS_PER_SEC);
        let event = r.expect("restart event expected");
        assert_eq!(event.service, "svc-a");
        assert_eq!(event.gap_seconds, 25);
        assert_eq!(event.last_seen_unix_nano, 1_000 * NANOS_PER_SEC);
        assert_eq!(event.resume_unix_nano, 1_025 * NANOS_PER_SEC);
    }

    #[test]
    fn multiple_services_tracked_independently() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let _ = d.observe_span("svc-a", 1_000 * NANOS_PER_SEC);
        let _ = d.observe_span("svc-b", 1_000 * NANOS_PER_SEC);
        // svc-a gaps; svc-b stays continuous.
        let r_a = d.observe_span("svc-a", 1_030 * NANOS_PER_SEC);
        let r_b = d.observe_span("svc-b", 1_010 * NANOS_PER_SEC);
        assert!(r_a.is_some(), "svc-a should emit");
        assert!(r_b.is_none(), "svc-b should not emit");
        assert_eq!(d.services_tracked(), 2);
    }

    #[test]
    fn observation_with_decreasing_timestamp_no_emit() {
        // Out-of-order arrival: should not be treated as gap.
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let _ = d.observe_span("svc-a", 1_100 * NANOS_PER_SEC);
        let r = d.observe_span("svc-a", 1_000 * NANOS_PER_SEC);
        assert!(r.is_none());
    }

    #[test]
    fn run_one_detect_cycle_reports_zero_when_empty() {
        let d = RestartDetector::new(TEST_GAP_THRESHOLD_SECS);
        let stats = run_one_detect_cycle(&d);
        assert_eq!(stats.services_tracked, 0);
        assert_eq!(stats.gap_threshold_seconds, TEST_GAP_THRESHOLD_SECS);
        assert_eq!(stats.restart_events_emitted_total, 0);
    }

    #[test]
    fn observe_and_dispatch_increments_emit_count_on_gap() {
        let d = Arc::new(RestartDetector::new(TEST_GAP_THRESHOLD_SECS));
        let b = RestartEventBroadcast::new();
        let mut rx = b.subscribe();
        observe_and_dispatch(&d, &b, "svc-a", 1_000 * NANOS_PER_SEC);
        observe_and_dispatch(&d, &b, "svc-a", 1_030 * NANOS_PER_SEC);
        assert_eq!(d.restart_events_emitted_total(), 1);
        let event = rx.try_recv().expect("event broadcast");
        assert_eq!(event.service, "svc-a");
        assert_eq!(event.gap_seconds, 30);
    }

    #[test]
    fn observe_and_dispatch_benign_without_subscribers() {
        let d = Arc::new(RestartDetector::new(TEST_GAP_THRESHOLD_SECS));
        let b = RestartEventBroadcast::new();
        observe_and_dispatch(&d, &b, "svc-a", 1_000 * NANOS_PER_SEC);
        observe_and_dispatch(&d, &b, "svc-a", 1_030 * NANOS_PER_SEC);
        // No subscribers; broadcast send returns Err but we still bump
        // the emit counter (the detector noticed the restart).
        assert_eq!(d.restart_events_emitted_total(), 1);
    }

    // CapturingSubscriber + FieldCollector pattern per testing.md §Session
    // Additions 2026-05-11 + chunk #62 `crates/triage/src/cue/emitter.rs::tests`
    // precedent. Captures tracing event target / level / field values for
    // negative-canary PII assertions.
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
            let d = Arc::new(RestartDetector::new(TEST_GAP_THRESHOLD_SECS));
            let b = RestartEventBroadcast::new();
            observe_and_dispatch(&d, &b, "svc-a", 1_000 * NANOS_PER_SEC);
            observe_and_dispatch(&d, &b, "svc-a", 1_030 * NANOS_PER_SEC);
            let _ = run_one_detect_cycle(&d);
        });
        let captured = events.lock().expect("lock");
        let targets: Vec<String> = captured.iter().map(|(t, _, _)| t.clone()).collect();
        assert!(targets.contains(&TARGET_PATTERN_RESTART_DETECT.to_string()));
        assert!(targets.contains(&TARGET_PATTERN_RESTART_EMIT.to_string()));
        assert!(targets.contains(&TARGET_PATTERN_TICK.to_string()));
        // Negative-canary: no captured field value contains substrings that
        // would betray OTLP attribute leakage. The service name "svc-a" is
        // identifier-class + allowlist-admitted (chunk #61 + #62 precedent).
        for banned in [
            "attribute",
            "instrumentation_scope",
            "trace_id",
            "span_id",
            "body",
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
    fn pii_negative_canary_in_restart_emit() {
        // Inject a span with a "PII canary" substring in service.name.
        // service.name IS the allowlist-admitted identifier-class field;
        // assertion: the canary appears ONLY in `service` field, never
        // leaked into any other captured field's name or value.
        let (subscriber, events) = CapturingSubscriber::new();
        const CANARY: &str = "service-with-secret-canary-token-12345";
        tracing::subscriber::with_default(subscriber, || {
            let d = Arc::new(RestartDetector::new(TEST_GAP_THRESHOLD_SECS));
            let b = RestartEventBroadcast::new();
            observe_and_dispatch(&d, &b, CANARY, 1_000 * NANOS_PER_SEC);
            observe_and_dispatch(&d, &b, CANARY, 1_030 * NANOS_PER_SEC);
        });
        let captured = events.lock().expect("lock");
        // No restart-detect / restart-emit tracing event field carries the
        // canary substring in its NAME or VALUE — the canary is only
        // surfaced via `RestartEvent.service` payload (which is the
        // identifier-class field). Verify by scanning all captured field
        // (name, value) tuples for the canary substring.
        for (target, _level, fields) in captured.iter() {
            for (name, value) in fields {
                assert!(
                    !name.contains(CANARY),
                    "CANARY in field NAME ({target}.{name})"
                );
                assert!(
                    !value.contains("secret-canary-token-12345"),
                    "CANARY substring leaked into field VALUE ({target}.{name}={value})"
                );
            }
        }
    }

    #[test]
    fn detect_cycle_stats_match_emitted_count() {
        let d = Arc::new(RestartDetector::new(TEST_GAP_THRESHOLD_SECS));
        let b = RestartEventBroadcast::new();
        observe_and_dispatch(&d, &b, "svc-a", 1_000 * NANOS_PER_SEC);
        observe_and_dispatch(&d, &b, "svc-a", 1_030 * NANOS_PER_SEC);
        observe_and_dispatch(&d, &b, "svc-b", 1_000 * NANOS_PER_SEC);
        observe_and_dispatch(&d, &b, "svc-b", 1_040 * NANOS_PER_SEC);
        let stats = run_one_detect_cycle(&d);
        assert_eq!(stats.services_tracked, 2);
        assert_eq!(stats.restart_events_emitted_total, 2);
    }

    #[test]
    fn default_heartbeat_interval_matches_obs_rule() {
        // Per .claude/rules/observability.md "Heartbeat ticks every 15s".
        assert_eq!(DEFAULT_HEARTBEAT_INTERVAL, Duration::from_secs(15));
    }
}
