use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default error rate multiplier — current EWMA value must exceed
/// `base_error_rate * multiplier` for an `ErrorRateSpike` cue to fire.
/// 3.0× per route §62 chunk text + capability spec P-021.
pub const DEFAULT_ERROR_RATE_MULTIPLIER: f64 = 3.0;

/// Default latency multiplier — current p95 must exceed
/// `base_latency_ms * multiplier` for a `LatencyRegression` cue to fire.
/// 2.5× per route §62 chunk text.
pub const DEFAULT_LATENCY_MULTIPLIER: f64 = 2.5;

/// Default base error rate (1% nominal) used as the multiplier denominator
/// when no streaming baseline is yet established. See chunk #62 plan
/// Implementation note 3.
pub const DEFAULT_BASE_ERROR_RATE: f64 = 0.01;

/// Default base latency (100 ms nominal) used as the multiplier denominator
/// for LatencyRegression detection. See chunk #62 plan Implementation note 4
/// for the fixed-base approach vs rotation-aware diffing trade-off.
pub const DEFAULT_BASE_LATENCY_MS: f64 = 100.0;

/// Default emitter tick interval — matches chunk #59 connection-state poller
/// cadence (1s) per route §62 "1-2s" spec.
pub const DEFAULT_TICK_INTERVAL: Duration = Duration::from_secs(1);

/// Default minimum persistence seconds — a deviation must persist for at
/// least this duration before classifying as `Autonomous` priority.
pub const DEFAULT_MIN_PERSISTENCE_SECONDS: u64 = 30;

/// Default latency percentile evaluated against the multiplier — p99 per
/// capability spec P-012 "current p99 latency for an operation exceeds
/// baseline p99 by factor of 2.5 or more".
pub const DEFAULT_LATENCY_PERCENTILE: f64 = 0.99;

/// Minimum EWMA samples required before a service participates in
/// ErrorRateSpike detection — warm-up gate к suppress cold-start noise.
pub const MIN_EWMA_SAMPLES: u64 = 10;

/// Default dual-condition bypass magnitude multiplier (P-057 chunk #63
/// spec). When a cue's `magnitude > multiplier × baseline` the bypass
/// short-circuits — cue survives restart-window suppression. Matches the
/// chunk #62 `cue::classify::dual_condition_bypass` literal (10.0)
/// extracted к Thresholds for hot-reload in chunk #86.
pub const DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER: f64 = 10.0;

/// Default dual-condition bypass absolute error-rate threshold (5% per
/// chunk #63 spec). `ErrorRateSpike` cues with absolute rate >
/// threshold bypass restart-window suppression.
pub const DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE: f64 = 0.05;

/// Default dual-condition bypass absolute latency threshold (1000ms).
/// `LatencyRegression` cues with absolute latency > threshold bypass
/// restart-window suppression. Mirrors the chunk #62
/// `cue::classify::dual_condition_bypass` literal extracted to Thresholds.
pub const DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS: f64 = 1000.0;

/// Default restart-detection gap threshold (20s per chunk #63 spec).
/// `RestartDetector` emits a `RestartEvent` when a service's gap from
/// the previous observation exceeds this threshold (gap-then-resume).
pub const DEFAULT_RESTART_GAP_THRESHOLD_SECONDS: u64 = 20;

/// Default restart-suppression window (60s per chunk #63 spec). After
/// each `RestartEvent`, the cue emitter suppresses short-persistence
/// `ErrorRateSpike` cues for this duration unless the dual-condition
/// magnitude bypass fires.
pub const DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS: u64 = 60;

/// Default suppression persistence cutoff (30s per chunk #63 spec).
/// `ErrorRateSpike` cues with `persistence_seconds < cutoff` are
/// suppression-eligible; cues with persistence ≥ cutoff survive even
/// during active restart windows (long-persistence cues are real signals,
/// not restart-induced noise).
pub const DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS: u64 = 30;

/// Default activity-floor bootstrap window (1h per chunk #64 spec).
/// During the first hour of observations for a service, `ServiceWentSilent`
/// emission is universally suppressed regardless of quiet duration.
/// Mirrors `baseline::BOOTSTRAP_WINDOW_SECONDS` so config-path / hot-reload
/// can tune the value without touching the baseline module's default.
pub const DEFAULT_BOOTSTRAP_WINDOW_SECONDS: u64 = 3_600;

/// Default percentile used to gate `ServiceWentSilent` emission against the
/// learned historical quiet-duration distribution (chunk #64 P-014).
pub const DEFAULT_QUIET_DURATION_PERCENTILE: f64 = 0.95;

/// Minimum quiet-duration floor for `ServiceWentSilent` emission per
/// capability spec P-014 ("minimum threshold of 30 seconds"). Applied as
/// `max(learned_p95, MIN_QUIET_SECONDS)` so high-frequency services with
/// sub-30s learned p95 still benefit from a 30s baseline floor; low-frequency
/// services with p95 >30s honor the learned value. Compile-time const (not
/// `Thresholds` field) per security extract — keeps the input-validation
/// surface narrow.
pub const MIN_QUIET_SECONDS: u64 = 30;

/// Validation error for `Thresholds`. Local к the cue module so threshold
/// validation does not couple к `BaselineError` shape (chunk #61). Future
/// config-path deserialization MAY convert to a unified error type at the
/// boundary.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThresholdsError {
    #[error("invalid threshold config field `{field}`")]
    InvalidConfig { field: &'static str },
}

/// Configuration for attention cue threshold evaluation. Constructed with
/// hardcoded defaults this chunk; hot-reload wiring + env/TOML injection
/// land in chunk #86. The `validate` method enforces bounded ranges per
/// security plan §Input Validation Configuration row so future config-path
/// deserialization rejects negative / NaN / out-of-range values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub error_rate_multiplier: f64,
    pub latency_multiplier: f64,
    pub base_error_rate: f64,
    pub base_latency_ms: f64,
    pub tick_interval: Duration,
    pub min_persistence_seconds: u64,
    pub latency_percentile: f64,
    pub min_ewma_samples: u64,
    pub magnitude_bypass_multiplier: f64,
    pub absolute_bypass_error_rate: f64,
    pub absolute_bypass_latency_ms: f64,
    pub restart_gap_threshold_seconds: u64,
    pub restart_suppression_window_seconds: u64,
    pub suppression_persistence_cutoff_seconds: u64,
    pub bootstrap_window_seconds: u64,
    pub quiet_duration_percentile: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            error_rate_multiplier: DEFAULT_ERROR_RATE_MULTIPLIER,
            latency_multiplier: DEFAULT_LATENCY_MULTIPLIER,
            base_error_rate: DEFAULT_BASE_ERROR_RATE,
            base_latency_ms: DEFAULT_BASE_LATENCY_MS,
            tick_interval: DEFAULT_TICK_INTERVAL,
            min_persistence_seconds: DEFAULT_MIN_PERSISTENCE_SECONDS,
            latency_percentile: DEFAULT_LATENCY_PERCENTILE,
            min_ewma_samples: MIN_EWMA_SAMPLES,
            magnitude_bypass_multiplier: DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER,
            absolute_bypass_error_rate: DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE,
            absolute_bypass_latency_ms: DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS,
            restart_gap_threshold_seconds: DEFAULT_RESTART_GAP_THRESHOLD_SECONDS,
            restart_suppression_window_seconds: DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS,
            suppression_persistence_cutoff_seconds: DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS,
            bootstrap_window_seconds: DEFAULT_BOOTSTRAP_WINDOW_SECONDS,
            quiet_duration_percentile: DEFAULT_QUIET_DURATION_PERCENTILE,
        }
    }
}

impl Thresholds {
    pub fn validate(&self) -> Result<(), ThresholdsError> {
        if !self.error_rate_multiplier.is_finite() || self.error_rate_multiplier <= 0.0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "error_rate_multiplier",
            });
        }
        if !self.latency_multiplier.is_finite() || self.latency_multiplier <= 0.0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "latency_multiplier",
            });
        }
        if !self.base_error_rate.is_finite()
            || self.base_error_rate <= 0.0
            || self.base_error_rate > 1.0
        {
            return Err(ThresholdsError::InvalidConfig {
                field: "base_error_rate",
            });
        }
        if !self.base_latency_ms.is_finite() || self.base_latency_ms <= 0.0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "base_latency_ms",
            });
        }
        if !(0.0..=1.0).contains(&self.latency_percentile) {
            return Err(ThresholdsError::InvalidConfig {
                field: "latency_percentile",
            });
        }
        if self.tick_interval.is_zero() {
            return Err(ThresholdsError::InvalidConfig {
                field: "tick_interval",
            });
        }
        if !self.magnitude_bypass_multiplier.is_finite() || self.magnitude_bypass_multiplier <= 1.0
        {
            return Err(ThresholdsError::InvalidConfig {
                field: "magnitude_bypass_multiplier",
            });
        }
        if !self.absolute_bypass_error_rate.is_finite()
            || self.absolute_bypass_error_rate <= 0.0
            || self.absolute_bypass_error_rate > 1.0
        {
            return Err(ThresholdsError::InvalidConfig {
                field: "absolute_bypass_error_rate",
            });
        }
        if !self.absolute_bypass_latency_ms.is_finite() || self.absolute_bypass_latency_ms <= 0.0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "absolute_bypass_latency_ms",
            });
        }
        if self.restart_gap_threshold_seconds == 0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "restart_gap_threshold_seconds",
            });
        }
        if self.restart_suppression_window_seconds == 0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "restart_suppression_window_seconds",
            });
        }
        if self.suppression_persistence_cutoff_seconds == 0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "suppression_persistence_cutoff_seconds",
            });
        }
        if self.bootstrap_window_seconds == 0 {
            return Err(ThresholdsError::InvalidConfig {
                field: "bootstrap_window_seconds",
            });
        }
        if !self.quiet_duration_percentile.is_finite()
            || self.quiet_duration_percentile <= 0.0
            || self.quiet_duration_percentile >= 1.0
        {
            return Err(ThresholdsError::InvalidConfig {
                field: "quiet_duration_percentile",
            });
        }
        Ok(())
    }
}

// Module-level compile-time sanity checks per testing.md Session Additions
// 2026-05-11 (clippy::assertions_on_constants forbidden in #[test] fn).
const _: () = {
    assert!(DEFAULT_ERROR_RATE_MULTIPLIER > 1.0);
    assert!(DEFAULT_LATENCY_MULTIPLIER > 1.0);
    assert!(DEFAULT_BASE_ERROR_RATE > 0.0);
    assert!(DEFAULT_BASE_LATENCY_MS > 0.0);
    assert!(MIN_EWMA_SAMPLES >= 1);
    assert!(DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER > 1.0);
    assert!(DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE > 0.0);
    assert!(DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS > 0.0);
    assert!(DEFAULT_RESTART_GAP_THRESHOLD_SECONDS > 0);
    assert!(DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS > 0);
    assert!(DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS > 0);
    assert!(DEFAULT_BOOTSTRAP_WINDOW_SECONDS > 0);
    assert!(DEFAULT_QUIET_DURATION_PERCENTILE > 0.0);
    assert!(DEFAULT_QUIET_DURATION_PERCENTILE < 1.0);
    assert!(MIN_QUIET_SECONDS > 0);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_thresholds_match_chunk_spec_constants() {
        let t = Thresholds::default();
        assert_eq!(t.error_rate_multiplier, 3.0);
        assert_eq!(t.latency_multiplier, 2.5);
        assert_eq!(t.base_error_rate, 0.01);
        assert_eq!(t.base_latency_ms, 100.0);
        assert_eq!(t.tick_interval, Duration::from_secs(1));
        assert_eq!(t.min_persistence_seconds, 30);
        assert_eq!(t.latency_percentile, 0.99);
        assert_eq!(t.min_ewma_samples, 10);
        assert_eq!(t.magnitude_bypass_multiplier, 10.0);
        assert_eq!(t.absolute_bypass_error_rate, 0.05);
        assert_eq!(t.absolute_bypass_latency_ms, 1000.0);
        assert_eq!(t.restart_gap_threshold_seconds, 20);
        assert_eq!(t.restart_suppression_window_seconds, 60);
        assert_eq!(t.suppression_persistence_cutoff_seconds, 30);
        assert_eq!(t.bootstrap_window_seconds, 3_600);
        assert_eq!(t.quiet_duration_percentile, 0.95);
    }

    #[test]
    fn validate_rejects_zero_bootstrap_window_seconds() {
        let t = Thresholds {
            bootstrap_window_seconds: 0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "bootstrap_window_seconds"
            }
        );
    }

    #[test]
    fn validate_rejects_quiet_duration_percentile_outside_unit_range() {
        let t = Thresholds {
            quiet_duration_percentile: 1.5,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "quiet_duration_percentile"
            }
        );
    }

    #[test]
    fn validate_rejects_nan_quiet_duration_percentile() {
        let t = Thresholds {
            quiet_duration_percentile: f64::NAN,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "quiet_duration_percentile"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_quiet_duration_percentile() {
        let t = Thresholds {
            quiet_duration_percentile: 0.0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "quiet_duration_percentile"
            }
        );
    }

    #[test]
    fn validate_rejects_magnitude_bypass_multiplier_below_or_equal_one() {
        let t = Thresholds {
            magnitude_bypass_multiplier: 1.0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "magnitude_bypass_multiplier"
            }
        );
    }

    #[test]
    fn validate_rejects_absolute_bypass_error_rate_above_one() {
        let t = Thresholds {
            absolute_bypass_error_rate: 1.5,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "absolute_bypass_error_rate"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_absolute_bypass_latency_ms() {
        let t = Thresholds {
            absolute_bypass_latency_ms: 0.0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "absolute_bypass_latency_ms"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_restart_gap_threshold_seconds() {
        let t = Thresholds {
            restart_gap_threshold_seconds: 0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "restart_gap_threshold_seconds"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_restart_suppression_window_seconds() {
        let t = Thresholds {
            restart_suppression_window_seconds: 0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "restart_suppression_window_seconds"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_suppression_persistence_cutoff_seconds() {
        let t = Thresholds {
            suppression_persistence_cutoff_seconds: 0,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "suppression_persistence_cutoff_seconds"
            }
        );
    }

    #[test]
    fn validate_rejects_nan_magnitude_bypass_multiplier() {
        let t = Thresholds {
            magnitude_bypass_multiplier: f64::NAN,
            ..Thresholds::default()
        };
        assert_eq!(
            t.validate().unwrap_err(),
            ThresholdsError::InvalidConfig {
                field: "magnitude_bypass_multiplier"
            }
        );
    }

    #[test]
    fn validate_default_thresholds_succeeds() {
        assert!(Thresholds::default().validate().is_ok());
    }

    #[test]
    fn validate_rejects_negative_error_rate_multiplier() {
        let t = Thresholds {
            error_rate_multiplier: -1.0,
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "error_rate_multiplier"
            }
        );
    }

    #[test]
    fn validate_rejects_nan_latency_multiplier() {
        let t = Thresholds {
            latency_multiplier: f64::NAN,
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "latency_multiplier"
            }
        );
    }

    #[test]
    fn validate_rejects_base_error_rate_above_one() {
        let t = Thresholds {
            base_error_rate: 1.5,
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "base_error_rate"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_base_latency() {
        let t = Thresholds {
            base_latency_ms: 0.0,
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "base_latency_ms",
            }
        );
    }

    #[test]
    fn validate_rejects_percentile_outside_unit_range() {
        let t = Thresholds {
            latency_percentile: 1.5,
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "latency_percentile"
            }
        );
    }

    #[test]
    fn validate_rejects_zero_tick_interval() {
        let t = Thresholds {
            tick_interval: Duration::from_secs(0),
            ..Thresholds::default()
        };
        let err = t.validate().unwrap_err();
        assert_eq!(
            err,
            ThresholdsError::InvalidConfig {
                field: "tick_interval"
            }
        );
    }

    #[test]
    fn thresholds_round_trip_through_serde() {
        let original = Thresholds::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: Thresholds = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, original);
    }
}
