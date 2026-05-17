use tokio::sync::broadcast;

use crate::contract::AttentionCue;

/// Broadcast topic for `AttentionCue` payloads emitted by the chunk #62
/// tick task. Webview subscription wiring lives in a future v0.2.0 chunk
/// (chunk #80+ per pulse v0.2.0 plan); chunk #62 emit only.
pub const STREAM_NAME_ATTENTION_CUES: &str = "pulse://stream/attention-cues";

/// Internal in-process channel name (label only; no Tauri stream binding).
/// Tier-2 (`PriorityTier::Suggested`) cues fan out here additionally for
/// the future Cadence Coordinator (chunk #72) per route §62 chunk text.
pub const CHANNEL_NAME_CADENCE_TRIGGERS: &str = "cadence-triggers";

/// Broadcast channel capacity — matches `ingest::connection::BROADCAST_CAPACITY`
/// (32). Sized to absorb subscriber lag under typical workloads without
/// dropping events; subscribers that lag beyond this surface as
/// `RecvError::Lagged` per tokio broadcast semantics.
pub const BROADCAST_CAPACITY: usize = 32;

/// Thin wrapper over `tokio::sync::broadcast::Sender<AttentionCue>`. Mirrors
/// `ingest::connection::ConnectionBroadcast` shape per chunk #59 precedent.
#[derive(Debug, Clone)]
pub struct AttentionCueBroadcast {
    sender: broadcast::Sender<AttentionCue>,
}

impl AttentionCueBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<AttentionCue> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AttentionCue> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for AttentionCueBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

/// Cadence-triggers channel — distinct broadcast handle for Tier-2 cue
/// fan-out per route §62 chunk text + chunk #72 Cadence Coordinator
/// dependency. Same wrapper shape as `AttentionCueBroadcast` for caller
/// API symmetry; broadcast (vs mpsc) preserves flexibility for future
/// debug subscribers per chunk #62 plan Open question 5.
#[derive(Debug, Clone)]
pub struct CadenceTriggerChannel {
    sender: broadcast::Sender<AttentionCue>,
}

impl CadenceTriggerChannel {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<AttentionCue> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AttentionCue> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for CadenceTriggerChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueKind, CueScope, PriorityTier};

    fn sample_cue() -> AttentionCue {
        AttentionCue {
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("svc-a".to_string()),
            magnitude: 4.0,
            absolute_value: 0.04,
            persistence_seconds: 30,
            confidence: 0.85,
            priority_tier: PriorityTier::Suggested,
            suppression_bypassed: false,
        }
    }

    #[test]
    fn attention_cue_broadcast_starts_zero_subscribers() {
        let b = AttentionCueBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn attention_cue_broadcast_subscribe_increments_count() {
        let b = AttentionCueBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn cadence_trigger_channel_starts_zero_subscribers() {
        let c = CadenceTriggerChannel::new();
        assert_eq!(c.subscriber_count(), 0);
    }

    #[test]
    fn attention_cue_broadcast_round_trips_payload() {
        let b = AttentionCueBroadcast::new();
        let mut rx = b.subscribe();
        let cue = sample_cue();
        b.sender().send(cue.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, cue);
    }

    #[test]
    fn cadence_trigger_channel_round_trips_payload() {
        let c = CadenceTriggerChannel::new();
        let mut rx = c.subscribe();
        let cue = sample_cue();
        c.sender().send(cue.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, cue);
    }

    #[test]
    fn attention_cue_broadcast_send_with_no_subscribers_is_benign() {
        let b = AttentionCueBroadcast::new();
        let result = b.sender().send(sample_cue());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_ATTENTION_CUES.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_ATTENTION_CUES, "pulse://stream/attention-cues");
    }

    #[test]
    fn channel_name_uses_kebab_case_label() {
        assert_eq!(CHANNEL_NAME_CADENCE_TRIGGERS, "cadence-triggers");
    }
}
