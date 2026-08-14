# UI Tooling Decisions — andromeda-pulse

## Context (from architecture.md + security-plan.md)

- Product type: cross-platform desktop application (Tauri 2 + Rust) — universal local OpenTelemetry dashboard with always-visible glance widget and one-click token-efficient AI-debug snapshot.
- Audience: developers using AI coding assistants (Claude Code / Cursor / ChatGPT) who want instant local observability without Docker / Jaeger overhead. Technical, dense-UI tolerant, dark-mode-first, value polish (CleanShot X / Linear / Things 3 reference bar).
- Platforms: Desktop only — Windows, macOS, Linux. Detected UI surfaces: `desktop-webview` (compact widget + full dashboard inside Tauri 2 WebView2 / WKWebView) and `desktop-native` (tray icon via `tauri-plugin-notification` / capability `pulse:tray`).
- Scale intent: startup. Single-machine, single-user, public OSS, MIT license, GitHub Releases distribution.
- Backend framework: `tonic` 0.14.x (gRPC OTLP `:4317`) + `axum` 0.8.x on `hyper` 1.x + `tower` (HTTP OTLP `:4318`); single Tokio multi-threaded runtime; Rust 2024 edition (rustc 1.84+); TauRPC IPC bridge with auto-generated TypeScript bindings; real-time push via Tauri 2 `Channel` API with binary Apache Arrow IPC payloads.
- Mobile framework: N/A — desktop only.
- Security tier: Minimal (0). Auth approach: none (Tauri 2 capability gating `pulse:default` + loopback-only OTLP binding is the trust model). CSP `script-src 'self'` (no eval, no remote scripts, no CDN); `worker-src 'self' blob:` is load-bearing for WebGPU compute pipelines.

## Family Selected

**React** — chosen by user, switched from sub-agent's Svelte recommendation. User accepted the React-family Q1 / Q2 / Q3 recommendations (no internal refinement after the family switch).

## Q1: Frontend framework

- **Research recommended:** React 19.x (with Vite + TanStack Router or React Router 6.x) — within React family. Sub-agent's overall recommendation was Svelte 5.x; user-driven family switch routed to React.
- **User response:** picked 1 (recommended)
- **Final answer:** React 19.x (Vite + TanStack Router or React Router 6.x for SPA routing)
- **Reasoning:** React 19's Compiler v1.0 (Oct 2025) auto-memoizes to mitigate VDOM overhead during high-frequency telemetry stream re-renders; ecosystem dominance maximizes AI-coding-assistant familiarity (Claude Code / Cursor / ChatGPT lean React-first), aligning with arch's agent-driven Development Style; 47 KB runtime fits comfortably within Tauri's 10–20 MB bundle envelope.

## Q2: CSS tooling

- **Research recommended:** Tailwind CSS v4.x — within React family.
- **User response:** picked 1 (recommended)
- **Final answer:** Tailwind CSS v4.x
- **Reasoning:** Class-based dark mode is CSP-safe under the locked-down `script-src 'self'` policy (no runtime CSS-in-JS eval); zero runtime overhead; dense utility scale matches arch's "dense, chart-first, low-chrome dashboard" Design Philosophy and pairs natively with shadcn/ui (Q3 final answer).

## Q3: Component library

- **Research recommended:** shadcn/ui (latest 2026 distribution, Radix UI primitives + Tailwind classes).
- **User response:** picked 1 (recommended)
- **Final answer:** shadcn/ui (Radix UI primitives + Tailwind classes; copy-not-install distribution model)
- **Reasoning:** Copy-not-install ownership lands primitives directly in the repo, eliminating dependency-pin drift across Tauri minor bumps for a long-lived OSS desktop app; 100+ accessible Radix-primitive components cover dialog / popover / table / data-grid patterns the dense dashboard surface needs; dark mode native and Tauri templates exist (e.g., tauri-app-template) proving production readiness.

## Decisions Log

`2026-05-02` — Initial UI tooling decisions by `/andromeda-design` Phase 1

- Family: React (user-picked, sub-agent recommended Svelte)
- Framework: React 19.x (Vite + TanStack Router or React Router 6.x)
- CSS: Tailwind CSS v4.x
- Components: shadcn/ui (Radix primitives + Tailwind, copy-not-install)
- Notes: User switched family from sub-agent's Svelte recommendation to React, then accepted all 3 in-family recommendations. The trade-off the user accepted: heavier bundle (~47 KB React vs 1.85 KB Svelte) and VDOM overhead at high-frequency telemetry streams in exchange for ecosystem depth, agent-familiarity bonus (matches arch's agent-driven Development Style), and shadcn/ui's copy-not-install ownership model. Charts are first-party WebGPU `<canvas>` elements managed outside React's render tree, so VDOM diff cost is concentrated in chrome (settings forms, snapshot viewer, navigation, status panels) rather than the hot 10k spans/sec streaming surface.
