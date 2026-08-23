//! Live-leg producer for ingestion scrub coverage.
//!
//! Puts one credential-shaped canary into each of the four client-controlled
//! DuckDB columns that previously reached storage unscrubbed
//! (`spans.service_name`, `span_events.name`, `metrics_points.metric_name`,
//! `log_records.severity_text`), and alongside each one a legitimate control
//! value, so a single run observes BOTH halves — the canary must not be stored
//! verbatim, and the control must survive byte-identical.
//!
//! Build it OUTSIDE any timed section, then invoke the binary by path — a
//! `cargo run` inside a timeout spends the budget compiling and sends nothing.
//!
//! Run while the app is open:
//!   cargo build -p ingest --example inject_scrub_canaries
//!   ./target/debug/examples/inject_scrub_canaries

use std::time::{SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::logs_service_client::LogsServiceClient;
use ingest::grpc::proto::opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{LogRecord, ResourceLogs, ScopeLogs};
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    Gauge, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics, metric, number_data_point,
};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span, span};

/// Bare provider-key shape — matches the catalog's eighth (`provider_key`) arm,
/// which is the arm the four key-name-anchored arms cannot reach.
const CANARY: &str = "sk_live_51NotARealKeyOnlyForPulseTests00"; // gitleaks:allow

const CONTROL_SERVICE: &str = "scrub-canary-control";
const CONTROL_EVENT: &str = "exception";
const CONTROL_METRIC: &str = "scrub.canary.control.total";
const CONTROL_SEVERITY: &str = "ERROR";
const SEVERITY_ERROR: i32 = 17;

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn resource(service: &str) -> Resource {
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

fn span_with_event(trace_seed: u8, ts: u64, event_name: &str) -> Span {
    Span {
        trace_id: vec![trace_seed; 16],
        span_id: vec![trace_seed; 8],
        trace_state: String::new(),
        parent_span_id: Vec::new(),
        flags: 0,
        name: "scrub-canary-span".into(),
        kind: 1,
        start_time_unix_nano: ts,
        end_time_unix_nano: ts + 1_000_000,
        attributes: Vec::new(),
        dropped_attributes_count: 0,
        events: vec![span::Event {
            time_unix_nano: ts + 1,
            name: event_name.into(),
            attributes: Vec::new(),
            dropped_attributes_count: 0,
        }],
        dropped_events_count: 0,
        links: Vec::new(),
        dropped_links_count: 0,
        status: None,
    }
}

fn gauge_metric(name: &str, ts: u64, value: i64) -> Metric {
    Metric {
        name: name.into(),
        description: String::new(),
        unit: String::new(),
        metadata: Vec::new(),
        data: Some(metric::Data::Gauge(Gauge {
            data_points: vec![NumberDataPoint {
                attributes: Vec::new(),
                start_time_unix_nano: 0,
                time_unix_nano: ts,
                exemplars: Vec::new(),
                flags: 0,
                value: Some(number_data_point::Value::AsInt(value)),
            }],
        })),
    }
}

fn log_record(ts: u64, severity_text: &str, body: &str) -> LogRecord {
    LogRecord {
        time_unix_nano: ts,
        observed_time_unix_nano: ts,
        severity_number: SEVERITY_ERROR,
        severity_text: severity_text.into(),
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
    let ts = now_ns();

    // spans.service_name (canary resource) + span_events.name (canary event),
    // each paired with a control emitted under a legitimate identity.
    let mut traces = TraceServiceClient::connect(endpoint)
        .await
        .expect("could not connect — is andromeda-pulse running on :4317?");
    traces
        .export(ExportTraceServiceRequest {
            resource_spans: vec![
                ResourceSpans {
                    resource: Some(resource(CANARY)),
                    scope_spans: vec![ScopeSpans {
                        scope: None,
                        spans: vec![span_with_event(0xA1, ts, CANARY)],
                        schema_url: String::new(),
                    }],
                    schema_url: String::new(),
                },
                ResourceSpans {
                    resource: Some(resource(CONTROL_SERVICE)),
                    scope_spans: vec![ScopeSpans {
                        scope: None,
                        spans: vec![span_with_event(0xB2, ts, CONTROL_EVENT)],
                        schema_url: String::new(),
                    }],
                    schema_url: String::new(),
                },
            ],
        })
        .await
        .expect("trace export must be accepted");

    // metrics_points.metric_name — the only target inside a primary key.
    let mut metrics = MetricsServiceClient::connect(endpoint)
        .await
        .expect("metrics connect");
    metrics
        .export(ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(resource(CONTROL_SERVICE)),
                scope_metrics: vec![ScopeMetrics {
                    scope: None,
                    metrics: vec![
                        gauge_metric(CANARY, ts, 1),
                        gauge_metric(CONTROL_METRIC, ts, 2),
                    ],
                    schema_url: String::new(),
                }],
                schema_url: String::new(),
            }],
        })
        .await
        .expect("metrics export must be accepted");

    // log_records.severity_text.
    let mut logs = LogsServiceClient::connect(endpoint)
        .await
        .expect("logs connect");
    logs.export(ExportLogsServiceRequest {
        resource_logs: vec![ResourceLogs {
            resource: Some(resource(CONTROL_SERVICE)),
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records: vec![
                    log_record(ts, CANARY, "scrub canary in severity_text"),
                    log_record(ts + 1, CONTROL_SEVERITY, "scrub canary control record"),
                ],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    })
    .await
    .expect("logs export must be accepted");

    println!("sent canaries at ts_unix_nano={ts}");
    println!("  spans.service_name          canary={CANARY:?} control={CONTROL_SERVICE:?}");
    println!("  span_events.name            canary={CANARY:?} control={CONTROL_EVENT:?}");
    println!("  metrics_points.metric_name  canary={CANARY:?} control={CONTROL_METRIC:?}");
    println!("  log_records.severity_text   canary={CANARY:?} control={CONTROL_SEVERITY:?}");
    println!("none of the four canaries may be stored verbatim; every control must survive as-is.");
}
