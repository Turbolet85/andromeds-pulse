use serde::Serialize;
use tokio::sync::broadcast;

use crate::cadence::coordinator::CadenceMode;
use crate::contract::AttentionCue;

/// Tauri IPC broadcast topic identifier for L6 visibility events emitted
/// by the cadence coordinator. Subscribers (webview / mcp / future tools)
/// see one event per cycle (per tier tick OR per Tier-1/2 trigger).
pub const STREAM_NAME_CADENCE_EVENTS: &str = "pulse://stream/cadence-events";

/// Broadcast channel capacity — matches `cue::broadcast::BROADCAST_CAPACITY`
/// (32) per chunk #62 precedent. Sized to absorb subscriber lag under
/// typical workloads without dropping events.
pub const BROADCAST_CAPACITY: usize = 32;

/// L6-visibility payload describing one cadence cycle invocation. Fields
/// carry only opaque coordinator-internal identifiers + bounded enum
/// labels + structural counts — NO `scope_id` / `service` / raw OTLP
/// attribute strings / L3 digest body content / L4 inference payloads
/// per security plan §Logging & Monitoring "What NEVER to log".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CadenceEvent {
    pub mode: CadenceMode,
    pub mode_label: &'static str,
    pub scheduled_at_unix_nano: i64,
    pub executed_at_unix_nano: i64,
    pub cue_kind_label: Option<&'static str>,
    pub cue_priority_label: Option<&'static str>,
    pub queries_executed: u32,
    pub queries_succeeded: u32,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<CadenceEvent>`. Mirrors
/// `cue::broadcast::AttentionCueBroadcast` shape per chunk #62 precedent.
#[derive(Debug, Clone)]
pub struct CadenceEventBroadcast {
    sender: broadcast::Sender<CadenceEvent>,
}

impl CadenceEventBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<CadenceEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CadenceEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for CadenceEventBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal (NON-L6) digest-assembly trigger. Distinct from [`CadenceEvent`]
/// (the PII-free `pulse://stream/cadence-events` L6 topic): this carrier
/// conveys the full triggering [`AttentionCue`] — including `scope_id` — to the
/// L3 digest assembler so the digest -> L4 -> incident chain can attribute and
/// dedup the incident. Deliberately NOT `Serialize`, never emitted on any
/// user-visible / self-observation surface; the legitimate surfaces for
/// `scope_id` are the digest + incident, not the cadence-events topic.
#[derive(Debug, Clone)]
pub struct DigestTrigger {
    pub mode: CadenceMode,
    pub executed_at_unix_nano: i64,
    pub triggering_cue: Option<AttentionCue>,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<DigestTrigger>`. Mirrors
/// [`CadenceEventBroadcast`]; the digest-assembler adapter
/// (`pulse-app/src/digest_runtime.rs`) is the sole subscriber.
#[derive(Debug, Clone)]
pub struct DigestTriggerBroadcast {
    sender: broadcast::Sender<DigestTrigger>,
}

impl DigestTriggerBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<DigestTrigger> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DigestTrigger> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for DigestTriggerBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueKind, CueScope, PriorityTier};

    fn sample_event() -> CadenceEvent {
        CadenceEvent {
            mode: CadenceMode::Tier3,
            mode_label: "tier3",
            scheduled_at_unix_nano: 1_000_000_000,
            executed_at_unix_nano: 1_000_000_500,
            cue_kind_label: None,
            cue_priority_label: None,
            queries_executed: 7,
            queries_succeeded: 7,
        }
    }

    #[test]
    fn cadence_event_broadcast_starts_zero_subscribers() {
        let b = CadenceEventBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn cadence_event_broadcast_subscribe_increments_count() {
        let b = CadenceEventBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn cadence_event_broadcast_round_trips_payload() {
        let b = CadenceEventBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample_event();
        b.sender().send(event).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, event);
    }

    #[test]
    fn cadence_event_broadcast_send_with_no_subscribers_is_benign() {
        let b = CadenceEventBroadcast::new();
        let result = b.sender().send(sample_event());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_CADENCE_EVENTS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_CADENCE_EVENTS, "pulse://stream/cadence-events");
    }

    #[test]
    fn cadence_event_payload_has_no_string_fields() {
        // PII discipline: payload carries only opaque enum labels (static
        // strings) + numeric counters + timestamps. NO `String` field that
        // could carry user-controlled service.name / scope_id / OTLP attr.
        // Static negative canary: serialize a sample event + verify no
        // recognizable user-content shape.
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize ok");
        // Verify all expected fields present
        assert!(json.contains("\"mode\""));
        assert!(json.contains("\"mode_label\""));
        assert!(json.contains("\"queries_executed\""));
        // Verify NO PII-shaped field names
        for banned in [
            "service",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "digest_body",
            "inference_input",
            "inference_output",
        ] {
            assert!(
                !json.contains(banned),
                "CadenceEvent serialized JSON must not contain `{banned}` field: {json}"
            );
        }
    }

    #[test]
    fn cadence_event_broadcast_default_matches_new() {
        let b = CadenceEventBroadcast::default();
        assert_eq!(b.subscriber_count(), 0);
    }

    fn sample_storm_cue() -> AttentionCue {
        AttentionCue {
            kind: CueKind::RetryStorm,
            scope: CueScope::Service,
            scope_id: Some("payment-service".to_string()),
            magnitude: 5.0,
            absolute_value: 50.0,
            persistence_seconds: 30,
            confidence: 1.0,
            priority_tier: PriorityTier::Autonomous,
            suppression_bypassed: false,
        }
    }

    #[test]
    fn digest_trigger_broadcast_round_trips_cue_including_scope_id() {
        let b = DigestTriggerBroadcast::new();
        let mut rx = b.subscribe();
        b.sender()
            .send(DigestTrigger {
                mode: CadenceMode::Tier1,
                executed_at_unix_nano: 1_700_000_000_000,
                triggering_cue: Some(sample_storm_cue()),
            })
            .expect("send ok");
        let received = rx.try_recv().expect("trigger received");
        assert_eq!(received.mode, CadenceMode::Tier1);
        let cue = received
            .triggering_cue
            .expect("internal carrier conveys the cue");
        assert_eq!(cue.scope_id.as_deref(), Some("payment-service"));
        assert_eq!(cue.kind, CueKind::RetryStorm);
    }

    #[test]
    fn digest_trigger_broadcast_default_starts_zero_subscribers() {
        assert_eq!(DigestTriggerBroadcast::default().subscriber_count(), 0);
    }
}
