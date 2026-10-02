//! Broadcast topic for `IncidentLifecycleEvent` payloads emitted by the
//! chunk #78 incident-lifecycle observer task. Mirrors chunk #67
//! `lifecycle::broadcast::ServiceLifecycleBroadcast` shape exactly per
//! chunk #62/#63/#67 precedent. Webview subscription wiring lives in a
//! future v0.2.0 chunk (incident UI consumer; deferred past chunk #83 L4
//! digest consumers); chunk #78 emit-only.

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::contract::{CueKind, CueScope, IncidentStatus};

/// Broadcast topic name per arch §Occupied Resources `pulse://stream/{kebab}`
/// URI convention. Post-merge, this requires
/// `/andromeda-evolve --allow-arch-registry` to legitimize in §Occupied
/// Resources Tauri IPC events (chunks #59/#62/#63/#67 precedent).
pub const STREAM_NAME_INCIDENTS: &str = "pulse://stream/incidents";

/// Broadcast channel capacity mirroring chunk #62 / #63 / #67 sibling
/// broadcasts (all 32). Sized to absorb subscriber lag under typical
/// workloads without dropping events.
pub const BROADCAST_CAPACITY: usize = 32;

/// Incident lifecycle state-transition event emitted by the chunk #78
/// observer task (`AutoResolveObserver`) and the resolver methods
/// (`incidents.acknowledge` / `incidents.mark_resolved`). Carries
/// identifier-class plus bounded enum fields only (`incident_id` is the
/// corpus rowid as i64; `kind` / `scope` / `from_state` / `to_state` are
/// bounded enums; `transitioned_at_unix_nano` is integer timestamp).
/// NEVER carries `title` / `detail` / `evidence_refs` payload content
/// (those stay in the corpus BLOB) per security plan §Logging "Never
/// log" discipline + arch §Cross-bridge data shape.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentLifecycleEvent {
    pub incident_id: i64,
    pub kind: CueKind,
    pub scope: CueScope,
    pub from_state: IncidentStatus,
    pub to_state: IncidentStatus,
    pub transitioned_at_unix_nano: i64,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<IncidentLifecycleEvent>`.
/// Mirrors chunk #67 `ServiceLifecycleBroadcast` shape per chunk
/// #62/#63/#67 precedent. Cheap to clone (Arc shares).
#[derive(Debug, Clone)]
pub struct IncidentLifecycleBroadcast {
    sender: broadcast::Sender<IncidentLifecycleEvent>,
}

impl IncidentLifecycleBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<IncidentLifecycleEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<IncidentLifecycleEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for IncidentLifecycleBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event() -> IncidentLifecycleEvent {
        IncidentLifecycleEvent {
            incident_id: 42,
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            from_state: IncidentStatus::Active,
            to_state: IncidentStatus::Resolved,
            transitioned_at_unix_nano: 1_700_000_000_000_000_000,
        }
    }

    #[test]
    fn incident_lifecycle_broadcast_starts_zero_subscribers() {
        let b = IncidentLifecycleBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn incident_lifecycle_broadcast_subscribe_increments_count() {
        let b = IncidentLifecycleBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn incident_lifecycle_broadcast_round_trips_payload() {
        let b = IncidentLifecycleBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample_event();
        b.sender().send(event.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, event);
    }

    #[test]
    fn incident_lifecycle_broadcast_send_with_no_subscribers_is_benign() {
        let b = IncidentLifecycleBroadcast::new();
        let result = b.sender().send(sample_event());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_INCIDENTS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_INCIDENTS, "pulse://stream/incidents");
    }

    #[test]
    fn incident_lifecycle_event_serde_round_trips() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        let parsed: IncidentLifecycleEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, event);
    }

    /// Negative-canary asserting `IncidentLifecycleEvent` payload fields
    /// are exactly the 6 bounded fields documented in the type's docstring.
    /// Guards against future drift that would admit raw OTLP attribute
    /// values, span/log content payloads, workspace paths, or incident
    /// title/detail content. Per security plan §Logging NEVER-log list +
    /// chunk #67 lifecycle precedent.
    #[test]
    fn incident_lifecycle_event_has_no_pii_fields() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        assert!(
            json.contains("\"incident_id\":"),
            "incident_id field must exist"
        );
        assert!(json.contains("\"kind\":"), "kind field must exist");
        assert!(json.contains("\"scope\":"), "scope field must exist");
        assert!(
            json.contains("\"from_state\":"),
            "from_state field must exist"
        );
        assert!(json.contains("\"to_state\":"), "to_state field must exist");
        assert!(
            json.contains("\"transitioned_at_unix_nano\":"),
            "transitioned_at_unix_nano field must exist"
        );
        for banned in [
            "title",
            "detail",
            "evidence_refs",
            "workspace",
            "attribute",
            "attributes",
            "span_id",
            "trace_id",
            "scope_name",
            "instrumentation_scope",
            "body",
            "description",
            "message",
            "exception_message",
            "exception_stacktrace",
            "fingerprint",
        ] {
            assert!(
                !json.contains(banned),
                "PII-bearing field substring `{banned}` MUST NOT appear in IncidentLifecycleEvent payload"
            );
        }
    }

    #[test]
    fn broadcast_capacity_matches_sibling_precedent() {
        assert_eq!(BROADCAST_CAPACITY, 32);
    }
}
