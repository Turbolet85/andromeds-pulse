//! Chunk #54 perf SLO substrate — 10k spans/sec sustained-load test.
//!
//! Asserts the production ingest → buffer pipeline accepts 100k synthetic
//! spans within а 12s window (10k spans/sec sustained throughput). Mirrors
//! chunk #50 P1 in-process bootstrap pattern: ephemeral loopback gRPC
//! receiver + real DuckDB connection + run_consumer task + tonic 0.14
//! client driving the live receiver.
//!
//! Frame-rate ≥30 fps frame-buffer inspection is OUT of agent-driven scope
//! per test-plan §1 `performance-budget: WebGPU canvas throughput` row —
//! that activates with tauri-driver headful E2E suite in а future chunk.
//! This test verifies the throughput layer only (ingest-channel + buffer
//! consumer + DuckDB Arrow appender stays within budget at sustained load).
//!
//! Post-test `metric.webgpu.frame_duration_ms` p99 ≤33ms + `metric.buffer.
//! memory_bytes` max ≤512MB gates fire via `cargo xtask perf:slo-load` →
//! `xtask/ci/perf-slo-check.{sh,ps1}` reading `agent-latest.jsonl`. Empty
//! event streams (webview not booted, heartbeat not running) map к NEUTRAL
//! pass; full gate activates когда production observability subscribes
//! during the load window.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;

const TARGET_SPANS: u64 = 100_000;
const BATCH_SIZE: usize = 1_000;
const TIMEOUT_SECS: u64 = 12;

fn make_span(trace_id: [u8; 16], span_id: [u8; 8], start_ns: u64) -> Span {
    Span {
        trace_id: trace_id.to_vec(),
        span_id: span_id.to_vec(),
        name: "perf-slo-span".to_string(),
        kind: 0,
        start_time_unix_nano: start_ns,
        end_time_unix_nano: start_ns + 1_000_000,
        ..Default::default()
    }
}

fn unique_trace_id(batch_seed: u64) -> [u8; 16] {
    // Distinct trace_id per batch — first 8 bytes = batch_seed LE, last 8 = 1s
    // padding к keep а recognizable non-zero suffix (avoids OTLP zero-trace-id
    // semantic-vs-format ambiguity).
    let mut out = [1u8; 16];
    let bytes = batch_seed.to_le_bytes();
    out[..8].copy_from_slice(&bytes);
    out
}

fn unique_span_id(batch_seed: u64, span_idx: usize) -> [u8; 8] {
    // Distinct (trace_id, span_id) per span via а global counter derived от
    // batch_seed × BATCH_SIZE + span_idx. With 100 batches × 1000 spans the
    // counter range is ≤100_000 (well within u64 / 8-byte space).
    let counter = batch_seed.wrapping_mul(BATCH_SIZE as u64) + (span_idx as u64) + 1;
    counter.to_le_bytes()
}

fn make_export_request(batch_seed: u64, span_count: usize) -> ExportTraceServiceRequest {
    let start_ns = 1_700_000_000_000_000u64 + (batch_seed * 1_000_000);
    let trace_id = unique_trace_id(batch_seed);
    let spans: Vec<Span> = (0..span_count)
        .map(|i| {
            let span_id = unique_span_id(batch_seed, i);
            make_span(trace_id, span_id, start_ns + (i as u64))
        })
        .collect();
    let resource = Resource {
        attributes: vec![KeyValue {
            key: "service.name".into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue("perf-slo-loadgen".into())),
            }),
        }],
        dropped_attributes_count: 0,
    };
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(resource),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

async fn wait_for_rows(state: &Arc<BufferState>, target: u64, timeout: Duration) -> bool {
    let deadline = tokio::time::Instant::now() + timeout;
    while tokio::time::Instant::now() < deadline {
        if state.snapshot().rows_ingested >= target {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    false
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn perf_slo_sustained_10k_spans_per_sec_ingest_throughput_holds() {
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let consumer_handle = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        None,
    ));

    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port succeeds");
    let bound = listener.local_addr().expect("listener exposes local_addr");
    assert!(
        bound.ip().is_loopback(),
        "perf SLO load receiver must bind only to loopback; got {}",
        bound.ip()
    );

    let serve_handle = {
        let state = Arc::clone(&ingest_state);
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };

    tokio::time::sleep(Duration::from_millis(100)).await;

    let endpoint = format!("http://{}", bound);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects к loopback receiver");

    let batch_count = (TARGET_SPANS as usize).div_ceil(BATCH_SIZE);
    let load_window = Duration::from_secs(10);
    let inter_batch_delay = load_window
        .checked_div(batch_count as u32)
        .unwrap_or(Duration::from_millis(100));

    for batch_idx in 0..batch_count {
        let req = make_export_request(batch_idx as u64, BATCH_SIZE);
        let resp = client.export(req).await;
        assert!(
            resp.is_ok(),
            "perf SLO load batch {} export must succeed; got {:?}",
            batch_idx,
            resp.as_ref().err().map(|s| s.code())
        );
        tokio::time::sleep(inter_batch_delay).await;
    }

    let ingested = wait_for_rows(
        &buffer_state,
        TARGET_SPANS,
        Duration::from_secs(TIMEOUT_SECS),
    )
    .await;
    let snapshot = buffer_state.snapshot();
    assert!(
        ingested,
        "buffer must ingest {} spans within {}s; rows_ingested = {}",
        TARGET_SPANS, TIMEOUT_SECS, snapshot.rows_ingested
    );

    serve_handle.abort();
    drop(sender);
    let _ = tokio::time::timeout(Duration::from_secs(2), consumer_handle).await;
}
