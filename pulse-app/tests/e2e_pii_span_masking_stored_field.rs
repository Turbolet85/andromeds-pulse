//! Span-level masking on the OTLP write boundary — the stored-field half.
//!
//! A secret embedded in a larger value is masked where it sits, and the rest of
//! the value reaches storage. This drives values carrying a keyed canary, a bare
//! provider-key canary and a Luhn-valid card through the real OTLP gRPC receiver
//! on an ephemeral loopback port and reads each DuckDB column back, per
//! architecture.md §Cross-cutting Patterns Test-time telemetry injection.
//!
//! Every canary is synthetic and non-resolvable. Assertions never echo the
//! stored value, per security-plan.md §Security Anti-Patterns Logging — a
//! failure reports length only.

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
const BARE_CANARY: &str = "sk_live_51NotARealKeyOnlyForPulseTests00"; // gitleaks:allow

/// The value half of a keyed `secret_kv` canary.
const KEYED_CANARY: &str = "Hunter2NotRealValue"; // gitleaks:allow

/// A Luhn-valid test card number (the issuer-published test PAN).
const CARD_CANARY: &str = "4111 1111 1111 1111"; // gitleaks:allow

/// The email canary carried by the second word of the service name.
const EMAIL_CANARY: &str = "owner@example.com"; // gitleaks:allow

const SERVICE_CONTEXT: &str = "spanmask-ctx-svc";

/// The keyed canary is followed by more words on its line: the keyed extent
/// runs to the end of the line, so those words are masked too.
fn embedded_value() -> String {
    format!(
        "spanmask-ctx-lead {BARE_CANARY} spanmask-ctx-mid {CARD_CANARY} spanmask-ctx-tail\n\
         spanmask-ctx-keyed password={KEYED_CANARY} rest of the keyed line"
    )
}

const EXPECTED_MASKED: &str = "spanmask-ctx-lead [REDACTED:provider_key] spanmask-ctx-mid \
     [REDACTED:credit_card] spanmask-ctx-tail\nspanmask-ctx-keyed [REDACTED:secret_kv]";

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

    fn count(&self, sql: &str) -> i64 {
        let guard = self.conn.lock().expect("conn lock");
        let mut stmt = guard.prepare(sql).expect("statement prepares");
        let mut rows = stmt.query([]).expect("query runs");
        let row = rows.next().expect("row step succeeds").expect("one row");
        row.get::<_, i64>(0).expect("column reads")
    }

    async fn wait_for_stored(&self) -> bool {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while tokio::time::Instant::now() < deadline {
            if self.count("SELECT count(*) FROM log_records") >= 1
                && self.count("SELECT count(*) FROM span_events") >= 1
                && self.count("SELECT count(*) FROM spans") >= 1
            {
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

/// Asserts a stored column masks every canary in place and keeps its context.
/// The value itself is never printed — a leak must not be re-leaked.
fn assert_span_masked(stored: &str, expected: &str, canaries: &[&str], column: &str) {
    for canary in canaries {
        assert!(
            !stored.contains(canary),
            "{column} retained a canary verbatim (value withheld); stored_len={}",
            stored.len()
        );
    }
    assert!(
        stored == expected,
        "{column} is not the span-masked form (value withheld); stored_len={}, expected_len={}",
        stored.len(),
        expected.len()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn span_mask_keeps_context_around_embedded_secrets_in_stored_columns() {
    let h = Harness::start().await;
    let value = embedded_value();
    let now = now_ns();

    let logs = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(service_resource("spanmask-log-svc")),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![LogRecord {
                    time_unix_nano: now,
                    observed_time_unix_nano: now,
                    severity_number: 17,
                    severity_text: "ERROR".into(),
                    body: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(value.clone())),
                    }),
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };
    LogsServiceClient::connect(h.endpoint.clone())
        .await
        .expect("logs client connects to loopback receiver")
        .export(logs)
        .await
        .expect("log export responds OK");

    let event = span::Event {
        time_unix_nano: now,
        name: "exception".into(),
        attributes: vec![
            string_attribute("exception.type", "AuthError"),
            string_attribute("exception.message", &value),
            string_attribute("exception.stacktrace", &value),
        ],
        dropped_attributes_count: 0,
    };
    let traces = ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(&format!(
                "{SERVICE_CONTEXT} {EMAIL_CANARY}"
            ))),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![0x5a; 16],
                    span_id: vec![0x5b; 8],
                    name: "spanmask-span".into(),
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
    TraceServiceClient::connect(h.endpoint.clone())
        .await
        .expect("trace client connects to loopback receiver")
        .export(traces)
        .await
        .expect("trace export responds OK");

    assert!(
        h.wait_for_stored().await,
        "log, span and span-event rows should all be stored within 5s; snapshot = {:?}",
        h.buffer_state.snapshot()
    );

    let canaries = [BARE_CANARY, KEYED_CANARY, CARD_CANARY];
    let body = h
        .scalar("SELECT body FROM log_records LIMIT 1")
        .expect("a log body was stored");
    assert_span_masked(&body, EXPECTED_MASKED, &canaries, "log_records.body");

    let message = h
        .scalar("SELECT exception_message FROM span_events LIMIT 1")
        .expect("an exception message was stored");
    assert_span_masked(
        &message,
        EXPECTED_MASKED,
        &canaries,
        "span_events.exception_message",
    );

    let stacktrace = h
        .scalar("SELECT exception_stacktrace FROM span_events LIMIT 1")
        .expect("an exception stacktrace was stored");
    assert_span_masked(
        &stacktrace,
        EXPECTED_MASKED,
        &canaries,
        "span_events.exception_stacktrace",
    );

    let service = h
        .scalar("SELECT service_name FROM spans LIMIT 1")
        .expect("a span service name was stored");
    assert_span_masked(
        &service,
        &format!("{SERVICE_CONTEXT} [REDACTED:email]"),
        &[EMAIL_CANARY],
        "spans.service_name",
    );

    // One increment per redacted VALUE: four values, ten masked spans. The fold
    // lands after each table's append, so wait for it rather than racing it.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while h.buffer_state.snapshot().redactions_applied < 4 && tokio::time::Instant::now() < deadline
    {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert_eq!(
        h.buffer_state.snapshot().redactions_applied,
        4,
        "redactions_applied counts redacted values, not masked spans"
    );

    h.shutdown().await;
}
