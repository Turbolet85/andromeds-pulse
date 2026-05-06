use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use buffer::BufferState;
use chrono::Utc;
use ingest::channel::IngestSender;
use ingest::state::IngestState;
use tokio::task::JoinHandle;
use ui_bridge::health::HeartbeatState;

const TICK_INTERVAL_SECS: u64 = 15;

pub(crate) fn spawn(
    state: Arc<HeartbeatState>,
    ingest_state: Arc<IngestState>,
    ingest_sender: Arc<IngestSender>,
    buffer_state: Arc<BufferState>,
    retention_seconds: u64,
) -> Vec<JoinHandle<()>> {
    vec![
        tokio::spawn(run_ingest(state.clone(), ingest_state, ingest_sender)),
        tokio::spawn(run_buffer(state.clone(), buffer_state, retention_seconds)),
        tokio::spawn(run_viz(state.clone())),
        tokio::spawn(run_plugins(state)),
    ]
}

async fn run_ingest(
    state: Arc<HeartbeatState>,
    ingest_state: Arc<IngestState>,
    ingest_sender: Arc<IngestSender>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_ingest_tick(&state, &ingest_state, &ingest_sender);
    }
}

async fn run_buffer(
    state: Arc<HeartbeatState>,
    buffer_state: Arc<BufferState>,
    retention_seconds: u64,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    let last_eviction = AtomicU64::new(0);
    loop {
        interval.tick().await;
        emit_buffer_tick(&state, &buffer_state, retention_seconds, &last_eviction);
    }
}

async fn run_viz(state: Arc<HeartbeatState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_viz_tick(&state);
    }
}

async fn run_plugins(state: Arc<HeartbeatState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_plugins_tick(&state);
    }
}

fn emit_ingest_tick(
    state: &HeartbeatState,
    ingest_state: &IngestState,
    ingest_sender: &IngestSender,
) {
    let payload = ingest::contract::heartbeat_payload(ingest_state, ingest_sender);
    state.record_ingest(Utc::now());
    tracing::info!(
        target: "ingest.tick",
        span_count = payload.span_count,
        buffer_capacity_pct = payload.buffer_capacity_pct,
        broadcast_subscribers = payload.broadcast_subscribers,
        "heartbeat",
    );
}

fn emit_buffer_tick(
    state: &HeartbeatState,
    buffer_state: &BufferState,
    retention_seconds: u64,
    last_eviction: &AtomicU64,
) {
    let snap = buffer_state.snapshot();
    let prev = last_eviction.swap(snap.eviction_count, Ordering::Relaxed);
    let delta = snap.eviction_count.saturating_sub(prev);
    let rows_active = snap.rows_ingested.saturating_sub(snap.eviction_count);

    let payload = buffer::contract::heartbeat_payload(buffer_state, retention_seconds, delta);
    state.record_buffer(Utc::now());

    tracing::info!(
        target: "buffer.tick",
        rows_ingested = payload.rows_ingested,
        retention_window_active = payload.retention_window_active,
        eviction_count = payload.eviction_count,
        memory_bytes = payload.memory_bytes,
        retention_window_seconds = payload.retention_window_seconds,
        eviction_count_since_last_tick = payload.eviction_count_since_last_tick,
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
}

fn emit_viz_tick(state: &HeartbeatState) {
    let payload = viz::contract::heartbeat_payload();
    state.record_viz(Utc::now());
    tracing::info!(
        target: "viz.tick",
        query_latency_ms = payload.query_latency_ms,
        subscribers_active = payload.subscribers_active,
        "heartbeat",
    );
}

fn emit_plugins_tick(state: &HeartbeatState) {
    let payload = plugins::contract::heartbeat_payload();
    state.record_plugins(Utc::now());
    tracing::info!(
        target: "plugins.tick",
        loaded_count = payload.loaded_count,
        active_invocations = payload.active_invocations,
        "heartbeat",
    );
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
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender));
        assert_eq!(lines.len(), 1);
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
        ingest_state.record_spans(7);
        let lines = capture_lines(|| emit_ingest_tick(&state, &ingest_state, &sender));
        assert_eq!(lines[0]["fields"]["span_count"], 7);
    }

    #[test]
    fn emit_buffer_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);
        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
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
        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
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
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
        assert_eq!(lines_first[0]["fields"]["eviction_count"], 5);
        assert_eq!(
            lines_first[0]["fields"]["eviction_count_since_last_tick"],
            5
        );

        // Second tick: another 3 evictions; delta should be 3 (5 was previous)
        buffer_state.record_eviction(3);
        let lines_second =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
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
        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
        assert_eq!(lines[0]["fields"]["memory_bytes"], 4096);
    }

    #[test]
    fn emit_buffer_tick_reflects_retention_window_active_after_flip() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        let last_eviction = AtomicU64::new(0);

        let lines_pre =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
        assert_eq!(lines_pre[0]["fields"]["retention_window_active"], false);

        buffer_state.mark_retention_active();
        let lines_post =
            capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
        assert_eq!(lines_post[0]["fields"]["retention_window_active"], true);
    }

    #[test]
    fn emit_buffer_tick_emits_metric_buffer_memory_bytes_event() {
        let state = HeartbeatState::new();
        let buffer_state = BufferState::new();
        buffer_state.record_rows_appended(10);
        buffer_state.set_memory_bytes(2560);
        let last_eviction = AtomicU64::new(0);

        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
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

        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
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

        let lines = capture_lines(|| emit_buffer_tick(&state, &buffer_state, 600, &last_eviction));
        assert!(
            lines
                .iter()
                .all(|l| l["target"] != "metric.buffer.evicted_span_count"),
            "metric.buffer.evicted_span_count must NOT emit when delta == 0"
        );
    }

    #[test]
    fn emit_viz_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let lines = capture_lines(|| emit_viz_tick(&state));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "viz.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("query_latency_ms").is_some());
        assert!(fields.get("subscribers_active").is_some());
        assert!(state.last_viz().is_some());
    }

    #[test]
    fn emit_plugins_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let lines = capture_lines(|| emit_plugins_tick(&state));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "plugins.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("loaded_count").is_some());
        assert!(fields.get("active_invocations").is_some());
        assert!(state.last_plugins().is_some());
    }

    #[tokio::test]
    async fn spawn_returns_four_handles_and_aborts_cleanly() {
        let state = Arc::new(HeartbeatState::new());
        let ingest_state = Arc::new(IngestState::new());
        let (sender, _rx) = build_channel();
        let sender = Arc::new(sender);
        let buffer_state = Arc::new(BufferState::new());
        let handles = spawn(state, ingest_state, sender, buffer_state, 600);
        assert_eq!(handles.len(), 4);
        for handle in handles {
            handle.abort();
            let _ = handle.await;
        }
    }
}
