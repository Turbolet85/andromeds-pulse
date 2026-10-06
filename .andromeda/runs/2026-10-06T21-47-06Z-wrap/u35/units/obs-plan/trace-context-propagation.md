### Trace context propagation

- **HTTP boundaries (`:4318` OTLP/HTTP receiver on axum 0.8):** W3C `traceparent` header extracted at request entry via manual `tower::Layer` (or `tower-http::TraceLayer` for plain `tracing` spans, NOT OTel-flavored); attached as a regular field on the inner `#[tracing::instrument]` span (`fields(traceparent = %tp)`) — downstream calls inherit via `tracing::Span::current()`
- **gRPC boundaries (`:4317` OTLP/gRPC receiver on tonic 0.14):** gRPC metadata (`grpc-trace-bin` header) extracted at gRPC service method handler entry via manual interceptor; attached as a `traceparent` field on the local `#[tracing::instrument]` span; same propagation pattern as HTTP
- **IPC boundaries (TauRPC):** TauRPC router handlers manually extract optional `traceparent` field from IPC envelope; initialize the local span with `fields(traceparent = %tp)` so downstream `tracing::Span::current()` calls inherit
- **Internal async boundaries:** `tokio::spawn` calls use `.in_current_span()` extension trait (from `tracing` 0.1) to propagate span context into spawned tasks
- **Real-time push streams:** `pulse://stream/spans` etc. carry trace context in Arrow metadata column (`_trace_context` schema field) for end-to-end correlation. NOTE: `traceparent` value is treated as an opaque string in `tracing` events, not bound to any OTel SDK trace context
