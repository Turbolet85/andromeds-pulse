use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::ExportLogsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::logs::v1::{
    LogRecord, ResourceLogs as LogResourceLogs, ScopeLogs,
};
use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{
    Metric, ResourceMetrics as MetricResourceMetrics, ScopeMetrics,
};
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use ingest::state::IngestState;
use prost::Message;
use reqwest::StatusCode;

async fn start_test_http_server() -> (SocketAddr, Arc<IngestState>, tokio::task::JoinHandle<()>) {
    let state = Arc::new(IngestState::new());
    let listener = ingest::http::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("bind on ephemeral loopback port must succeed in test");
    let bound = listener.local_addr().expect("listener has local_addr");
    let state_clone = Arc::clone(&state);
    let handle = tokio::spawn(async move {
        let _ = ingest::http::serve_on(listener, state_clone).await;
    });
    // Best-effort wait for the server's accept loop to be ready before clients connect.
    tokio::time::sleep(Duration::from_millis(50)).await;
    (bound, state, handle)
}

fn make_span(trace_id: [u8; 16], span_id: [u8; 8]) -> Span {
    Span {
        trace_id: trace_id.to_vec(),
        span_id: span_id.to_vec(),
        name: "test-span".to_string(),
        kind: 0,
        start_time_unix_nano: 0,
        end_time_unix_nano: 0,
        ..Default::default()
    }
}

fn make_traces_request(span_count: usize) -> ExportTraceServiceRequest {
    let spans: Vec<Span> = (0..span_count)
        .map(|i| make_span([1u8; 16], [(i % 250) as u8 + 1; 8]))
        .collect();
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

fn make_metrics_request(metric_count: usize) -> ExportMetricsServiceRequest {
    let metrics: Vec<Metric> = (0..metric_count)
        .map(|i| Metric {
            name: format!("metric.{i}"),
            ..Default::default()
        })
        .collect();
    ExportMetricsServiceRequest {
        resource_metrics: vec![MetricResourceMetrics {
            resource: None,
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                metrics,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

fn make_logs_request(log_count: usize) -> ExportLogsServiceRequest {
    let log_records: Vec<LogRecord> = (0..log_count).map(|_| LogRecord::default()).collect();
    ExportLogsServiceRequest {
        resource_logs: vec![LogResourceLogs {
            resource: None,
            scope_logs: vec![ScopeLogs {
                scope: None,
                log_records,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

fn endpoint(addr: SocketAddr, path: &str) -> String {
    format!("http://{}{}", addr, path)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p1_http_post_protobuf_traces_roundtrip() {
    let (addr, state, handle) = start_test_http_server().await;
    let body = make_traces_request(100).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/traces"))
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert_eq!(resp.status(), StatusCode::OK, "expected 200 OK");
    assert_eq!(state.snapshot().span_count, 100);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p1_http_post_protobuf_metrics_roundtrip() {
    let (addr, state, handle) = start_test_http_server().await;
    let body = make_metrics_request(50).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/metrics"))
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert_eq!(resp.status(), StatusCode::OK, "expected 200 OK");
    assert_eq!(state.snapshot().metric_data_point_count, 50);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn p1_http_post_protobuf_logs_roundtrip() {
    let (addr, state, handle) = start_test_http_server().await;
    let body = make_logs_request(25).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/logs"))
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert_eq!(resp.status(), StatusCode::OK, "expected 200 OK");
    assert_eq!(state.snapshot().log_record_count, 25);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn body_over_8mb_returns_413_or_4xx() {
    let (addr, state, handle) = start_test_http_server().await;
    // Build a >8MB body. 9500 spans × 1024-byte ballast ≈ 9MB encoded.
    let mut spans = Vec::new();
    let ballast: String = "x".repeat(1024);
    for i in 0..9_500 {
        let mut s = make_span([1u8; 16], [(i % 250) as u8 + 1; 8]);
        s.name = ballast.clone();
        spans.push(s);
    }
    let req = ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: None,
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    };
    let body = req.encode_to_vec();
    assert!(body.len() > 8 * 1024 * 1024, "test ballast must exceed 8MB");
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/traces"))
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    // axum's DefaultBodyLimit returns 413 (Payload Too Large) when body exceeds cap.
    let status = resp.status();
    assert!(
        status == StatusCode::PAYLOAD_TOO_LARGE || status.is_client_error(),
        "expected 413 or 4xx body-rejection status, got {}",
        status
    );
    assert_eq!(
        state.snapshot().span_count,
        0,
        "rejected payload must not increment span counter"
    );
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn host_header_evil_example_returns_403() {
    let (addr, state, handle) = start_test_http_server().await;
    let body = make_traces_request(5).encode_to_vec();
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/traces"))
        .header("host", "evil.example.com:4318")
        .header("content-type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "expected 403 for non-allowlisted Host header"
    );
    assert_eq!(
        state.snapshot().span_count,
        0,
        "host-rejected payload must not increment span counter"
    );
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cors_preflight_has_no_allow_origin_header() {
    let (addr, _state, handle) = start_test_http_server().await;
    let resp = reqwest::Client::new()
        .request(reqwest::Method::OPTIONS, endpoint(addr, "/v1/traces"))
        .header("origin", "https://evil.example.com")
        .header("access-control-request-method", "POST")
        .header("access-control-request-headers", "content-type")
        .send()
        .await
        .expect("OPTIONS preflight must reach loopback receiver");
    let acao = resp.headers().get("access-control-allow-origin");
    assert!(
        acao.is_none(),
        "default-deny CORS must NOT echo Access-Control-Allow-Origin; got {:?}",
        acao
    );
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unsupported_content_type_json_returns_415() {
    let (addr, state, handle) = start_test_http_server().await;
    let resp = reqwest::Client::new()
        .post(endpoint(addr, "/v1/traces"))
        .header("content-type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("HTTP POST must reach loopback receiver");
    assert_eq!(
        resp.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "application/json deferred to follow-on chunk; chunk #17 ships protobuf only"
    );
    assert_eq!(state.snapshot().span_count, 0);
    handle.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn try_bind_rejects_collision_with_bind_failed() {
    let listener = ingest::http::try_bind(SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("first bind must succeed");
    let bound = listener.local_addr().unwrap();
    let collision = ingest::http::try_bind(bound).await;
    assert!(
        matches!(collision, Err(ingest::contract::Error::BindFailed { .. })),
        "second bind on same port must return BindFailed; got {:?}",
        collision,
    );
}
