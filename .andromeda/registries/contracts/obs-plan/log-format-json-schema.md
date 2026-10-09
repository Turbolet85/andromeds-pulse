### Log format JSON schema

Verbatim from upstream-context Section 5 Test Plan Excerpt → Test Harness Contract Summary (binding contract):
```json
{
  "timestamp": "2026-05-02T16:18:34.567Z",
  "level": "INFO",
  "target": "ingest::grpc",
  "message": "TraceService.Export received 10 spans",
  "fields": {
    "span_count": 10,
    "service": "my-app"
  }
}
```

Extensions (optional fields per Section 5 telemetry triggers):
- `trace_id` / `span_id` — W3C traceparent string fields, attached at receiver entry from external client headers; treated as opaque strings by `tracing` (no OTel SDK trace context binding)
- `duration_ms`, `span_count`, `service`
- Per-trigger fields: `plugin_path_basename`, `query_id`, `param_count`, `token_count_actual`, `token_budget_limit`, `body_size_bytes`, `webview_backend`, `tray_api`, `wgpu_backend`, `dedup_count`, `anomaly_markers`, `p50_ms`/`p95_ms`/`p99_ms`/`max_ms`, `error_rate_percent`, `value` (metric event payload)
- `service.name` / `service.version` / `deployment.environment` — populated as default subscriber fields per Service identity above
