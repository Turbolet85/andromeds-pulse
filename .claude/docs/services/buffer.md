# `buffer` — DuckDB Ring Buffer + Arrow Appender

## Responsibility
Owns the in-memory DuckDB ring buffer (5–10 min retention, configurable) + Apache Arrow zero-copy appender. Periodic retention enforcement via `DELETE WHERE ts < cutoff`. Broadcast fan-out from buffer to viz / MCP / snapshot subscribers via `tokio::sync::broadcast`.

## Key integrations

### Consumes from
- `ingest` crate via `tokio::sync::mpsc` (decoded OTLP batches).
- `viz` crate query-side via DuckDB `Connection` for prepared statement reads.
- `snapshot` crate via DuckDB query for curation pipeline.
- `mcp-server` crate via DuckDB query for tool methods.

### Publishes to
- `tokio::sync::broadcast` channel → live UI subscribers (compact widget, full dashboard, tray icon, MCP).
- `tracing` events at module boundaries (`duckdb.append`, `buffer.tick`, `buffer.eviction`).

### Dependencies
- `duckdb` crate 1.10500.x (DuckDB 1.5.x bundled) — embedded columnar OLAP, in-memory `:memory:` connection.
- `arrow-rs` 55.x — Arrow IPC + RecordBatch + `Appender::append_record_batch()` / `stream_arrow()`.

## Internal conventions
- **Schema name:** `pulse_buffer` (single in-memory `:memory:` DuckDB connection, schema `main`).
- **Reserved tables (canonical OTLP entities):** `spans`, `span_events`, `metrics_points`, `log_records` (+ `log_templates`) — the producer-less `span_links`/`resources`/`instrumentation_scopes` CREATEs were deleted at chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep; retention DELETEs 7 → 4.
- **Schema creation:** on startup; no migrations (per arch §Established Decisions ORM/Migrations None).
- **Primary keys:** OTLP-native — spans use `(trace_id BLOB(16), span_id BLOB(8))` composite; metric points + log records use `(timestamp, resource_hash, name)`.
- **Timestamps:** `TIMESTAMPTZ` (microsecond precision, UTC-stored) + sibling `BIGINT ts_unix_nano` when nanosecond precision required.
- **Nullable patterns:** reserved for OTLP-spec-optional fields (`parent_span_id`, optional resource attrs); required spec fields are `NOT NULL`.
- **Retention enforcement:** `DELETE FROM <table> WHERE ts < now() - INTERVAL '<retention> seconds'` on periodic timer (configured via `ANDROMEDA_PULSE_RETENTION_SECONDS`, default range 300–600).
- **Arrow zero-copy:** ingest payloads enter via `Appender::append_record_batch()` to skip row-by-row marshalling; broadcast emits Arrow IPC bytes directly.
- **Broadcast trace_id correlation:** Arrow `_trace_context` metadata column carries traceparent end-to-end.
- **Errors:** module-internal `thiserror` enum; collapses to `AppError::Storage { message }` at IPC bridge.
- **`buffer.tick` heartbeat every 15s** with `rows_ingested`, `retention_window_active`, `eviction_count`, `memory_bytes`.

## Service-specific gotchas
- **DuckDB encryption is BANNED** — CVE-2025-64429 against the encryption feature. The `:memory:` connection avoids the entire surface; an unnecessary feature flip would re-introduce it.
- **Arrow IPC size cap:** plugin-returned Arrow IPC must be size-bounded (8 MB) at host boundary before re-emit on Tauri Channel API.

## Entry points for modification
- **Schema definitions:** `crates/buffer/src/schema.rs` (DDL for the 7 reserved tables)
- **Appender pipeline:** `crates/buffer/src/appender.rs` (Arrow zero-copy ingest path)
- **Retention task:** `crates/buffer/src/retention.rs` (periodic DELETE)
- **Broadcast fan-out:** `crates/buffer/src/broadcast.rs` (tokio broadcast → channel emit)
- **Connection pool:** `crates/buffer/src/conn.rs` (DuckDB `Connection` lifecycle)
- **Tests:** colocated `#[cfg(test)]` per module

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(buffer)'`
- **Integration:** ephemeral `duckdb::open_in_memory()` per test; rows seeded by OTLP ingest or direct in-memory inserts (the specified `MockArrowBatch::builder()` is not implemented — 0 workspace hits, measured 2026-08-23); `tokio::time::pause()` + `advance(Duration)` for retention window tests.
- **Chaos test (test-plan §10):** Inject 10k spans/sec for 15 min (exceeds 10-min window) → assert oldest spans evicted; assert `metric.buffer.memory_bytes` ≤512 MB.

## Local development
- **Override retention:** `ANDROMEDA_PULSE_RETENTION_SECONDS=60 cargo run --bin pulse-app` (1-minute retention for fast eviction observation).
- **Inspect schema:** `duckdb` CLI does NOT attach to in-memory connections of a running process; use `traces.query` TauRPC for inspection in tests.

## References
- `.andromeda/architecture.md` §Stack + §Conventions Database entity naming + §Occupied Resources DuckDB tables
- `.andromeda/security-plan.md` §Input Validation (DuckDB prepared statements) + §Anti-Patterns Data Protection (DuckDB encryption ban)
- `.andromeda/test-plan.md` §10 (chaos: buffer overflow / retention enforcement)
- `.andromeda/obs-plan.md` §1 P1 (must-trace `duckdb.append` span)
