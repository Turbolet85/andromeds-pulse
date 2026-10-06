### Status endpoint shape

_This is the TauRPC `health` IPC payload (arch §Standard Contracts), unchanged — it is NOT the `status` verb's output: since chunk `2026-08-30-diagnostics-un-muting-harness-truth-sweep` the harness `status` verb emits the `harness:status` verdict JSON above, and this envelope is reached only through IPC (`health`)._

```json
{
  "status": "ok" | "degraded" | "unhealthy",
  "subsystems": {
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "buffer": { "status": "ready" | "error", "rows_ingested": 1250, "retention_seconds": 600 },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": 3 }
  },
  "uptime_ms": 5432,
  "pid": 12345
}
```

**Agent reads:**
- `status == "ok"` for healthy; `"degraded"` acceptable if non-blocking subsystem failure
- `subsystems.*.status` all == `"initialized" | "ready"` (no `"error"`)
- `subsystems.buffer.rows_ingested` increment confirms OTLP ingest occurred
- `subsystems.ingest_channel.broadcast_subscribers >= 1` if real-time streams active
- `pid` matched against spawned process PID for identity verification
