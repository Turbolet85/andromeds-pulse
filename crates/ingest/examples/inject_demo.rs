//! Dev-only demo telemetry injector — retry-storm path.
//!
//! Short healthy warmup, then payment-service degrades: every error span
//! carries an OTLP `exception` span event with a FIXED `exception.type` +
//! `exception.stacktrace`, so the buffer computes one identical fingerprint
//! repeatedly. At ~10 error spans/s, the chunk-#66 RetryStormDetector crosses
//! its `count >= 10 / 60s` Autonomous threshold within ~1s -> RetryStorm cue
//! -> accelerated cadence -> rich digest -> L4 -> incident -> payment dot red.
//!
//! # Supported invocation
//!
//! BUILD OUTSIDE the timed section, then run the binary BY PATH:
//!
//! ```text
//! cargo build -p ingest --example inject_demo
//! ./target/debug/examples/inject_demo[.exe] [--sustained]
//! ```
//!
//! `cargo run --example inject_demo` inside a timed section measures the
//! COMPILER, not the pipeline: measured 2026-08-16, a post-`cargo clean` cache
//! spent the whole budget building `rustls` + `h2`, was killed at the boundary
//! and sent ZERO spans while the app looked healthy — indistinguishable from a
//! broken gate. `xtask::webview_drive` already builds-then-passes-the-path via
//! `PULSE_INJECTOR`; use that same shape.
//!
//! Any recipe driving this tool must assert the FEED PRECONDITION —
//! `rows_ingested > 0` on `buffer.tick` / the `ready` envelope — BEFORE judging
//! any downstream counter, so "the producer never ran" stays distinguishable
//! from "the feature is broken" (obs-plan §1 Heartbeat ticks).
//!
//! # Profiles
//!
//! With NO arguments the default profile is unchanged — a finite ~600-batch
//! 100%-error storm. That default is load-bearing: `webview_drive` threads no
//! arguments, so its storm-stage budget is calibrated against it.
//!
//! `--sustained` runs UNBOUNDED at a moderate error rate instead. A finite
//! storm exits, and the traces view then ages out against its 60s window, so a
//! delayed operator sees "No traces yet" on a healthy boot; and a 100% firehose
//! drives cadence into tier1, where the LWW queue drops digests before L4 can
//! consume them (~4s/inference).

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
const BATCH_INTERVAL: Duration = Duration::from_millis(500);
const SUSTAINED_ERROR_PCT: u64 = 20;
const BACKPRESSURE_BACKOFF: Duration = Duration::from_secs(2);
const MAX_CONSECUTIVE_FAILURES: u32 = 10;

/// Emission profile. `total_batches: None` runs until interrupted.
struct Profile {
    label: &'static str,
    total_batches: Option<u64>,
    error_pct_override: Option<u64>,
    /// Whether a failed export is retried rather than ending the run. Only
    /// the sustained profile sets this — the bounded storm fails fast so a
    /// broken receiver surfaces immediately.
    resilient: bool,
}

impl Profile {
    fn storm() -> Self {
        Self {
            label: "storm",
            total_batches: Some(TOTAL_BATCHES),
            error_pct_override: None,
            resilient: false,
        }
    }

    fn sustained() -> Self {
        Self {
            label: "sustained",
            total_batches: None,
            error_pct_override: Some(SUSTAINED_ERROR_PCT),
            resilient: true,
        }
    }

    /// The error percentage in force for `svc`. A service configured healthy
    /// stays healthy under every profile — the override only gentles the
    /// service that is meant to degrade.
    fn error_pct_for(&self, svc: &Svc) -> u64 {
        match self.error_pct_override {
            Some(pct) if svc.error_pct > 0 => pct,
            _ => svc.error_pct,
        }
    }

    fn budget_label(&self) -> String {
        match self.total_batches {
            Some(n) => n.to_string(),
            None => "unbounded".to_string(),
        }
    }
}

/// Parses the profile from argv. Unknown flags are rejected rather than
/// ignored — a silently-dropped flag is the same false-negative class the
/// module doc's invocation guidance exists to remove.
fn parse_profile<I: Iterator<Item = String>>(args: I) -> Result<Profile, String> {
    let mut profile = Profile::storm();
    let mut minutes: Option<u64> = None;
    let mut error_pct: Option<u64> = None;

    for arg in args {
        if arg == "--sustained" {
            profile = Profile::sustained();
        } else if let Some(v) = arg.strip_prefix("--minutes=") {
            minutes = Some(v.parse().map_err(|_| format!("bad --minutes value: {v}"))?);
        } else if let Some(v) = arg.strip_prefix("--error-pct=") {
            let pct: u64 = v
                .parse()
                .map_err(|_| format!("bad --error-pct value: {v}"))?;
            if pct > 100 {
                return Err(format!("--error-pct must be 0..=100, got {pct}"));
            }
            error_pct = Some(pct);
        } else {
            return Err(format!("unknown argument: {arg}"));
        }
    }

    if let Some(m) = minutes {
        let per_minute = 60_000 / BATCH_INTERVAL.as_millis().max(1) as u64;
        profile.total_batches = Some(m.saturating_mul(per_minute).max(1));
    }
    if let Some(pct) = error_pct {
        profile.error_pct_override = Some(pct);
    }
    Ok(profile)
}

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

/// Deterministic 0..=99 roll deciding whether `seq` errors. Spread across the
/// whole seq space rather than `seq % 100`, whose low digits only ever take the
/// three per-op values — so a percentage between 1 and 99 would round to
/// all-or-nothing. Identical to the old predicate at the two values the default
/// profile uses (0 short-circuits; 100 admits every roll).
fn error_roll(seq: u64) -> u64 {
    (seq.wrapping_mul(2_654_435_761) >> 8) % 100
}

#[tokio::main]
async fn main() {
    let profile = match parse_profile(std::env::args().skip(1)) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("inject_demo: {e}");
            eprintln!(
                "usage: inject_demo [--sustained] [--minutes=N] [--error-pct=0..100]\n\
                 no arguments = the default finite storm profile"
            );
            std::process::exit(2);
        }
    };

    let endpoint = "http://127.0.0.1:4317";
    println!("connecting to {endpoint} ...");
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("could not connect — is andromeda-pulse running on :4317?");
    println!(
        "connected. profile={} batches={} PHASE 1: ~5s warmup, then payment degradation.\n",
        profile.label,
        profile.budget_label()
    );

    let mut total: u64 = 0;
    let mut exc: u64 = 0;
    let mut announced = false;
    let mut consecutive_failures: u32 = 0;

    let mut b: u64 = 0;
    while profile.total_batches.is_none_or(|n| b < n) {
        let in_spike = b >= WARMUP_BATCHES;
        if in_spike && !announced {
            announced = true;
            println!("\n>>> PHASE 2: payment-service throwing PaymentGatewayTimeoutError <<<\n");
        }
        let now = now_ns();
        let mut resource_spans = Vec::new();

        for (si, svc) in SERVICES.iter().enumerate() {
            let eff_err = if in_spike {
                profile.error_pct_for(svc)
            } else {
                0
            };
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
                    let is_err = eff_err > 0 && error_roll(seq) < eff_err;
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
                consecutive_failures = 0;
                if b % 10 == 0 {
                    let phase = if in_spike { "STORM" } else { "warmup" };
                    println!(
                        "  [{phase}] batch {:>3}/{}  ({total} spans, {exc} exceptions)",
                        b + 1,
                        profile.budget_label()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "export failed at batch {b}: {} ({})",
                    e.code() as i32,
                    e.message()
                );
                // A sustained run exists to OUTLIVE the operator's arrival, so
                // backpressure must not end it: the receiver returns
                // ResourceExhausted when its ingest channel is momentarily
                // full, which is a transient condition the next batch usually
                // clears. Measured 2026-08-25: an unbounded run ended at batch
                // 1025 on exactly this, leaving the traces view to age out —
                // the failure mode the profile was added to remove. A
                // consecutive-failure ceiling still terminates a genuinely
                // dead receiver. The bounded storm profile keeps failing fast.
                consecutive_failures += 1;
                if !profile.resilient || consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                    break;
                }
                tokio::time::sleep(BACKPRESSURE_BACKOFF).await;
            }
        }
        tokio::time::sleep(BATCH_INTERVAL).await;
        b += 1;
    }

    println!(
        "\nDONE — {total} spans, {exc} identical-fingerprint exceptions. Watch the payment dot."
    );
}
