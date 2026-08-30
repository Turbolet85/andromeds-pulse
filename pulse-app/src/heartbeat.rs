use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::{BroadcastSenders, BufferState, DrainMiner};
use chrono::Utc;
use ingest::channel::IngestSender;
use ingest::connection::{
    ConnectionState, ReceiverBindStatus, compute_state, last_span_ago_ms, severity_label,
    state_label,
};
use ingest::state::IngestState;
use plugins::loader::PluginRegistry;
use tokio::task::JoinHandle;
use ui_bridge::health::HeartbeatState;
use viz::VizState;

const TICK_INTERVAL_SECS: u64 = 15;

/// Transition-scoped target announcing that the buffer consumer stopped
/// draining. Needs its OWN exact allowlist leaf: `for_target` strips `.tick`
/// then falls back to the first `.`-segment, so this would otherwise resolve to
/// the `buffer` field set and every field below would be silently redacted.
#[doc(hidden)]
pub const TARGET_CONSUMER_STALLED: &str = "buffer.consumer.stalled";

/// Consecutive non-draining ticks before the stall is announced. 30 × 15s =
/// 450s, deliberately clear of the bounds obs-plan §10 declares in-spec: a
/// single append can block >60s under DuckDB row-group maintenance, and the
/// sustained-drain profile is allowed 420s. A shorter window would fire on
/// healthy load.
#[doc(hidden)]
pub const STALL_CONSECUTIVE_TICKS: u32 = 30;

const _: () = {
    assert!(STALL_CONSECUTIVE_TICKS as u64 * TICK_INTERVAL_SECS > 420);
};

/// Consumer drain progress for one heartbeat tick.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrainProgress {
    /// Rows landed since the previous tick.
    Advancing,
    /// No rows landed and the ingest channel holds nothing — the producer is
    /// idle, which is not a defect.
    ProducerIdle,
    /// No rows landed while the ingest channel still holds queued work: the
    /// consumer is not taking what the receiver accepted.
    NotDraining,
}

/// The discriminator half B exists for. `rows_ingested` alone cannot separate a
/// stalled consumer from an idle producer — both leave it static. The ingest
/// channel's occupancy is what separates them: an idle producer drains to
/// empty, a stalled consumer leaves work queued.
#[doc(hidden)]
pub fn classify_drain_progress(rows_delta: u64, buffer_capacity_pct: f64) -> DrainProgress {
    if rows_delta > 0 {
        DrainProgress::Advancing
    } else if buffer_capacity_pct > 0.0 {
        DrainProgress::NotDraining
    } else {
        DrainProgress::ProducerIdle
    }
}

#[doc(hidden)]
pub fn last_append_age_seconds(last_append_at_nanos: u64, now_nanos: u64) -> u64 {
    if last_append_at_nanos == 0 {
        return 0;
    }
    now_nanos.saturating_sub(last_append_at_nanos) / 1_000_000_000
}

fn now_unix_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    state: Arc<HeartbeatState>,
    ingest_state: Arc<IngestState>,
    ingest_sender: Arc<IngestSender>,
    buffer_state: Arc<BufferState>,
    retention_seconds: u64,
    viz_state: Arc<VizState>,
    broadcast_senders: Arc<BroadcastSenders>,
    plugins_registry: Arc<Mutex<PluginRegistry>>,
    bind_status: Arc<dyn ReceiverBindStatus>,
    // Chunk #69 Phase B Session 7+: when Some, the buffer.tick heartbeat
    // surfaces drain_template_count + drain_lru_evictions_since_tick fields
    // AND emits a metric.pipeline.l1c.drain_template_count_total event per
    // tick. When None, both heartbeat fields default to 0 + the metric
    // event is suppressed (matches pre-Drain-enabled boot mode).
    drain_miner: Option<Arc<DrainMiner>>,
) -> Vec<JoinHandle<()>> {
    vec![
        tokio::spawn(run_ingest(
            state.clone(),
            Arc::clone(&ingest_state),
            Arc::clone(&ingest_sender),
            broadcast_senders,
        )),
        tokio::spawn(run_buffer(
            state.clone(),
            buffer_state,
            ingest_sender,
            retention_seconds,
            drain_miner,
        )),
        tokio::spawn(run_viz(state.clone(), viz_state)),
        tokio::spawn(run_plugins(state.clone(), plugins_registry)),
        tokio::spawn(run_connection(state, ingest_state, bind_status)),
    ]
}

async fn run_ingest(
    state: Arc<HeartbeatState>,
    ingest_state: Arc<IngestState>,
    ingest_sender: Arc<IngestSender>,
    broadcast_senders: Arc<BroadcastSenders>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_ingest_tick(&state, &ingest_state, &ingest_sender, &broadcast_senders);
    }
}

async fn run_buffer(
    state: Arc<HeartbeatState>,
    buffer_state: Arc<BufferState>,
    ingest_sender: Arc<IngestSender>,
    retention_seconds: u64,
    drain_miner: Option<Arc<DrainMiner>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    let last_eviction = AtomicU64::new(0);
    let last_rows = AtomicU64::new(0);
    let mut stalled_ticks: u32 = 0;
    let mut stall_announced = false;
    loop {
        interval.tick().await;
        let progress = emit_buffer_tick(
            &state,
            &buffer_state,
            &ingest_sender,
            retention_seconds,
            &last_eviction,
            &last_rows,
            drain_miner.as_deref(),
        );

        stalled_ticks = match progress {
            DrainProgress::NotDraining => stalled_ticks.saturating_add(1),
            _ => 0,
        };

        if stalled_ticks >= STALL_CONSECUTIVE_TICKS {
            if !stall_announced {
                stall_announced = true;
                tracing::warn!(
                    target: TARGET_CONSUMER_STALLED,
                    reason = "rows_static_while_channel_queued",
                    consequence = "accepted_spans_not_persisted",
                    stalled_seconds = stalled_ticks as u64 * TICK_INTERVAL_SECS,
                    buffer_capacity_pct = ingest_sender.capacity_pct(),
                    "buffer consumer is not draining; accepted spans are not reaching DuckDB",
                );
            }
        } else if stall_announced && matches!(progress, DrainProgress::Advancing) {
            stall_announced = false;
            tracing::warn!(
                target: TARGET_CONSUMER_STALLED,
                reason = "recovered",
                consequence = "drain_resumed",
                stalled_seconds = 0_u64,
                buffer_capacity_pct = ingest_sender.capacity_pct(),
                "buffer consumer resumed draining",
            );
        }
    }
}

async fn run_viz(state: Arc<HeartbeatState>, viz_state: Arc<VizState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_viz_tick(&state, &viz_state);
    }
}

async fn run_plugins(state: Arc<HeartbeatState>, plugins_registry: Arc<Mutex<PluginRegistry>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_plugins_tick(&state, &plugins_registry);
    }
}

// Chunk #59 — connection state heartbeat. 15s sibling cadence (matches
// ingest.tick / buffer.tick / viz.tick / plugins.tick) emitting a periodic
// snapshot of current connection-state. The 1-2s FSM detector loop in
// crates/ingest/src/connection.rs::start_poller emits transition events
// ONLY on state changes — that's a separate concern, NOT this heartbeat.
async fn run_connection(
    _state: Arc<HeartbeatState>,
    ingest_state: Arc<IngestState>,
    bind_status: Arc<dyn ReceiverBindStatus>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_connection_tick(&ingest_state, bind_status.as_ref());
    }
}

#[doc(hidden)]
pub fn emit_ingest_tick(
    state: &HeartbeatState,
    ingest_state: &IngestState,
    ingest_sender: &IngestSender,
    broadcast_senders: &BroadcastSenders,
) {
    let total_subs = broadcast_senders.spans.receiver_count()
        + broadcast_senders.metrics.receiver_count()
        + broadcast_senders.logs.receiver_count();
    ingest_state.set_broadcast_subscribers(total_subs as u32);

    let payload = ingest::contract::heartbeat_payload(ingest_state, ingest_sender);
    state.record_ingest(Utc::now());
    tracing::info!(
        target: "ingest.tick",
        span_count = payload.span_count,
        buffer_capacity_pct = payload.buffer_capacity_pct,
        broadcast_subscribers = payload.broadcast_subscribers,
        "heartbeat",
    );

    // Per-stream gauge events (chunk #23) — channel_name label enumerated to the
    // three reserved stream names per arch §Occupied Resources Tauri IPC events.
    emit_broadcast_gauge(
        buffer::STREAM_NAME_SPANS,
        broadcast_senders.spans.receiver_count(),
    );
    emit_broadcast_gauge(
        buffer::STREAM_NAME_METRICS,
        broadcast_senders.metrics.receiver_count(),
    );
    emit_broadcast_gauge(
        buffer::STREAM_NAME_LOGS,
        broadcast_senders.logs.receiver_count(),
    );
}

fn emit_broadcast_gauge(channel_name: &'static str, value: usize) {
    tracing::info!(
        target: "metric.ingest.channel.broadcast_subscribers",
        value = value as u64,
        channel_name = channel_name,
        "broadcast subscriber gauge",
    );
}

#[doc(hidden)]
pub fn emit_buffer_tick(
    state: &HeartbeatState,
    buffer_state: &BufferState,
    ingest_sender: &IngestSender,
    retention_seconds: u64,
    last_eviction: &AtomicU64,
    last_rows: &AtomicU64,
    drain_miner: Option<&DrainMiner>,
) -> DrainProgress {
    let snap = buffer_state.snapshot();
    let prev = last_eviction.swap(snap.eviction_count, Ordering::Relaxed);
    let delta = snap.eviction_count.saturating_sub(prev);
    let rows_active = snap.rows_ingested.saturating_sub(snap.eviction_count);

    let prev_rows = last_rows.swap(snap.rows_ingested, Ordering::Relaxed);
    let rows_ingested_delta = snap.rows_ingested.saturating_sub(prev_rows);
    let buffer_capacity_pct = ingest_sender.capacity_pct();
    let last_append_age = last_append_age_seconds(snap.last_append_at_nanos, now_unix_nanos());

    // Chunk #69 Phase B Session 7+: pull Drain heartbeat counters (zero
    // when no miner threaded in). `take_lru_evictions_since_tick` resets
    // the counter to 0 after read — matches the `eviction_count_since_last_tick`
    // sibling semantics (delta-since-last-tick, not running total).
    let drain_template_count = drain_miner.map(|m| m.template_count()).unwrap_or(0);
    let drain_lru_evictions_since_tick = drain_miner
        .map(|m| m.take_lru_evictions_since_tick())
        .unwrap_or(0);

    let payload = buffer::contract::heartbeat_payload(
        buffer_state,
        retention_seconds,
        delta,
        drain_template_count,
        drain_lru_evictions_since_tick,
    );
    state.record_buffer(Utc::now());

    tracing::info!(
        target: "buffer.tick",
        rows_ingested = payload.rows_ingested,
        retention_window_active = payload.retention_window_active,
        eviction_count = payload.eviction_count,
        memory_bytes = payload.memory_bytes,
        retention_window_seconds = payload.retention_window_seconds,
        eviction_count_since_last_tick = payload.eviction_count_since_last_tick,
        drain_template_count = payload.drain_template_count,
        drain_lru_evictions_since_tick = payload.drain_lru_evictions_since_tick,
        span_events_seen = payload.span_events_seen,
        fingerprints_computed = payload.fingerprints_computed,
        observer_invocations = payload.observer_invocations,
        redactions_applied = payload.redactions_applied,
        append_rejections = payload.append_rejections,
        rows_ingested_delta = rows_ingested_delta,
        last_append_age_seconds = last_append_age,
        "heartbeat",
    );

    tracing::info!(
        target: "metric.buffer.memory_bytes",
        value = payload.memory_bytes,
        retention_window_seconds = retention_seconds,
        rows_active = rows_active,
        eviction_count_since_last_tick = delta,
        "buffer memory gauge",
    );

    if delta > 0 {
        tracing::info!(
            target: "metric.buffer.evicted_span_count",
            value = delta,
            retention_window_seconds = retention_seconds,
            "buffer eviction counter",
        );
    }

    // Chunk #69 Phase B Session 7+ (Step 4 follow-up): emit the
    // `metric.pipeline.l1c.drain_template_count_total` event per tick
    // when a Drain miner is threaded in. Aggregate-only field discipline
    // per AllowList registration at observability.rs:1385 (`value` only).
    // Emitted unconditionally when miner present — even at zero count
    // (presence-of-tick semantics; CI heartbeat-gap-check tolerates
    // value=0 as a healthy signal).
    if drain_miner.is_some() {
        tracing::info!(
            target: "metric.pipeline.l1c.drain_template_count_total",
            value = drain_template_count,
            "drain template count",
        );
    }

    classify_drain_progress(rows_ingested_delta, buffer_capacity_pct)
}

#[doc(hidden)]
pub fn emit_viz_tick(state: &HeartbeatState, viz_state: &VizState) {
    let payload = viz::contract::heartbeat_payload(viz_state);
    state.record_viz(Utc::now());
    tracing::info!(
        target: "viz.tick",
        query_latency_ms = payload.query_latency_ms,
        subscribers_active = payload.subscribers_active,
        "heartbeat",
    );
}

#[doc(hidden)]
pub fn emit_plugins_tick(state: &HeartbeatState, plugins_registry: &Arc<Mutex<PluginRegistry>>) {
    // Brief lock; payload computation is cheap (size + atomic load).
    // If the lock is poisoned, fall back to an empty payload so the tick
    // still emits — heartbeat-stall CI gate cares about presence-of-tick
    // more than payload accuracy on a poisoned mutex.
    let payload = match plugins_registry.lock() {
        Ok(registry) => plugins::contract::heartbeat_payload(&registry),
        Err(_) => Default::default(),
    };
    state.record_plugins(Utc::now());
    tracing::info!(
        target: "plugins.tick",
        loaded_count = payload.loaded_count,
        active_invocations = payload.active_invocations,
        "heartbeat",
    );
}

#[doc(hidden)]
pub fn emit_connection_tick(ingest_state: &IngestState, bind_status: &dyn ReceiverBindStatus) {
    let now_nanos = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let last = ingest_state.last_ingest_at_nanos();
    let failed = bind_status.any_receiver_failed();
    let panicked = bind_status.panic_signaled();
    let state = compute_state(last, now_nanos, failed, panicked);
    let lag = last_span_ago_ms(last, now_nanos);
    let severity = severity_for_state(state);
    tracing::info!(
        target: "connection.tick",
        state = state_label(state),
        last_span_ago_ms = lag,
        severity = severity_label(severity),
        "heartbeat",
    );
}

fn severity_for_state(state: ConnectionState) -> ingest::connection::Severity {
    match state {
        ConnectionState::Listening | ConnectionState::Receiving | ConnectionState::Idle => {
            ingest::connection::Severity::Info
        }
        ConnectionState::Stalled => ingest::connection::Severity::Warning,
        ConnectionState::ReceiverFailed => ingest::connection::Severity::Critical,
    }
}

// Tests migrated to `pulse-app/tests/unit_heartbeat_ticks.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
