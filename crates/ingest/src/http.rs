use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Router, http};
use http::{HeaderMap, HeaderValue, StatusCode};
use prost::Message;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::GlobalKeyExtractor;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::channel::{Batch, IngestSender};
use crate::contract::Error;
use crate::grpc::proto::opentelemetry::proto::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};
use crate::grpc::proto::opentelemetry::proto::collector::metrics::v1::{
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
};
use crate::grpc::proto::opentelemetry::proto::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use crate::invariants::{
    validate_resource_logs, validate_resource_metrics, validate_resource_spans,
};
use crate::state::IngestState;

pub const DEFAULT_HTTP_PORT: u16 = 4318;
pub const MAX_DECODING_BODY_SIZE: usize = 8 * 1024 * 1024;
const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";

/// Coarse-global rate-limit for OTLP HTTP ingestion. Same shape as gRPC:
/// ~1.2ms replenishment ≈ 833 req/sec ≈ 50_000 req/min sustained, 1000-token
/// burst. Per security plan §API Security row 1 + arch §Conventions
/// Configuration units (no new env var).
pub const OTLP_HTTP_RATE_LIMIT_BURST_SIZE: u32 = 1000;
pub const OTLP_HTTP_RATE_LIMIT_PERIOD: Duration = Duration::from_micros(1200);

#[derive(Clone)]
struct AppState {
    ingest: Arc<IngestState>,
    sender: Arc<IngestSender>,
}

/// Bind a TCP listener on `addr`. Splitting bind from serve allows callers
/// (e.g., pulse-app/src/main.rs) to record `BindStatus::Ok` to the health
/// envelope as soon as the listener accepts before serve_on enters its
/// long-running future.
pub async fn try_bind(addr: SocketAddr) -> Result<tokio::net::TcpListener, Error> {
    tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::BindFailed {
            reason: sanitize_error(&e),
        })
}

/// Serve OTLP HTTP traffic on a previously-bound listener. Resolves only
/// when the server stops; `Err` is returned on a fatal serve error.
pub async fn serve_on(
    listener: tokio::net::TcpListener,
    state: Arc<IngestState>,
    sender: Arc<IngestSender>,
) -> Result<(), Error> {
    serve_on_with_rate_limit(
        listener,
        state,
        sender,
        OTLP_HTTP_RATE_LIMIT_PERIOD,
        OTLP_HTTP_RATE_LIMIT_BURST_SIZE,
    )
    .await
}

/// Serve with a parameterized rate-limit (period + burst). Used by tests to
/// drive saturation in tight windows; production uses `serve_on` which fixes
/// the production constants.
pub async fn serve_on_with_rate_limit(
    listener: tokio::net::TcpListener,
    state: Arc<IngestState>,
    sender: Arc<IngestSender>,
    rate_limit_period: Duration,
    rate_limit_burst_size: u32,
) -> Result<(), Error> {
    let port = listener
        .local_addr()
        .map(|a| a.port())
        .unwrap_or(DEFAULT_HTTP_PORT);
    let app_state = AppState {
        ingest: state,
        sender,
    };
    let router = build_router(app_state, port, rate_limit_period, rate_limit_burst_size);
    axum::serve(listener, router.into_make_service())
        .await
        .map_err(|e| Error::ServeFailed {
            reason: sanitize_error(&e),
        })
}

fn build_router(
    state: AppState,
    port: u16,
    rate_limit_period: Duration,
    rate_limit_burst_size: u32,
) -> Router {
    let allowed_hosts: Arc<Vec<String>> = Arc::new(vec![
        format!("127.0.0.1:{port}"),
        format!("localhost:{port}"),
        format!("[::1]:{port}"),
    ]);

    let governor_config = GovernorConfigBuilder::default()
        .key_extractor(GlobalKeyExtractor)
        .period(rate_limit_period)
        .burst_size(rate_limit_burst_size)
        .finish()
        .expect("rate-limit config must be valid (non-zero period + non-zero burst)");
    let governor_layer: GovernorLayer<_, _, axum::body::Body> = GovernorLayer::new(governor_config)
        .error_handler(|err| {
            let wait_time = match &err {
                tower_governor::GovernorError::TooManyRequests { wait_time, .. } => *wait_time,
                _ => 0,
            };
            tracing::warn!(
                target: "ingest.http.rate_limit.rejected",
                quota_window_seconds = wait_time,
                reject_reason = "rate_limit_exceeded",
                "OTLP HTTP rate limit hit",
            );
            err.into()
        });

    Router::new()
        .route("/v1/traces", post(handle_traces))
        .route("/v1/metrics", post(handle_metrics))
        .route("/v1/logs", post(handle_logs))
        .with_state(state)
        .layer(CorsLayer::new())
        .layer(DefaultBodyLimit::max(MAX_DECODING_BODY_SIZE))
        .layer(governor_layer)
        .layer({
            let allowed = Arc::clone(&allowed_hosts);
            middleware::from_fn(move |req, next| {
                let allowed = Arc::clone(&allowed);
                host_header_check(allowed, req, next)
            })
        })
        .layer(TraceLayer::new_for_http())
}

async fn host_header_check(
    allowed: Arc<Vec<String>>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let host_ok = req
        .headers()
        .get(http::header::HOST)
        .and_then(|h| h.to_str().ok())
        .map(|host| allowed.iter().any(|a| a == host))
        .unwrap_or(false);
    if !host_ok {
        tracing::warn!(
            target: "ingest::http",
            host_header_rejected = true,
            expected_host_class = "loopback",
            "host header rejected"
        );
        return StatusCode::FORBIDDEN.into_response();
    }
    next.run(req).await
}

#[tracing::instrument(
    skip(state, headers, body),
    fields(
        http.method = "POST",
        http.route = "/v1/traces",
        content_type = tracing::field::Empty,
        body_size_bytes = tracing::field::Empty,
        status_code = tracing::field::Empty,
        span_count = tracing::field::Empty,
        traceparent = tracing::field::Empty,
    ),
)]
async fn handle_traces(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let body_size = body.len();
    tracing::Span::current().record("body_size_bytes", body_size);
    if let Some(tp) = extract_traceparent_http(&headers) {
        tracing::Span::current().record("traceparent", tracing::field::display(&tp));
    }
    if !content_type_is_protobuf(&headers) {
        let content_type = headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        tracing::Span::current().record("content_type", content_type);
        tracing::Span::current().record("status_code", 415);
        return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
    }
    tracing::Span::current().record("content_type", PROTOBUF_CONTENT_TYPE);
    let payload: ExportTraceServiceRequest = {
        let _decode_span = tracing::debug_span!("parse.protobuf").entered();
        match Message::decode(body.as_ref()) {
            Ok(req) => req,
            Err(_) => {
                tracing::Span::current().record("status_code", 400);
                return StatusCode::BAD_REQUEST.into_response();
            }
        }
    };
    let span_count = count_spans(&payload);
    tracing::Span::current().record("span_count", span_count);
    if let Err(e) = validate_resource_spans(&payload.resource_spans) {
        log_invariant(&e);
        tracing::Span::current().record("status_code", 400);
        return StatusCode::BAD_REQUEST.into_response();
    }
    if state
        .sender
        .try_send(Batch::Spans(payload.resource_spans))
        .is_err()
    {
        log_channel_full();
        tracing::Span::current().record("status_code", 503);
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    state.ingest.record_spans(span_count);
    tracing::Span::current().record("status_code", 200);
    encode_protobuf_response(&ExportTraceServiceResponse::default())
}

#[tracing::instrument(
    skip(state, headers, body),
    fields(
        http.method = "POST",
        http.route = "/v1/metrics",
        content_type = tracing::field::Empty,
        body_size_bytes = tracing::field::Empty,
        status_code = tracing::field::Empty,
        span_count = tracing::field::Empty,
        traceparent = tracing::field::Empty,
    ),
)]
async fn handle_metrics(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let body_size = body.len();
    tracing::Span::current().record("body_size_bytes", body_size);
    if let Some(tp) = extract_traceparent_http(&headers) {
        tracing::Span::current().record("traceparent", tracing::field::display(&tp));
    }
    if !content_type_is_protobuf(&headers) {
        let content_type = headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        tracing::Span::current().record("content_type", content_type);
        tracing::Span::current().record("status_code", 415);
        return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
    }
    tracing::Span::current().record("content_type", PROTOBUF_CONTENT_TYPE);
    let payload: ExportMetricsServiceRequest = {
        let _decode_span = tracing::debug_span!("parse.protobuf").entered();
        match Message::decode(body.as_ref()) {
            Ok(req) => req,
            Err(_) => {
                tracing::Span::current().record("status_code", 400);
                return StatusCode::BAD_REQUEST.into_response();
            }
        }
    };
    let dp_count = count_metric_data_points(&payload);
    tracing::Span::current().record("span_count", dp_count);
    if let Err(e) = validate_resource_metrics(&payload.resource_metrics) {
        log_invariant(&e);
        tracing::Span::current().record("status_code", 400);
        return StatusCode::BAD_REQUEST.into_response();
    }
    if state
        .sender
        .try_send(Batch::Metrics(payload.resource_metrics))
        .is_err()
    {
        log_channel_full();
        tracing::Span::current().record("status_code", 503);
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    state.ingest.record_metric_data_points(dp_count);
    tracing::Span::current().record("status_code", 200);
    encode_protobuf_response(&ExportMetricsServiceResponse::default())
}

#[tracing::instrument(
    skip(state, headers, body),
    fields(
        http.method = "POST",
        http.route = "/v1/logs",
        content_type = tracing::field::Empty,
        body_size_bytes = tracing::field::Empty,
        status_code = tracing::field::Empty,
        span_count = tracing::field::Empty,
        traceparent = tracing::field::Empty,
    ),
)]
async fn handle_logs(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let body_size = body.len();
    tracing::Span::current().record("body_size_bytes", body_size);
    if let Some(tp) = extract_traceparent_http(&headers) {
        tracing::Span::current().record("traceparent", tracing::field::display(&tp));
    }
    if !content_type_is_protobuf(&headers) {
        let content_type = headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        tracing::Span::current().record("content_type", content_type);
        tracing::Span::current().record("status_code", 415);
        return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
    }
    tracing::Span::current().record("content_type", PROTOBUF_CONTENT_TYPE);
    let payload: ExportLogsServiceRequest = {
        let _decode_span = tracing::debug_span!("parse.protobuf").entered();
        match Message::decode(body.as_ref()) {
            Ok(req) => req,
            Err(_) => {
                tracing::Span::current().record("status_code", 400);
                return StatusCode::BAD_REQUEST.into_response();
            }
        }
    };
    let log_count = count_log_records(&payload);
    tracing::Span::current().record("span_count", log_count);
    if let Err(e) = validate_resource_logs(&payload.resource_logs) {
        log_invariant(&e);
        tracing::Span::current().record("status_code", 400);
        return StatusCode::BAD_REQUEST.into_response();
    }
    if state
        .sender
        .try_send(Batch::Logs(payload.resource_logs))
        .is_err()
    {
        log_channel_full();
        tracing::Span::current().record("status_code", 503);
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    state.ingest.record_log_records(log_count);
    tracing::Span::current().record("status_code", 200);
    encode_protobuf_response(&ExportLogsServiceResponse::default())
}

fn content_type_is_protobuf(headers: &HeaderMap) -> bool {
    headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .map(|ct| ct.split(';').next().unwrap_or("").trim() == PROTOBUF_CONTENT_TYPE)
        .unwrap_or(false)
}

fn extract_traceparent_http(headers: &HeaderMap) -> Option<String> {
    headers
        .get("traceparent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

fn encode_protobuf_response<M: Message>(msg: &M) -> Response {
    let mut buf = Vec::with_capacity(msg.encoded_len());
    if msg.encode(&mut buf).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let mut response = (StatusCode::OK, buf).into_response();
    response.headers_mut().insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static(PROTOBUF_CONTENT_TYPE),
    );
    response
}

fn count_spans(req: &ExportTraceServiceRequest) -> u64 {
    req.resource_spans
        .iter()
        .map(|rs| {
            rs.scope_spans
                .iter()
                .map(|ss| ss.spans.len() as u64)
                .sum::<u64>()
        })
        .sum()
}

fn count_metric_data_points(req: &ExportMetricsServiceRequest) -> u64 {
    req.resource_metrics
        .iter()
        .map(|rm| {
            rm.scope_metrics
                .iter()
                .map(|sm| sm.metrics.len() as u64)
                .sum::<u64>()
        })
        .sum()
}

fn count_log_records(req: &ExportLogsServiceRequest) -> u64 {
    req.resource_logs
        .iter()
        .map(|rl| {
            rl.scope_logs
                .iter()
                .map(|sl| sl.log_records.len() as u64)
                .sum::<u64>()
        })
        .sum()
}

fn sanitize_error(e: &(impl std::fmt::Display + ?Sized)) -> String {
    format!("{}", e)
}

fn log_invariant(e: &Error) {
    if let Error::InvariantViolation {
        kind,
        expected,
        actual,
    } = e
    {
        tracing::error!(
            target: "ingest.http.parse.error",
            span_field_invalid = *kind,
            expected_length = *expected as u64,
            actual_length = *actual as u64,
            rejection_reason = "post_decode_invariant",
            "OTLP invariant violation",
        );
    }
}

fn log_channel_full() {
    tracing::warn!(
        target: "ingest.channel.full",
        channel_name = "ingest",
        capacity_pct = 100.0,
        rejection_reason = "saturated",
        "ingest channel saturated",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};

    #[test]
    fn max_decoding_body_size_is_8_mb() {
        assert_eq!(MAX_DECODING_BODY_SIZE, 8 * 1024 * 1024);
    }

    #[test]
    fn default_http_port_is_4318() {
        assert_eq!(DEFAULT_HTTP_PORT, 4318);
    }

    #[test]
    fn count_spans_sums_across_resource_and_scope() {
        let req = ExportTraceServiceRequest {
            resource_spans: vec![
                ResourceSpans {
                    resource: None,
                    scope_spans: vec![
                        ScopeSpans {
                            scope: None,
                            spans: vec![Span::default(), Span::default()],
                            schema_url: String::new(),
                        },
                        ScopeSpans {
                            scope: None,
                            spans: vec![Span::default()],
                            schema_url: String::new(),
                        },
                    ],
                    schema_url: String::new(),
                },
                ResourceSpans {
                    resource: None,
                    scope_spans: vec![ScopeSpans {
                        scope: None,
                        spans: vec![Span::default()],
                        schema_url: String::new(),
                    }],
                    schema_url: String::new(),
                },
            ],
        };
        assert_eq!(count_spans(&req), 4);
    }

    #[test]
    fn count_spans_zero_for_empty() {
        let req = ExportTraceServiceRequest::default();
        assert_eq!(count_spans(&req), 0);
    }

    #[test]
    fn extract_traceparent_returns_none_when_missing() {
        let headers = HeaderMap::new();
        assert_eq!(extract_traceparent_http(&headers), None);
    }

    #[test]
    fn extract_traceparent_returns_value_when_present() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "traceparent",
            HeaderValue::from_static("00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"),
        );
        assert_eq!(
            extract_traceparent_http(&headers),
            Some("00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".to_string())
        );
    }

    #[test]
    fn content_type_is_protobuf_accepts_canonical() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        assert!(content_type_is_protobuf(&headers));
    }

    #[test]
    fn content_type_is_protobuf_accepts_canonical_with_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf; charset=utf-8"),
        );
        assert!(content_type_is_protobuf(&headers));
    }

    #[test]
    fn content_type_is_protobuf_rejects_json() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        assert!(!content_type_is_protobuf(&headers));
    }

    #[test]
    fn content_type_is_protobuf_false_when_missing() {
        let headers = HeaderMap::new();
        assert!(!content_type_is_protobuf(&headers));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_bind_succeeds_on_ephemeral_port() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = try_bind(addr)
            .await
            .expect("bind on ephemeral port must succeed");
        let bound = listener.local_addr().expect("listener has local addr");
        assert_eq!(bound.ip().to_string(), "127.0.0.1");
        assert_ne!(bound.port(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn try_bind_returns_bind_failed_on_collision() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener_a = try_bind(addr).await.expect("first bind must succeed");
        let bound = listener_a.local_addr().unwrap();
        let result = try_bind(bound).await;
        assert!(matches!(result, Err(Error::BindFailed { .. })));
    }
}
