### Test data bootstrap

- **Strategy:** Self-bootstrapping via OTLP ingest (tests send synthetic spans to live receiver)
- **Mechanism:** Rust builder factories + `rstest` fixtures
  - `MockTraceSpan::builder().service("test-app").trace_id([0u8; 16]).span_id([0u8; 8]).duration_ms(50).build()` → serializes to OTLP protobuf, sent via `tonic` client to `:4317`
  - `#[fixture] fn test_spans() -> Vec<TraceSpan> { vec![MockTraceSpan::builder()...] }` → composed with other fixtures per `rstest` composition
- **Per-test isolation:** `tempfile::TempDir` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$` ensures fresh DuckDB `:memory:` per test; `tokio::time::pause()` for deterministic clock in retention tests; retention window override via config env `ANDROMEDA_PULSE_BUFFER_RETENTION_SEC=300` (verify old rows discarded after TTL expiry)
- **Cleanup:** `cleanup` command removes test data directory; explicit `drop(temp_dir)` in Rust tests
