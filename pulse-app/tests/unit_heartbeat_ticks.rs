// Migrated 2026-08-30 from `pulse-app/src/heartbeat.rs::tests` — that crate
// sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. Internals reach here
// via `pub` + `#[doc(hidden)]` per test-plan §2/§4. The capture helper is
// deliberately a plain `fmt::layer().json()` (no allowlist): these pins
// assert emit-site truth UPSTREAM of redaction.

use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;

use buffer::{BufferState, DrainConfig, DrainMiner};
use ingest::channel::build_channel;
use ingest::connection::ReceiverBindStatus;
use ingest::state::IngestState;
use plugins::loader::PluginRegistry;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use ui_bridge::health::HeartbeatState;
use viz::VizState;

use pulse_app::heartbeat::{
    emit_buffer_tick, emit_connection_tick, emit_ingest_tick, emit_plugins_tick, emit_viz_tick,
    spawn_engine_ticks, spawn_window_ticks,
};

fn buffer_tick_for_test(
    state: &HeartbeatState,
    buffer_state: &BufferState,
    last_eviction: &AtomicU64,
) {
    buffer_tick_with_miner_for_test(state, buffer_state, last_eviction, None);
}

fn buffer_tick_with_miner_for_test(
    state: &HeartbeatState,
    buffer_state: &BufferState,
    last_eviction: &AtomicU64,
    drain_miner: Option<&DrainMiner>,
) {
    let (sender, _rx) = build_channel();
    let last_rows = AtomicU64::new(0);
    let _ = emit_buffer_tick(
        state,
        buffer_state,
        &sender,
        600,
        last_eviction,
        &last_rows,
        drain_miner,
    );
}

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
        .map(|l| serde_json::from_str::<serde_json::Value>(l).expect("each line parses as JSON"))
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
    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
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
    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
    assert_eq!(lines[0]["fields"]["rows_ingested"], 11);
}

#[test]
fn emit_buffer_tick_reflects_real_eviction_count_delta() {
    let state = HeartbeatState::new();
    let buffer_state = BufferState::new();
    let last_eviction = AtomicU64::new(0);

    // First tick: no eviction yet; delta should be 0
    buffer_state.record_eviction(5);
    let lines_first = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
    assert_eq!(lines_first[0]["fields"]["eviction_count"], 5);
    assert_eq!(
        lines_first[0]["fields"]["eviction_count_since_last_tick"],
        5
    );

    // Second tick: another 3 evictions; delta should be 3 (5 was previous)
    buffer_state.record_eviction(3);
    let lines_second =
        capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
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
    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
    assert_eq!(lines[0]["fields"]["memory_bytes"], 4096);
}

#[test]
fn emit_buffer_tick_reflects_retention_window_active_after_flip() {
    let state = HeartbeatState::new();
    let buffer_state = BufferState::new();
    let last_eviction = AtomicU64::new(0);

    let lines_pre = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
    assert_eq!(lines_pre[0]["fields"]["retention_window_active"], false);

    buffer_state.mark_retention_active();
    let lines_post = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
    assert_eq!(lines_post[0]["fields"]["retention_window_active"], true);
}

#[test]
fn emit_buffer_tick_emits_metric_buffer_memory_bytes_event() {
    let state = HeartbeatState::new();
    let buffer_state = BufferState::new();
    buffer_state.record_rows_appended(10);
    buffer_state.set_memory_bytes(2560);
    let last_eviction = AtomicU64::new(0);

    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
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

    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
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

    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));
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
    let state = HeartbeatState::new();
    let buffer_state = BufferState::new();
    let last_eviction = AtomicU64::new(0);
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    miner.assign("connection from server completed at startup");
    miner.assign("connection from server completed at startup");

    let lines = capture_lines(|| {
        buffer_tick_with_miner_for_test(&state, &buffer_state, &last_eviction, Some(miner.as_ref()))
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
    let state = HeartbeatState::new();
    let buffer_state = BufferState::new();
    let last_eviction = AtomicU64::new(0);
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    miner.assign("event log alpha occurred at startup");
    miner.assign("event log beta occurred at startup");

    let lines = capture_lines(|| {
        buffer_tick_with_miner_for_test(&state, &buffer_state, &last_eviction, Some(miner.as_ref()))
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

    let lines = capture_lines(|| buffer_tick_for_test(&state, &buffer_state, &last_eviction));

    assert!(
        lines
            .iter()
            .all(|l| l["target"] != "metric.pipeline.l1c.drain_template_count_total"),
        "drain template count total metric must NOT emit when no miner threaded in"
    );
}

#[test]
fn emit_buffer_tick_resets_drain_lru_evictions_counter_each_tick() {
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
        buffer_tick_with_miner_for_test(&state, &buffer_state, &last_eviction, Some(miner.as_ref()))
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
        buffer_tick_with_miner_for_test(&state, &buffer_state, &last_eviction, Some(miner.as_ref()))
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
async fn spawn_engine_ticks_returns_three_handles_and_aborts_cleanly() {
    struct StubBindStatus;
    impl ReceiverBindStatus for StubBindStatus {
        fn any_receiver_failed(&self) -> bool {
            false
        }
    }

    let state = Arc::new(HeartbeatState::new());
    let ingest_state = Arc::new(IngestState::new());
    let (sender, _rx) = build_channel();
    let sender = Arc::new(sender);
    let buffer_state = Arc::new(BufferState::new());
    let senders = Arc::new(buffer::broadcast::create());
    let bind_status: Arc<dyn ReceiverBindStatus> = Arc::new(StubBindStatus);
    let handles = spawn_engine_ticks(
        state,
        ingest_state,
        sender,
        buffer_state,
        600,
        senders,
        bind_status,
        None,
    );
    assert_eq!(handles.len(), 3);
    for handle in handles {
        handle.abort();
        let _ = handle.await;
    }
}

#[tokio::test]
async fn spawn_window_ticks_returns_two_handles_and_aborts_cleanly() {
    let state = Arc::new(HeartbeatState::new());
    let viz_state = Arc::new(VizState::new());
    let plugins_registry = Arc::new(Mutex::new(PluginRegistry::empty()));
    let handles = spawn_window_ticks(state, viz_state, plugins_registry);
    assert_eq!(handles.len(), 2);
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
