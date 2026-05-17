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

/// Default latency percentile evaluated against the multiplier — p95 is the
/// stable signal of tail latency per obs-plan §5 percentile choice.
pub const DEFAULT_LATENCY_PERCENTILE: f64 = 0.95;

/// Minimum EWMA samples required before a service participates in
/// ErrorRateSpike detection — warm-up gate к suppress cold-start noise.
pub const MIN_EWMA_SAMPLES: u64 = 10;

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
        assert_eq!(t.latency_percentile, 0.95);
        assert_eq!(t.min_ewma_samples, 10);
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
