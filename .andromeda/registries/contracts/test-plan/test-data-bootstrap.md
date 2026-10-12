### Test data bootstrap

- **Strategy:** Self-bootstrapping via OTLP ingest (tests send synthetic spans to live receiver)
- **Mechanism:** Rust builder factories — one set, `test_support::otlp` in the dev-only member `crates/test-support` (chunk 2026-10-10-shared-telemetry-test-data): a test sets what it reasons about and the rest takes a default
  - `resource().service("test-app").span(span().trace_id([0u8; 16]).span_id([0u8; 8])).trace_request()` → the export request a test sends via a `tonic` client to the engine's gRPC port (`logs_request()` for log records; no metrics export request is built)
  - `resource_spans()` / `resource_metrics()` / `resource_logs()` on the same builder → the resource-level batches a test hands the buffer
  - Defaults are fixed constants (an unset event time, an unset service); an unset span id is the span's position under its resource plus one; a span ends `SPAN_DURATION_NANOS` after its start (no duration setter); no clock is read and nothing is random. `rstest` composition stays in a consuming crate that already carries `rstest`; the member has no `rstest` dependency
- **Per-test isolation:** `tempfile::TempDir` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$` ensures fresh DuckDB `:memory:` per test; `tokio::time::pause()` for deterministic clock in retention tests; retention window override via config env `ANDROMEDA_PULSE_BUFFER_RETENTION_SEC=300` (verify old rows discarded after TTL expiry)
- **Cleanup:** `cleanup` command removes test data directory; explicit `drop(temp_dir)` in Rust tests
