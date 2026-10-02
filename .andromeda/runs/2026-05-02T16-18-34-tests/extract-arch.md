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
