use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tokio::task::JoinHandle;
use ui_bridge::health::HeartbeatState;

const TICK_INTERVAL_SECS: u64 = 15;

pub(crate) fn spawn(state: Arc<HeartbeatState>) -> Vec<JoinHandle<()>> {
    vec![
        tokio::spawn(run_ingest(state.clone())),
        tokio::spawn(run_buffer(state.clone())),
        tokio::spawn(run_viz(state.clone())),
        tokio::spawn(run_plugins(state)),
    ]
}

async fn run_ingest(state: Arc<HeartbeatState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_ingest_tick(&state);
    }
}

async fn run_buffer(state: Arc<HeartbeatState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(TICK_INTERVAL_SECS));
    loop {
        interval.tick().await;
        emit_buffer_tick(&state);
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

fn emit_ingest_tick(state: &HeartbeatState) {
    let payload = ingest::contract::heartbeat_payload();
    state.record_ingest(Utc::now());
    tracing::info!(
        target: "ingest.tick",
        span_count = payload.span_count,
        buffer_capacity_pct = payload.buffer_capacity_pct,
        broadcast_subscribers = payload.broadcast_subscribers,
        "heartbeat",
    );
}

fn emit_buffer_tick(state: &HeartbeatState) {
    let payload = buffer::contract::heartbeat_payload();
    state.record_buffer(Utc::now());
    tracing::info!(
        target: "buffer.tick",
        rows_ingested = payload.rows_ingested,
        retention_window_active = payload.retention_window_active,
        eviction_count = payload.eviction_count,
        "heartbeat",
    );
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
        let lines = capture_lines(|| emit_ingest_tick(&state));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "ingest.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("span_count").is_some());
        assert!(fields.get("buffer_capacity_pct").is_some());
        assert!(fields.get("broadcast_subscribers").is_some());
        assert!(state.last_ingest().is_some());
    }

    #[test]
    fn emit_buffer_tick_emits_target_and_fields() {
        let state = HeartbeatState::new();
        let lines = capture_lines(|| emit_buffer_tick(&state));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["target"], "buffer.tick");
        let fields = &lines[0]["fields"];
        assert!(fields.get("rows_ingested").is_some());
        assert!(fields.get("retention_window_active").is_some());
        assert!(fields.get("eviction_count").is_some());
        assert!(state.last_buffer().is_some());
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
        let handles = spawn(state);
        assert_eq!(handles.len(), 4);
        for handle in handles {
            handle.abort();
            let _ = handle.await;
        }
    }
}
