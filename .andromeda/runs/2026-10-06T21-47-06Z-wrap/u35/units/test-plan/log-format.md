### Log format

**Stream:** `RUST_LOG=debug cargo nextest run --workspace 2>&1 | tee test.log`

**Format:** Rust `tracing` crate JSON output via `tracing-subscriber::fmt().json().init()`

**Example:**
```json
{"timestamp":"2026-05-02T16:18:34.567Z","level":"INFO","target":"ingest::grpc","message":"TraceService.Export received 10 spans","fields":{"span_count":10,"service":"my-app"}}
{"timestamp":"2026-05-02T16:18:35.123Z","level":"ERROR","target":"buffer","message":"Row insertion failed","fields":{"error_code":"INVARIANT_VIOLATION","reason":"span_id length != 8"}}
```

**Agent-parseable signals:**
- Error detection: `jq 'select(.level == "ERROR") | .message' test.log | head -20`
- Ingest confirmation: `grep 'TraceService.Export received' test.log | tail -1`
- Subsystem startup: `grep 'listening on 127.0.0.1:4317' test.log` (confirm gRPC bind)
- Data leakage check: `! grep 'query parameters:' test.log` (assert no SQL params logged; validates anti-pattern)
