//! Dev-only demo telemetry injector — retry-storm path.
//!
//! Short healthy warmup, then payment-service degrades: every error span
//! carries an OTLP `exception` span event with a FIXED `exception.type` +
//! `exception.stacktrace`, so the buffer computes one identical fingerprint
//! repeatedly. At ~10 error spans/s, the chunk-#66 RetryStormDetector crosses
//! its `count >= 10 / 60s` Autonomous threshold within ~1s -> RetryStorm cue
//! -> accelerated cadence -> rich digest -> L4 -> incident -> payment dot red.
//!
//! Run while the app is open:  `cargo run -p ingest --example inject_demo`

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::span::Event as SpanEvent;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{
    ResourceSpans, ScopeSpans, Span, Status,
};

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn kv(k: &str, v: &str) -> KeyValue {
    KeyValue {
        key: k.into(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue(v.into())),
        }),
    }
}

/// One fixed exception event — identical type+stack every time so the buffer
/// hashes it to one fingerprint (storm = many of the SAME fingerprint).
fn exception_event(now: u64) -> SpanEvent {
    SpanEvent {
        time_unix_nano: now,
        name: "exception".to_string(),
        attributes: vec![
            kv("exception.type", "PaymentGatewayTimeoutError"),
            kv(
                "exception.message",
                "upstream payment gateway did not respond within 1500ms",
            ),
            kv(
                "exception.stacktrace",
                "at PaymentService.charge (payment.rs:142)\n\
                 at PaymentService.call_gateway (gateway.rs:88)\n\
                 at Gateway.post (http_client.rs:301)",
            ),
        ],
        dropped_attributes_count: 0,
    }
}

struct Svc {
    name: &'static str,
    ops: &'static [&'static str],
    base_ms: u64,
    error_pct: u64,
}

const SERVICES: &[Svc] = &[
    Svc {
        name: "checkout-service",
        ops: &["GET /checkout", "POST /cart/submit"],
        base_ms: 35,
        error_pct: 0,
    },
    Svc {
        name: "payment-service",
        ops: &["charge", "refund"],
        base_ms: 2500,
        error_pct: 100,
    },
    Svc {
        name: "inventory-service",
        ops: &["reserve", "release"],
        base_ms: 20,
        error_pct: 0,
    },
    Svc {
        name: "auth-service",
        ops: &["verify-token"],
        base_ms: 12,
        error_pct: 0,
    },
    Svc {
        name: "notification-svc",
        ops: &["send-email", "send-push"],
        base_ms: 60,
        error_pct: 0,
    },
];

const WARMUP_BATCHES: u64 = 10; // ~5s healthy
const TOTAL_BATCHES: u64 = 600; // ~300s sustained

fn trace_id(seq: u64) -> [u8; 16] {
    let mut id = [0u8; 16];
    id[0..8].copy_from_slice(&seq.to_le_bytes());
    id[8..16].copy_from_slice(&seq.wrapping_mul(2_654_435_761).to_le_bytes());
    id[15] = 1;
    id
}
fn span_id(seq: u64) -> [u8; 8] {
    let mut id = seq.wrapping_add(1).to_le_bytes();
    id[7] |= 1;
    id
}

#[tokio::main]
async fn main() {
    let endpoint = "http://127.0.0.1:4317";
    println!("connecting to {endpoint} ...");
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("could not connect — is andromeda-pulse running on :4317?");
    println!("connected. PHASE 1: ~10s warmup, then payment retry-storm.\n");

    let mut total: u64 = 0;
    let mut exc: u64 = 0;
    let mut announced = false;

    for b in 0..TOTAL_BATCHES {
        let in_spike = b >= WARMUP_BATCHES;
        if in_spike && !announced {
            announced = true;
            println!("\n>>> PHASE 2: payment-service throwing PaymentGatewayTimeoutError <<<\n");
        }
        let now = now_ns();
        let mut resource_spans = Vec::new();

        for (si, svc) in SERVICES.iter().enumerate() {
            let eff_err = if in_spike { svc.error_pct } else { 0 };
            let eff_base = if in_spike {
                svc.base_ms
            } else {
                svc.base_ms.min(40)
            };

            let mut spans = Vec::new();
            for (oi, op) in svc.ops.iter().enumerate() {
                for k in 0..3u64 {
                    let seq = b * 100_000 + (si as u64) * 1_000 + (oi as u64) * 100 + k;
                    let jitter = (seq * 7) % eff_base.max(1);
                    let dur_ns = (eff_base + jitter) * 1_000_000;
                    let is_err = eff_err > 0 && (seq % 100) < eff_err;
                    let events = if is_err {
                        exc += 1;
                        vec![exception_event(now)]
                    } else {
                        Vec::new()
                    };
                    spans.push(Span {
                        trace_id: trace_id(seq).to_vec(),
                        span_id: span_id(seq).to_vec(),
                        name: (*op).to_string(),
                        kind: 2,
                        start_time_unix_nano: now.saturating_sub(dur_ns),
                        end_time_unix_nano: now,
                        events,
                        status: Some(Status {
                            code: if is_err { 2 } else { 1 },
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }
            }
            total += spans.len() as u64;
            resource_spans.push(ResourceSpans {
                resource: Some(Resource {
                    attributes: vec![kv("service.name", svc.name)],
                    dropped_attributes_count: 0,
                }),
                scope_spans: vec![ScopeSpans {
                    scope: None,
                    spans,
                    schema_url: String::new(),
                }],
                schema_url: String::new(),
            });
        }

        match client
            .export(ExportTraceServiceRequest { resource_spans })
            .await
        {
            Ok(_) => {
                if b % 10 == 0 {
                    let phase = if in_spike { "STORM" } else { "warmup" };
                    println!(
                        "  [{phase}] batch {:>3}/{TOTAL_BATCHES}  ({total} spans, {exc} exceptions)",
                        b + 1
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "export failed at batch {b}: {} ({})",
                    e.code() as i32,
                    e.message()
                );
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    println!(
        "\nDONE — {total} spans, {exc} identical-fingerprint exceptions. Watch the payment dot."
    );
}
