# `viz` — Query Layer + Real-time Push

## Responsibility
Owns the query layer feeding webview WebGPU charts. Hosts the `traces.*` / `metrics.*` / `logs.*` TauRPC routers. Aggregates buffer data by service / time-bucket via DuckDB prepared statements; emits real-time push streams as binary Arrow IPC payloads via Tauri Channel API.

## Key integrations

### Consumes from
- `buffer` crate via DuckDB `Connection` (prepared statements with `?` placeholders only).
- `buffer` broadcast channel for real-time fan-out.

### Publishes to
- TauRPC routers: `traces.query`, `metrics.query`, `logs.query` (paginated result sets).
- Tauri Channel API: `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs` (binary Arrow IPC payloads).
- `tracing` events: `viz.query.{traces|metrics|logs}`, `viz.tick`, `metric.trace.latency_percentiles`.

### Dependencies
- `duckdb` crate (read-side prepared statements).
- `arrow-rs` 55.x for Arrow RecordBatch + `arrow_ipc::StreamWriter` to emit binary payloads.
- `tauri` 2.x for Channel API.
- `taurpc` for IPC router registration.

## Internal conventions
- **Prepared statements ONLY** — `Connection::prepare("SELECT … WHERE service_name = ?").execute([&service_name])`. NEVER `format!` SQL with user input.
- **Pagination contract** — every list-returning IPC command wraps results in `{ items: [], total: N, next_cursor: opaque-string-or-null }`.
- **Arrow IPC payload size cap** at 8 MB before `Channel::send` per security plan §Anti-Patterns Code Patterns (truncate or paginate larger result sets).
- **Trace context propagation** — Arrow `_trace_context` metadata column carries traceparent end-to-end on push streams.
- **Anonymized query logging** — `tracing` events emit `query_id` + `param_count` + `param_types: ["string","timestamp"]` only; NEVER raw query text or parameter values.
- **`viz.tick` heartbeat every 15s** with `query_latency_ms`, `subscribers_active`.
- **Errors** collapse to `AppError::Storage { message }` (sanitized).

## Service-specific gotchas
- **High-cardinality span names** — avoid using full trace_id / per-arbitrary-request-path in span names; explodes downstream `jq` aggregation cost. Use `{module}.{operation}` pattern.
- **Channel emit binary discipline** — payloads are Arrow IPC bytes (zero-copy); webview decodes via `apache-arrow` JS package + `arrow_ipc.StreamReader`.

## Entry points for modification
- **Query routers:** `crates/viz/src/{traces,metrics,logs}.rs` (TauRPC procedure handlers)
- **Aggregation queries:** `crates/viz/src/aggregator.rs` (time-bucket grouping, percentile computation)
- **Channel emit pipeline:** `crates/viz/src/stream.rs` (broadcast subscriber → Arrow IPC encoder → Tauri Channel)
- **Tests:** colocated per module

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(viz)'`
- **Integration:** seed buffer via `MockArrowBatch`; invoke `traces.query` via `tauri::test::mock_builder()` + `get_ipc_response()`; assert response shape + row count.
- **Cross-surface coordination test (test-plan §10):** subscribe to `pulse://stream/spans` → ingest gRPC → call `traces.query` → assert returned row set includes span from channel event (no desync).

## Local development
- **Tauri dev mode:** `cargo tauri dev` (hot reload + auto-open DevTools).
- **Inspect channel emission:** Subscribe in Tauri test client via `Channel::new()`; decode Arrow IPC bytes via `arrow_ipc::StreamReader::try_new(bytes, None)`.

## References
- `.andromeda/architecture.md` §Conventions (TauRPC procedure naming) + §Standard Contracts (Real-time push contract)
- `.andromeda/security-plan.md` §Anti-Patterns Input (DuckDB prepared statements) + §Logging Vector 5 (query_id + param_count)
- `.andromeda/test-plan.md` §6 P1, P6
- `.andromeda/obs-plan.md` §1 P1 + §5 Metric Coverage (`metric.trace.latency_percentiles`)
