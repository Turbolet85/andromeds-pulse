//! P1 E2E coverage — OTLP gRPC ingest → buffer → viz::query::query_traces
//! returns matching rows. Per test-plan §6 P1 canonical 7-step structure.
//!
//! Drives the production ingest gRPC receiver via tonic 0.14 client on an
//! ephemeral loopback port; bootstraps а real DuckDB connection + buffer
//! consumer task; queries via `viz::query::query_traces` directly (the
//! direct-function-call fallback path per plan.md Open Question 3 —
//! preserves cross-crate data-flow coverage without requiring TauRPC mock
//! builder which is unverified в codebase).

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use viz::VizState;
use viz::query::{TracesQueryArgs, query_traces};

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn make_span(trace_id: [u8; 16], span_id: [u8; 8], start_ns: u64) -> Span {
    Span {
        trace_id: trace_id.to_vec(),
        span_id: span_id.to_vec(),
        name: "p1-span".to_string(),
        kind: 0,
        start_time_unix_nano: start_ns,
        end_time_unix_nano: start_ns + 1_000_000, // 1ms duration
        ..Default::default()
    }
}

fn make_export_request(span_count: usize, service: &str) -> ExportTraceServiceRequest {
    let now = now_ns();
    let spans: Vec<Span> = (0..span_count)
        .map(|i| make_span([1u8; 16], [(i % 250) as u8 + 1; 8], now))
        .collect();
    let resource = Resource {
        attributes: vec![KeyValue {
            key: "service.name".into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(service.into())),
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
async fn p1_otlp_grpc_to_traces_query_returns_ingested_rows() {
    // ===== Bootstrap production wiring in-process =====
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());
    let viz_state = Arc::new(VizState::new());

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    // Spawn buffer consumer drains ingest channel into DuckDB.
    let consumer_handle = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
    ));

    // Bind ingest gRPC receiver on ephemeral loopback port.
    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port succeeds");
    let bound = listener.local_addr().expect("listener exposes local_addr");

    // Per crates/ingest/tests/grpc_loopback.rs::no_non_loopback_bind_literals
    // discipline — sanity check that the test fixture is bound к loopback.
    assert!(
        bound.ip().is_loopback(),
        "test receiver must bind only to loopback; got {}",
        bound.ip()
    );

    let serve_handle = {
        let state = Arc::clone(&ingest_state);
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };

    // Best-effort wait for the server's accept loop before clients connect.
    // Matches crates/ingest/tests/grpc_loopback.rs::start_test_server discipline.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ===== Drive client via production-stack tonic client =====
    let endpoint = format!("http://{}", bound);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects к loopback receiver");
    let request = make_export_request(100, "p1-test-app");
    let response = client.export(request).await;
    assert!(
        response.is_ok(),
        "100-span export must respond OK; got {:?}",
        response.as_ref().err().map(|s| s.code())
    );
    let inner = response.unwrap().into_inner();
    // Per arch §Standard Contracts: a fully-accepted batch carries no
    // partial_success body.
    assert!(
        inner.partial_success.is_none(),
        "no partial_success on full accept"
    );

    // ===== Wait for buffer to drain channel + persist =====
    let ingested = wait_for_rows(&buffer_state, 100, Duration::from_secs(5)).await;
    assert!(
        ingested,
        "buffer should ingest 100 spans within 5s; snapshot = {:?}",
        buffer_state.snapshot()
    );

    // ===== Invoke viz::query::query_traces directly =====
    let args = TracesQueryArgs {
        time_window_seconds: 60,
        limit: 200,
        cursor: None,
    };
    let result = query_traces(&conn, &viz_state, &args).expect("query_traces succeeds");

    // ===== Assert response carries the ingested rows =====
    assert!(
        !result.items.is_empty(),
        "traces.query response must contain ingested rows; got items.len()={}, total={}",
        result.items.len(),
        result.total
    );
    assert!(
        result.total >= 100,
        "total span count should be >= 100; got {}",
        result.total
    );

    // Verify trace_ids match the injected payload — все [1u8; 16] became
    // hex-encoded "01" × 16 = 32 chars of literal "01". Sanity check that
    // round-tripped data is the data we sent, not garbage.
    let expected_trace_id = "01".repeat(16);
    assert!(
        result
            .items
            .iter()
            .any(|row| row.trace_id == expected_trace_id),
        "expected trace_id {} not found in result; got {:?}",
        expected_trace_id,
        result
            .items
            .iter()
            .take(3)
            .map(|r| r.trace_id.as_str())
            .collect::<Vec<_>>()
    );

    // Verify service.name attribute round-tripped from resource к viz row.
    assert!(
        result.items.iter().any(|row| row.service == "p1-test-app"),
        "expected service 'p1-test-app' not found; got services: {:?}",
        result
            .items
            .iter()
            .map(|r| r.service.as_str())
            .collect::<Vec<_>>()
    );

    // Cleanup — abort tasks к release the port + drop the consumer.
    serve_handle.abort();
    drop(sender);
    // consumer task exits when the sender drops + channel closes.
    let _ = tokio::time::timeout(Duration::from_secs(2), consumer_handle).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p1_negative_canary_sql_injection_in_cursor_rejected_safely() {
    // Per security plan §Input Validation row DuckDB + plan.md security
    // canary acceptance criterion — SQL injection via the cursor argument
    // MUST hit а prepared-statement boundary OR be rejected at parse step
    // before any DuckDB query runs. Schema must remain intact either way.
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("buffer schema creates");
        Arc::new(Mutex::new(raw))
    };
    let viz_state = Arc::new(VizState::new());

    let args = TracesQueryArgs {
        time_window_seconds: 60,
        limit: 50,
        cursor: Some("'; DROP TABLE spans; --".to_string()),
    };
    let result = query_traces(&conn, &viz_state, &args);

    // Either rejected at compute_window parse step (Err) OR safely returns
    // empty (Ok с items empty). Both proof that the injection didn't execute.
    match result {
        Ok(response) => {
            // Schema sanity — а follow-up query must succeed if the table
            // still exists. If DROP TABLE had executed, this would error.
            assert!(
                response.items.is_empty(),
                "injection should yield empty result; got {} items",
                response.items.len()
            );
            let recheck = query_traces(
                &conn,
                &viz_state,
                &TracesQueryArgs {
                    time_window_seconds: 60,
                    limit: 1,
                    cursor: None,
                },
            );
            assert!(
                recheck.is_ok(),
                "spans table must remain intact after injection attempt; got {:?}",
                recheck.err()
            );
        }
        Err(_) => {
            // Rejected at cursor-parse boundary; spans table still intact.
            let recheck = query_traces(
                &conn,
                &viz_state,
                &TracesQueryArgs {
                    time_window_seconds: 60,
                    limit: 1,
                    cursor: None,
                },
            );
            assert!(
                recheck.is_ok(),
                "spans table must remain intact after injection-cursor reject; got {:?}",
                recheck.err()
            );
        }
    }
}
