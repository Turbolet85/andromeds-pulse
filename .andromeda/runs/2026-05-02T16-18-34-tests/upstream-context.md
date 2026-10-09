## 1. Architecture Excerpt

### Stack (testable surfaces)

- **Rust 2024 edition (rustc 1.84+)** — primary language and runtime for receivers, buffer, viz host, IPC, plugin host, MCP server
- **Tauri 2.x** — native window, webview, OS bundlers, updater, tray, notifications for the desktop shell
- **tonic 0.14.x** — gRPC server for OTLP/gRPC receiver on `:4317` with protobuf codegen
- **axum 0.8.x on hyper 1.x + tower** — HTTP server for OTLP/HTTP receiver on `:4318` supporting protobuf and JSON
- **tokio (current stable)** — async multi-threaded runtime serving OTLP ports and IPC
- **DuckDB 1.5.x via duckdb crate 1.10500.x** — embedded columnar OLAP storage with in-memory ring buffer (5–10 min)
- **Apache Arrow** — zero-copy columnar interchange between OTLP decode, DuckDB, and viz/MCP surfaces
- **wasmtime 25+ with WASM Component Model + WIT** — plugin runtime with capability-scoped third-party extensions
- **Webview WebGPU** — GPU-accelerated charts inside WebView2 (Windows) and WKWebView (macOS/Linux) using WGSL shaders
- **TauRPC (taurpc crate)** — Rust ↔ webview IPC bridge with auto-generated TypeScript bindings
- **rmcp (official Rust SDK) over stdio** — MCP server surface for query_traces / query_metrics / query_logs / generate_snapshot as #[tool] methods
- **tauri-plugin-notification 2.x** — native OS notifications (Notification Center / Action Center / freedesktop) for snapshot completion and updater state
- **tauri-plugin-updater 2.x with latest.json** — in-app updates from GitHub Releases

### Workspace / Modules

- **ingest** — OTLP receivers (tonic + axum) for gRPC and HTTP ingest on `:4317` and `:4318`
- **buffer** — DuckDB ring buffer and Arrow appender for columnar storage and zero-copy hand-off
- **viz** — query layer feeding webview WebGPU charts with aggregation by service/time-bucket
- **ui-bridge** — TauRPC routers for Tauri IPC command surface
- **snapshot** — curated markdown generator for LLM-context snapshots (balanced 25k tokens default)
- **workspace-detector** — detection of host project context for telemetry correlation
- **plugins** — wasmtime Component Model host for WASM plugin loading and invocation
- **mcp-server** — rmcp stdio sidecar for external LLM agent access (feature-gated, `--features mcp-server`)
- **pulse-app** — Tauri binary crate that wires the eight library crates and webview
- **xtask** — cargo-xtask task runner for release, signing, notarization, and changelog tasks

### Standard Contracts

- **app_info command (Tauri IPC introspection)** — returns application identity envelope with name, version, rust_version, tauri_version, features, build_profile
- **health command (Tauri IPC liveness)** — returns status ("ok" or "degraded") and subsystem states (otlp_grpc_receiver, otlp_http_receiver, buffer, ingest_channel)
- **ready command (Tauri IPC readiness)** — returns ready boolean and check states (duckdb_connection, ingest_mpsc_capacity_pct, broadcast_subscribers, plugins_loaded, mcp_server_enabled)
- **OTLP receiver (HTTP endpoint)** — `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` on `:4318` accepting protobuf and JSON per OpenTelemetry spec
- **OTLP receiver (gRPC service)** — TraceService, MetricsService, LogsService on `:4317` per opentelemetry-proto
- **query_traces (MCP tool)** — queries spans from the buffer via JSON-RPC 2.0
- **query_metrics (MCP tool)** — queries metric points from the buffer via JSON-RPC 2.0
- **query_logs (MCP tool)** — queries log records from the buffer via JSON-RPC 2.0
- **generate_snapshot (MCP tool)** — generates curated markdown snapshot for external LLM context via JSON-RPC 2.0
- **traces.query (Tauri IPC procedure)** — queries spans with aggregation support
- **metrics.query (Tauri IPC procedure)** — queries metric points with aggregation support
- **logs.query (Tauri IPC procedure)** — queries log records with aggregation support
- **snapshot.generate (Tauri IPC procedure)** — generates balanced snapshot (25k tokens, dedupe + critical-path + p50/p95/p99 + anomaly highlighting)
- **snapshot.list_recent (Tauri IPC procedure)** — lists recently generated snapshots
- **snapshot.copy_to_clipboard (Tauri IPC procedure)** — copies snapshot markdown to clipboard
- **plugins.list (Tauri IPC procedure)** — lists loaded WASM Component Model plugins
- **plugins.reload (Tauri IPC procedure)** — reloads plugins from `~/.andromeda-pulse/plugins/`
- **plugins.invoke (Tauri IPC procedure)** — invokes a plugin capability-scoped function
- **workspace.detect (Tauri IPC procedure)** — detects host project context
- **workspace.list (Tauri IPC procedure)** — lists detected workspace metadata
- **Real-time push contract (Tauri IPC channels)** — live span/metric/log streams using Tauri 2 IPC `Channel` API with binary Arrow IPC payloads; event names are `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`

### Surfaces

**Product type** — cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles. Not a web app, not a CLI, not a microservice fleet.

### Test Harness Commitment

- **Development Style:** agent-driven. Downstream specialist plans (tests, obs, setup-project) should branch their research toward agent-driven development workflows (deterministic harness invocations, machine-parseable outputs, schema-stable contracts, `cargo xtask` task surfaces).

### Project Intent Summary

- **Core functionality:** Local-first, zero-infrastructure telemetry platform where every byte stays on the developer's machine; OTLP ingest, DuckDB buffer, token-efficient LLM snapshot curation, and plugin extensibility.
- **Target users:** developers instrumenting software with OpenTelemetry; growth model is modular monolith with WASM Component Model plugin extension layer for community/third-party functionality.
- **Critical paths hint:** no flows enumerated in arch — derive from input.md or design in Phase 1.

### CI/CD Platform

- **Platform:** GitHub Actions
- **Pipeline note:** matrix over Linux/macOS/Windows; `tauri-action` builds `.msi` / `.dmg` / `.AppImage` / `.deb` distribution artifacts per OS; signs Windows via Azure Key Vault EV; notarizes macOS via Apple Developer ID; uploads bundles + `latest.json` to GitHub Releases; `update-channels.yml` drives Homebrew tap and Scoop manifest updates on release completion.

### Test-Relevant Conventions

- **Test file location:** co-located with source under each crate's `src/` (no separate `tests/` directory specified); tests run via `cargo xtask test` and `cargo test --workspace`.
- **Test naming pattern:** standard Rust test naming conventions (functions marked with `#[test]` or integration tests in `tests/` subdirectory per crate) — no project-specific pattern override in arch.
- **Test telemetry injection:** end-to-end tests inject synthetic spans/metrics/logs by speaking OTLP to the running receiver on `:4317` (gRPC) or `:4318` (HTTP) — the same surface external SDKs use; no in-process test-mode bypass. Tests resolve non-default ports via `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT` environment variables. Tests inspect buffer state through TauRPC `traces.*` / `metrics.*` / `logs.*` query routers.
## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal (0)
- **Justification:** This is a local-first, zero-infrastructure single-user desktop app with no user accounts, no persistent user data store, no internet-exposed network surface (OTLP receivers bound to 127.0.0.1 only), and no compliance-regulated data classifications.

### Attack Vectors

- **Vector 1:** public API (network-bound on loopback only) — scope: OTLP/gRPC on `:4317` and OTLP/HTTP on `:4318`, both bound `127.0.0.1` only. Mitigation: Loopback-only binding is the de facto authorization boundary; plaintext loopback OTLP servers run without TLS per Minimal-tier risk model.

- **Vector 2:** IPC (Tauri TauRPC bridge — webview ↔ Rust) — scope: procedures `app_info`, `health`, `ready`, `get_settings`, `update_settings`, routers `traces.*`, `metrics.*`, `logs.*`, `snapshot.*`, `plugins.*`, `mcp.*` (feature-gated), `workspace.*`. Mitigation: Tauri 2 capability gating `pulse:default` permits exactly the enumerated procedures; errors are forced to `Serialize`-able `AppError` enum variants.

- **Vector 3:** plugin host (WASM Component Model) — scope: `~/.andromeda-pulse/plugins/` filesystem load by `wasmtime` 25+ and `plugins.invoke` IPC dispatch. Mitigation: Capability-scoped sandboxing — guests receive only host imports declared in their WIT; no syscalls, file, or socket access unless explicitly granted.

- **Vector 4:** MCP stdio surface (feature-gated) — scope: `andromeda-pulse-mcp` rmcp sidecar, JSON-RPC 2.0 over stdin/stdout with tools `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`. Mitigation: Stdio caller is an LLM client with no network exposure; gated by compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`.

- **Vector 5:** filesystem reads (config + workspace detection) — scope: `config.toml` under per-platform data dir; env-var-overridable paths (`ANDROMEDA_PULSE_CONFIG_PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_PLUGIN_DIR`); workspace-detector crate. Mitigation: Canonicalize and confine all `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars using `std::path::Path::canonicalize()` + assertion that canonical path starts under resolved data dir.

- **Vector 6:** in-app updater — scope: `tauri-plugin-updater` 2.x consuming `latest.json` from GitHub Releases. Mitigation: Updater public key baked into Tauri config; Minisign Ed25519 signature verification is mandatory and cannot be disabled.

- **Vector 7:** CLI input (env vars + binary launch) — scope: Reserved env vars `ANDROMEDA_PULSE_*`, `RUST_LOG`. Mitigation: `serde` + `TryFrom<u16>` for ports; path env vars must undergo canonicalization and confinement per Vector 5 mitigation.

- **Vector 8:** webview content (host-side) — scope: WebView2 (Windows) / WKWebView (macOS/Linux) rendering WebGPU canvas from Arrow data under `pulse-app/ui/`. Mitigation: Tauri capability model `pulse:default` gates IPC procedures; no `fs`, `shell`, `dialog`, `http` core APIs granted to webview.

### Anti-Patterns Rejected

- **NEVER trust `Content-Length` or trailers as substitute for `DefaultBodyLimit` on OTLP HTTP `:4318`** — rejected because: JFrog axum-core advisory documents unbounded request DoS root cause.

- **NEVER format user input directly into SQL (e.g. `format!("SELECT … WHERE service_name = '{}'", user_input)`)** — rejected because: Dagster GHSA-mjw2-v2hm-wj34, Vanna CVE-2024-5827, dagster-duckdb CVE-2026-41490 cluster proves SQL injection is the dominant 2026 DuckDB vulnerability class; ALWAYS use `Connection::prepare` with `?` placeholders and parameter binding.

- **NEVER skip post-`prost`-decode invariant checks on OTLP payloads** — rejected because: `prost` only validates wire format; `span_id` (8 bytes), `trace_id` (16 bytes), attribute key sizes, etc. must be checked before passing to `buffer::Appender`.

- **NEVER read path env vars without `Path::canonicalize()` + confinement assertion** — rejected because: CWE-22 (Path Traversal); zip crate CVE-2025-29787, RustFS CVE-2025-68705 document symlink-chain / TOCTOU traversal class affecting user-overridable path env vars.

- **NEVER call `tonic::transport::Server::builder()` without `.max_decoding_message_size(8 * 1024 * 1024)`** — rejected because: `tonic` historically accepted 1 GB+ unary requests by default.

- **NEVER trust plugin-returned Arrow IPC bytes without a size cap** — rejected because: Risk of unbounded memory consumption on re-emit to Tauri `Channel` API.

- **NEVER use non-Cranelift `wasmtime` feature flag on x86_64** — rejected because: April 2026 advisory cluster documents two Critical sandbox escapes (CVE-2026-34941, CVE-2026-35195) in non-Cranelift backends; Cranelift is the unaffected configuration.

- **NEVER override `tauri-plugin-updater` Minisign signature verification** — rejected because: Minisign Ed25519 verification cannot be disabled; any wrapper must not attempt bypass.

- **NEVER ship Tauri updater Minisign private key in repo or artifact** — rejected because: Only public key belongs in `tauri.conf.json`.

- **NEVER store Windows EV signing cert outside Azure Key Vault HSM** — rejected because: Private key must never leave HSM per DigiCert/GlobalSign supported HSM-RSA CA policy.

- **NEVER enable DuckDB encryption at rest on in-memory ring buffer** — rejected because: CVE-2025-64429 is documented against encryption feature; in-memory `:memory:` connection avoids surface.

- **NEVER use `CorsLayer::permissive()` or `allow_origin(Any)` on OTLP HTTP `:4318`** — rejected because: Default-deny is correct posture.

- **NEVER skip Host-header allowlist middleware on `:4318`** — rejected because: DNS rebinding is documented attack against localhost OTLP/HTTP per Coder Agent API CVE-2025-09-19, CVE-2025-66414; "localhost is not a security boundary."

- **NEVER add TauRPC procedure without matching capability entry in `pulse-app/capabilities/`** — rejected because: Silent runtime rejection becomes hard-to-diagnose UX bug; xtask drift check enforces.

- **NEVER grant Tauri core APIs (`fs`, `shell`, `dialog`, `http`) to `pulse:default` without explicit per-feature capability** — rejected because: Must have stated rationale and only when necessary.

- **NEVER expose `pulse:updater` to webview JavaScript** — rejected because: Updater capability is bound to `tauri-plugin-updater` flow only; webview exposure bypasses gate.

- **NEVER widen `pulse:notification` or `pulse:tray` beyond outbound emit** — rejected because: Granting webview-side input handlers converts outbound surfaces into phishing / IPC-abuse vectors.

- **NEVER widen `pulse:plugin-fs` beyond host's read of resolved plugin dir** — rejected because: Webview exposure converts internal loader into unconstrained filesystem-read; write/execute widening converts to plugin-supply-chain compromise vector.

- **NEVER bind OTLP receivers to `0.0.0.0` or non-`127.0.0.1` interface** — rejected because: Loopback binding is de facto authorization boundary; `0.0.0.0` invalidates entire Minimal-tier risk model.

- **NEVER instrument product's own telemetry with OTLP exporter to product's own `:4317`/`:4318`** — rejected because: Creates infinite loop and bypasses loopback authorization model.

- **NEVER commit `.env`, `*.p12`, `*.pem`, `*.cer`, `*.key` files to git** — rejected because: `.gitignore` enforcement + gitleaks CI verification required.

- **NEVER store Azure Key Vault service principal as long-lived GitHub Actions secret** — rejected because: Use GitHub OIDC federation instead.

- **NEVER reference third-party GitHub Actions by floating tag** — rejected because: Pin by 40-char commit SHA; tj-actions/changed-files CVE-2025-30066 (23k repos) demonstrated retroactive-tag-rewrite attack.

- **NEVER grant `permissions: { contents: write }` at workflow level** — rejected because: Set to `contents: read`, elevate to `write` per-job only on publish per minimum-permission principle.

- **NEVER place Azure Key Vault, Apple Developer ID, or Tauri Minisign private key outside `production-release` GitHub Environment with manual approval** — rejected because: Credential containment critical for code-signing supply chain.

- **NEVER log raw OTLP attribute values, payload contents, snapshots, clipboard, or MCP responses** — rejected because: May contain incidentally captured secrets from instrumented application.

- **NEVER log full plugin file paths** — rejected because: Log basename of canonicalized path only.

- **NEVER log DuckDB query parameter values** — rejected because: Log query identifier + parameter count instead.

- **NEVER expose stack traces, struct names, file paths, or library versions in `AppError::Internal` to webview** — rejected because: Must sanitize at `From<thiserror::Error> for AppError` impl.

- **NEVER expose internal hostnames or IPs in OTLP or MCP error responses** — rejected because: Risk of information disclosure.

- **NEVER use `tokio::process::Command::new()` with user input from OTLP, MCP, or workspace-detector** — rejected because: MCP STDIO command-injection cluster (CVE-2025-49596, CVE-2025-54994, CVE-2025-54136, CVE-2026-22252) demonstrates live 2025–2026 class.

- **NEVER load plugin from uncanicalized path or outside resolved plugin dir** — rejected because: Defends against symlink-traversal via `ANDROMEDA_PULSE_PLUGIN_DIR`; pair with Vector 5 mitigation.

- **NEVER spawn rmcp sidecar without verifying compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`** — rejected because: Single-gate is regression of double-gate architecture per Established Decisions.

- **NEVER serialize `anyhow::Error` directly across TauRPC bridge** — rejected because: Must convert to `serde`-friendly `AppError` enum; direct serialization leaks error chain to webview.

- **NEVER use `serde_json::from_slice::<T>()` on buffer without `DefaultBodyLimit` size bound** — rejected because: Covers OTLP HTTP/JSON path on `:4318`.

- **NEVER use `wasmtime` `Linker` to expose host functions outside WIT contract** — rejected because: Bypasses capability-scoped sandbox per architecture.

- **NEVER let rust-toolchain drift below 1.85.0** — rejected because: Edition 2024 parsing + security-positive defaults (`unsafe_op_in_unsafe_fn`, `unsafe extern`, `static mut` denial, tightened `if let` scopes) require 1.85+.

- **NEVER ship release without resolving `tonic` 0.14 vs `opentelemetry-otlp` 0.31 / `tonic` 0.13 duplicate** — rejected because: `cargo deny check bans` enforces; duplicate versions create linker conflicts.

### Data Classifications

- **user-content (telemetry payloads — traces, metrics, logs)** (Sensitivity: medium) — stored in OTLP receivers (`ingest` crate on `:4317`/`:4318`), DuckDB in-memory ring buffer (`buffer` crate), broadcast to viz/MCP, and curated snapshots at `~/.andromeda-pulse/snapshots/`; testability hint: partially-testable with stub (loopback binding, in-memory buffer are deterministic; snapshot file I/O requires filesystem stub).

- **config (user settings)** (Sensitivity: low) — stored in `~/.andromeda-pulse/config.toml` and env vars `ANDROMEDA_PULSE_*`; testability hint: testable (enum settings, port ranges, file paths, retention seconds, log level are directly exercisable).

- **config (signing/release credentials — out of app runtime)** (Sensitivity: high) — stored in GitHub Actions encrypted secrets, Azure Key Vault HSM, and Apple Developer ID; testability hint: untestable in current harness (build/release pipeline only; runtime artifact contains only public verification key).

- **user-content (third-party WASM plugin binaries)** (Sensitivity: medium) — stored in `~/.andromeda-pulse/plugins/` loaded by `wasmtime` Component Model host; testability hint: partially-testable with stub (plugin lifecycle testable; arbitrary guest behavior requires fixture mocking or sandbox instrumentation).
## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (React 19 + Tailwind CSS v4 webview) — Windows/macOS/Linux WebView2/WKWebView hosting a React dashboard with compact widget and full dashboard expansion, using WebGPU canvas for real-time telemetry visualization.
- **desktop-native** (Tauri 2 native integration) — Windows/macOS/Linux tray icon and menu providing always-visible access to snapshot generation, MCP server toggle, and settings without requiring the webview window.

### Layout Categories

- **Compact widget** — used in: desktop-webview (primary surface, quarter-screen size with titlebar and canvas).
- **Full dashboard** — used in: desktop-webview (expanded window with sidebar/tab navigation to Traces / Metrics / Logs / Snapshots / Settings views).
- **Tray icon and menu** — used in: desktop-native (always-visible status icon with context menu for actions and app control).

### Brand Identity Anchors

- **Earth Blue (#4A90E2)** (primary color, 30% opacity for default borders, 60% opacity for active states, solid for focus rings) — drives selector for: active/focused states, healthy service status, navigation focus, control emphasis.
- **Alert Burgundy (#8B2E3B)** (accent color for error states) — drives selector for: error/alert states, anomaly indicators, error text and borders.
- **Status White-Blue (#E8EEF7)** (primary text color, ~8.5:1 contrast) — drives selector for: critical display text and primary labels distinguishable from secondary/tertiary text hierarchy.
## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** compact-widget, full-dashboard-traces, full-dashboard-metrics, full-dashboard-logs, full-dashboard-snapshots, settings-panel
- **desktop-native:** tray-icon, tray-menu

### Signature Placements

- **compact-widget:** signature element = Halo State Pulse unified badge at bottom-right quadrant of canvas (rationale: "single unified halo circles the service-count badge in the bottom-right quadrant of the canvas, encoding aggregated service health and ingest volume")
- **full-dashboard-traces:** signature element = per-service Halo dots on constellation map at hero section (rationale: "Per-service halos on each constellation dot" with "motion is data-driven (not decorative) — throughput directly controls pulse frequency, error rate directly controls hue")
- **tray-icon:** signature element = unified halo composited around glyph as secondary layer (rationale: "Halo State Pulse (secondary composited layer)" positioned "behind / around the tray icon glyph, rendering the unified halo per the Brand Identity spec")
## 5. Creator Brief Excerpt

### Must-Work Scenarios

- "Cross-platform desktop app that receives OTLP telemetry (HTTP `:4318` and gRPC `:4317`) from any local application, visualizes traces, metrics, and logs in a polished GPU-accelerated UI"
- "runs as a compact always-visible glance-monitor (quarter-screen widget mode) plus a full expanded dashboard"
- "an 'Investigate' button that captures a **token-efficient curated snapshot** (not raw telemetry dump) of recent activity — copies an AI-ready prompt to clipboard for one-paste debug sessions with Claude Code / Cursor / ChatGPT"
- "Optional MCP server lets AI agents query telemetry directly without manual snapshots"
- "Quarter-screen widget mode — default surface; window snaps к side of screen (left / right / corner); always-on-top toggle; remembers position per display"
- "Service health constellation (animated dots per service; color + pulse rate indicate health; throughput visualized as ring intensity)"
- "Click к expand — opens full dashboard window; widget remains mounted (returns к compact mode on close)"
- "Tray icon (secondary) — traffic-light status; click cycles widget visibility (visible / minimized / hidden)"
- "'Investigate' button on widget + main window + context menu" — generates curated snapshot pipeline: Time window selection → Filter (by service / trace ID / error-only / latency-outlier) → Curate (deduplicate, highlight anomalies, extract critical path, aggregate metrics p50/p95/p99/max, drop low-signal attributes) → Format (hierarchical markdown with citation anchors) → Token budget (10k / 25k / 50k preset)
- "Snapshot path detection — if cwd has `.andromeda/` → `.andromeda/pulse/{timestamp}.md`; else → `~/.cache/andromeda-pulse/snapshots/{timestamp}.md`"
- "Clipboard prompt — one of 4 preset templates: 'Claude Code' (default), 'Cursor', 'ChatGPT', 'Custom'"
- "Dual format — both raw `.json` (OTLP-native, for tooling) and `.md` (curated, for AI consumption) written"
- "Notification — `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`"
- "AI agents query telemetry via MCP `tools/call` requests: `query_traces(time_range, service_filter, limit)`, `query_metrics(metric_name, time_range, aggregation)`, `query_logs(filter, time_range, limit)`, `generate_snapshot(time_range, token_budget)`"
- "Custom dashboards — load WASM module that defines а new dashboard panel (input: query results from DuckDB; output: rendered viz spec)"
- "Data transforms — WASM module receives ingest stream; emits transformed events"
- "Snapshot templates — WASM module receives curated snapshot; emits formatted markdown / JSON / custom format"
- "Plugin sandbox — WASM Component Model boundaries; capability-based access (plugins declare needed APIs via WIT interfaces; runtime grants per-plugin)"
- "Settings: Buffer size / retention window. Ingest ports. Snapshot preset template. Snapshot token budget. Snapshot format. Theme. Widget mode. Widget snap position. MCP server toggle. Plugin manager."

### Risk Tolerance Hints

- "BAR. Beat otel-desktop-viewer on UI polish; approach Jaeger UI quality; compete with Uptrace / SigNoz on lightweight-ness; zero-config local install; **portfolio-worthy GPU-accelerated visualization**; AI debug workflow that saves real token cost per investigation."
- "Hand-built Canvas falls over at 10k+ spans/sec. Pulse v2 uses WebGPU / WGSL compute shaders for time-series aggregation + render for trace timeline / flamegraph / metrics charts. Smooth animation; no jank at high cardinality."
- "Scale Intent: startup (shipped public OSS used by external devs, not enterprise-production APM scale)."
- "WebGPU compute shaders ... handles 10k+ spans/sec" (target throughput baseline for performance budgets)
- "WASM Component Model" + "wasmtime sandboxed plugin runtime" + "SIMD vectorization" + "Tauri 2 cross-platform desktop с small bundle (vs Electron)" — bleeding-edge tech stack signals comprehensive coverage warranted on hot paths.
- "Public OSS, MIT, GitHub Releases distribution" — community-facing release; quality bar above MVP but below enterprise APM (startup tier).

### Test Anti-Patterns (creator's explicit asks)

- **Self-observation must not dial own OTLP ports:** "instrumenting an OTLP receiver with an OTLP network exporter pointed back at itself creates an infinite loop. ... Any future `ANDROMEDA_OBSERVER_URL`-shaped variable must explicitly distinguish 'outbound observer' (we *are* the observer — do not dial) from 'inbound receivers' (the OTLP ports we listen on)." (Tests must not configure the product to send its own telemetry to its own receivers; tests that verify this constraint must assert no self-OTLP dialing occurs.)
- **No raw OTLP JSON dump as snapshot:** "Existing snapshot tools dump raw OTLP JSON — а production trace at moderate load = hundreds of thousands of tokens к paste into Claude. Pulse v2 generates **curated** snapshots." (Tests covering snapshot generation must verify token-budget enforcement, dedupe, anomaly highlighting, critical-path extraction — not just byte-for-byte OTLP roundtrip.)
- **Zero-config local install:** "zero-config local install" (BAR section). Tests should not require external services, Docker / Jaeger / cluster infra, or environment-specific setup beyond the bundled binary; harness must be invokable end-to-end on a clean dev machine via `cargo xtask test` or equivalent.
- **No explicit "no flaky" / "no human-in-the-loop verification" asks** — but Development Style = agent-driven (per Setup gate) implicitly forbids manual verification gates and human review checkpoints throughout the test plan.
