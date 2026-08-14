## Quiz I Fields

### 1. Scale Intent [FIRST]
Options: personal / startup / production
- **personal**: just you, local machine, no public release, lowest overhead
- **startup**: shipped public OSS, external developers use it, basic release engineering, signed binaries on macOS, public issue tracker
- **production**: enterprise/team APM scale, SLAs, compliance, multi-tenant ingest, formal support — overkill for a local-dev viewer

### 2. Platform
Options refined from the local-OTel-viewer landscape (otel-desktop-viewer, otel-tui, otel-front, Aspire Dashboard, Teley):
- **Tauri 2 desktop (native window + webview)** [inferred]: small bundle (~10–20 MB), Rust backend, capability-scoped permissions, mobile targets available; matches the always-visible widget surface that none of the existing tools ship
- **Browser-served local UI (otel-desktop-viewer pattern)**: binary launches localhost server + opens tab; simpler shell but no real "always-visible quarter-screen widget" surface and no GPU-app polish bar
- **Terminal UI (otel-tui pattern)**: keyboard-driven, lowest overhead; rules out the GPU-accelerated visualization differentiator
- **Hybrid (desktop GUI + headless MCP sidecar)**: same as Tauri but explicitly exposes a separate AI-agent surface; more interfaces to maintain

### 3. Primary Language Preference
Options grounded in the receiver-language landscape (Go dominates collector ecosystem; Rust is the 2025–2026 high-performance frontier — Rotel, OTAP Phase 2, GreptimeDB):
- **Rust** [inferred]: required for Tauri 2 backend, `wgpu`/WGSL, `wasmtime` plugins, OTAP/Arrow Rust crates, `rmcp`; matches every edge-tech item in the input
- **Go**: the OTel Collector lingua franca (otel-desktop-viewer, otel-tui), strongest receiver ecosystem, but no Tauri path and weaker WebGPU/WASM-host story
- **TypeScript/Node**: viable for browser-served UI shells, but loses the receiver-throughput and GPU-compute advantages the bar requires
- **Mixed (Rust core + TS frontend inside webview)**: standard Tauri pattern; frontend framework choice deferred to design specialist

### 4. Target Users
- **Public OSS — devs using AI coding assistants** [inferred]: the stated audience; drives MIT licensing, GitHub Releases distribution, zero-config install, "one-paste into Claude/Cursor/ChatGPT" snapshot ergonomics
- **Internal team / single-org**: simpler distribution but loses the OSS portfolio-showcase positioning
- **Enterprise APM customers**: drags in compliance, SSO, multi-tenant — orthogonal to the local-dev niche the input targets

### 5. Growth Model
Options informed by how local OTel viewers and the OTel Collector itself are organized:
- **Modular monolith** [inferred]: 8 named Rust modules already in input (`ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `workspace-detector`, `plugins`, `mcp-server`); matches otel-desktop-viewer / OTel Collector core / Rotel
- **Plugin/extension system layered on top**: WASM Component Model surface for custom dashboards / data transforms / snapshot templates; coexists with modular monolith rather than replacing it
- **Plain monolith**: simpler but loses the plugin extension story that's part of the v2 differentiation
- **Microservice fleet**: anti-pattern for a single-binary local-dev tool; explicitly the friction the niche removes

### 6. Development Style
Options: classic / agent-driven
- **classic**: humans write code, run tests by hand, no harness scaffolding; standard recipes downstream
- **agent-driven** [inferred]: structured agent pipeline + verification harness (`scripts/agent-run.*`, structured logs, status endpoint, PID file); cascades to `tests` and `obs` specialist plans; input states "built via Andromeda v2 pipeline — recursive dogfood"

### 7. Core Functionality
[inferred] — Universal local OpenTelemetry desktop dashboard that ingests OTLP HTTP+gRPC from any local app, visualizes traces/metrics/logs via GPU-accelerated charts in a quarter-screen always-visible widget plus an expanded view, and produces token-efficient curated AI-debug snapshots (markdown for LLM ingestion) on one click, with optional MCP server for direct AI-agent queries.

### 8. OTLP Ingest Wire Format Coverage
Research found OTAP Phase 2 (Arrow-encoded OTLP) shipped in 2025 with 30–70% network savings, but production SDKs in the wild still emit standard OTLP — the OTAP path is mostly server-side and emerging. This decision shapes the `ingest` module surface.
- **Classic OTLP only (protobuf over HTTP/4318 + gRPC/4317)**: matches every SDK shipping in 2025–2026; lowest implementation cost; what otel-desktop-viewer / otel-tui / otel-front do today
- **Classic OTLP + OTAP (Arrow) ingest from day one**: positions the product on the 2025–2026 performance frontier (uses `otel-arrow` Rust crates); higher implementation surface and few SDK clients exist yet
- **Classic OTLP for v1, OTAP behind a feature flag**: ships the working ingest path now; treats OTAP as an optional path activated when SDKs catch up

### 9. Telemetry Retention Surface
Research shows every credible local viewer uses a finite ring buffer (5–30 min) backed by DuckDB / Arrow / Vec; persistent disk storage is reserved for the heavier SigNoz/Uptrace tier. Input lists persistent storage as out-of-scope for v1, but the storage abstraction shape is a Quiz I product decision.
- **In-memory DuckDB ring buffer only (5–10 min, configurable)** [inferred from input "Out of v1 scope: Persistent storage"]: matches otel-front, fits the local-dev iteration loop, lowest disk footprint
- **In-memory ring buffer + opt-in disk persistence (longer windows)**: keeps v1 simple but reserves a slot for a settings toggle later; aligns with the "deferred to v2" note already in input
- **DuckDB-backed disk store with rolling retention from day one**: heavier; closer to the SigNoz/Uptrace tier the input explicitly says it does NOT compete with

### 10. AI-Agent Integration Surface
Research established MCP-for-observability as a real 2025–2026 pattern (Datadog, Grafana, Honeycomb, OpenObserve, OneUptime, IBM Instana all shipped MCP servers; `rmcp` is the canonical Rust SDK). The product's core differentiator is AI-debug workflow, so this surface is product-shaping.
- **Clipboard snapshot only (curated markdown to clipboard, no MCP)**: zero new attack surface; relies on user paste; matches the Investigate-button core flow
- **Clipboard + optional MCP server (default off)** [inferred from input "MCP server toggle (off / on; off by default)"]: covers both AI tools that ingest pasted prompts (ChatGPT) and MCP-aware agents (Claude Code, Cursor); rmcp stdio sidecar; eliminates copy-paste for MCP-aware tools
- **MCP-first (server always on, snapshot is a fallback)**: optimizes for agent-native workflows but exposes a query surface the user might not want by default
- **Clipboard + MCP + remote share link (URL to hosted snapshot)**: broadest reach but introduces a network surface and storage contract the input does not request

### 11. Snapshot Curation Aggressiveness Default
Research called the curation pipeline "the actual product differentiator and the riskiest module." The default preset shapes how the feature lands for first-time users — too aggressive loses fidelity, too light blows the token budget.
- **Conservative default (25k tokens, light dedupe, anomaly highlight only)**: less risk of dropping context the user actually wanted; safer first impression
- **Balanced default (25k tokens, full dedupe + critical-path + p50/p95/p99 + anomaly)** [inferred from input default 25k bullet]: matches input's middle option across all three budgets; demonstrates the differentiator's full pipeline on first use
- **Aggressive default (10k tokens, heavy truncation, anomalies + critical path only)**: best showcase of token-efficiency claim; risks "where did my data go?" feedback

### 12. Plugin Distribution Channel for v1
Research confirmed WASM Component Model + Wasmtime is the right plugin runtime for 2025+ but flagged a sharper learning curve (WIT, capability scoping, `wit-bindgen`, `cargo-component`). Input lists "Plugin marketplace UI" as out-of-v1 but built-in templates + filesystem loading as in-scope — the plugin acquisition story is still a product decision.
- **Built-in templates only (no third-party plugins in v1)**: lowest support burden; defers all plugin-author UX risk; users get the curated snapshot templates that ship in the binary
- **Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/`** [inferred from input scope]: lets early adopters write/share plugins out-of-band (gist, repo); no marketplace yet
- **Built-in + filesystem + signed-plugin verification**: input explicitly defers signing to post-v1 if scope creeps; listed for completeness
- **Built-in + online registry from day one**: out-of-v1 per input; would multiply scope

## Pre-filled Values

- **Scale Intent: startup** [inferred from: input "Development context > Scale Intent: startup (shipped public OSS used by external devs, not enterprise-production APM scale)"]
- **Platform: Tauri 2 desktop (native window + webview)** [inferred from: input "Cross-platform desktop app... Rust + Tauri 2 for cross-platform desktop UI" and "Platform: Desktop app (cross-platform Win / Mac / Linux)"]
- **Primary Language: Rust** [inferred from: input "Primary Language: Rust" and the entire edge-tech surface (wgpu/WGSL, wasmtime, DuckDB Rust bindings, Apache Arrow, rmcp)]
- **Target Users: Public OSS — developers using AI coding assistants** [inferred from: input "Target Users: public OSS — developers using AI coding assistants who want instant local observability + token-efficient AI debug workflow"]
- **Growth Model: Modular monolith (with WASM plugin extension layer)** [inferred from: input "Growth Model: modular monolith. Rust modules: ingest, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server"]
- **Development Style: agent-driven** [inferred from: input "Development Style: agent-driven (built via Andromeda v2 pipeline — recursive dogfood validates our OTel mandate on its own creator tool)"]
- **Core Functionality: Universal local OpenTelemetry desktop dashboard that ingests OTLP HTTP+gRPC, visualizes traces/metrics/logs via GPU-accelerated charts in a quarter-screen widget + expanded view, and produces token-efficient curated AI-debug snapshots on one click, with optional MCP server for AI agents** [inferred from: input WHAT, Scope v1, Positioning one-liner, Extracted/Core idea]
- **Telemetry Retention Surface: In-memory DuckDB ring buffer only (5–10 min, configurable)** [inferred from: input "Out of v1 scope: Persistent storage — v1 is in-memory ring buffer (5-10 min). Disk persistence + longer retention is а separate scope"]
- **AI-Agent Integration Surface: Clipboard + optional MCP server (default off)** [inferred from: input "MCP server toggle (off / on; off by default)" and entire Investigate workflow + MCP server sections]
- **Snapshot Curation Aggressiveness Default: Balanced (25k tokens)** [inferred from: input "Snapshot token budget (10k / 25k / 50k)" with default-25k bullet in pipeline step 5]
- **Plugin Distribution Channel for v1: Built-in templates + filesystem loading from `~/.andromeda-pulse/plugins/`** [inferred from: input "v1 ships built-in templates; community plugins loadable from `~/.andromeda-pulse/plugins/` directory; signed plugin verification optional (defer к post-v1)" and "Out of v1 scope: Plugin marketplace UI"]

## Quiz I Scope Note
Quiz I collects PRODUCT-level decisions: what, for whom, how it grows, and how we develop it (classic vs agent-driven).
Technical decisions are collected later by:
- Quiz II (architectural forks: framework, database, deployment, API style, module boundaries, validation library, error handling).
- Specialist skills (tests / obs / security / design / a11y — their respective domains).
