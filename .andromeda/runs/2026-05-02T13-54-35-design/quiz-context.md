# Brand Quiz Context — andromeda-pulse

Constant context for all 5 per-Q research sub-agents. Pre-extracted by
orchestrator from architecture.md + security-plan.md + tooling-decisions.md.

## Project intent (from architecture.md)

- **Product type:** cross-platform desktop application — universal local
  OpenTelemetry dashboard with always-visible glance widget and one-click
  token-efficient AI-debug snapshot. Public OSS, MIT license, GitHub
  Releases distribution.
- **Target users / audience:** developers who want instant local
  observability during development (without Docker / Jaeger overhead)
  AND use AI coding assistants (Claude Code / Cursor / ChatGPT). Any
  app speaking OTLP works out-of-the-box. Stare at telemetry for hours
  during dev iteration. Reference polish bar: CleanShot X / Linear /
  Things 3 / Apple Activity rings / tldraw.
- **Platforms:** Desktop only — Windows, macOS, Linux. Detected UI
  surfaces: `desktop-webview` (compact widget primary surface, ~quarter-
  screen always-visible + full dashboard expansion via Tauri 2 WebView2 /
  WKWebView) and `desktop-native` (tray icon via `pulse:tray` capability,
  traffic-light status + minimal action menu). Mobile = N/A.
- **Scale intent:** startup — single-machine, single-user, no tenancy,
  no orchestration, no clustering. Public OSS used by external devs.
- **Growth model:** modular monolith (8 Rust crates wired into one
  Tauri binary, plus xtask) + WASM Component Model plugin layer for
  third-party extensions.
- **Core functionality:** receive OTLP telemetry from local applications
  on `:4317` gRPC + `:4318` HTTP; visualize traces / metrics / logs in
  GPU-accelerated dashboard; capture token-efficient curated markdown
  snapshots for AI-assisted debug; optional rmcp MCP server for AI agents
  to query telemetry directly. **Critical edge case:** the product IS
  the local observer (receives OTLP) — own self-telemetry exports to
  stdout, not back to its own OTLP ports.
- **Development style:** agent-driven (built via Andromeda v2 pipeline
  — recursive dogfood). Deterministic harness invocations,
  machine-parseable outputs, schema-stable contracts.

## Security (from security-plan.md)

- **Security tier:** Minimal (0). Local-first, single-user,
  zero-infrastructure. No user accounts, no public API, no
  internet-exposed surface, no compliance triggers.
- **Auth approach:** none. Tauri 2 capability gating (`pulse:default`,
  `pulse:tray`, `pulse:notification`, `pulse:updater`,
  `pulse:plugin-fs`) is the runtime trust model. Loopback-only OTLP
  binding is the de facto authorization boundary.
- **CSP constraints:** `script-src 'self'` (no eval, no remote, no
  CDN); `style-src 'self' 'unsafe-inline'` (first-party only — no
  third-party fonts loaded over network); `worker-src 'self' blob:`
  (load-bearing for WebGPU compute pipelines). Web fonts served
  from CDN are NOT possible — fonts must be bundled with the app.
- **Reduced motion:** must respect `prefers-reduced-motion` per arch
  Visual reference / aesthetic anchors.

## Tooling (from Phase 1 tooling-decisions.md — DO NOT change)

- **Family chosen:** React (user-picked, switched from sub-agent's
  Svelte recommendation).
- **Frontend framework:** React 19.x (with Vite + TanStack Router or
  React Router 6.x for SPA routing); React Compiler v1.0 (Oct 2025)
  enabled for auto-memoization.
- **CSS tool:** Tailwind CSS v4.x (class-based dark mode; no runtime
  CSS-in-JS; CSP-safe).
- **Component library:** shadcn/ui (Radix UI primitives + Tailwind;
  copy-not-install distribution model — primitives land directly in
  the repo).
- **Mobile framework:** N/A — desktop only.

## Brand-relevant cues from input.md (creator's aesthetic touchpoints)

The brief explicitly names these aesthetic anchors and negative
anchors. The brand quiz sub-agents must honor them as inputs, not
discard them:

- **Aesthetic stance:** "quiet ambient telemetry presence. Not a
  noisy dashboard demanding attention; a glanceable surface that
  conveys state through motion and color quality."
- **Positive references named in brief:** macOS Activity Monitor
  compact view (info density), Apple Watch Activity rings (motion
  as data), CleanShot X (polished tray + capture flow), Linear app
  (typographic restraint), tldraw (canvas polish).
- **Negative anchors (explicitly excluded):** Datadog / New Relic /
  Grafana enterprise-dashboard density (info overload). Status Hero
  / Pingdom (vacant marketing-app sterility). Neon / Supabase
  (heavy gradient SaaS aesthetic).
- **Signature element candidates the brief proposes:** "Service
  constellation" (animated dot field; pulse rate ∝ throughput; halo
  color ∝ error rate), "Latency river" (flowing horizontal stream
  of recent traces; GPU-rendered animation), "Investigation portal"
  (transition animation gathers visible signal into a snapshot
  "object" before clipboard copy notification).
- **Typography hint:** monospace for timestamps / IDs (data);
  sans-serif display for service names / counts. References:
  JetBrains Mono + Inter / Geist / Berkeley Mono.
- **Color hint:** dark + light themes. Status colors not-color-alone
  (paired with iconography per WCAG). Accent color saturation
  modulates with anomaly intensity (subtle when steady; vivid
  when alerts fire).
- **Motion hint:** every state change is a transition; respects
  `prefers-reduced-motion` (degrades to instant); motion-as-data
  principle (motion reflects telemetry character, not decoration).
- **Glance legibility:** "glance-readable from 2 meters" —
  typography legible at distance; high-contrast palette; motion
  conveys state without requiring focused attention.
