//! L4 degraded-mode finite state machine + exponential backoff scheduler
//! (chunk #86 — Epoch 9 Foundation v0.2.0; closes capability P-020
//! graceful degradation reaching "full" status).
//!
//! Contract surface only — concrete impl lives at the binary boundary
//! (`pulse-app/src/degraded_mode_runtime.rs`) per arch §Module dependency
//! direction. The `DegradedModeStatus` async trait uses object-safe
//! `Pin<Box<dyn Future + Send + 'a>>` return positions for cross-crate
//! `Arc<dyn DegradedModeStatus>` injection, mirroring chunk #80
//! `SqlQueryRunner` + chunk #82 `LlmInferenceRunner` precedents per
//! `.claude/rules/security.md` 2026-05-23 session-learning.
//!
//! Per chunk #86 user-confirmed Phase 6 resolution, the FSM is GLOBAL
//! (one shared instance per L4 subscriber); per-(kind, scope, workspace)
//! tuple granularity defers to а follow-up chunk if observed-needed.

use serde::{Deserialize, Serialize};

/// Failure-window across which consecutive failures are counted toward the
/// degraded-mode threshold. Per chunk #86 detail spec:
/// "Three consecutive failures within 5 minutes → L4 degraded mode".
pub const FAILURE_WINDOW_SECS: u64 = 300;

/// Consecutive failure count that triggers degraded-mode entry. Per chunk
/// #86 detail spec ("Three consecutive failures").
pub const FAILURE_THRESHOLD: u32 = 3;

/// Exponential backoff progression in seconds. After degraded-mode entry,
/// the Nth additional failure (capped at progression length) sets the
/// next-retry window to this value. Per chunk #86 detail spec:
/// "backoff retry (2→5→10 min capped)".
pub const BACKOFF_PROGRESSION_SECS: &[u64] = &[120, 300, 600];

/// Cap (in seconds) once `BACKOFF_PROGRESSION_SECS` is exhausted. Subsequent
/// failures hold backoff at this value. Equal к the last progression entry.
pub const BACKOFF_CAP_SECS: u64 = 600;

/// Distillation-pipeline degraded-mode state. Active = healthy path; the
/// L4 subscriber generates per cadence tick. Degraded = backoff window
/// active; the L4 subscriber suppresses generation until window expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedModeState {
    Active,
    Degraded,
}

/// Bounded static label for `DegradedModeState` per obs §5 cardinality
/// discipline (mirrors `interpretation::contract::model_tier_label` +
/// `model_status_label` shape).
pub fn degraded_mode_state_label(state: DegradedModeState) -> &'static str {
    match state {
        DegradedModeState::Active => "active",
        DegradedModeState::Degraded => "degraded",
    }
}

/// Snapshot of the degraded-mode FSM at а moment in time. Returned by
/// `DegradedModeStatus` trait methods; consumed by `diagnostics.retry_
/// interpretation()` resolver payload + L4 subscriber backoff-skip logic
/// + obs metric tick task.
///
/// All fields bounded к scalar / enum / Option<scalar>; no LLM-emitted
/// content or per-incident identifiers (per chunk #86 obs constraint
/// "aggregate-only fields").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackoffSnapshot {
    pub state: DegradedModeState,
    /// Count of consecutive parse failures observed within the rolling
    /// FAILURE_WINDOW_SECS. Resets to 0 on `record_success`.
    pub consecutive_failures: u32,
    /// Wall-clock unix-nanosecond timestamp at which the next L4 inference
    /// invocation is eligible. `None` in Active state. Set on entry к
    /// Degraded and on each additional failure (backoff progression).
    pub next_retry_at_unix_nano: Option<i64>,
    /// Remaining seconds until next eligible retry. 0 in Active state OR
    /// when degraded window has expired (caller re-invokes; success or
    /// failure determines next transition).
    pub backoff_seconds_remaining: u64,
}

impl BackoffSnapshot {
    /// Default-Active snapshot used by tests + initial state construction.
    pub fn fresh_active() -> Self {
        Self {
            state: DegradedModeState::Active,
            consecutive_failures: 0,
            next_retry_at_unix_nano: None,
            backoff_seconds_remaining: 0,
        }
    }
}

/// L4 degraded-mode FSM trait. Concrete impl lives at the binary boundary
/// (`pulse-app/src/degraded_mode_runtime.rs`). Methods are SYNCHRONOUS —
/// the FSM state is held in а `std::sync::Mutex` (or equivalent); no
/// I/O involved, so async-trait dyn-compat patterns do not apply here.
///
/// All methods accept `now_unix_nano: i64` as injected time — enables
/// deterministic tests via `tokio::time::pause()` clock advancement OR
/// direct value injection. Mirrors chunk #67 / #78 lifecycle FSM patterns.
pub trait DegradedModeStatus: Send + Sync {
    /// Record an L4 inference failure (JSON parse / schema violation /
    /// output-too-large). Updates the consecutive-failure counter, may
    /// transition Active → Degraded if threshold reached, may extend
    /// the backoff window if already Degraded. Returns the post-update
    /// snapshot.
    fn record_failure(&self, now_unix_nano: i64) -> BackoffSnapshot;

    /// Record an L4 inference success (parse + validate succeeded).
    /// Resets the consecutive-failure counter to 0 + transitions
    /// Degraded → Active immediately if в Degraded state. Returns the
    /// post-update snapshot.
    fn record_success(&self, now_unix_nano: i64) -> BackoffSnapshot;

    /// Read the current FSM snapshot WITHOUT mutating state. Used by
    /// the obs gauge tick task + diagnostics resolver read path +
    /// L4 subscriber backoff-skip decision. The `now_unix_nano`
    /// parameter recomputes `backoff_seconds_remaining` against the
    /// current clock (snapshot's stored value may be stale if degraded
    /// window has elapsed).
    fn current_snapshot(&self, now_unix_nano: i64) -> BackoffSnapshot;

    /// Returns `true` iff the FSM is in Degraded state AND the current
    /// `now_unix_nano` is BEFORE `next_retry_at_unix_nano`. The L4
    /// subscriber consults this BEFORE invoking generation; `true`
    /// indicates the cadence-driven invocation should skip (log а
    /// dropped-digest event + return without subprocess spawn).
    fn is_in_backoff(&self, now_unix_nano: i64) -> bool;

    /// Manual override invoked by the Settings → Diagnostics "Retry
    /// interpretation now" button (via `diagnostics.retry_interpretation()`
    /// TauRPC). Resets the FSM к Active с consecutive_failures=0
    /// regardless of current state. The next L4 inference invocation
    /// proceeds normally; if it fails, the failure counter restarts from 1.
    /// Returns the post-reset snapshot.
    fn trigger_manual_retry(&self, now_unix_nano: i64) -> BackoffSnapshot;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degraded_mode_state_label_is_bounded() {
        assert_eq!(
            degraded_mode_state_label(DegradedModeState::Active),
            "active"
        );
        assert_eq!(
            degraded_mode_state_label(DegradedModeState::Degraded),
            "degraded"
        );
    }

    #[test]
    fn backoff_progression_constants_are_strictly_increasing() {
        // Compile-time invariant via runtime assertion: backoff progression
        // must be monotonically increasing per chunk #86 spec 2→5→10 min.
        for window in BACKOFF_PROGRESSION_SECS.windows(2) {
            assert!(
                window[0] < window[1],
                "BACKOFF_PROGRESSION_SECS must be strictly increasing: {} >= {}",
                window[0],
                window[1]
            );
        }
        assert_eq!(
            *BACKOFF_PROGRESSION_SECS
                .last()
                .expect("non-empty progression"),
            BACKOFF_CAP_SECS,
            "BACKOFF_CAP_SECS must equal last progression entry"
        );
    }

    #[test]
    fn backoff_snapshot_serializes_to_bounded_shape() {
        let snap = BackoffSnapshot::fresh_active();
        let json = serde_json::to_string(&snap).expect("serializes");
        // Bounded fields only — no PII / LLM-content / incident identifiers
        // per chunk #86 obs constraint aggregate-only discipline.
        assert!(json.contains("\"state\""));
        assert!(json.contains("\"consecutive_failures\""));
        assert!(json.contains("\"backoff_seconds_remaining\""));
        assert!(!json.contains("incident_id"));
        assert!(!json.contains("service_name"));
        assert!(!json.contains("model_output"));
    }

    #[test]
    fn fresh_active_snapshot_has_zero_counter_and_no_retry_window() {
        let snap = BackoffSnapshot::fresh_active();
        assert_eq!(snap.state, DegradedModeState::Active);
        assert_eq!(snap.consecutive_failures, 0);
        assert_eq!(snap.next_retry_at_unix_nano, None);
        assert_eq!(snap.backoff_seconds_remaining, 0);
    }
}
