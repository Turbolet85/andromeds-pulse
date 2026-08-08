# andromeda-pulse — product description (v2)

Feed this to `/andromeda-arch` Phase 0 input dialogue.

---

andromeda-pulse — universal local OpenTelemetry dashboard with AI-assisted
debug workflow.

**WHAT.** Cross-platform desktop app that receives OTLP telemetry (HTTP
`:4318` and gRPC `:4317`) from any local application, visualizes traces,
metrics, and logs in a polished GPU-accelerated UI, runs as a compact
always-visible glance-monitor (quarter-screen widget mode) plus a full
expanded dashboard, and has an "Investigate" button that captures a
**token-efficient curated snapshot** (not raw telemetry dump) of recent
activity — copies an AI-ready prompt to clipboard for one-paste debug
sessions with Claude Code / Cursor / ChatGPT. Optional MCP server lets
AI agents query telemetry directly without manual snapshots.

**WHO.** Developers who want instant local observability during development
(without Docker / Jaeger overhead) AND use AI coding assistants. Any app
speaking OTLP works out-of-the-box. No Andromeda dependency — works
standalone for any OTel-emitting product.

**KEY DIFFERENTIATION.** Three differentiators stack:

1. **Compact glance-monitor surface.** Existing local OTel viewers
   (otel-desktop-viewer, Jaeger-lite forks) are full-window dashboards
   you flip to when something goes wrong. Pulse v2 ships a quarter-screen
   always-visible widget you keep on a side monitor — beautiful real-time
   infographics make it ambient context, not a tool you remember to open.
   Click expands to full dashboard when needed drill in.

2. **Token-efficient AI snapshots.** Existing snapshot tools dump raw OTLP
   JSON — a production trace at moderate load = hundreds of thousands of
   tokens to paste into Claude. Pulse v2 generates **curated** snapshots:
   deduplication of repetitive spans, anomaly highlighting, critical-path
   extraction, metrics aggregation (p50/p95/p99 instead of every data
   point), smart truncation to target token budget. Snapshot is hierarchical
   markdown with citation anchors — designed for LLM ingestion.

3. **GPU-accelerated visualization.** Hand-built Canvas falls over at 10k+
   spans/sec. Pulse v2 uses WebGPU / WGSL compute shaders for time-series
   aggregation + render for trace timeline / flamegraph / metrics charts.
   Smooth animation; no jank at high cardinality.

**BAR.** Beat otel-desktop-viewer on UI polish; approach Jaeger UI quality;
compete with Uptrace / SigNoz on lightweight-ness; zero-config local install;
**portfolio-worthy GPU-accelerated visualization**; AI debug workflow that
saves real token cost per investigation.

**TECH.** Rust + Tauri 2 for cross-platform desktop UI. WebGPU/WGSL for
visualizations (compute shaders + render). WASM Component Model for plugin
system (custom dashboards / data transforms / snapshot template engines).
DuckDB embedded columnar database (telemetry storage; SQL queries). Apache
Arrow for zero-copy ingest pipeline. Frontend framework TBD in arch
(React / Svelte / Solid candidates). MIT-licensed open source, GitHub
Releases distribution.

---

## Scope v1

### Core observability

- OTLP HTTP ingest (`:4318`) and gRPC ingest (`:4317`).
- DuckDB-backed ring buffer (5-10 min default, configurable; columnar
  storage with SQL query power).
- Live trace view — span tree with timing, per-service filter,
  GPU-rendered timeline.
- Metrics view — time-series charts per metric name (WebGPU compute for
  aggregation; WGSL render for charts).
- Log stream with span correlation.
- Search / filter by attribute key-value (DuckDB SQL backend).
- Multi-service tabs (service.name from resource attrs).
- Dark / light theme auto.
- Cross-platform: Windows / macOS / Linux.

### Compact glance-monitor surface (NEW v2 — primary surface)

- **Quarter-screen widget mode** — default surface; window snaps to
  side of screen (left / right / corner); always-on-top toggle;
  remembers position per display.
- **Live infographics** — beautiful real-time visualizations:
  - Service health constellation (animated dots per service; color +
    pulse rate indicate health; throughput visualized as ring intensity)
  - Recent traces (top-N latest spans, color-coded by status / latency)
  - Latency heatmap strip (last N minutes; sparkline-style; clickable
    to drill into time window)
  - Error rate sparkline + count
  - Throughput counter (events/sec with smooth animation)
- **Glance-readable from 2 meters** — typography legible at distance;
  high-contrast palette; motion convey state (pulse / flow / steady)
  without requiring focused attention.
- **Click to expand** — opens full dashboard window; widget remains
  mounted (returns to compact mode on close).
- **Tray icon (secondary)** — traffic-light status; click cycles
  widget visibility (visible / minimized / hidden).
- **Beautiful infographics requirement** — visual bar matches portfolio
  showcase tools (think Apple Activity rings, Cleanshot X, Things 3
  polish). UI is part of the product, not just a tool.

### Investigate workflow (CORE feature)

- "Investigate" button on widget + main window + context menu.
- Generates **token-efficient curated snapshot** (not raw OTLP dump).
- **Snapshot generation pipeline** (configurable preset):
  1. **Time window selection** — last N minutes (default 5 min;
     configurable 30s..30min) OR user-selected range from heatmap
  2. **Filter** — by service / trace ID / error-only / latency-outlier
     (preset options + custom SQL filter)
  3. **Curate**:
     - Deduplicate identical spans (collapse 50× → "50× duplicate
       {span name} median {latency}")
     - Highlight anomalies (latency outliers; error correlation;
       cardinality spikes)
     - Extract critical path (longest span path through service mesh
       per trace)
     - Aggregate metrics (p50/p95/p99/max instead of every data point)
     - Drop verbose / low-signal attributes (keep service.name,
       request_id, error; drop runtime.go.gc.heap stats unless they
       are anomalous)
  4. **Format** — hierarchical markdown with citation anchors:
     ```
     # Trace abc123 — POST /api/checkout (1.2s, error)
     ## Critical path (1.05s)
     - service.name=gateway → span.cart.validate (10ms) [healthy]
     - service.name=cart → span.db.query (945ms) [⚠ p99 outlier; normal p99=120ms]
       └── error="connection_pool_exhausted"
     ## Anomalies
     - cart.db.connections.active spiked 5→47 at 14:23:45.123 (5x baseline)
     ## Related logs
     - 14:23:45.456 ERROR cart "Failed to acquire DB connection" trace=abc123
     ```
  5. **Token budget** — target ≤10k / ≤25k / ≤50k tokens preset; smart
     truncation prioritizes anomalies + critical path; trims low-signal
     bulk data first.
- **Snapshot path detection** (preserved from v1):
  - If cwd has `.andromeda/` → `.andromeda/pulse/{timestamp}.md`
  - Else → `~/.cache/andromeda-pulse/snapshots/{timestamp}.md`
    (XDG-respecting on Linux, `~/Library/Caches/` on macOS,
    `%LOCALAPPDATA%` on Windows)
- **Clipboard prompt** — one of 4 preset templates:
  - **"Claude Code"** (default) — `"Read the snapshot at {path}.
    Investigate the issue I'm seeing in this trace."`
  - **"Cursor"**, **"ChatGPT"**, **"Custom"** — user-editable in Settings.
- **Dual format** — both raw `.json` (OTLP-native, for tooling) and
  `.md` (curated, for AI consumption) written; clipboard link references
  `.md` by default; Settings switch to `.json` for users who prefer raw.
- **Notification** — `Snapshot ready ({N} tokens). Paste in {AI tool}
  to investigate.`

### MCP server (optional)

- rmcp 1.5.0 stdio-transport sidecar.
- Settings toggle (default off) per Quiz I hybrid decision.
- AI agents query telemetry via MCP `tools/call` requests:
  - `query_traces(time_range, service_filter, limit)`
  - `query_metrics(metric_name, time_range, aggregation)`
  - `query_logs(filter, time_range, limit)`
  - `generate_snapshot(time_range, token_budget)` — agent invokes
    same curation pipeline as Investigate button
- Eliminates copy-paste step entirely for MCP-aware AI tools.

### Plugin system (NEW v2 — WASM Component Model)

- **Custom dashboards** — load WASM module that defines a new dashboard
  panel (input: query results from DuckDB; output: rendered viz spec).
- **Data transforms** — WASM module receives ingest stream; emits
  transformed events (e.g., redact PII attributes; derive synthetic spans).
- **Snapshot templates** — WASM module receives curated snapshot;
  emits formatted markdown / JSON / custom format. Replaces hardcoded
  preset templates with user-extensible system.
- **Plugin sandbox** — WASM Component Model boundaries; capability-based
  access (plugins declare needed APIs via WIT interfaces; runtime grants
  per-plugin).
- **Plugin marketplace** — v1 ships built-in templates; community plugins
  loadable from `~/.andromeda-pulse/plugins/` directory; signed plugin
  verification optional (defer to post-v1 if scope creeps).

### Settings

- Buffer size / retention window.
- Ingest ports.
- Snapshot preset template (Claude Code / Cursor / ChatGPT / Custom).
- Snapshot token budget (10k / 25k / 50k).
- Snapshot format (`.md` curated / `.json` raw / both).
- Theme (auto / light / dark).
- Widget mode (compact / hidden / disabled — main-window-only).
- Widget snap position (left / right / top-left / top-right / etc.).
- MCP server toggle (off / on; off by default).
- Plugin manager (list installed; enable / disable / configure).

---

## Edge tech surface area (portfolio showcase)

This project deliberately incorporates bleeding-edge technologies that
demonstrate technical depth — vibe-coders won't touch most of these:

| Tech | Where used |
|---|---|
| **WebGPU compute shaders** | Time-series aggregation on GPU (group-by; downsampling); handles 10k+ spans/sec |
| **WGSL render shaders** | Trace timeline / flamegraph / metrics charts; smooth animation |
| **WASM Component Model** | Plugin system (custom dashboards / transforms / snapshot templates) |
| **wasmtime** | Sandboxed plugin runtime |
| **DuckDB embedded** | Telemetry storage with SQL query power; columnar OLAP |
| **Apache Arrow** | Zero-copy IPC between ingest and DuckDB; columnar in-memory |
| **SIMD vectorization** | Hot-path OTLP protobuf parsing |
| **Tauri 2** | Cross-platform desktop with small bundle (vs Electron) |
| **OTLP HTTP + gRPC** | Multi-protocol receiver implementation |
| **rmcp (MCP server)** | AI agent direct query interface |
| **opentelemetry-stdout** | Self-observation exporter (recursive dogfood — see edge case below) |

---

## Distribution

GitHub Releases (single binary Win / Mac / Linux:
`.msi` / `.dmg` / `.AppImage`). Homebrew / Scoop manifests. No external
runtime required (DuckDB + WebGPU implementation bundled).

---

## Development context (for Alpha Quiz I)

- **Development Style:** agent-driven (built via Andromeda v2 pipeline —
  recursive dogfood validates our OTel mandate on its own creator tool).
- **Scale Intent:** startup (shipped public OSS used by external devs,
  not enterprise-production APM scale).
- **Growth Model:** modular monolith. Rust modules: `ingest` (OTLP HTTP+gRPC
  receivers), `buffer` (DuckDB ring buffer + indices + Arrow ingest pipeline),
  `viz` (WebGPU compute + WGSL render), `ui-bridge` (Tauri commands),
  `snapshot` (curation pipeline + template engine), `workspace-detector`
  (Andromeda-aware path resolution), `plugins` (WASM Component Model
  runtime), `mcp-server` (optional sidecar).
- **Primary Language:** Rust.
- **Platform:** Desktop app (cross-platform Win / Mac / Linux).
- **Surfaces:**
  - Compact widget (primary; always-visible quarter-screen)
  - Full dashboard (expanded view)
  - Tray icon (secondary status indicator)
  - MCP server (optional headless interface for AI agents)
- **Target Users:** public OSS — developers using AI coding assistants
  who want instant local observability + token-efficient AI debug workflow.

---

## Positioning one-liner

> **Local OTel dashboard + glanceable always-on monitor + one-click
> token-efficient AI snapshot. GPU-accelerated visualization. Works
> with anything OTLP. Zero config.**

---

## Critical note for Phase 3b (self-observation edge case)

**andromeda-pulse IS observability infrastructure** — it RECEIVES OTLP
telemetry from other apps and IS the local observer itself. Phase 3b
sub-agent MUST apply the self-observation edge case per:

- `observability-profiles.md` §"Self-observation edge case (recursion)"
- `phase-3b/prompt.md` step 5d

**Required output adaptations:**

- `OTel SDK:` field — still names OTel SDK packages (self-instrumentation
  is mandatory even for observers) BUT the exporter package is
  **stdout / console / file**, NOT the OTLP network exporter. For Rust:
  `opentelemetry_sdk` + `opentelemetry-stdout` crate (not
  `opentelemetry-otlp`). Exporting OTLP to itself would be an infinite
  recursion loop.

- `Observer endpoint:` field — still mentions `ANDROMEDA_OBSERVER_URL`
  for downstream tooling consistency BUT describes its role as "not
  dialed — this project IS the observer; own operational telemetry
  exports to stdout via console exporter". Make the deviation explicit.

- `Notes:` field MUST cite the Quiz I input that identified this as
  self-observation (e.g., "Core Functionality = receives OTLP
  telemetry from local applications — product IS the observer, not a
  consumer of it").

**Signals in Quiz I that trigger this case** (already present in the
description above):

- WHAT: "receives OTLP telemetry from any local application"
- Core Functionality / Product Type: observability dashboard
- Multiple explicit references: "OTLP HTTP ingest", "OTLP gRPC ingest",
  "local observer", "self-observation edge case" (this file's section
  header)

If Phase 3b sub-agent emits a standard OTLP network exporter pointed
at `ANDROMEDA_OBSERVER_URL` WITHOUT applying the self-observation
adaptation, that is a bug — the sub-agent missed step 5d.

---

## Visual reference / aesthetic anchors

For the design specialist's Color World + Signature Element work:

- **Aesthetic stance:** quiet ambient telemetry presence. Not a noisy
  dashboard demanding attention; a glanceable surface that conveys
  state through motion and color quality. References: macOS Activity
  Monitor compact view (information density), Apple Watch Activity
  rings (motion as data), Cleanshot X (polished tray + capture flow),
  Linear app (typographic restraint), tldraw (canvas polish).
- **NOT:** Datadog / New Relic / Grafana enterprise-dashboard density
  (information overload). Not Status Hero / Pingdom (vacant marketing-
  app sterility). Not Neon / Supabase (heavy gradient SaaS aesthetic).
- **Signature element candidates** (design specialist picks):
  - "Service constellation" — animated dot field, each service a dot;
    pulse rate ∝ throughput; halo color ∝ error rate
  - "Latency river" — flowing horizontal stream of recent traces,
    color-coded; smooth GPU-rendered animation
  - "Investigation portal" — when Investigate clicked, a beautiful
    transition animation gathers visible signal into a snapshot
    "object" before clipboard copy notification
- **Typography:** monospace for timestamps / IDs (data); sans-serif
  display for service names / counts. Reference: JetBrains Mono +
  Inter / Geist / Berkeley Mono.
- **Color:** dark + light themes. Status colors not-color-alone (paired
  with iconography per WCAG). Accent color saturation modulates with anomaly
  intensity (subtle when steady; vivid when alerts fire).
- **Motion:** every state change is a transition; respects
  `prefers-reduced-motion` for accessibility (degrades to instant);
  motion-as-data principle (motion reflects telemetry character, not
  decoration).

---

## Out of v1 scope (deferred to v2 Pulse OR scope-arch additions)

- **Persistent storage** — v1 is in-memory ring buffer (5-10 min). Disk
  persistence + longer retention (hours / days) is a separate scope.
- **Distributed tracing** — v1 is single-machine local. Multi-host
  collection / federation is a separate scope.
- **Alerting** — v1 surfaces visible state. Threshold-based alert
  dispatch (Slack / email / webhook) is a separate scope.
- **Saved searches** — v1 has runtime filter UI. Persistent saved
  searches across restarts is a minor scope.
- **Dashboard customization** — v1 ships fixed widget panels. User-
  arrangeable layouts is a separate scope.
- **Plugin marketplace UI** — v1 ships plugin runtime + filesystem
  loading. Online discovery / auto-install marketplace is post-v1.
