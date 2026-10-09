//! Bare-credential recall coverage for P-047 — the stored-field half.
//!
//! The four credential arms of the scrubber catalog are key-name-anchored, so
//! a standalone provider token carries no `api_key=` / `password=` / `Bearer `
//! prefix to match on. This drives such a token through the real OTLP gRPC
//! receiver on an ephemeral loopback port and reads the value back out of the
//! DuckDB column it lands in, per architecture.md §Cross-cutting Patterns
//! Test-time telemetry injection (no in-process bypass, no mock channel).
//!
//! Both canaries are synthetic and non-resolvable. Assertions never echo the
//! stored value, per security-plan.md §Security Anti-Patterns Logging — a
//! failure reports length and prefix only.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer};
use duckdb::Connection;
use ingest::channel::build_channel;
use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::logs_service_client::LogsServiceClient;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span, span};
use ingest::state::IngestState;

/// Synthetic Stripe-shaped secret key — never issued, never valid.
const BARE_LIVE_KEY: &str = "sk_live_51NotARealKeyOnlyForPulseTests00"; // gitleaks:allow

/// Synthetic OpenAI-project-shaped key — never issued, never valid.
const BARE_PROJ_KEY: &str = "sk-proj-NotARealKeyOnlyForPulseTests0000"; // gitleaks:allow

const REDACTION_PREFIX: &str = "[REDACTED:";

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn service_resource(service: &str) -> Resource {
    Resource {
        attributes: vec![KeyValue {
            key: "service.name".into(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(service.into())),
            }),
        }],
        dropped_attributes_count: 0,
    }
}

fn string_attribute(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.into(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(value.into())),
        }),
    }
}

struct Harness {
    conn: Arc<Mutex<Connection>>,
    buffer_state: Arc<BufferState>,
    endpoint: String,
    sender: Option<Arc<ingest::channel::IngestSender>>,
    serve_handle: tokio::task::JoinHandle<()>,
    consumer_handle: tokio::task::JoinHandle<()>,
}

impl Harness {
    async fn start() -> Self {
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

        let serve_handle = {
            let state = Arc::clone(&ingest_state);
            let sender = Arc::clone(&sender);
            tokio::spawn(async move {
                let _ = ingest::grpc::serve_on(listener, state, sender).await;
            })
        };

        tokio::time::sleep(Duration::from_millis(100)).await;

        Self {
            conn,
            buffer_state,
            endpoint: format!("http://{}", bound),
            sender: Some(sender),
            serve_handle,
            consumer_handle,
        }
    }

    async fn wait_for_rows(&self, target: u64) -> bool {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while tokio::time::Instant::now() < deadline {
            if self.buffer_state.snapshot().rows_ingested >= target {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        false
    }

    fn scalar(&self, sql: &str) -> Option<String> {
        let guard = self.conn.lock().expect("conn lock");
        let mut stmt = guard.prepare(sql).expect("statement prepares");
        let mut rows = stmt.query([]).expect("query runs");
        let row = rows.next().expect("row step succeeds")?;
        row.get::<_, Option<String>>(0).expect("column reads")
    }

    async fn shutdown(mut self) {
        self.serve_handle.abort();
        drop(self.sender.take());
        let _ = tokio::time::timeout(Duration::from_secs(2), self.consumer_handle).await;
    }
}

/// Asserts a stored column no longer carries the canary. The value itself is
/// never printed — a leak must not be re-leaked into test output.
fn assert_redacted(stored: &str, canary: &str, column: &str) {
    assert!(
        !stored.contains(canary),
        "{column} retained the bare credential verbatim (value withheld); stored_len={}, starts_with_marker={}",
        stored.len(),
        stored.starts_with(REDACTION_PREFIX)
    );
    assert!(
        stored.starts_with(REDACTION_PREFIX),
        "{column} was not replaced by a redaction marker (value withheld); stored_len={}",
        stored.len()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn bare_provider_key_in_log_body_is_redacted_before_storage() {
    let h = Harness::start().await;

    let request = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(service_resource("pii-recall-test")),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: now_ns(),
                    observed_time_unix_nano: now_ns(),
                    severity_number: 9,
                    severity_text: "INFO".into(),
                    body: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(BARE_LIVE_KEY.into())),
                    }),
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };

    let mut client = LogsServiceClient::connect(h.endpoint.clone())
        .await
        .expect("logs client connects to loopback receiver");
    client
        .export(request)
        .await
        .expect("log export responds OK");

    assert!(
        h.wait_for_rows(1).await,
        "buffer should ingest the log record within 5s; snapshot = {:?}",
        h.buffer_state.snapshot()
    );

    let stored = h
        .scalar("SELECT body FROM log_records ORDER BY ts_unix_nano DESC LIMIT 1")
        .expect("a log record row was stored");

    assert_redacted(&stored, BARE_LIVE_KEY, "log_records.body");

    // CARRY #10 — the redaction is gradeable from outside the process, not
    // only by reading the stored row back.
    assert!(
        h.buffer_state.snapshot().redactions_applied >= 1,
        "redactions_applied must advance when the log path redacts; snapshot = {:?}",
        h.buffer_state.snapshot()
    );

    h.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn bare_provider_key_in_span_event_exception_message_is_redacted_before_storage() {
    let h = Harness::start().await;

    let now = now_ns();
    let event = span::Event {
        time_unix_nano: now,
        name: "exception".into(),
        attributes: vec![
            string_attribute("exception.type", "AuthError"),
            string_attribute("exception.message", BARE_PROJ_KEY),
        ],
        dropped_attributes_count: 0,
    };
    let request = ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource("pii-recall-test")),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![7u8; 16],
                    span_id: vec![9u8; 8],
                    name: "pii-recall-span".into(),
                    kind: 0,
                    start_time_unix_nano: now,
                    end_time_unix_nano: now + 1_000_000,
                    events: vec![event],
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };

    let mut client = TraceServiceClient::connect(h.endpoint.clone())
        .await
        .expect("trace client connects to loopback receiver");
    client
        .export(request)
        .await
        .expect("trace export responds OK");

    assert!(
        h.wait_for_rows(1).await,
        "buffer should ingest the span within 5s; snapshot = {:?}",
        h.buffer_state.snapshot()
    );

    let stored = h
        .scalar("SELECT exception_message FROM span_events ORDER BY ts_unix_nano DESC LIMIT 1")
        .expect("a span_events row with an exception message was stored");

    assert_redacted(&stored, BARE_PROJ_KEY, "span_events.exception_message");

    // CARRY #10 — the span-event path folds into the same counter as the log
    // path, so a reader of `buffer.tick` sees both.
    assert!(
        h.buffer_state.snapshot().redactions_applied >= 1,
        "redactions_applied must advance when the span-event path redacts; snapshot = {:?}",
        h.buffer_state.snapshot()
    );

    h.shutdown().await;
}
