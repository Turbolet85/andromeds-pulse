# `ingest` — OTLP Receivers

## Responsibility
Owns the two OTLP receiver surfaces (`:4317` gRPC + `:4318` HTTP) bound `127.0.0.1` only. Decodes incoming OTLP via `prost` + `opentelemetry-proto`, runs post-decode invariant checks, hands off to `buffer` via `tokio::sync::mpsc` (with backpressure). Does NOT own storage / query / visualization.

## Key integrations

### Consumes from
- External clients (any OTel SDK speaking OTLP) → `:4317` gRPC TraceService/MetricsService/LogsService + `:4318` HTTP `POST /v1/{traces,metrics,logs}` (protobuf or JSON).
- W3C `traceparent` from gRPC `grpc-trace-bin` metadata + HTTP header at request entry.

### Publishes to
- `buffer` crate via `tokio::sync::mpsc` channel (decoded `OtlpBatch` records).
- `tracing` events at module boundaries (`ingest.grpc.export.request`, `ingest.http.export.request`, `ingest.tick`).

### Dependencies
- `tonic` 0.14.x — gRPC server + `prost` 0.14 codegen against `opentelemetry-proto`.
- `axum` 0.8.x on `hyper` 1.x + `tower` — HTTP server.
- `prost` 0.14 — protobuf wire format decode.
- `opentelemetry-proto` — OTLP message definitions.
- `tower_governor` — coarse global + per-source-port rate limiting (pinned at bootstrap; SHA-pinned in CI).
- `std` both-sides-canonicalize — env var path canonicalization where a path var applies (port overrides via `ANDROMEDA_PULSE_OTLP_*_PORT` are `TryFrom<u16>`, not paths).
- `tower_http::cors::CorsLayer::new()` — default-deny CORS on `:4318`.

## Internal conventions
- All boundary handlers wrapped with `#[tracing::instrument(skip(req), fields(rpc.system, rpc.service, rpc.method, span_count, traceparent))]`.
- Post-`prost` invariant checks REQUIRED before handoff to `buffer::Appender`: `span_id` is 8 bytes, `trace_id` is 16 bytes, attribute keys/values bounded.
- Body size cap: `axum::DefaultBodyLimit::max(8 * 1024 * 1024)` on each route + `tonic` `.max_decoding_message_size(8 * 1024 * 1024)` per `*ServiceServer` builder.
- Host-header allowlist middleware on `:4318` (reject non-`{127.0.0.1, localhost, [::1]}:configured-port`) — DNS rebinding mitigation.
- Loopback-only bind ENFORCED in startup config — env var override `ANDROMEDA_PULSE_OTLP_*_PORT` parses as `u16` via `TryFrom<u16>` but bind address is hardcoded `127.0.0.1`.
- Errors collapse to `AppError::Ingest { message }` at the bridge; OTLP receiver returns standard `tonic::Status` codes / OTLP `Status` proto in body.
- `ingest.tick` heartbeat every 15s with `span_count`, `buffer_capacity_pct`, `broadcast_subscribers`.

## Service-specific gotchas
- **`tonic 0.14` vs `opentelemetry-otlp 0.31`** transitive duplicate (the latter still pins `tonic 0.13`) — `cargo deny check bans` enforces; reconcile before tagging v0.1.0. Note: this crate doesn't actually depend on `opentelemetry-otlp` (we use `prost` + `opentelemetry-proto` directly), so the duplicate is purely transitive.
- See `.claude/docs/gotchas.md` for the full list.

## Entry points for modification
- **gRPC handlers:** `crates/ingest/src/grpc.rs` (TraceService / MetricsService / LogsService impls)
- **HTTP routes:** `crates/ingest/src/http.rs` (axum router for `/v1/traces`, `/v1/metrics`, `/v1/logs`)
- **Post-decode invariants:** `crates/ingest/src/invariants.rs` (span_id / trace_id length checks, attribute bounds)
- **Channel hand-off:** `crates/ingest/src/pipeline.rs` (mpsc to `buffer::Appender`)
- **Tests:** `crates/ingest/src/{grpc,http,invariants,pipeline}.rs` colocated `#[cfg(test)]` modules

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(ingest)'`
- **Integration:** `tonic` 0.14.5 native client → loopback `:4317`; `axum-test` 18.7 + `reqwest` 0.12 → `:4318`. Use `MockTraceSpan::builder()` for fixture seeding.
- **Negative tests REQUIRED:** post-`prost` invariant rejection, oversized body 413, `0.0.0.0` bind rejection, Host-header allowlist miss, CORS rejection, rate-limit hit.

## Local development
- **Run locally:** `cargo run --bin pulse-app` (the receiver crate is library-only; the binary wires it).
- **Override ports:** `ANDROMEDA_PULSE_OTLP_GRPC_PORT=14317 ANDROMEDA_PULSE_OTLP_HTTP_PORT=14318 cargo run --bin pulse-app`
- **Send test span:** `grpcurl -plaintext -d @ 127.0.0.1:4317 opentelemetry.proto.collector.trace.v1.TraceService/Export < test-span.json` or `curl -X POST -H "Content-Type: application/x-protobuf" --data-binary @test-span.bin http://127.0.0.1:4318/v1/traces`

## References
- `.andromeda/architecture.md` §Stack + §Standard Contracts (OTLP receiver entry)
- `.andromeda/security-plan.md` §Input Validation + §API Security (loopback / Host header / CORS / body cap / rate limit)
- `.andromeda/test-plan.md` §6 P1 (E2E receive-and-visualize)
- `.andromeda/obs-plan.md` §3 (tracing instrumentation pattern) + §1 P1 (must-trace spans)
