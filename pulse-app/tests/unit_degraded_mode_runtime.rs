//! Integration tests для `pulse-app/src/degraded_mode_runtime.rs`
//! `LocalDegradedModeStatus` concrete impl (chunk #86).
//!
//! pulse-app's `[lib] test = false` setting disables source-level
//! `mod tests` blocks on Windows due к WebView2 DLL load (per CLAUDE.md
//! testing.md 2026-05-20 session 107 entry); all unit tests for pulse-app
//! crate code live в `pulse-app/tests/unit_*.rs` integration test files.

use std::sync::{Arc, Mutex};

use interpretation::contract::InferenceError;
use interpretation::degraded_mode::{
    BACKOFF_CAP_SECS, BACKOFF_PROGRESSION_SECS, DegradedModeState, DegradedModeStatus,
    FAILURE_WINDOW_SECS,
};
use pulse_app::degraded_mode_runtime::{
    LocalDegradedModeStatus, interpretation_retry_error_to_app_error,
};
use tracing::Level;
use ui_bridge::contract::AppError;

const NANOS_PER_SEC: i64 = 1_000_000_000;

fn fresh() -> LocalDegradedModeStatus {
    LocalDegradedModeStatus::new()
}

#[test]
fn initial_state_is_active_with_zero_counter() {
    let dm = fresh();
    let snap = dm.current_snapshot(0);
    assert_eq!(snap.state, DegradedModeState::Active);
    assert_eq!(snap.consecutive_failures, 0);
    assert_eq!(snap.next_retry_at_unix_nano, None);
    assert_eq!(snap.backoff_seconds_remaining, 0);
}

#[test]
fn record_failure_below_threshold_keeps_active() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    let s1 = dm.record_failure(t0);
    assert_eq!(s1.state, DegradedModeState::Active);
    assert_eq!(s1.consecutive_failures, 1);
    let s2 = dm.record_failure(t0 + NANOS_PER_SEC);
    assert_eq!(s2.state, DegradedModeState::Active);
    assert_eq!(s2.consecutive_failures, 2);
}

#[test]
fn record_failure_at_threshold_transitions_to_degraded() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0 + NANOS_PER_SEC);
    let s3 = dm.record_failure(t0 + 2 * NANOS_PER_SEC);
    assert_eq!(s3.state, DegradedModeState::Degraded);
    assert_eq!(s3.consecutive_failures, 3);
    assert_eq!(
        s3.next_retry_at_unix_nano,
        Some(t0 + 2 * NANOS_PER_SEC + (BACKOFF_PROGRESSION_SECS[0] as i64) * NANOS_PER_SEC)
    );
    assert_eq!(s3.backoff_seconds_remaining, BACKOFF_PROGRESSION_SECS[0]);
}

#[test]
fn record_failure_advances_backoff_progression_2_5_10_min() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // enters degraded at progression[0] = 120s (2min)
    let s4 = dm.record_failure(t0);
    assert_eq!(s4.consecutive_failures, 4);
    assert_eq!(s4.backoff_seconds_remaining, BACKOFF_PROGRESSION_SECS[1]); // 300s (5min)
    let s5 = dm.record_failure(t0);
    assert_eq!(s5.consecutive_failures, 5);
    assert_eq!(s5.backoff_seconds_remaining, BACKOFF_PROGRESSION_SECS[2]); // 600s (10min)
    let s6 = dm.record_failure(t0);
    assert_eq!(s6.consecutive_failures, 6);
    assert_eq!(s6.backoff_seconds_remaining, BACKOFF_CAP_SECS); // capped at 600s
    let s7 = dm.record_failure(t0);
    assert_eq!(s7.consecutive_failures, 7);
    assert_eq!(s7.backoff_seconds_remaining, BACKOFF_CAP_SECS); // still capped
}

#[test]
fn record_success_after_degraded_resets_state() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // now Degraded
    let snap = dm.record_success(t0 + 60 * NANOS_PER_SEC);
    assert_eq!(snap.state, DegradedModeState::Active);
    assert_eq!(snap.consecutive_failures, 0);
    assert_eq!(snap.next_retry_at_unix_nano, None);
    assert_eq!(snap.backoff_seconds_remaining, 0);
}

#[test]
fn record_failure_outside_window_resets_counter() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0 + NANOS_PER_SEC);
    // Gap > FAILURE_WINDOW_SECS from the LAST failure (not t0) — counter
    // restarts from 1. Last failure was at t0+1s; outside must be
    // > t0+1s + 300s = t0 + 301s. Use +302s к stay strictly outside.
    let outside = t0 + NANOS_PER_SEC + (FAILURE_WINDOW_SECS as i64 + 1) * NANOS_PER_SEC;
    let snap = dm.record_failure(outside);
    assert_eq!(snap.consecutive_failures, 1);
    assert_eq!(snap.state, DegradedModeState::Active);
}

#[test]
fn trigger_manual_retry_resets_backoff_immediately() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded; backoff = 120s
    // Manual retry mid-backoff window — should override.
    let snap = dm.trigger_manual_retry(t0 + 60 * NANOS_PER_SEC);
    assert_eq!(snap.state, DegradedModeState::Active);
    assert_eq!(snap.consecutive_failures, 0);
    assert_eq!(snap.next_retry_at_unix_nano, None);
    assert_eq!(snap.backoff_seconds_remaining, 0);
}

#[test]
fn is_in_backoff_returns_false_when_active() {
    let dm = fresh();
    assert!(!dm.is_in_backoff(0));
    dm.record_failure(0);
    assert!(!dm.is_in_backoff(0));
}

#[test]
fn is_in_backoff_returns_true_when_degraded_and_before_next_retry() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded; next retry @ t0 + 120s
    assert!(dm.is_in_backoff(t0 + 30 * NANOS_PER_SEC));
    assert!(dm.is_in_backoff(t0 + 119 * NANOS_PER_SEC));
    assert!(!dm.is_in_backoff(t0 + 121 * NANOS_PER_SEC));
}

#[test]
fn current_snapshot_recomputes_backoff_seconds_remaining_against_clock() {
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded; next retry @ t0 + 120s
    let snap_mid = dm.current_snapshot(t0 + 30 * NANOS_PER_SEC);
    assert_eq!(snap_mid.backoff_seconds_remaining, 90);
    let snap_after = dm.current_snapshot(t0 + 130 * NANOS_PER_SEC);
    assert_eq!(snap_after.backoff_seconds_remaining, 0);
}

// =============================================================================
// AppError sanitization tests — extends chunk #77 LANDED security-vector-coverage
// =============================================================================

#[test]
fn interpretation_retry_error_to_app_error_sanitizes_inner_reason_strings() {
    // Variant with inner `reason` carrying canary content that MUST NOT escape.
    let err = InferenceError::JsonParseFailed {
        reason:
            "secret-canary-API-key-12345-svc at /home/secret/path.rs:42 (mistralrs v0.8.0 SecretType)"
                .to_string(),
    };
    let app_err = interpretation_retry_error_to_app_error(err);
    let AppError::Internal { message } = app_err else {
        panic!("expected AppError::Internal");
    };
    assert_eq!(message, "interpretation retry rejected: parse failed");
    assert!(!message.contains("secret-canary"));
    assert!(!message.contains("/home/secret"));
    assert!(!message.contains("path.rs"));
    assert!(!message.contains("mistralrs"));
    assert!(!message.contains("v0.8.0"));
    assert!(!message.contains("SecretType"));
}

#[test]
fn interpretation_retry_error_to_app_error_covers_all_variants_with_sanitization() {
    let variants = vec![
        InferenceError::ModelNotConfigured,
        InferenceError::InvalidModelPath,
        InferenceError::ModelLoadFailed {
            reason: "/secret/path.rs".to_string(),
        },
        InferenceError::TokenizerInitFailed {
            reason: "/secret/path.rs".to_string(),
        },
        InferenceError::InferenceFailed {
            reason: "/secret/path.rs".to_string(),
        },
        InferenceError::OutputTooLarge {
            actual_bytes: 999,
            max_bytes: 100,
        },
        InferenceError::JsonParseFailed {
            reason: "/secret/path.rs".to_string(),
        },
        InferenceError::SchemaViolation {
            reason: "/secret/path.rs".to_string(),
        },
    ];
    for variant in variants {
        let app_err = interpretation_retry_error_to_app_error(variant);
        let AppError::Internal { message } = app_err else {
            panic!("expected AppError::Internal");
        };
        assert!(message.starts_with("interpretation retry rejected:"));
        assert!(!message.contains("/secret/path.rs"));
        // OutputTooLarge variant's numeric fields MUST not leak either.
        assert!(!message.contains("999"));
    }
}

// =============================================================================
// Tracing event emission tests — CapturingSubscriber pattern from chunk #44
// (CLAUDE.md testing.md 2026-05-11 entry — extends к field-VALUE assertions
// для chunk #86 degraded.enter / degraded.exit events).
// =============================================================================

struct CapturedEvent {
    target: String,
    level: Level,
    fields: String,
}

#[derive(Default)]
struct CapturingSubscriber {
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl CapturingSubscriber {
    fn new() -> (Self, Arc<Mutex<Vec<CapturedEvent>>>) {
        let events: Arc<Mutex<Vec<CapturedEvent>>> = Arc::new(Mutex::new(Vec::new()));
        (
            CapturingSubscriber {
                events: Arc::clone(&events),
            },
            events,
        )
    }
}

struct FieldCollector<'a> {
    sink: &'a mut String,
}

impl<'a> tracing::field::Visit for FieldCollector<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value:?};", field.name());
    }
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        use std::fmt::Write;
        let _ = write!(self.sink, "{}={value};", field.name());
    }
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _meta: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _id: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _id: &tracing::span::Id, _follows: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        let metadata = event.metadata();
        let mut sink = String::new();
        let mut collector = FieldCollector { sink: &mut sink };
        event.record(&mut collector);
        self.events.lock().expect("lock").push(CapturedEvent {
            target: metadata.target().to_string(),
            level: *metadata.level(),
            fields: sink,
        });
    }
    fn enter(&self, _span: &tracing::span::Id) {}
    fn exit(&self, _span: &tracing::span::Id) {}
}

#[test]
fn local_degraded_mode_emits_degraded_enter_warning_on_threshold() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // entry transition fires
    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();
    let enter_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|e| e.target == "interpretation.degraded.enter")
        .collect();
    assert_eq!(enter_events.len(), 1, "exactly one entry transition");
    assert_eq!(enter_events[0].level, Level::WARN);
    assert!(enter_events[0].fields.contains("consecutive_failures=3"));
    assert!(enter_events[0].fields.contains("window_seconds=300"));
    assert!(enter_events[0].fields.contains("backoff_seconds=120"));
}

#[test]
fn local_degraded_mode_emits_degraded_exit_info_on_recovery() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded
    dm.record_success(t0 + 60 * NANOS_PER_SEC); // exit transition
    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();
    let exit_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|e| e.target == "interpretation.degraded.exit")
        .collect();
    assert_eq!(exit_events.len(), 1, "exactly one exit transition");
    assert_eq!(exit_events[0].level, Level::INFO);
    assert!(exit_events[0].fields.contains("consecutive_successes=1"));
    assert!(exit_events[0].fields.contains("duration_seconds=60"));
}

#[test]
fn local_degraded_mode_emits_degraded_exit_on_manual_retry_when_degraded() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded
    dm.trigger_manual_retry(t0 + 90 * NANOS_PER_SEC); // exit (manual)
    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();
    let exit_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|e| e.target == "interpretation.degraded.exit")
        .collect();
    assert_eq!(exit_events.len(), 1);
    assert!(exit_events[0].fields.contains("consecutive_successes=0"));
}

#[test]
fn local_degraded_mode_emits_no_events_when_active_throughout() {
    let (subscriber, events) = CapturingSubscriber::new();
    let _guard = tracing::subscriber::set_default(subscriber);
    let dm = fresh();
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_success(t0 + NANOS_PER_SEC); // never reached Degraded
    drop(_guard);
    let captured = events.lock().expect("lock").drain(..).collect::<Vec<_>>();
    let degraded_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|e| e.target.starts_with("interpretation.degraded"))
        .collect();
    assert!(
        degraded_events.is_empty(),
        "no degraded entry/exit events when FSM never transitions out of Active"
    );
}
