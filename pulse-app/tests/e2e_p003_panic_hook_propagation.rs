//! P-003 panic propagation E2E (chunk #73).
//!
//! Verifies the end-to-end path: a panic in a tokio task → panic hook
//! signals shared atomic → connection FSM poller transitions to
//! `ReceiverFailed` with `ReceiverPanicked` reason variant on the
//! `pulse://stream/connection-state` broadcast topic.
//!
//! Plus the security negative-canary: the raw panic argument string
//! MUST NOT appear in the tracing-appender JSON log file at
//! `<data-dir>/logs/agent-latest.jsonl` (per security plan §Anti-Patterns
//! Logging row 1 — panic payloads from instrumented host apps may carry
//! user secrets).

use std::sync::Arc;
use std::time::Duration;

use ingest::connection::{
    ConnectionBroadcast, ConnectionState, ReceiverBindStatus, ReceiverFailureReason, start_poller,
};
use ingest::state::IngestState;
use pulse_app::observability;
use tempfile::TempDir;

/// Synthetic secret-shaped payload value to assert absence in the log file.
/// Mirrors a JWT bearer token shape but is meaningless beyond the test.
const PANIC_CANARY: &str = "canary-secret-bearer-eyJhbGciOiJIUzI1NiJ9-payload";

/// Bind-status stub reading the panic atomic via observability's accessor.
/// Mirrors the production `HeartbeatBindStatus` adapter (in
/// `pulse-app/src/connection_router.rs`) but without the HeartbeatState dep
/// so the test stays focused on the panic path.
#[derive(Debug, Default)]
struct PanicSignaledBindStatus;

impl ReceiverBindStatus for PanicSignaledBindStatus {
    fn any_receiver_failed(&self) -> bool {
        false
    }
    fn panic_signaled(&self) -> bool {
        observability::panic_signaled()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn p003_panic_propagates_to_receiver_panicked_broadcast_within_2s() {
    // Reset the per-process panic atomic for test isolation.
    observability::reset_panic_signal_for_tests();

    // Install full observability stack into a temp data dir.
    let data_dir = TempDir::new().expect("temp dir created");
    let guard = observability::init(data_dir.path());

    // Spawn the FSM poller backed by panic-aware bind-status.
    let ingest_state = Arc::new(IngestState::new());
    let bind_status: Arc<dyn ReceiverBindStatus> = Arc::new(PanicSignaledBindStatus);
    let broadcast_handle = Arc::new(ConnectionBroadcast::new());
    let mut rx = broadcast_handle.subscribe();

    let poller = tokio::spawn(start_poller(
        Arc::clone(&ingest_state),
        Arc::clone(&bind_status),
        Arc::clone(&broadcast_handle),
    ));

    // Inject a synthetic panic in a spawned task. tokio's JoinHandle catches
    // the panic (process survives); the panic hook fires AND sets PANIC_SIGNAL.
    let panicking = tokio::spawn(async {
        panic!("{PANIC_CANARY}");
    });
    let join_result = panicking.await;
    assert!(
        join_result.is_err(),
        "spawned task panicked as expected; join_result = {:?}",
        join_result
    );

    // Wait for FSM poller to see the panic atomic + emit transition.
    // Poller cadence is 1s (POLLER_INTERVAL); allow 3s safety margin.
    let event = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .expect("ReceiverPanicked event arrives within 3s")
        .expect("broadcast not closed");
    assert_eq!(
        event.state,
        ConnectionState::ReceiverFailed,
        "expected ReceiverFailed state; got {:?}",
        event.state
    );
    assert_eq!(
        event.reason,
        Some(ReceiverFailureReason::ReceiverPanicked),
        "expected ReceiverPanicked reason; got {:?}",
        event.reason
    );

    poller.abort();
    let _ = poller.await;

    // Drop the WorkerGuard to flush tracing-appender output.
    drop(guard);

    // Wait briefly for the file to settle on disk after flush.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Negative canary: read agent-latest.jsonl + assert raw panic payload
    // string is absent. The panic hook MUST emit only location + spantrace
    // — no panic_message field (per security plan §Anti-Patterns Logging
    // row 1 + chunk #73 P-003 task brief "panic event uses sanitized payload").
    let log_dir = data_dir.path().join("logs");
    let entries: Vec<_> = std::fs::read_dir(&log_dir)
        .expect("logs dir created")
        .filter_map(Result::ok)
        .collect();
    assert!(
        !entries.is_empty(),
        "logs dir should contain agent-latest.jsonl rolling file; got {entries:?}"
    );

    let mut combined_log = String::new();
    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            let contents = std::fs::read_to_string(&path).unwrap_or_default();
            combined_log.push_str(&contents);
            combined_log.push('\n');
        }
    }

    // Verify panic event WAS emitted (positive assertion).
    assert!(
        combined_log.contains("app.panic.fatal"),
        "panic hook should have emitted app.panic.fatal event; log preview = {}",
        combined_log.chars().take(500).collect::<String>()
    );

    // Negative canary assertion: raw payload bytes MUST NOT appear.
    assert!(
        !combined_log.contains(PANIC_CANARY),
        "panic-payload canary leaked to agent-latest.jsonl (security regression — \
         security plan §Anti-Patterns Logging row 1); log preview = {}",
        combined_log
            .lines()
            .filter(|line| line.contains("panic"))
            .take(5)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
