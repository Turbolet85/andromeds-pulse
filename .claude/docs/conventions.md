# Project Conventions

_Extracted from `.andromeda/architecture.md` Conventions section by `/setup-project`. Primary source: architecture.md._

## File naming
- Rust files: `snake_case.rs`
- One library crate per module under `crates/<module-name>/`; binary crate is `pulse-app/`.
- WIT interface files: `kebab-case.wit`.
- Webview source files: per design specialist convention (out of arch scope).

## Variable / function naming (Rust)
- `snake_case` for functions, variables, modules
- `UpperCamelCase` for types, traits, enums
- `SCREAMING_SNAKE_CASE` for constants
- Standard `cargo clippy` `style` group enforced in CI.

## Endpoint naming
- **OTLP receivers:** spec-fixed paths — `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` on `:4318`. NO project-specific URL versioning prefix beyond OTLP spec.
- **Tauri IPC procedures:** two authorized shapes:
  - **Top-level bare `snake_case` verbs** (cross-cutting envelope): `app_info`, `health`, `ready`, `get_settings`, `update_settings`
  - **`<router>.<verb>` dotted namespaces** (per-crate routers): `traces.query`, `metrics.query`, `logs.query`, `snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard`, `plugins.list`, `plugins.reload`, `plugins.invoke`, `mcp.status`, `mcp.start`, `mcp.stop`, `workspace.detect`, `workspace.list`, `telemetry.frontend.record_web_vital`, `telemetry.frontend.record_frame_ms`
  - Both segments are `snake_case`. The canonical procedure list is in arch §Occupied Resources Tauri IPC routes.
- **MCP server:** spec-fixed JSON-RPC 2.0 method names — `initialize`, `tools/list`, `tools/call`, `notifications/*`. Tool methods: `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`.

## Database entity naming
- DuckDB tables use plural `snake_case` matching OTLP entity: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`.
- Columns: `snake_case`.
- Ring-buffer cutoff via periodic `DELETE FROM <table> WHERE ts < now() - INTERVAL '<retention> minutes'`.

## Primary keys
- Spans: OTLP-native 16-byte `trace_id` + 8-byte `span_id` composite (no surrogate UUID).
- Metric points: OTLP-native identity (`metric_name` + `ts_unix_nano` + `resource_hash`).
- Log records: `(ts_unix_nano, resource_hash, severity_number, seq)`. An OTLP LogRecord has no spec-defined unique id and the table has no `name` column, so the OTLP-native columns alone cannot separate two records from one resource in the same nanosecond at the same severity; `seq` is a monotonic in-process ordinal allocated per batch by `BufferState::reserve_log_seq_block`.
- No UUID or random surrogate keys. `seq` is the one declared exception — an internal disambiguating ordinal, never an observable (absent from `BufferStateSnapshot`, `buffer.tick`, `viz` `SELECT_LOGS`, and the MCP response shape).

## Timestamp handling
- All DuckDB timestamp columns: `TIMESTAMPTZ` (microsecond precision, UTC-stored).
- Incoming OTLP nanosecond timestamps: stored as `TIMESTAMPTZ` (microsecond truncation accepted) plus sibling `BIGINT` column `ts_unix_nano` when nanosecond precision required for spec round-tripping.

## Nullable patterns
- Nullable columns reserved for OTLP-spec-optional fields (e.g., `parent_span_id`, optional resource attributes).
- Required spec fields are `NOT NULL`.

## Module visibility (Cargo workspace boundaries)
- Each crate exposes only its public contract via `pub`.
- Cross-crate utilities use `pub(crate)`.
- NO `pub use` re-exports across crate boundaries except in the explicit contract module of each crate.
- Dependency graph forms a DAG with `pulse-app` as the only root — no library crate depends on the binary crate; no library crate depends on a sibling unless its declared contract requires it.

## Feature flags
- Cargo features: `kebab-case` (`mcp-server`, `otap-ingest`).
- Default features: minimal (no `mcp-server`, no `otap-ingest`) — keeps the small-binary baseline.

## Configuration units
- Durations in user-facing config: seconds with explicit units in field names (`retention_seconds`).
- Ports: `u16` with `TryFrom<u16>` validating non-privileged-or-explicitly-allowed ranges.

## Error response schemas
- **Tauri IPC:** every `#[taurpc::procedure]` returns `Result<T, AppError>` where `AppError` is a `serde`-friendly enum with stable variants:
  - `Validation { field, reason }`
  - `NotFound { resource }`
  - `Internal { message }`
  - `Plugin { plugin_id, message }`
  - `Storage { message }`
  - `Ingest { message }`
- **OTLP receiver:** per OpenTelemetry HTTP spec — non-2xx responses carry `Status` proto in body for HTTP/protobuf, or HTTP/JSON for HTTP/JSON; gRPC errors use standard `tonic::Status` codes.
- **MCP server:** standard JSON-RPC 2.0 error object — numeric `code`, `message` (string), optional `data`; MCP-spec error codes honored.
- Module-internal code uses `thiserror`-derived enums and converts at the bridge via `From` impls. `main.rs` and any non-IPC top-level uses `anyhow::Result`.

## Common response envelope (Tauri IPC paginated lists)
```json
{
  "items": [],
  "total": 0,
  "next_cursor": null
}
```
Cursor is an opaque string; `null` means "no more results."

## Logging conventions (binding to obs §3)
- Structured JSON via `tracing-subscriber::fmt::Layer::json()`.
- One JSON object per line.
- Required fields: `timestamp` (ISO-8601), `level`, `target`, `message`, `fields.service.name`, `fields.service.version`, `fields.deployment.environment`.
- Span naming: `{module}.{operation}` — never high-cardinality (no per-trace-ID, no per-arbitrary-request-path).
- Metric event naming: `metric.{module}.{measure}` target prefix (e.g., `metric.snapshot.token_count_ms`).

## Git / commit conventions
- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, `test:`, `perf:`.
- Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> when applicable.
- Feature branches per task; never force-push to main/master.

## File modification discipline
- TypeScript strict mode — no `any`, no `as` casts unless justified with a comment.
- ES modules (import/export) by default in webview; CommonJS only in config files that require it.
- React: functional components with hooks, destructured imports.
- Default to writing no comments — only when WHY is non-obvious (hidden constraint, subtle invariant, workaround for specific bug).
