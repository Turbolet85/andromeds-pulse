# Quiz II — Technical Layer Results

## Stack Decisions
- Primary Language & Runtime: Rust 2024 edition (rustc 1.84+)
- OTLP Receiver Stack: `tonic` 0.14.x + `axum` 0.8.x on shared `hyper` 1.x + `tower` (single Tokio runtime serves `:4317` gRPC and `:4318` HTTP)
  - Caveat to record in architecture: reconcile `tonic` 0.14 with `opentelemetry-otlp` 0.31's `tonic` 0.13 pin before locking versions
- Storage Engine: DuckDB embedded via `duckdb` crate 1.10500.x (DuckDB 1.5.x bundled), in-memory ring buffer (5–10 min, configurable); first-class Apache Arrow zero-copy via `Appender::append_record_batch()` / `stream_arrow()`
- In-Process Channel Architecture: `tokio::sync::mpsc` for ingest→appender hand-off (built-in backpressure) + `tokio::sync::broadcast` for buffer→subscribers fan-out (compact widget, full dashboard, tray icon, MCP server)
- Plugin Runtime: WASM Component Model via `wasmtime` 25+ with WIT interfaces (capability-based sandboxing; type-safe host imports)
- WebGPU Visualization Surface: Webview WebGPU (`<canvas>` + `navigator.gpu`); WGSL runs unchanged across WebView2 (Windows) and WKWebView (macOS); IPC pushes Arrow data into canvas
  - Documented upgrade path: native `wgpu` 25+ surface for the compact widget if browser GC jank becomes a measured problem (hybrid Option C)
- MCP Server Surface: `rmcp` (official Model Context Protocol Rust SDK) over stdio with `#[tool]`-annotated methods; `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` map to tool methods; gated by `--features mcp-server` (default off)
  - Caveat to record: verify `rmcp` "1.5.0" claim from input.md against currently published 0.3.x line before locking (input may be forward-looking or referring to a different fork)
- Mobile Framework: N/A — Tauri 2 desktop only (Windows / macOS / Linux)
- Message Broker: N/A — single-process desktop app, no external infrastructure
- AI/ML Serving: N/A — no in-process model; product emits curated markdown for external AI clients
- Push Notifications: OS-native via `tauri-plugin-notification` 2.x (no external push service)

## Infrastructure Decisions
- Deployment / Release Pipeline: `tauri-action` GitHub Action + Tauri 2 native bundlers (single workflow → `.msi` / `.app` / `.dmg` / `.AppImage` / `.deb`); auto-uploads `latest.json` for the Tauri updater plugin; Azure Key Vault for Windows EV signing; Apple Developer ID for macOS notarization
- CI Task Runner: `cargo-xtask` (Rust binaries in the same workspace; matches agent-driven harness ergonomics)
- Distribution Channels: GitHub Releases (primary); Homebrew tap and Scoop manifest jobs in the same workflow

## Convention Decisions
- OTLP Wire Format Coverage: Classic OTLP for v1 (protobuf over HTTP `:4318` + gRPC `:4317`); OTAP (Arrow-encoded) ingest behind a feature flag, activated when SDK clients catch up
- API Style — Tauri IPC Bridge (Rust ↔ webview): TauRPC (`taurpc` crate) — derive macro auto-generates fully-typed TypeScript bindings from the Rust command surface; router-style; one router per module crate
- API Style — MCP Server: JSON-RPC 2.0 over stdio per MCP spec; `rmcp` `#[tool]` macro is the canonical surface
- Error Handling Pattern: `thiserror` 2.x for module-internal error enums + `anyhow` 1.x at the Tauri command boundary and `main.rs` + a `serde`-friendly `AppError` enum wrapper for IPC (lossy-but-explicit cross-bridge contract)
  - Documented upgrade path: add `miette` 7.x for plugin-loading diagnostics if WIT resolution UX surfaces source-span-worthy failures
- Validation Library: None — `serde` + smart enum types + `TryFrom<u16>` impls (validation surface is tiny: enum settings, port ranges, file paths; OTLP protobuf payloads are validated by `prost`/`tonic` decode itself)
  - Documented upgrade path: reach for `garde` 0.20+ if the plugin manifest grows complex
- Module Boundary Enforcement: Cargo workspace, one library crate per module (`ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `workspace-detector`, `plugins`, `mcp-server`) + `pulse-app/` binary that wires them; boundaries enforced by `Cargo.toml` `[dependencies]` graph + compile-time `pub` / `pub(crate)` visibility
  - Deferred: `cargo-deny` 0.16+ `bans` rules added when a boundary is actually breached, not from day 1
- Snapshot Curation Default: Balanced — 25k tokens, full dedupe + critical-path + p50/p95/p99 + anomaly highlighting (inherited from Quiz I)
- Plugin Distribution Channel: Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/` (signed-plugin verification deferred post-v1; no marketplace UI in v1) (inherited from Quiz I)

## Recommendation Adherence
All recommendations accepted on first pass.
- 7 open forks: 7/7 accepted the template's strongest option (★) — no overrides, no `dig` requests, no custom answers
- 6 confirmation-only fields (inherited from Quiz I): 6/6 confirmed in batch
- Total Quiz II adherence: 13/13 accepted on first pass

## Defaults Applied
None — every fork was explicitly grounded in `research-targeted.md` (Phase 4) or pre-bound by `quiz-product.md` (Quiz I). No `[default — from recommendation]` tags.

## Keystone Decisions Recorded
Two forks were flagged as keystone (their choice cascades to ≥2 later decisions or downstream specialist plans):
- **#6 WebGPU Visualization Surface (Webview WebGPU)** — affects #4 (channel architecture stays pure tokio; native wgpu would have required a crossbeam bridge) and the design specialist's WGSL tooling decision.
- **#8 Tauri IPC Bridge (TauRPC)** — affects #13 (errors must be `Serialize`-able across the bridge; `AppError` wrapper threads that) and the design specialist's frontend type-bridge decision.

These are surfaced explicitly here so future revisions understand the cascade scope before changing them.
