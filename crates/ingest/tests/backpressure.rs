use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use ingest::channel::{IngestReceiver, IngestSender, build_channel_with_capacity};
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use prost::Message;
use reqwest::StatusCode;

#[allow(clippy::type_complexity)]
async fn start_http_server_with_small_channel(
    capacity: usize,
) -> (
    SocketAddr,
    Arc<IngestState>,
    Arc<IngestSender>,
    IngestReceiver,
    tokio::task::JoinHandle<()>,
) {
    let state = Arc::new(IngestState::new());
    let listener = ingest::http::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port must succeed");
    let bound = listener.local_addr().expect("listener has local_addr");
    let state_clone = Arc::clone(&state);
    let (sender, receiver) = build_channel_with_capacity(capacity);
    let sender = Arc::new(sender);
    let sender_clone = Arc::clone(&sender);
    let handle = tokio::spawn(async move {
        let _ = ingest::http::serve_on(listener, state_clone, sender_clone).await;
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
                    name: "test".to_string(),
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_returns_503_when_channel_saturated() {
    let capacity = 2;
    let (addr, _state, _sender, mut rx, handle) =
        start_http_server_with_small_channel(capacity).await;
    let url = format!("http://{}/v1/traces", addr);
    let body = make_request().encode_to_vec();
    let host_header = format!("127.0.0.1:{}", addr.port());

    // Fill the channel: each successful request enqueues one batch.
    for _ in 0..capacity {
        let resp = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/x-protobuf")
            .header("Host", &host_header)
            .body(body.clone())
            .send()
            .await
            .expect("first batches succeed");
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // The next request should be rejected because the channel is at capacity
    // and no consumer has drained yet.
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", &host_header)
        .body(body.clone())
        .send()
        .await
        .expect("over-capacity request reaches server");
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);

    // Drain one batch — capacity recovers — next request succeeds.
    let drained = rx.recv().await.expect("at least one batch was buffered");
    drop(drained);

    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", &host_header)
        .body(body)
        .send()
        .await
        .expect("post-drain request succeeds");
    assert_eq!(resp.status(), StatusCode::OK);

    handle.abort();
}
