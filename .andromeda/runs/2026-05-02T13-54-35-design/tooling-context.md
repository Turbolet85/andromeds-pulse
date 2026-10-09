# Tooling Context — andromeda-pulse

Extracted by orchestrator from `architecture.md` + `security-plan.md` for the
Phase 1 UI Tooling Quiz sub-agent. This is the substrate the sub-agent
researches against. Frontend framework / CSS / component library are NOT
named here — the sub-agent decides those. Backend / mobile framework are
arch's choices and do NOT change.

## Product

- **Product type:** cross-platform desktop application — public OSS, MIT
  license, GitHub Releases distribution. NOT a web app, NOT a CLI, NOT
  a server. Universal local OpenTelemetry dashboard with always-visible
  glance widget + one-click token-efficient AI-debug snapshot.
- **Audience:** developers using AI coding assistants (Claude Code /
  Cursor / ChatGPT) who want instant local observability without Docker /
  Jaeger overhead. Technical, comfortable with the command line, value
  density and polish (think CleanShot X / Linear / Things 3 reference
  aesthetic). Will stare at telemetry for hours during dev iteration —
  legibility and dark mode are first-order requirements.
- **Platforms:** Desktop only — Windows, macOS, Linux. Single Tauri 2
  process per machine; one binary launch, no Docker, no cloud backend.
  Detected UI surfaces: `desktop-webview` (compact widget + full
  dashboard, both inside the Tauri 2 WebView2 / WKWebView host) and
  `desktop-native` (tray icon via `tauri-plugin-notification` /
  capability `pulse:tray`). Mobile = N/A.
- **Scale intent:** startup. Single-machine, single-user; no tenancy,
  no orchestration, no clustering. Ship public OSS with portfolio-grade
  visual polish (compete with otel-desktop-viewer on UI; approach
  Jaeger UI quality; match CleanShot / Linear polish bar).

## Stack (from arch — DO NOT change)

- **Primary language / runtime:** Rust 2024 edition (rustc 1.84+);
  single Tokio multi-threaded runtime.
- **Desktop shell:** Tauri 2.x with TauRPC IPC bridge (router-style
  Rust ↔ webview commands; auto-generated TypeScript bindings).
- **Backend framework (in-process):** `tonic` 0.14.x (gRPC OTLP on
  `:4317`) + `axum` 0.8.x on `hyper` 1.x + `tower` (HTTP OTLP on
  `:4318`). Plain Rust services in-process — NOT a separate web
  backend the frontend talks to over HTTP.
- **IPC / data path to the webview:** TauRPC procedures (typed
  `Result<T, AppError>`); real-time push via Tauri 2 IPC `Channel` API
  with **binary Apache Arrow IPC payloads** (no JSON-stringify tax)
  on event names `pulse://stream/spans`, `pulse://stream/metrics`,
  `pulse://stream/logs`, `pulse://stream/snapshot-progress`,
  `pulse://stream/plugin-events`.
- **Visualization surface:** Webview WebGPU (`<canvas>` +
  `navigator.gpu`, WGSL compute + render shaders) inside WebView2
  (Windows) and WKWebView (macOS/Linux). The frontend framework
  must coexist with — not own — the WebGPU canvas; chart rendering
  is GPU-side, not React/Vue VDOM. Frontend framework owns: layout,
  navigation, settings forms, snapshot viewer, status panels, tray
  menu shell. Charts own themselves on a `<canvas>`.
- **Mobile framework:** N/A.
- **Storage:** DuckDB embedded ring buffer in-memory (5–10 min
  retention). Zero-copy Apache Arrow ingest pipeline.
- **Plugin runtime:** `wasmtime` 25+ WASM Component Model with WIT
  interfaces (out-of-band — does not affect frontend choice, but
  plugins MAY produce viz-spec output a future scope renders).

## Security

- **Security tier:** **Minimal (0)** — local-first, single-user,
  zero-infrastructure. No user accounts, no public API surface, no
  internet-exposed network surface, no compliance triggers (no
  PCI / GDPR-as-controller / HIPAA / COPPA). OTLP receivers bind
  `127.0.0.1` only.
- **Auth approach:** **none** — Tauri 2 capability-based authorization
  (`pulse:default` enumerates exactly the allowed TauRPC procedures;
  `pulse:tray`, `pulse:notification`, `pulse:updater`,
  `pulse:plugin-fs` for the rest) is the runtime trust model; the
  OS user owns the app. Loopback-only OTLP binding is the de facto
  authorization boundary. **No SSR vs SPA trade-off triggered by
  auth** — the webview is local-only, no session cookies, no
  cross-origin token flows. Frontend framework is free to be either
  shape; the typical pull toward SSR-first frameworks for auth UX
  does not apply here.
- **CSP (declared in security plan):** `default-src 'self' ipc:
  http://ipc.localhost; img-src 'self' data:; style-src 'self'
  'unsafe-inline'; script-src 'self'; connect-src 'self' ipc:
  http://ipc.localhost; worker-src 'self' blob:`. The
  `worker-src 'self' blob:` clause is **load-bearing for WebGPU
  compute pipelines** — any frontend framework that requires a
  loose `script-src` (CDN-hosted runtimes, eval, dev-mode unsafe-
  eval that ships in production) is incompatible. NEVER `'unsafe-
  inline'` on `script-src`. NEVER remote `script-src` (no CDN).

## Frontend constraints derived from arch + security

- **Bundle size matters** — Tauri 2 was chosen for ~10–20 MB bundles
  vs Electron's 100 MB+. A frontend framework that ships a 200 KB+
  runtime to render mostly canvas + sparse settings forms is a poor
  fit; small-runtime / compile-away frameworks score higher.
- **TypeScript-first** — TauRPC auto-generates `.d.ts` bindings;
  frontend MUST consume typed bindings. No JS-only / dynamic-types
  framework should be considered.
- **No SSR / no Node server** — the app is a single Rust process
  hosting a webview. SSR-only frameworks (Next.js with required
  Node runtime, Nuxt SSR mode) don't fit; SPA / static-build modes
  are required. SSG+islands frameworks are viable as long as build
  produces static assets the Tauri shell loads.
- **Density + dark-mode-default + 2-meter glance legibility** —
  arch's Design Philosophy explicitly commits to a "dense, chart-
  first, low-chrome dashboard" with "dark-mode default". The
  frontend framework's idiomatic component library should support
  dense layouts and not impose airy SaaS-marketing spacing defaults.
- **Long sessions, hours of viewing** — re-render perf matters
  more than first-paint. Reactive frameworks with fine-grained
  updates (signals, no VDOM diff) align with the use case; VDOM
  diff overhead at 10k spans/sec stream is a real concern even
  though the heavy lifting happens GPU-side.
- **WebGPU compatibility** — the framework must not interfere
  with `<canvas>` lifecycle / `navigator.gpu` access. Any
  framework that owns the DOM aggressively (full SSR hydration
  flicker, suspense boundaries around canvas) introduces friction.
- **Agent-driven development** — arch's Project Intent commits to
  agent-driven dev workflow with deterministic harness invocations
  and machine-parseable outputs. Frontend toolchain should have a
  clean CLI surface (npm/pnpm + `vite build` style) and produce
  reproducible artifacts. No frameworks that require interactive
  setup wizards or proprietary cloud-only tooling.

## What the sub-agent decides

- Frontend framework + version (Q1)
- CSS tool + version (Q2)
- Component library + version, OR build-from-scratch (Q3)

What the sub-agent does NOT touch: backend (`tonic` + `axum`),
mobile framework (N/A), language (Rust), shell (Tauri 2), data
layer (DuckDB + Arrow), GPU layer (WebGPU + WGSL), plugin
runtime (wasmtime). Those are arch's choices.
