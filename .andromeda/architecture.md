## Design Philosophy

- **Local-first, zero-infrastructure** — every byte of telemetry stays on the developer's machine; no Docker, no collector cluster, no cloud backend means the install-to-first-trace loop is one binary launch, not a stack.
- **Single-process modular monolith** — eight library crates linked into the `pulse-app` Tauri binary (ten workspace members total: eight library crates + the `pulse-app` binary + the `xtask` task-runner crate) share memory via tokio channels rather than network hops, so ingest→buffer→viz latency is measured in microseconds and the OS only sees one process. Occupied Resources is the canonical workspace-member list; any claim of a count word elsewhere refers back to it.
- **Standards-track at the edges, opinionated in the middle** — OTLP at `:4317`/`:4318`, MCP over stdio, and WASM Component Model plugins are all spec-conformant so external tooling Just Works; internal contracts (TauRPC bridge, Arrow zero-copy hand-off) are tightly opinionated to keep agent-driven development unambiguous.
- **Token-efficient curation as a first-class output** — the snapshot generator is treated as a peer of the visualization surface, not a side feature, because turning local telemetry into LLM context is the differentiator.
- **Capability-scoped extensibility** — WASM Component Model plugins receive only the host imports they declare in WIT, so third-party plugins cannot escalate beyond explicitly granted resources, and the security posture remains auditable.
- **Developer-tool surface** — the visualization shell is a dense, chart-first, low-chrome dashboard targeting developers staring at telemetry for hours; dark-mode is the default color scheme, and the WebGPU canvas + tray icon + OS-notification surfaces are the visual touchpoints arch commits to. Concrete tokens (color, motion, typography, density scale) are owned by the design specialist but must respect this density and color-scheme constraint.

## Stack and Technologies

| Layer | Technology | Role |
|---|---|---|
| Primary language / runtime | Rust 2024 edition (rustc 1.84+) | Single language across receivers, buffer, viz host, IPC, plugin host, MCP server |
| Desktop shell | Tauri 2.x | Native window + webview, OS bundlers, updater, tray, notifications |
| gRPC server | `tonic` 0.14.x | OTLP/gRPC receiver on `:4317` (with `prost` 0.14 codegen against `opentelemetry-proto`) |
| HTTP server | `axum` 0.8.x on `hyper` 1.x + `tower` | OTLP/HTTP receiver on `:4318` (protobuf and JSON) sharing the Tokio runtime with tonic |
| Async runtime | `tokio` (current stable) | Single multi-threaded runtime serving both OTLP ports and IPC |
| In-process channels | `tokio::sync::mpsc` + `tokio::sync::broadcast` | Ingest→appender hand-off (mpsc, backpressure) and buffer→subscribers fan-out (broadcast) |
| Storage engine | DuckDB 1.5.x via `duckdb` crate 1.10500.x | Embedded columnar OLAP, in-memory ring buffer (5–10 min, configurable) |
| Columnar interchange | Apache Arrow (via `Appender::append_record_batch()` / `stream_arrow()`) | Zero-copy hand-off between OTLP decode → DuckDB → viz/MCP |
| Plugin runtime | `wasmtime` 25+ with WASM Component Model + WIT | Capability-scoped third-party extensions loaded from `~/.andromeda-pulse/plugins/` |
| Visualization surface | Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL) | GPU-accelerated charts inside WebView2 (Windows) and WKWebView (macOS/Linux equivalents) |
| Tauri IPC bridge | TauRPC (`taurpc` crate) | Router-style Rust ↔ webview commands with auto-generated TypeScript bindings |
| MCP server | `rmcp` (official Rust SDK) over stdio | `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` as `#[tool]` methods (gated by `--features mcp-server`) |
| OS notifications | `tauri-plugin-notification` 2.x | Native "Snapshot ready" toasts (Notification Center / Action Center / freedesktop) |
| Updater | `tauri-plugin-updater` 2.x with `latest.json` | In-app updates from GitHub Releases |
| Mobile framework | N/A — desktop only (Windows / macOS / Linux) | — |
| Message broker | N/A — single-process desktop app | — |
| AI/ML serving | N/A — no in-process model; emits curated markdown for external clients | — |
| Push notifications | N/A — OS-native local toasts only, no FCM/APNS/web push | — |
| Error handling | `thiserror` 2.x (modules) + `anyhow` 1.x (boundaries) | Module-internal enums; boundary catch-all at Tauri commands and `main.rs` |
| Validation | `serde` + smart enum types + `TryFrom<u16>` | Validation surface is small (enum settings, port ranges, file paths); OTLP validated by `prost`/`tonic` decode |
| Module boundary enforcement | Cargo workspace, one crate per module | Compile-time `[dependencies]` graph + `pub`/`pub(crate)` visibility |
| Build / package manager | Cargo (workspace) | One workspace, multiple library crates, one binary crate |
| CI task runner | `cargo-xtask` | Release / sign / notarize as Rust binaries inside the same workspace |
| Release pipeline | `tauri-action` GitHub Action + Tauri 2 native bundlers | Builds `.msi` / `.dmg` / `.AppImage` / `.deb` distribution artifacts (macOS `.app` bundle is wrapped into the `.dmg` rather than published standalone) and uploads `latest.json` per release |
| Code signing | Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization) | Trusted-publisher signing for all distributed bundles |
| Distribution channels | GitHub Releases (primary) + Homebrew tap + Scoop manifest | Three channels driven from the same workflow |
| Code quality (lint/typecheck) | `cargo fmt` + `cargo clippy` (Rust); TauRPC-generated `.d.ts` + `tsc --noEmit` (webview) | Lint and typecheck on every CI run; clippy lints fail the build |

## Established Decisions

- **[Platform] Tauri 2 desktop shell**: chosen over Electron because bundle size is 5–10× smaller (~10–20 MB vs 100 MB+), the security model is capability-scoped, and a Rust backend means the receiver, buffer, and UI bridge live in one process with no Node bridge tax.
- **[Primary Language] Rust 2024 edition (rustc 1.84+)**: matches the high-performance OTel-receiver trend (Rotel benchmarks: 75% less memory, 50% less CPU than the Go Collector) and means one language end-to-end through ingest, buffer, viz host, plugin host, and MCP server.
- **[Backend Framework — OTLP receiver] `tonic` 0.14.x + `axum` 0.8.x on shared `hyper` 1.x + `tower`**: a single Tokio runtime serves both `:4317` gRPC and `:4318` HTTP without two unrelated substrates; axum's middleware/extractor ergonomics outweigh the ~30 transitive deps for an 8-module monolith. Alternatives (raw hyper, poem+tonic, actix+tonic) either lost on ergonomics or fought Tauri's `tokio::main` integration. **Caveat to reconcile before locking versions**: `tonic` 0.14 vs `opentelemetry-otlp` 0.31 (which still pins `tonic` 0.13 in some feature combinations) — read both `Cargo.toml`s before tagging.
- **[Database] DuckDB embedded via `duckdb` crate 1.10500.x (DuckDB 1.5.x bundled), in-memory ring buffer**: columnar+SQL+Arrow trifecta is exactly what aggregating spans/metrics by service/time-bucket needs at 10k spans/sec. SQLite was rejected because aggregating 10k spans/sec into per-service p99 series in row-store SQL turns the query into the bottleneck. DataFusion remains a documented swap-out only if DuckDB's C++ FFI complicates Tauri cross-compile.
- **[ORM / Migrations] None — direct SQL via `duckdb` crate's `Connection` and `Appender`**: ring-buffer schema is small, lifecycle is "create on startup" + periodic `DELETE WHERE ts < cutoff`, and Arrow zero-copy ingest sidesteps any ORM marshalling tax.
- **[In-Process Channel Architecture] `tokio::sync::mpsc` + `tokio::sync::broadcast`**: mpsc for the OTLP-ingest → DuckDB-appender hand-off (built-in backpressure absorbs receiver bursts); broadcast for fan-out from buffer to live UI subscribers (compact widget, full dashboard, tray icon, MCP server can all subscribe to the same span stream). `crossbeam-channel` was rejected because mixing tokio mpsc with crossbeam adds bridging complexity for a pure-tokio stack.
- **[Plugin Runtime] WASM Component Model via `wasmtime` 25+ with WIT interfaces**: capability-based sandboxing (no syscalls, file, or socket access unless explicitly granted) and type-safe host imports match the standards path. Extism was considered for easier multi-language guest support, but Component Model is the standards-track choice the input.md explicitly calls out and aligns with Bytecode Alliance LTS.
- **[WebGPU Visualization Surface] Webview WebGPU (`<canvas>` + `navigator.gpu`)**: WGSL shaders run unchanged across WebView2 (Windows) and WKWebView (macOS); IPC pushes Arrow data into the canvas. Native `wgpu` 25+ surface was rejected for v1 because Tauri 2 native-overlay flickering is documented (issue #9220), and "ship on three OSes with one codebase" beats "avoid theoretical browser GC pauses." **Documented upgrade path**: native `wgpu` 25+ surface for the compact widget if browser GC jank becomes a measured problem (hybrid Option C).
- **[Tauri IPC Bridge — Surface 2] TauRPC (`taurpc` crate)**: derive macro auto-generates fully-typed TypeScript bindings from the Rust command surface, eliminating the drift bugs that an 8-module monolith with 30+ commands would otherwise produce. Stock `#[tauri::command]` was rejected because manual TS re-declaration is error-prone at this module count; tauri-specta is composable but more ceremonial per command. **Keystone**: this choice forces all error types crossing the bridge to be `Serialize`-able.
- **[MCP Server Surface] `rmcp` (official Rust SDK) over stdio with `#[tool]`-annotated methods**: `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` map to tool methods; gated by `--features mcp-server` (default off) so the rmcp dependency only enters the build when explicitly enabled. **Caveat to verify before locking**: the input-cited "rmcp 1.5.0" must be reconciled against the currently published 0.3.x line — verify whether input is forward-looking, refers to an internal spec, or refers to the unrelated `4t145/rmcp` fork.
- **[OTLP Wire Format Coverage] Classic OTLP for v1 (protobuf over HTTP `:4318` + gRPC `:4317`)**: every SDK in the wild emits classic OTLP today; OTAP (Arrow-encoded) is the 2025–2026 performance frontier but server-side and emerging. OTAP ingest is implemented behind a feature flag, activated when SDK clients catch up.
- **[API Style — OTLP Receiver Surface 1] OpenTelemetry spec, no fork**: `:4317` = gRPC over `opentelemetry-proto`; `:4318` = HTTP/protobuf or HTTP/JSON. Wire format is fixed by the spec.
- **[API Style — MCP Server Surface 3] JSON-RPC 2.0 over stdio per MCP spec**: rmcp's macros are the canonical surface; do not invent a custom wire format.
- **[Error Handling Pattern] `thiserror` 2.x for module-internal error enums + `anyhow` 1.x at the Tauri command boundary and `main.rs` + a `serde`-friendly `AppError` enum wrapper for IPC**: matches Rust 2026 community consensus; AppError is lossy-but-explicit because TauRPC requires `Serialize`-able errors. **Documented upgrade path**: add `miette` 7.x for plugin-loading diagnostics if WIT resolution UX surfaces source-span-worthy failures.
- **[Validation Library] None — `serde` + smart enum types + `TryFrom<u16>` impls**: validation surface is tiny (enum settings, port ranges, file paths) and OTLP protobuf payloads are validated by `prost`/`tonic` decode itself. **Documented upgrade path**: reach for `garde` 0.20+ when the plugin manifest gains cross-field validation (any rule that must inspect more than one field together — e.g. "version range required if capability list non-empty"), or when the manifest exceeds ~10 declared fields, whichever comes first; until then `serde` + `TryFrom` covers each field independently.
- **[Module Boundaries] Cargo workspace, one library crate per module + `pulse-app/` binary**: an 8-module project with "modular monolith" stated intent must enforce boundaries via the dependency graph (not review discipline); single-crate `mod` directories let any internal `pub(crate)` leak across modules. **Deferred**: `cargo-deny` 0.16+ `bans` rules are added when a boundary is actually breached, not from day 1.
- **[Snapshot Curation Default] Balanced — 25k tokens, full dedupe + critical-path + p50/p95/p99 + anomaly highlighting**: matches the "AI-debug snapshot" differentiator and is small enough to fit in any frontier-model context budget while large enough to preserve a meaningful distributed trace.
- **[Plugin Distribution Channel — v1] Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/`**: signed-plugin verification deferred post-v1; no marketplace UI in v1. Lowest-friction path to "first plugin in 30 minutes."
- **[Telemetry Retention Surface] In-memory DuckDB ring buffer only (5–10 min, configurable)**: matches the "local-dev iteration loop" use case; persistent disk storage is reserved for the heavier backends and is out of scope.
- **[Mobile / Message Broker / AI-ML Serving / Push Notifications] N/A**: single-process desktop app on Windows/macOS/Linux only; no in-process model; OS-native local notifications only.
- **[Self-Observation] Use `opentelemetry-stdout` (or file exporter) for the product's own telemetry; never dial the product's own OTLP ports**: instrumenting an OTLP receiver with an OTLP network exporter pointed back at itself creates an infinite loop. Any future `ANDROMEDA_OBSERVER_URL`-shaped variable must explicitly distinguish "outbound observer" (we *are* the observer — do not dial) from "inbound receivers" (the OTLP ports we listen on).
- **[Deployment / Release Pipeline] `tauri-action` GitHub Action + Tauri 2 native bundlers**: one workflow file produces `.msi` / `.dmg` / `.AppImage` / `.deb` (the macOS `.app` bundle is wrapped into the `.dmg` rather than uploaded standalone) and auto-uploads `latest.json` for the Tauri updater plugin. Hand-rolled matrix and cross-rs-with-custom-packagers were rejected because they lose Tauri-updater integration.
- **[Code Signing] Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization)**: the standard trusted-publisher path for OSS desktop apps in 2026.
- **[CI Task Runner] `cargo-xtask`**: lightest fit for a Rust-only repo; release/sign/notarize tasks are Rust binaries in the same workspace, matching agent-driven harness ergonomics. `just` and `cargo-make` add DSL surface area without buying anything for one app with no non-Rust contributors.
- **[Distribution Channels] GitHub Releases (primary) + Homebrew tap + Scoop manifest**: three channels driven from the same workflow via `taiki-e/upload-rust-binary-action`-style patterns. Rationale: GitHub Releases is the minimum viable distribution; Homebrew + Scoop match developer-tool install norms (`brew install andromeda-pulse` / `scoop install andromeda-pulse`) for the agent-driven-developer target user. **Deferral path**: if the Homebrew tap or Scoop bucket setup blocks the v0.1.0 ship, drop them to v0.2.0 and ship GitHub Releases alone.

## Conventions

- **Workspace API style — internal Rust↔webview**: TauRPC routers, one router per module crate (`ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `workspace-detector`, `plugins`, `mcp-server`); no manual TypeScript re-declaration of command signatures. Frontend consumes only TauRPC-generated `.ts` bindings.
- **External wire — OTLP receiver**: OpenTelemetry spec exactly. `:4317` accepts gRPC over `opentelemetry-proto`; `:4318` accepts HTTP/protobuf at `/v1/traces`, `/v1/metrics`, `/v1/logs` (HTTP/JSON encoding optional). No URL versioning prefix beyond what the OTLP spec defines.
- **External wire — MCP server**: JSON-RPC 2.0 over stdio per MCP spec; tool methods are `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`.
- **Error response schema (Tauri IPC)**: every `#[taurpc::procedure]` returns `Result<T, AppError>` where `AppError` is a `serde`-friendly enum with stable variants (`Validation { field, reason }`, `NotFound { resource }`, `Internal { message }`, `Plugin { plugin_id, message }`, `Storage { message }`, `Ingest { message }`). Module-internal code uses `thiserror`-derived enums and converts at the bridge via `From` impls. `main.rs` and any non-IPC top-level uses `anyhow::Result`.
- **Error response schema (OTLP receiver)**: per OpenTelemetry HTTP spec — non-2xx responses carry `Status` proto in body for HTTP/protobuf, or HTTP/JSON for HTTP/JSON; gRPC errors use standard `tonic::Status` codes.
- **Error response schema (MCP)**: standard JSON-RPC 2.0 error object with stable `code` (numeric), `message` (string), and optional `data`; MCP-spec error codes are honored.
- **File naming**: Rust files are `snake_case.rs`; one library crate per module under `crates/<module-name>/`; binary crate is `pulse-app/`. WIT interface files are `kebab-case.wit`. Webview source files follow the design specialist's convention (out of scope here).
- **Variable / function naming**: Rust `snake_case` for functions/variables/modules, `UpperCamelCase` for types/traits/enums, `SCREAMING_SNAKE_CASE` for constants — standard `cargo clippy` `style` group enforced in CI.
- **Endpoint naming**: OTLP endpoints are spec-fixed (`/v1/traces`, `/v1/metrics`, `/v1/logs`). Tauri IPC procedures use one of two authorized shapes: (a) top-level bare `snake_case` verbs for the cross-cutting envelope (`app_info`, `health`, `ready`, `get_settings`, `update_settings`); (b) `<router>.<verb>` dotted namespaces for per-crate routers (e.g., `traces.query`, `snapshot.generate`, `plugins.list`, `mcp.start`, `workspace.detect`). Both segments are `snake_case`. Occupied Resources is the canonical procedure list; the Convention defines the shape, not the enumeration.
- **Database entity naming**: DuckDB tables use plural `snake_case` matching the OTLP entity they hold (reserved set in Occupied Resources: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`); columns are `snake_case`; ring-buffer cutoff is enforced via a periodic `DELETE FROM <table> WHERE ts < now() - INTERVAL '<retention> minutes'` task. Occupied Resources is the canonical table-name list; the Convention defines the shape, not the enumeration.
- **Primary key convention**: spans use the OTLP-native 16-byte `trace_id` + 8-byte `span_id` as composite identity (no surrogate); metric points and log records use OTLP-native identity (timestamp + resource hash + name); no UUID surrogate keys are introduced because OTLP entities already have spec-defined IDs.
- **Timestamp handling**: all DuckDB timestamp columns are `TIMESTAMPTZ` (DuckDB native, microsecond precision, UTC-stored); incoming OTLP nanosecond timestamps are stored as `TIMESTAMPTZ` (microsecond truncation accepted) plus a sibling `BIGINT` column `ts_unix_nano` when nanosecond precision is required for spec round-tripping.
- **Nullable patterns**: nullable columns are reserved for OTLP-spec-optional fields (e.g., `parent_span_id`, optional resource attributes); required spec fields are `NOT NULL`.
- **Module visibility discipline**: each crate exposes only its public contract via `pub`; cross-crate utilities use `pub(crate)`; no `pub use` re-exports across crate boundaries except in the explicit contract module of each crate.
- **Feature flags**: Cargo features are `kebab-case` (`mcp-server`, `otap-ingest`); default features are minimal (no `mcp-server`, no `otap-ingest`).
- **Configuration units**: durations in user-facing config are seconds with explicit units in field names (`retention_seconds`); ports are `u16` with `TryFrom<u16>` validating non-privileged-or-explicitly-allowed ranges.

## Standard Contracts

- **Tauri IPC introspection — `app_info` command**: every TauRPC router exposes a top-level `app_info` returning the application identity envelope.

  ```json
  {
    "name": "andromeda-pulse",
    "version": "0.1.0",
    "rust_version": "1.84.0",
    "tauri_version": "2.x",
    "features": ["mcp-server"],
    "build_profile": "release"
  }
  ```

- **Tauri IPC liveness — `health` command**: returns liveness state of the in-process subsystems.

  ```json
  {
    "status": "ok",
    "checked_at": "2026-05-02T12:34:56Z",
    "subsystems": {
      "otlp_grpc_receiver": "ok",
      "otlp_http_receiver": "ok",
      "buffer": "ok",
      "ingest_channel": "ok"
    }
  }
  ```

  When degraded, `status` becomes `"degraded"` and any failing subsystem string becomes the failure reason (e.g., `"otlp_grpc_receiver": "bind_failed: address in use"`); HTTP-style status is implicit via the IPC `Result` (a hard failure returns `Err(AppError::Internal { ... })`).

- **Tauri IPC readiness — `ready` command**: returns whether the app is ready to ingest and serve queries.

  ```json
  {
    "ready": true,
    "checked_at": "2026-05-02T12:34:56Z",
    "checks": {
      "duckdb_connection": "ok",
      "ingest_mpsc_capacity_pct": 0,
      "broadcast_subscribers": 2,
      "plugins_loaded": 0,
      "mcp_server_enabled": false
    }
  }
  ```

  When not ready, `ready` is `false` and any failing check is the offending reason string (e.g., `"duckdb_connection": "init_in_progress"`).

- **OTLP receiver — spec-conformant**: `:4317` gRPC implements the `TraceService`, `MetricsService`, `LogsService` from `opentelemetry-proto`; `:4318` HTTP exposes `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` accepting `Content-Type: application/x-protobuf` (and `application/json` per spec). No project-specific envelope.

- **MCP server — spec-conformant**: standard MCP `tools/list` returns `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`; standard `tools/call` invokes them with JSON-RPC 2.0 framing over stdio. Capabilities, prompts, resources, sampling sections follow the MCP spec defaults.

- **Common response envelope (Tauri IPC paginated lists)**: list-returning IPC commands wrap results in:

  ```json
  {
    "items": [],
    "total": 0,
    "next_cursor": null
  }
  ```

  Cursor is an opaque string; absent (`null`) means "no more results."

- **Real-time push contract**: live span/metric streams to the webview use Tauri 2 IPC `Channel` API with binary Arrow IPC payloads (no JSON-stringify tax); event names are `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`. SSE/WebSocket are not used because the consumer is in-process.

## Occupied Resources

- **Network ports**:
  - `:4317` — OTLP/gRPC receiver (spec-fixed; bound on `127.0.0.1` only)
  - `:4318` — OTLP/HTTP receiver (spec-fixed; bound on `127.0.0.1` only)
  - No frontend dev server port reserved at arch level (managed by webview, not exposed)
- **Tauri IPC routes (TauRPC procedures)**:
  - `app_info`, `health`, `ready`, `get_settings`, `update_settings` — top-level (ui-bridge crate)
  - `traces.*`, `metrics.*`, `logs.*` — query routers (viz crate)
  - `streams.subscribe_spans`, `streams.subscribe_metrics`, `streams.subscribe_logs` — pulse-app crate (Tauri Channel<Vec<u8>> binding to `buffer::BroadcastSenders` for binary Arrow IPC; chunk #23) — see §Architecture Registry Updates 2026-05-09
  - `telemetry.frontend.record_frame_ms` — ui-bridge crate (`FrameDurationInput` → `metric.webgpu.frame_duration_ms` tracing event per obs-plan §11 Frontend bridge; chunk #29) — see §Architecture Registry Updates 2026-05-09
  - `snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard` — snapshot crate
  - `plugins.list`, `plugins.reload`, `plugins.invoke` — plugins crate
  - `mcp.status`, `mcp.start`, `mcp.stop` — mcp-server crate (only when `--features mcp-server`)
  - `workspace.detect`, `workspace.list` — workspace-detector crate
  - `connection.current_state` — pulse-app crate (`ConnectionApiImpl` returning `ConnectionStatePayload` from `crates/ingest::connection::compute_state()`; chunk #59) — see §Architecture Registry Updates 2026-05-16
  - `services.list_with_states` — pulse-app crate (`ServicesApiImpl` returning `ServiceListPayload` from `crates/triage::lifecycle::InMemoryServiceRegistry::list`; chunk #67) — see §Architecture Registry Updates 2026-05-18
  - `storage.inspect`, `storage.path` — pulse-app crate (`StorageApiImpl` returning corpus inspection metadata + data-dir absolute path from `crates/corpus::CorpusReader`; chunk #68) — see §Architecture Registry Updates 2026-05-18
  - `diagnostics.template_distribution` — pulse-app crate (`DiagnosticsApiImpl` returning `TemplateDistributionPayload` from `crates/buffer::DrainMiner::template_distribution()`; chunk #69) — see §Architecture Registry Updates 2026-05-19
- **External HTTP routes (OTLP HTTP)**: `POST /v1/traces`, `POST /v1/metrics`, `POST /v1/logs` on `:4318`.
- **MCP stdio surface**: standard MCP `initialize`, `tools/list`, `tools/call`, `notifications/*` over stdin/stdout when the rmcp sidecar is started.
- **Tauri IPC events (broadcast channels)**: `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs`, `pulse://stream/snapshot-progress`, `pulse://stream/plugin-events`, `pulse://stream/connection-state` (chunk #59 — see §Architecture Registry Updates 2026-05-16), `pulse://stream/attention-cues` (chunk #62 — see §Architecture Registry Updates 2026-05-17), `pulse://stream/restart-events` (chunk #63 — see §Architecture Registry Updates 2026-05-17), `pulse://stream/service-lifecycle` (chunk #67 — see §Architecture Registry Updates 2026-05-18).
- **Process / service identity**:
  - Tauri app bundle identifier: `com.andromeda.pulse`
  - Binary name: `andromeda-pulse` (Linux/macOS), `andromeda-pulse.exe` (Windows)
  - Binary crate name: `pulse-app`
  - rmcp sidecar binary (when feature enabled): `andromeda-pulse-mcp`
- **Cargo workspace crate names**: `ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `curation`, `triage`, `workspace-detector`, `plugins`, `mcp-server`, `corpus`, `security`, `pulse-app`, `xtask` — these names are reserved at the workspace level and cannot be reused by scopes.
- **DuckDB database / schema names**:
  - In-memory database identity: `pulse_buffer` (single in-memory `:memory:` DuckDB connection, schema `main`)
  - Reserved tables: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`
- **Filesystem locations** (the `~/.andromeda-pulse/` notation below is the Linux canonical form; per-platform resolution is fixed and applies to every path under this root):
  - Linux: `~/.andromeda-pulse/` (i.e. `$XDG_CONFIG_HOME/andromeda-pulse/` if set, else `$HOME/.andromeda-pulse/`)
  - macOS: `~/Library/Application Support/com.andromeda.pulse/`
  - Windows: `%APPDATA%\andromeda-pulse\` (i.e. `%APPDATA%\andromeda-pulse\config.toml`, `...\plugins\`, `...\snapshots\`, `...\logs\`)
  - Subpaths under the resolved root: `config.toml` (user settings), `plugins/` (WASM Component Model plugin loading directory), `snapshots/` (generated snapshot markdown files), `logs/` (stdout-exporter destination for self-telemetry), `corpus/corpus.db` (persistent incident corpus SQLite; OS-keychain-encrypted cell-level AES-256-GCM — chunk #68).
  - `ANDROMEDA_PULSE_DATA_DIR` overrides the resolved root on every platform; subpath layout under the override is identical to the per-platform default.
- **Environment variables (reserved at arch level)**:
  - `ANDROMEDA_PULSE_CONFIG_PATH` — override path to `config.toml`
  - `ANDROMEDA_PULSE_DATA_DIR` — override `~/.andromeda-pulse/` root
  - `ANDROMEDA_PULSE_OTLP_GRPC_PORT` — override `:4317`
  - `ANDROMEDA_PULSE_OTLP_HTTP_PORT` — override `:4318`
  - `ANDROMEDA_PULSE_RETENTION_SECONDS` — override ring-buffer retention (300–600 default range)
  - `ANDROMEDA_PULSE_LOG_LEVEL` — `trace|debug|info|warn|error`
  - `ANDROMEDA_PULSE_MCP_ENABLED` — `true|false` (only honored when binary built with `--features mcp-server`). When set to `true` against a binary built without the feature, startup logs a warning at `warn` level naming the missing feature flag and proceeds with MCP disabled (rather than failing to start), so a misconfigured environment variable does not block the rest of the app.
  - `ANDROMEDA_PULSE_PLUGIN_DIR` — override `~/.andromeda-pulse/plugins/`
  - `RUST_LOG` — honored as fallback for log level filter
- **Tauri capability identifiers (reserved at arch level)**: `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`, `pulse:clipboard` — concrete capability JSON files live in `pulse-app/capabilities/`. See §Architecture Registry Updates 2026-05-11 for `pulse:clipboard` chunk #43 acknowledgment.
- **Updater channel**: `latest.json` published to GitHub Releases under the canonical repository; updater public key is shipped baked into the Tauri config.
- **Bundle artifact names** (per release): `andromeda-pulse_<version>_x64-setup.msi`, `andromeda-pulse_<version>_x64.dmg`, `andromeda-pulse_<version>_aarch64.dmg`, `andromeda-pulse_<version>_amd64.AppImage`, `andromeda-pulse_<version>_amd64.deb`.
- **Docker volumes**: N/A — no Docker.

## Infrastructure Patterns

**Build system.**
- Cargo workspace; package manager is `cargo`.
- Lint: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Typecheck: implicit in `cargo check --workspace --all-targets`; webview TypeScript bindings emitted by TauRPC are typechecked with `tsc --noEmit`.
- Build: `cargo tauri build` (invoked by `tauri-action`); release profile is workspace default with `lto = "thin"`, `codegen-units = 1`, `strip = true` for distribution bundles.

**Deployment model.**
- Public OSS desktop app distributed through GitHub Releases; "deployment" is the release pipeline, not server hosting.
- No Docker, no Kubernetes, no serverless, no docker-compose.
- Runtime topology on the user's machine: one Tauri process hosting all eight library crates and the embedded webview; one optional rmcp sidecar process (only when `--features mcp-server` is enabled at build time and `ANDROMEDA_PULSE_MCP_ENABLED=true` at runtime).

**Project directory structure.**

```
andromeda-pulse/
├── Cargo.toml                      # workspace manifest
├── Cargo.lock
├── rust-toolchain.toml             # pin rustc 1.84+
├── .github/
│   └── workflows/
│       ├── release.yml             # tauri-action: build + sign + notarize + publish
│       ├── ci.yml                  # fmt + clippy + cargo-xtask test
│       └── update-channels.yml     # Homebrew tap + Scoop manifest jobs
├── crates/
│   ├── ingest/                     # OTLP receivers (tonic + axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── buffer/                     # DuckDB ring buffer + Arrow appender
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── viz/                        # query layer feeding webview WebGPU charts
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── ui-bridge/                  # TauRPC routers
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── snapshot/                   # curated markdown generator
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── workspace-detector/         # detect host project context
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugins/                    # wasmtime Component Model host
│   │   ├── Cargo.toml
│   │   ├── wit/                    # WIT interface definitions
│   │   └── src/
│   └── mcp-server/                 # rmcp stdio sidecar (feature-gated)
│       ├── Cargo.toml
│       └── src/
├── pulse-app/                      # Tauri binary crate that wires the workspace
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/               # Tauri 2 capability JSON files
│   ├── icons/
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── ui/                         # webview source root (frontend tooling owned by design specialist)
│       └── src/
├── xtask/                          # cargo-xtask: release/sign/notarize/changelog tasks
│   ├── Cargo.toml
│   └── src/
├── plugins-examples/               # built-in plugin templates shipped with v1
│   └── README.md
├── docs/
└── README.md
```

**CI/CD approach.**
- Platform: GitHub Actions.
- `ci.yml` runs on every PR and push to main: matrix over Linux/macOS/Windows; steps `cargo fmt --check` → `cargo clippy ... -D warnings` → `cargo xtask test` → `cargo build --workspace` (release profile smoke).
- `release.yml` runs on tag push (`v*`): `tauri-action` builds `.msi` / `.dmg` / `.AppImage` / `.deb` per OS in matrix (macOS `.app` bundle is built then wrapped into `.dmg`, not published standalone); signs Windows artifacts via Azure Key Vault EV cert; notarizes macOS artifacts via Apple Developer ID; uploads bundles + `latest.json` to GitHub Releases.
- `update-channels.yml` runs on completion of `release.yml`: updates Homebrew tap and Scoop manifest with new version + sha256.
- All shared CI logic that needs Rust lives in the `xtask` crate so contributors can run identical commands locally with `cargo xtask <task>`.

## Cross-cutting Patterns

- **Config management**: layered with explicit precedence (highest wins) — process env vars (`ANDROMEDA_PULSE_*`) > user `~/.andromeda-pulse/config.toml` > built-in defaults. No external secrets manager (this is a local OSS desktop app); code-signing secrets live only in GitHub Actions encrypted secrets and Azure Key Vault.
- **Self-observation discipline**: `opentelemetry-stdout` (or file exporter targeting `~/.andromeda-pulse/logs/`) is the only exporter the product itself uses for its own telemetry; the OTLP network exporter is never pointed at the product's own `:4317`/`:4318` ports. Any future "outbound observer" variable must be explicitly distinct from the inbound receiver port variables.
- **Feature-gate hygiene**: `--features mcp-server` and `--features otap-ingest` (and any future flags) are off by default; flag-specific dependencies (`rmcp`, OTAP-specific crates) only enter the build graph when the feature is enabled, preserving the small-binary baseline.
- **Cross-bridge data shape**: any type that crosses the TauRPC bridge or appears in an MCP tool response must be `serde::Serialize`; this is enforced at compile time by the bridge derive macros. Internal-only types are not constrained.
- **OS notification policy**: native OS notifications (via `tauri-plugin-notification`, gated by capability `pulse:notification`) fire on completion of `pulse://stream/snapshot-progress` and on updater state transitions; no other subsystem emits notifications without an explicit decision. User opt-out is a single boolean `notifications_enabled` in `~/.andromeda-pulse/config.toml` (default `true`); when disabled, the snapshot generator and updater still complete their work but suppress the toast. Notification text shape and per-locale strings are owned by the design specialist.
- **Tray icon policy**: a single tray icon (gated by capability `pulse:tray`) is the always-on surface that signals app-running state and offers a minimal action menu — at minimum: open/focus the main window, show ingest summary (current spans/sec, retention used), toggle MCP server (only when `--features mcp-server` is built and the `mcp-server` crate is loaded), generate snapshot (invokes `snapshot.generate`), quit. Tray click on the icon focuses or restores the main window; closing the main window minimizes to tray rather than terminating the process (process termination requires the explicit Quit menu item or OS-level kill). Tray icon glyphs and per-locale menu labels are owned by the design specialist; menu accessibility (keyboard navigation, screen-reader labels) is owned by the a11y specialist.
- **Webview IPC capability policy**: the main webview window runs under capability `pulse:default`, which permits exactly the TauRPC procedures enumerated in Occupied Resources (the top-level envelope plus every `<router>.<verb>` in the reserved list) and nothing else; adding a new TauRPC procedure requires both a router registration in the owning crate and an entry in `pulse-app/capabilities/` so the bridge call is not silently rejected at runtime. The updater capability `pulse:updater` is bound to the `tauri-plugin-updater` flow that consumes `latest.json` from GitHub Releases and is not exposed to webview JavaScript; Tauri core APIs beyond the enumerated TauRPC surface (filesystem, shell, dialog, http) are not granted to `pulse:default` and require an explicit per-feature capability addition with a stated rationale.
- **Module dependency direction**: dependencies in `Cargo.toml` flow toward the `pulse-app` binary; no library crate depends on the binary crate; no library crate depends on a sibling unless its declared contract requires it. The dependency graph forms a DAG with `pulse-app` as the only root.
- **Test-time telemetry injection**: end-to-end tests inject synthetic spans/metrics/logs by speaking OTLP to the running receiver on `:4317` (gRPC) or `:4318` (HTTP) — the same surface external SDKs use. No in-process test-mode bypass, no separate "test ingest" feature flag, no mock-channel back-door. Tests that need a non-default port resolve it via `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT`. Tests that need to inspect buffer state read it back through the TauRPC `traces.*` / `metrics.*` / `logs.*` query routers.
- **Development Style**: agent-driven. Downstream specialist plans (tests, obs, setup-project) should branch their research toward agent-driven development workflows (deterministic harness invocations, machine-parseable outputs, schema-stable contracts, `cargo xtask` task surfaces).

## Project Intent

- **Product type**: cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles. Not a web app, not a CLI, not a microservice fleet.
- **Scale intent**: startup. Single-machine, single-user; no tenancy, no orchestration, no clustering.
- **Growth model**: modular monolith (eight Rust crates wired into one Tauri binary) with a WASM Component Model plugin extension layer for third-party additions. Internal modules are the unit of growth for first-party functionality; WASM components are the unit of growth for community/third-party functionality.
- **How new functionality is added**:
  - First-party features: new scopes via `/andromeda-scope-arch`, which generally adds either (a) a new Cargo crate to the workspace following the established crate-per-module pattern, or (b) new TauRPC routers/queries/snapshot strategies inside an existing crate. The eight reserved crate names cannot be re-purposed.
  - Third-party features: WASM Component Model plugins dropped into `~/.andromeda-pulse/plugins/`, exposing functionality through host-imported WIT interfaces with capability-scoped access.
- **Template patterns**: each library crate follows the same shape — a public contract module (the only `pub` surface), `pub(crate)` internals, `thiserror`-derived error enum, and (where applicable) a TauRPC-router module mounted by `pulse-app`. Plugin authors follow the WIT-interface template under `crates/plugins/wit/` and the example plugins under `plugins-examples/`.

## Inherited Defaults

- Language / runtime: Rust 2024 edition (rustc 1.84+), single Tokio multi-threaded runtime.
- Desktop shell: Tauri 2.x with TauRPC IPC bridge.
- Backend framework (in-process receivers): `tonic` 0.14.x (gRPC, `:4317`) + `axum` 0.8.x on `hyper` 1.x + `tower` (HTTP, `:4318`). Open question carried from Established Decisions: if `opentelemetry-otlp` 0.31's transitive pin on `tonic` 0.13 cannot be resolved at lock time, the fallback is to downgrade Stack to `tonic` 0.13.x (matching `opentelemetry-otlp`) rather than fork or wait for upstream; the Stack/Decisions/Defaults rows are then re-pinned in the same iteration.
- Database: DuckDB 1.5.x via `duckdb` crate 1.10500.x, in-memory ring buffer (5–10 min retention, configurable).
- ORM / migrations: none — direct SQL via `duckdb` crate `Connection` + `Appender`; schema created on startup.
- Columnar interchange: Apache Arrow zero-copy via `Appender::append_record_batch()` / `stream_arrow()`.
- Channels: `tokio::sync::mpsc` for ingest→appender, `tokio::sync::broadcast` for buffer→subscribers.
- API style — internal IPC: TauRPC routers, one per crate, with two authorized procedure shapes — top-level bare `snake_case` verbs for the cross-cutting envelope (`app_info`, `health`, `ready`, `get_settings`, `update_settings`) and `<router>.<verb>` dotted namespaces for per-crate routers (`traces.query`, `snapshot.generate`, `plugins.list`, etc.). See Conventions for the shape rule and Occupied Resources for the canonical procedure list.
- API style — external OTLP: spec-fixed (`/v1/traces`, `/v1/metrics`, `/v1/logs` on `:4318`; gRPC services on `:4317`).
- API style — external MCP: JSON-RPC 2.0 over stdio with rmcp `#[tool]` methods (feature-gated). Open question carried from Established Decisions: the input-cited "rmcp 1.5.0" must be reconciled against the published `0.3.x` line before locking — verify whether the reference is forward-looking, an internal spec name, or the unrelated `4t145/rmcp` fork; if 1.5.0 cannot be sourced, fall back to the latest published `0.3.x` and re-pin Stack accordingly.
- Plugin runtime: `wasmtime` 25+ Component Model with WIT interfaces, capability-scoped. The Tauri-side capability `pulse:plugin-fs` gates the host's read of `~/.andromeda-pulse/plugins/` for module discovery and is independent of the per-guest WIT capability grants (which never include host filesystem unless a plugin's WIT explicitly imports a fs interface).
- Visualization: webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL).
- Error handling: `thiserror` 2.x in modules, `anyhow` 1.x at boundaries, `serde`-friendly `AppError` enum at the IPC bridge.
- Validation: `serde` + smart enum types + `TryFrom<u16>`; no validation library by default.
- Module boundaries: Cargo workspace, one crate per module, dependency-graph enforcement.
- Build: Cargo + `cargo-xtask` for release/sign/notarize tasks.
- Deployment: `tauri-action` GitHub Action + Tauri 2 native bundlers + Tauri updater plugin (`latest.json`).
- Distribution: GitHub Releases (primary) + Homebrew tap + Scoop manifest.
- Code signing: Azure Key Vault (Windows EV) + Apple Developer ID (macOS notarization).
- Config precedence: env vars > `~/.andromeda-pulse/config.toml` > built-in defaults.
- Development Style: agent-driven.

## Existing Scopes

- **`pulse-v0_2_0-route`** — Pulse v0.2.0 evolution scope. Defined at `docs/v0_2_0/pulse-v0_2_0-route.md`. Active scope drives Epoch 9 (Foundation v0.2.0) work: chunks #57 (widget real-data binding) → #58 (curation crate extraction) → #59 (connection state machine) → #60 (triage crate scaffold + attention cue contract types) → subsequent v0.2.0 chunks (#61+ streaming baseline trackers / #62 attention cue emitter / #63 restart event detector / etc.). Registered 2026-05-16 per chunk #60 substrate landing (session 74 wrap commit `589225f` — see §Architecture Registry Updates 2026-05-16). Supporting documents: `docs/v0_2_0/pulse-capability-spec.md` (capability spec P-001 through P-060+), `docs/v0_2_0/pulse-distillation-architecture.md` (L1-L4 layered pipeline), `docs/v0_2_0/pulse-vision-and-backlog.md` (product framing + backlog).

## Architecture Registry Updates

_This section accumulates entries from `/andromeda-evolve --allow-arch-registry` invocations that legitimize implementation reality in the registry sections of this document (typically §Occupied Resources). Entries are NOT specialist-plan Decisions Log entries — they record arch-level acknowledgments of code that landed via /andromeda-implement chunks before /andromeda-arch could update the canonical registry list. Entry format mirrors specialist-plan Decisions Log conventions (`### {YYYY-MM-DD} — {title}`). Cleanup convention: this section is preserved across /andromeda-arch re-runs as audit trail; never deleted._

### 2026-05-09 — Acknowledge `streams.*` namespace (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC routes.
**Added:**
- `streams.subscribe_spans` (`pulse-app/src/streams.rs:16`, chunk #23)
- `streams.subscribe_metrics` (`pulse-app/src/streams.rs:17`, chunk #23)
- `streams.subscribe_logs` (`pulse-app/src/streams.rs:18`, chunk #23)
**Rationale:** D3 stale-drift closure for chunk #23 TauRPC procedures (age 7 wraps in state.yaml.drift_warnings). Sibling amendment legitimizes telemetry.frontend.* simultaneously.
**Marker:** `.andromeda/runs/2026-05-09T11-45-00-spec-amendment-legitimize-streams-namespace/amendment.md`

### 2026-05-09 — Acknowledge `telemetry.frontend.*` namespace (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `telemetry.frontend.record_frame_ms` (`crates/ui-bridge/src/telemetry.rs:99`, chunk #29).
**Rationale:** D3 capability-drift closure for chunk #29 frontend telemetry resolver. Sibling amendment legitimizes streams.* simultaneously.
**Marker:** `.andromeda/runs/2026-05-09T11-45-00-spec-amendment-legitimize-telemetry-namespace/amendment.md`

### 2026-05-11 — Acknowledge `pulse:clipboard` capability (--allow-arch-registry)

**Section:** §Occupied Resources Tauri capability identifiers.
**Added:** `pulse:clipboard` (`pulse-app/capabilities/clipboard.json`, chunk #43 partial commit `6e2d398`).
**Rationale:** D3 capability-drift closure for chunk #43 clipboard-manager write-only capability (security plan §Anti-Patterns API row 6 — clipboard read excluded). Mirrors 2026-05-09 streams.* / telemetry.* precedent.
**Marker:** `.andromeda/runs/2026-05-11T00-15-00-spec-amendment-acknowledge-pulse-clipboard-capability/amendment.md`

### 2026-05-16 — Acknowledge `curation` crate (--allow-arch-registry)

**Section:** §Occupied Resources Cargo workspace crate names.
**Added:** `curation` (`crates/curation/Cargo.toml`, chunk #58 Epoch 9 Foundation v0.2.0).
**Rationale:** D3 capability-drift closure for chunk #58 workspace member (curation primitives extracted from snapshot). Mirrors 2026-05-11 pulse:clipboard precedent.
**Marker:** `.andromeda/runs/2026-05-16T16-15-00-spec-amendment-acknowledge-curation-crate/amendment.md`

### 2026-05-16 — Acknowledge `connection.current_state` + `pulse://stream/connection-state` (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events (broadcast channels).
**Added:**
- `connection.current_state` (`pulse-app/src/connection_router.rs:46`, chunk #59)
- `pulse://stream/connection-state` (`crates/ingest/src/connection.rs:25`, chunk #59)
**Rationale:** D3 capability-drift closure for chunk #59 connection FSM TauRPC + broadcast topic. Mirrors 2026-05-16 curation crate precedent.
**Marker:** `.andromeda/runs/2026-05-16T22-08-32-spec-amendment-acknowledge-connection-namespace/amendment.md`

### 2026-05-16 — Acknowledge `triage` crate + register `pulse-v0_2_0-route` scope (--allow-arch-registry)

**Section:** §Occupied Resources Cargo workspace crate names + §Existing Scopes.
**Added:**
- `triage` (`crates/triage/Cargo.toml`, chunk #60 commit `589225f`)
- `pulse-v0_2_0-route` (first registered scope; defined at `docs/v0_2_0/pulse-v0_2_0-route.md`)
**Rationale:** D3 capability-drift closure for chunk #60 workspace member + first scope registration (Check 7.4 first-entry case for §Existing Scopes). Mirrors 2026-05-16 curation crate precedent + extends with first-ever scope.
**Marker:** `.andromeda/runs/2026-05-16T23-39-39-spec-amendment-acknowledge-triage-crate-and-scope/amendment.md`

### 2026-05-17 — Acknowledge `pulse://stream/attention-cues` (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC events (broadcast channels).
**Added:** `pulse://stream/attention-cues` (`crates/triage/src/cue/broadcast.rs:8`, chunk #62 commit `aeb4d7d`).
**Rationale:** D3 capability-drift closure for chunk #62 cue emitter broadcast topic. Mirrors 2026-05-16 chunk #59 `connection-state` precedent.
**Marker:** `.andromeda/runs/2026-05-17T10-34-52-spec-amendment-acknowledge-attention-cues-broadcast/amendment.md`

### 2026-05-17 — Acknowledge `pulse://stream/restart-events` (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC events (broadcast channels).
**Added:** `pulse://stream/restart-events` (`crates/triage/src/pattern/broadcast.rs:8`, chunk #63 commit `61ca564`).
**Rationale:** D3 capability-drift closure for chunk #63 restart-event broadcast topic. Mirrors 2026-05-17 chunk #62 `attention-cues` precedent.
**Marker:** `.andromeda/runs/2026-05-17T14-15-00-spec-amendment-acknowledge-restart-events-broadcast/amendment.md`

### 2026-05-18 — Acknowledge `services.list_with_states` + `pulse://stream/service-lifecycle` (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events (broadcast channels).
**Added:**
- `services.list_with_states` (`pulse-app/src/services_router.rs:61`, chunk #67)
- `pulse://stream/service-lifecycle` (`crates/triage/src/lifecycle/broadcast.rs:16`, chunk #67)
**Rationale:** D3 capability-drift closure for chunk #67 service registry + lifecycle FSM TauRPC + broadcast topic. Mirrors 2026-05-16 chunk #59 `connection-state` precedent (single-coordinated dual TauRPC + broadcast amendment).
**Marker:** `.andromeda/runs/2026-05-18T16-53-11-spec-amendment-acknowledge-services-namespace/amendment.md`

### 2026-05-18 — Acknowledge `corpus` + `security` crates + `storage.{inspect,path}` TauRPC + `corpus/corpus.db` filesystem subpath (--allow-arch-registry)

**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Filesystem locations.
**Added:**
- `corpus` (`Cargo.toml:11`, `crates/corpus/src/lib.rs`, chunk #68)
- `security` (`Cargo.toml:12`, `crates/security/src/lib.rs`, chunk #68)
- `storage.inspect` (`pulse-app/src/storage_router.rs:50`, chunk #68)
- `storage.path` (`pulse-app/src/storage_router.rs:50`, chunk #68)
- `corpus/corpus.db` subpath under data dir root (`pulse-app/src/main.rs:353`, chunk #68)
**Rationale:** D3 capability-drift closure for chunk #68 persistent incident corpus + PII scrubber + storage router. Mirrors 2026-05-18 chunk #67 `services-namespace` precedent (single-coordinated multi-item Registry Update across sub-sections under §Occupied Resources).
**Marker:** `.andromeda/runs/2026-05-18T19-55-24-spec-amendment-acknowledge-chunk-68-corpus-additions/amendment.md`

### 2026-05-19 — Acknowledge `diagnostics.template_distribution` (--allow-arch-registry)

**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `diagnostics.template_distribution` (`pulse-app/src/diagnostics_router.rs:59`, chunk #69).
**Rationale:** D3 capability-drift closure for chunk #69 Drain template-profiling diagnostics TauRPC procedure. Mirrors 2026-05-17 chunk #62 `attention-cues` precedent (single-item Type 6).
**Marker:** `.andromeda/runs/2026-05-19T20-54-03-spec-amendment-acknowledge-diagnostics-namespace/amendment.md`

