## 6. Creator Brief Excerpt

### Must-Work Scenarios

- "Cross-platform desktop app that receives OTLP telemetry (HTTP `:4318` and gRPC `:4317`) from any local application, visualizes traces, metrics, and logs in a polished GPU-accelerated UI"
- "Runs as a compact always-visible glance-monitor (quarter-screen widget mode) plus a full expanded dashboard"
- "Has an 'Investigate' button that captures a **token-efficient curated snapshot** (not raw telemetry dump) of recent activity — copies an AI-ready prompt to clipboard for one-paste debug sessions with Claude Code / Cursor / ChatGPT"
- "Optional MCP server lets AI agents query telemetry directly without manual snapshots"
- "Snapshot generation pipeline: (1) Time window selection — last N minutes (default 5 min; configurable 30s..30min) OR user-selected range from heatmap; (2) Filter — by service / trace ID / error-only / latency-outlier; (3) Curate: dedupe identical spans, highlight anomalies, extract critical path, aggregate metrics (p50/p95/p99/max), drop verbose / low-signal attributes; (4) Format hierarchical markdown with citation anchors; (5) Token budget — target ≤10k / ≤25k / ≤50k tokens preset"
- "Live trace view — span tree with timing, per-service filter, GPU-rendered timeline"
- "Metrics view — time-series charts per metric name (WebGPU compute for aggregation; WGSL render for charts)"
- "Log stream with span correlation"
- "Real-time push: Service health constellation (animated dots per service; color + pulse rate indicate health; throughput visualized as ring intensity); Recent traces (top-N latest spans, color-coded by status / latency); Latency heatmap strip; Error rate sparkline + count; Throughput counter (events/sec with smooth animation)"
- "Plugin sandbox — WASM Component Model boundaries; capability-based access (plugins declare needed APIs via WIT interfaces; runtime grants per-plugin)"
- "MCP `tools/call` requests: query_traces(time_range, service_filter, limit), query_metrics(metric_name, time_range, aggregation), query_logs(filter, time_range, limit), generate_snapshot(time_range, token_budget)"
- "Notification — `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`"

### Rigor Hints

- "**BAR.** Beat otel-desktop-viewer on UI polish; approach Jaeger UI quality; compete with Uptrace / SigNoz on lightweight-ness; zero-config local install; **portfolio-worthy GPU-accelerated visualization**; AI debug workflow that saves real token cost per investigation."
- "**Beautiful infographics requirement** — visual bar matches portfolio showcase tools (think Apple Activity rings, Cleanshot X, Things 3 polish). UI is part of the product, not just a tool."
- "Glance-readable from 2 meters — typography legible at distance; high-contrast palette; motion convey state (pulse / flow / steady) without requiring focused attention."
- "**Development Style:** agent-driven (built via Andromeda v2 pipeline — recursive dogfood validates our OTel mandate on its own creator tool)."
- "**Scale Intent:** startup (shipped public OSS used by external devs, not enterprise-production APM scale)."
- "Public OSS — developers using AI coding assistants who want instant local observability + token-efficient AI debug workflow."
- "Smooth animation; no jank at high cardinality."
- "Zero external runtime required (DuckDB + WebGPU implementation bundled)."
- "10k+ spans/sec" target (WebGPU compute shaders for aggregation; SIMD vectorization for OTLP protobuf parsing)

### Obs Anti-Patterns (creator's explicit asks)

- "**andromeda-pulse IS observability infrastructure** — it RECEIVES OTLP telemetry from other apps and IS the local observer itself."
- "`OTel SDK:` field — still names OTel SDK packages (self-instrumentation is mandatory even for observers) BUT the exporter package is **stdout / console / file**, NOT the OTLP network exporter. For Rust: `opentelemetry_sdk` + `opentelemetry-stdout` crate (not `opentelemetry-otlp`). **Exporting OTLP to itself would be an infinite recursion loop.**"
- "`Observer endpoint:` field — still mentions `ANDROMEDA_OBSERVER_URL` for downstream tooling consistency BUT describes its role as 'not dialed — this project IS the observer; own operational telemetry exports to stdout via console exporter'. Make the deviation explicit."
- "If Phase 3b sub-agent emits a standard OTLP network exporter pointed at `ANDROMEDA_OBSERVER_URL` WITHOUT applying the self-observation adaptation, that is a bug — the sub-agent missed step 5d."
- "opentelemetry-stdout — Self-observation exporter (recursive dogfood — see edge case below)" — listed as the only exporter the product itself uses.
- Token-efficient snapshot guarantee: "Existing snapshot tools dump raw OTLP JSON — a production trace at moderate load = hundreds of thousands of tokens to paste into Claude. Pulse v2 generates **curated** snapshots" — implies obs telemetry feeding the snapshot pipeline must be queryable / aggregatable per the dedupe-anomaly-extraction-aggregation steps without raw OTLP attribute dumps in the snapshot output.
