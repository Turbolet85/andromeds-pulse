//! Config-events broadcast topic (`pulse://stream/config-events`).
//!
//! Mirrors `triage::cadence::broadcast` shape: a thin
//! `tokio::sync::broadcast::Sender` wrapper carrying an aggregate-only
//! payload (bounded enum labels + counts + a timestamp — NO config key
//! values, NO file paths, NO secret-shaped strings) per security plan
//! §Logging & Monitoring + obs-plan §8 PII discipline.

use serde::Serialize;
use tokio::sync::broadcast;

/// Tauri IPC broadcast topic for configuration-reload lifecycle events.
pub const STREAM_NAME_CONFIG_EVENTS: &str = "pulse://stream/config-events";

/// Broadcast channel capacity — matches the chunk #62/#80 precedent (32).
pub const BROADCAST_CAPACITY: usize = 32;

/// Bounded kind discriminant for a config-events payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigEventKind {
    /// A settled `config.toml` change validated and applied to the watch
    /// channel (hot keys re-applied; restart-required keys noted).
    Reloaded,
    /// One or more changed keys are restart-required — a notice, not an
    /// applied change.
    RestartRequired,
    /// A settled change failed parse / validation / path checks; the
    /// previous valid config was retained.
    ParseRejected,
}

/// Static snake_case label for a `ConfigEventKind`.
pub fn kind_label(kind: ConfigEventKind) -> &'static str {
    match kind {
        ConfigEventKind::Reloaded => "reloaded",
        ConfigEventKind::RestartRequired => "restart_required",
        ConfigEventKind::ParseRejected => "parse_rejected",
    }
}

/// Aggregate-only config-reload lifecycle event. `Copy` by construction —
/// every field is a scalar, a bounded `&'static str` label, or an enum.
/// No `String` field that could carry a user-edited config value / path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ConfigEvent {
    pub kind: ConfigEventKind,
    pub kind_label: &'static str,
    pub at_unix_nano: i64,
    pub hot_applied_count: u32,
    pub restart_required_count: u32,
    pub silent_count: u32,
    /// Bounded category label for `ParseRejected` (e.g. `"toml_parse"` /
    /// `"validation"` / `"read_failed"` / `"path_traversal"`); `None` for
    /// non-rejection kinds. Bounded `&'static str` — never the raw error.
    pub error_category: Option<&'static str>,
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<ConfigEvent>`.
#[derive(Debug, Clone)]
pub struct ConfigEventBroadcast {
    sender: broadcast::Sender<ConfigEvent>,
}

impl ConfigEventBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<ConfigEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ConfigEvent> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for ConfigEventBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(kind: ConfigEventKind) -> ConfigEvent {
        ConfigEvent {
            kind,
            kind_label: kind_label(kind),
            at_unix_nano: 1_000_000_000,
            hot_applied_count: 2,
            restart_required_count: 1,
            silent_count: 0,
            error_category: None,
        }
    }

    #[test]
    fn stream_name_matches_pulse_uri_convention() {
        assert!(STREAM_NAME_CONFIG_EVENTS.starts_with("pulse://stream/"));
        assert_eq!(STREAM_NAME_CONFIG_EVENTS, "pulse://stream/config-events");
    }

    #[test]
    fn broadcast_round_trips_payload() {
        let b = ConfigEventBroadcast::new();
        let mut rx = b.subscribe();
        let event = sample(ConfigEventKind::Reloaded);
        b.sender().send(event).expect("send ok");
        assert_eq!(rx.try_recv().expect("received"), event);
    }

    #[test]
    fn broadcast_starts_zero_subscribers() {
        assert_eq!(ConfigEventBroadcast::new().subscriber_count(), 0);
    }

    #[test]
    fn config_event_payload_has_no_user_content_string_fields() {
        // PII discipline: the serialized payload carries only enum labels +
        // numeric counters + a timestamp. A user-edited config VALUE that
        // happened to look secret-shaped must never ride this topic.
        const CANARY: &str = "secret-canary-API-key-12345";
        let mut event = sample(ConfigEventKind::ParseRejected);
        // error_category is a bounded &'static str — a user value cannot
        // reach it, but assert the serialized form regardless.
        event.error_category = Some("validation");
        let json = serde_json::to_string(&event).expect("serialize ok");
        assert!(!json.contains(CANARY), "canary leaked: {json}");
        for banned in ["service", "scope_id", "config_value", "path", "config.toml"] {
            assert!(
                !json.contains(banned),
                "ConfigEvent JSON must not contain `{banned}`: {json}"
            );
        }
    }
}
