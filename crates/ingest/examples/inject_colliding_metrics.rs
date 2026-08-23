//! Live-leg producer for the metric-point identity fix.
//!
//! Emits BOTH collision sources in one export, at one `time_unix_nano` from one
//! resource:
//!
//! (a) LABEL-SET — one metric carrying two data points that differ only by their
//!     attribute set. `push_metric_row` never reads `p.attributes`, so the two
//!     rows are identical on every pre-fix key column.
//! (b) REDACTED-NAME — two DISTINCT provider-key-shaped metric names. The
//!     scrubber's `Redacted { category }` carries no content, so both render
//!     `[REDACTED:provider_key]` and collide on `metric_name`.
//!
//! A control metric rides along: it must survive in the same batch, so the GREEN
//! leg proves selectivity rather than blanket survival. Pre-fix it dies too —
//! the loss is whole-batch, not per-row.
//!
//! Build it OUTSIDE any timed section, then invoke the binary by path — a
//! `cargo run` inside a timeout spends the budget compiling and sends nothing.
//!
//! Run while the app is open:
//!   cargo build -p ingest --example inject_colliding_metrics
//!   ./target/debug/examples/inject_colliding_metrics

use std::time::{SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    Gauge, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics, metric, number_data_point,
};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;

const SERVICE: &str = "metric-collision-probe";

/// (a) One metric, two label-differing points. The name itself is clean — the
/// scrubber must leave it alone, or the redaction count stops being readable.
const LABELLED_METRIC: &str = "collision.probe.requests";

/// (b) Two DISTINCT bare provider-key shapes. Both match the catalog's eighth
/// arm and redact to the same category-only placeholder.
const CREDENTIAL_NAME_ALPHA: &str = "sk_live_51AlphaNotARealKeyForPulseTests00"; // gitleaks:allow
const CREDENTIAL_NAME_BRAVO: &str = "sk_live_51BravoNotARealKeyForPulseTests00"; // gitleaks:allow

/// Rides in the same batch and must land: distinct name, so it never collides.
const CONTROL_METRIC: &str = "collision.probe.control";

const VALUE_ROUTE_A: i64 = 11;
const VALUE_ROUTE_B: i64 = 22;
const VALUE_CREDENTIAL_ALPHA: i64 = 33;
const VALUE_CREDENTIAL_BRAVO: i64 = 44;
const VALUE_CONTROL: i64 = 55;

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

fn point(ts: u64, value: i64, label: Option<(&str, &str)>) -> NumberDataPoint {
    NumberDataPoint {
        attributes: label
            .map(|(k, v)| {
                vec![KeyValue {
                    key: k.into(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(v.into())),
                    }),
                }]
            })
            .unwrap_or_default(),
        start_time_unix_nano: 0,
        time_unix_nano: ts,
        exemplars: Vec::new(),
        flags: 0,
        value: Some(number_data_point::Value::AsInt(value)),
    }
}

fn gauge(name: &str, data_points: Vec<NumberDataPoint>) -> Metric {
    Metric {
        name: name.into(),
        description: String::new(),
        unit: String::new(),
        metadata: Vec::new(),
        data: Some(metric::Data::Gauge(Gauge { data_points })),
    }
}

#[tokio::main]
async fn main() {
    let endpoint = "http://127.0.0.1:4317";
    println!("connecting to {endpoint} ...");
    let mut client = MetricsServiceClient::connect(endpoint)
        .await
        .expect("could not connect — is andromeda-pulse running on :4317?");

    // One timestamp for every point is the whole point: they collide on every
    // pre-fix key column and are separated only by the `seq` ordinal.
    let ts = now_ns();

    let request = ExportMetricsServiceRequest {
        resource_metrics: vec![ResourceMetrics {
            resource: Some(resource(SERVICE)),
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                metrics: vec![
                    gauge(
                        LABELLED_METRIC,
                        vec![
                            point(ts, VALUE_ROUTE_A, Some(("http.route", "/alpha"))),
                            point(ts, VALUE_ROUTE_B, Some(("http.route", "/bravo"))),
                        ],
                    ),
                    gauge(
                        CREDENTIAL_NAME_ALPHA,
                        vec![point(ts, VALUE_CREDENTIAL_ALPHA, None)],
                    ),
                    gauge(
                        CREDENTIAL_NAME_BRAVO,
                        vec![point(ts, VALUE_CREDENTIAL_BRAVO, None)],
                    ),
                    gauge(CONTROL_METRIC, vec![point(ts, VALUE_CONTROL, None)]),
                ],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };

    client
        .export(request)
        .await
        .expect("export must be accepted by the receiver");

    println!("sent 5 metric points at ts_unix_nano={ts}, service={SERVICE}");
    println!("  label pair : {LABELLED_METRIC} values {VALUE_ROUTE_A}/{VALUE_ROUTE_B}");
    println!(
        "  redacted   : 2 distinct provider-key names, values {VALUE_CREDENTIAL_ALPHA}/{VALUE_CREDENTIAL_BRAVO}"
    );
    println!("  control    : {CONTROL_METRIC} value {VALUE_CONTROL}");
    println!("all 5 must be readable back; pre-fix the batch was rejected whole.");
}
