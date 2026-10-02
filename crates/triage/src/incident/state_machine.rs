//! Incident lifecycle state machine — chunk #78.
//!
//! Pure-function transition table + cool-down expiry helper + auto-resolve
//! predicate. State variants live on `crate::contract::IncidentStatus`
//! (defined in chunk #60); this module covers the transition gate +
//! timing helpers consumed by `IncidentRegistry`.

use crate::contract::IncidentStatus;

/// Spec-valid lifecycle edges per capability spec P-022:
///
///   - Active → Acknowledged (user-driven via `incidents.acknowledge`)
///   - Active → Resolved (auto via 120s no-reemission OR
///     `incidents.mark_resolved`)
///   - Acknowledged → Resolved (auto via 120s no-reemission OR
///     `incidents.mark_resolved`)
///
/// Resolved is an absorbing state — no transitions out. Self-loops are NOT
/// valid (no Active → Active or Resolved → Resolved).
pub fn is_valid_incident_transition(from: IncidentStatus, to: IncidentStatus) -> bool {
    use IncidentStatus::*;
    matches!(
        (from, to),
        (Active, Acknowledged) | (Active, Resolved) | (Acknowledged, Resolved)
    )
}

/// Compute the cool-down expiry timestamp from `now_unix_nano` and the
/// configured cool-down window in seconds. Used by
/// `IncidentRegistry::acknowledge` to enforce the 5-min cool-down per
/// `(kind, scope, workspace)` tuple per capability spec P-023.
pub fn cooldown_expiry_unix_nano(now_unix_nano: i64, cooldown_secs: u64) -> i64 {
    let cooldown_nanos = (cooldown_secs as i64).saturating_mul(1_000_000_000);
    now_unix_nano.saturating_add(cooldown_nanos)
}

/// Predicate: should an incident be auto-resolved? Returns true if the
/// incident's `updated_at_unix_nano` plus the no-reemission window is at
/// or before `now_unix_nano`. Per capability spec P-022 default window =
/// 120s.
///
/// Pure function w.r.t. inputs — caller injects deterministic time for
/// unit tests.
pub fn should_auto_resolve(
    updated_at_unix_nano: i64,
    now_unix_nano: i64,
    no_reemission_window_secs: u64,
) -> bool {
    let window_nanos = (no_reemission_window_secs as i64).saturating_mul(1_000_000_000);
    let deadline = updated_at_unix_nano.saturating_add(window_nanos);
    deadline <= now_unix_nano
}

/// Stable snake_case label for an `IncidentStatus` — used in SQL TEXT
/// column writes + tracing field VALUES. Matches `#[serde(rename_all =
/// "snake_case")]` serialization on the enum so log emissions are
/// consistent with broadcast payloads + corpus column reads + TypeScript
/// bindings.
pub fn status_label(status: IncidentStatus) -> &'static str {
    match status {
        IncidentStatus::Active => "active",
        IncidentStatus::Acknowledged => "acknowledged",
        IncidentStatus::Resolved => "resolved",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use IncidentStatus::*;

    #[test]
    fn is_valid_incident_transition_accepts_spec_edges() {
        assert!(is_valid_incident_transition(Active, Acknowledged));
        assert!(is_valid_incident_transition(Active, Resolved));
        assert!(is_valid_incident_transition(Acknowledged, Resolved));
    }

    #[test]
    fn is_valid_incident_transition_rejects_self_loops() {
        for status in [Active, Acknowledged, Resolved] {
            assert!(
                !is_valid_incident_transition(status, status),
                "self-loop {status:?} → {status:?} should be invalid",
            );
        }
    }

    #[test]
    fn is_valid_incident_transition_rejects_reverse_progression() {
        assert!(!is_valid_incident_transition(Acknowledged, Active));
        assert!(!is_valid_incident_transition(Resolved, Active));
        assert!(!is_valid_incident_transition(Resolved, Acknowledged));
    }

    #[test]
    fn cooldown_expiry_computes_5min_from_now() {
        let now = 1_700_000_000_000_000_000i64;
        let expiry = cooldown_expiry_unix_nano(now, 300);
        assert_eq!(expiry, now + 300_000_000_000);
    }

    #[test]
    fn cooldown_expiry_saturates_on_overflow() {
        let now = i64::MAX - 100;
        let expiry = cooldown_expiry_unix_nano(now, 300);
        assert_eq!(expiry, i64::MAX);
    }

    #[test]
    fn should_auto_resolve_true_after_window() {
        let updated = 1_000_000_000_000_000_000i64;
        let later = updated + 120_000_000_000; // exactly 120s
        assert!(should_auto_resolve(updated, later, 120));
        let way_later = updated + 200_000_000_000;
        assert!(should_auto_resolve(updated, way_later, 120));
    }

    #[test]
    fn should_auto_resolve_false_within_window() {
        let updated = 1_000_000_000_000_000_000i64;
        let just_under = updated + 119_000_000_000; // 119s
        assert!(!should_auto_resolve(updated, just_under, 120));
        let just_after_update = updated + 1;
        assert!(!should_auto_resolve(updated, just_after_update, 120));
    }

    #[test]
    fn should_auto_resolve_at_boundary_is_true() {
        let updated = 1_000_000_000_000_000_000i64;
        let exact = updated + 120_000_000_000;
        assert!(should_auto_resolve(updated, exact, 120));
    }

    #[test]
    fn status_label_matches_snake_case_serde() {
        assert_eq!(status_label(Active), "active");
        assert_eq!(status_label(Acknowledged), "acknowledged");
        assert_eq!(status_label(Resolved), "resolved");
    }

    #[test]
    fn status_label_round_trips_through_serde() {
        for status in [Active, Acknowledged, Resolved] {
            let label = status_label(status);
            // Reconstitute via JSON; status_label MUST match the serde
            // serialization on IncidentStatus.
            let from_json: IncidentStatus =
                serde_json::from_str(&format!("\"{label}\"")).expect("deserialize");
            assert_eq!(from_json, status);
        }
    }
}
