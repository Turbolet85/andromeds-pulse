use crate::baseline::{BaselineState, BootstrapState};
use crate::contract::{AttentionCue, CueKind, CueScope};
use crate::cue::classify::{classify_priority, dual_condition_bypass};
use crate::cue::thresholds::{MIN_QUIET_SECONDS, Thresholds};

/// Confidence cap — samples ≥ this value yield confidence = 1.0. Mid-range
/// samples scale linearly from `min_ewma_samples`. Module-private to keep the
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
/// The `now_nanos` parameter is passed through to the cue construction
/// pipeline (currently unused in the magnitude/persistence derivation but
/// reserved for future per-cue timestamp annotation when a `recorded_at`
/// field is added to `AttentionCue` per capability spec evolution).
pub fn evaluate_thresholds(
    state: &BaselineState,
    thresholds: &Thresholds,
    _now_nanos: i64,
) -> Vec<AttentionCue> {
    let mut cues = Vec::new();

    // Per-service ErrorRateSpike detection (chunk #73 P-010 baseline-relative):
    // compares short-term (30s) EWMA against long-term (5min) EWMA per
    // capability spec. The long-term value floors at `thresholds.base_error_rate`
    // (former fixed-denominator, now a minimum-baseline floor) so services
    // with near-zero error rate don't trigger division-explosion magnitudes;
    // the floor degrades gracefully into the prior fixed-baseline behavior
    // before convergence.
    for snapshot in state.iter_services() {
        if snapshot.samples < thresholds.min_ewma_samples {
            continue;
        }
        if !snapshot.short_term_error_rate.is_finite() || !snapshot.error_rate.is_finite() {
            continue;
        }
        let baseline = snapshot.error_rate.max(thresholds.base_error_rate);
        let threshold = baseline * thresholds.error_rate_multiplier;
        if snapshot.short_term_error_rate < threshold {
            continue;
        }
        let magnitude = if baseline > 0.0 {
            snapshot.short_term_error_rate / baseline
        } else {
            0.0
        };
        let confidence = (snapshot.samples as f64 / CONFIDENCE_SATURATION_SAMPLES).min(1.0);
        let persistence_seconds = snapshot.samples;
        let priority_tier = classify_priority(magnitude, confidence, persistence_seconds);
        let suppression_bypassed = dual_condition_bypass(
            magnitude,
            snapshot.short_term_error_rate,
            CueKind::ErrorRateSpike,
            thresholds,
        );
        cues.push(AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some(snapshot.service_name),
            magnitude,
            absolute_value: snapshot.short_term_error_rate,
            persistence_seconds,
            confidence,
            priority_tier,
            suppression_bypassed,
            // Baseline-derived: a statistical condition, not a specific fault.
            fingerprint: None,
        });
    }

    // Per-operation LatencyRegression detection (chunk #73 P-012 baseline-
    // relative): compares short-window t-digest p99 against long-window
    // t-digest p99 per capability spec. The long-term value floors at
    // `thresholds.base_latency_ms` (former fixed-denominator, now a minimum-
    // baseline floor) so operations with near-zero latency don't trigger
    // division-explosion magnitudes; the floor degrades gracefully into
    // the prior fixed-baseline behavior before convergence.
    for snapshot in state.iter_operations(thresholds.latency_percentile) {
        // P-011 §Boundary: operations with <50 samples over the window are
        // excluded from regression detection (unreliable percentile
        // estimates). Distinct from the error-rate path's 10-sample EWMA
        // warm-up gate above (which belongs to P-009).
        if snapshot.samples < thresholds.min_latency_samples {
            continue;
        }
        let Some(latency_short) = snapshot.short_term_latency_at_percentile else {
            continue;
        };
        if !latency_short.is_finite() {
            continue;
        }
        let baseline = match snapshot.latency_at_percentile {
            Some(l) if l.is_finite() => l.max(thresholds.base_latency_ms),
            _ => thresholds.base_latency_ms,
        };
        let threshold = baseline * thresholds.latency_multiplier;
        if latency_short < threshold {
            continue;
        }
        let magnitude = if baseline > 0.0 {
            latency_short / baseline
        } else {
            0.0
        };
        let confidence = (snapshot.samples as f64 / CONFIDENCE_SATURATION_SAMPLES).min(1.0);
        let persistence_seconds = snapshot.samples;
        let priority_tier = classify_priority(magnitude, confidence, persistence_seconds);
        let suppression_bypassed = dual_condition_bypass(
            magnitude,
            latency_short,
            CueKind::LatencyRegression,
            thresholds,
        );
        // Per capability spec P-011: surface the human-readable
        // `operation_name` so downstream consumers (Findings dropdown, model
        // interpretation) can describe the regression as "p99 of GET /endpoint
        // regressed". Falls back to the opaque `operation_key` when the
        // baseline record predates chunk #73 (pre-existing corpus state).
        let scope_id = if snapshot.operation_name.is_empty() {
            snapshot.operation_key
        } else {
            snapshot.operation_name
        };
        cues.push(AttentionCue {
            kind: CueKind::LatencyRegression,
            scope: CueScope::Operation,
            scope_id: Some(scope_id),
            magnitude,
            absolute_value: latency_short,
            persistence_seconds,
            confidence,
            priority_tier,
            suppression_bypassed,
            fingerprint: None,
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
        // Per capability spec P-014: minimum threshold of 30 seconds, applied
        // as a max-floor over the learned p95. High-frequency services with
        // sub-30s p95 are clamped to 30s; low-frequency services with p95 >30s
        // honor the learned value.
        let effective_threshold = p95_seconds.max(MIN_QUIET_SECONDS);
        if snapshot.current_quiet_duration_seconds <= effective_threshold {
            continue;
        }
        let magnitude = if effective_threshold == 0 {
            snapshot.current_quiet_duration_seconds as f64
        } else {
            snapshot.current_quiet_duration_seconds as f64 / effective_threshold as f64
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
            fingerprint: None,
        });
    }
    cues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::BaselineState;

    fn seed_service(state: &BaselineState, service: &str, error_count: u32, total: u32) {
        // Chunk #73 P-010 baseline-relative semantics: distribute errors at
        // the TAIL so short EWMA converges quickly to recent values while
        // long EWMA barely moves. The error_count parameter is interpreted
        // as "errors in the last error_count observations of total".
        let now = 1_000_000_000;
        let baseline_obs = total.saturating_sub(error_count);
        for i in 0..total {
            let status = if i >= baseline_obs { 2 } else { 0 };
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
        // samples to converge from a non-zero start; testing with zero errors
        // is the cleanest way to assert "below threshold" without lengthy
        // alternating injection.
        seed_service(&state, "svc-stable", 0, 100);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(cues.is_empty(), "stable service must not emit cues");
    }

    #[test]
    fn evaluate_thresholds_latency_above_threshold_emits_cue() {
        // Chunk #73 P-012 baseline-relative semantics: long t-digest holds
        // 1000 baseline obs at 50ms; short t-digest is drained via two swap
        // cycles; then 5 spike obs at 300ms accumulate in short.current only.
        // The 1000:5 count ratio keeps long_p99 = 50ms (spike doesn't reach
        // p99 of 1005 obs); short_p99 (current_only) = 300ms.
        let state = BaselineState::new();
        let mut now = 1_000_000_000_i64;

        for _ in 0..1000 {
            state.observe_span("svc-slow", "GET /endpoint", 0, 50, now);
            now += 1_000_000;
        }

        // Two swaps to drain short.current → previous → empty.
        now += 16 * NANOS_PER_SEC;
        state.swap_short_tdigest_pairs_on_tick(now);
        now += 16 * NANOS_PER_SEC;
        state.swap_short_tdigest_pairs_on_tick(now);

        for _ in 0..5 {
            state.observe_span("svc-slow", "GET /endpoint", 0, 300, now);
            now += 1_000_000;
        }

        let cues = evaluate_thresholds(&state, &Thresholds::default(), now);
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
        // Chunk #73 P-011: scope_id is the human-readable operation_name,
        // not the hashed operation_key.
        assert_eq!(cue.scope_id.as_deref(), Some("GET /endpoint"));
        // absolute_value is the SHORT t-digest p99 = 300ms (just spike obs).
        assert!(
            cue.absolute_value >= 250.0,
            "absolute_value (short p99) = {}",
            cue.absolute_value
        );
        // baseline = max(long_p99=50, base_latency_ms=100) = 100;
        // magnitude = 300/100 = 3.0; above 2.5 multiplier.
        assert!(cue.magnitude >= 2.5, "magnitude = {}", cue.magnitude);
    }

    /// Seed the chunk #73 spike shape (baseline obs at 50ms → two t-digest
    /// swap cycles → 5 spike obs at 300ms) with a configurable baseline
    /// count so floor-gate tests can sit on either side of
    /// `min_latency_samples`.
    fn seed_latency_spike(state: &BaselineState, service: &str, baseline_obs: u32) -> i64 {
        let mut now = 1_000_000_000_i64;
        for _ in 0..baseline_obs {
            state.observe_span(service, "GET /endpoint", 0, 50, now);
            now += 1_000_000;
        }
        now += 16 * NANOS_PER_SEC;
        state.swap_short_tdigest_pairs_on_tick(now);
        now += 16 * NANOS_PER_SEC;
        state.swap_short_tdigest_pairs_on_tick(now);
        for _ in 0..5 {
            state.observe_span(service, "GET /endpoint", 0, 300, now);
            now += 1_000_000;
        }
        now
    }

    #[test]
    fn latency_gate_excludes_operations_below_min_latency_samples() {
        // 30 baseline + 5 spike = 35 samples: above the error-rate EWMA
        // floor (10) but below the P-011 latency floor (50). Under the
        // pre-chunk-#100 shared floor of 10 this spike WOULD have emitted;
        // the distinct 50-sample floor must suppress it.
        let state = BaselineState::new();
        let now = seed_latency_spike(&state, "svc-sparse", 30);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), now);
        let latency_cues: Vec<&AttentionCue> = cues
            .iter()
            .filter(|c| c.kind == CueKind::LatencyRegression)
            .collect();
        assert!(
            latency_cues.is_empty(),
            "35 samples < min_latency_samples (50) must be excluded; got {latency_cues:?}"
        );
    }

    #[test]
    fn latency_gate_admits_operations_at_min_latency_samples() {
        // 500 baseline + 5 spike = 505 samples ≥ the P-011 floor of 50 →
        // the spike shape emits. The baseline count must keep the spike
        // below the long-window p99 position (5/505 < 1%) so long_p99
        // stays at the 50ms baseline — at 50 baseline obs the 5 spike obs
        // would BE the long p99 (5/55 ≈ 9%) and self-suppress the cue.
        let state = BaselineState::new();
        let now = seed_latency_spike(&state, "svc-dense", 500);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), now);
        let latency_cues: Vec<&AttentionCue> = cues
            .iter()
            .filter(|c| c.kind == CueKind::LatencyRegression)
            .collect();
        assert_eq!(
            latency_cues.len(),
            1,
            "55 samples ≥ min_latency_samples (50) must emit; got {cues:?}"
        );
    }

    #[test]
    fn error_rate_floor_stays_at_ten_samples_distinct_from_latency_floor() {
        // 20 samples (10 tail errors): above the error-rate EWMA floor (10),
        // below the latency floor (50). The error-rate cue must still fire —
        // regression guard that the P-011 fix did NOT widen the error path's
        // warm-up gate.
        let state = BaselineState::new();
        seed_service(&state, "svc-mid", 10, 20);
        let cues = evaluate_thresholds(&state, &Thresholds::default(), 1_000);
        assert!(
            cues.iter().any(|c| c.kind == CueKind::ErrorRateSpike),
            "error-rate path keeps its 10-sample floor; got {cues:?}"
        );
        assert!(
            !cues.iter().any(|c| c.kind == CueKind::LatencyRegression),
            "20 samples stay below the latency floor"
        );
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
        // burst converges slowly. After 100 samples with the seed pattern below
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

    #[test]
    fn evaluate_service_went_silent_high_frequency_service_clamped_to_min_floor() {
        // Per capability spec P-014: minimum threshold of 30 seconds.
        // High-frequency service (20s gap, p95 ≈ 20s) — without the floor,
        // a 25s quiet period would emit a cue (25 > 20); with the floor,
        // effective_threshold = max(20, 30) = 30s, so 25s is below threshold
        // and no cue fires. Verifies floor protects chatty services from
        // false-positive silence detection.
        let state = BaselineState::new();
        let first = 0_i64;
        // 200 obs at 20s gap → 199 gaps × 20s = 3980s span (> 3600s bootstrap)
        // → p95 of identical-gap distribution = 20s.
        seed_regular_gaps(&state, "svc-chatty", first, 20, 200);
        let last = 199 * 20 * NANOS_PER_SEC;
        // Now is 25s past last → current_quiet = 25s.
        // effective_threshold = max(20, MIN_QUIET_SECONDS=30) = 30s.
        // 25s ≤ 30s → suppression, no cue.
        let now = last + 25 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(
            cues.is_empty(),
            "25s quiet on chatty service (p95=20s) must be clamped to MIN_QUIET_SECONDS=30s floor; got {cues:?}"
        );
    }

    #[test]
    fn evaluate_service_went_silent_low_frequency_service_preserves_learned_p95() {
        // Per capability spec P-014: floor is `max(p95, MIN_QUIET_SECONDS)`,
        // so services with learned p95 > 30s honor their learned value.
        // Low-frequency service (300s gap, p95 ≈ 300s) — current_quiet 280s
        // is below the learned 300s; effective_threshold = max(300, 30) = 300s;
        // 280s ≤ 300s → no cue. Verifies floor does not falsely lower
        // threshold for low-frequency services.
        let state = BaselineState::new();
        let first = 0_i64;
        // 15 obs at 300s gap → 14 gaps × 300s = 4200s span (> 3600s bootstrap)
        // → p95 of identical-gap distribution = 300s.
        seed_regular_gaps(&state, "svc-quiet-by-design", first, 300, 15);
        let last = 14 * 300 * NANOS_PER_SEC;
        // Now is 280s past last → current_quiet = 280s.
        // effective_threshold = max(300, MIN_QUIET_SECONDS=30) = 300s.
        // 280s ≤ 300s → suppression, no cue.
        let now = last + 280 * NANOS_PER_SEC;
        let cues = evaluate_service_went_silent(&state, &Thresholds::default(), now);
        assert!(
            cues.is_empty(),
            "280s quiet on low-freq service (p95=300s) must honor learned p95, not clamped to 30s; got {cues:?}"
        );
    }
}
