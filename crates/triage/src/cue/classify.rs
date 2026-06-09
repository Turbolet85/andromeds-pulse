use crate::contract::{CueKind, PriorityTier};
use crate::cue::thresholds::Thresholds;

/// Classify a detected deviation into one of the three priority tiers per
/// capability spec P-019. Decision boundary (chunk #62 plan Implementation
/// note 1 mapping):
///
/// - `Autonomous` — high magnitude (≥5×), high confidence (≥0.9), sustained
///   (≥30s persistence). Surfaces with prominent halo shift + counter
///   increment.
/// - `Suggested`  — moderate magnitude (≥3×) AND moderate confidence (≥0.7).
///   Quiet counter increment, minimal halo. **Tier-2 cues additionally emit
///   to `cadence-triggers` channel for chunk #72 Cadence Coordinator.**
/// - `Curious`    — everything else worth recording. No interruption.
pub fn classify_priority(
    magnitude: f64,
    confidence: f64,
    persistence_seconds: u64,
) -> PriorityTier {
    if !magnitude.is_finite() || !confidence.is_finite() {
        return PriorityTier::Curious;
    }
    if magnitude >= 5.0 && confidence >= 0.9 && persistence_seconds >= 30 {
        PriorityTier::Autonomous
    } else if magnitude >= 3.0 && confidence >= 0.7 {
        PriorityTier::Suggested
    } else {
        PriorityTier::Curious
    }
}

/// Dual-condition bypass per capability spec P-057: extreme magnitude OR
/// extreme absolute value bypasses restart-window suppression in the
/// chunk #63 `pattern::suppression` evaluator. Chunk #62 computes + records
/// the flag on emission; downstream consumers honor it.
///
/// Thresholds extracted to `&Thresholds` at chunk #63 (previously hardcoded
/// to 10.0 / 0.05 / 1000.0 literals). Short-circuit priority: Relative >
/// Absolute, mirrored by `pattern::suppression::derive_bypass_reason`.
pub fn dual_condition_bypass(
    magnitude: f64,
    absolute_value: f64,
    kind: CueKind,
    thresholds: &Thresholds,
) -> bool {
    if !magnitude.is_finite() || !absolute_value.is_finite() {
        return false;
    }
    if magnitude > thresholds.magnitude_bypass_multiplier {
        return true;
    }
    match kind {
        CueKind::ErrorRateSpike => absolute_value > thresholds.absolute_bypass_error_rate,
        CueKind::LatencyRegression => absolute_value > thresholds.absolute_bypass_latency_ms,
        CueKind::RestartEvent
        | CueKind::ServiceWentSilent
        | CueKind::RetryStorm
        | CueKind::ReflectionTrend => false,
    }
}

/// Stable label for `CueKind` enum variants used in bounded-cardinality
/// tracing field values. Mirrors `serde(rename_all = "snake_case")` on the
/// contract type but as a compile-time string lookup avoiding allocation in
/// the hot path.
pub fn cue_kind_label(kind: CueKind) -> &'static str {
    match kind {
        CueKind::ErrorRateSpike => "error_rate_spike",
        CueKind::LatencyRegression => "latency_regression",
        CueKind::RestartEvent => "restart_event",
        CueKind::ServiceWentSilent => "service_went_silent",
        CueKind::RetryStorm => "retry_storm",
        CueKind::ReflectionTrend => "reflection_trend",
    }
}

/// Stable label for `PriorityTier` enum variants — bounded-cardinality
/// tracing field values. Mirrors `serde(rename_all = "snake_case")`.
pub fn priority_tier_label(tier: PriorityTier) -> &'static str {
    match tier {
        PriorityTier::Autonomous => "autonomous",
        PriorityTier::Suggested => "suggested",
        PriorityTier::Curious => "curious",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5.0, 0.9, 30, PriorityTier::Autonomous)]
    #[case(10.0, 1.0, 60, PriorityTier::Autonomous)]
    #[case(4.99, 0.9, 30, PriorityTier::Suggested)] // magnitude just below 5x
    #[case(5.0, 0.89, 30, PriorityTier::Suggested)] // confidence below 0.9
    #[case(5.0, 0.9, 29, PriorityTier::Suggested)] // persistence below 30s
    #[case(3.0, 0.7, 0, PriorityTier::Suggested)] // boundary into Suggested
    #[case(3.0, 0.69, 0, PriorityTier::Curious)] // confidence below 0.7
    #[case(2.99, 0.9, 60, PriorityTier::Curious)] // magnitude below 3x
    #[case(1.0, 0.5, 5, PriorityTier::Curious)]
    fn classify_priority_decision_boundary(
        #[case] magnitude: f64,
        #[case] confidence: f64,
        #[case] persistence_seconds: u64,
        #[case] expected: PriorityTier,
    ) {
        assert_eq!(
            classify_priority(magnitude, confidence, persistence_seconds),
            expected
        );
    }

    #[test]
    fn classify_priority_nan_inputs_return_curious() {
        assert_eq!(classify_priority(f64::NAN, 0.9, 60), PriorityTier::Curious);
        assert_eq!(classify_priority(5.0, f64::NAN, 60), PriorityTier::Curious);
        assert_eq!(
            classify_priority(f64::INFINITY, 0.9, 60),
            PriorityTier::Curious
        );
    }

    #[rstest]
    #[case(10.5, 0.0, CueKind::ErrorRateSpike, true)] // magnitude > 10x
    #[case(15.0, 0.01, CueKind::LatencyRegression, true)] // magnitude > 10x
    #[case(3.0, 0.06, CueKind::ErrorRateSpike, true)] // abs error > 5%
    #[case(3.0, 0.04, CueKind::ErrorRateSpike, false)] // abs error below 5%
    #[case(3.0, 1500.0, CueKind::LatencyRegression, true)] // abs latency > 1000ms
    #[case(3.0, 900.0, CueKind::LatencyRegression, false)] // abs latency below 1000ms
    #[case(15.0, 0.0, CueKind::RestartEvent, true)] // magnitude > 10x triggers regardless of kind
    fn dual_condition_bypass_matrix(
        #[case] magnitude: f64,
        #[case] absolute_value: f64,
        #[case] kind: CueKind,
        #[case] expected: bool,
    ) {
        let t = Thresholds::default();
        assert_eq!(
            dual_condition_bypass(magnitude, absolute_value, kind, &t),
            expected
        );
    }

    #[test]
    fn dual_condition_bypass_nan_inputs_return_false() {
        let t = Thresholds::default();
        assert!(!dual_condition_bypass(
            f64::NAN,
            0.5,
            CueKind::ErrorRateSpike,
            &t
        ));
        assert!(!dual_condition_bypass(
            5.0,
            f64::NAN,
            CueKind::ErrorRateSpike,
            &t
        ));
    }

    #[test]
    fn dual_condition_bypass_respects_configured_multiplier_override() {
        let t = Thresholds {
            magnitude_bypass_multiplier: 5.0,
            ..Thresholds::default()
        };
        // Magnitude 6.0 < default 10.0 multiplier but > 5.0 override.
        assert!(dual_condition_bypass(6.0, 0.0, CueKind::ErrorRateSpike, &t));
    }

    #[test]
    fn dual_condition_bypass_respects_configured_absolute_error_rate_override() {
        let t = Thresholds {
            absolute_bypass_error_rate: 0.10,
            ..Thresholds::default()
        };
        // Absolute 0.07 > default 0.05 but < 0.10 override → no bypass.
        assert!(!dual_condition_bypass(
            3.0,
            0.07,
            CueKind::ErrorRateSpike,
            &t
        ));
    }

    #[test]
    fn cue_kind_label_matches_serde_snake_case() {
        assert_eq!(cue_kind_label(CueKind::ErrorRateSpike), "error_rate_spike");
        assert_eq!(
            cue_kind_label(CueKind::LatencyRegression),
            "latency_regression"
        );
        assert_eq!(cue_kind_label(CueKind::RestartEvent), "restart_event");
        assert_eq!(
            cue_kind_label(CueKind::ServiceWentSilent),
            "service_went_silent"
        );
        assert_eq!(cue_kind_label(CueKind::RetryStorm), "retry_storm");
    }

    #[test]
    fn priority_tier_label_matches_serde_snake_case() {
        assert_eq!(priority_tier_label(PriorityTier::Autonomous), "autonomous");
        assert_eq!(priority_tier_label(PriorityTier::Suggested), "suggested");
        assert_eq!(priority_tier_label(PriorityTier::Curious), "curious");
    }
}
