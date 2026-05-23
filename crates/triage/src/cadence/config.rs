use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Safety-floor minimum for `baseline_seconds` per pulse-v0_2_0-route §80.
/// Below this floor the cadence ticker would saturate the runtime; reject
/// at config-load boundary rather than silently clamp.
pub const CADENCE_BASELINE_SECONDS_MIN: u32 = 5;

/// Safety-floor minimum for `accelerated_seconds` per pulse-v0_2_0-route §80.
pub const CADENCE_ACCELERATED_SECONDS_MIN: u32 = 1;

/// Safety-floor minimum for `reflection_seconds` per pulse-v0_2_0-route §80.
pub const CADENCE_REFLECTION_SECONDS_MIN: u32 = 300;

/// Default `baseline_seconds` per pulse-v0_2_0-route §80 ("Default cadence
/// 60s baseline").
pub const DEFAULT_CADENCE_BASELINE_SECONDS: u32 = 60;

/// Default `accelerated_seconds` per pulse-v0_2_0-route §80 ("20s
/// accelerated").
pub const DEFAULT_CADENCE_ACCELERATED_SECONDS: u32 = 20;

/// Default `reflection_seconds` per pulse-v0_2_0-route §80 ("1800s
/// reflection").
pub const DEFAULT_CADENCE_REFLECTION_SECONDS: u32 = 1800;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CadenceConfigError {
    #[error("baseline_seconds={requested} below safety floor {floor}")]
    BaselineBelowFloor { requested: u32, floor: u32 },
    #[error("accelerated_seconds={requested} below safety floor {floor}")]
    AcceleratedBelowFloor { requested: u32, floor: u32 },
    #[error("reflection_seconds={requested} below safety floor {floor}")]
    ReflectionBelowFloor { requested: u32, floor: u32 },
}

/// Cadence coordinator configuration. Loaded from `[triage.cadence]` config
/// section persisted via `ui_bridge::Settings`; values pre-validated at
/// `Settings::validate()` boundary so `try_new` at coordinator-spawn time
/// is a defensive-but-redundant gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CadenceConfig {
    pub baseline_seconds: u32,
    pub accelerated_seconds: u32,
    pub reflection_seconds: u32,
    pub tier2_acceleration_enabled: bool,
}

impl CadenceConfig {
    /// Construct a `CadenceConfig` with safety-floor validation. Returns
    /// the first violating-field `Err` variant; downstream callers MAY
    /// emit a `warn`-level event at `target = "cadence.config.safety_floor"`
    /// before discarding the requested value and falling back к defaults.
    pub fn try_new(
        baseline_seconds: u32,
        accelerated_seconds: u32,
        reflection_seconds: u32,
        tier2_acceleration_enabled: bool,
    ) -> Result<Self, CadenceConfigError> {
        if baseline_seconds < CADENCE_BASELINE_SECONDS_MIN {
            return Err(CadenceConfigError::BaselineBelowFloor {
                requested: baseline_seconds,
                floor: CADENCE_BASELINE_SECONDS_MIN,
            });
        }
        if accelerated_seconds < CADENCE_ACCELERATED_SECONDS_MIN {
            return Err(CadenceConfigError::AcceleratedBelowFloor {
                requested: accelerated_seconds,
                floor: CADENCE_ACCELERATED_SECONDS_MIN,
            });
        }
        if reflection_seconds < CADENCE_REFLECTION_SECONDS_MIN {
            return Err(CadenceConfigError::ReflectionBelowFloor {
                requested: reflection_seconds,
                floor: CADENCE_REFLECTION_SECONDS_MIN,
            });
        }
        Ok(Self {
            baseline_seconds,
            accelerated_seconds,
            reflection_seconds,
            tier2_acceleration_enabled,
        })
    }
}

impl Default for CadenceConfig {
    fn default() -> Self {
        Self {
            baseline_seconds: DEFAULT_CADENCE_BASELINE_SECONDS,
            accelerated_seconds: DEFAULT_CADENCE_ACCELERATED_SECONDS,
            reflection_seconds: DEFAULT_CADENCE_REFLECTION_SECONDS,
            tier2_acceleration_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn default_passes_validation() {
        let default = CadenceConfig::default();
        let validated = CadenceConfig::try_new(
            default.baseline_seconds,
            default.accelerated_seconds,
            default.reflection_seconds,
            default.tier2_acceleration_enabled,
        );
        assert_eq!(validated, Ok(default));
    }

    #[test]
    fn baseline_at_floor_passes_and_below_floor_rejects() {
        assert!(CadenceConfig::try_new(CADENCE_BASELINE_SECONDS_MIN, 20, 1800, true).is_ok());
        assert!(CadenceConfig::try_new(CADENCE_BASELINE_SECONDS_MIN + 1, 20, 1800, true).is_ok());
        let err = CadenceConfig::try_new(CADENCE_BASELINE_SECONDS_MIN - 1, 20, 1800, true)
            .expect_err("below floor must reject");
        assert!(matches!(err, CadenceConfigError::BaselineBelowFloor { .. }));
    }

    #[test]
    fn accelerated_at_floor_passes_and_below_floor_rejects() {
        assert!(CadenceConfig::try_new(60, CADENCE_ACCELERATED_SECONDS_MIN, 1800, true).is_ok());
        assert!(
            CadenceConfig::try_new(60, CADENCE_ACCELERATED_SECONDS_MIN + 1, 1800, true).is_ok()
        );
        let err = CadenceConfig::try_new(60, 0, 1800, true).expect_err("below floor must reject");
        assert!(matches!(
            err,
            CadenceConfigError::AcceleratedBelowFloor { .. }
        ));
    }

    #[test]
    fn reflection_at_floor_passes_and_below_floor_rejects() {
        assert!(CadenceConfig::try_new(60, 20, CADENCE_REFLECTION_SECONDS_MIN, true).is_ok());
        assert!(CadenceConfig::try_new(60, 20, CADENCE_REFLECTION_SECONDS_MIN + 1, true).is_ok());
        let err = CadenceConfig::try_new(60, 20, CADENCE_REFLECTION_SECONDS_MIN - 1, true)
            .expect_err("below floor must reject");
        assert!(matches!(
            err,
            CadenceConfigError::ReflectionBelowFloor { .. }
        ));
    }

    #[test]
    fn tier2_acceleration_enabled_false_passes_validation() {
        let cfg = CadenceConfig::try_new(60, 20, 1800, false).expect("ok");
        assert!(!cfg.tier2_acceleration_enabled);
    }

    proptest! {
        #[test]
        fn safety_floor_invariant(
            baseline in 0u32..86_400,
            accelerated in 0u32..86_400,
            reflection in 0u32..86_400,
            tier2 in any::<bool>(),
        ) {
            let result = CadenceConfig::try_new(baseline, accelerated, reflection, tier2);
            match result {
                Ok(cfg) => {
                    prop_assert!(cfg.baseline_seconds >= CADENCE_BASELINE_SECONDS_MIN);
                    prop_assert!(cfg.accelerated_seconds >= CADENCE_ACCELERATED_SECONDS_MIN);
                    prop_assert!(cfg.reflection_seconds >= CADENCE_REFLECTION_SECONDS_MIN);
                    prop_assert_eq!(cfg.baseline_seconds, baseline);
                    prop_assert_eq!(cfg.accelerated_seconds, accelerated);
                    prop_assert_eq!(cfg.reflection_seconds, reflection);
                    prop_assert_eq!(cfg.tier2_acceleration_enabled, tier2);
                }
                Err(CadenceConfigError::BaselineBelowFloor { requested, floor }) => {
                    prop_assert_eq!(requested, baseline);
                    prop_assert_eq!(floor, CADENCE_BASELINE_SECONDS_MIN);
                    prop_assert!(baseline < CADENCE_BASELINE_SECONDS_MIN);
                }
                Err(CadenceConfigError::AcceleratedBelowFloor { requested, floor }) => {
                    prop_assert_eq!(requested, accelerated);
                    prop_assert_eq!(floor, CADENCE_ACCELERATED_SECONDS_MIN);
                    prop_assert!(accelerated < CADENCE_ACCELERATED_SECONDS_MIN);
                    prop_assert!(baseline >= CADENCE_BASELINE_SECONDS_MIN);
                }
                Err(CadenceConfigError::ReflectionBelowFloor { requested, floor }) => {
                    prop_assert_eq!(requested, reflection);
                    prop_assert_eq!(floor, CADENCE_REFLECTION_SECONDS_MIN);
                    prop_assert!(reflection < CADENCE_REFLECTION_SECONDS_MIN);
                    prop_assert!(baseline >= CADENCE_BASELINE_SECONDS_MIN);
                    prop_assert!(accelerated >= CADENCE_ACCELERATED_SECONDS_MIN);
                }
            }
        }
    }
}
