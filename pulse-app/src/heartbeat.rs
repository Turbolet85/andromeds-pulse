use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

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
            ingest_sender,
            broadcast_senders,
        )),
        tokio::spawn(run_buffer(
            state.clone(),
            buffer_state,
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
    retention_seconds: u64,
    drain_miner: Option<Arc<DrainMiner>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    let last_eviction = AtomicU64::new(0);
    loop {
        interval.tick().await;
        emit_buffer_tick(
            &state,
            &buffer_state,
            retention_seconds,
            &last_eviction,
            drain_miner.as_deref(),
        );
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

fn emit_ingest_tick(
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

fn emit_buffer_tick(
    state: &HeartbeatState,
    buffer_state: &BufferState,
    retention_seconds: u64,
    last_eviction: &AtomicU64,
    drain_miner: Option<&DrainMiner>,
) {
    let snap = buffer_state.snapshot();
    let prev = last_eviction.swap(snap.eviction_count, Ordering::Relaxed);
    let delta = snap.eviction_count.saturating_sub(prev);
    let rows_active = snap.rows_ingested.saturating_sub(snap.eviction_count);

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
}

fn emit_viz_tick(state: &HeartbeatState, viz_state: &VizState) {
    let payload = viz::contract::heartbeat_payload(viz_state);
    state.record_viz(Utc::now());
    tracing::info!(
        target: "viz.tick",
        query_latency_ms = payload.query_latency_ms,
        subscribers_active = payload.subscribers_active,
        "heartbeat",
    );
}

fn emit_plugins_tick(state: &HeartbeatState, plugins_registry: &Arc<Mutex<PluginRegistry>>) {
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

fn emit_connection_tick(ingest_state: &IngestState, bind_status: &dyn ReceiverBindStatus) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ingest::channel::build_channel;
    use std::io::Write;
    use std::sync::Mutex;
    use tracing_subscriber::Registry;
    use tracing_subscriber::layer::SubscriberExt;

    #[derive(Clone)]
    struct VecMakeWriter(Arc<Mutex<Vec<u8>>>);

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for VecMakeWriter {
        type Writer = VecWriter;
        fn make_writer(&'a self) -> Self::Writer {
            VecWriter(Arc::clone(&self.0))
        }
    }

    struct VecWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn capture_lines<F: FnOnce()>(f: F) -> Vec<serde_json::Value> {
        let buf = Arc::new(Mutex::new(Vec::<u8>::new()));
        let writer = VecMakeWriter(Arc::clone(&buf));
        let layer = tracing_subscriber::fmt::layer().json().with_writer(writer);
        let subscriber = Registry::default().with(layer);
        tracing::subscriber::with_default(subscriber, f);
        let bytes = buf.lock().unwrap().clone();
        let s = String::from_utf8(bytes).expect("captured bytes should be valid UTF-8");
        s.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| {
                serde_json::from_str::<serde_json::Value>(l).expect("each line parses as JSON")
            })
            .collect()
    }

    #[test]
    fn emit_ingest_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let ingest_state = IngestState::new();
        let (sender, _rx) = build_channel();
        let senders = buffer::broadcast::create();
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender, &senders));
        assert!(!lines.is_empty());
        assert_eq!(lines[0]["target"], "ingest.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("span_count").is_some());
        assert!(fields.get("buffer_capacity_pct").is_some());
        assert!(fields.get("broadcast_subscribers").is_some());
        assert!(state.last_ingest().is_some());
    }

    #[test]
    fn emit_ingest_tick_reflects_real_span_count_from_ingest_state() {
        let state = HeartbeatState::new();
        let ingest_state = IngestState::new();
        let (sender, _rx) = build_channel();
        let senders = buffer::broadcast::create();
        ingest_state.record_spans(7);
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender, &senders));
        assert_eq!(lines[0]["fields"]["span_count"], 7);
    }

    #[test]
    fn emit_ingest_tick_reflects_real_broadcast_subscriber_sum() {
        let state = HeartbeatState::new();
        let ingest_state = IngestState::new();
        let (sender, _rx) = build_channel();
        let senders = buffer::broadcast::create();
        let _r1 = senders.spans.subscribe();
        let _r2 = senders.metrics.subscribe();
        let _r3 = senders.metrics.subscribe();
        let _r4 = senders.logs.subscribe();
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender, &senders));
        assert_eq!(lines[0]["fields"]["broadcast_subscribers"], 4);
    }

    #[test]
    fn emit_ingest_tick_emits_per_stream_broadcast_subscribers_gauges() {
        let state = HeartbeatState::new();
        let ingest_state = IngestState::new();
        let (sender, _rx) = build_channel();
        let senders = buffer::broadcast::create();
        let _r = senders.spans.subscribe();
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender, &senders));

        let gauges: Vec<_> = lines
            .iter()
            .filter(|l| l["target"] == "metric.ingest.channel.broadcast_subscribers")
            .collect();
        assert_eq!(gauges.len(), 3, "one gauge per stream");

        let span_gauge = gauges
            .iter()
            .find(|g| g["fields"]["channel_name"] == "pulse://stream/spans")
            .expect("spans gauge");
        assert_eq!(span_gauge["fields"]["value"], 1);
    }

    #[test]
    fn emit_buffer_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);
        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        // First tick emits buffer.tick + metric.buffer.memory_bytes; no eviction
        // delta so metric.buffer.evicted_span_count is suppressed.
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0]["target"], "buffer.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("rows_ingested").is_some());
        assert!(fields.get("retention_window_active").is_some());
        assert!(fields.get("eviction_count").is_some());
        assert!(fields.get("memory_bytes").is_some());
        assert!(fields.get("retention_window_seconds").is_some());
        assert!(fields.get("eviction_count_since_last_tick").is_some());
        assert!(state.last_buffer().is_some());
    }

    #[test]
    fn emit_buffer_tick_reflects_real_rows_ingested_count() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        buffer_state.record_rows_appended(11);
        let last_eviction = AtomicU64::new(0);
        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines[0]["fields"]["rows_ingested"], 11);
    }

    #[test]
    fn emit_buffer_tick_reflects_real_eviction_count_delta() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);

        // First tick: no eviction yet; delta should be 0
        buffer_state.record_eviction(5);
        let lines_first =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines_first[0]["fields"]["eviction_count"], 5);
        assert_eq!(
            lines_first[0]["fields"]["eviction_count_since_last_tick"],
            5
        );

        // Second tick: another 3 evictions; delta should be 3 (5 was previous)
        buffer_state.record_eviction(3);
        let lines_second =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines_second[0]["fields"]["eviction_count"], 8);
        assert_eq!(
            lines_second[0]["fields"]["eviction_count_since_last_tick"],
            3
        );
    }

    #[test]
    fn emit_buffer_tick_reflects_real_memory_bytes() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        buffer_state.set_memory_bytes(4096);
        let last_eviction = AtomicU64::new(0);
        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines[0]["fields"]["memory_bytes"], 4096);
    }

    #[test]
    fn emit_buffer_tick_reflects_retention_window_active_after_flip() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);

        let lines_pre =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines_pre[0]["fields"]["retention_window_active"], false);

        buffer_state.mark_retention_active();
        let lines_post =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert_eq!(lines_post[0]["fields"]["retention_window_active"], true);
    }

    #[test]
    fn emit_buffer_tick_emits_metric_buffer_memory_bytes_event() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        buffer_state.record_rows_appended(10);
        buffer_state.set_memory_bytes(2560);
        let last_eviction = AtomicU64::new(0);

        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        let memory_event = lines
            .iter()
            .find(|l| l["target"] == "metric.buffer.memory_bytes")
            .expect("metric.buffer.memory_bytes event must be emitted per tick");
        let fields = &memory_event["fields"];
        assert_eq!(fields["value"], 2560);
        assert_eq!(fields["retention_window_seconds"], 600);
        assert_eq!(fields["rows_active"], 10);
        assert_eq!(fields["eviction_count_since_last_tick"], 0);
    }

    #[test]
    fn emit_buffer_tick_emits_metric_buffer_evicted_span_count_event_when_delta_positive() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        buffer_state.record_eviction(7);
        let last_eviction = AtomicU64::new(0);

        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        let evicted_event = lines
            .iter()
            .find(|l| l["target"] == "metric.buffer.evicted_span_count")
            .expect("metric.buffer.evicted_span_count event must emit when delta > 0");
        let fields = &evicted_event["fields"];
        assert_eq!(fields["value"], 7);
        assert_eq!(fields["retention_window_seconds"], 600);
    }

    #[test]
    fn emit_buffer_tick_suppresses_metric_evicted_span_count_when_delta_zero() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);

        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));
        assert!(
            lines
                .iter()
                .all(|l| l["target"] != "metric.buffer.evicted_span_count"),
            "metric.buffer.evicted_span_count must NOT emit when delta == 0"
        );
    }

    // ─── Chunk #69 Phase B Session 7+ — Drain heartbeat path ──────────────

    #[test]
    fn emit_buffer_tick_surfaces_drain_fields_when_miner_threaded_in() {
        use buffer::{DrainConfig, DrainMiner};
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);
        let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
        miner.assign("connection from server completed at startup");
        miner.assign("connection from server completed at startup");

        let lines = capture_lines(|| {
            emit_buffer_tick(
                &state,
                &buffer_state,
                600,
                &last_eviction,
                Some(miner.as_ref()),
            )
        });

        let tick = lines
            .iter()
            .find(|l| l["target"] == "buffer.tick")
            .expect("buffer.tick line present");
        let fields = &tick["fields"];
        assert_eq!(fields["drain_template_count"], 1);
        assert_eq!(fields["drain_lru_evictions_since_tick"], 0);
    }

    #[test]
    fn emit_buffer_tick_emits_metric_pipeline_l1c_drain_template_count_total_when_miner_present() {
        use buffer::{DrainConfig, DrainMiner};
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);
        let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
        miner.assign("event log alpha occurred at startup");
        miner.assign("event log beta occurred at startup");

        let lines = capture_lines(|| {
            emit_buffer_tick(
                &state,
                &buffer_state,
                600,
                &last_eviction,
                Some(miner.as_ref()),
            )
        });

        let event = lines
            .iter()
            .find(|l| l["target"] == "metric.pipeline.l1c.drain_template_count_total")
            .expect("drain template count total metric must emit when miner threaded in");
        assert_eq!(event["fields"]["value"], 2);
    }

    #[test]
    fn emit_buffer_tick_suppresses_drain_metric_when_miner_not_threaded_in() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);

        let lines =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction, None));

        assert!(
            lines
                .iter()
                .all(|l| l["target"] != "metric.pipeline.l1c.drain_template_count_total"),
            "drain template count total metric must NOT emit when no miner threaded in"
        );
    }

    #[test]
    fn emit_buffer_tick_resets_drain_lru_evictions_counter_each_tick() {
        use buffer::{DrainConfig, DrainMiner};
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);
        // Configure a tight max_clusters so the LRU eviction loop fires
        // deterministically; disable masking so each message stays
        // distinct (and similarity=0.99 forces new clusters per message).
        let cfg = DrainConfig {
            depth: 4,
            similarity: 0.99,
            max_clusters: 2,
            masking_patterns: Vec::new(),
        };
        let miner = Arc::new(DrainMiner::new(cfg, None));
        for i in 0..6 {
            miner.assign(&format!("distinct message_x{i} word tail content"));
        }

        let lines_first = capture_lines(|| {
            emit_buffer_tick(
                &state,
                &buffer_state,
                600,
                &last_eviction,
                Some(miner.as_ref()),
            )
        });
        let first_tick = lines_first
            .iter()
            .find(|l| l["target"] == "buffer.tick")
            .expect("first tick present");
        let first_evictions = first_tick["fields"]["drain_lru_evictions_since_tick"]
            .as_u64()
            .expect("u64");
        assert!(
            first_evictions >= 1,
            "first tick must reflect accumulated evictions; got {}",
            first_evictions
        );

        // Second tick (immediately after) — counter reset; no new assigns.
        let lines_second = capture_lines(|| {
            emit_buffer_tick(
                &state,
                &buffer_state,
                600,
                &last_eviction,
                Some(miner.as_ref()),
            )
        });
        let second_tick = lines_second
            .iter()
            .find(|l| l["target"] == "buffer.tick")
            .expect("second tick present");
        assert_eq!(second_tick["fields"]["drain_lru_evictions_since_tick"], 0);
    }

    #[test]
    fn emit_viz_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let viz_state = VizState::new();
        let lines = capture_lines(|| emit_viz_tick(&state, &viz_state));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "viz.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("query_latency_ms").is_some());
        assert!(fields.get("subscribers_active").is_some());
        assert!(state.last_viz().is_some());
    }

    #[test]
    fn emit_viz_tick_reflects_real_query_latency_avg() {
        let state = HeartbeatState::new();
        let viz_state = VizState::new();
        viz_state.record_query_latency_ms(50);
        viz_state.record_query_latency_ms(150);
        let lines = capture_lines(|| emit_viz_tick(&state, &viz_state));
        assert_eq!(lines[0]["fields"]["query_latency_ms"], 100.0);
    }

    #[test]
    fn emit_viz_tick_reflects_real_subscribers_count() {
        let state = HeartbeatState::new();
        let viz_state = VizState::new();
        viz_state.inc_subscribers();
        viz_state.inc_subscribers();
        viz_state.inc_subscribers();
        let lines = capture_lines(|| emit_viz_tick(&state, &viz_state));
        assert_eq!(lines[0]["fields"]["subscribers_active"], 3);
    }

    #[test]
    fn emit_plugins_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let registry = Arc::new(Mutex::new(PluginRegistry::empty()));
        let lines = capture_lines(|| emit_plugins_tick(&state, &registry));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "plugins.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("loaded_count").is_some());
        assert!(fields.get("active_invocations").is_some());
        assert_eq!(fields["loaded_count"], 0);
        assert_eq!(fields["active_invocations"], 0);
        assert!(state.last_plugins().is_some());
    }

    #[test]
    fn emit_plugins_tick_reflects_registry_invocation_counter() {
        let state = HeartbeatState::new();
        let registry = PluginRegistry::empty();
        registry.record_invocation();
        registry.record_invocation();
        registry.record_invocation();
        let registry = Arc::new(Mutex::new(registry));
        let lines = capture_lines(|| emit_plugins_tick(&state, &registry));
        assert_eq!(lines[0]["fields"]["active_invocations"], 3);
    }

    #[tokio::test]
    async fn spawn_returns_five_handles_and_aborts_cleanly() {
        use std::sync::atomic::AtomicBool;
        struct StubBindStatus;
        impl ReceiverBindStatus for StubBindStatus {
            fn any_receiver_failed(&self) -> bool {
                false
            }
        }
        // Suppress dead-code lint on AtomicBool import path consistency:
        let _ = AtomicBool::new(false);

        let state = Arc::new(HeartbeatState::new());
        let ingest_state = Arc::new(IngestState::new());
        let (sender, _rx) = build_channel();
        let sender = Arc::new(sender);
        let buffer_state = Arc::new(BufferState::new());
        let viz_state = Arc::new(VizState::new());
        let senders = Arc::new(buffer::broadcast::create());
        let plugins_registry = Arc::new(Mutex::new(PluginRegistry::empty()));
        let bind_status: Arc<dyn ReceiverBindStatus> = Arc::new(StubBindStatus);
        let handles = spawn(
            state,
            ingest_state,
            sender,
            buffer_state,
            600,
            viz_state,
            senders,
            plugins_registry,
            bind_status,
            None,
        );
        assert_eq!(handles.len(), 5);
        for handle in handles {
            handle.abort();
            let _ = handle.await;
        }
    }

    // Chunk #59 — connection.tick emission tests.

    struct StubBindStatusFalse;
    impl ReceiverBindStatus for StubBindStatusFalse {
        fn any_receiver_failed(&self) -> bool {
            false
        }
    }

    struct StubBindStatusTrue;
    impl ReceiverBindStatus for StubBindStatusTrue {
        fn any_receiver_failed(&self) -> bool {
            true
        }
    }

    #[test]
    fn emit_connection_tick_listening_when_no_ingest_and_bind_ok() {
        let ingest_state = IngestState::new();
        let bind = StubBindStatusFalse;
        let lines = capture_lines(|| emit_connection_tick(&ingest_state, &bind));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "connection.tick");
        let fields = &lines[0]["fields"];
        assert_eq!(fields["state"], "Listening");
        assert_eq!(fields["last_span_ago_ms"], 0);
        assert_eq!(fields["severity"], "info");
    }

    #[test]
    fn emit_connection_tick_receiving_after_ingest_recent() {
        let ingest_state = IngestState::new();
        ingest_state.record_spans(1);
        let bind = StubBindStatusFalse;
        let lines = capture_lines(|| emit_connection_tick(&ingest_state, &bind));
        assert_eq!(lines[0]["fields"]["state"], "Receiving");
        assert_eq!(lines[0]["fields"]["severity"], "info");
    }

    #[test]
    fn emit_connection_tick_receiver_failed_when_bind_failed() {
        let ingest_state = IngestState::new();
        ingest_state.record_spans(1);
        let bind = StubBindStatusTrue;
        let lines = capture_lines(|| emit_connection_tick(&ingest_state, &bind));
        assert_eq!(lines[0]["fields"]["state"], "ReceiverFailed");
        assert_eq!(lines[0]["fields"]["severity"], "critical");
    }
}
