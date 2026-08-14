## 5. Test Plan Excerpt

### Tests Tier

- **Tier:** Standard
- **Justification:** Andromeda Pulse is a cross-platform desktop application (Windows/macOS/Linux via Tauri 2) with two primary surfaces (desktop-webview: React + WebGPU dashboard; desktop-native: tray icon + menu) and persistent in-memory data (DuckDB ring buffer holding 5–10 min of OTLP telemetry), spanning 19 testable entities with 7 critical user-facing flows, security tier Minimal (loopback-only binding), and explicit rejection of 28+ attack vectors with high-quality rejection logic; creator's risk tolerance signals "portfolio-worthy GPU-accelerated visualization" approaching Jaeger UI quality, with agent-driven development requiring every test layer to be machine-parseable and runnable end-to-end with no human in the loop.

### Test Harness Contract Summary

- **5-command names:** boot, run, status, cleanup, logs
- **Status JSON shape:**
```json
{
  "status": "ok" | "degraded" | "unhealthy",
  "subsystems": {
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "buffer": { "status": "ready" | "error", "rows_ingested": number, "retention_seconds": number },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": number }
  },
  "uptime_ms": number,
  "pid": number
}
```
- **Log format JSON schema:**
```json
{"timestamp":"2026-05-02T16:18:34.567Z","level":"INFO","target":"ingest::grpc","message":"TraceService.Export received 10 spans","fields":{"span_count":10,"service":"my-app"}}
```

### Critical Paths (must-trace)

- **P1 (Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard):** Agent sends 100 gRPC trace spans to `:4317` via synthetic generator → awaits `status.subsystems.buffer.rows_ingested >= 100` → invokes TauRPC `traces.query({service: "test-app", time_range: [now-1m, now]})` → asserts response.rows.length > 0 with matching trace_ids; webview rendering unverified (assertion relies on WebGPU compilation success + IPC contract).
- **P2 (Generate token-efficient curated snapshot):** Populate buffer with 500 synthetic spans (varying service names, latencies, error rates) → invokes TauRPC `snapshot.generate({time_range: [now-10m, now], token_budget: 25000})` → asserts response.markdown contains anomaly markers (e.g., "⚠ latency spike" or "🔴 error cluster") AND response.token_count <= 25000 AND response.dedup_count shows reductions (not raw OTLP dump).
- **P3 (MCP server query):** Spawns app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` → pipes JSON-RPC 2.0 call to stdin: `{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "query_traces", "arguments": {"time_range": [now-5m, now], "service_filter": "my-app", "limit": 100}}}` → reads stdout for JSON-RPC result → asserts `result.traces` is array of trace objects with schema match.
- **P4 (Plugin lifecycle):** Stage fixture WASM module at `~/.andromeda-pulse/plugins/my-plugin.wasm` → invokes TauRPC `plugins.reload` → asserts response.plugins array contains entry with name "my-plugin" → invokes TauRPC `plugins.invoke({plugin: "my-plugin", capability: "transform_spans", input: […spans…]})` → asserts response contains transformed output; negative test invokes with disallowed capability → asserts rejection due to capability gating.
- **P5 (Widget compact mode ↔ dashboard expansion ↔ tray icon visibility toggle):** Deferred to tauri-driver headful E2E suite; agent-driven surrogate verifies IPC contract consistency via TauRPC `health` command with valid JSON response shape.
- **P6 (Real-time push via Tauri IPC Channel):** Subscribe to channel `pulse://stream/spans` via Tauri test client → send gRPC trace to `:4317` → await Channel event (timeout 5s) → receive binary Arrow IPC payload → decode schema (column names: trace_id, span_name, duration_ms, etc.) → assert row count > 0 and schema matches contract.
- **P7 (Workspace detection):** Boot app with CWD set to repository root (has `.andromeda/` marker) → invoke TauRPC `workspace.detect()` → assert response.workspace contains project_name, root_path, vcs_type ("git"), vcs_root (path to `.git/`).

### Coverage Triggers Summary

- **OTLP loopback-only binding** (security-vector) — obs implication: "Negative test: assert env var override `ANDROMEDA_PULSE_OTLP_GRPC_BIND=0.0.0.0:4317` is rejected; live receiver only accepts loopback source"
- **DuckDB SQL injection prevention** (security-vector) — obs implication: "Negative test: `snapshot.generate` with malicious service_filter (SQL payload) executes safely as literal string; no table dropped"
- **OTLP post-prost invariant checks** (security-vector) — obs implication: "Negative test: Send gRPC span with malformed span_id (< 8 bytes) or trace_id (> 16 bytes) → assert ingest rejects and logs error"
- **Tauri IPC capability gating** (security-vector) — obs implication: "Negative test: attempt undeclared procedure → assert TauRPC rejects with AppError; positive test: call declared procedure → assert success"
- **Path canonicalization + confinement** (security-vector) — obs implication: "Negative test: Set `ANDROMEDA_PULSE_PLUGIN_DIR` with escape attempt → assert canonicalization blocks traversal; symlink chain → assert resolved canonically"
- **DefaultBodyLimit on OTLP HTTP** (security-vector) — obs implication: "Negative test: POST oversized payload (> 8 MB) to `:4318 /v1/traces` → assert HTTP 413 or axum rejection; no panic"
- **Cranelift-only WASM** (security-vector) — obs implication: "Build-time test: assert `wasmtime` resolved with Cranelift feature enabled; negative test: attempt alternate backend → build fails"
- **MCP feature + runtime gating** (security-vector) — obs implication: "Negative test: (1) attempt spawn MCP without feature flag → build fails; (2) env var unset → sidecar not spawned; positive test: both gates enabled → sidecar spawns accepting JSON-RPC 2.0"
- **Minisign updater signature verification** (security-vector) — obs implication: "Positive test: valid Minisign signature in latest.json → updater accepts; negative test: invalid signature → updater rejects"
- **No self-OTLP dialing** (security-vector) — obs implication: "Negative test: configure with OTLP exporter pointing to own `:4317` or `:4318` → assert no self-dialing occurs; monitor network logs or IPC liveness"
- **OTLP protocol compliance (gRPC)** (compliance-test) — obs implication: "Valid OTLP gRPC TraceService.Export RPC (per OpenTelemetry spec) → server responds with status OK; malformed requests → gRPC error code"
- **OTLP protocol compliance (HTTP)** (compliance-test) — obs implication: "Valid OTLP HTTP POST `/v1/traces` with protobuf/JSON body → 200 OK; unsupported media type → 415 or parse attempt; invalid protobuf → error"
- **WebGPU canvas throughput** (performance-budget) — obs implication: "Inject 10,000 spans/sec to `:4317` gRPC for 10 seconds (100k total) → assert buffer ingests all without dropping; frame-rate validation (>= 30 fps) deferred to tauri-driver headful suite"
- **Snapshot token budget enforcement** (performance-budget) — obs implication: "Generate snapshot from 5000+ spans with token_budget=25000 → assert response.token_count <= 25000 (strict); stress test with 10k span backlog → assert snapshot fits"
- **Buffer overflow / retention window enforcement** (chaos-test) — obs implication: "Inject spans continuously at 10k/sec for 15 min (exceeds 10 min window) → assert oldest spans evicted; buffer memory stays bounded"
- **TauRPC ↔ IPC Channels consistency** (cross-surface-coordination) — obs implication: "Subscribe to `pulse://stream/spans` → send gRPC trace → call `traces.query` TauRPC → assert returned row set includes span from channel event"
- **Snapshot generation + Notification emit** (cross-surface-coordination) — obs implication: "Invoke `snapshot.generate` TauRPC → await `pulse://stream/snapshot-progress` events → assert notification emitted with token count"
- **Windows WebView2 vs macOS WKWebView vs Linux GTK WebKit** (multi-platform-compat) — obs implication: "Platform compat test matrix (Windows/macOS/Linux CI runners): verify tauri-driver harness boots app; IPC works consistently; WebGPU canvas available; tray icon renders; notifications dispatch via native API"
- **OTLP receiver saturation (HTTP vs gRPC trade-off)** (load-test) — obs implication: "Drive both receivers simultaneously; gRPC with 100 concurrent clients (100 span/sec); HTTP with 50 clients (50 span/sec); assert no cross-protocol interference; assert buffer ingests all"
- **OpenTelemetry protobuf evolution** (contract-test-against-OTLP-sandbox) — obs implication: "Use OpenTelemetry reference SDK to generate valid spans and send to app receivers; assert app accepts and buffers all; test with multiple protobuf versions for forward/backward compat"
- **No manual verification checkpoints** (agent-driven-discipline) — obs implication: "Every test exits with deterministic signal (exit code, structured stdout/JSON, log line); WebGPU rendering out of agent scope (assert IPC contract only); snapshot verified via token count + markers, not manual reading"
- **Self-bootstrapping test data** (agent-driven-discipline) — obs implication: "Every integration/E2E test generates fixture data at runtime via OTLP ingest or Rust builders; no `.sql` scripts or pre-baked snapshots; ensures repeatability and verifies ingest path"

### Quality Gates Summary

- **Zero-flakiness statement:** "Flaky tests are NOT tolerated. If a test flakes once: (1) Quarantine immediately (skip in CI via `#[ignore]` or GitHub Actions conditional); (2) Root-cause investigation required (not retry-once budget); (3) Fix or delete test before unquarantining. Rationale: agent-driven dev cannot distinguish flake from real bug; retry policies mask actual failures."
- **Coverage thresholds:** 75% line, 70% branch, 85% function (Standard tier cumulative across workspace; exclude generated code, test fixtures, mock implementations)
