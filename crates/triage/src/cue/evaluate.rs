use crate::baseline::BaselineState;
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
        let suppression_bypassed =
            dual_condition_bypass(magnitude, snapshot.error_rate, CueKind::ErrorRateSpike);
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
            dual_condition_bypass(magnitude, latency, CueKind::LatencyRegression);
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
}
