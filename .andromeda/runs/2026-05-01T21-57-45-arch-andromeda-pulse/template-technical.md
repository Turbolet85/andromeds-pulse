## Quiz II Fields

### 1. Primary Language & Runtime [maps to: Stack and Technologies, Inherited Defaults]
Pre-filled from Quiz I. Listed for completeness only — no user input required.

- **Rust 2024 edition (rustc 1.84+)** [inferred from Quiz I — Primary Language: Rust] — desktop app via Tauri 2 mandates Rust on the host side; webview frontend language is a separate downstream design-specialist decision.

### 2. OTLP Receiver Stack [maps to: Stack and Technologies, Established Decisions, Standard Contracts]

- **Option A — `tonic` 0.14.x + `axum` 0.8.x on shared `hyper` 1.x + `tower`** — single Tokio runtime serves `:4317` gRPC and `:4318` HTTP; ergonomic extractors and middleware; ~30 transitive deps and a few hundred KB binary cost; community gravity an order of magnitude greater than alternatives.
- **Option B — `tonic` 0.14.x + raw `hyper` 1.x (no Axum)** — leaner bundle, faster cold start, three hand-written POST handlers (`/v1/traces`, `/v1/metrics`, `/v1/logs`); right when distribution binary size dominates.
- **Option C — `poem` 3.x or `actix-web` 4.x + tonic** — two runtime substrates inside Tauri; wasted memory; deal-breaker for actix's actor system fighting `tokio::main` Tauri integration.

Caveat: must reconcile `tonic` 0.14 with `opentelemetry-otlp` 0.31's tonic 0.13 pin before locking versions.

### 3. Storage Engine [maps to: Stack and Technologies, Established Decisions — Data Persistence, Inherited Defaults]

- **Option A — DuckDB embedded via `duckdb` crate 1.10500.x (DuckDB 1.5.x bundled)** [inferred from Quiz I — Telemetry Retention Surface: in-memory DuckDB ring buffer] — columnar OLAP, vectorized exec, first-class Arrow zero-copy via `Appender::append_record_batch()` / `stream_arrow()`; one-writer model needs care under concurrent ring-buffer eviction.
- **Option B — DataFusion 43+ (pure Rust, Apache top-level)** — same SQL+Arrow surface without C++ FFI; smaller cross-compile surface; only worth raising if DuckDB cross-compile becomes a blocker.
- **Option C — Polars 0.43+** — fast lazy DataFrames but no SQL surface; conflicts with v1 plugin "custom SQL filter" requirement.

### 4. In-Process Channel Architecture [maps to: Standard Contracts, Cross-cutting Patterns]

- **Option A — `tokio::sync::mpsc` for ingest→appender + `tokio::sync::broadcast` for buffer→subscribers (widget, main window, MCP)** — built-in backpressure on mpsc; broadcast fans out the same span stream to N consumers without re-querying DuckDB.
- **Option B — `tokio::sync::mpsc` only, with subscriber multiplexing in `viz`** — fewer primitives; `viz` becomes a fan-out hub, adds complexity to viz module.
- **Option C — Mixed `tokio` + `crossbeam-channel` for sync render-thread bridge** — needed only if wgpu render thread cannot run on a tokio task; introduces async/sync bridging complexity.

### 5. Plugin Runtime [maps to: Stack and Technologies, Established Decisions, Standard Contracts]

- **Option A — WASM Component Model via `wasmtime` 25+ with WIT interfaces** [inferred from Quiz I — Growth Model: modular monolith with WASM plugin extension layer] — standards path, type-safe host imports, capability-based sandboxing; Preview 2 → Preview 3 churn means SDK migration over the next 12 months.
- **Option B — Extism PDK on top of wasmtime** — higher-level "plugin developer kit" with Go/Rust/Zig/JS guests out of the box; lower author friction; less standards-aligned, weaker type contracts.
- **Option C — Defer plugin runtime; load only built-in templates in v1** — matches Quiz I's "filesystem loading from `~/.andromeda-pulse/plugins/`"; enables locking the plugin host crate later.

### 6. WebGPU Visualization Surface [maps to: Stack and Technologies, Established Decisions, Cross-cutting Patterns]

- **Option A — Webview WebGPU (`<canvas>` + `navigator.gpu`)** — WGSL runs unchanged across WebView2 (Windows) / WKWebView (macOS); IPC pushes Arrow data into canvas; simpler three-OS shipping; subject to browser GC pauses.
- **Option B — Native `wgpu` 25+ surface overlaid on Tauri window** — faster, no GC jank at high cardinality; introduces flicker bug under Tauri 2 webview compositing (tauri#9220, discussion #11944).
- **Option C — Hybrid: native wgpu for the always-visible quarter-screen widget, webview WebGPU for the expanded view** — optimizes for the "always visible, no jank" path while keeping the dense expanded UI in HTML.

### 7. Deployment / Release Pipeline [maps to: Stack and Technologies, Infrastructure Patterns]

- **Option A — `tauri-action` GitHub Action + Tauri 2 native bundlers** — one workflow produces `.msi`/`.app`/`.dmg`/`.AppImage`/`.deb`; auto-uploads `latest.json` for the Tauri updater plugin; integrates Azure Key Vault (Windows EV) and Apple Developer ID (macOS notarization).
- **Option B — Hand-rolled GitHub Actions matrix calling `cargo tauri build`** — more control, more boilerplate; only worth it for custom WiX fragments or `extension-template-rs` integration.
- **Option C — `cross-rs` + `cargo-bundle`/`cargo-wix`/`cargo-deb` (no Tauri bundler)** — wrong fit; loses Tauri updater integration that `latest.json` requires.

Sub-decision (CI task runner):
- **`cargo-xtask`** — Rust-only repo, scripts compile in same workspace.
- **`just`** — fine if non-Rust contributors run scripts.
- **`cargo-make`** — DSL surface area without a clear win for one app.

### 8. API Conventions — Tauri Rust ↔ Webview Bridge [maps to: Conventions, Standard Contracts]

- **Option A — Plain `#[tauri::command]` + `app.emit_to(...)` + manually maintained TypeScript contracts** — zero new deps; type drift risk grows with command count (8 modules × ~30 commands).
- **Option B — TauRPC (`taurpc` crate)** — derive macro auto-generates fully-typed TypeScript bindings from Rust command surface; router-style; opinionated.
- **Option C — Specta + `tauri-specta`** — composable per-command typegen; commonly paired with TanStack Query.

### 9. API Conventions — MCP Server Surface [maps to: Conventions, Standard Contracts, Occupied Resources]

- **Option A — `rmcp` (official MCP Rust SDK) over stdio with `#[tool]`-annotated methods** [inferred from Quiz I — AI-Agent Integration Surface: rmcp stdio sidecar] — `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot` map cleanly to tool methods; verify input.md "1.5.0" claim against published 0.3.x line before locking.
- **Option B — Custom JSON-RPC 2.0 over stdio (DIY)** — only if rmcp version situation cannot be resolved; reimplements capabilities/prompts/resources/tools/sampling spec.
- **Option C — Defer MCP entirely behind a `--features mcp-server` Cargo flag (clipboard-only build by default)** — matches Quiz I "default off"; treats MCP as compile-time opt-in rather than runtime toggle.

### 10. OTLP Wire Format Coverage [maps to: Established Decisions, Standard Contracts]

Pre-filled from Quiz I. No user input required.

- **Classic OTLP for v1, OTAP behind a feature flag** [inferred from Quiz I — OTLP Ingest Wire Format Coverage].

### 11. Validation Library [maps to: Established Decisions — Validation Library, Conventions]

- **Option A — None: `serde` + smart enum types + `TryFrom<u16>` impls** — validation surface is tiny (enum settings, port ranges, file paths); leanest path; reach for a library only when plugin manifest grows.
- **Option B — `garde` 0.20+** — modern derive macro, async support; pairs cleanly with serde-deserialized config; growing community.
- **Option C — `validator` 0.18+** — older, larger ecosystem footprint; less ergonomic API than garde.
- **Option D — `serde_valid` 0.20+** — JSON-Schema-based; only interesting if plugin manifests are spec'd as JSON Schema.

Note: OTLP protobuf payload validation is enforced by `prost`/`tonic` decode itself — not in scope for this field.

### 12. Module Boundary Enforcement [maps to: Established Decisions — Module Boundaries, Infrastructure Patterns, Conventions]

- **Option A — Cargo workspace, one library crate per module + `pulse-app/` binary** [inferred from Quiz I — 8 Rust modules: ingest / buffer / viz / ui-bridge / snapshot / workspace-detector / plugins / mcp-server] — boundaries enforced by `Cargo.toml` `[dependencies]`; compile-time visibility via `pub` / `pub(crate)`; parallel incremental rebuild.
- **Option B — Workspace + `cargo-deny` 0.16+ `bans` rules from day 1** — adds CI lint banning e.g. "no crate may depend on `wgpu` except `viz`"; layered on top of A.
- **Option C — Single crate with `mod ingest;` + `pub(crate)` discipline** — faster to start, slower to evolve; review-enforced rather than compile-enforced; unsuitable for 8 modules.

### 13. Error Handling Pattern [maps to: Established Decisions — Error Handling, Conventions]

- **Option A — `thiserror` 2.x for module-internal error enums + `anyhow` 1.x at Tauri command boundary and `main.rs` + `serde`-friendly `AppError` wrapper for IPC** — Rust 2026 community consensus; explicit module errors, ergonomic top-level error handling, lossy-but-explicit IPC contract.
- **Option B — Pattern A + `miette` 7.x diagnostics for plugin-loading errors** — pretty source-span diagnostics where users see WIT resolution errors; overkill for ingest/buffer paths.
- **Option C — `anyhow` everywhere (no `thiserror`)** — fastest to write; loses typed error contracts at module boundaries; harder to test failure paths.

## Section Coverage Map

| scope-arch Section | Covered by |
|---|---|
| Design Philosophy | Derived from Quiz I (scale: startup, growth: modular monolith with WASM plugins, agent-driven) + Quiz II (#5 plugin runtime, #6 WebGPU, #12 module boundaries, #13 error handling) |
| Stack and Technologies | Quiz II fields: #1 language/runtime, #2 OTLP receiver stack, #3 storage engine, #5 plugin runtime, #6 WebGPU surface, #7 deployment, #9 MCP surface |
| Established Decisions | Every Quiz II answer becomes a decision — fields #2, #3, #4, #5, #6, #7, #8, #9, #10, #11, #12, #13 |
| Conventions | Quiz II fields: #8 Tauri IPC bridge, #9 MCP surface, #11 validation, #12 module boundaries, #13 error handling |
| Standard Contracts | Quiz II fields: #2 OTLP receiver protocol surfaces, #4 in-process channel architecture, #5 plugin WIT interfaces, #8 Tauri command/event bridge, #9 MCP tool surface, #10 OTLP wire format coverage |
| Occupied Resources | Quiz II fields: #2 (ports `:4317` gRPC + `:4318` HTTP), #9 (MCP stdio sidecar process namespace), #5 (plugin filesystem path `~/.andromeda-pulse/plugins/` from Quiz I), #12 (8 reserved crate names) |
| Infrastructure Patterns | Quiz II fields: #7 deployment & CI task runner, #12 cargo workspace structure, #5 plugin distribution model (Quiz I) |
| Cross-cutting Patterns | Quiz II fields: #4 in-process channel architecture, #6 WebGPU surface placement, #11 config validation approach + Quiz I development style (agent-driven) |
| Project Intent | [covered by Quiz I] — modular monolith desktop app, OSS, growth via WASM plugin extension |
| Inherited Defaults | Derived from Quiz II answers: #1 Rust + Tauri 2, #2 OTLP receiver stack, #3 DuckDB, #7 release pipeline, #8 Tauri IPC convention, #11 validation default, #13 error handling default |
| Existing Scopes | Always "None" — new project |

## Pre-filled Values

- **Primary Language: Rust 2024 edition (rustc 1.84+)** [inferred from Quiz I — Primary Language]
- **Platform Host: Tauri 2 desktop (Windows / macOS / Linux), webview frontend** [inferred from Quiz I — Platform]
- **Storage Engine: DuckDB embedded (in-memory ring buffer, 5–10 min configurable)** [inferred from Quiz I — Telemetry Retention Surface]
- **OTLP Wire Format: Classic OTLP for v1; OTAP behind feature flag** [inferred from Quiz I — OTLP Ingest Wire Format Coverage]
- **MCP Surface: rmcp stdio sidecar, default off** [inferred from Quiz I — AI-Agent Integration Surface]
- **Plugin Distribution Channel: Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/`; signed-plugin verification deferred post-v1** [inferred from Quiz I — Plugin Distribution Channel]
- **Module Set: 8 crates — ingest / buffer / viz / ui-bridge / snapshot / workspace-detector / plugins / mcp-server** [inferred from Quiz I — Growth Model]
- **Snapshot Curation Default: Balanced — 25k tokens, full dedupe + critical-path + p50/p95/p99 + anomaly highlighting** [inferred from Quiz I — Snapshot Curation Aggressiveness Default]
- **Development Style: agent-driven** [inferred from Quiz I — Development Style]
- **Target Users: Public OSS, developers using AI coding assistants** [inferred from Quiz I — Target Users]
- **No external infrastructure: no Kafka/NATS/Redis, no FCM/APNS/web push, no server hosting** [inferred from Quiz I — single-machine local desktop OSS]
- **Mobile framework: N/A** [inferred from Quiz I — Platform: Tauri 2 desktop only]
- **Frontend framework: deferred to design specialist**
- **CSS / component library / visual tokens / typography: deferred to design specialist**
- **Auth library: deferred to security specialist** (local-only desktop, no external auth surface in v1)
- **Test framework: deferred to tests specialist**
- **Logging / observability stack: deferred to obs specialist** (note: this app *receives* OTLP from other apps; its own internal logging is a separate obs-specialist decision)
- **Accessibility requirements: deferred to a11y specialist**

## Quiz II Scope Note

Quiz II collects ARCHITECTURAL decisions: how to build it at arch level. Each answer becomes an Established Decision in the architecture document. Product decisions (what, for whom) were settled in Quiz I.

Specialist-domain decisions (auth library, test framework, logging/observability, frontend framework / CSS tools / component libraries / visual tokens / typography, a11y) are NOT in Quiz II — they're collected by specialist skills (security / tests / obs / design / a11y) downstream.
