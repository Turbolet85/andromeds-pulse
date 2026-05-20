//! Service lifecycle state machine — chunk #67.
//!
//! Seven-state finite state machine per service identity (capability spec
//! P-027 Service Constellation Auto-Discovery, formal lifecycle). State
//! transitions are driven by per-service activity inputs derived from
//! chunk #61 `BaselineState` queries (at heartbeat tick time) and chunk
//! #63 restart events (subscribed on `pulse://stream/restart-events`).
//!
//! Variants serialize as snake_case for stable TypeScript binding shape
//! across the TauRPC bridge.

use serde::{Deserialize, Serialize};

/// Per-service lifecycle state per capability spec P-027. Seven discrete
/// states ordered roughly by activity intensity (most-active first;
/// Archived = effectively-removed end state). The `Unknown` variant exists
/// for services seen via prior corpus but not yet observed in the current
/// session (chunk #69 corpus restore territory).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceLifecycleState {
    Unknown,
    Bootstrapping,
    Active,
    Quiet,
    Silent,
    Dormant,
    Archived,
}

/// Trigger reason for a lifecycle transition, carried in
/// `ServiceLifecycleEvent::trigger` to disambiguate the upstream cause.
/// Activity = baseline tracker derived activity-floor signal; Restart =
/// chunk #63 restart event; ThresholdExpiry = silent/dormant/archived
/// progression by clock; ManualOverride = user-driven via
/// `set_manual_override`; CorpusRestore = fires on boot when a service
/// is restored from corpus per chunk #71 LifecyclePersistence trait
/// (emitted as a self-loop `from_state == to_state` for downstream
/// constellation observer cascade).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionTrigger {
    Activity,
    Restart,
    ThresholdExpiry,
    ManualOverride,
    CorpusRestore,
}

/// Returns `true` if a transition `from → to` is permitted by the FSM
/// invariants. Manual override and corpus-restore edges accept any
/// `from → to` pair (covered separately by `is_manual_override_valid`);
/// this function checks only the natural / threshold-driven progression.
///
/// Spec-valid edges (chunk #67 plan §Implementation Step 2):
///   - Unknown → Bootstrapping
///   - Bootstrapping → Active
///   - Active ↔ Quiet  (bidirectional — activity returns lift back to Active)
///   - Quiet → Silent
///   - Silent → Quiet  (activity returns)
///   - Silent → Dormant
///   - Dormant → Archived
///   - Archived → Active (corpus-restore special edge)
///
/// Any other natural-progression edge is rejected. Self-loops
/// (`X → X`) are NOT valid transitions (`tick_all` emits an event only
/// when state actually changes).
pub fn is_valid_transition(from: ServiceLifecycleState, to: ServiceLifecycleState) -> bool {
    use ServiceLifecycleState::*;
    matches!(
        (from, to),
        (Unknown, Bootstrapping)
            | (Bootstrapping, Active)
            | (Active, Quiet)
            | (Quiet, Active)
            | (Quiet, Silent)
            | (Silent, Quiet)
            | (Silent, Dormant)
            | (Dormant, Archived)
            | (Archived, Active)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ServiceLifecycleState::*;

    #[test]
    fn service_lifecycle_state_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&Unknown).expect("serialize"),
            "\"unknown\""
        );
        assert_eq!(
            serde_json::to_string(&Bootstrapping).expect("serialize"),
            "\"bootstrapping\""
        );
        assert_eq!(
            serde_json::to_string(&Active).expect("serialize"),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&Quiet).expect("serialize"),
            "\"quiet\""
        );
        assert_eq!(
            serde_json::to_string(&Silent).expect("serialize"),
            "\"silent\""
        );
        assert_eq!(
            serde_json::to_string(&Dormant).expect("serialize"),
            "\"dormant\""
        );
        assert_eq!(
            serde_json::to_string(&Archived).expect("serialize"),
            "\"archived\""
        );
    }

    #[test]
    fn service_lifecycle_state_round_trips_through_serde() {
        for state in [
            Unknown,
            Bootstrapping,
            Active,
            Quiet,
            Silent,
            Dormant,
            Archived,
        ] {
            let json = serde_json::to_string(&state).expect("serialize");
            let parsed: ServiceLifecycleState = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(parsed, state);
        }
    }

    #[test]
    fn transition_trigger_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&TransitionTrigger::Activity).expect("serialize"),
            "\"activity\""
        );
        assert_eq!(
            serde_json::to_string(&TransitionTrigger::Restart).expect("serialize"),
            "\"restart\""
        );
        assert_eq!(
            serde_json::to_string(&TransitionTrigger::ThresholdExpiry).expect("serialize"),
            "\"threshold_expiry\""
        );
        assert_eq!(
            serde_json::to_string(&TransitionTrigger::ManualOverride).expect("serialize"),
            "\"manual_override\""
        );
        assert_eq!(
            serde_json::to_string(&TransitionTrigger::CorpusRestore).expect("serialize"),
            "\"corpus_restore\""
        );
    }

    #[test]
    fn is_valid_transition_accepts_spec_edges() {
        assert!(is_valid_transition(Unknown, Bootstrapping));
        assert!(is_valid_transition(Bootstrapping, Active));
        assert!(is_valid_transition(Active, Quiet));
        assert!(is_valid_transition(Quiet, Active));
        assert!(is_valid_transition(Quiet, Silent));
        assert!(is_valid_transition(Silent, Quiet));
        assert!(is_valid_transition(Silent, Dormant));
        assert!(is_valid_transition(Dormant, Archived));
        assert!(is_valid_transition(Archived, Active));
    }

    #[test]
    fn is_valid_transition_rejects_self_loops() {
        for state in [
            Unknown,
            Bootstrapping,
            Active,
            Quiet,
            Silent,
            Dormant,
            Archived,
        ] {
            assert!(
                !is_valid_transition(state, state),
                "self-loop {state:?} → {state:?} should be invalid",
            );
        }
    }

    #[test]
    fn is_valid_transition_rejects_skipping_edges() {
        // Skipping Bootstrapping from Unknown.
        assert!(!is_valid_transition(Unknown, Active));
        // Skipping Active from Bootstrapping.
        assert!(!is_valid_transition(Bootstrapping, Quiet));
        // Skipping intermediate states.
        assert!(!is_valid_transition(Active, Silent));
        assert!(!is_valid_transition(Active, Dormant));
        assert!(!is_valid_transition(Quiet, Dormant));
        assert!(!is_valid_transition(Silent, Archived));
        // Resurrection from Archived to non-Active.
        assert!(!is_valid_transition(Archived, Bootstrapping));
        assert!(!is_valid_transition(Archived, Quiet));
    }

    #[test]
    fn is_valid_transition_rejects_reverse_natural_progression() {
        assert!(!is_valid_transition(Bootstrapping, Unknown));
        assert!(!is_valid_transition(Active, Bootstrapping));
        assert!(!is_valid_transition(Dormant, Silent));
        assert!(!is_valid_transition(Archived, Dormant));
    }
}
