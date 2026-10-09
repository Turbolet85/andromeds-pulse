## 1. Architecture Excerpt

### Stack (instrumentation surfaces)

- **Rust 2024 edition (rustc 1.84+)** — Single language across receivers, buffer, viz host, IPC, plugin host, and MCP server; OTel-receiver trend matches high-performance receiver benchmarks.
- **`tonic` 0.14.x** — gRPC server for OTLP/gRPC receiver on `:4317` with `prost` codegen against `opentelemetry-proto`; requires OTel SDK integration for trace context propagation.
- **`axum` 0.8.x on `hyper` 1.x + `tower`** — HTTP server for OTLP/HTTP receiver on `:4318` (protobuf and JSON) sharing the Tokio runtime; requires manual instrumentation and trace context propagation.
- **`tokio` (current stable)** — Multi-threaded async runtime serving both OTLP ports and IPC; structured concurrency surfaces for instrumentation.
- **`tokio::sync::mpsc` + `tokio::sync::broadcast`** — In-process channels for ingest→appender hand-off (backpressure) and buffer→subscribers fan-out; emit metrics on channel capacity and subscriber counts.
- **DuckDB 1.5.x via `duckdb` crate 1.10500.x** — Embedded columnar OLAP storage with in-memory ring buffer (5–10 min, configurable); requires query instrumentation for aggregation latency.
- **Apache Arrow** — Zero-copy columnar interchange between OTLP decode, DuckDB, and viz/MCP; OTel-native for IPC serialization.
- **`wasmtime` 25+ with WASM Component Model + WIT** — Plugin runtime with capability-scoped sandboxing; emit metrics on loaded plugin count and plugin invocation latency.
- **Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL)** — GPU-accelerated charts inside WebView2 (Windows) and WKWebView (macOS/Linux); requires frontend telemetry and render metrics.
- **TauRPC (`taurpc` crate)** — Router-style Rust ↔ webview IPC bridge with auto-generated TypeScript bindings; structured-log scope for each TauRPC call.
- **`rmcp` (official Rust SDK) over stdio** — MCP server surface with `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` as tool methods (feature-gated).
- **`tauri-plugin-notification` 2.x** — Native OS notifications (Notification Center / Action Center / freedesktop); emit events on notification dismiss/interaction.
- **`tauri-plugin-updater` 2.x with `latest.json`** — In-app updater from GitHub Releases; emit structured logs on update check/download/install transitions.
- **`thiserror` 2.x + `anyhow` 1.x** — Error handling with module-internal enums and boundary catch-all; structured error tags for categorization in logs.

### Workspace / Modules

- **`ingest`** — OTLP receivers (tonic gRPC + axum HTTP) on ports `:4317` and `:4318`; ingest span boundary.
- **`buffer`** — DuckDB ring buffer and Arrow appender; storage and query boundary.
- **`viz`** — Query layer feeding webview WebGPU charts; visualization query boundary.
- **`ui-bridge`** — TauRPC routers for all command surfaces; IPC boundary.
- **`snapshot`** — Curated markdown generator for LLM context; curation boundary.
- **`workspace-detector`** — Detect host project context; discovery boundary.
- **`plugins`** — Wasmtime Component Model host for WASM plugin loading; plugin invocation boundary.
- **`mcp-server`** — rmcp stdio sidecar (feature-gated); MCP protocol boundary.

### Standard Contracts

- **`app_info` (Tauri IPC command)** — Returns application identity envelope (name, version, rust_version, tauri_version, features, build_profile); span boundary for lifecycle queries.
- **`health` (Tauri IPC command)** — Returns liveness state of subsystems (otlp_grpc_receiver, otlp_http_receiver, buffer, ingest_channel); structured-log status point.
- **`ready` (Tauri IPC command)** — Returns readiness state with checks (duckdb_connection, ingest_mpsc_capacity_pct, broadcast_subscribers, plugins_loaded, mcp_server_enabled); instrumentation readiness signal.
- **`POST /v1/traces` (OTLP HTTP endpoint)** — Receives trace spans (protobuf or JSON); OTLP spec-fixed, trace context must propagate via W3C traceparent in request headers.
- **`POST /v1/metrics` (OTLP HTTP endpoint)** — Receives metrics (protobuf or JSON); metric context propagation via OTLP spec conventions.
- **`POST /v1/logs` (OTLP HTTP endpoint)** — Receives logs (protobuf or JSON); log context propagation via OTLP spec conventions.
- **gRPC TraceService / MetricsService / LogsService (`:4317`)** — Spec-conformant gRPC services from `opentelemetry-proto`; gRPC metadata carries trace context (grpc-trace-bin header).
- **`tools/list` and `tools/call` (MCP server)** — JSON-RPC 2.0 over stdio; tool methods are `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`.
- **Real-time push streams** — Tauri 2 IPC `Channel` API with binary Arrow IPC payloads on event names: `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`; no JSON stringify, Arrow binary codec.

### Surfaces

**Product type** — cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles.

### Observability Hints

#### From Cross-cutting Patterns

**Logging library**: `opentelemetry-stdout` (or file exporter targeting `~/.andromeda-pulse/logs/`) is the only exporter the product itself uses for its own telemetry; the OTLP network exporter is never pointed at the product's own `:4317`/`:4318` ports. Any future "outbound observer" variable must be explicitly distinct from the inbound receiver port variables.

**Configuration management**: layered with explicit precedence (highest wins) — process env vars (`ANDROMEDA_PULSE_*`) > user `~/.andromeda-pulse/config.toml` > built-in defaults. No external secrets manager.

#### From Inherited Defaults

- **OTel SDK**: not explicitly named; receivers are `tonic` 0.14.x + `axum` 0.8.x; clients use external OpenTelemetry SDKs.
- **Error reporting platform**: none named; errors use `thiserror` 2.x (modules) + `anyhow` 1.x (boundaries) + `serde`-friendly `AppError` enum at IPC bridge.
- **Self-observation discipline**: `opentelemetry-stdout` or file exporter; never dial own OTLP ports.

#### From Observability Design Section

(No dedicated Observability Design section in arch — Phase 3 will derive defaults from Cross-cutting Patterns + Inherited Defaults + stack research.)

### Project Intent Summary

- **Core functionality:** "Local-first, zero-infrastructure" desktop application where every byte of telemetry stays on the developer's machine; single-process modular monolith with eight library crates sharing memory via tokio channels.
- **Target users:** Developer-tool surface targeting developers staring at telemetry for hours; dense, chart-first, low-chrome dashboard with dark-mode default and WebGPU canvas.
- **Critical paths hint:** No flows enumerated in arch — derive from input.md or tests' critical paths in Phase 1.

### CI/CD Platform

- **Platform:** GitHub Actions
- **Pipeline note:** `ci.yml` runs on every PR/push (fmt + clippy + xtask test + release build); `release.yml` runs on tag push, builds/signs/notarizes bundles and uploads to GitHub Releases; `update-channels.yml` updates Homebrew tap and Scoop manifest.

### Obs-Relevant Conventions

- **Log file location pattern:** `~/.andromeda-pulse/logs/` (stdout-exporter destination for self-telemetry; per-platform resolution via `ANDROMEDA_PULSE_DATA_DIR` override).
- **Log level conventions:** `ANDROMEDA_PULSE_LOG_LEVEL` env var (`trace|debug|info|warn|error`); fallback to `RUST_LOG`.
- **Service identity convention:** Tauri app bundle identifier `com.andromeda.pulse`; binary name `andromeda-pulse` (Linux/macOS), `andromeda-pulse.exe` (Windows); binary crate name `pulse-app`; rmcp sidecar `andromeda-pulse-mcp` (when feature-enabled).

---

## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal (0)
- **Justification:** "This is a local-first, zero-infrastructure single-user desktop app with no user accounts, no persistent user data store (in-memory DuckDB ring buffer with 5–10 min retention), no internet-exposed network surface (OTLP receivers bound to `127.0.0.1` only), and no compliance-regulated data classifications."

### Logging-Sensitive Vectors

- **Vector 1: public API (OTLP/gRPC `:4317` + OTLP/HTTP `:4318`)** — logging implication: raw OTLP attribute values, span/log/metric content payloads must NEVER appear in logs; they may contain incidentally captured secrets, IDs, URLs, error messages, and SQL fragments from the instrumented host application
- **Vector 2: IPC (Tauri TauRPC bridge)** — logging implication: `AppError` enum variants crossing the bridge must be sanitized; never expose stack traces, Rust struct names, file paths, or library versions in content surfaced to webview
- **Vector 3: plugin host (WASM Component Model)** — logging implication: plugin load failures must be logged for operational debugging, but never log full canonicalized plugin file paths (basename only is acceptable)
- **Vector 4: MCP stdio surface** — logging implication: MCP tool response bodies must NEVER be logged; they surface OTLP attribute values to external LLM clients and are the indirect-prompt-injection surface
- **Vector 5: filesystem reads (config + workspace detection)** — logging implication: DuckDB query parameters and workspace-detector output must NEVER be logged as-is; log query identifier + parameter count instead
- **Vector 6: CLI input (env vars)** — logging implication: path env vars are subject to CWE-22 (Path Traversal) risks; canonicalize and validate paths during load, log basename only in errors

### Anti-Patterns Rejected

- **Raw OTLP attribute logging** — rejected because: OTLP values are user-controlled telemetry containing incidentally captured secrets, IDs, URLs, error messages from instrumented applications
- **DuckDB string interpolation (`format!("SELECT … WHERE service_name = '{}'")`)** — rejected because: Dagster GHSA-mjw2-v2hm-wj34, Vanna CVE-2024-5827, dagster-duckdb CVE-2026-41490 document this as the dominant 2026 DuckDB vulnerability class; always use `Connection::prepare` with `?` placeholders
- **Untrusted plugin-returned Arrow IPC without size cap** — rejected because: plugin-sourced Arrow IPC must be size-bounded before re-emitting on Tauri `Channel` API
- **Snapshot file contents logged or unguarded** — rejected because: snapshots persist to disk containing OTLP-derived secrets; must surface user-facing warning and emit a non-suppressible clipboard-write toast event
- **Token exposure in updater URLs** — rejected because: tauri-plugin-updater Minisign Ed25519 verification is mandatory and cannot be disabled; never log download URL query strings (may contain auth tokens in future scopes)
- **MCP double-gate omission** — rejected because: MCP stdio surface must require both `--features mcp-server` compile-time gate AND `ANDROMEDA_PULSE_MCP_ENABLED=true` runtime gate; graceful-degrade `warn` when env-var set without feature flag MUST be preserved
- **Suboptimal TauRPC bridge error handling** — rejected because: never serialize `anyhow::Error` directly; must convert to `serde`-friendly `AppError` enum to avoid leaking full error chain to webview
- **Uncanonicalized path env vars** — rejected because: CWE-22 (Path Traversal) class attack via `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars; must canonicalize and confine to resolved per-platform data dir or explicit override base

### Data Classifications

- **telemetry payloads (traces, metrics, logs)** (Sensitivity: High) — obs handling: scrub-required; appears in: OTLP receivers (ingest crate `:4317`/`:4318`), DuckDB in-memory ring buffer (buffer crate), broadcast fan-out to viz/MCP, curated snapshot files (`~/.andromeda-pulse/snapshots/`); Note: can contain incidentally captured secrets, IDs, URLs, error messages, SQL fragments from instrumented host application
- **config (user settings)** (Sensitivity: Low) — obs handling: OK to log structured; appears in: `~/.andromeda-pulse/config.toml`, env vars `ANDROMEDA_PULSE_*` (enums, port ranges, file paths, retention seconds, log level, MCP enabled flag, plugin dir)
- **config (signing/release credentials)** (Sensitivity: Critical) — obs handling: never-log; appears in: GitHub Actions encrypted secrets, Azure Key Vault, Apple Developer ID; Note: out of runtime scope; binary ships public key only
- **third-party WASM plugin binaries** (Sensitivity: High) — obs handling: scrub-required; appears in: `~/.andromeda-pulse/plugins/` loaded by wasmtime Component Model host; Note: signed-plugin verification deferred post-v1; basenames only in logs

---

## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (Windows/macOS/Linux WebView2 + WKWebView with React 19 + Tailwind v4) — Service constellation dashboard and compact widget in browser environment; instrumentation via browser OTel SDK + web-vitals for TTI/LCP metrics on Halo State Pulse canvas updates and panel transitions.

- **desktop-native** (Windows/macOS/Linux tray via NotifyIcon/NSStatusItem/AppIndicator) — Always-visible tray icon with unified Halo State Pulse glow encoding service health; native telemetry via OS notification system (no frontend SDK — emit state changes via app logs or custom telemetry endpoint).

### Loading / Error / Empty State Patterns

- **Skeleton pulse** — visibility: background overlay on card/panel (opacity 50–100% discrete pulse at 1.2s cycle) — telemetry hook: observe skeleton duration as span within component-render lifecycle; metric: time from mount to content visibility (LCP proxy).

- **Empty state** — visibility: centered text + optional icon within container (color #7D8697 Tertiary) — telemetry hook: counter for empty-state occurrence per view (Traces / Metrics / Logs / Snapshots); error attribute if triggered by data-fetch failure.

- **Error state** — visibility: inline text below input field or modal body (color #8B2E3B Alert Burgundy, 1px border rgba(139, 46, 59, 0.5)) — telemetry hook: capture error message and error class; span attribute for user-facing error surface; duration of error state visibility.

### User-Facing Error Surfaces

- **Input field error** (location: inline below input) — appears for: form validation failure, API response error on field submission; recovery affordance: retry (form re-submit) or contact (support link if provided in error text) — feedback widget candidate: no (field-level scope too narrow for Sentry widget).

- **Modal error state** (location: modal body with Alert Burgundy border and text) — appears for: operation failure (e.g., snapshot generation failure, trace query failure); recovery affordance: retry button or dismiss + fallback action — feedback widget candidate: yes (modal contains sufficient context and user attention for feedback collection).

- **Notification (OS-native)** (location: system notification center) — appears for: snapshot generation completion status (success or failure), MCP server status change, update available; recovery affordance: action button ("View" snapshot, "Copy to Clipboard", or implicit dismiss) — feedback widget candidate: no (native notification scope outside web instrumentation; design defers to OS feedback mechanisms).

---

## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** compact-widget, traces-dashboard, metrics-dashboard, logs-dashboard, snapshots-dashboard, settings-modal, investigation-modal, custom-titlebar, trace-data-table, footer
- **desktop-native:** tray-icon, tray-menu, notifications, file-picker

### Error Boundary Placement

(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms.)

---

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

---

## 6. Creator Brief Excerpt

### Must-Work Scenarios

- "Cross-platform desktop app that receives OTLP telemetry (HTTP `:4318` and gRPC `:4317`) from any local application, visualizes traces, metrics, and logs in a polished GPU-accelerated UI"
- "Runs as a compact always-visible glance-monitor (quarter-screen widget mode) plus a full expanded dashboard"
- "Has an 'Investigate' button that captures a **token-efficient curated snapshot** (not raw telemetry dump) of recent activity — copies an AI-ready prompt to clipboard for one-paste debug sessions with Claude Code / Cursor / ChatGPT"
- "Optional MCP server lets AI agents query telemetry directly without manual snapshots"
- "Snapshot generation pipeline: (1) Time window selection — last N minutes (default 5 min; configurable 30s..30min) OR user-selected range from heatmap; (2) Filter — by service / trace ID / error-only / latency-outlier; (3) Curate: dedupe identical spans, highlight anomalies, extract critical path, aggregate metrics (p50/p95/p99/max), drop verbose / low-signal attributes; (4) Format hierarchical markdown with citation anchors; (5) Token budget — target ≤10k / ≤25k / ≤50k tokens preset"
- "Live trace view — span tree with timing, per-service filter, GPU-rendered timeline"
- "Metrics view — time-series charts per metric name (WebGPU compute for aggregation; WGSL render for charts)"
- "Log stream with span correlation"
- "Real-time push: Service health constellation (animated dots per service; color + pulse rate indicate health; throughput visualized as ring intensity); Recent traces (top-N latest spans, color-coded by status / latency); Latency heatmap strip; Error rate sparkline + count; Throughput counter (events/sec with smooth animation)"
- "Plugin sandbox — WASM Component Model boundaries; capability-based access (plugins declare needed APIs via WIT interfaces; runtime grants per-plugin)"
- "MCP `tools/call` requests: query_traces(time_range, service_filter, limit), query_metrics(metric_name, time_range, aggregation), query_logs(filter, time_range, limit), generate_snapshot(time_range, token_budget)"
- "Notification — `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`"

### Rigor Hints

- "**BAR.** Beat otel-desktop-viewer on UI polish; approach Jaeger UI quality; compete with Uptrace / SigNoz on lightweight-ness; zero-config local install; **portfolio-worthy GPU-accelerated visualization**; AI debug workflow that saves real token cost per investigation."
- "**Beautiful infographics requirement** — visual bar matches portfolio showcase tools (think Apple Activity rings, Cleanshot X, Things 3 polish). UI is part of the product, not just a tool."
- "Glance-readable from 2 meters — typography legible at distance; high-contrast palette; motion convey state (pulse / flow / steady) without requiring focused attention."
- "**Development Style:** agent-driven (built via Andromeda v2 pipeline — recursive dogfood validates our OTel mandate on its own creator tool)."
- "**Scale Intent:** startup (shipped public OSS used by external devs, not enterprise-production APM scale)."
- "Public OSS — developers using AI coding assistants who want instant local observability + token-efficient AI debug workflow."
- "Smooth animation; no jank at high cardinality."
- "Zero external runtime required (DuckDB + WebGPU implementation bundled)."
- "10k+ spans/sec" target (WebGPU compute shaders for aggregation; SIMD vectorization for OTLP protobuf parsing)

### Obs Anti-Patterns (creator's explicit asks)

- "**andromeda-pulse IS observability infrastructure** — it RECEIVES OTLP telemetry from other apps and IS the local observer itself."
- "`OTel SDK:` field — still names OTel SDK packages (self-instrumentation is mandatory even for observers) BUT the exporter package is **stdout / console / file**, NOT the OTLP network exporter. For Rust: `opentelemetry_sdk` + `opentelemetry-stdout` crate (not `opentelemetry-otlp`). **Exporting OTLP to itself would be an infinite recursion loop.**"
- "`Observer endpoint:` field — still mentions `ANDROMEDA_OBSERVER_URL` for downstream tooling consistency BUT describes its role as 'not dialed — this project IS the observer; own operational telemetry exports to stdout via console exporter'. Make the deviation explicit."
- "If Phase 3b sub-agent emits a standard OTLP network exporter pointed at `ANDROMEDA_OBSERVER_URL` WITHOUT applying the self-observation adaptation, that is a bug — the sub-agent missed step 5d."
- "opentelemetry-stdout — Self-observation exporter (recursive dogfood — see edge case below)" — listed as the only exporter the product itself uses.
- Token-efficient snapshot guarantee: "Existing snapshot tools dump raw OTLP JSON — a production trace at moderate load = hundreds of thousands of tokens to paste into Claude. Pulse v2 generates **curated** snapshots" — implies obs telemetry feeding the snapshot pipeline must be queryable / aggregatable per the dedupe-anomaly-extraction-aggregation steps without raw OTLP attribute dumps in the snapshot output.
