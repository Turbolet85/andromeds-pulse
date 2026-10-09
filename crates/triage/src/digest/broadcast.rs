//! `DigestBroadcast` — Tauri IPC broadcast channel for L3 → L4 digest
//! delivery (chunk #81). Mirrors chunk #80 `CadenceEventBroadcast` shape.
//!
//! Payload: full `crate::contract::Digest` (PRE-SCRUBBED via
//! `Digest::scrubbed_clone` at the producer side BEFORE send). Subscribers
//! observe `pulse://stream/digests` events for downstream LLM inference
//! consumption (chunks #82-#85, deferred pending Pre-D1 LLM runtime).

use tokio::sync::broadcast;

use crate::contract::Digest;

/// Tauri IPC broadcast topic identifier for L3 digest events emitted by
/// the digest assembler.
pub const STREAM_NAME_DIGESTS: &str = "pulse://stream/digests";

/// Broadcast channel capacity — matches `cue::broadcast::BROADCAST_CAPACITY`
/// (32) + chunks #62/#63/#67/#78/#80 precedents. Sized to absorb subscriber
/// lag under typical workloads without dropping events.
pub const BROADCAST_CAPACITY: usize = 32;

/// Thin wrapper over `tokio::sync::broadcast::Sender<Digest>`. Mirrors
/// chunk #80 `cadence::broadcast::CadenceEventBroadcast` shape.
#[derive(Debug, Clone)]
pub struct DigestBroadcast {
    sender: broadcast::Sender<Digest>,
}

impl DigestBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<Digest> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Digest> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for DigestBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{DigestKind, DigestLwwMode};

    fn sample_digest() -> Digest {
        Digest {
            kind: DigestKind::CadenceTier3,
            token_count: 1024,
            payload_summary: "WINDOW: ... PROJECT: ... SERVICES: ...".to_string(),
            incident_refs: vec![],
            generated_at_unix_nano: 1_700_000_000_000,
            workspace: "/home/dev/example".to_string(),
            window_start_unix_nano: 1_700_000_000_000,
            window_end_unix_nano: 1_700_000_060_000,
            services: vec![],
            attention_cues: vec![],
            corpus_matches: vec![],
            lww_mode: DigestLwwMode::Default,
            active_incident_bypass: false,
            resolution_event: false,
        }
    }

    #[test]
    fn digest_broadcast_starts_zero_subscribers() {
        let b = DigestBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn digest_broadcast_subscribe_increments_count() {
        let b = DigestBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn digest_broadcast_round_trips_payload() {
        let b = DigestBroadcast::new();
        let mut rx = b.subscribe();
        let digest = sample_digest();
        b.sender().send(digest.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, digest);
    }

    #[test]
    fn digest_broadcast_send_with_no_subscribers_is_benign() {
        let b = DigestBroadcast::new();
        let result = b.sender().send(sample_digest());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_DIGESTS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_DIGESTS, "pulse://stream/digests");
    }

    #[test]
    fn digest_broadcast_default_matches_new() {
        let b = DigestBroadcast::default();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn multiple_subscribers_each_receive_send() {
        let b = DigestBroadcast::new();
        let mut rx1 = b.subscribe();
        let mut rx2 = b.subscribe();
        assert_eq!(b.subscriber_count(), 2);
        let digest = sample_digest();
        b.sender().send(digest.clone()).expect("send");
        assert_eq!(rx1.try_recv().expect("rx1"), digest);
        assert_eq!(rx2.try_recv().expect("rx2"), digest);
    }
}
