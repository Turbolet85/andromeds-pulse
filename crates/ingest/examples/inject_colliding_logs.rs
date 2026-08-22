//! Live-leg producer for the log-record identity fix.
//!
//! Emits two DISTINCT log records that share `time_unix_nano`, resource, and
//! `severity_number` — the ordinary same-tick INFO burst that collided on the
//! pre-fix primary key `(ts_unix_nano, resource_hash, severity_number)`. Both
//! must survive ingestion and be readable back.
//!
//! Build it OUTSIDE any timed section, then invoke the binary by path — a
//! `cargo run` inside a timeout spends the budget compiling and sends nothing.
//!
//! Run while the app is open:
//!   cargo build -p ingest --example inject_colliding_logs
//!   ./target/debug/examples/inject_colliding_logs

use std::time::{SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::logs_service_client::LogsServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;

const SERVICE: &str = "collision-probe";
const SEVERITY_INFO: i32 = 9;

/// Distinct enough to tell the two rows apart on read-back, and carrying no
/// credential shape that the scrubber would redact.
const FIRST_BODY: &str = "collision probe record ALPHA";
const SECOND_BODY: &str = "collision probe record BRAVO";

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn log_record(ts: u64, body: &str) -> LogRecord {
    LogRecord {
        time_unix_nano: ts,
        observed_time_unix_nano: ts,
        severity_number: SEVERITY_INFO,
        severity_text: "INFO".into(),
        body: Some(AnyValue {
            value: Some(any_value::Value::StringValue(body.into())),
        }),
        attributes: Vec::new(),
        dropped_attributes_count: 0,
        flags: 0,
        trace_id: Vec::new(),
        span_id: Vec::new(),
        event_name: String::new(),
    }
}

#[tokio::main]
async fn main() {
    let endpoint = "http://127.0.0.1:4317";
    println!("connecting to {endpoint} ...");
    let mut client = LogsServiceClient::connect(endpoint)
        .await
        .expect("could not connect — is andromeda-pulse running on :4317?");

    // One timestamp for both records is the whole point: they collide on every
    // pre-fix key column and are separated only by the `seq` ordinal.
    let ts = now_ns();

    let request = ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "service.name".into(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(SERVICE.into())),
                    }),
                }],
                dropped_attributes_count: 0,
            }),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![log_record(ts, FIRST_BODY), log_record(ts, SECOND_BODY)],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };

    client
        .export(request)
        .await
        .expect("export must be accepted by the receiver");

    println!(
        "sent 2 log records at ts_unix_nano={ts}, severity={SEVERITY_INFO}, service={SERVICE}"
    );
    println!("bodies: {FIRST_BODY:?} / {SECOND_BODY:?}");
    println!("both must be readable back; pre-fix the batch was rejected whole.");
}
