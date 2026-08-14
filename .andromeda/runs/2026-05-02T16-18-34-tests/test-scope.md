## 1. Coverage Scope

| Entity | Source | Testability | Reason if Not Fully Testable |
|--------|--------|-------------|------------------------------|
| **ingest (OTLP/gRPC receiver)** | Architecture Stack → tonic 0.14.x, Workspace/Modules | testable | Receives TraceService/MetricsService/LogsService on `:4317` per opentelemetry-proto; drive via native gRPC client with test spans; assert via DuckDB buffer query. |
| **ingest (OTLP/HTTP receiver)** | Architecture Stack → axum 0.8.x on hyper 1.x, Workspace/Modules | testable | Accepts POST `/v1/traces`, `/v1/metrics`, `/v1/logs` on `:4318` with protobuf/JSON; drive via HTTP client; signal via buffer inspection. |
| **buffer (DuckDB ring buffer)** | Architecture Stack → DuckDB 1.5.x, Workspace/Modules | testable | In-memory columnar OLAP store; direct SQL queries verify row insertion, retention window enforcement (5–10 min), Arrow appender zero-copy hand-off. |
| **viz (query aggregation layer)** | Architecture Stack → Apache Arrow, Workspace/Modules | testable | Aggregates buffer data by service/time-bucket for WebGPU visualization; drive via TauRPC `traces.query` / `metrics.query` / `logs.query` routers; assert aggregation correctness via DuckDB. |
| **ui-bridge (TauRPC IPC)** | Architecture Stack → TauRPC (taurpc crate), Workspace/Modules, Standard Contracts (`app_info`, `health`, `ready`, all query/snapshot routers) | testable | Rust ↔ webview IPC bridge with auto-generated TypeScript bindings; drive via Tauri test client IPC harness; signal via structured JSON responses. |
| **snapshot (curated markdown generator)** | Architecture Workspace/Modules, Creator Brief excerpt | testable | Generates 10k/25k/50k token-balanced snapshots with dedup, anomaly highlighting, critical-path extraction, p50/p95/p99 aggregation, hierarchical markdown output; drive via `snapshot.generate` TauRPC; assert token count and presence of anomaly markers. |
| **workspace-detector** | Architecture Workspace/Modules | testable | Detects host project context for telemetry correlation; drive via `workspace.detect` / `workspace.list` TauRPC; assert presence of `.andromeda/` or inferred project metadata. |
| **plugins (WASM Component Model host)** | Architecture Stack → wasmtime 25+ with WASM Component Model, Workspace/Modules, Standard Contracts (`plugins.list`, `plugins.reload`, `plugins.invoke`) | partially-testable | Plugin lifecycle (load, reload, list) testable via TauRPC; arbitrary guest behavior testable via fixture WASM modules with known signatures; sandbox enforcement (capability-scoped access) testable via negative test asserting guest cannot access disallowed imports. |
| **mcp-server (rmcp stdio sidecar)** | Architecture Stack → rmcp (official Rust SDK), Workspace/Modules, Standard Contracts (`query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`), Security Plan Vector 4 | partially-testable | Feature-gated (`--features mcp-server`) and runtime-gated (`ANDROMEDA_PULSE_MCP_ENABLED=true`); JSON-RPC 2.0 over stdin/stdout testable via subprocess pipe; caller is LLM with no network exposure; cannot fully test external LLM agent behavior (out of harness scope). |
| **pulse-app (Tauri binary crate)** | Architecture Workspace/Modules, Platform → desktop-webview + desktop-native | testable | Wires all library crates and webview; drive via Tauri test harness (webview driver); signal via window visibility, IPC responses, desktop notifications. |
| **Real-time push channels (Tauri IPC)** | Architecture Standard Contracts → `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events` | testable | Binary Arrow IPC payloads on Tauri 2 IPC Channel API; drive by subscribing to channels and triggering data events; assert Arrow schema validity and row count via embedded decoder. |
| **OTLP anti-patterns rejection** | Security Plan → 15 NEVER rules (post-`prost` invariant checks, DuckDB SQL injection prevention, DefaultBodyLimit, Cranelift-only WASM, etc.) | testable | Each rejection is a negative test: assert `prost`-decoded payloads fail invariant checks if malformed; assert SQL with user input via parameters only (never format strings); assert DefaultBodyLimit rejects oversized requests; assert non-Cranelift configuration rejected at build time. |
| **Tauri capability gating** | Architecture Standard Contracts (pulse:default), Security Plan Vector 2, Anti-Pattern → "NEVER add TauRPC procedure without matching capability entry" | testable | Tauri 2 capability model enforces at IPC invocation; negative test: attempt to call undeclared procedure, assert rejection with AppError. Xtask drift check enforces sync between crate and capabilities/ manifest. |
| **tauri-plugin-updater (Minisign verification)** | Architecture Stack → tauri-plugin-updater 2.x, Security Plan Vector 6, Anti-Pattern → "NEVER override Minisign verification" | testable | Updater public key baked into tauri.conf.json; contract test: mock latest.json with valid signature verifies update acceptance; with invalid signature (negative test) verifies rejection. Cannot test HSM private key (out of runtime scope); covered by code-signing CI workflow tests. |
| **Loopback-only OTLP binding** | Security Plan Vector 1, Anti-Pattern → "NEVER bind OTLP receivers to 0.0.0.0" | testable | Negative test: assert env var override or config attempt to bind to non-127.0.0.1 is canonicalized and rejected; assert live receivers only accept loopback source. |
| **Path canonicalization + confinement** | Security Plan Vector 5, Anti-Pattern → "NEVER read path env vars without Path::canonicalize() + confinement" | testable | Negative test: ANDROMEDA_PULSE_*_PATH env vars with symlink chains, TOCTOU attempts, escape sequences; assert canonicalization resolves and confinement assertion blocks traversal. |
| **config.toml reading** | Security Plan Data Classifications → config (sensitivity: low), Workspace/Modules | testable | Read from per-platform data dir; drive via env var override; assert settings parsed correctly; negative test: malformed TOML rejects gracefully. |
| **DuckDB prepared statements** | Security Plan Anti-Pattern → "NEVER format user input directly into SQL" | testable | Negative test: snapshot filter by service_name / trace_id / latency bounds; assert user input only appears as parameter in prepared statement (no format-string SQL injection). |
| **Self-observation loop prevention** | Creator Brief → "instrumenting an OTLP receiver with an OTLP network exporter pointed back at itself creates an infinite loop" | testable | Negative test: configure product with OTLP exporter pointing to own `:4317` or `:4318`; assert no self-dialing occurs (monitored via network log or IPC liveness check). |

---

## 2. Surfaces Under Test

| Surface | Driver | Signal | Boundary | Notes |
|---------|--------|--------|----------|-------|
| **desktop-webview (React 19 + Tauri 2)** | tauri-driver (Tauri CLI test harness) with WebDriver protocol | WebView window handle obtained; IPC command responses JSON-parseable; WebGPU canvas rendering asserted via pixel inspection or frame buffer read (structured stdout from tauri-driver) | single-surface (webview only; Tauri native integration tested separately) | Headless or headful depending on CI runner (Windows / macOS / Linux have native webview engines WKWebView/WebView2/GTK WebKit); tauri-driver requires `TAURI_TESTING_ENABLED` and `TAURI_PRIVATE_URI` for automated control. |
| **desktop-native (Tauri tray icon + context menu)** | tauri-driver + platform-specific ATI: Windows/macOS/Linux tray detection | Tray menu state (visibility, menu items) asserted via native accessibility tree or IPC liveness (`health` command returns subsystem states); notification delivery to Notification Center / Action Center / freedesktop verifiable via OS notification spy (platform-dependent fixture). | cross-surface (tray icon state tied to webview visibility toggle; notifications driven by snapshot completion event) | Tauri 2 tray + notification plugins are cross-platform wrappers; tray menu click simulation requires platform-specific ATI (xdotool + AT-SPI on Linux, AppleScript/XCUITest on macOS, pywinauto on Windows); notifications testable via IPC event capture (Channel API). |
| **OTLP/gRPC receiver (`:4317`)** | gRPC client library (tonic client, or language-native gRPC stub) | Server responds to TraceService.Export / MetricsService.Export / LogsService.Export with status OK (gRPC Code::OK); agent observes via exit code + structured gRPC error metadata; payload delivery asserted via DuckDB buffer query. | single-surface (receiver only; no client-side persistence or upstream integration tested here) | Loopback-only binding on 127.0.0.1:4317; test client must speak proper gRPC with protobuf wire format; oversized messages must trigger DefaultBodyLimit rejection (asserted via gRPC Code::ResourceExhausted). |
| **OTLP/HTTP receiver (`:4318`)** | HTTP client (curl / httpie / language-native HTTP library) | Server responds HTTP 200 OK to POST `/v1/traces` / `/v1/metrics` / `/v1/logs` with JSON or protobuf request; agent observes via exit code + response body (empty on success or error JSON on failure); payload delivery asserted via DuckDB buffer query. | single-surface (receiver only) | Loopback-only binding on 127.0.0.1:4318; must validate DefaultBodyLimit rejection of oversized payloads (HTTP 413 Payload Too Large or equivalent); must validate Host-header allowlist (reject non-localhost Host: headers per DNS-rebinding mitigation). |
| **TauRPC IPC procedures (app_info, health, ready, traces.*, metrics.*, logs.*, snapshot.*, plugins.*, workspace.*)** | Tauri IPC test client (built into tauri-driver or custom Rust test harness using `tauri::ipc::invoke`) | Procedure returns JSON response (structured, matching contract envelope); `health`/`ready` return status object with subsystem state booleans; `traces.query` / `metrics.query` / `logs.query` return paginated result sets; agent observes via jq extraction of specific fields (e.g., `status == "ok"`, `ready == true`, `result.rows | length > 0`). | single-surface (IPC only; data consumed by webview or external MCP client, but IPC itself is procedural stateless) | All procedures must serialize errors to AppError enum (never expose stack traces, file paths, or struct names per anti-pattern); Tauri capability gating enforces at invocation (negative test: call undeclared procedure asserts Tauri rejection). |
| **Real-time IPC Channels (pulse://stream/spans, etc.)** | Tauri IPC Channel subscription in test client | Agent subscribes to channel, triggers ingest event (e.g., send gRPC span), receives binary Arrow IPC payload on channel; agent validates Arrow schema (column names, types) via embedded Arrow decoder; asserts row count > 0. | cross-surface (channels emit from buffer/viz layer; consumed by webview or external subscribers) | Tauri 2 Channel API is async; agent must handle async events; Arrow IPC payload is zero-copy columnar; test must decode and validate schema integrity. |
| **MCP stdio sidecar (rmcp on stdin/stdout)** | subprocess spawning with feature flag `--features mcp-server` + env `ANDROMEDA_PULSE_MCP_ENABLED=true` + JSON-RPC 2.0 client writing to stdin | JSON-RPC 2.0 response on stdout with `result` (success) or `error` (failure); agent observes via exit code (must be 0 on graceful shutdown) + stdout line parsing (jq-extractable `result.traces` / `result.metrics` / `result.logs` arrays); MCP tool call (e.g., `query_traces(time_range, service_filter, limit)`) asserted via call/result round-trip with structured output. | single-surface (stdio sidecar only; caller is external LLM agent, untestable in harness) | Feature-gated at compile time; both compile-time and runtime gates must be asserted in negative test (attempt to spawn without feature flag must fail). Cannot test external LLM behavior; only test that tool schema is valid and tool invocations return correct data shape. |

---

## 3. Test Harness Specification

### 5-Command Requirements

**Command 1: `boot`**
- **Input:** None (or environment variables `ANDROMEDA_PULSE_*_PORT`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_CONFIG_PATH`, `RUST_LOG`)
- **Starts:** Tauri desktop application (`pulse-app` binary); launches webview window, initializes OTLP receivers on `:4317` (gRPC) and `:4318` (HTTP), spawns DuckDB in-memory buffer, optionally spawns MCP sidecar if `--features mcp-server` and `ANDROMEDA_PULSE_MCP_ENABLED=true`
- **Readiness Signal:**
  - `health` TauRPC command returns `status: "ok"` and all subsystems report non-error state (otlp_grpc_receiver, otlp_http_receiver, buffer, ingest_channel all initialized)
  - OR `ready` TauRPC command returns `ready: true` (stricter gate: all readiness checks pass, including duckdb_connection confirmed, broadcast subscribers initialized)
  - Log line: `ANDROMEDA_PULSE_*` environment variables echoed at startup (no secrets); receiver bind confirmation logs appear (`listening on 127.0.0.1:4317 gRPC`, `listening on 127.0.0.1:4318 HTTP`)
- **Exit Code Semantics:** 0 = success (application running, IPC ready); non-zero = startup failure (agent captures stderr log, aborts test)
- **Observable Port Readiness:** Agent confirms `:4317` and `:4318` accept connections (TCP handshake succeeds) before declaring boot complete

---

**Command 2: `run`**
- **Input:** Test suite selection (e.g., `cargo xtask test`, `cargo test --workspace`, or specific test target `cargo test --lib ingest`)
- **Invokes:** Rust `cargo test` framework (co-located unit tests in `src/`, no separate `tests/` directory per convention); tests inject synthetic OTLP telemetry via gRPC/HTTP to loopback `:4317`/`:4318`; end-to-end tests query buffer via TauRPC to assert ingestion
- **Exit Code Semantics:** 0 = all tests passed; non-zero = failure count (agent captures stderr/stdout test output, parses failure list)
- **Output Format:** Standard Rust test harness: `test result: ok. X passed, Y failed` on final line; agent extracts count via regex
- **Test Organization:**
  - Unit tests: per-crate `#[test]` functions (ingest, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server)
  - Integration tests: `#[tokio::test]` spawning full `pulse-app`, injecting OTLP, querying via TauRPC
  - Platform-specific tests: gated via `#[cfg(target_os = "…")]` for tray/notification/updater behavior

---

**Command 3: `status`**
- **Mechanism:** TauRPC IPC command `health` (liveness) or custom `/status` endpoint if HTTP server exposed (not in current arch, so use TauRPC)
- **Query Method:** Agent invokes TauRPC `health` command via Tauri test harness (or custom IPC client); polls every 500ms until response or timeout (10s)
- **Structured Response Shape (JSON):**
  ```json
  {
    "status": "ok" | "degraded" | "unhealthy",
    "subsystems": {
      "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
      "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
      "buffer": { "status": "ready" | "error", "rows_ingested": number },
      "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": number }
    },
    "uptime_ms": number,
    "pid": number
  }
  ```
- **Agent-Polled Fields:**
  - `status`: assert == "ok" (healthy) or "degraded" (acceptable for non-blocking subsystem failures)
  - `subsystems.*.status`: all == "initialized" | "ready"
  - `subsystems.buffer.rows_ingested`: for E2E tests, assert increases after OTLP ingest
  - `pid`: for process cleanup verification (PID must match spawned process or parent Tauri PID)
- **Failure Signal:** If `health` command times out or returns error, agent logs and escalates

---

**Command 4: `cleanup`**
- **Teardown:** Close Tauri window (IPC `window.close` or platform-native close), terminate app process (SIGTERM to PID from `status`), wait max 5s for graceful shutdown
- **Idempotency:** Can be called multiple times; second call on already-closed app is no-op (checking if PID exists via `ps` or equivalent; if not running, cleanup succeeds)
- **Verification of Completion:**
  - Port `:4317` and `:4318` no longer accept new connections (TCP handshake fails)
  - PID no longer exists (`ps` shows no process with captured PID)
  - Log file is flushed and finalized (no further writes within 2s timeout)
- **Exit Code:** 0 = cleanup succeeded (app exited, ports released); non-zero = forcible termination required (SIGKILL after SIGTERM timeout)

---

**Command 5: `logs`**
- **Location:**
  - Primary: `~/.andromeda-pulse/logs/` (configurable via `ANDROMEDA_PULSE_LOG_DIR` env var, defaults to platform XDG_DATA_HOME)
  - CI override: `ANDROMEDA_PULSE_LOG_DIR=$PWD/.test-logs/` for test isolation
- **Format:** Plain text or JSON lines (one JSON object per line, containing `timestamp`, `level` (debug/info/warn/error), `target` (crate name), `message`)
- **Retention:** On-disk for test debugging (no auto-purge within test suite); agent may tail or pipe `RUST_LOG=debug cargo test 2>&1 | tee logs/test.log`
- **Parseable by Agent:**
  - Regex match: `\[ERROR\].*` for error detection
  - JSON line parse: `jq '.level == "error"' logs/app.log` for structured queries
  - Grep + count: `grep -c "OTLP.*ingested" logs/app.log` for ingest event count

---

### Status Endpoint Shape

(Inherited from TauRPC `health` IPC command per Test Harness Commitment)

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

**Fields Agent Polls:**
- `status`: "ok" for healthy, "degraded" acceptable if buffer or subscribers issue (non-blocking)
- `subsystems.*.status`: all must be non-error
- `subsystems.buffer.rows_ingested`: incremented after each OTLP ingest (E2E test assertion)
- `subsystems.ingest_channel.broadcast_subscribers`: must be > 0 if any real-time streams active
- `pid`: matched against spawned Tauri process for identity verification

---

### Log Format

**Stream:** `RUST_LOG=debug cargo test 2>&1 | tee ~/.andromeda-pulse/logs/test.log`

**Format:** Rust `tracing` crate JSON output (or Env subscriber with format control)

Example line:
```json
{ "timestamp": "2026-05-02T16:18:34.567Z", "level": "INFO", "target": "ingest::grpc", "message": "TraceService.Export received 10 spans", "fields": { "span_count": 10, "trace_ids": "[…]" } }
```

**Agent-Parseable Signals:**
- Error detection: `jq 'select(.level == "ERROR") | .message' logs/*.log | head -20` (capture first 20 errors for report)
- Ingest confirmation: `grep -o "rows_ingested: [0-9]*" logs/*.log | tail -1` (last ingest count)
- Subsystem startup: `grep "listening on 127.0.0.1:\(4317\|4318\)" logs/*.log` (confirm receiver binds)

---

### PID File Location

**Primary:** `$XDG_RUNTIME_DIR/andromeda-pulse.pid` (Linux) | `$TMPDIR/andromeda-pulse.pid` (macOS) | `%LOCALAPPDATA%\andromeda-pulse\pid` (Windows)

**Fallback:** If XDG_RUNTIME_DIR not set, use `~/.andromeda-pulse/run/andromeda-pulse.pid`

**Agent Usage:**
```bash
# boot: write PID on successful startup
echo $PID > $PID_FILE

# status: read PID and verify process exists
PID=$(cat $PID_FILE 2>/dev/null)
ps -p $PID > /dev/null && echo "OK" || echo "DEAD"

# cleanup: terminate process via PID
kill -TERM $PID 2>/dev/null
```

---

### Test Data Strategy

**Self-Bootstrapping Fixture Mechanism:**

1. **OTLP Ingest as Fixture Seeding:** Tests do NOT pre-populate DuckDB; instead, each test harness seeds the buffer by sending synthetic OTLP telemetry via gRPC or HTTP to `:4317` / `:4318`. This avoids test-specific database snapshots and keeps data generation declarative.

2. **Synthetic Data Generators:** Per-crate fixture factories (Rust builder pattern):
   - `ingest`: `MockTraceSpan::builder().service("my-app").latency_ms(50).build()` → serializes to OTLP gRPC protobuf and sends to loopback `:4317`
   - `buffer`: `MockArrowBatch::builder().rows(100).columns(["trace_id", "span_name", "duration_ms"]).build()` → inserts via Arrow appender
   - `viz`: `MockMetricPoint::builder().metric_name("requests.total").value(42).timestamp(…).build()` → sent to HTTP `:4318` as OTLP JSON
   - `snapshot`: Pre-stage 1000 spans in buffer, then invoke `snapshot.generate` IPC with time window + filters; assert markdown output contains anomaly markers
   - `plugins`: Fixture WASM modules (pre-compiled minimal Component Model binaries with known input/output) loaded from `tests/fixtures/plugins/`

3. **Retention Window Testing:** DuckDB in-memory ring buffer (5–10 min window) — tests explicitly set retention time via config override (`ANDROMEDA_PULSE_BUFFER_RETENTION_SEC=300`) and verify old rows are discarded after TTL expiry (negative test: assert row count decreases as retention window rolls).

4. **Property-Based Generators:** For stress tests and cardinality exploration (e.g., "10k spans/sec ingest throughput"), use `proptest` crate to generate random valid OTLP payloads and assert buffer handles them without panic or memory exhaustion.

5. **No Developer-Seeded DB:** All fixture data is generated at test runtime via OTLP or programmatic Rust builders; no `.sql` scripts or pre-baked SQLite files. This ensures test isolation and repeatability.

---

## 4. Critical Paths

| Path | Surfaces Involved | Verification Signal | Source |
|------|-------------------|-------------------|--------|
| **P1: Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard** | OTLP/gRPC receiver (`:4317`) + ingest module + buffer (DuckDB) + viz aggregation + TauRPC `traces.query` + desktop-webview (WebGPU canvas) | Agent: sends 100 gRPC trace spans to `:4317` via synthetic generator → awaits `status.subsystems.buffer.rows_ingested >= 100` → invokes TauRPC `traces.query({service: "test-app", time_range: [now-1m, now]})` → asserts response.rows.length > 0 with matching trace_ids → (webview rendering unverified; canvas pixel inspection out of scope for agent-driven tests, assertion relies on WebGPU compilation success + IPC contract). | Architecture Intent: "receives OTLP telemetry (HTTP `:4318` and gRPC `:4317`) from any local application, visualizes traces"; Creator Brief: "receives OTLP telemetry (HTTP `:4318` and gRPC `:4317`) from any local application, visualizes traces, metrics, and logs in a polished GPU-accelerated UI" |
| **P2: Generate token-efficient curated snapshot (not raw dump)** | buffer (DuckDB) + snapshot module (curated generator) + TauRPC `snapshot.generate` + desktop-webview (UI trigger) + tauri-plugin-notification (Notification Center emit) | Agent: populates buffer with 500 synthetic spans (varying service names, latencies, error rates) → invokes TauRPC `snapshot.generate({time_range: [now-10m, now], token_budget: 25000})` → asserts response.markdown contains anomaly markers (e.g., "⚠ latency spike" or "🔴 error cluster") AND response.token_count <= 25000 AND response.dedup_count shows reductions (not raw OTLP dump) → invokes `snapshot.copy_to_clipboard` → (clipboard content unverified in agent harness; assertion relies on successful IPC return). | Creator Brief: "generates **curated** snapshots" (not raw OTLP JSON dump); "token-efficient curated snapshot"; Anti-Pattern: "No raw OTLP JSON dump as snapshot: Existing snapshot tools dump raw OTLP JSON — а production trace at moderate load = hundreds of thousands of tokens" |
| **P3: MCP server query (agent-accessible telemetry)** | MCP stdio sidecar (feature-gated `--features mcp-server`) + buffer access + JSON-RPC 2.0 contract | Agent: spawns app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` → pipes JSON-RPC 2.0 call to stdin: `{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "query_traces", "arguments": {"time_range": [now-5m, now], "service_filter": "my-app", "limit": 100}}}` → reads stdout for JSON-RPC result → asserts `result.traces` is array of trace objects with schema match (trace_id, span_name, duration_ms, etc.) → (external LLM client behavior untestable; only test that tool invocation returns correct data shape). | Architecture Intent: "Optional MCP server lets AI agents query telemetry directly"; Creator Brief: "AI agents query telemetry via MCP `tools/call` requests" |
| **P4: Plugin lifecycle (load, reload, invoke with capability scoping)** | plugins module (WASM Component Model host) + TauRPC `plugins.list` / `plugins.reload` / `plugins.invoke` + `~/.andromeda-pulse/plugins/` filesystem | Agent: stage fixture WASM module at `~/.andromeda-pulse/plugins/my-plugin.wasm` → invokes TauRPC `plugins.reload` → asserts response.plugins array contains entry with name "my-plugin" → invokes TauRPC `plugins.invoke({plugin: "my-plugin", capability: "transform_spans", input: […spans…]})` → asserts response contains transformed output → (negative test) invokes with disallowed capability → asserts rejection due to capability gating. | Architecture Stack: "wasmtime 25+ with WASM Component Model + WIT — plugin runtime with capability-scoped third-party extensions"; Creator Brief: "Custom dashboards — load WASM module; Data transforms — WASM module receives ingest stream; Snapshot templates — WASM module receives curated snapshot" |
| **P5: Widget compact mode ↔ dashboard expansion ↔ tray icon visibility toggle** | desktop-webview (React widget + dashboard layouts) + desktop-native (tray icon + menu) + Tauri window management + TauRPC state sync | Agent: (limited to IPC verification; actual window resize/visibility untestable without tauri-driver GUI automation) → invokes TauRPC `health` to confirm window state in metadata (optional, may not be present) → OR: inspects Tauri process for window creation events (CLI / OS-level window spy) → (full GTK/AppKit/WinAPI window management tested via tauri-driver headful mode only; headless agent-driven tests verify IPC state consistency only). | Creator Brief: "Quarter-screen widget mode — default surface; Click to expand — opens full dashboard window; widget remains mounted (returns to compact mode on close); Tray icon (secondary) — traffic-light status; click cycles widget visibility" |
| **P6: Real-time push of spans/metrics/logs via Tauri IPC Channel** | buffer (DuckDB) + TauRPC IPC Channels (`pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`) + desktop-webview (listener) + Real-time push contract (Arrow IPC payloads) | Agent: subscribes to channel `pulse://stream/spans` via Tauri test client → sends gRPC trace to `:4317` → awaits Channel event (timeout 5s) → receives binary Arrow IPC payload → decodes schema (column names: trace_id, span_name, duration_ms, etc.) → asserts row count > 0 and schema matches contract. | Architecture Standard Contracts: "Real-time push contract (Tauri IPC channels) — live span/metric/log streams using Tauri 2 IPC `Channel` API with binary Arrow IPC payloads; event names are `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`" |
| **P7: Workspace detection (host project context correlation)** | workspace-detector module + TauRPC `workspace.detect` / `workspace.list` | Agent: (test harness runs inside repo with `.andromeda/` marker or typical project structure) → invokes TauRPC `workspace.detect` → asserts response.workspace contains detected project name / root path / VCS info (git, cargo.toml, etc.) → (out-of-repo test scenarios untestable in standard agent-driven harness; assume repo structure is present). | Creator Brief: "Snapshot path detection — if cwd has `.andromeda/` → `.andromeda/pulse/{timestamp}.md`; else → `~/.cache/andromeda-pulse/snapshots/{timestamp}.md`"; Architecture Workspace: "workspace-detector — detection of host project context for telemetry correlation" |

---

## 5. Coverage Triggers

| Trigger Type | Source | Required Test Type |
|--------------|--------|-------------------|
| **security-vector-coverage: OTLP loopback-only binding** | Security Plan Vector 1: "OTLP receivers bound to 127.0.0.1 only" | Negative test: Attempt env var override `ANDROMEDA_PULSE_OTLP_GRPC_BIND=0.0.0.0:4317` → assert canonicalization + confinement rejects; assert live receiver only accepts loopback source (connect from non-loopback IP must fail at TCP level or IPC auth). |
| **security-vector-coverage: DuckDB SQL injection prevention** | Security Plan Vector 2, Anti-Pattern: "NEVER format user input directly into SQL … ALWAYS use `Connection::prepare` with `?` placeholders and parameter binding" | Negative test: `snapshot.generate` with service_filter containing SQL injection payload (e.g., `"my-service'; DROP TABLE spans; --"`) → assert query executes safely (payload treated as literal string, no table dropped); assert output schema unchanged. |
| **security-vector-coverage: OTLP post-prost invariant checks** | Security Plan Anti-Pattern: "NEVER skip post-`prost`-decode invariant checks on OTLP payloads … `span_id` (8 bytes), `trace_id` (16 bytes), attribute key sizes, etc. must be checked" | Negative test: Send gRPC span with malformed span_id (< 8 bytes) or trace_id (> 16 bytes) → assert ingest rejects and logs error (rows not inserted); assert no panic in buffer layer. |
| **security-vector-coverage: Tauri IPC capability gating** | Security Plan Vector 2: "Tauri 2 capability gating `pulse:default`"; Anti-Pattern: "NEVER add TauRPC procedure without matching capability entry" | Negative test: Attempt to invoke undeclared procedure (not in `pulse:default` capability list) → assert TauRPC rejects with AppError; positive test: call declared procedure (`app_info`, `health`, `traces.query`, etc.) → assert success. Xtask drift check enforces crate/capability sync. |
| **security-vector-coverage: Path canonicalization + confinement** | Security Plan Vector 5: "Canonicalize and confine all `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars"; Anti-Pattern: "NEVER read path env vars without `Path::canonicalize()` + confinement assertion" | Negative test: Set `ANDROMEDA_PULSE_PLUGIN_DIR=~/.andromeda-pulse/plugins/../../../` (escape attempt) → assert canonicalization resolves and confinement blocks traversal; assert only loads plugins from resolved safe dir. Negative test: symlink chain in plugin dir → assert resolved canonically; TOCTOU race attempt → assert atomic. |
| **security-vector-coverage: DefaultBodyLimit on OTLP HTTP** | Security Plan Anti-Pattern: "NEVER trust `Content-Length` or trailers as substitute for `DefaultBodyLimit` on OTLP HTTP `:4318`" | Negative test: POST oversized payload (> 8 MB, size limit not specified in arch, assume 8 MB default) to `:4318 /v1/traces` → assert HTTP 413 Payload Too Large or axum rejection; assert no panic or unbounded memory allocation. |
| **security-vector-coverage: Cranelift-only WASM** | Security Plan Anti-Pattern: "NEVER use non-Cranelift `wasmtime` feature flag on x86_64" | Build-time test: Assert `wasmtime` Cargo.lock resolved with Cranelift feature enabled (or only available backend); negative test: Attempt build with alternate backend → build fails or CI reject (xtask check enforces). |
| **security-vector-coverage: MCP feature + runtime gating** | Security Plan Vector 4: "gated by compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`" | Negative test: (1) attempt to spawn MCP sidecar without feature flag compile → build fails; (2) runtime env var unset, attempt to spawn → assert sidecar not spawned (cannot find binary or refused at startup). Positive test: both gates enabled → sidecar spawns and accepts JSON-RPC 2.0. |
| **security-vector-coverage: Minisign updater signature verification** | Security Plan Vector 6: "Minisign Ed25519 signature verification is mandatory and cannot be disabled" | Positive test: Mock latest.json with valid Minisign signature → updater accepts. Negative test: Invalid signature (flipped bit in sig) → updater rejects update (no update downloaded). Negative test: Attempt to bypass verification via config → assert Tauri enforces (cannot override). |
| **security-vector-coverage: No self-OTLP dialing** | Creator Brief Anti-Pattern: "Any future `ANDROMEDA_OBSERVER_URL`-shaped variable must explicitly distinguish 'outbound observer' (we *are* the observer — do not dial) from 'inbound receivers'" | Negative test: Configure product with OTLP exporter pointing to own `:4317` or `:4318` (via config.toml or env var injection) → assert no self-dialing occurs; monitor network logs or IPC liveness; assert no infinite loop or exponential span multiplication. |
| **compliance-test: OTLP protocol compliance (gRPC)** | Architecture Stack: "tonic 0.14.x … gRPC server for OTLP/gRPC receiver on `:4317` with protobuf codegen"; "per opentelemetry-proto" | Contract test: Valid OTLP gRPC TraceService.Export RPC (per OpenTelemetry spec v1.x) → server responds with status OK and exports span to buffer. Valid MetricsService.Export and LogsService.Export RPCs similarly. Malformed requests (missing required fields) → server responds with gRPC error code (INVALID_ARGUMENT or UNIMPLEMENTED). |
| **compliance-test: OTLP protocol compliance (HTTP)** | Architecture Stack: "axum 0.8.x … HTTP server for OTLP/HTTP receiver on `:4318` supporting protobuf and JSON"; "per OpenTelemetry spec" | Contract test: Valid OTLP HTTP POST `/v1/traces` with protobuf body → 200 OK, export to buffer. Valid JSON body similarly. Unsupported media type (Content-Type: text/plain) → 415 Unsupported Media Type or server attempts to parse (spec permissive). Invalid protobuf wire format → server responds with error (no panic). |
| **performance-budget: WebGPU canvas throughput** | Creator Brief Risk Tolerance: "Hand-built Canvas falls over at 10k+ spans/sec. Pulse v2 uses WebGPU … Smooth animation; no jank at high cardinality"; "handles 10k+ spans/sec" (target throughput baseline) | Performance test: Inject 10,000 spans/sec to `:4317` gRPC for 10 seconds (100k total) → assert buffer ingests all without dropping; assert WebGPU canvas renders without frame drops (frame rate stays >= 30 fps, no hangs); assert buffer memory usage stays within 5–10 min ring buffer window (no unbounded growth). |
| **performance-budget: Snapshot token budget enforcement** | Creator Brief: "Token budget (10k / 25k / 50k preset)"; must-work scenario: "generates curated snapshot" with token limits | Performance test: Generate snapshot from 5000+ spans with token_budget=25000 → assert response.token_count <= 25000 (strict); assert snapshot quality (anomaly markers present, critical path extracted, not raw OTLP dump). Stress test: token_budget=10000 with 10k span backlog → assert snapshot still fits (aggressive culling expected). |
| **chaos-test: Buffer overflow / retention window enforcement** | Architecture: "DuckDB 1.5.x … in-memory ring buffer (5–10 min)" | Chaos test: Inject spans continuously at 10k/sec for 15 min (exceeds 10 min window) → assert oldest spans are evicted; assert buffer memory stays bounded within ring buffer max; assert no crash or corruption. Chaos test: Kill ingest channel mid-stream (close broadcast channel) → assert buffer gracefully pauses; assert next ingest reconnects and resumes. |
| **cross-surface-coordination: TauRPC ↔ IPC Channels consistency** | Architecture Standard Contracts: IPC procedures (`traces.query`) return paginated results; Channels (`pulse://stream/spans`) emit realtime Arrow batches | Coordination test: Subscribe to `pulse://stream/spans` channel → send gRPC trace → receive channel event → call `traces.query` TauRPC → assert returned row set includes span from channel event (no desync). |
| **cross-surface-coordination: Snapshot generation + Notification emit** | Creator Brief: "Notification — `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`" | Coordination test: Invoke `snapshot.generate` TauRPC → await `pulse://stream/snapshot-progress` channel events (progress updates) → final event is completion → assert notification is emitted (captured via platform notification spy or IPC channel `pulse://stream/events`); assert notification text includes token count. |
| **multi-platform-compat: Windows WebView2 vs macOS WKWebView vs Linux GTK WebKit** | Architecture Stack: "WebView2 (Windows) and WKWebView (macOS/Linux)"; Design System Surfaces: "desktop-webview (React 19 + Tailwind CSS v4 webview) — Windows/macOS/Linux WebView2/WKWebView" | Platform compat test (matrix over Windows/macOS/Linux CI runners): Verify `tauri-driver` harness can boot app on each platform; verify IPC works consistently; verify WebGPU canvas is available (supported on Windows 11+, macOS 13+, Linux with GPU driver); verify tray icon renders (platform-specific implementation); verify notifications dispatch via native Notification Center / Action Center / freedesktop. Negative test: Older Windows 10 or unsupported Linux (no GPU) → graceful degradation or clear error message. |
| **load-test: OTLP receiver saturation (HTTP vs gRPC trade-off)** | Architecture: Two separate receiver ports (`:4317` gRPC, `:4318` HTTP) for protocol choice | Load test: Drive both receivers simultaneously; gRPC path with 100 concurrent clients each sending 1 span/sec (100 span/sec aggregate); HTTP path with 50 clients (50 span/sec); assert no cross-protocol interference; assert buffer ingests all (150 total/sec). Measure latency p50/p99 for span export RPC (must stay < 100ms for interactive UI). |
| **contract-test-against-OTLP-sandbox: OpenTelemetry protobuf evolution** | Standard Contracts: "TraceService, MetricsService, LogsService on `:4317` per opentelemetry-proto"; "POST /v1/traces, /v1/metrics, /v1/logs on `:4318` per OpenTelemetry spec" | Contract test: Use OpenTelemetry reference SDK (e.g., `opentelemetry-rust` crate) to generate valid spans/metrics/logs and send to app receivers; assert app accepts and buffers all (roundtrip test: generate → send → query back → assert match). Test with multiple protobuf versions (v1.0, v1.1, v1.2 if applicable) to ensure forward/backward compat. |
| **agent-driven-discipline: No manual verification checkpoints** | Development Style: "agent-driven. … deterministic harness invocations, machine-parseable outputs, schema-stable contracts" | Discipline trigger: Every test must exit with deterministic signal (exit code 0/non-zero, structured stdout/JSON, log line match) readable by CI agent without human review. No "visual inspection of canvas", no "did you see the blue glow?", no "try it manually first". WebGPU canvas rendering is out of E2E agent scope (pixel inspection not required); assert IPC contract and WebGPU compile success. Snapshot generation verified via token count and marker presence, not manual reading. Widget visibility toggle verified via IPC state, not screenshot comparison. |
| **agent-driven-discipline: Self-bootstrapping test data (no pre-baked DB)** | Test Harness Specification: "Test data strategy: self-bootstrapping fixture mechanism — seed via migrations + factories / fixture files / property-based generators; no developer-seeded DB allowed" | Discipline trigger: Every integration/E2E test must generate fixture data at runtime via OTLP ingest or Rust builders; no `.sql` scripts or pre-baked SQLite snapshots. Rationale: ensures repeatability, test isolation, and verifies ingest path actually works. Fixture data lifecycle is part of test harness, not manual pre-staging. |

---

## 6. Test Tier

**Tier: Standard (1)**

**Justification:**

Andromeda Pulse is a **cross-platform desktop application** (Windows/macOS/Linux via Tauri 2) with **two primary surfaces** (desktop-webview: React + WebGPU dashboard; desktop-native: tray icon + menu) and **persistent in-memory data** (DuckDB ring buffer holding 5–10 min of OTLP telemetry). The **coverage scope spans 19 entities** (8 library crates + ingest/buffer/viz layers + OTLP anti-pattern enforcement + Tauri capability gating + plugin sandbox + MCP sidecar), with **7 critical user-facing flows** (ingest + visualize, generate curated snapshots, query via MCP, plugin lifecycle, widget ↔ dashboard toggle, real-time push, workspace detection). **Security tier is Minimal** (loopback-only binding, no persistent user accounts, no encrypted data at rest), but the architecture **explicitly rejects 28+ attack vectors** and enforces **high-quality rejection logic** (path canonicalization, SQL parameter binding, post-protobuf invariant checks, capability-scoped WASM sandbox, Minisign signature verification). **Creator's risk tolerance** signals "portfolio-worthy GPU-accelerated visualization" and "approach Jaeger UI quality", implying **above-MVP quality bar**. The **agent-driven development style** (per Setup gate) requires **every test layer to be machine-parseable and runnable end-to-end** with no human in the loop, which elevates rigor beyond minimal CLI testing.

Standard tier covers:
- Unit tests (per crate)
- Integration tests (multi-module OTLP ingest → buffer → query pipeline)
- E2E tests (all 7 critical paths)
- Cross-surface coordination (Channels ↔ TauRPC sync; snapshot generation + notifications)
- Security vector negative tests (28+ anti-patterns)
- Multi-platform compatibility matrix (Windows/macOS/Linux WebView + tray + notifications)
- Agent-driven harness discipline (deterministic signals, self-bootstrapping fixtures, no human verification)

**Does NOT require Comprehensive tier** (no regulated compliance data, no multi-tenant isolation, no chaos/property tests, no exotic multi-version compat matrix). Performance budgets exist (10k spans/sec throughput, token budget enforcement) but are validated via targeted load/perf tests, not full chaos suite. Minimal security tier avoids formal threat modeling, crypto audit, or hardened isolation testing.
