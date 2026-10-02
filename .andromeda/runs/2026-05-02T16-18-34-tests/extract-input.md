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
