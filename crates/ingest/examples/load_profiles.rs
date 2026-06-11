//! Chunk #99 — live-load injector for the booted-app ACTIVE perf window.
//!
//! Drives a RUNNING pulse-app instance over the real OTLP/gRPC surface
//! (the same surface external SDKs use; arch §Cross-cutting Test-time
//! telemetry injection) at one of the dist-arch v3 load-profile rates so
//! the production observability pipeline emits the frame / memory /
//! heartbeat / L4 metrics that `cargo xtask perf:load-profiles` and the
//! `xtask/ci/{perf-slo-check,heartbeat-gap-check,l4-latency-p99}` gates
//! read from `agent-latest.jsonl`. This flips those gates from NEUTRAL to
//! ACTIVE locally — the release-gate evidence flow is `scripts/agent-run
//! boot`, then this injector, then the gate scripts over the produced log.
//!
//! Usage (app must already be running):
//!   cargo run -p ingest --example load_profiles -- baseline
//!   cargo run -p ingest --example load_profiles -- high
//!   cargo run -p ingest --example load_profiles -- burst
//!   cargo run -p ingest --example load_profiles -- sustained
//!   cargo run -p ingest --example load_profiles -- custom <spans_per_sec> <seconds>
//!
//! Port resolves via ANDROMEDA_PULSE_OTLP_GRPC_PORT (default 4317),
//! loopback only. Spans carry synthetic service names and NO attribute
//! payloads beyond service.name — nothing sensitive enters the pipeline.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};

const BATCH_SIZE: usize = 500;

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn make_batch(counter: &mut u64, service: &str) -> ExportTraceServiceRequest {
    let base = *counter;
    *counter += BATCH_SIZE as u64;
    let start_ns = now_ns();
    let mut trace_id = [1u8; 16];
    trace_id[..8].copy_from_slice(&base.to_le_bytes());
    let spans: Vec<Span> = (0..BATCH_SIZE)
        .map(|i| Span {
            trace_id: trace_id.to_vec(),
            span_id: (base + i as u64 + 1).to_le_bytes().to_vec(),
            name: "load-profile-span".to_string(),
            kind: 0,
            start_time_unix_nano: start_ns + i as u64,
            end_time_unix_nano: start_ns + i as u64 + 1_000_000,
            ..Default::default()
        })
        .collect();
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "service.name".into(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(service.into())),
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
        }],
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let profile = args.first().map(String::as_str).unwrap_or("baseline");
    let (rate, seconds, service) = match profile {
        "baseline" => (1_000u64, 60u64, "load-baseline"),
        "high" => (10_000, 300, "load-high"),
        "burst" => (50_000, 30, "load-burst"),
        "sustained" => (50_000, 300, "load-sustained"),
        "custom" => {
            let rate = args
                .get(1)
                .and_then(|s| s.parse().ok())
                .expect("custom needs <spans_per_sec>");
            let secs = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .expect("custom needs <seconds>");
            (rate, secs, "load-custom")
        }
        other => {
            eprintln!("unknown profile `{other}`; use baseline|high|burst|sustained|custom");
            std::process::exit(2);
        }
    };

    let port: u16 = std::env::var("ANDROMEDA_PULSE_OTLP_GRPC_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(4317);
    let endpoint = format!("http://127.0.0.1:{port}");
    eprintln!("load_profiles: profile={profile} rate={rate}/s duration={seconds}s -> {endpoint}");

    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("connect to running pulse-app OTLP/gRPC receiver on loopback");

    let interval = Duration::from_millis((1_000 * BATCH_SIZE as u64) / rate.max(1));
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut counter = 0u64;
    let mut accepted = 0u64;
    let mut rejected = 0u64;
    let mut tick = tokio::time::interval(interval.max(Duration::from_millis(1)));
    while Instant::now() < deadline {
        tick.tick().await;
        let req = make_batch(&mut counter, service);
        match client.export(req).await {
            Ok(_) => accepted += BATCH_SIZE as u64,
            Err(_) => rejected += 1,
        }
    }
    eprintln!(
        "load_profiles: done — accepted={accepted} spans, rejected_batches={rejected} (boundary rejections under burst backpressure are in-contract)"
    );
}
