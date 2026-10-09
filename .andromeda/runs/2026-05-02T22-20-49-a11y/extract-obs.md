## 6. Obs Plan Excerpt

### Obs Tier

- **Tier:** Standard
- **Justification:** "Andromeda Pulse is a cross-platform desktop application (Windows/macOS/Linux via Tauri 2) with 8 instrumentable modules and 8 major surfaces with multi-surface coordination requiring cross-surface trace context propagation, creator brief explicitly asking for token-efficient snapshot guarantees, real-time push via binary Arrow IPC and frontend telemetry flow, and security plan specifying 6 logging-sensitive vectors driving structured instrumentation."

### Log Format JSON Schema (binding)

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

**Extensions (optional fields per telemetry triggers):**
- `trace_id` / `span_id` — W3C traceparent string fields
- `duration_ms`, `span_count`, `service`
- Per-trigger fields: `plugin_path_basename`, `query_id`, `param_count`, `token_count_actual`, `token_budget_limit`, `body_size_bytes`, `webview_backend`, `tray_api`, `wgpu_backend`, `dedup_count`, `anomaly_markers`, `p50_ms`/`p95_ms`/`p99_ms`/`max_ms`, `error_rate_percent`, `value`
- `service.name` / `service.version` / `deployment.environment` — populated as default subscriber fields

### Service Identity

- **service.name:** `"com.andromeda.pulse"` (compile-time constant, Tauri bundle identifier; for mcp-server sidecar: `"andromeda-pulse-mcp"`)
- **service.version:** compile-time `env!("CARGO_PKG_VERSION")` (or runtime read from `tauri.conf.json` for the bundled desktop build)
- **deployment.environment:** hardcoded `"production"` (desktop app, no staging/dev distinction at runtime)

### Sentry User-Feedback Widget (if applicable)

(No error reporting platform in obs plan — a11y Phase 3 will derive default a11y discipline for user-feedback if Phase 1 introduces error reporting; otherwise N/A.)

### Focus-Relevant Span Coverage (filtered)

(No focus-relevant spans in obs plan Section 4 — a11y Phase 3 will recommend focus tracing spans that obs may add later: focus.shift / focus.trap.enter / focus.trap.exit / focus.restore.)