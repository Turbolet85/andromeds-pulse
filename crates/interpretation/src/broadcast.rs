//! Broadcast topic + payload wrapper for the `pulse://stream/model-status`
//! channel. Mirrors `crates/triage/src/cadence/broadcast.rs` shape per
//! chunk #80 precedent — broadcast definition lives with the contract crate;
//! the binary boundary (`pulse-app/`) constructs + injects the
//! broadcaster into the concrete `MistralRsInference` impl.

use tokio::sync::broadcast;

use crate::contract::ModelLoadEvent;

/// Tauri IPC broadcast topic identifier for L4 model lifecycle events.
/// Subscribers (webview Settings → Diagnostics → Model surface in chunk
/// #95) see one event per load/unload/error transition.
pub const STREAM_NAME_MODEL_STATUS: &str = "pulse://stream/model-status";

/// Broadcast channel capacity — matches `cadence::broadcast::BROADCAST_CAPACITY`
/// (32) per chunk #80 precedent. Sized to absorb subscriber lag under
/// typical lifecycle event rates without dropping events.
pub const BROADCAST_CAPACITY: usize = 32;

/// Thin wrapper over `tokio::sync::broadcast::Sender<ModelLoadEvent>`.
/// Mirrors `cadence::broadcast::CadenceEventBroadcast` shape per chunk #80
/// precedent.
#[derive(Debug, Clone)]
pub struct ModelStatusBroadcast {
    sender: broadcast::Sender<ModelLoadEvent>,
}

impl ModelStatusBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<ModelLoadEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ModelLoadEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for ModelStatusBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{ModelStatus, ModelTier};
    use chrono::Utc;

    fn sample_event() -> ModelLoadEvent {
        ModelLoadEvent {
            timestamp: Utc::now(),
            model_identity: None,
            profile_label: "cpu_primary".into(),
            tier: ModelTier::Primary,
            status: ModelStatus::Loading,
            error_message: None,
        }
    }

    #[test]
    fn model_status_broadcast_starts_zero_subscribers() {
        let b = ModelStatusBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn model_status_broadcast_subscribe_increments_count() {
        let b = ModelStatusBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    #[test]
    fn model_status_broadcast_round_trips_payload() {
        let b = ModelStatusBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample_event();
        b.sender().send(event.clone()).expect("send ok");
        let received = rx.try_recv().expect("payload received");
        assert_eq!(received, event);
    }

    #[test]
    fn model_status_broadcast_send_with_no_subscribers_is_benign() {
        let b = ModelStatusBroadcast::new();
        let result = b.sender().send(sample_event());
        assert!(result.is_err(), "send without subscribers returns Err");
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_MODEL_STATUS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_MODEL_STATUS, "pulse://stream/model-status");
    }

    #[test]
    fn model_status_broadcast_default_matches_new() {
        let b = ModelStatusBroadcast::default();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn model_status_broadcast_payload_has_no_path_fields() {
        // PII discipline: payload carries only bounded label + identity
        // (semantic name only) + timestamps. NO file path / checkpoint URL
        // / mistralrs internal type leak per security extract.
        let event = sample_event();
        let json = serde_json::to_string(&event).expect("serialize ok");
        for banned in [
            "model_path",
            "checkpoint_url",
            "file_path",
            "checkpoint_path",
            "weights_path",
        ] {
            assert!(
                !json.contains(banned),
                "ModelLoadEvent serialized JSON must not contain `{banned}` field: {json}"
            );
        }
    }
}
