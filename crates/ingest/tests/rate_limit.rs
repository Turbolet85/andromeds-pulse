//! Integration test for chunk #19 — rate-limit middleware on both
//! `:4317` (tonic) and `:4318` (axum). Drives both receivers above a
//! tight `tower_governor` rate then asserts:
//!   (a) excess requests reject with `Code::ResourceExhausted` (gRPC) or
//!       HTTP 429 (HTTP),
//!   (b) recovery after real-time replenishment.
//!
//! Real-time `tokio::time::sleep` is used because the underlying
//! `governor` crate uses a quanta-backed monotonic clock that is not
//! mocked by `tokio::time::pause` — see research.md Patterns detected
//! `crates/ingest/tests/backpressure.rs` precedent. The test windows
//! are sub-second so total wall-time impact is bounded.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use ingest::channel::{IngestReceiver, IngestSender, build_channel};
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use prost::Message;
use reqwest::StatusCode;
use tonic::Code;

const TIGHT_PERIOD: Duration = Duration::from_millis(100);
const TIGHT_BURST_SIZE: u32 = 2;
const RECOVERY_WAIT: Duration = Duration::from_millis(250);

#[allow(clippy::type_complexity)]
async fn start_grpc_server_with_tight_rate_limit() -> (
    SocketAddr,
    Arc<IngestState>,
    Arc<IngestSender>,
    IngestReceiver,
    tokio::task::JoinHandle<()>,
) {
    let state = Arc::new(IngestState::new());
    let listener = ingest::grpc::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port must succeed in test");
    let bound = listener.local_addr().expect("listener has local_addr");
    let state_clone = Arc::clone(&state);
    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);
    let sender_clone = Arc::clone(&sender);
    let handle = tokio::spawn(async move {
        let _ = ingest::grpc::serve_on_with_rate_limit(
            listener,
            state_clone,
            sender_clone,
            TIGHT_PERIOD,
            TIGHT_BURST_SIZE,
        )
        .await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, sender, receiver, handle)
}

#[allow(clippy::type_complexity)]
async fn start_http_server_with_tight_rate_limit() -> (
    SocketAddr,
    Arc<IngestState>,
    Arc<IngestSender>,
    IngestReceiver,
    tokio::task::JoinHandle<()>,
) {
    let state = Arc::new(IngestState::new());
    let listener = ingest::http::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port must succeed in test");
    let bound = listener.local_addr().expect("listener has local_addr");
    let state_clone = Arc::clone(&state);
    let (sender, receiver) = build_channel();
    let sender = Arc::new(sender);
    let sender_clone = Arc::clone(&sender);
    let handle = tokio::spawn(async move {
        let _ = ingest::http::serve_on_with_rate_limit(
            listener,
            state_clone,
            sender_clone,
            TIGHT_PERIOD,
            TIGHT_BURST_SIZE,
        )
        .await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, sender, receiver, handle)
}

fn make_request() -> ExportTraceServiceRequest {
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![1u8; 16],
                    span_id: vec![2u8; 8],
                    name: "rl-test".to_string(),
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn grpc_returns_resource_exhausted_when_rate_limit_exceeded() {
    let (addr, _state, _sender, mut rx, handle) = start_grpc_server_with_tight_rate_limit().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects");

    // Drain in background to keep mpsc capacity recovering — rate-limit
    // should be the only rejection vector under test.
    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let mut accepted = 0u32;
    let mut rate_limited = 0u32;
    for _ in 0..(TIGHT_BURST_SIZE + 1) {
        match client.export(make_request()).await {
            Ok(_) => accepted += 1,
            Err(status) if status.code() == Code::ResourceExhausted => rate_limited += 1,
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
    assert!(
        accepted >= 1,
        "at least one burst request must be accepted; got accepted={accepted}, rate_limited={rate_limited}",
    );
    assert!(
        rate_limited >= 1,
        "at least one over-burst request must be rate-limited; got accepted={accepted}, rate_limited={rate_limited}",
    );

    tokio::time::sleep(RECOVERY_WAIT).await;
    let post_recovery = client.export(make_request()).await;
    assert!(
        post_recovery.is_ok(),
        "request after rate-limit recovery window must succeed; got {post_recovery:?}",
    );

    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_returns_429_when_rate_limit_exceeded() {
    let (addr, _state, _sender, mut rx, handle) = start_http_server_with_tight_rate_limit().await;
    let url = format!("http://{}/v1/traces", addr);
    let host_header = format!("127.0.0.1:{}", addr.port());
    let body = make_request().encode_to_vec();

    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let client = reqwest::Client::new();
    let mut accepted = 0u32;
    let mut rate_limited = 0u32;
    for _ in 0..(TIGHT_BURST_SIZE + 1) {
        let resp = client
            .post(&url)
            .header("Content-Type", "application/x-protobuf")
            .header("Host", &host_header)
            .body(body.clone())
            .send()
            .await
            .expect("HTTP POST reaches loopback receiver");
        match resp.status() {
            StatusCode::OK => accepted += 1,
            StatusCode::TOO_MANY_REQUESTS => rate_limited += 1,
            other => panic!("unexpected status: {other}"),
        }
    }
    assert!(
        accepted >= 1,
        "at least one burst request must be accepted; got accepted={accepted}, rate_limited={rate_limited}",
    );
    assert!(
        rate_limited >= 1,
        "at least one over-burst request must return 429; got accepted={accepted}, rate_limited={rate_limited}",
    );

    tokio::time::sleep(RECOVERY_WAIT).await;
    let post_recovery = client
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", &host_header)
        .body(body)
        .send()
        .await
        .expect("recovery request reaches receiver");
    assert_eq!(
        post_recovery.status(),
        StatusCode::OK,
        "request after rate-limit recovery window must succeed",
    );

    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_429_includes_retry_after_header() {
    let (addr, _state, _sender, mut rx, handle) = start_http_server_with_tight_rate_limit().await;
    let url = format!("http://{}/v1/traces", addr);
    let host_header = format!("127.0.0.1:{}", addr.port());
    let body = make_request().encode_to_vec();

    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let client = reqwest::Client::new();
    // Burn through the burst quickly.
    for _ in 0..(TIGHT_BURST_SIZE + 5) {
        let resp = client
            .post(&url)
            .header("Content-Type", "application/x-protobuf")
            .header("Host", &host_header)
            .body(body.clone())
            .send()
            .await
            .expect("HTTP POST reaches loopback receiver");
        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            assert!(
                resp.headers().get("retry-after").is_some()
                    || resp.headers().get("x-ratelimit-after").is_some(),
                "rate-limit rejection must carry retry-after-style header",
            );
            handle.abort();
            return;
        }
    }
    handle.abort();
    panic!(
        "no 429 observed across burst+5 requests; tighten TIGHT_BURST_SIZE or check rate-limit wiring"
    );
}
