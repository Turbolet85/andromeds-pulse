## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Standard
- **Justification:** Andromeda Pulse is a cross-platform desktop application (Windows/macOS/Linux via Tauri 2) with two primary surfaces and persistent in-memory data, spanning 19 entities with 7 critical user-facing flows.

### Test Harness Contract Summary

#### 5-command names:
- `boot` — Start the Tauri desktop application; initialize OTLP receivers on `:4317` (gRPC) and `:4318` (HTTP); spawn DuckDB in-memory buffer
- `run` — Invoke test suites via `cargo nextest run --workspace`
- `status` — Query product state via TauRPC `health` command
- `cleanup` — Terminate app process, close ports, verify graceful shutdown
- `logs` — Fetch product logs from `~/.andromeda-pulse/logs/` in JSON lines format

#### Status JSON shape:
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

### Critical Paths (must-be-accessible)

- **P1: Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard:** Send gRPC trace spans to `:4317`; query via TauRPC `traces.query`; assert response rows contain matching trace_ids.

- **P2: Generate token-efficient curated snapshot (not raw dump):** Populate buffer with 500 synthetic spans; invoke TauRPC `snapshot.generate` with 25000 token budget; assert response contains anomaly markers and dedup evidence.

- **P3: MCP server query (agent-accessible telemetry):** Spawn app with `ANDROMEDA_PULSE_MCP_ENABLED=true`; send JSON-RPC 2.0 `query_traces` call to stdin; assert `result.traces` array returned with correct schema.

- **P4: Plugin lifecycle (load, reload, invoke with capability scoping):** Stage fixture WASM module; invoke TauRPC `plugins.reload` and `plugins.invoke`; assert capability gating prevents disallowed operations.

- **P5: Widget compact mode ↔ dashboard expansion ↔ tray icon visibility toggle:** Verify IPC contract consistency via TauRPC `health` command; window/tray UI automation deferred to tauri-driver headful E2E.

- **P6: Real-time push of spans/metrics/logs via Tauri IPC Channel:** Subscribe to channel `pulse://stream/spans`; trigger gRPC ingest; assert binary Arrow IPC payload received with valid schema.

- **P7: Workspace detection (host project context correlation):** Invoke TauRPC `workspace.detect`; assert response contains detected project name and VCS info.

### Coverage Triggers Summary

- **OTLP loopback-only binding** (security-vector-coverage) — a11y implication: validate loopback-only binding enforcement via negative test (reject non-loopback source)

- **DuckDB SQL injection prevention** (security-vector-coverage) — a11y implication: validate parameter binding in prepared statements (no format-string SQL injection)

- **OTLP post-prost invariant checks** (security-vector-coverage) — a11y implication: validate span_id/trace_id length constraints (8/16 bytes); reject malformed payloads

- **Tauri IPC capability gating** (security-vector-coverage) — a11y implication: validate undeclared procedures are rejected; declared procedures succeed

- **Path canonicalization + confinement** (security-vector-coverage) — a11y implication: validate symlink chains and escape attempts are blocked

- **DefaultBodyLimit on OTLP HTTP** (security-vector-coverage) — a11y implication: validate oversized payloads rejected at HTTP 413 level

- **Cranelift-only WASM** (security-vector-coverage) — a11y implication: validate Cranelift backend enforced at build time

- **MCP feature + runtime gating** (security-vector-coverage) — a11y implication: validate both compile-time and runtime gates prevent unauthorized sidecar spawn

- **Minisign updater signature verification** (security-vector-coverage) — a11y implication: validate valid signatures accepted, invalid signatures rejected

- **No self-OTLP dialing** (security-vector-coverage) — a11y implication: validate receiver does not create infinite loop via self-dialing

- **OTLP protocol compliance (gRPC)** (compliance-test) — a11y implication: validate TraceService/MetricsService/LogsService RPCs conform to OpenTelemetry spec v1.x

- **OTLP protocol compliance (HTTP)** (compliance-test) — a11y implication: validate POST `/v1/traces`, `/v1/metrics`, `/v1/logs` with protobuf/JSON bodies conform to OpenTelemetry spec

- **WebGPU canvas throughput** (performance-budget) — a11y implication: validate 10k spans/sec injection sustained without buffer overflow or frame rate degradation

- **Snapshot token budget enforcement** (performance-budget) — a11y implication: validate snapshot generation respects token_budget <= 25000 limit

- **Buffer overflow / retention window enforcement** (chaos-test) — a11y implication: validate ring buffer eviction at 5–10 min window; bounded memory under sustained load

- **TauRPC ↔ IPC Channels consistency** (cross-surface-coordination) — a11y implication: validate TauRPC query results include rows from Channel events (no desync)

- **Snapshot generation + Notification emit** (cross-surface-coordination) — a11y implication: validate notification is emitted upon snapshot completion with token count

- **Multi-platform compat (Windows/macOS/Linux WebView2/WKWebView/GTK)** (multi-platform-compat) — a11y implication: validate boot, IPC, WebGPU availability, tray icon, notifications consistent across platforms

- **OTLP receiver saturation (HTTP vs gRPC trade-off)** (load-test) — a11y implication: validate concurrent HTTP and gRPC paths maintain < 100ms p99 latency without cross-protocol interference

- **OpenTelemetry protobuf evolution** (contract-test-against-OTLP-sandbox) — a11y implication: validate forward/backward compat with multiple protobuf versions

- **No manual verification checkpoints** (agent-driven-discipline) — a11y implication: all test assertions must be deterministic (exit code, structured output); no visual inspection required

- **Self-bootstrapping test data (no pre-baked DB)** (agent-driven-discipline) — a11y implication: all fixture data generated at runtime via OTLP ingest or Rust builders; no pre-baked snapshots

### Quality Gates Summary

- **Zero-flakiness statement:** Flaky tests are NOT tolerated. If a test flakes once: quarantine immediately via `#[ignore]` or CI conditional; root-cause investigation required; fix or delete before unquarantining. Rationale: agent-driven dev cannot distinguish flake from real bug; retry policies mask actual failures.

- **Coverage thresholds:** 
  - Line coverage ≥ 75%
  - Branch coverage ≥ 70%
  - Function coverage ≥ 85%
  - Exclude generated code (prost protobuf stubs, taurpc IPC bindings), test fixtures, and mock implementations from coverage metrics