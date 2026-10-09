//! The instant a service's maximum active-incident tier last changed value.
//!
//! The constellation dot's hue is driven by the maximum `priority_tier`
//! across the service's active incidents, so the instant that hue became
//! true is: the `opened_at_unix_nano` of the incident that raised the
//! maximum (a rise), or the `resolved_at_unix_nano` of the last max-tier
//! holder to leave the active set (a fall). An incident's tier is immutable
//! after opening and acknowledgement keeps it active, so replaying the
//! open/close instants reproduces every value the maximum has taken.

use crate::contract::{CueScope, Incident, IncidentStatus, PriorityTier};

// A fourth private rank beside `cue/emitter.rs`, `digest/assembler.rs` and
// the services resolver; hoisting one shared rank would touch those callers.
fn rank(tier: PriorityTier) -> usize {
    match tier {
        PriorityTier::Curious => 1,
        PriorityTier::Suggested => 2,
        PriorityTier::Autonomous => 3,
    }
}

fn replay(incidents: &[Incident], service: &str) -> (Option<i64>, Option<usize>) {
    let mut events: Vec<(i64, usize, i64)> = Vec::new();
    for inc in incidents {
        if inc.scope != CueScope::Service || inc.scope_id.as_deref() != Some(service) {
            continue;
        }
        let tier = rank(inc.priority_tier);
        if inc.status == IncidentStatus::Resolved {
            let Some(resolved_at) = inc.resolved_at_unix_nano else {
                continue;
            };
            events.push((resolved_at, tier, -1));
        }
        events.push((inc.opened_at_unix_nano, tier, 1));
    }
    events.sort_by_key(|(instant, _, _)| *instant);

    let mut open = [0_i64; 4];
    let mut max: Option<usize> = None;
    let mut last_change = None;
    let mut i = 0;
    while i < events.len() {
        let instant = events[i].0;
        while i < events.len() && events[i].0 == instant {
            open[events[i].1] += events[i].2;
            i += 1;
        }
        let next = (1..open.len()).rev().find(|&r| open[r] > 0);
        if next != max {
            max = next;
            last_change = Some(instant);
        }
    }
    (last_change, max)
}

/// The last instant at which `service`'s maximum active-incident tier
/// changed value, or `None` if it never had an incident.
pub fn tier_effective_at(incidents: &[Incident], service: &str) -> Option<i64> {
    replay(incidents, service).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueKind, EvidenceRefs, Severity};

    const SVC: &str = "payment-service";

    fn incident(id: i64, tier: PriorityTier, opened_at: i64) -> Incident {
        Incident {
            id,
            workspace: "ws".to_string(),
            fingerprint: String::new(),
            title: String::new(),
            detail: String::new(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some(SVC.to_string()),
            status: IncidentStatus::Active,
            severity: Severity::Error,
            priority_tier: tier,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![],
                fingerprint_hashes: vec![],
                timestamps_unix_nano: vec![],
            },
            opened_at_unix_nano: opened_at,
            updated_at_unix_nano: opened_at,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    fn resolved(mut inc: Incident, at: i64) -> Incident {
        inc.status = IncidentStatus::Resolved;
        inc.resolved_at_unix_nano = Some(at);
        inc
    }

    fn acknowledged(mut inc: Incident, at: i64) -> Incident {
        inc.status = IncidentStatus::Acknowledged;
        inc.acknowledged_at_unix_nano = Some(at);
        inc
    }

    fn active_max(incidents: &[Incident]) -> Option<usize> {
        incidents
            .iter()
            .filter(|i| {
                i.scope == CueScope::Service
                    && i.scope_id.as_deref() == Some(SVC)
                    && i.status != IncidentStatus::Resolved
            })
            .map(|i| rank(i.priority_tier))
            .max()
    }

    #[test]
    fn a_higher_tier_opening_is_the_rise() {
        let set = [
            incident(1, PriorityTier::Curious, 100),
            incident(2, PriorityTier::Autonomous, 500),
        ];
        assert_eq!(tier_effective_at(&set, SVC), Some(500));
    }

    #[test]
    fn a_same_tier_second_incident_does_not_move_it() {
        let set = [
            incident(1, PriorityTier::Suggested, 100),
            incident(2, PriorityTier::Suggested, 500),
        ];
        assert_eq!(tier_effective_at(&set, SVC), Some(100));
    }

    #[test]
    fn the_max_holder_resolving_falls_to_the_lower_tier() {
        let set = [
            incident(1, PriorityTier::Curious, 100),
            resolved(incident(2, PriorityTier::Autonomous, 500), 900),
        ];
        assert_eq!(tier_effective_at(&set, SVC), Some(900));
    }

    #[test]
    fn the_last_holder_resolving_falls_to_none() {
        let set = [resolved(incident(1, PriorityTier::Suggested, 100), 700)];
        assert_eq!(tier_effective_at(&set, SVC), Some(700));
    }

    #[test]
    fn concurrent_max_holders_fall_only_when_the_last_resolves() {
        let set = [
            resolved(incident(1, PriorityTier::Autonomous, 100), 400),
            resolved(incident(2, PriorityTier::Autonomous, 200), 800),
            incident(3, PriorityTier::Curious, 50),
        ];
        assert_eq!(tier_effective_at(&set, SVC), Some(800));
    }

    #[test]
    fn acknowledgement_keeps_the_tier() {
        let set = [acknowledged(incident(1, PriorityTier::Suggested, 100), 600)];
        assert_eq!(tier_effective_at(&set, SVC), Some(100));
    }

    #[test]
    fn other_services_and_non_service_scopes_are_ignored() {
        let mut other = incident(2, PriorityTier::Autonomous, 900);
        other.scope_id = Some("checkout".to_string());
        let mut global = incident(3, PriorityTier::Autonomous, 950);
        global.scope = CueScope::Global;
        let set = [incident(1, PriorityTier::Curious, 100), other, global];
        assert_eq!(tier_effective_at(&set, SVC), Some(100));
    }

    #[test]
    fn no_incidents_is_none() {
        assert_eq!(tier_effective_at(&[], SVC), None);
    }

    #[test]
    fn rise_fall_rise_returns_the_second_rise() {
        let set = [
            resolved(incident(1, PriorityTier::Suggested, 100), 300),
            incident(2, PriorityTier::Suggested, 700),
        ];
        assert_eq!(tier_effective_at(&set, SVC), Some(700));
    }

    #[test]
    fn a_resolved_incident_without_a_resolution_instant_is_skipped() {
        let mut ghost = incident(2, PriorityTier::Autonomous, 500);
        ghost.status = IncidentStatus::Resolved;
        let set = [incident(1, PriorityTier::Curious, 100), ghost];
        assert_eq!(tier_effective_at(&set, SVC), Some(100));
    }

    #[test]
    fn the_replay_final_maximum_equals_the_max_over_the_active_subset() {
        let fixtures: Vec<Vec<Incident>> = vec![
            vec![],
            vec![incident(1, PriorityTier::Curious, 100)],
            vec![
                incident(1, PriorityTier::Curious, 100),
                incident(2, PriorityTier::Autonomous, 500),
            ],
            vec![
                incident(1, PriorityTier::Curious, 100),
                resolved(incident(2, PriorityTier::Autonomous, 500), 900),
            ],
            vec![
                resolved(incident(1, PriorityTier::Autonomous, 100), 400),
                resolved(incident(2, PriorityTier::Autonomous, 200), 800),
                incident(3, PriorityTier::Curious, 50),
            ],
            vec![acknowledged(incident(1, PriorityTier::Suggested, 100), 600)],
            vec![
                resolved(incident(1, PriorityTier::Suggested, 100), 300),
                incident(2, PriorityTier::Suggested, 700),
            ],
        ];
        for set in &fixtures {
            assert_eq!(replay(set, SVC).1, active_max(set), "fixture {set:?}");
        }
    }
}
