//! L4 degraded-mode FSM concrete impl (chunk #86 — Epoch 9 Foundation v0.2.0).
//!
//! Holds the shared `LocalDegradedModeStatus` state behind a `std::sync::Mutex`;
//! injected into `pulse-app/src/inference_runtime.rs::spawn_l4_inference_subscriber`
//! AND `pulse-app/src/diagnostics_router.rs::DiagnosticsApiImpl` via cloneable
//! `Arc<dyn DegradedModeStatus>` (one shared instance per L4 subscriber per chunk
//! #86 Phase 6 user-confirmed GLOBAL FSM scope).
//!
//! Emits two tracing events on FSM transitions:
//! - `tracing::warn!(target: "interpretation.degraded.enter", ...)` on
//!   Active → Degraded (chunk-named constraint).
//! - `tracing::info!(target: "interpretation.degraded.exit", ...)` on
//!   Degraded → Active (chunk-named constraint).
//!
//! Aggregate-only fields per chunk #86 obs constraint + CLAUDE.md
//! 2026-05-17 session 84 AGGREGATE-ONLY mandate.

use std::sync::Mutex;

use interpretation::contract::InferenceError;
use interpretation::degraded_mode::{
    BACKOFF_CAP_SECS, BACKOFF_PROGRESSION_SECS, BackoffSnapshot, DegradedModeState,
    DegradedModeStatus, FAILURE_THRESHOLD, FAILURE_WINDOW_SECS,
};
use ui_bridge::contract::AppError;

const NANOS_PER_SEC: i64 = 1_000_000_000;

/// Internal FSM state held behind `Mutex`. Separate from `BackoffSnapshot`
/// because the snapshot also exposes a derived `backoff_seconds_remaining`
/// computed against the caller's `now_unix_nano` — internal state stores
/// the absolute `next_retry_at_unix_nano` only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BackoffState {
    state: DegradedModeState,
    /// Count of consecutive failures within the rolling FAILURE_WINDOW_SECS.
    consecutive_failures: u32,
    /// Wall-clock unix-nano of the most recent recorded failure. `None`
    /// before any failures observed; used to compute window-bound resets.
    last_failure_unix_nano: Option<i64>,
    /// Wall-clock unix-nano at which the next L4 invocation is eligible.
    /// `None` in Active state.
    next_retry_at_unix_nano: Option<i64>,
    /// Wall-clock unix-nano at which Active → Degraded transition fired.
    /// `None` in Active state. Used for the exit-event `duration_seconds`
    /// computation.
    degraded_entered_at_unix_nano: Option<i64>,
}

impl BackoffState {
    const fn fresh_active() -> Self {
        Self {
            state: DegradedModeState::Active,
            consecutive_failures: 0,
            last_failure_unix_nano: None,
            next_retry_at_unix_nano: None,
            degraded_entered_at_unix_nano: None,
        }
    }
}

/// Concrete `DegradedModeStatus` impl. Shared via `Arc<dyn DegradedModeStatus>`.
pub struct LocalDegradedModeStatus {
    state: Mutex<BackoffState>,
}

impl LocalDegradedModeStatus {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(BackoffState::fresh_active()),
        }
    }

    fn snapshot_from_state(state: &BackoffState, now_unix_nano: i64) -> BackoffSnapshot {
        let backoff_seconds_remaining = match state.next_retry_at_unix_nano {
            Some(next) if next > now_unix_nano => {
                let remaining_nanos = next - now_unix_nano;
                (remaining_nanos / NANOS_PER_SEC).max(0) as u64
            }
            _ => 0,
        };
        BackoffSnapshot {
            state: state.state,
            consecutive_failures: state.consecutive_failures,
            next_retry_at_unix_nano: state.next_retry_at_unix_nano,
            backoff_seconds_remaining,
        }
    }
}

impl Default for LocalDegradedModeStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl DegradedModeStatus for LocalDegradedModeStatus {
    fn record_failure(&self, now_unix_nano: i64) -> BackoffSnapshot {
        let mut state = self.state.lock().expect("degraded-mode state lock");

        // Counter window reset: if last failure outside FAILURE_WINDOW_SECS,
        // restart the count from 1 (this failure as fresh head).
        let window_nanos = (FAILURE_WINDOW_SECS as i64) * NANOS_PER_SEC;
        let in_window = state
            .last_failure_unix_nano
            .map(|last| now_unix_nano - last <= window_nanos)
            .unwrap_or(false);
        let new_count = if in_window {
            state.consecutive_failures.saturating_add(1)
        } else {
            1
        };
        state.consecutive_failures = new_count;
        state.last_failure_unix_nano = Some(now_unix_nano);

        // Already in degraded? Advance backoff progression on this additional
        // failure.
        if state.state == DegradedModeState::Degraded {
            let progression_idx = additional_failure_index(new_count, FAILURE_THRESHOLD)
                .min(BACKOFF_PROGRESSION_SECS.len().saturating_sub(1));
            let backoff_secs = BACKOFF_PROGRESSION_SECS
                .get(progression_idx)
                .copied()
                .unwrap_or(BACKOFF_CAP_SECS);
            state.next_retry_at_unix_nano =
                Some(now_unix_nano + (backoff_secs as i64) * NANOS_PER_SEC);
            return Self::snapshot_from_state(&state, now_unix_nano);
        }

        // Active state — check threshold for entry.
        if new_count >= FAILURE_THRESHOLD {
            let backoff_secs = BACKOFF_PROGRESSION_SECS
                .first()
                .copied()
                .unwrap_or(BACKOFF_CAP_SECS);
            state.state = DegradedModeState::Degraded;
            state.next_retry_at_unix_nano =
                Some(now_unix_nano + (backoff_secs as i64) * NANOS_PER_SEC);
            state.degraded_entered_at_unix_nano = Some(now_unix_nano);

            tracing::warn!(
                target: "interpretation.degraded.enter",
                consecutive_failures = new_count,
                window_seconds = FAILURE_WINDOW_SECS,
                backoff_seconds = backoff_secs,
                "L4 interpretation entering degraded mode after {} consecutive failures",
                new_count,
            );
        }

        Self::snapshot_from_state(&state, now_unix_nano)
    }

    fn record_success(&self, now_unix_nano: i64) -> BackoffSnapshot {
        let mut state = self.state.lock().expect("degraded-mode state lock");

        // Emit exit event if transitioning out of Degraded.
        if state.state == DegradedModeState::Degraded {
            let duration_secs = state
                .degraded_entered_at_unix_nano
                .map(|entered| ((now_unix_nano - entered).max(0) / NANOS_PER_SEC) as u64)
                .unwrap_or(0);
            tracing::info!(
                target: "interpretation.degraded.exit",
                consecutive_successes = 1u64,
                duration_seconds = duration_secs,
                "L4 interpretation exiting degraded mode after recovery",
            );
        }

        state.state = DegradedModeState::Active;
        state.consecutive_failures = 0;
        state.last_failure_unix_nano = None;
        state.next_retry_at_unix_nano = None;
        state.degraded_entered_at_unix_nano = None;

        Self::snapshot_from_state(&state, now_unix_nano)
    }

    fn current_snapshot(&self, now_unix_nano: i64) -> BackoffSnapshot {
        let state = self.state.lock().expect("degraded-mode state lock");
        Self::snapshot_from_state(&state, now_unix_nano)
    }

    fn is_in_backoff(&self, now_unix_nano: i64) -> bool {
        let state = self.state.lock().expect("degraded-mode state lock");
        if state.state != DegradedModeState::Degraded {
            return false;
        }
        match state.next_retry_at_unix_nano {
            Some(next) => next > now_unix_nano,
            None => false,
        }
    }

    fn trigger_manual_retry(&self, now_unix_nano: i64) -> BackoffSnapshot {
        let mut state = self.state.lock().expect("degraded-mode state lock");

        // Emit exit event if previously in Degraded (manual retry IS a
        // recovery transition for telemetry purposes).
        if state.state == DegradedModeState::Degraded {
            let duration_secs = state
                .degraded_entered_at_unix_nano
                .map(|entered| ((now_unix_nano - entered).max(0) / NANOS_PER_SEC) as u64)
                .unwrap_or(0);
            tracing::info!(
                target: "interpretation.degraded.exit",
                consecutive_successes = 0u64,
                duration_seconds = duration_secs,
                "L4 interpretation manual retry reset (exiting degraded mode)",
            );
        }

        state.state = DegradedModeState::Active;
        state.consecutive_failures = 0;
        state.last_failure_unix_nano = None;
        state.next_retry_at_unix_nano = None;
        state.degraded_entered_at_unix_nano = None;

        Self::snapshot_from_state(&state, now_unix_nano)
    }
}

fn additional_failure_index(consecutive_failures: u32, threshold: u32) -> usize {
    // FAILURE_THRESHOLD-th failure → progression[0]; threshold+1 → progression[1];
    // threshold+2 → progression[2]; subsequent capped to last progression entry.
    (consecutive_failures.saturating_sub(threshold)) as usize
}

/// Sanitized boundary conversion for the L4 interpretation retry path.
/// Per arch §Established Decisions [Error Handling Pattern]: no stack
/// traces, file paths, library versions, Rust struct names, or LLM-emitted
/// `reason` strings escape across the TauRPC bridge. Strips the inner
/// `reason` field carried by some `InferenceError` variants because that
/// field may include sanitized-but-still-implementation-detail content
/// (file paths from runtime errors / library version markers / etc.) that
/// security plan §Anti-Patterns §Logging row 4 generalization bans across
/// the bridge.
///
/// Free function (not `From` impl) for symmetry with
/// `pulse-app/src/diagnostics_router.rs::drain_error_to_app_error` (chunk
/// #69) + the orphan-rule resolution pattern per CLAUDE.md 2026-05-18
/// session-learning (orphan rule blocks `impl From<InferenceError> for
/// AppError` at the binary boundary since neither trait nor types belong
/// to pulse-app).
pub fn interpretation_retry_error_to_app_error(err: InferenceError) -> AppError {
    use InferenceError as I;
    let message = match err {
        I::ModelNotConfigured => "interpretation retry rejected: model not configured",
        I::InvalidModelPath => "interpretation retry rejected: invalid model path",
        I::ModelLoadFailed { .. } => "interpretation retry rejected: model load failed",
        I::TokenizerInitFailed { .. } => "interpretation retry rejected: tokenizer init failed",
        I::InferenceFailed { .. } => "interpretation retry rejected: inference failed",
        I::OutputTooLarge { .. } => "interpretation retry rejected: output too large",
        I::JsonParseFailed { .. } => "interpretation retry rejected: parse failed",
        I::SchemaViolation { .. } => "interpretation retry rejected: schema violation",
    };
    AppError::Internal {
        message: message.to_string(),
    }
}

// Tests live at `pulse-app/tests/unit_degraded_mode_runtime.rs` (integration
// test crate) per CLAUDE.md testing.md 2026-05-20 lesson — pulse-app's
// `[lib] test = false` setting disables source-level `mod tests` blocks
// on Windows due to WebView2 DLL load.
