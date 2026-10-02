//! In-process producer of the memory and snapshot perf samples that
//! `cargo xtask perf:budget --require memory,snapshot` grades (obs-plan §10
//! Performance budgets, rows 1 and 3).
//!
//! It runs the real pieces with no window: the production obs sink, a loopback
//! gRPC receiver over a real DuckDB, the real consumer, the real retention
//! sweep, the real `buffer.tick` emitter and the production snapshot path. The
//! log lands in `<CARGO_TARGET_TMPDIR>/perf-budget-samples/logs/`, rewritten from
//! scratch every run. The test asserts only its own preconditions; the budget
//! verdict belongs to the xtask verb.
//!
//! Selected only by nextest `--profile perf-samples` (`.config/nextest.toml`).

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer, run_retention};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{
    ResourceSpans, ScopeSpans, Span, Status,
};
use ingest::state::IngestState;
use ui_bridge::health::HeartbeatState;

const SERVICES: [&str; 5] = [
    "checkout-service",
    "cart-service",
    "payment-service",
    "inventory-service",
    "shipping-service",
];
const SPANS_PER_BATCH: usize = 500;
const BATCHES: u64 = 60;
const BATCH_SPACING: Duration = Duration::from_millis(500);
const BATCHES_PER_TICK_WINDOW: u64 = 20;
/// Sweep cadence is `retention_seconds.max(60) / 6` = 10 s, first tick skipped.
const RETENTION_SECONDS: u64 = 60;
const SNAPSHOTS: usize = 50;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("perf-budget-samples")
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos() as u64
}

fn export_request(batch: u64) -> ExportTraceServiceRequest {
    let start = now_ns();
    let mut trace_id = [7u8; 16];
    trace_id[..8].copy_from_slice(&batch.to_le_bytes());
    let resource_spans = SERVICES
        .iter()
        .enumerate()
        .map(|(s, service)| {
            let spans = (0..SPANS_PER_BATCH / SERVICES.len())
                .map(|i| {
                    let counter = batch * 10_000 + (s * 1_000 + i) as u64 + 1;
                    let duration_ns = 1_000_000 * (1 + (i as u64 % 40));
                    Span {
                        trace_id: trace_id.to_vec(),
                        span_id: counter.to_le_bytes().to_vec(),
                        name: format!("{service}.handle"),
                        kind: 2,
                        start_time_unix_nano: start + i as u64,
                        end_time_unix_nano: start + i as u64 + duration_ns,
                        status: Some(Status {
                            code: if i % 25 == 0 { 2 } else { 1 },
                            message: String::new(),
                        }),
                        ..Default::default()
                    }
                })
                .collect();
            ResourceSpans {
                resource: Some(Resource {
                    attributes: vec![KeyValue {
                        key: "service.name".into(),
                        value: Some(AnyValue {
                            value: Some(any_value::Value::StringValue((*service).into())),
                        }),
                    }],
                    dropped_attributes_count: 0,
                }),
                scope_spans: vec![ScopeSpans {
                    scope: None,
                    spans,
                    schema_url: String::new(),
                }],
                schema_url: String::new(),
            }
        })
        .collect();
    ExportTraceServiceRequest { resource_spans }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn perf_budget_samples_are_produced_by_the_real_pipeline() {
    let dir = data_dir();
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("clear the previous run's data dir");
    }
    std::fs::create_dir_all(&dir).expect("create data dir");
    let guard = pulse_app::observability::init(&dir);

    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());
    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let consumer = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        None,
        None,
    ));
    let retention = tokio::spawn(run_retention(
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        RETENTION_SECONDS,
    ));

    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind an ephemeral loopback port");
    let bound = listener.local_addr().expect("listener local_addr");
    assert!(bound.ip().is_loopback(), "receiver must bind loopback only");
    let serve = {
        let state = Arc::new(IngestState::new());
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };
    let mut client = TraceServiceClient::connect(format!("http://{bound}"))
        .await
        .expect("client connects to the loopback receiver");

    let heartbeat = HeartbeatState::new();
    let last_eviction = AtomicU64::new(0);
    let last_rows = AtomicU64::new(0);
    let mut nonzero_ticks = 0usize;
    for batch in 0..BATCHES {
        client
            .export(export_request(batch))
            .await
            .unwrap_or_else(|s| panic!("batch {batch} export: {:?}", s.code()));
        tokio::time::sleep(BATCH_SPACING).await;
        if (batch + 1) % BATCHES_PER_TICK_WINDOW == 0 {
            let gauge = buffer_state.snapshot().memory_bytes;
            pulse_app::heartbeat::emit_buffer_tick(
                &heartbeat,
                &buffer_state,
                &sender,
                RETENTION_SECONDS,
                &last_eviction,
                &last_rows,
                None,
            );
            if gauge > 0 {
                nonzero_ticks += 1;
            }
        }
    }

    let expected_rows = BATCHES * SPANS_PER_BATCH as u64;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    while buffer_state.snapshot().rows_ingested < expected_rows
        && tokio::time::Instant::now() < deadline
    {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let rows = buffer_state.snapshot().rows_ingested;
    assert!(
        rows >= expected_rows,
        "spans landed: {rows} of {expected_rows}"
    );
    assert!(
        nonzero_ticks >= 2,
        "buffer ticks after a retention sweep carrying a non-zero gauge: {nonzero_ticks}"
    );

    let read_conn = conn
        .lock()
        .expect("appender connection lock")
        .try_clone()
        .expect("read connection clone");
    let generated = tokio::task::spawn_blocking(move || {
        (0..SNAPSHOTS)
            .filter(|_| pulse_app::snapshot_runtime::load_curated_markdown(&read_conn).is_ok())
            .count()
    })
    .await
    .expect("snapshot task joins");
    assert_eq!(generated, SNAPSHOTS, "snapshots generated Ok");

    serve.abort();
    retention.abort();
    drop(sender);
    let _ = tokio::time::timeout(Duration::from_secs(2), consumer).await;
    drop(guard);
}
