//! P6 E2E coverage — Channel pulse://stream/spans subscription + binary
//! Arrow IPC decode. Per test-plan §6 P6 canonical structure.
//!
//! Subscribes к buffer::broadcast spans Sender → drives synthetic OTLP gRPC
//! injection → awaits Arrow IPC payload from broadcast → decodes via
//! arrow::ipc::reader::StreamReader → asserts schema column names + row count.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use arrow::ipc::reader::StreamReader;
use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn make_export_request(span_count: usize) -> ExportTraceServiceRequest {
    let now = now_ns();
    let spans: Vec<Span> = (0..span_count)
        .map(|i| Span {
            trace_id: vec![2u8; 16],
            span_id: vec![(i % 250) as u8 + 1; 8],
            name: "p6-span".into(),
            kind: 0,
            start_time_unix_nano: now,
            end_time_unix_nano: now + 1_000_000,
            ..Default::default()
        })
        .collect();
    let resource = Resource {
        attributes: vec![KeyValue {
            key: "service.name".into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue("p6-test".into())),
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

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p6_channel_spans_emits_arrow_ipc_decodable_payload() {
    // Bootstrap minimal production wiring: DuckDB + buffer consumer + ingest
    // gRPC + broadcast.
    let conn = {
        let raw = Connection::open_in_memory().expect("in-memory DuckDB opens");
        create_schema(&raw).expect("schema creates");
        Arc::new(Mutex::new(raw))
    };
    let buffer_state = Arc::new(BufferState::new());
    let ingest_state = Arc::new(IngestState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    // Subscribe to spans broadcast BEFORE spawning consumer (avoid missing
    // the broadcast that fires when the first batch lands).
    let mut spans_rx = broadcast_senders.spans.subscribe();

    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);

    let _consumer = tokio::spawn(run_consumer(
        receiver,
        Arc::clone(&conn),
        Arc::clone(&buffer_state),
        Arc::clone(&broadcast_senders),
        Arc::new(ingest::observer::NoopSpanObserver),
        None,
        // Chunk #69 Phase B Session 2: drain_miner param (None = no template assignment).
        None,
    ));

    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind succeeds");
    let bound = listener.local_addr().expect("local_addr");

    let _serve = {
        let state = Arc::clone(&ingest_state);
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let _ = ingest::grpc::serve_on(listener, state, sender).await;
        })
    };
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send synthetic OTLP batch.
    let mut client = TraceServiceClient::connect(format!("http://{}", bound))
        .await
        .expect("client connects");
    client
        .export(make_export_request(50))
        .await
        .expect("50-span export OK");

    // Receive Arrow IPC bytes from spans broadcast — bounded wait.
    let bytes = tokio::time::timeout(Duration::from_secs(5), spans_rx.recv())
        .await
        .expect("broadcast received within 5s")
        .expect("Bytes payload");

    // Per arch §Standard Contracts: Real-time push contract = binary Arrow
    // IPC. Decode via StreamReader.
    let reader = StreamReader::try_new(bytes.as_ref(), None)
        .expect("Arrow IPC StreamReader decodes broadcast bytes");

    let batches: Vec<_> = reader
        .collect::<Result<Vec<_>, _>>()
        .expect("collect batches");
    assert!(
        !batches.is_empty(),
        "Arrow IPC stream should contain ≥1 RecordBatch"
    );

    let first = &batches[0];
    assert!(
        first.num_rows() > 0,
        "RecordBatch.num_rows() > 0; got {}",
        first.num_rows()
    );

    // Schema column inventory per buffer::appender::build_spans_record_batch.
    let schema = first.schema();
    let column_names: Vec<&str> = schema.fields().iter().map(|f| f.name().as_str()).collect();
    let required = [
        "trace_id",
        "span_id",
        "ts",
        "ts_unix_nano",
        "service_name",
        "end_time_unix_nano",
        "status_code",
    ];
    for col in required {
        assert!(
            column_names.contains(&col),
            "expected column '{}' in schema; got {:?}",
            col,
            column_names
        );
    }
}

#[test]
fn p6_broadcast_size_cap_invariant_holds() {
    // Per security plan §Input Validation row OTLP/HTTP + arch §Standard
    // Contracts: broadcast payload size cap is 8 MB; payloads exceeding
    // cap return BroadcastSizeCapExceeded error before re-emit. Verified
    // here via const-value assertion (chunk #23 substrate).
    assert_eq!(
        buffer::broadcast::MAX_PAYLOAD_BYTES,
        8 * 1024 * 1024,
        "broadcast size cap must remain 8 MiB"
    );
}
