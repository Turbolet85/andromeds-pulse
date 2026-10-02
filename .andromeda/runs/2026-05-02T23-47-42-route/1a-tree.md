# 1A Tree (intermediate — for Phase 1B + 1C consumption)

## Epoch 1 — Foundation (depth 2, 9 chunks across 3 sub-blocks)

- sub-block 1a: Project init
  - chunk: Cargo workspace + 10 crate stubs + rust-toolchain.toml 1.85+ (source: arch §Established Decisions [Module Boundaries] + §Occupied Resources Cargo workspace crate names + security plan §Universal "rust-toolchain ≥ 1.85.0 for Edition 2024")
  - chunk: Tauri 2 scaffold + 5 capability JSON files + bundle id `com.andromeda.pulse` (source: arch §Stack Tauri 2.x + §Occupied Resources Tauri capability identifiers `pulse:default`/`pulse:tray`/`pulse:notification`/`pulse:updater`/`pulse:plugin-fs`)
  - chunk: Code-signing setup — Azure Key Vault HSM + GitHub OIDC + Apple Developer ID + Tauri updater Minisign Ed25519 (source: security plan §Bootstrap phases secret-management-init + §Secret Management; arch §Established Decisions [Code Signing])

- sub-block 1b: Harness + CI
  - chunk: xtask agent-run harness — 5-command discipline + health endpoint + PID file + JSON log format (source: test plan §3 Test Harness Contract Bootstrap phases #1-#5; arch §Cross-cutting Patterns Development Style=agent-driven)
  - chunk: Base CI workflow — matrix Linux/macOS/Windows + harden-runner SHA-pinned + cargo-nextest + cargo-llvm-cov coverage gate (source: test plan §9 CI Integration; security plan §Bootstrap phases dep-security-ci-gate harden-runner)
  - chunk: Supply-chain CI gates + secret-scanning + .gitignore — cargo-audit/deny/auditable + Dependabot + gitleaks + GitHub Environment production-release (source: security plan §Bootstrap phases dep-audit-tooling-install + secret-scanning-ci-gate + dep-security-ci-gate; §Dependency Security)

- sub-block 1c: Specialist tooling install
  - chunk: Tracing self-observation harness — tracing + tracing-subscriber JSON + tracing-appender daily + tracing-error + service.name compile-time (source: obs plan §3 Tracing init "NO OTel SDK linked into self-observation runtime — recursion-free by construction"; arch §Cross-cutting Patterns Self-observation discipline)
  - chunk: Design tokens bundle — Tailwind v4 @theme NASA palette + WOFF2 fonts bundled local (source: design plan §Color Palette + §Typography + §Spacing + §Motion + §Surface desktop-webview Tokens)
  - chunk: A11y dev stack install — axe-core/playwright + Lighthouse + pa11y + react-aria-components + focus-trap-react + tabbable + colorjs.io + eslint-plugin-jsx-a11y + motion/react useReducedMotion (source: a11y plan §3 Bootstrap phases all 9 phases consolidated as one install unit per agent-driven harness convention)

## Epoch 2 — Ingest pipeline (depth 1, 4 chunks)

- chunk: OTLP gRPC receiver — tonic 0.14 :4317 bound 127.0.0.1, TraceService/MetricsService/LogsService, .max_decoding_message_size 8MB, grpc-trace-bin extraction → traceparent (source: arch §Standard Contracts OTLP receiver spec-conformance + §Stack tonic 0.14.x; security plan §Input Validation OTLP/gRPC + §Universal anti-pattern; obs plan §3 Trace context propagation gRPC)
- chunk: OTLP HTTP receiver — axum 0.8 :4318 bound 127.0.0.1, POST /v1/{traces,metrics,logs}, DefaultBodyLimit 8MB, Host-header allowlist, CORS deny-by-default (source: arch §Standard Contracts OTLP HTTP + §Stack axum 0.8.x; security plan §API Security CORS + Host header + body limit)
- chunk: Ingest channel + post-decode invariants — tokio mpsc backpressure, span_id 8/trace_id 16/attribute bounds, AppError::Ingest variant (source: arch §Established Decisions [In-Process Channel Architecture]; security plan §Input Validation post-prost + anti-pattern "NEVER skip post-prost invariant checks")
- chunk: Rate limiting + port-override validation — tower_governor coarse + per-source-port + ANDROMEDA_PULSE_OTLP_*_PORT TryFrom<u16> (source: security plan §API Security rate limiting; arch §Occupied Resources environment variables; arch §Conventions Configuration units `TryFrom<u16>`)

## Epoch 3 — Storage & query (depth 2, 4 chunks across 2 sub-blocks)

- sub-block 3a: Buffer
  - chunk: DuckDB ring buffer schema + Arrow appender — :memory: connection, 7 reserved tables with TIMESTAMPTZ + ts_unix_nano BIGINT, OTLP-native composite keys (source: arch §Established Decisions [Database] + §Occupied Resources reserved tables + §Conventions Database entity naming + Timestamp handling + Primary key convention)
  - chunk: Retention task + buffer.tick heartbeat — periodic DELETE WHERE ts < cutoff via ANDROMEDA_PULSE_RETENTION_SECONDS, eviction_count + memory_bytes ticks (source: arch §Established Decisions [Telemetry Retention Surface]; obs plan §3 Heartbeat ticks; test plan chaos-test buffer overflow)

- sub-block 3b: Query
  - chunk: Query routers (traces/metrics/logs) — viz crate prepared statements (no format-string SQL), query_id + param_count anonymized logging (source: arch §Standard Contracts paginated list envelope + §Occupied Resources viz routers; security plan §Input Validation DuckDB prepared statements anti-pattern)
  - chunk: Broadcast fan-out + Tauri Channel API — tokio broadcast → pulse://stream/{spans,metrics,logs} binary Arrow IPC + _trace_context metadata + size cap (source: arch §Standard Contracts Real-time push + §Occupied Resources Tauri IPC events; obs plan §3 Trace context propagation real-time push streams)

## Epoch 4 — Webview shell + TauRPC bridge (depth 1, 4 chunks)

- chunk: Tauri webview shell — WebView2/WKWebView, frameless + custom titlebar with drag region, close→minimize-to-tray policy (source: arch §Cross-cutting Patterns Tray icon policy; design system §Surface desktop-webview + Custom titlebar component; layout templates §Component custom titlebar)
- chunk: TauRPC routers + TypeScript bindings — derive macros generate .d.ts per crate router, tsc --noEmit gate (source: arch §Established Decisions [Tauri IPC Bridge] + §Conventions Workspace API style; arch §Occupied Resources TauRPC procedures)
- chunk: AppError serde enum + From impls — Validation/NotFound/Internal/Plugin/Storage/Ingest with sanitization (no stack traces / paths / library versions) (source: arch §Conventions Error response schema Tauri IPC; security plan §Error Handling + §Bootstrap phases error-sanitization-wire)
- chunk: IPC introspection + capability-drift check — app_info/health/ready/get_settings/update_settings + xtask diff TauRPC procedures vs pulse-app/capabilities/ JSON (source: arch §Standard Contracts introspection envelope; security plan §API Security TauRPC capability authorization)

## Epoch 5 — Visualization surfaces (depth 2, 10 chunks across 4 sub-blocks)

- sub-block 5a: WebGPU primitives
  - chunk: WebGPU canvas + WGSL render pipeline — navigator.gpu adapter, render shaders for trace timeline / flamegraph / metrics charts, fallback message on unsupported (source: arch §Stack Visualization surface; design system §Surface Canvas Container; layout templates §Halo State Pulse canvas)
  - chunk: WGSL compute aggregation + 10k spans/sec budget — compute shaders for time-series aggregation/downsampling/group-by, frame_duration_ms metric event bridged via TauRPC, reduced-motion respect (source: input.md performance bar; obs plan perf-budget-instruments WebGPU canvas; design system §Motion accessibility)

- sub-block 5b: Compact widget surface (primary)
  - chunk: Compact widget shell — quarter-screen window, snap-to-edge per-display memory, always-on-top toggle, custom titlebar (source: input.md compact glance-monitor surface; layout templates §Wireframe Compact widget)
  - chunk: Halo State Pulse signature element — WebGPU shader pulse 0.8-2.4 Hz from throughput/1000, LCH hue Earth Blue ↔ Alert Burgundy from error rate, 4-16px blur (source: design system §Brand Identity Signature element; layout templates §Component Halo State Pulse canvas)
  - chunk: Compact widget infographics + footer — service constellation aggregated badge, ingest/error/retention footer band, glance-readable from 2m typography (source: input.md compact widget infographics; layout templates §Wireframe footer; design system §Typography surface-conditional)

- sub-block 5c: Full dashboard surface
  - chunk: Full dashboard shell + tab nav — resizable window, tabs for Traces/Metrics/Logs/Snapshots/Settings, TanStack Router routable views, Cmd+K command palette (source: layout templates §Wireframe Full dashboard + IA notes; design system §Surface desktop-webview Component Patterns Navigation)
  - chunk: Trace timeline + per-service constellation — sortable trace data table (Trace ID/Service/Latency/Error), constellation map with per-service Halo dots (source: layout templates §Component Trace data table; design system §Surface Tables; input.md live trace view)
  - chunk: Metrics charts + logs stream — time-series WebGPU compute aggregation, log stream with span correlation + severity colors + search/filter UI (source: input.md metrics view + log stream; layout templates §Primary screens Metrics/Logs view)

- sub-block 5d: Tray + Settings UI
  - chunk: Tray icon + native menu — monochrome SVG glyph (NSStatusItem/NotifyIcon/AppIndicator), unified Halo overlay layer, OS-native menu Open/GenerateSnapshot/ToggleMCP/Settings/Quit (source: arch §Cross-cutting Patterns Tray icon policy; design system §Surface desktop-native; layout templates §Tray menu)
  - chunk: Settings modal form — theme/widget-position/retention/MCP-toggle/snapshot-preset+budget+format/plugin-manager + keyboard nav + focus trap (source: input.md Settings; layout templates §Component Settings modal; a11y plan critical path P7 settings modal)

## Epoch 6 — Snapshot & Investigate (depth 2, 5 chunks across 3 sub-blocks)

- sub-block 6a: Curation pipeline
  - chunk: Curation primitives — dedupe identical spans, anomaly highlight (latency/error/cardinality), critical-path extraction (source: input.md snapshot generation pipeline curate step; arch §Established Decisions [Snapshot Curation Default] Balanced 25k tokens)
  - chunk: Aggregation + low-signal drop — p50/p95/p99/max metric aggregation, drop verbose attributes (keep service.name/request_id/error) (source: input.md curate step; arch §Established Decisions [Snapshot Curation Default])

- sub-block 6b: Output formatting
  - chunk: Markdown formatter + token budget — hierarchical markdown with citation anchors, 10k/25k/50k budget enforcement, smart truncation prioritizing anomalies (source: input.md format step + token budget; obs plan perf-budget-instruments snapshot token count)

- sub-block 6c: Investigate workflow
  - chunk: Investigate trigger + capture collapse — button on widget/main/context-menu/trace-row + 350ms scale+opacity supporting moment + aria-busy/aria-live (source: design system §Motion Investigation Capture Collapse; layout templates §Component Investigation modal; a11y plan critical path P2 + P6)
  - chunk: Workspace path detection + clipboard + notification — workspace.detect (.andromeda/ marker) + dual .json/.md + 4 preset prompts + tauri-plugin-notification "Snapshot ready ({N} tokens)" (source: input.md snapshot path detection + clipboard prompt; arch §Cross-cutting Patterns OS notification policy; security plan snapshot/clipboard hygiene)

## Epoch 7 — Plugin runtime + MCP server (depth 2, 5 chunks across 2 sub-blocks)

- sub-block 7a: Plugin runtime
  - chunk: wasmtime Component Model + WIT — wasmtime 25+ Cranelift-on-x86_64, 3 plugin categories (custom-dashboard/data-transform/snapshot-template), Config::epoch_interruption + max_wasm_http_fields_size (source: arch §Established Decisions [Plugin Runtime]; security plan §API Security plugin host capability sandbox; input.md plugin system)
  - chunk: Capability sandbox + ResourceLimiter — per-Store memory cap 64MB / table / instance, capability-scoped WIT host imports, basename-only path logging (source: security plan §Input Validation plugin host inputs; obs plan logging-sensitive Vector 3)
  - chunk: Plugin loader + IPC routers — strict-path canonicalize from ~/.andromeda-pulse/plugins/, plugins.list/reload/invoke + xtask drift check, built-in templates under plugins-examples/ (source: arch §Occupied Resources plugins routers; security plan §Code Patterns plugin path canonicalization; input.md plugin marketplace)

- sub-block 7b: MCP server
  - chunk: rmcp stdio sidecar + double-gate — andromeda-pulse-mcp binary feature-gated --features mcp-server + ANDROMEDA_PULSE_MCP_ENABLED runtime gate, JSON-RPC 2.0, stderr-forced-JSON (source: arch §Established Decisions [MCP Server Surface]; obs plan §3 Logging stack mcp-server stderr forced; security plan §API Security MCP feature double-gate)
  - chunk: MCP tool methods + IPC routers — query_traces/query_metrics/query_logs/generate_snapshot #[tool] sharing curation pipeline + mcp.status/start/stop, response.body NEVER logged (source: arch §Standard Contracts MCP server + §Occupied Resources mcp routers; security plan logging Vector 4)

## Epoch 8 — Polish & ship (depth 1, 5 chunks)

- chunk: End-to-end test pass — synthetic OTLP via :4317/:4318 + TauRPC roundtrip + Channels + MCP subprocess + plugin lifecycle + workspace detection (P1-P7) (source: test plan §6 E2E Test Strategy critical paths; arch §Cross-cutting Patterns Test-time telemetry injection)
- chunk: Smoke tests + tauri-driver matrix — install-launch-ingest-query smoke per .msi/.dmg/.AppImage/.deb, tauri-driver headful for tray + window state P5 (source: test plan §6 P5 deferred to tauri-driver headful; arch §Established Decisions [Distribution Channels])
- chunk: Release pipeline + signing automation — release.yml tauri-action + Azure Key Vault EV (Windows) + Apple Developer ID (macOS) + Tauri updater Minisign + latest.json (source: arch §Established Decisions [Deployment / Release Pipeline]; security plan §Secret Management code-signing custody)
- chunk: Distribution channels — Homebrew tap + Scoop manifest via update-channels.yml triggered on release.yml completion (source: arch §Established Decisions [Distribution Channels]; security plan §Dependency Security Homebrew/Scoop license channel)
- chunk: A11y audit + perf SLO + violation-JSON gates — final WCAG 2.1 AA pass (axe/Lighthouse/pa11y), SC 2.3.3 AAA reduced-motion, 10k spans/sec sustained (frame ≥30 fps), violation-summary.json regression detection (source: a11y plan §3 Bootstrap a11y-ci-gate-wire + violation-json-emission-wire; obs plan perf-budget-instruments; test plan §10 Quality Gates performance budgets)
