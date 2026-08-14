# Phase 0 — Input Validation Dialogue

**User input:**

User pointed to `andromeda-pulse-product-description.md` in the project
folder (348 lines). Full text preserved verbatim in `input.md`. Summary:
universal local OpenTelemetry dashboard (Rust + Tauri 2 desktop app) with
always-visible quarter-screen glance widget, GPU-accelerated visualization
(WebGPU/WGSL), DuckDB columnar ring buffer, WASM Component Model plugin
system, optional MCP server sidecar, and a token-efficient curated
"Investigate" snapshot that copies an AI-ready prompt to clipboard.

**Clarifications (if any):**

- None. Input was complete and unambiguous — product type, research
  signal, scale intent, growth model, primary language, target users,
  surfaces, distribution model, and key technical components were all
  explicitly named in the description.

**Confirmed summary:**

- Type: Cross-platform desktop app (Tauri 2 + Rust)
- Core idea: Universal local OpenTelemetry dashboard with always-visible
  glance-widget surface and a one-click "Investigate" button that
  captures a token-efficient curated snapshot of recent telemetry,
  copies an AI-ready prompt to clipboard for one-paste debug sessions
  with Claude Code / Cursor / ChatGPT.
- Key aspects:
  - OTLP HTTP (:4318) + gRPC (:4317) ingest
  - DuckDB columnar ring buffer + Apache Arrow zero-copy ingest pipeline
  - WebGPU / WGSL compute + render shaders for GPU-accelerated trace /
    metric / flamegraph viz
  - Compact quarter-screen widget (primary surface) + full expanded
    dashboard + tray icon
  - "Investigate" → curated markdown snapshot (dedupe / anomaly highlight /
    critical-path / p50-p95-p99 / token budget) — not raw OTLP dump
  - WASM Component Model plugin system (custom dashboards / data
    transforms / snapshot template engines)
  - Optional rmcp MCP server sidecar so AI agents can query telemetry
    directly
  - Self-observation edge case — the app IS the local observer, so its
    own telemetry exports to stdout, not OTLP
  - Modular monolith — 8 Rust modules
  - Public OSS, MIT, GitHub Releases distribution

**Slug:** andromeda-pulse

---

# Phase 3 — Quiz I Dialogue

**Pre-filled confirmations** (all 12 fields presented as a single batch
because every field's value was inferable from `input.md`):

Core fields:
- Scale Intent: startup — confirmed
- Platform: Tauri 2 desktop (native window + webview) — confirmed
- Primary Language: Rust — confirmed
- Target Users: Public OSS — developers using AI coding assistants — confirmed
- Growth Model: Modular monolith (with WASM plugin extension layer) — confirmed
- Development Style: agent-driven — confirmed
- Core Functionality: Universal local OTel desktop dashboard with widget
  + dashboard surfaces, AI-debug curated snapshot, optional MCP server — confirmed

Research-derived fields (template's recommendation accepted on each):
- OTLP Ingest Wire Format Coverage: Classic OTLP for v1, OTAP behind a
  feature flag — confirmed (the orchestrator surfaced this one explicitly
  because the input did not pre-bind it; user accepted)
- Telemetry Retention Surface: In-memory DuckDB ring buffer only
  (5–10 min, configurable) — confirmed
- AI-Agent Integration Surface: Clipboard + optional MCP server (default off) — confirmed
- Snapshot Curation Aggressiveness Default: Balanced (25k tokens, full
  dedupe + critical-path + p50/p95/p99 + anomaly) — confirmed
- Plugin Distribution Channel for v1: Built-in templates + filesystem
  loading from `~/.andromeda-pulse/plugins/` — confirmed

**Core field answers (no research):** all pre-filled — see batch above.

**Research-derived field answers:** all pre-filled — see batch above.
No `dig` requests. No custom answers. No overrides.

**Final summary confirmed:** yes — user replied "correct" to the
single-card pre-fill confirmation.

---

# Phase 6 — Quiz II Dialogue

**Pre-filled confirmations (6 fields, batch-confirmed in single card):**
- #1  Primary Language & Runtime: Rust 2024 edition (rustc 1.84+) — confirmed
- #3  Storage Engine: DuckDB embedded (`duckdb` 1.10500.x, DuckDB 1.5.x bundled) — confirmed
- #5  Plugin Runtime: WASM Component Model via `wasmtime` 25+ with WIT — confirmed
- #9  MCP Server Surface: `rmcp` over stdio with `#[tool]` methods, gated by `--features mcp-server` (default off) — confirmed
- #10 OTLP Wire Format: Classic OTLP for v1; OTAP behind feature flag — confirmed
- #12 Module Boundary Enforcement: Cargo workspace, one crate per module + `pulse-app/` binary; cargo-deny rules deferred — confirmed

**Fork decisions (7 fields, presented as a single research-grounded batch
card; user replied "accept all"):**

- Q: #2 OTLP Receiver Stack
  Stakes: shapes the ingest module's HTTP+gRPC server substrate
  Research basis: research-targeted.md § Backend Framework Comparison
    (tonic 0.14.x, axum 0.8.x, hyper 1.x, opentelemetry-otlp 0.31)
  Options: ① tonic+raw-hyper / ② tonic+axum (★) / ③ poem or actix-web + tonic
  Recommended: ② tonic+axum
  Reasoning: 8-module workspace + agent-driven harness + "approach Jaeger UI quality"
    favor middleware/extractors out of the box; axum's hyper 1.x substrate is shared
    with Tauri's tokio runtime; bundle delta small relative to bundled DuckDB.
  Caveat: must reconcile tonic 0.14 with opentelemetry-otlp 0.31's tonic 0.13 pin.
  Downstream forks: none (independent of later forks for this project).
  User response: accepted (batch).
  Final answer: tonic 0.14.x + axum 0.8.x on shared hyper 1.x + tower.

- Q: #4 In-Process Channel Architecture
  Stakes: how span/metric data flows from ingest to N subscribers
  Research basis: research-targeted.md § Message Queue / Event System
    (tokio mpsc, tokio broadcast, crossbeam-channel)
  Options: ① mpsc-only with viz multiplexing / ② mpsc + broadcast (★) / ③ mixed tokio + crossbeam
  Recommended: ② mpsc (ingest→appender) + broadcast (buffer→subscribers)
  Reasoning: 4 distinct subscribers (compact widget, full dashboard, tray icon, MCP)
    all need the same live span stream — broadcast is canonical tokio fan-out;
    mpsc gives backpressure so a slow subscriber can't stall ingest.
  Downstream forks: none.
  User response: accepted (batch).
  Final answer: tokio::sync::mpsc + tokio::sync::broadcast.

- Q: #6 WebGPU Visualization Surface
  Stakes: how (and where) WGSL shaders run; jank-vs-portability trade-off
  Research basis: research-targeted.md § Key Trade-offs Summary item 3
    (Webview WebGPU on WebView2/WKWebView, native wgpu 25+ overlay,
     tauri#9220 flicker bug, wgpu overlay discussion #11944)
  Options: ① Webview WebGPU (★) / ② native wgpu overlay / ③ hybrid native+webview
  Recommended: ① Webview WebGPU (`<canvas>` + `navigator.gpu`)
  Reasoning: ships v1 on three OSes from one WGSL codebase with no flicker bug
    to chase; existing OTel viewers (otel-desktop-viewer, Jaeger UI) all run in
    webview and meet the "approach Jaeger UI quality" bar; native wgpu reserved
    as explicit upgrade trigger if widget GC jank is measured.
  Downstream forks: #4 (channels stay pure tokio under webview path; native wgpu
    would have required crossbeam sync bridge), and design specialist's WGSL
    tooling decision. → KEYSTONE (≥2 downstream).
  User response: accepted (batch).
  Final answer: Webview WebGPU + documented upgrade path to native wgpu for the
    widget if measured GC jank emerges.

- Q: #7 Deployment / Release Pipeline
  Stakes: how cross-OS bundles ship + sign + auto-update
  Research basis: research-targeted.md § Deployment & Infrastructure
    (tauri-action, Tauri 2 native bundlers, Azure Key Vault, Apple Developer ID)
  Options: ① tauri-action + Tauri bundlers (★) / ② hand-rolled GH Actions matrix /
    ③ cross-rs + cargo-bundle/cargo-wix/cargo-deb
  Recommended: ① tauri-action + Tauri 2 native bundlers
  Sub-decision (CI task runner): cargo-xtask (vs just / cargo-make)
  Reasoning: matches the chosen framework, ships the updater contract for free,
    well-trodden signing/notarization path; xtask keeps release/sign/notarize as
    Rust binaries the agent harness already understands.
  Downstream forks: none.
  User response: accepted (batch).
  Final answer: tauri-action + Tauri 2 native bundlers + cargo-xtask + Homebrew/Scoop
    side-jobs in the same workflow.

- Q: #8 Tauri IPC Bridge (Rust ↔ webview)
  Stakes: type contract drift across 8 modules × ~30 commands
  Research basis: research-targeted.md § API Style & Conventions surface 2
    (stock #[tauri::command], TauRPC, tauri-specta)
  Options: ① stock + manual TS contracts / ② TauRPC (★) / ③ Specta + tauri-specta
  Recommended: ② TauRPC (`taurpc` crate)
  Reasoning: 8 modules × ~30 commands creates real type-drift cost without
    auto-typegen; TauRPC's router-style maps to per-module command groups
    (one router per module → predictable surface).
  Downstream forks: #13 (errors must serialize across the bridge; AppError
    wrapper threads that), and design specialist's frontend type-bridge
    decision. → KEYSTONE (≥2 downstream).
  User response: accepted (batch).
  Final answer: TauRPC (taurpc crate); design specialist may revisit when
    frontend stack lands.

- Q: #11 Validation Library
  Stakes: how to validate user settings + plugin manifests
  Research basis: research-targeted.md § Validation Library
    (garde 0.20+, validator 0.18+, serde_valid 0.20+, serde-only baseline)
  Options: ① none — serde + smart enums (★) / ② garde / ③ validator / ④ serde_valid
  Recommended: ① no library — serde + smart enum types + `TryFrom<u16>` impls
  Reasoning: validation surface is tiny (enum settings, port ranges, file paths);
    OTLP protobuf payloads are validated by prost/tonic decode itself; reaching
    for a library is premature until plugin manifest grows.
  Downstream forks: none.
  User response: accepted (batch).
  Final answer: no library; documented upgrade trigger to garde when plugin
    manifest complexity demands it.

- Q: #13 Error Handling Pattern
  Stakes: error contract across 8 modules + IPC + plugin loading
  Research basis: research-targeted.md § API Style & Conventions error handling
    + § Key Trade-offs (thiserror 2.x, anyhow 1.x, miette 7.x, tauri Serialize)
  Options: ① thiserror + anyhow + AppError (★) / ② ① + miette / ③ anyhow only
  Recommended: ① thiserror 2.x (module enums) + anyhow 1.x (boundary/main) +
    serde-friendly AppError wrapper for IPC
  Reasoning: matches the 8-module workspace boundary model; #8 (TauRPC) needs
    Serialize-able errors — AppError wrapper threads that; ② is a probable
    upgrade once plugin-loading DX surfaces miette-worthy failures.
  Downstream forks: none (acts as a sink, not a source).
  User response: accepted (batch).
  Final answer: thiserror 2.x + anyhow 1.x + AppError wrapper; miette 7.x
    documented as upgrade trigger for plugin-loading diagnostics.

**Final summary confirmed:** yes — user replied "accept all" to the single-card
batch presentation. 7 forks accepted on first pass; 0 overrides; 0 dig requests;
0 custom answers. Adherence: 13/13.
