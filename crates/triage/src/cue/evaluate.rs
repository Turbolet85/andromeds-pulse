use crate::baseline::{BaselineState, BootstrapState};
use crate::contract::{AttentionCue, CueKind, CueScope};
use crate::cue::classify::{classify_priority, dual_condition_bypass};
use crate::cue::thresholds::Thresholds;

/// Confidence cap — samples ≥ this value yield confidence = 1.0. Mid-range
/// samples scale linearly от `min_ewma_samples`. Module-private к keep the
/// derivation curve local; tune at /implement if calibration warrants.
const CONFIDENCE_SATURATION_SAMPLES: f64 = 100.0;

/// Pure synchronous threshold evaluator. Iterates per-service + per-operation
/// snapshots from `BaselineState`; constructs `AttentionCue` payloads for
/// each above-threshold case after applying warm-up + classification.
///
/// Returns the cues unsorted; caller's emit step iterates the Vec. Designed
/// for direct unit-test invocation without `tokio::time` orchestration per
/// chunk #62 plan Implementation Steps step 4.
///
/// The `now_nanos` parameter is passed through к the cue construction
/// pipeline (currently unused в the magnitude/persistence derivation but
/// reserved for future per-cue timestamp annotation when a `recorded_at`
/// field is added к `AttentionCue` per capability spec evolution).
pub fn evaluate_thresholds(
    state: &BaselineState,
    thresholds: &Thresholds,
    _now_nanos: i64,
) -> Vec<AttentionCue> {
    let mut cues = Vec::new();

    // Per-service ErrorRateSpike detection.
    for snapshot in state.iter_services() {
        if snapshot.samples < thresholds.min_ewma_samples {
            continue;
        }
        let threshold = thresholds.base_error_rate * thresholds.error_rate_multiplier;
        if !snapshot.error_rate.is_finite() || snapshot.error_rate < threshold {
            continue;
        }
        let magnitude = if thresholds.base_error_rate > 0.0 {
            snapshot.error_rate / thresholds.base_error_rate
        } else {
            0.0
        };
        let confidence = (snapshot.samples as f64 / CONFIDENCE_SATURATION_SAMPLES).min(1.0);
        let persistence_seconds = snapshot.samples;
        let priority_tier = classify_priority(magnitude, confidence, persistence_seconds);
        let suppression_bypassed = dual_condition_bypass(
            magnitude,
            snapshot.error_rate,
            CueKind::ErrorRateSpike,
            thresholds,
        );
        cues.push(AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some(snapshot.service_name),
            magnitude,
            absolute_value: snapshot.error_rate,
            persistence_seconds,
            confidence,
            priority_tier,
            suppression_bypassed,
        });
    }

    // Per-operation LatencyRegression detection.
    for snapshot in state.iter_operations(thresholds.latency_percentile) {
        if snapshot.samples < thresholds.min_ewma_samples {
            continue;
        }
        let Some(latency) = snapshot.latency_at_percentile else {
            continue;
        };
        let threshold = thresholds.base_latency_ms * thresholds.latency_multiplier;
        if !latency.is_finite() || latency < threshold {
            continue;
        }
        let magnitude = if thresholds.base_latency_ms > 0.0 {
            latency / thresholds.base_latency_ms
        } else {
            0.0
        };
        let confidence = (snapshot.samples as f64 / CONFIDENCE_SATURATION_SAMPLES).min(1.0);
        let persistence_seconds = snapshot.samples;
        let priority_tier = classify_priority(magnitude, confidence, persistence_seconds);
        let suppression_bypassed =
            dual_condition_bypass(magnitude, latency, CueKind::LatencyRegression, thresholds);
        cues.push(AttentionCue {
            kind: CueKind::LatencyRegression,
            scope: CueScope::Operation,
            scope_id: Some(snapshot.operation_key),
            magnitude,
            absolute_value: latency,
            persistence_seconds,
            confidence,
            priority_tier,
            suppression_bypassed,
        });
    }

    cues
}

/// Pure synchronous evaluator for chunk #64 ServiceWentSilent cues. Iterates
/// per-service silence snapshots; emits a cue ONLY when the service has
/// crossed the bootstrap window (`bootstrap_state == Ready`) AND its current
/// quiet duration exceeds its own learned p95 quiet duration. Cues are NOT
/// subject to chunk #63 restart-window suppression (the suppression filter
/// in `pattern::suppression::evaluate_with_suppression` only filters
/// `ErrorRateSpike`), so `suppression_bypassed = false` is the only valid
/// value at construction.
///
/// Bootstrap state derivation depends ONLY on internal clock per security
/// extract — incoming OTLP attribute values cannot influence the gate
/// regardless of attribute shape.
pub fn evaluate_service_went_silent(
    state: &BaselineState,
    _thresholds: &Thresholds,
    now_nanos: i64,
) -> Vec<AttentionCue> {
    let mut cues = Vec::new();
    for snapshot in state.iter_service_silence_snapshots(now_nanos) {
        if snapshot.bootstrap_state != BootstrapState::Ready {
            continue;
        }
        let Some(p95_seconds) = snapshot.p95_historical_quiet_duration_seconds else {
            continue;
        };
        if snapshot.current_quiet_duration_seconds <= p95_seconds {
            continue;
        }
        let magnitude = if p95_seconds == 0 {
            snapshot.current_quiet_duration_seconds as f64
        } else {
            snapshot.current_quiet_duration_seconds as f64 / p95_seconds as f64
        };
        let confidence = 1.0;
        let persistence_seconds = snapshot.current_quiet_duration_seconds;
        let priority_tier = classify_priority(magnitude, confidence, persistence_seconds);
        cues.push(AttentionCue {
            kind: CueKind::ServiceWentSilent,
            scope: CueScope::Service,
            scope_id: Some(snapshot.service_name),
            magnitude,
            absolute_value: snapshot.current_quiet_duration_seconds as f64,
            persistence_seconds,
            confidence,
            priority_tier,
            suppression_bypassed: false,
        });
    }
    cues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::BaselineState;

    fn seed_service(state: &BaselineState, service: &str, error_count: u32, total: u32) {
        let now = 1_000_000_000;
        for i in 0..total {
            let status = if i < error_count { 2 } else { 0 };
            state.observe_span(service, "op-x", status, 50, now + (i as i64) * 1_000_000);
        }
    }

    fn seed_latency(
        state: &BaselineState,
        service: &str,
        operation: &str,
        latency_ms: u64,
        count: u32,
    ) {
        let now = 1_000_000_000;
        for i in 0..count {
            state.observe_span(
                service,
                operation,
                0,
                latency_ms,
                now + (i as i64) * 1_000_000,
            );
        }
    }

    #[test]
    fn evaluate_thresholds_empty_state_returns_no_cues() {
        let state = BaselineState::new();
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(cues.is_empty());
    }

    #[test]
    fn evaluate_thresholds_below_warmup_returns_no_cues() {
        let state = BaselineState::new();
        // Only 5 samples — below MIN_EWMA_SAMPLES (10).
        seed_service(&state, "svc-a", 5, 5);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(
            cues.is_empty(),
            "warm-up gate must suppress cold-start cues"
        );
    }

    #[test]
    fn evaluate_thresholds_error_rate_above_threshold_emits_cue() {
        let state = BaselineState::new();
        // 100 spans, 50% errors → EWMA converges toward 0.5; threshold = 0.03 (3% = 3.0 * 1%).
        seed_service(&state, "svc-checkout", 50, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert_eq!(cues.len(), 1, "expected one cue, got {cues:?}");
        let cue = &cues[0];
        assert_eq!(cue.kind, CueKind::ErrorRateSpike);
        assert_eq!(cue.scope, CueScope::Service);
        assert_eq!(cue.scope_id.as_deref(), Some("svc-checkout"));
        assert!(cue.magnitude > 1.0, "magnitude must be > 1x base rate");
        assert!(cue.absolute_value > 0.03, "absolute > threshold floor");
        assert!(cue.confidence > 0.0 && cue.confidence <= 1.0);
    }

    #[test]
    fn evaluate_thresholds_low_error_rate_does_not_emit_cue() {
        let state = BaselineState::new();
        // 100 spans, 0 errors → EWMA stays at 0.0 (well below 3% threshold).
        // Note: the 5-minute EWMA window (alpha=0.00333) needs many more
        // samples to converge from a non-zero start; testing с zero errors
        // is the cleanest way to assert "below threshold" without lengthy
        // alternating injection.
        seed_service(&state, "svc-stable", 0, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(cues.is_empty(), "stable service must not emit cues");
    }

    #[test]
    fn evaluate_thresholds_latency_above_threshold_emits_cue() {
        let state = BaselineState::new();
        // Default threshold = 100 * 2.5 = 250ms. Seed 300ms latency.
        seed_latency(&state, "svc-slow", "GET /endpoint", 300, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        let latency_cues: Vec<&AttentionCue> = cues
            .iter()
            .filter(|c| c.kind == CueKind::LatencyRegression)
            .collect();
        assert_eq!(
            latency_cues.len(),
            1,
            "expected one latency cue, got {cues:?}"
        );
        let cue = latency_cues[0];
        assert_eq!(cue.scope, CueScope::Operation);
        assert!(cue.scope_id.as_deref().unwrap().starts_with("svc-slow/"));
        assert!(cue.absolute_value >= 250.0);
    }

    #[test]
    fn evaluate_thresholds_fast_latency_does_not_emit_cue() {
        let state = BaselineState::new();
        // 50ms latency — well below 250ms threshold.
        seed_latency(&state, "svc-fast", "GET /endpoint", 50, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        let latency_cues: Vec<&AttentionCue> = cues
            .iter()
            .filter(|c| c.kind == CueKind::LatencyRegression)
            .collect();
        assert!(latency_cues.is_empty());
    }

    #[test]
    fn evaluate_thresholds_classification_flips_when_multiplier_raised() {
        // EWMA with alpha=0.00333 (5-min window) starting from a 50%-error
        // burst converges slowly. After 100 samples с the seed pattern below
        // (50 errors then 50 good) the EWMA is ~0.846. Default multiplier
        // 3.0 × 0.01 = 0.03 threshold → cue fires (0.846 >> 0.03).
        // Multiplier 100.0 × 0.01 = 1.0 threshold → cue suppressed
        // (0.846 < 1.0).
        let state = BaselineState::new();
        seed_service(&state, "svc-active", 50, 100);
        let cues_default = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert_eq!(cues_default.len(), 1);

        let strict_thresholds = Thresholds {
            error_rate_multiplier: 100.0,
            ..Thresholds::default()
        };
        let cues_strict = evaluate_thresholds(&state, &strict_thresholds, 1_000);
        assert!(
            cues_strict.is_empty(),
            "raised multiplier must suppress cue at this EWMA value"
        );
    }

    #[test]
    fn evaluate_thresholds_priority_tier_classifies_by_magnitude_and_samples() {
        let state = BaselineState::new();
        // 50% error rate × 100 samples → magnitude ~50x, confidence 1.0,
        // persistence 100 → Autonomous tier.
        seed_service(&state, "svc-severe", 50, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        let cue = cues
            .iter()
            .find(|c| c.kind == CueKind::ErrorRateSpike)
            .expect("cue");
        // High magnitude + saturated samples + persistence ≥ 30s → Autonomous.
        assert_eq!(
            cue.priority_tier,
            crate::contract::PriorityTier::Autonomous,
            "severe ongoing condition must classify as Autonomous"
        );
        assert!(
            cue.suppression_bypassed,
            "magnitude > 10x must bypass suppression"
        );
    }

    #[test]
    fn evaluate_thresholds_empty_service_name_excluded_from_cues() {
        let state = BaselineState::new();
        // observe_span with empty service.name drops the span (chunk #61
        // discipline); state.services stays empty.
        for i in 0..100 {
            state.observe_span("", "op", 2, 50, 1_000_000 + i * 1_000_000);
        }
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(
            cues.is_empty(),
            "empty service.name spans must not produce cues"
        );
    }

    const NANOS_PER_SEC: i64 = 1_000_000_000;

    /// Observe a service at regular gaps to build a p95 distribution.
    /// `count` observations at `gap_seconds` apart starting at `first_nanos`.
    fn seed_regular_gaps(
        state: &BaselineState,
        service: &str,
        first_nanos: i64,
        gap_seconds: i64,
        count: i64,
    ) {
        for i in 0..count {
            let now = first_nanos + (i * gap_seconds) * NANOS_PER_SEC;
            state.observe_span(service, "op", 0, 50, now);
        }
    }

    #[test]
    fn evaluate_service_went_silent_empty_state_returns_no_cues() {
        let state = BaselineState::new();
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), 1_000);
        assert!(cues.is_empty());
    }

    #[test]
    fn evaluate_service_went_silent_during_bootstrap_returns_no_cues_even_when_quiet_long() {
        let state = BaselineState::new();
        let first = 1_000 * NANOS_PER_SEC;
        seed_regular_gaps(&state, "svc-fresh", first, 60, 30);
        // 30 minutes of observations + now = first + 2 hours but observe stopped
        // after ~30min → current_quiet = ~5400s. Bootstrap end requires
        // 3600s elapsed since FIRST observation. now = first + 1800s + 3600s
        // would be past bootstrap. Instead pick now = first + 2000s (still in
        // bootstrap window — only 2000s elapsed since first).
        let now = first + 2_000 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(cues.is_empty(), "bootstrap suppression must hold");
    }

    #[test]
    fn evaluate_service_went_silent_post_bootstrap_below_p95_returns_no_cues() {
        let state = BaselineState::new();
        let first = 0_i64;
        // 70 observations at 60s gap → first=0, last=4140s, p95~60s, bootstrap
        // requires now ≥ first + 3600s.
        seed_regular_gaps(&state, "svc-active", first, 60, 70);
        let last = 69 * 60 * NANOS_PER_SEC;
        // now is 30s after last → current_quiet = 30s ≤ p95 (60s) → no cue.
        let now = last + 30 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(cues.is_empty(), "current_quiet ≤ p95 must not emit");
    }

    #[test]
    fn evaluate_service_went_silent_post_bootstrap_above_p95_returns_cue() {
        let state = BaselineState::new();
        let first = 0_i64;
        seed_regular_gaps(&state, "svc-quiet", first, 60, 70);
        let last = 69 * 60 * NANOS_PER_SEC;
        // Advance now to 600s past last → current_quiet = 600s >> p95 (60s).
        // Bootstrap done: now - first = ~4740s > 3600s.
        let now = last + 600 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert_eq!(cues.len(), 1, "expected one ServiceWentSilent cue");
        let cue = &cues[0];
        assert_eq!(cue.kind, CueKind::ServiceWentSilent);
        assert_eq!(cue.scope, CueScope::Service);
        assert_eq!(cue.scope_id.as_deref(), Some("svc-quiet"));
        assert!(cue.magnitude > 1.0);
        assert!(!cue.suppression_bypassed);
    }

    #[test]
    fn evaluate_service_went_silent_bursty_pattern_keeps_quiet_phases_silent() {
        let state = BaselineState::new();
        // Bursty: 10 bursts of 5 observations each. Within-burst gaps are
        // 10s (40 small gaps); between-burst inter-arrival is 1200s producing
        // 9 large gaps of ~1160s. p95 of [10×40, 1160×9] lands in the upper
        // tail (near 1160s) because 95% of 49 samples = index 46, which is
        // the 7th of the 9 large gaps after sorting.
        let bursts: i64 = 10;
        let per_burst: i64 = 5;
        let within_gap: i64 = 10;
        let between_gap: i64 = 1_200;
        for burst in 0..bursts {
            let burst_start = burst * between_gap * NANOS_PER_SEC;
            for i in 0..per_burst {
                state.observe_span(
                    "svc-bursty",
                    "op",
                    0,
                    50,
                    burst_start + (i * within_gap) * NANOS_PER_SEC,
                );
            }
        }
        let last = (bursts - 1) * between_gap * NANOS_PER_SEC
            + (per_burst - 1) * within_gap * NANOS_PER_SEC;
        // Now = last + 500s → less than learned p95 (~1160s) → no cue.
        // Bootstrap done: now - first = (bursts-1)*between_gap + 4*within_gap + 500
        // = 9*1200 + 40 + 500 = 11340s > 3600s ✓.
        let now = last + 500 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(
            cues.is_empty(),
            "500s quiet inside bursty pattern (learned p95 ~1160s) should not emit; got {cues:?}"
        );
    }

    #[test]
    fn evaluate_service_went_silent_attribute_poisoning_does_not_bypass_bootstrap() {
        let state = BaselineState::new();
        let first = 1_000_000_000_000_i64;
        // Poison: service.name shaped like an internal-state-machine
        // assertion. Bootstrap MUST stay Learning regardless of the string
        // content — the gate consults internal clock only.
        for name in [
            "bootstrap.complete=true",
            "quiet.duration=0",
            "ready",
            "BootstrapState::Ready",
        ] {
            state.observe_span(name, "op", 0, 50, first);
        }
        // now is only 1 second after first observations → bootstrap not done.
        let now = first + NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(
            cues.is_empty(),
            "bootstrap suppression must hold under attribute-shaped service names"
        );
    }
}
