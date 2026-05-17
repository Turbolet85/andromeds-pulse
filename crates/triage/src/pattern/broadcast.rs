use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Broadcast topic for `RestartEvent` payloads emitted by the chunk #63
/// `RestartDetector` tick task. Webview subscription wiring lives in a
/// future v0.2.0 chunk (chunk #80+ per pulse v0.2.0 plan); chunk #63
/// emit-only.
pub const STREAM_NAME_RESTART_EVENTS: &str = "pulse://stream/restart-events";

/// Broadcast channel capacity — mirrors `cue::BROADCAST_CAPACITY` (32).
/// Sized to absorb subscriber lag under typical workloads without
/// dropping events; subscribers that lag beyond this surface as
/// `RecvError::Lagged` per tokio broadcast semantics.
pub const BROADCAST_CAPACITY: usize = 32;

/// Algorithmic restart event emitted by `RestartDetector` per capability
/// spec P-015. Identifier-class fields only — `service` is the OTLP
/// service.name; timestamps are unix-nanos integers; gap_seconds is
/// derived. NEVER carries raw OTLP attribute values, span content
/// payloads, or `instrumentation_scope.name` strings per security plan
/// §Logging "Never log" discipline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartEvent {
    pub service: String,
    pub gap_seconds: u64,
    pub last_seen_unix_nano: i64,
    pub resume_unix_nano: i64,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<RestartEvent>`.
/// Mirrors `cue::AttentionCueBroadcast` shape per chunk #62 precedent.
#[derive(Debug, Clone)]
pub struct RestartEventBroadcast {
    sender: broadcast::Sender<RestartEvent>,
}

impl RestartEventBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<RestartEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RestartEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for RestartEventBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event() -> RestartEvent {
        RestartEvent {
            service: "svc-a".to_string(),
            gap_seconds: 25,
            last_seen_unix_nano: 1_700_000_000_000_000_000,
            resume_unix_nano: 1_700_000_025_000_000_000,
        }
    }

    #[test]
    fn restart_event_broadcast_starts_zero_subscribers() {
        let b = RestartEventBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn restart_event_broadcast_subscribe_increments_count() {
        let b = RestartEventBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn restart_event_broadcast_round_trips_payload() {
        let b = RestartEventBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample_event();
        b.sender().send(event.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, event);
    }

    #[test]
    fn restart_event_broadcast_send_with_no_subscribers_is_benign() {
        let b = RestartEventBroadcast::new();
        let result = b.sender().send(sample_event());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_RESTART_EVENTS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_RESTART_EVENTS, "pulse://stream/restart-events");
    }

    #[test]
    fn restart_event_payload_serde_round_trips_through_serde() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        let parsed: RestartEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, event);
    }

    /// Negative-canary asserting `RestartEvent` payload fields are exactly
    /// the 4 identifier-class fields — `service` / `gap_seconds` /
    /// `last_seen_unix_nano` / `resume_unix_nano`. Guards against future
    /// drift that would admit raw OTLP attribute values, span/log content
    /// payloads, or instrumentation_scope strings. Per security plan
    /// §Logging NEVER-log list + chunk #63 plan §Acceptance security line 1.
    #[test]
    fn restart_event_payload_has_no_pii_fields() {
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize");
        // Field set: assert presence of allowed identifier-class field names
        // + absence of disallowed PII-bearing field substrings.
        assert!(json.contains("\"service\":"), "service field must exist");
        assert!(
            json.contains("\"gap_seconds\":"),
            "gap_seconds field must exist"
        );
        assert!(
            json.contains("\"last_seen_unix_nano\":"),
            "last_seen_unix_nano field must exist"
        );
        assert!(
            json.contains("\"resume_unix_nano\":"),
            "resume_unix_nano field must exist"
        );
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
        ] {
            assert!(
                !json.contains(banned),
                "PII-bearing field substring `{banned}` MUST NOT appear in RestartEvent payload"
            );
        }
    }

    #[test]
    fn broadcast_capacity_matches_cue_precedent() {
        // Mirrors `crate::cue::BROADCAST_CAPACITY` chunk #62 precedent.
        assert_eq!(BROADCAST_CAPACITY, 32);
    }
}
