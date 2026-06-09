use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use crate::contract::{AttentionCue, CueKind};

use super::broadcast::RestartEvent;

/// Reason for a dual-condition magnitude bypass — surfaces in the
/// `metric.pipeline.l2.magnitude_bypass_triggered_total{reason}` metric
/// emission and the `triage.cue.suppression_bypass` tracing event.
/// Bounded enumeration (exactly 2 values per chunk spec); short-circuit
/// priority is `Relative` then `Absolute`, mirroring
/// `cue::classify::dual_condition_bypass`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BypassReason {
    Relative,
    Absolute,
}

impl BypassReason {
    pub fn label(self) -> &'static str {
        match self {
            BypassReason::Relative => "relative",
            BypassReason::Absolute => "absolute",
        }
    }
}

/// One bypass trigger captured during a suppression cycle. Emitted to the
/// `metric.pipeline.l2.magnitude_bypass_triggered_total{reason, cue_kind}`
/// stream by the cue emitter integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BypassTrigger {
    pub cue_kind: CueKind,
    pub reason: BypassReason,
}

/// Outcome of one suppression evaluation cycle. `cues_kept` are the cues
/// that survive surgical suppression (including bypass-triggered survivors);
/// `cues_suppressed` counts dropped cues; `bypass_triggers` records the
/// per-cue bypass-reason for downstream metric emission.
///
/// `PartialEq` only (not `Eq`) — `AttentionCue` carries `f64` fields
/// (`magnitude`, `absolute_value`, `confidence`) so `Eq` cannot be derived
/// on a struct containing `Vec<AttentionCue>` per IEEE 754 semantics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SuppressionOutcome {
    pub cues_kept: Vec<AttentionCue>,
    pub cues_suppressed: usize,
    pub bypass_triggers: Vec<BypassTrigger>,
}

/// Suppression-filter thresholds + bypass thresholds. Built by the cue
/// emitter at tick time from `cue::Thresholds`; passed by reference to
/// `evaluate_with_suppression`. Pattern-side type (rather than directly
/// using `cue::Thresholds`) preserves the documented module dependency
/// direction (cue → pattern per arch §Cross-cutting Patterns).
///
/// `suppression_window_seconds` is NOT part of this struct — that value
/// is consumed at `SuppressionState::record_restart` time (called from
/// the cue emitter's `drain_restart_events` helper before evaluation
/// begins), not during the per-cue filter pass.
#[derive(Debug, Clone, Copy)]
pub struct SuppressionParams {
    pub persistence_cutoff_seconds: u64,
    pub magnitude_bypass_multiplier: f64,
    pub absolute_bypass_error_rate: f64,
    pub absolute_bypass_latency_ms: f64,
}

/// Per-service restart-suppression window tracker. After `record_restart`
/// for a service, `is_active` returns true until the window-end timestamp
/// (resume + suppression_window_seconds) elapses.
#[derive(Debug, Default)]
pub struct SuppressionState {
    /// service.name -> window_end_unix_nano
    windows: DashMap<String, i64>,
}

impl SuppressionState {
    pub fn new() -> Self {
        Self {
            windows: DashMap::new(),
        }
    }

    /// Record a restart event — opens a suppression window for the event's
    /// service ending at `event.resume_unix_nano + window_seconds * 1e9`.
    /// Replaces any prior window for the service (most recent restart wins).
    pub fn record_restart(&self, event: &RestartEvent, window_seconds: u64) {
        let window_nanos = (window_seconds as i64).saturating_mul(1_000_000_000);
        let end = event.resume_unix_nano.saturating_add(window_nanos);
        self.windows.insert(event.service.clone(), end);
    }

    /// True iff `service` has an active suppression window at `now_nanos`.
    pub fn is_active(&self, service: &str, now_nanos: i64) -> bool {
        self.windows
            .get(service)
            .map(|end| now_nanos <= *end)
            .unwrap_or(false)
    }

    pub fn windows_tracked(&self) -> usize {
        self.windows.len()
    }
}

/// Surgical suppression filter — drops cues that match ALL of:
///   1. `cue.kind == CueKind::ErrorRateSpike` (only ErrorRateSpike is
///      suppression-eligible per chunk #63 spec; other kinds always
///      survive regardless of restart-window state)
///   2. `cue.persistence_seconds < params.persistence_cutoff_seconds`
///      (default 30s — short-persistence cues are the noisy ones during
///      restart windows; long-persistence cues are real signals)
///   3. `state.is_active(cue.scope_id, now_nanos)` (cue's service has an
///      active restart-suppression window)
///   4. `!cue.suppression_bypassed` (dual-condition magnitude bypass
///      overrides suppression — P-057 guarantees catastrophic regressions
///      remain visible during restart-induced noise)
///
/// For cues where `cue.suppression_bypassed == true` AND the cue would
/// otherwise have been suppressed (rules 1-3 match), records a
/// `BypassTrigger` in the outcome so the caller can emit the
/// `metric.pipeline.l2.magnitude_bypass_triggered_total{reason}` metric.
/// The reason is derived from the cue's magnitude vs absolute_value against
/// `params` (short-circuit priority: Relative > Absolute, mirroring
/// `cue::classify::dual_condition_bypass`).
pub fn evaluate_with_suppression(
    cues: Vec<AttentionCue>,
    state: &SuppressionState,
    params: &SuppressionParams,
    now_nanos: i64,
) -> SuppressionOutcome {
    let mut outcome = SuppressionOutcome::default();
    for cue in cues {
        let service = cue.scope_id.as_deref().unwrap_or("");
        let in_window = !service.is_empty() && state.is_active(service, now_nanos);
        let suppression_eligible = cue.kind == CueKind::ErrorRateSpike
            && cue.persistence_seconds < params.persistence_cutoff_seconds;

        if in_window && suppression_eligible {
            if cue.suppression_bypassed {
                let reason = derive_bypass_reason(&cue, params);
                outcome.bypass_triggers.push(BypassTrigger {
                    cue_kind: cue.kind,
                    reason,
                });
                outcome.cues_kept.push(cue);
            } else {
                outcome.cues_suppressed += 1;
            }
        } else {
            outcome.cues_kept.push(cue);
        }
    }
    outcome
}

fn derive_bypass_reason(cue: &AttentionCue, params: &SuppressionParams) -> BypassReason {
    if cue.magnitude.is_finite() && cue.magnitude > params.magnitude_bypass_multiplier {
        return BypassReason::Relative;
    }
    // Magnitude did not trip — by P-057 OR-semantics, the cue's
    // absolute_value must have crossed the kind-specific absolute threshold
    // (or the upstream `dual_condition_bypass` flag is inconsistent with
    // these params, e.g., config-mid-flight reload). Verify per-kind.
    let absolute_satisfied = cue.absolute_value.is_finite()
        && match cue.kind {
            CueKind::ErrorRateSpike => cue.absolute_value > params.absolute_bypass_error_rate,
            CueKind::LatencyRegression => cue.absolute_value > params.absolute_bypass_latency_ms,
            CueKind::RestartEvent
            | CueKind::ServiceWentSilent
            | CueKind::RetryStorm
            | CueKind::ReflectionTrend => false,
        };
    if absolute_satisfied {
        BypassReason::Absolute
    } else {
        // Fallback for inconsistent caller (flag set but neither condition
        // holds under these params). Surface as Relative — the more
        // operator-visible reason — rather than silently dropping the
        // bypass classification.
        BypassReason::Relative
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueScope, PriorityTier};
    use rstest::rstest;

    const NANOS_PER_SEC: i64 = 1_000_000_000;

    fn default_params() -> SuppressionParams {
        SuppressionParams {
            persistence_cutoff_seconds: 30,
            magnitude_bypass_multiplier: 10.0,
            absolute_bypass_error_rate: 0.05,
            absolute_bypass_latency_ms: 1000.0,
        }
    }

    fn restart_event(service: &str, resume_ts_seconds: i64) -> RestartEvent {
        RestartEvent {
            service: service.to_string(),
            gap_seconds: 25,
            last_seen_unix_nano: (resume_ts_seconds - 25) * NANOS_PER_SEC,
            resume_unix_nano: resume_ts_seconds * NANOS_PER_SEC,
        }
    }

    fn cue(
        kind: CueKind,
        service: &str,
        magnitude: f64,
        absolute_value: f64,
        persistence_seconds: u64,
        suppression_bypassed: bool,
    ) -> AttentionCue {
        AttentionCue {
            kind,
            scope: CueScope::Service,
            scope_id: Some(service.to_string()),
            magnitude,
            absolute_value,
            persistence_seconds,
            confidence: 0.9,
            priority_tier: PriorityTier::Suggested,
            suppression_bypassed,
        }
    }

    #[test]
    fn suppression_state_starts_empty() {
        let s = SuppressionState::new();
        assert_eq!(s.windows_tracked(), 0);
        assert!(!s.is_active("svc-a", 1_000 * NANOS_PER_SEC));
    }

    #[test]
    fn record_restart_opens_window() {
        let s = SuppressionState::new();
        let ev = restart_event("svc-a", 1_000);
        s.record_restart(&ev, 60);
        // At resume + 30s → still in window.
        assert!(s.is_active("svc-a", 1_030 * NANOS_PER_SEC));
        // At resume + 60s → still in window (inclusive).
        assert!(s.is_active("svc-a", 1_060 * NANOS_PER_SEC));
        // At resume + 61s → window closed.
        assert!(!s.is_active("svc-a", 1_061 * NANOS_PER_SEC));
    }

    #[test]
    fn window_per_service_tracked_independently() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        assert!(s.is_active("svc-a", 1_030 * NANOS_PER_SEC));
        assert!(!s.is_active("svc-b", 1_030 * NANOS_PER_SEC));
    }

    #[test]
    fn evaluate_with_no_active_windows_keeps_all_cues() {
        let s = SuppressionState::new();
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 3.0, 0.04, 10, false)];
        let outcome = evaluate_with_suppression(cues.clone(), &s, &p, 1_000 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1);
        assert_eq!(outcome.cues_suppressed, 0);
        assert!(outcome.bypass_triggers.is_empty());
    }

    #[test]
    fn evaluate_suppresses_short_persistence_error_rate_spike_in_active_window() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        // ErrorRateSpike, persistence=10s (<30s), no bypass, active window.
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 3.0, 0.04, 10, false)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert!(outcome.cues_kept.is_empty(), "cue should be suppressed");
        assert_eq!(outcome.cues_suppressed, 1);
        assert!(outcome.bypass_triggers.is_empty());
    }

    #[test]
    fn evaluate_keeps_long_persistence_error_rate_spike_in_active_window() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        // persistence=45s (>=30s cutoff) — survives even without bypass.
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 3.0, 0.04, 45, false)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1);
        assert_eq!(outcome.cues_suppressed, 0);
        assert!(outcome.bypass_triggers.is_empty());
    }

    #[test]
    fn evaluate_keeps_non_error_rate_spike_kinds_in_active_window() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![
            cue(CueKind::LatencyRegression, "svc-a", 3.0, 200.0, 10, false),
            cue(CueKind::ServiceWentSilent, "svc-a", 0.0, 0.0, 10, false),
            cue(CueKind::RetryStorm, "svc-a", 3.0, 100.0, 10, false),
        ];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(
            outcome.cues_kept.len(),
            3,
            "non-ErrorRateSpike kinds always survive"
        );
        assert_eq!(outcome.cues_suppressed, 0);
        assert!(outcome.bypass_triggers.is_empty());
    }

    /// Negative-bypass scenario per chunk spec test grid: relative_magnitude=8
    /// (<10 multiplier) AND absolute_rate=0.03 (<0.05 threshold) — neither
    /// dual-condition triggers; cue.suppression_bypassed should be false;
    /// cue IS suppressed during active restart window.
    #[test]
    fn bypass_scenario_8x_3pct_does_not_bypass_suppression() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 8.0, 0.03, 10, false)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert!(
            outcome.cues_kept.is_empty(),
            "8x/3% scenario: cue dropped (no bypass)"
        );
        assert_eq!(outcome.cues_suppressed, 1);
        assert!(outcome.bypass_triggers.is_empty());
    }

    /// Positive-bypass scenario per chunk spec test grid: magnitude=12
    /// (>10 multiplier) triggers Relative bypass; cue survives suppression
    /// + BypassTrigger recorded with reason=Relative.
    #[test]
    fn bypass_scenario_12x_4pct_bypasses_via_relative() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 12.0, 0.04, 10, true)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1, "bypass keeps the cue");
        assert_eq!(outcome.cues_suppressed, 0);
        assert_eq!(outcome.bypass_triggers.len(), 1);
        assert_eq!(outcome.bypass_triggers[0].cue_kind, CueKind::ErrorRateSpike);
        assert_eq!(outcome.bypass_triggers[0].reason, BypassReason::Relative);
    }

    /// Positive-bypass scenario per chunk spec test grid: magnitude=6
    /// (<10 multiplier) BUT absolute_rate=0.07 (>0.05 threshold) triggers
    /// Absolute bypass; cue survives suppression + BypassTrigger recorded
    /// with reason=Absolute.
    #[test]
    fn bypass_scenario_6x_7pct_bypasses_via_absolute() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 6.0, 0.07, 10, true)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1);
        assert_eq!(outcome.cues_suppressed, 0);
        assert_eq!(outcome.bypass_triggers.len(), 1);
        assert_eq!(outcome.bypass_triggers[0].reason, BypassReason::Absolute);
    }

    /// Parametric bypass-scenario coverage per chunk #63 plan §Acceptance
    /// tests line 2. The three canonical bypass scenarios are:
    ///   - 8x/3% → NEITHER condition crossed → no bypass, suppressed
    ///   - 12x/4% → magnitude > 10x → Relative bypass
    ///   - 6x/7% → magnitude < 10x BUT absolute > 5% → Absolute bypass
    #[rstest]
    #[case(8.0, 0.03, false, None, 0, 1)]
    #[case(12.0, 0.04, true, Some(BypassReason::Relative), 1, 0)]
    #[case(6.0, 0.07, true, Some(BypassReason::Absolute), 1, 0)]
    fn dual_condition_bypass_scenarios_during_active_restart_window(
        #[case] magnitude: f64,
        #[case] absolute_rate: f64,
        #[case] suppression_bypassed: bool,
        #[case] expected_reason: Option<BypassReason>,
        #[case] expected_kept_len: usize,
        #[case] expected_suppressed: usize,
    ) {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(
            CueKind::ErrorRateSpike,
            "svc-a",
            magnitude,
            absolute_rate,
            10,
            suppression_bypassed,
        )];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), expected_kept_len);
        assert_eq!(outcome.cues_suppressed, expected_suppressed);
        match expected_reason {
            None => assert!(outcome.bypass_triggers.is_empty()),
            Some(r) => {
                assert_eq!(outcome.bypass_triggers.len(), 1);
                assert_eq!(outcome.bypass_triggers[0].reason, r);
            }
        }
    }

    #[test]
    fn evaluate_after_window_expires_does_not_suppress() {
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 3.0, 0.04, 10, false)];
        // Now = resume + 61s — window closed.
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_061 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1, "window expired; cue survives");
        assert_eq!(outcome.cues_suppressed, 0);
    }

    #[test]
    fn evaluate_with_no_scope_id_does_not_match_any_window() {
        // Cue without scope_id can't match a service window — survives.
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Global,
            scope_id: None,
            magnitude: 3.0,
            absolute_value: 0.04,
            persistence_seconds: 10,
            confidence: 0.9,
            priority_tier: PriorityTier::Suggested,
            suppression_bypassed: false,
        }];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.cues_kept.len(), 1);
        assert_eq!(outcome.cues_suppressed, 0);
    }

    #[test]
    fn evaluate_relative_bypass_priority_over_absolute() {
        // Both conditions satisfied: magnitude=15 (>10) AND absolute=0.10 (>0.05).
        // Per derive_bypass_reason short-circuit, Relative wins.
        let s = SuppressionState::new();
        s.record_restart(&restart_event("svc-a", 1_000), 60);
        let p = default_params();
        let cues = vec![cue(CueKind::ErrorRateSpike, "svc-a", 15.0, 0.10, 10, true)];
        let outcome = evaluate_with_suppression(cues, &s, &p, 1_030 * NANOS_PER_SEC);
        assert_eq!(outcome.bypass_triggers.len(), 1);
        assert_eq!(outcome.bypass_triggers[0].reason, BypassReason::Relative);
    }

    #[test]
    fn bypass_reason_labels_match_metric_field_values() {
        assert_eq!(BypassReason::Relative.label(), "relative");
        assert_eq!(BypassReason::Absolute.label(), "absolute");
    }

    #[test]
    fn bypass_reason_round_trips_through_serde() {
        for r in [BypassReason::Relative, BypassReason::Absolute] {
            let json = serde_json::to_string(&r).expect("serialize");
            let parsed: BypassReason = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, r);
        }
    }
}
