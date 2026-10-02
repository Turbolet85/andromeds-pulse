## Domain Overview

This product is a **local-first OpenTelemetry (OTLP) developer dashboard** — a desktop application that listens on the standard OTLP ports (`:4317` gRPC and `:4318` HTTP), ingests traces, metrics, and logs from any locally-running application, and visualizes them without requiring Docker, a collector cluster, or a cloud backend. The problem it solves: developers instrumenting apps with OpenTelemetry SDKs need somewhere for that telemetry to land during local development, and spinning up Jaeger/SigNoz/Grafana stacks via Docker Compose is heavyweight relative to the iteration loop. A second, distinct problem in this product: turning a moment of local telemetry into a debug prompt for an AI coding assistant without manually paging through traces or pasting tens of thousands of tokens of raw OTLP JSON.

Existing products in this space:

- **otel-desktop-viewer** (CtrlSpice) — Go CLI built on the OpenTelemetry Collector with a custom desktop exporter; serves a static React app over a local port; uses DuckDB for storage. The reference comparator. Becomes sluggish at high data volume.
- **.NET Aspire Dashboard (standalone)** — Microsoft's dashboard as a standalone Docker container; full OTLP/gRPC + OTLP/HTTP receiver, traces/metrics/logs/structured-logs UI; language-agnostic despite the .NET branding.
- **otel-tui** (ymtdzzz) — Terminal UI viewer; OTLP receiver on 4317/4318; traces, metrics, logs in a TUI; popular for "keep telemetry local while developing a package."
- **otel-front** (mesaglio) — Single-binary lightweight viewer with embedded frontend and in-memory DuckDB; explicit "no Docker, no databases" positioning; inspired by otel-desktop-viewer.
- **Teley** (logaretm) — Real-time trace/log viewer built with Nuxt 3 + Vue 3; waterfall visualization; focused on "session debugger for distributed systems."
- **Uptrace / SigNoz / HyperDX** — Heavier OTel-native backends usable locally but designed for team/production scale; ClickHouse-backed; not really "developer's local widget" tools.
- **Jaeger** — The classic open-source distributed tracing UI; flamegraph view available; typically run via Docker for local dev.

## Common Approaches

**Architecture patterns.**

- **Local-first single-binary desktop or CLI** — every credible tool in this niche ships as one executable that opens both OTLP ports and a local UI. No collector cluster, no separate database server.
- **Modular monolith for the engine** — receivers / pipeline / storage / UI bridge as in-process modules sharing memory rather than network-separated services. Used by otel-desktop-viewer, otel-tui, and the OTel Collector core.
- **In-process embedded analytical store** — DuckDB is the dominant choice (otel-desktop-viewer and otel-front both use it); ClickHouse is the heavyweight peer for the SigNoz/HyperDX/Uptrace tier.
- **Receiver → ring buffer → query** — finite retention is the local-dev norm; persistent disk storage is reserved for the heavier backends.
- **Optional sidecar interfaces** — MCP servers, gRPC introspection, Prometheus scrape endpoints are increasingly added on top of the core receiver+UI.

**User interaction models.**

- Desktop GUI (Tauri / Electron / native webview) is the most polished surface.
- Terminal UI (otel-tui pattern) competes for the keyboard-driven crowd.
- Browser-served local UI (otel-desktop-viewer pattern) — the binary launches a localhost server and opens a tab.
- Tray/menu-bar surfaces and always-visible "glance" widgets are uncommon in this specific niche but are an established pattern in observability-adjacent tools (CleanShot X, Activity Monitor compact, Stats).

**Common workflows / data flows.**

1. Receiver accepts OTLP/gRPC (port 4317) and OTLP/HTTP (port 4318); both transports use protobuf payloads, with HTTP/JSON as an optional encoding.
2. Spans/metrics/logs are decoded, optionally batched, and written to columnar storage (DuckDB, ClickHouse, in-memory Arrow).
3. UI queries via SQL or a domain-specific query layer; trace assembly walks parent_span_id links to reconstruct trees.
4. Visualizations: span waterfall, flamegraph, service map, latency histograms / heatmaps, metric time-series charts, correlated log stream.
5. Export/share: copy trace ID, export OTLP JSON, deep-link, or — in the AI-debug genre — generate a markdown summary for an LLM.

## Technology Landscape

**Languages/Runtimes.** Rust dominates the high-performance OTel-receiver layer in 2025–2026: F5 + Microsoft's joint **OTAP Phase 2** project standardized a Rust pipeline framework (the OTAP Dataflow Engine) with thread-per-core, shared-nothing, zero-copy Arrow internals; **Rotel** is a production Rust OTLP receiver/exporter cited at 75% less memory and 50% less CPU than the Go Collector in benchmarks. Go remains the lingua franca of the OpenTelemetry Collector ecosystem (otel-desktop-viewer, otel-tui, the official collector). Node/TypeScript shows up in browser-served UI layers. Swift/Kotlin appear when projects use Tauri 2 mobile targets.

**Frameworks.**
- **Desktop shell:** Tauri 2 (Rust backend + native webview, ~10–20 MB bundles vs Electron's 100 MB+; supports Windows, macOS, Linux, iOS, Android from one codebase, production-ready in 2025) and Electron (Node + Chromium, larger but ecosystem-heavy) are the two real choices. Native AppKit/Win32/GTK is rarely used here.
- **Web UI frameworks (context only — these appear inside desktop shells of existing products):** existing OTel viewers ship React (otel-desktop-viewer), Vue/Nuxt (Teley), or hand-rolled HTML. Frontend framework selection for the new product is design-specialist territory.
- **Receiver/pipeline frameworks (Rust):** `opentelemetry-proto` for the protobuf schemas, `tonic` for gRPC, `axum` or `hyper` for HTTP, `opentelemetry-rust` for self-instrumentation, the new **OTAP Phase 2** crates for Arrow-native pipelines.
- **TUI:** Bubble Tea (Go), Ratatui (Rust) for the otel-tui-style surface.
- **Plugin runtime:** Wasmtime is the reference WASM Component Model runtime in 2025–2026 (WASI Preview 2 in production; WASI 0.3 emerging; 2-year LTS windows from Bytecode Alliance).

**Data storage.**
- **DuckDB embedded** — the de facto choice for local OTel viewers (used by otel-desktop-viewer, otel-front, OneUptime tutorials); columnar, vectorized, in-process, zero network overhead; first-class Apache Arrow zero-copy interop; Rust bindings mature; query power via standard SQL.
- **Apache Arrow (in-memory columnar)** — the connective tissue between OTLP decode, processing, and storage; OTAP Phase 2 keeps data in Arrow end-to-end; eliminates serialization/deserialization between stages.
- **ClickHouse** — used by the heavier tier (SigNoz, Uptrace, HyperDX); overkill for a single-machine dev tool.
- **In-memory ring buffer** — common pattern for ≤30 min retention windows; backed by Arrow record batches or simple Vec-of-spans.
- **SQLite** — appears in some lighter tools but lacks columnar speed for analytical queries.

**Key libraries / services (domain-specific).**
- **OTLP protobuf definitions:** `opentelemetry-proto` (canonical schemas).
- **OpenTelemetry Rust SDK:** `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp` (network exporter), `opentelemetry-stdout` (console/file exporter — used to break recursion when the product is itself an OTLP observer).
- **OTAP / OTel-Arrow:** `otel-arrow` Rust crates and the OTAP Dataflow Engine — the 2025 inflection point for high-throughput pipelines.
- **MCP server SDK:** the official Rust SDK (`rmcp`) is the standard for exposing telemetry as AI-agent-callable tools; the MCP-for-observability pattern (Datadog MCP, Grafana MCP, OpenObserve MCP, Honeycomb MCP) is now established.
- **GPU / visualization:** `wgpu` (Rust WebGPU implementation, also targets native Vulkan/Metal/DX12), WGSL shaders, plus charting prior art like ChartGPU (1M-point pan/zoom benchmark, LTTB downsampling on the GPU); flamegraph rendering via Rust → WASM → WebGL/WebGPU is reported at ~6× speedups (zymtrace).
- **WASM plugin runtime:** Wasmtime + the Component Model + WIT interface descriptions; capability-based access (no syscalls, file, or socket access unless explicitly granted by the host).

**Deployment.** GitHub Releases is the standard for OSS desktop OTel tools — `.msi` (Windows), `.dmg` (macOS, signed/notarized), `.AppImage` / `.deb` / `.rpm` (Linux). Homebrew taps and Scoop manifests are the typical secondary channels. Cargo install for Rust source builds. Docker is used by the heavier-tier tools (SigNoz, Uptrace, Aspire Dashboard standalone) but is exactly the friction the local-desktop niche exists to remove.

## Recent Trends

- **OTLP with Apache Arrow (OTAP) Phase 2 shipped (2025).** Microsoft + F5 contributed Rust libraries that move OTLP pipelines from row-oriented protobuf to columnar Arrow end-to-end, claiming 30–70% less network data and substantial CPU/memory wins. New 2025–2026 OTel infrastructure is increasingly built around Arrow rather than raw protobuf decode loops.
- **Rust is becoming the high-performance receiver language.** Rotel benchmarks (75% less memory, 50% less CPU vs the Go Collector) and GreptimeDB's OTel-Arrow Rust contributions signal a shift; the Go collector is still dominant by ecosystem inertia, but new performance-focused projects start in Rust.
- **WebGPU is past the "experimental" mark for charting.** ChartGPU (1M points pan/zoom, 35M points at ~72fps in benchmarks), Platformatic's Node.js flamegraph rewrite (Rust → WASM → WebGL/WebGPU, 6× faster), and academic work all confirm WebGPU is the go-to for browser-rendered, large-cardinality observability charts in 2025–2026. WGSL is the shader language; compute shaders are commonly used for downsampling (LTTB) and aggregation as GPU compute passes.
- **WASM Component Model + WASI Preview 2 reached production in 2025**, with WASI 0.3 emerging. Wasmtime 30+ ships with full Component Model support and Bytecode Alliance LTS windows. For new plugin systems in 2025+, "Component Model on Wasmtime" has become the default recommendation (Sy Brand, NGINX Unit, Bytecode Alliance docs all converge on this).
- **MCP-for-observability is a real pattern, not just hype.** Datadog, Grafana, OpenObserve, IBM Instana, OneUptime, and Honeycomb shipped MCP servers in 2025–2026. The MCP Rust SDK (`rmcp`) is the canonical way to expose telemetry tools to AI agents; the workflow is `tools/call` for `query_traces`, `query_metrics`, `query_logs`, etc.
- **AI-debug snapshot tooling is emerging.** LLM-observability platforms (Langfuse, LangSmith, Maxim AI, Arize, Comet Opik) have normalized "trace → context for AI" workflows; teams report 30–50% LLM cost reductions through curation and aggregation. Token-budget-aware snapshot generation (curated markdown vs raw OTLP dump) is a recognized but still under-served gap for *application* (non-LLM) telemetry.
- **Tauri 2 is now the production-recommended Electron alternative** for new Rust-fronted desktop projects in 2025–2026, with mobile targets added. Bundle sizes 5–10× smaller than Electron and a clearer security model (capability-scoped permissions).
- **DuckDB ecosystem expansion in 2025.** 127 extensions (24 core, 103 community), continued Rust-binding maturation, and Infera (ML-inference-via-SQL Rust extension) are the headline items; DuckDB is now the default columnar in-process store for analytical workloads on the desktop.
- **"Dev-only OTel viewer" as a recognized product category.** otel-desktop-viewer, otel-tui, otel-front, Teley, and the standalone Aspire Dashboard are all explicitly positioned against the Jaeger-via-Docker friction curve. The category is small, mostly individual-maintainer projects, with no clearly dominant polished player.

## Considerations for This Project

- **Self-observation / recursive-exporter trap.** This product *is* the OTLP receiver. Instrumenting it with `opentelemetry-otlp` pointed back at itself creates an infinite loop; standard practice is to use `opentelemetry-stdout` (or a file exporter) for the product's own telemetry while the OTLP network exporter is reserved for downstream apps. Any architecture decision involving `ANDROMEDA_OBSERVER_URL` must explicitly distinguish "outbound observer" (don't dial — we *are* the observer) from "inbound receivers" (the OTLP ports we listen on).
- **OTLP wire format is a moving target.** Standard protobuf-over-gRPC/HTTP is the baseline (OTLP spec 1.10.0 in 2025), but OTAP Phase 2 (Arrow-encoded) is the 2025–2026 performance frontier. SDKs in the wild emit standard OTLP today; OTAP support is mostly server-side and emerging. The decision is whether to support OTAP ingest from day one, defer until SDK support broadens, or stick with classic OTLP for v1.
- **GPU-accelerated visualization has well-documented prior art and well-documented pitfalls.** Compute-shader downsampling (LTTB), instanced draws for time-series, persistent mapped buffers, and LOD/culling are the techniques that work; the trap is hand-writing each chart type from scratch when ChartGPU-style libraries exist, vs the upside of full control matching the design vision. Also: `prefers-reduced-motion` accessibility and graceful degradation when WebGPU isn't available (older Linux drivers, certain remote sessions) are real concerns.
- **Token-efficient snapshot curation is the actual product differentiator and the riskiest module.** Dedupe / anomaly detection / critical-path extraction / metric aggregation / smart truncation are non-trivial — each is a small algorithm, but the integration (preserving citation anchors, hitting a token budget, ranking what to keep) is where this product earns its positioning. Most existing tools either dump raw JSON (useless for LLMs) or pretty-print without curation. The output format should be optimized for LLM ingestion (hierarchical markdown with stable anchors), not human reading.
- **WASM Component Model is the right choice but expect a sharper learning curve than typical plugin systems.** WIT interface design, capability scoping, host bindings, and the Rust → WIT toolchain (`wit-bindgen`, `cargo-component`) are all maturing but still rougher than mature plugin systems (e.g., Lua, JavaScript-VM). The capability-based security model is a major win, but plugin authors need clear examples to be productive. Plan for a "first plugin in 30 minutes" tutorial.
- **The compact-widget surface is novel in this niche.** None of the existing OTel viewers (otel-desktop-viewer, otel-tui, otel-front, Teley, Aspire Dashboard) ship a quarter-screen always-visible widget. This is real differentiation but also real UX risk: keeping a widget always-on-top across multi-monitor setups, full-screen apps, and DPI scaling is fiddly on every OS, and Tauri's `alwaysOnTop` has had bugs (e.g., issues #5793 and #9439). Validate the widget surface on the three OSes early.
