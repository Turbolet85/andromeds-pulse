use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use ingest::channel::{IngestReceiver, IngestSender, build_channel};
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use prost::Message;
use reqwest::StatusCode;
use tonic::Code;

#[allow(clippy::type_complexity)]
async fn start_test_grpc_server() -> (
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
        let _ = ingest::grpc::serve_on(listener, state_clone, sender_clone).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, sender, receiver, handle)
}

#[allow(clippy::type_complexity)]
async fn start_test_http_server() -> (
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
        let _ = ingest::http::serve_on(listener, state_clone, sender_clone).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, sender, receiver, handle)
}

fn make_span(trace_id: Vec<u8>, span_id: Vec<u8>) -> Span {
    Span {
        trace_id,
        span_id,
        name: "test-span".to_string(),
        kind: 0,
        ..Default::default()
    }
}

fn wrap_request(span: Span) -> ExportTraceServiceRequest {
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![span],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn trace_id_wrong_length_rejects_grpc_with_invalid_argument() {
    let (addr, state, _sender, _rx, handle) = start_test_grpc_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects");
    let req = wrap_request(make_span(vec![1u8; 8], vec![2u8; 8]));
    let result = client.export(req).await;
    let status = result.expect_err("invalid trace_id must be rejected");
    assert_eq!(status.code(), Code::InvalidArgument);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn span_id_wrong_length_rejects_grpc_with_invalid_argument() {
    let (addr, state, _sender, _rx, handle) = start_test_grpc_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects");
    let req = wrap_request(make_span(vec![1u8; 16], vec![2u8; 4]));
    let result = client.export(req).await;
    let status = result.expect_err("invalid span_id must be rejected");
    assert_eq!(status.code(), Code::InvalidArgument);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn attribute_count_over_bound_rejects_grpc() {
    let (addr, state, _sender, _rx, handle) = start_test_grpc_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects");
    let mut span = make_span(vec![1u8; 16], vec![2u8; 8]);
    span.attributes = (0..200)
        .map(|i| KeyValue {
            key: format!("k{i}"),
            value: None,
        })
        .collect();
    let result = client.export(wrap_request(span)).await;
    let status = result.expect_err("attribute count over bound must be rejected");
    assert_eq!(status.code(), Code::InvalidArgument);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn valid_payload_records_spans_through_channel_grpc() {
    let (addr, state, _sender, mut rx, handle) = start_test_grpc_server().await;
    let endpoint = format!("http://{}", addr);
    let mut client = TraceServiceClient::connect(endpoint)
        .await
        .expect("client connects");
    let req = wrap_request(make_span(vec![1u8; 16], vec![2u8; 8]));
    let resp = client.export(req).await.expect("valid payload accepted");
    assert!(resp.into_inner().partial_success.is_none());
    assert_eq!(state.snapshot().span_count, 1);
    let batch = rx.recv().await.expect("channel received batch");
    match batch {
        ingest::channel::Batch::Spans(_) => {}
        other => panic!("expected Spans batch, got {other:?}"),
    }
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn trace_id_wrong_length_rejects_http_with_400() {
    let (addr, state, _sender, _rx, handle) = start_test_http_server().await;
    let url = format!("http://{}/v1/traces", addr);
    let req = wrap_request(make_span(vec![1u8; 8], vec![2u8; 8]));
    let body = req.encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", format!("127.0.0.1:{}", addr.port()))
        .body(body)
        .send()
        .await
        .expect("HTTP request completes");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn span_id_wrong_length_rejects_http_with_400() {
    let (addr, state, _sender, _rx, handle) = start_test_http_server().await;
    let url = format!("http://{}/v1/traces", addr);
    let req = wrap_request(make_span(vec![1u8; 16], vec![2u8; 4]));
    let body = req.encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", format!("127.0.0.1:{}", addr.port()))
        .body(body)
        .send()
        .await
        .expect("HTTP request completes");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn attribute_key_over_bound_rejects_http() {
    let (addr, state, _sender, _rx, handle) = start_test_http_server().await;
    let url = format!("http://{}/v1/traces", addr);
    let mut span = make_span(vec![1u8; 16], vec![2u8; 8]);
    span.attributes = vec![KeyValue {
        key: "x".repeat(300),
        value: None,
    }];
    let body = wrap_request(span).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", format!("127.0.0.1:{}", addr.port()))
        .body(body)
        .send()
        .await
        .expect("HTTP request completes");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn attribute_value_over_bound_rejects_http() {
    let (addr, state, _sender, _rx, handle) = start_test_http_server().await;
    let url = format!("http://{}/v1/traces", addr);
    let mut span = make_span(vec![1u8; 16], vec![2u8; 8]);
    span.attributes = vec![KeyValue {
        key: "k".to_string(),
        value: Some(AnyValue {
            value: Some(any_value::Value::StringValue("v".repeat(5000))),
        }),
    }];
    let body = wrap_request(span).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", format!("127.0.0.1:{}", addr.port()))
        .body(body)
        .send()
        .await
        .expect("HTTP request completes");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn valid_payload_records_spans_through_channel_http() {
    let (addr, state, _sender, mut rx, handle) = start_test_http_server().await;
    let url = format!("http://{}/v1/traces", addr);
    let req = wrap_request(make_span(vec![1u8; 16], vec![2u8; 8]));
    let body = req.encode_to_vec();
    let resp = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .header("Host", format!("127.0.0.1:{}", addr.port()))
        .body(body)
        .send()
        .await
        .expect("HTTP request completes");
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(state.snapshot().span_count, 1);
    let batch = rx.recv().await.expect("channel received batch");
    match batch {
        ingest::channel::Batch::Spans(_) => {}
        other => panic!("expected Spans batch, got {other:?}"),
    }
    handle.abort();
}
