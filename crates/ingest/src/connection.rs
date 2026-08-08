//! Connection state machine — chunk #59.
//!
//! Orthogonal "connection health" domain (capability P-004 per pulse v0.2.0
//! plan) decoupled from existing throughput / error-rate visualization.
//!
//! Components: 5-state FSM (`ConnectionState`) + wire payload + broadcast
//! channel + 1-2s poller. The poller reads `IngestState::last_ingest_at_nanos`
//! and a `ReceiverBindStatus` trait (so this crate stays workspace-boundary-
//! clean — `HeartbeatState` lives in ui-bridge and implements the trait at
//! the binary level). Transition events broadcast on state change only,
//! never per-tick (per a11y plan §11 Screen Reader anti-pattern).
//!
//! Heartbeat-style ticks at 15s sibling cadence live in
//! `pulse-app/src/heartbeat.rs::run_connection` — separate concern from this
//! file's 1-2s FSM detector loop (per obs-plan §3 cadence convention).

use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::state::IngestState;

pub const STREAM_NAME_CONNECTION_STATE: &str = "pulse://stream/connection-state";

/// Broadcast channel capacity. Connection-state transitions are infrequent
/// (only on state change, not per tick); 32 slots absorbs reasonable
/// subscriber lag without dropping events.
const BROADCAST_CAPACITY: usize = 32;

/// FSM detector poller cadence. Sub-15s relative to sibling heartbeat ticks
/// (per `pulse-app/src/heartbeat.rs` 15s convention) so state changes are
/// detected promptly; broadcast emits only on transitions so the 1s cadence
/// does NOT regress log volume.
const POLLER_INTERVAL: Duration = Duration::from_secs(1);

/// Age (nanos) after which ingest is considered "Idle". 10 seconds per
/// capability spec P-001 ("Idle 10–60s, quiet but otherwise healthy").
pub const IDLE_THRESHOLD_NANOS: i64 = 10_000_000_000;

/// Age (nanos) after which ingest is considered "Stalled". 60 seconds per
/// capability spec P-001 ("Stalled >60s, suspect"). Independent of the 45s
/// heartbeat-gap CI alarm (obs-plan §10) — the two are orthogonal mechanisms
/// per 2026-05-08 obs Decisions Log "heartbeat ticks vs health command
/// semantics": ticks measure subsystem self-emission cadence, FSM measures
/// downstream OTLP span ingestion.
pub const STALLED_THRESHOLD_NANOS: i64 = 60_000_000_000;

/// 5-state connection lifecycle FSM. `#[serde(tag = "state")]` discriminator
/// gives the TauRPC TypeScript binding a discriminated union the compiler
/// enforces exhaustive `switch` on — preserves a11y SC 1.4.1 not-color-alone
/// affordance at the wire boundary (per a11y plan §6 + plan.md §A11y row).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "state")]
pub enum ConnectionState {
    Listening,
    Receiving,
    Idle,
    Stalled,
    ReceiverFailed,
}

/// Severity hint enabling future webview `aria-live` polite-vs-assertive
/// selection without re-deriving severity from state-enum string matching
/// (per a11y plan §7 Live regions table).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// Closed-enum failure reason. Carries NO file-path / line-number /
/// library-version / Rust-struct-name content — satisfies security plan
/// §Anti-Patterns §Logging row 1 + a11y plan §8 plain-language commitment.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverFailureReason {
    BindFailed,
    StaleHeartbeat,
    ReceiverPanicked,
}

impl ReceiverFailureReason {
    pub fn label(&self) -> &'static str {
        match self {
            Self::BindFailed => "receiver bind failed",
            Self::StaleHeartbeat => "receiver task stalled",
            Self::ReceiverPanicked => "receiver task panicked",
        }
    }
}

/// Wire payload for the `pulse://stream/connection-state` broadcast topic
/// AND the `connection.current_state` TauRPC procedure response. Carries
/// only semantic state + lag + sanitized reason — NO presentation fields
/// (`displayColor` / `iconGlyph` / `cssClass` are forbidden per design
/// plan extract).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatePayload {
    pub state: ConnectionState,
    pub last_span_ago_ms: u64,
    pub severity: Severity,
    pub message: Option<String>,
    pub reason: Option<ReceiverFailureReason>,
}

impl ConnectionStatePayload {
    pub fn from_parts(
        state: ConnectionState,
        last_span_ago_ms: u64,
        reason: Option<ReceiverFailureReason>,
    ) -> Self {
        let severity = severity_for(state);
        let message = message_for(state, last_span_ago_ms, reason);
        Self {
            state,
            last_span_ago_ms,
            severity,
            message,
            reason,
        }
    }
}

fn severity_for(state: ConnectionState) -> Severity {
    match state {
        ConnectionState::Listening | ConnectionState::Receiving | ConnectionState::Idle => {
            Severity::Info
        }
        ConnectionState::Stalled => Severity::Warning,
        ConnectionState::ReceiverFailed => Severity::Critical,
    }
}

fn message_for(
    state: ConnectionState,
    last_span_ago_ms: u64,
    reason: Option<ReceiverFailureReason>,
) -> Option<String> {
    match state {
        ConnectionState::Stalled => {
            let seconds = last_span_ago_ms / 1000;
            Some(format!("no spans received in {seconds}s"))
        }
        ConnectionState::ReceiverFailed => Some(match reason {
            Some(r) => format!("OTLP receiver unavailable: {}", r.label()),
            None => "OTLP receiver unavailable".to_string(),
        }),
        _ => None,
    }
}

/// Receiver bind status surface — abstraction so the FSM poller in
/// `crates/ingest` can read pulse-app's `HeartbeatState` bind flags without
/// ingest depending on ui-bridge (per arch §Cross-cutting Patterns Module
/// dependency direction).
pub trait ReceiverBindStatus: Send + Sync {
    /// Returns true if either gRPC or HTTP OTLP receiver bind has reported
    /// a failure.
    fn any_receiver_failed(&self) -> bool;
    /// Returns true if a panic has been signaled by the application's
    /// `std::panic::set_hook` (chunk #73 P-003). Default impl returns false
    /// for backward compatibility; pulse-app's `HeartbeatBindStatus` adapter
    /// overrides this to read a module-level atomic signaled by the panic
    /// hook. When true, the FSM transitions to `ReceiverFailed` with reason
    /// `ReceiverPanicked` per capability spec P-003 "panics in receiver tasks".
    fn panic_signaled(&self) -> bool {
        false
    }
}

/// Pure FSM transition function. Edges:
/// - bind failure OR panic signaled → ReceiverFailed (overrides all other signals)
/// - no ingest yet (last == 0) → Listening
/// - age < IDLE_THRESHOLD → Receiving
/// - IDLE_THRESHOLD ≤ age < STALLED_THRESHOLD → Idle
/// - age ≥ STALLED_THRESHOLD → Stalled
pub fn compute_state(
    last_ingest_at_nanos: i64,
    now_nanos: i64,
    any_receiver_failed: bool,
    receiver_panicked: bool,
) -> ConnectionState {
    if any_receiver_failed || receiver_panicked {
        return ConnectionState::ReceiverFailed;
    }
    if last_ingest_at_nanos == 0 {
        return ConnectionState::Listening;
    }
    let age = now_nanos.saturating_sub(last_ingest_at_nanos);
    if age < IDLE_THRESHOLD_NANOS {
        ConnectionState::Receiving
    } else if age < STALLED_THRESHOLD_NANOS {
        ConnectionState::Idle
    } else {
        ConnectionState::Stalled
    }
}

/// Computes the last-span-ago lag in milliseconds. Returns 0 when no
/// ingest has happened yet.
pub fn last_span_ago_ms(last_ingest_at_nanos: i64, now_nanos: i64) -> u64 {
    if last_ingest_at_nanos == 0 {
        return 0;
    }
    let age_nanos = now_nanos.saturating_sub(last_ingest_at_nanos).max(0);
    (age_nanos / 1_000_000) as u64
}

/// Thin wrapper over `tokio::sync::broadcast::Sender<ConnectionStatePayload>`.
/// Mirrors the `pulse-app/src/streams.rs` broadcast usage pattern.
#[derive(Debug, Clone)]
pub struct ConnectionBroadcast {
    sender: broadcast::Sender<ConnectionStatePayload>,
}

impl ConnectionBroadcast {
    pub fn new() -> Self {
        let (sender, _initial_receiver) = broadcast::channel(BROADCAST_CAPACITY);
        Self { sender }
    }

    pub fn sender(&self) -> &broadcast::Sender<ConnectionStatePayload> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ConnectionStatePayload> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for ConnectionBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection-state poller. 1-second tick reading the atomic + bind status,
/// computing FSM state, broadcasting payload ONLY on state changes (never
/// per-tick). Emits structured `tracing` event at each transition.
pub async fn start_poller(
    ingest_state: Arc<IngestState>,
    bind_status: Arc<dyn ReceiverBindStatus>,
    broadcast_handle: Arc<ConnectionBroadcast>,
) {
    let mut interval = tokio::time::interval(POLLER_INTERVAL);
    let mut current = ConnectionState::Listening;
    loop {
        interval.tick().await;
        let now_nanos = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let last = ingest_state.last_ingest_at_nanos();
        let failed = bind_status.any_receiver_failed();
        let panicked = bind_status.panic_signaled();
        let next = compute_state(last, now_nanos, failed, panicked);
        if next != current {
            let lag = last_span_ago_ms(last, now_nanos);
            let reason = derive_reason(next, failed, panicked);
            let payload = ConnectionStatePayload::from_parts(next, lag, reason);
            emit_transition(current, next, lag, reason);
            // Best-effort send: ignore SendError (no subscribers is benign;
            // the broadcast is still useful for in-process consumers).
            let _ = broadcast_handle.sender().send(payload);
            current = next;
        }
    }
}

fn derive_reason(
    state: ConnectionState,
    any_receiver_failed: bool,
    receiver_panicked: bool,
) -> Option<ReceiverFailureReason> {
    if matches!(state, ConnectionState::ReceiverFailed) {
        // Precedence: panic > bind failure > stale heartbeat. Per capability
        // spec P-003 panic is the most-actionable failure category and
        // surfaces first when both conditions apply.
        Some(if receiver_panicked {
            ReceiverFailureReason::ReceiverPanicked
        } else if any_receiver_failed {
            ReceiverFailureReason::BindFailed
        } else {
            ReceiverFailureReason::StaleHeartbeat
        })
    } else {
        None
    }
}

fn emit_transition(
    from_state: ConnectionState,
    to_state: ConnectionState,
    last_span_ago_ms: u64,
    reason: Option<ReceiverFailureReason>,
) {
    // Per obs-plan §6 Log levels: boundary state transitions are the must-log
    // category. ReceiverFailed-via-panic transitions log at `error`; all
    // other transitions at `info`.
    let from_label = state_label(from_state);
    let to_label = state_label(to_state);
    let reason_label = reason.map(|r| r.label()).unwrap_or("");
    let severity_lbl = severity_label(severity_for(to_state));
    if matches!(to_state, ConnectionState::ReceiverFailed) {
        tracing::error!(
            target: "connection.state.transition",
            from_state = from_label,
            to_state = to_label,
            last_span_ago_ms,
            trigger_reason = reason_label,
            severity = severity_lbl,
            "connection state transition",
        );
    } else {
        tracing::info!(
            target: "connection.state.transition",
            from_state = from_label,
            to_state = to_label,
            last_span_ago_ms,
            trigger_reason = reason_label,
            severity = severity_lbl,
            "connection state transition",
        );
    }
}

pub fn state_label(state: ConnectionState) -> &'static str {
    match state {
        ConnectionState::Listening => "Listening",
        ConnectionState::Receiving => "Receiving",
        ConnectionState::Idle => "Idle",
        ConnectionState::Stalled => "Stalled",
        ConnectionState::ReceiverFailed => "ReceiverFailed",
    }
}

pub fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Critical => "critical",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    // ---- Test helper: bind-status stub ----

    #[derive(Debug, Default)]
    struct StubBindStatus {
        failed: AtomicBool,
    }

    impl StubBindStatus {
        fn new(failed: bool) -> Self {
            Self {
                failed: AtomicBool::new(failed),
            }
        }

        fn set_failed(&self, value: bool) {
            self.failed.store(value, Ordering::Relaxed);
        }
    }

    impl ReceiverBindStatus for StubBindStatus {
        fn any_receiver_failed(&self) -> bool {
            self.failed.load(Ordering::Relaxed)
        }
    }

    // ---- compute_state matrix ----

    #[test]
    fn compute_state_no_ingest_returns_listening() {
        assert_eq!(
            compute_state(0, 1_000_000_000_000, false, false),
            ConnectionState::Listening
        );
    }

    #[test]
    fn compute_state_recent_ingest_returns_receiving() {
        let now = 100_000_000_000;
        let last = now - 1_000_000_000; // 1s ago < 5s threshold
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Receiving
        );
    }

    #[test]
    fn compute_state_age_at_idle_threshold_boundary_returns_idle() {
        let now = 100_000_000_000;
        let last = now - IDLE_THRESHOLD_NANOS; // exactly at threshold
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Idle
        );
    }

    #[test]
    fn compute_state_age_between_idle_and_stalled_returns_idle() {
        let now = 100_000_000_000;
        let last = now - 10_000_000_000; // 10s ago = IDLE boundary, < STALLED (60s)
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Idle
        );
    }

    #[test]
    fn compute_state_age_at_stalled_threshold_returns_stalled() {
        let now = 100_000_000_000;
        let last = now - STALLED_THRESHOLD_NANOS; // exactly at threshold
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Stalled
        );
    }

    #[test]
    fn compute_state_age_past_stalled_returns_stalled() {
        let now = 100_000_000_000;
        let last = now - 120_000_000_000; // 120s ago, well past STALLED (60s)
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Stalled
        );
    }

    #[test]
    fn compute_state_bind_failed_overrides_all_other_signals() {
        let now = 100_000_000_000;
        // Recent ingest BUT bind failed: must transition to ReceiverFailed
        let last = now - 100_000_000;
        assert_eq!(
            compute_state(last, now, true, false),
            ConnectionState::ReceiverFailed
        );
        // No ingest + bind failed → still ReceiverFailed (not Listening)
        assert_eq!(
            compute_state(0, now, true, false),
            ConnectionState::ReceiverFailed
        );
    }

    #[test]
    fn compute_state_negative_age_treated_as_recent() {
        // Clock-skew defense: future-dated last_ingest reads as 0 age (Receiving).
        let now = 100_000_000_000;
        let last = now + 1_000_000_000; // last in the future
        assert_eq!(
            compute_state(last, now, false, false),
            ConnectionState::Receiving
        );
    }

    // ---- last_span_ago_ms ----

    #[test]
    fn last_span_ago_ms_zero_when_no_ingest() {
        assert_eq!(last_span_ago_ms(0, 100_000_000_000), 0);
    }

    #[test]
    fn last_span_ago_ms_computes_millis() {
        let now = 100_000_000_000;
        let last = now - 7_500_000_000; // 7.5 seconds ago
        assert_eq!(last_span_ago_ms(last, now), 7500);
    }

    #[test]
    fn last_span_ago_ms_clamps_negative_to_zero() {
        let now = 100_000_000_000;
        let last = now + 5_000_000_000; // future
        assert_eq!(last_span_ago_ms(last, now), 0);
    }

    // ---- severity_for ----

    #[test]
    fn severity_for_maps_each_state() {
        assert_eq!(severity_for(ConnectionState::Listening), Severity::Info);
        assert_eq!(severity_for(ConnectionState::Receiving), Severity::Info);
        assert_eq!(severity_for(ConnectionState::Idle), Severity::Info);
        assert_eq!(severity_for(ConnectionState::Stalled), Severity::Warning);
        assert_eq!(
            severity_for(ConnectionState::ReceiverFailed),
            Severity::Critical
        );
    }

    // ---- message_for ----

    #[test]
    fn message_for_listening_receiving_idle_returns_none() {
        assert!(message_for(ConnectionState::Listening, 0, None).is_none());
        assert!(message_for(ConnectionState::Receiving, 100, None).is_none());
        assert!(message_for(ConnectionState::Idle, 6000, None).is_none());
    }

    #[test]
    fn message_for_stalled_describes_lag_in_seconds() {
        let msg = message_for(ConnectionState::Stalled, 45_000, None).unwrap();
        assert_eq!(msg, "no spans received in 45s");
    }

    #[test]
    fn message_for_receiver_failed_includes_reason_label() {
        let msg = message_for(
            ConnectionState::ReceiverFailed,
            0,
            Some(ReceiverFailureReason::BindFailed),
        )
        .unwrap();
        assert_eq!(msg, "OTLP receiver unavailable: receiver bind failed");
    }

    #[test]
    fn message_for_receiver_failed_without_reason_is_generic() {
        let msg = message_for(ConnectionState::ReceiverFailed, 0, None).unwrap();
        assert_eq!(msg, "OTLP receiver unavailable");
    }

    // ---- sanitization regression: messages contain no path / line markers ----

    #[test]
    fn message_for_stalled_contains_no_path_or_line_markers() {
        let msg = message_for(ConnectionState::Stalled, 30_000, None).unwrap();
        assert!(
            !msg.contains('/'),
            "message must not contain path separator"
        );
        assert!(
            !msg.contains('\\'),
            "message must not contain windows separator"
        );
        // Line:col heuristic — no ascii digit immediately after ':'.
        for (idx, ch) in msg.char_indices() {
            if ch == ':' {
                if let Some(next) = msg[idx + 1..].chars().next() {
                    assert!(
                        !next.is_ascii_digit(),
                        "stalled message contains line:column pattern at idx={idx}: {msg}"
                    );
                }
            }
        }
    }

    #[test]
    fn message_for_receiver_failed_contains_no_path_or_line_markers() {
        for reason in [
            ReceiverFailureReason::BindFailed,
            ReceiverFailureReason::StaleHeartbeat,
            ReceiverFailureReason::ReceiverPanicked,
        ] {
            let msg = message_for(ConnectionState::ReceiverFailed, 0, Some(reason)).unwrap();
            assert!(!msg.contains('/'), "{reason:?} message has path separator");
            assert!(
                !msg.contains('\\'),
                "{reason:?} message has windows separator"
            );
            // No "src/foo.rs:42:9" pattern: require no ascii digit immediately after ':'.
            for (idx, ch) in msg.char_indices() {
                if ch == ':' {
                    let next = msg[idx + 1..].chars().next();
                    if let Some(c) = next {
                        assert!(
                            !c.is_ascii_digit(),
                            "{reason:?} message contains line:column pattern at idx={idx}: {msg}"
                        );
                    }
                }
            }
        }
    }

    // ---- payload serialization includes state discriminator ----

    #[test]
    fn payload_serialization_includes_state_tag() {
        let payload = ConnectionStatePayload::from_parts(
            ConnectionState::ReceiverFailed,
            0,
            Some(ReceiverFailureReason::BindFailed),
        );
        let json = serde_json::to_string(&payload).expect("serializes");
        assert!(
            json.contains("\"state\":\"ReceiverFailed\""),
            "payload JSON missing state discriminator tag: {json}"
        );
    }

    #[test]
    fn payload_serialization_includes_severity_and_reason_lowercase() {
        let payload = ConnectionStatePayload::from_parts(
            ConnectionState::ReceiverFailed,
            0,
            Some(ReceiverFailureReason::BindFailed),
        );
        let json = serde_json::to_string(&payload).expect("serializes");
        assert!(json.contains("\"severity\":\"critical\""));
        assert!(json.contains("\"reason\":\"bind_failed\""));
    }

    #[test]
    fn payload_serialization_omits_presentation_fields() {
        let payload = ConnectionStatePayload::from_parts(ConnectionState::Receiving, 100, None);
        let json = serde_json::to_string(&payload).expect("serializes");
        // Design extract: NEVER include presentation hints in wire payload.
        assert!(!json.contains("displayColor"));
        assert!(!json.contains("iconGlyph"));
        assert!(!json.contains("cssClass"));
    }

    // ---- state_label / severity_label ----

    #[test]
    fn state_label_returns_pascal_case_for_each_variant() {
        assert_eq!(state_label(ConnectionState::Listening), "Listening");
        assert_eq!(state_label(ConnectionState::Receiving), "Receiving");
        assert_eq!(state_label(ConnectionState::Idle), "Idle");
        assert_eq!(state_label(ConnectionState::Stalled), "Stalled");
        assert_eq!(
            state_label(ConnectionState::ReceiverFailed),
            "ReceiverFailed"
        );
    }

    #[test]
    fn severity_label_returns_lowercase() {
        assert_eq!(severity_label(Severity::Info), "info");
        assert_eq!(severity_label(Severity::Warning), "warning");
        assert_eq!(severity_label(Severity::Critical), "critical");
    }

    // ---- ConnectionBroadcast ----

    #[test]
    fn broadcast_new_starts_with_zero_subscribers() {
        let b = ConnectionBroadcast::new();
        assert_eq!(b.subscriber_count(), 0);
    }

    #[test]
    fn broadcast_subscribe_increments_subscriber_count() {
        let b = ConnectionBroadcast::new();
        let _r = b.subscribe();
        assert_eq!(b.subscriber_count(), 1);
    }

    // ---- start_poller integration tests ----

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn poller_transitions_listening_to_receiving_on_first_ingest() {
        let ingest = Arc::new(IngestState::new());
        let bind: Arc<dyn ReceiverBindStatus> = Arc::new(StubBindStatus::new(false));
        let broadcast_handle = Arc::new(ConnectionBroadcast::new());
        let mut sub = broadcast_handle.subscribe();

        let poller = tokio::spawn(start_poller(
            Arc::clone(&ingest),
            Arc::clone(&bind),
            Arc::clone(&broadcast_handle),
        ));

        // Tick once (interval initial tick fires immediately) — initial state
        // matches `current = Listening`, no transition, no broadcast.
        tokio::time::advance(Duration::from_millis(10)).await;

        // Simulate ingest, then advance past one tick.
        ingest.record_spans(1);
        tokio::time::advance(POLLER_INTERVAL + Duration::from_millis(50)).await;

        // Expect a single transition event Listening → Receiving.
        let event = tokio::time::timeout(Duration::from_secs(5), sub.recv())
            .await
            .expect("transition event arrives within timeout")
            .expect("broadcast not closed");
        assert_eq!(event.state, ConnectionState::Receiving);
        assert_eq!(event.severity, Severity::Info);
        assert!(event.message.is_none());

        poller.abort();
        let _ = poller.await;
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn poller_emits_only_on_state_change_no_per_tick_events() {
        let ingest = Arc::new(IngestState::new());
        let bind: Arc<dyn ReceiverBindStatus> = Arc::new(StubBindStatus::new(false));
        let broadcast_handle = Arc::new(ConnectionBroadcast::new());
        let mut sub = broadcast_handle.subscribe();

        let poller = tokio::spawn(start_poller(
            Arc::clone(&ingest),
            Arc::clone(&bind),
            Arc::clone(&broadcast_handle),
        ));

        // No ingest, state stays Listening. Advance through 5 poller ticks.
        for _ in 0..5 {
            tokio::time::advance(POLLER_INTERVAL + Duration::from_millis(50)).await;
        }

        // Subscriber received no events.
        let recv = tokio::time::timeout(Duration::from_millis(100), sub.recv()).await;
        assert!(
            recv.is_err(),
            "broadcast emitted on tick despite no state change: {recv:?}"
        );

        poller.abort();
        let _ = poller.await;
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn poller_transitions_to_receiver_failed_on_bind_failure() {
        let ingest = Arc::new(IngestState::new());
        let stub = Arc::new(StubBindStatus::new(false));
        let bind: Arc<dyn ReceiverBindStatus> = stub.clone();
        let broadcast_handle = Arc::new(ConnectionBroadcast::new());
        let mut sub = broadcast_handle.subscribe();

        let poller = tokio::spawn(start_poller(
            Arc::clone(&ingest),
            Arc::clone(&bind),
            Arc::clone(&broadcast_handle),
        ));

        // First tick: no transition (Listening → Listening).
        tokio::time::advance(POLLER_INTERVAL + Duration::from_millis(50)).await;

        // Flip bind status to failed.
        stub.set_failed(true);
        tokio::time::advance(POLLER_INTERVAL + Duration::from_millis(50)).await;

        let event = tokio::time::timeout(Duration::from_secs(5), sub.recv())
            .await
            .expect("transition event arrives within timeout")
            .expect("broadcast not closed");
        assert_eq!(event.state, ConnectionState::ReceiverFailed);
        assert_eq!(event.severity, Severity::Critical);
        assert_eq!(event.reason, Some(ReceiverFailureReason::BindFailed));
        assert!(event.message.is_some());

        poller.abort();
        let _ = poller.await;
    }
}
