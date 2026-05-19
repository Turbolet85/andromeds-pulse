//! Chunk #66 E2E coverage — exception fingerprinting + retry storm detector
//! end-to-end. Drives the production OTLP gRPC receiver via tonic 0.14
//! client on an ephemeral loopback port; bootstraps а real DuckDB
//! connection + buffer consumer task + RetryStormDetector wired through
//! the FingerprintObserver hot-path hook + AttentionCueBroadcast
//! subscription. Asserts:
//!
//! 1. The 5th identical-fingerprint exception event triggers а `RetryStorm`
//!    cue с `priority_tier: Suggested` on the broadcast channel.
//! 2. The 10th event triggers escalation к `priority_tier: Autonomous`.
//! 3. `span_events.fingerprint BLOB` column populated с deterministic
//!    16-byte hash bytes (chunk #65 substrate + chunk #66 compute).
//! 4. All 10 fingerprints identical (deterministic — same `exception.type` +
//!    normalized stacktrace → same hash).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::fingerprint::FingerprintObserver;
use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span, span};
use ingest::state::IngestState;
use pulse_app::storm_observer::StormObserverAdapter;
use triage::contract::{
    AttentionCueBroadcast, CueKind, DEFAULT_AUTONOMOUS_THRESHOLD,
    DEFAULT_DETECTION_SUB_WINDOW_SECONDS, DEFAULT_STORM_WINDOW_SECONDS,
    DEFAULT_SUGGESTED_THRESHOLD, PriorityTier, RetryStormDetector,
};

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn str_attr(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.into(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(value.into())),
        }),
    }
}

fn make_exception_span(trace_id: [u8; 16], span_id: [u8; 8], start_ns: u64) -> Span {
    let event = span::Event {
        time_unix_nano: start_ns + 100_000,
        name: "exception".into(),
        attributes: vec![
            str_attr("exception.type", "java.lang.RuntimeException"),
            str_attr("exception.message", "boom"),
            str_attr(
                "exception.stacktrace",
                "    at com.example.Foo.bar(Foo.java:42)\n    at com.example.Foo.baz(Foo.java:50)\n    at com.example.Foo.qux(Foo.java:58)",
            ),
        ],
        dropped_attributes_count: 0,
    };
    Span {
        trace_id: trace_id.to_vec(),
        span_id: span_id.to_vec(),
        name: "storm-test-span".to_string(),
        kind: 0,
        start_time_unix_nano: start_ns,
        end_time_unix_nano: start_ns + 1_000_000,
        events: vec![event],
        ..Default::default()
    }
}

fn make_export_request(span_id_byte: u8) -> ExportTraceServiceRequest {
    let now = now_ns();
    let mut trace_id = [0u8; 16];
    trace_id[15] = span_id_byte;
    let mut span_id = [0u8; 8];
    span_id[7] = span_id_byte;
    let span = make_exception_span(trace_id, span_id, now);
    let resource = Resource {
        attributes: vec![str_attr("service.name", "storm-test-app")],
        dropped_attributes_count: 0,
    };
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(resource),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![span],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

async fn wait_for_event_rows(
    conn: &Arc<Mutex<Connection>>,
    target: i64,
    timeout: Duration,
) -> bool {
    let deadline = tokio::time::Instant::now() + timeout;
    while tokio::time::Instant::now() < deadline {
        let count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM span_events", [], |r| r.get(0))
            .unwrap_or(0);
        if count >= target {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    false
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn chunk_66_storm_detector_emits_suggested_at_5th_autonomous_at_10th() {
    // ===== Bootstrap production wiring in-process =====
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    // Chunk #66 wiring: storm detector + observer adapter + cue broadcast.
    let storm_detector = Arc::new(RetryStormDetector::new(
        DEFAULT_STORM_WINDOW_SECONDS,
        DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
        DEFAULT_SUGGESTED_THRESHOLD,
        DEFAULT_AUTONOMOUS_THRESHOLD,
    ));
    let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
    let mut cue_rx = cue_broadcast.subscribe();
    let fingerprint_observer: Option<Arc<dyn FingerprintObserver>> = Some(Arc::new(
        StormObserverAdapter::new(Arc::clone(&storm_detector), Arc::clone(&cue_broadcast)),
    ));

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let _consumer_handle = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        fingerprint_observer,
        // Chunk #69 Phase B Session 2: drain_miner param (None = no template assignment).
        None,
    ));

    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port succeeds");
    let bound = listener.local_addr().expect("listener exposes local_addr");
    assert!(
        bound.ip().is_loopback(),
        "test receiver must bind only to loopback; got {}",
        bound.ip()
    );

    let _serve_handle = {
        let state = Arc::clone(&ingest_state);
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };

    tokio::time::sleep(Duration::from_millis(100)).await;

    // ===== Drive 10 identical-fingerprint exception events sequentially =====
    let endpoint = format!("http://{}", bound);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects к loopback receiver");

    for i in 0..10 {
        let request = make_export_request((i + 1) as u8);
        client.export(request).await.expect("export ok");
    }

    // ===== Wait for buffer to drain channel + persist span_events =====
    let ingested = wait_for_event_rows(&conn, 10, Duration::from_secs(5)).await;
    assert!(
        ingested,
        "buffer should ingest 10 span_events within 5s; got count = {}",
        conn.lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM span_events", [], |r| r
                .get::<_, i64>(0))
            .unwrap_or(-1)
    );

    // ===== Assert all 10 fingerprints populated + identical =====
    let with_fp: i64 = conn
        .lock()
        .unwrap()
        .query_row(
            "SELECT COUNT(*) FROM span_events WHERE fingerprint IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .expect("count fingerprints");
    assert_eq!(
        with_fp, 10,
        "all 10 exception events MUST populate fingerprint column"
    );

    let distinct_fps: i64 = conn
        .lock()
        .unwrap()
        .query_row(
            "SELECT COUNT(DISTINCT fingerprint) FROM span_events WHERE fingerprint IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .expect("count distinct");
    assert_eq!(
        distinct_fps, 1,
        "identical exception.type + normalized stack MUST produce ONE fingerprint across 10 events; got {distinct_fps} distinct values"
    );

    // ===== Assert storm detector emitted Suggested then Autonomous =====
    let mut emitted_tiers: Vec<PriorityTier> = Vec::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while tokio::time::Instant::now() < deadline && emitted_tiers.len() < 2 {
        if let Ok(cue) = cue_rx.try_recv() {
            assert_eq!(
                cue.kind,
                CueKind::RetryStorm,
                "non-RetryStorm cue surfaced: {:?}",
                cue
            );
            emitted_tiers.push(cue.priority_tier);
        } else {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    assert_eq!(
        emitted_tiers,
        vec![PriorityTier::Suggested, PriorityTier::Autonomous],
        "storm detector MUST emit Suggested (5th occurrence) then Autonomous (10th occurrence); got {:?}",
        emitted_tiers
    );
    assert_eq!(
        storm_detector.storms_detected_total(),
        2,
        "storms_detected_total counter MUST equal 2 (one Suggested + one Autonomous)"
    );
}
