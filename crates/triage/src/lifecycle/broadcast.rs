//! Broadcast topic for `ServiceLifecycleEvent` payloads emitted by the
//! chunk #67 lifecycle tick task. Mirrors chunk #63
//! `pattern::broadcast::RestartEventBroadcast` shape exactly per chunk
//! #62/#63 precedent. Webview subscription wiring lives in a future
//! v0.2.0 chunk (chunk #80+ per pulse v0.2.0 plan); chunk #67 emit-only.

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use super::state_machine::{ServiceLifecycleState, TransitionTrigger};

/// Broadcast topic name per arch §Occupied Resources `pulse://stream/{kebab}`
/// URI convention. Post-merge, this requires
/// `/andromeda-evolve --allow-arch-registry` to legitimize in §Occupied
/// Resources Tauri IPC events (chunks #59/#62/#63 precedent).
pub const STREAM_NAME_SERVICE_LIFECYCLE: &str = "pulse://stream/service-lifecycle";

/// Broadcast channel capacity — mirrors chunk #62 `cue::BROADCAST_CAPACITY`
/// and chunk #63 `pattern::BROADCAST_CAPACITY` (32). Sized to absorb
/// subscriber lag under typical workloads without dropping events.
pub const BROADCAST_CAPACITY: usize = 32;

/// Lifecycle state transition event emitted by the registry tick loop
/// (chunk #67 `start_lifecycle_heartbeat`). Identifier-class fields only —
/// `service` is the OTLP service.name; `from_state` / `to_state` are
/// bounded enum tags; `transitioned_at_unix_nano` is an integer timestamp;
/// `trigger` is a bounded enum. NEVER carries raw OTLP attribute values,
/// span content payloads, or `instrumentation_scope.name` strings per
/// security plan §Logging "Never log" discipline.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceLifecycleEvent {
    pub service: String,
    pub from_state: ServiceLifecycleState,
    pub to_state: ServiceLifecycleState,
    pub transitioned_at_unix_nano: i64,
    pub trigger: TransitionTrigger,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<ServiceLifecycleEvent>`.
/// Mirrors chunk #63 `RestartEventBroadcast` shape per chunk #62/#63
/// precedent. Cheap to clone (Arc shares).
#[derive(Debug, Clone)]
pub struct ServiceLifecycleBroadcast {
    sender: broadcast::Sender<ServiceLifecycleEvent>,
}

impl ServiceLifecycleBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<ServiceLifecycleEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServiceLifecycleEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for ServiceLifecycleBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event() -> ServiceLifecycleEvent {
        ServiceLifecycleEvent {
            service: "svc-a".to_string(),
            from_state: ServiceLifecycleState::Bootstrapping,
            to_state: ServiceLifecycleState::Active,
            transitioned_at_unix_nano: 1_700_000_000_000_000_000,
            trigger: TransitionTrigger::Activity,
        }
    }

    #[test]
    fn service_lifecycle_broadcast_starts_zero_subscribers() {
        let b = ServiceLifecycleBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn service_lifecycle_broadcast_subscribe_increments_count() {
        let b = ServiceLifecycleBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn service_lifecycle_broadcast_round_trips_payload() {
        let b = ServiceLifecycleBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample_event();
        b.sender().send(event.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, event);
    }

    #[test]
    fn service_lifecycle_broadcast_send_with_no_subscribers_is_benign() {
        let b = ServiceLifecycleBroadcast::new();
        let result = b.sender().send(sample_event());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_SERVICE_LIFECYCLE.starts_with("pulse://stream/"));
        assert_eq!(
            STREAM_NAME_SERVICE_LIFECYCLE,
            "pulse://stream/service-lifecycle"
        );
    }

    #[test]
    fn service_lifecycle_event_serde_round_trips() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        let parsed: ServiceLifecycleEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, event);
    }

    /// Negative-canary asserting `ServiceLifecycleEvent` payload fields are
    /// exactly the 5 identifier-class fields documented in the type's docstring.
    /// Guards against future drift that would admit raw OTLP attribute
    /// values, span/log content payloads, or instrumentation_scope strings.
    /// Per security plan §Logging NEVER-log list + chunk #67 plan §Step 5.
    #[test]
    fn service_lifecycle_event_has_no_pii_fields() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        assert!(json.contains("\"service\":"), "service field must exist");
        assert!(
            json.contains("\"from_state\":"),
            "from_state field must exist"
        );
        assert!(json.contains("\"to_state\":"), "to_state field must exist");
        assert!(
            json.contains("\"transitioned_at_unix_nano\":"),
            "transitioned_at_unix_nano field must exist"
        );
        assert!(json.contains("\"trigger\":"), "trigger field must exist");
        for banned in [
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
        ] {
            assert!(
                !json.contains(banned),
                "PII-bearing field substring `{banned}` MUST NOT appear in ServiceLifecycleEvent payload"
            );
        }
    }

    #[test]
    fn broadcast_capacity_matches_sibling_precedent() {
        assert_eq!(BROADCAST_CAPACITY, 32);
    }
}
