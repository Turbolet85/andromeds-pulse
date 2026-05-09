---
paths:
  - "pulse-app/ui/**/*.{ts,tsx,jsx,js}"
  - "pulse-app/ui/**/*.css"
---

# Frontend Rules

Path-scoped rules for the desktop-webview React frontend (Tauri 2 webview).

**Authoritative source:** `.andromeda/design-system.md` §Surface desktop-webview + `.andromeda/layout-templates.md` desktop-webview surface + `.andromeda/architecture.md` §Cross-cutting Patterns Webview IPC capability policy.

## Framework
- **Framework:** React 19.x (Vite + TanStack Router)
- **CSS:** Tailwind CSS v4.x with `@theme` block (consumes `--color-*` / `--spacing-*` / `--font-*` / `--duration-*` tokens)
- **Component library:** shadcn/ui (Radix UI primitives + Tailwind, copy-not-install distribution)
- **Canvas:** WebGPU `<canvas>` + `navigator.gpu` + WGSL shaders (Halo State Pulse, latency river, throughput counter)

## State management
- **Server state:** TanStack Query for cached IPC responses + automatic invalidation
- **Real-time push:** Tauri 2 IPC `Channel` API subscriptions (`pulse://stream/spans` etc.) decoded via `apache-arrow` JS package; updates flow into a Zustand store (NOT React state — VDOM diff cost)
- **Client state (local):** `useState` / `useReducer` for component-scoped
- **Client state (global):** Zustand for cross-component (telemetry stream cache, settings, panel layout)
- **Form state:** `react-hook-form` + Zod schemas matching the Rust `AppError::Validation { field, reason }` enum shape

## TauRPC integration (binding)
- Frontend consumes ONLY TauRPC-generated `.d.ts` bindings — never manually re-declare command signatures.
- Generated bindings live alongside `pulse-app/ui/src/bindings/` (per crate router).
- All backend errors deserialize as `AppError` discriminated union (`Validation` / `NotFound` / `Internal` / `Plugin` / `Storage` / `Ingest`); UI maps to user-facing messages without exposing `Internal { message }` raw.
- Real-time push: subscribe to `pulse://stream/spans` / `pulse://stream/metrics` / `pulse://stream/logs` / `pulse://stream/snapshot-progress` / `pulse://stream/plugin-events` via `Channel<Uint8Array>` API; payloads are binary Arrow IPC.

## Capability discipline
- Adding a new TauRPC procedure invocation in the frontend REQUIRES the corresponding `pulse-app/capabilities/` JSON entry. The `xtask capability-drift` check enforces in CI.
- The webview runs under `pulse:default` capability — webview cannot invoke `pulse:updater` / `pulse:plugin-fs` / Tauri core APIs (`fs`, `shell`, `dialog`, `http`).
- NEVER attempt to call Tauri core APIs from the webview without an explicit per-feature capability addition with stated rationale.

## Routing (TanStack Router)
- Compact widget: no deep links (glance-only).
- Full dashboard: routable views per tab/sidebar entry — `/traces`, `/metrics`, `/logs`, `/snapshots`, `/settings`. Tab vs sidebar IA TBD downstream.
- Keyboard shortcuts (TBD): `Cmd+K` / `Ctrl+K` command palette or global search; `Cmd+Shift+P` / `Ctrl+Shift+P` widget ↔ dashboard toggle; `Esc` minimize widget or close modal.

## WebGPU canvas integration
- Halo State Pulse renders on a dedicated `<canvas>` layer OUTSIDE React's render tree — Arrow data is pushed into the canvas via WebGPU compute shader, side-stepping VDOM diff cost.
- WebGPU init: `navigator.gpu.requestAdapter()` + device / pipeline / shader compile at component mount; reuse pipeline across re-renders.
- Fallback when `navigator.gpu` is undefined: `<canvas>` shows `font-body` text in `color-text-tertiary` ("WebGPU not supported in this context"). MUST be screen-reader accessible (wrap in `<region role="region" aria-label="...">`).
- Frame timing emitted via TauRPC `telemetry.frontend.record_frame_ms(duration_ms, wgpu_backend)` for SLO enforcement (p99 ≤33ms / 30 fps).
- Reduced-motion: degrade to static glow (no pulse rhythm); hue still updates per error rate. Use `useReducedMotion` hook from `motion/react` 12.x.

## Custom titlebar (Tauri 2 frameless)
- `data-tauri-drag-region` on titlebar container (full width except buttons).
- Frame: `decorations: false` in `tauri.conf.json`.
- Window controls: traffic-light buttons left side on macOS (OS chrome convention); minimize/maximize/close right side on Windows/Linux at 32px from edge.
- High-DPI: React + browser handle `devicePixelRatio` automatically; no manual scaling logic.

## React 19 specifics
- React 19 Compiler v1.0 auto-memoizes — do NOT manually wrap with `useMemo` / `useCallback` unless profiling shows necessity.
- Avoid React.memo unless a clear performance win is measured.
- Server components are NOT applicable (Tauri webview is fully client-side); avoid RSC patterns.

## Performance
- Code-split at route boundaries via TanStack Router lazy loaders.
- Lazy-load below-the-fold panels (Snapshots view, Plugin manager).
- Real-time data updates: route through Zustand store, NOT React state (VDOM diff cost at 10k spans/sec is unacceptable).
- Web vitals (LCP / CLS / INP / FCP / TTFB) emitted via `web-vitals` 5.x callbacks → TauRPC `telemetry.frontend.record_web_vital(name, value)` → backend `tracing` log.
- NEVER fetch inside render — move to effect, IPC subscription, or loader.
- Always handle loading / error / empty states explicitly (skeleton component for loading; `font-body` muted message for empty; `--color-accent` text + icon for error).

## CSP
`script-src 'self'` (no eval, no remote, no CDN). WOFF2 fonts bundled locally. WGSL shaders first-party inline or module-imported.

## Accessibility
See `.claude/rules/a11y.md` for full WCAG discipline. Quick reference:
- Semantic HTML first (`<button>`, `<dialog>`, `<table>`, `<label>`).
- All interactive elements keyboard-navigable (Tab/Shift+Tab/Enter/Space/Arrow/Esc).
- Focus ring: `--border-focus` token (`outline 3px solid #4A90E2`); never `outline: none` without alternative.
- Reduced-motion respect on every transition + Halo State Pulse.

## Design tokens
See `.claude/rules/design-tokens.md` for full token spec + component patterns + Self-Validation Protocol. Quick reference:
- NASA Deep Space palette (Earth Blue / Alert Burgundy / Status White-Blue / Stellar Indigo / Feedback Cyan).
- IBM Plex Sans (UI) + JetBrains Mono (data) — bundled WOFF2 locally; banned: Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui, Space Grotesk.
- Borders-only depth strategy (no shadows on dark surfaces).
- Expression level 0.35: max 2 high-impact moments (Halo Pulse + Investigation Capture Collapse).

## Anti-patterns (Tauri 2 webview specific)
- NEVER ship visible Chromium/WebView2 artifacts (default context menu, dev tools open, text selection on non-text elements).
- NEVER use unstyled web scrollbars — apply Tailwind v4 `scrollbar-*` utilities (dark bg + Earth Blue thumb).
- NEVER use browser navigation chrome (back/forward, URL bar).
- NEVER use `alert()` / `confirm()` / `prompt()` — use shadcn/ui Dialog component.
- NEVER fight OS-level keyboard shortcuts (Cmd+Q, Ctrl+W, Alt+F4) — Tauri 2 respects by default.
- NEVER make the window non-resizable without strong justification.
- NEVER mix React Aria Components + Headless UI in the same app.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._

- 2026-05-03: Chaining a build step into the same Vite `outDir` (e.g., chunk #10's Tailwind writes `dist/tokens.css` first, then chunk #11's Vite builds the React bundle into the same `dist/`) requires `emptyOutDir: false` on the Vite side — both as `vite.config.mjs` `build.emptyOutDir: false` AND on the CLI via `--emptyOutDir=false`. Vite's default is to wipe `outDir` before bundling; without the override, the prior step's output is silently deleted и the runtime `<link href="/tokens.css">` resolves к а 404. Pattern landed in `pulse-app/ui/scripts/build.mjs` (chunk #11 Vite invocation following Tailwind step). Any future build step that writes incrementally к the same `dist/` (a future SVG sprite generator, font subset, i18n bundle) must keep this flag set OR restructure the orchestrator к write through Vite's plugin pipeline. Note: when using Vite's `publicDir` for static assets (e.g., `public/fonts/*.woff2`), Vite handles those via its own copy step и does NOT consider them "outDir contents" for the empty check — they coexist с Tailwind output safely.

- 2026-05-09: TauRPC `#[taurpc::procedures(path = "...")]` accepts dotted-namespace path values (e.g. `path = "telemetry.frontend"`) — the macro treats `path` as a string literal, not a single Rust identifier, so 3-segment IPC procedure shapes (`<router>.<sub>.<verb>`) are achievable without trait-name acrobatics. Three downstream effects to know when adding such a router: (a) Specta-generated TS bindings emit the dotted form verbatim as a Router key (`Router["telemetry.frontend"]`) — webview consumers access via bracket OR via the runtime nestedProxy's prefix-match cascade (`proxy.telemetry.frontend.method()` works at runtime even though TS would object к dotted member access); (b) the runtime nestedProxy in `pulse-app/ui/node_modules/taurpc/dist/index.js:30-69` walks dotted args_map keys via `args_maps[nested_path.join(".")]` direct match OR `Object.keys(args_maps).some(k => k.startsWith(...))` — so `proxy.telemetry.frontend.record_frame_ms()` resolves through 2 nested-proxy hops + final args_map["telemetry.frontend"].record_frame_ms lookup; (c) the `xtask capability-drift` parser composes `format!("{router}.{method}")` (xtask/src/main.rs:550), so a dotted router emits in the discovered set as `telemetry.frontend.record_frame_ms` — matching arch §Occupied Resources naming. Pattern verified at chunk #29 with `crates/ui-bridge/src/telemetry.rs` `path = "telemetry.frontend"` + chunk #29 capability-drift report `extra: ["telemetry.frontend.record_frame_ms"]`. Adding such a router still requires both `pulse-app/capabilities/` registration AND arch §Occupied Resources update via `/andromeda-scope-arch` (per security.md Session Additions 2026-05-09).
