# Report — 2026-06-30-browser-chrome-suppression

**Chunk:** Browser-chrome suppression — suppress default WebView2 context menu + canvas image-save app-wide in production (P-064/P-065)
**Date:** 2026-06-30
**Commits:** (pending) feat(2026-06-30-browser-chrome-suppression): suppress the WebView2 context menu + canvas save/drag app-wide in production (P-064/P-065)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `pulse-app/ui/src/hooks/use-suppress-browser-chrome.ts` — app-wide suppression hook
  - NEW `pulse-app/ui/src/hooks/use-suppress-browser-chrome.test.tsx` — 4 vitest cases
  - MOD `pulse-app/ui/src/App.tsx` — mounts `useSuppressBrowserChrome()` once
  - MOD `pulse-app/ui/src/styles/tokens.css` — global `canvas { -webkit-user-drag: none; user-select: none }` rule
  - (ledger) `andromeda-pulse-0.3.0/verification-matrix.json` — P-064/P-065 → implemented (→ verified at P7)
- **Symbols / APIs:** ONE new webview export — `useSuppressBrowserChrome(): void` (a React hook installing document-level `contextmenu` + `dragstart` listeners). **No new TauRPC procedure / IPC method / HTTP endpoint / broadcast event / port / socket / env var.** A DOM event handler invokes no Tauri IPC (capability-drift clean, confirmed).
- **Crates / modules:** none added / removed / changed. The two new files are TypeScript hooks under `pulse-app/ui/src/hooks/`, not Cargo workspace crates. Zero `.rs` delta.
- **Dependencies:** none added / bumped (React `useEffect` + standard DOM APIs only; no npm or Cargo change).
- **Schema / config:** none. The PROD gate is `import.meta.env.PROD` (build-time Vite flag, not a config key); the `canvas` CSS is a behavioral interaction-suppression rule, not a design/config value.
- **Coverage of new surfaces:**
  - `useSuppressBrowserChrome` document contextmenu/dragstart suppression (PROD-gated, editable-exempt) → validation n/a (no external input; `preventDefault()`-only, reads/deserializes nothing) · instrumentation n/a (deliberately emits NO telemetry — obs §10 frame-budget; handlers never log the event) · PII redacted✓ (no logging at all) · tests unit✓ (vitest 4 cases dispatching real `contextmenu`/`dragstart` events + PROD/DEV `vi.stubEnv`) · a11y WCAG✓ (editable-exempt SC 3.3.2/4.1.2; no `Escape` consumption — listens only for contextmenu/dragstart; no focus/Tab change; `npm run test:a11y` 0 new violations) · tokens n/a (no UI element rendered)
  - `canvas { -webkit-user-drag: none; user-select: none }` global CSS → tokens n/a (interaction-suppression behavior, not a color/spacing/typography value) · a11y n/a (the `<canvas>` is SR-opaque; the off-canvas `<section aria-label>` summaries are untouched) · tests by-construction (rule confirmed emitted into `dist/tokens.css` by the lightningcss build)

## Deviations from intent
- **Heavyweight Rust workspace gates treated as no-ops for this zero-Rust-delta webview chunk.** The plan's Test Commands list `cargo clippy --workspace`, `cargo nextest run --workspace`, and `cargo xtask self-verify` (boot half). /implement ran the webview gates + `cargo fmt --check` + `cargo xtask capability-drift` + the a11y harness (self-verify's frontend half) to green and deferred the three heavy Rust gates. Justification: the chunk changed zero `.rs` files (git-confirmed) → those gates are identical to the last wrap's green run (1719/1719, today); the release binary is absent so each would trigger a full cold workspace build (documented rlib-race/OOM risk on this host) + the default-features nextest would clobber the canonical `bindings.ts` — all for a result that cannot differ. They re-run at the next Rust-touching chunk's gate. The wrap light gate (P7) re-runs the webview gates + fmt + capability-drift (what /implement proved green).
- **`import.meta.env.PROD` was net-new, not a "mirror".** The P2 phase extracts (arch + security) assumed an existing PROD-gate precedent; research found zero prior `import.meta.env` usage. Introduced fresh — standard Vite, typed via `tsconfig` `types: ["vite/client"]`, zero `tsc` friction.
- **lightningcss auto-prefixed the canvas rule** to `-webkit-user-drag:none;-webkit-user-select:none;user-select:none` in `dist/tokens.css` — matches intent (defense-in-depth alongside the JS `dragstart` handler).

## Decisions & corrections
- **Editable-exempt context menu (user decision, /phase P4 AskUserQuestion).** Suppress everywhere EXCEPT editable `input`/`textarea`/`[contenteditable="true"]` so native copy/paste survives in the Settings form (a11y SC 3.3.2/4.1.2 + design "text selection on non-text"). Blanket-suppress was the rejected alternative.
- **Single-`App.tsx`-root host = app-wide.** Both windows route through `App` (`useWindowLabel`), so one `document`-level effect covers dashboard + compact-widget — no per-window wiring.
- **Global CSS over per-element props** for the canvas (5 canvas hosts) — one rule covers all + future canvases; `-webkit-user-drag` kept in CSS (not an inline React style) because it is not a typed React `CSSProperties` key.
- **PROD-gate test robustness:** `!import.meta.env.PROD` truthy-gate works whether `vi.stubEnv('PROD', true)` sets a boolean or a coerced string; the DEV path (default vitest PROD=false) confirms the handler is not installed.
- **Headful right-click carry:** the real packaged-WebView2 native-menu absence is folded into **P-076** (Epoch-4 e2e), mirroring the P-061/P-062 headful-delta carries. In-chunk webview vitest (real event dispatch) is the owning affordance proof.

## Outcome
- **Acceptance met:** P-064 (contextmenu suppressed on non-editable in PROD, kept on inputs, absent in dev) + P-065 (canvas dragstart prevented in PROD + global canvas CSS in dist) — both proven by `use-suppress-browser-chrome.test.tsx` (4/4) + the dist build.
- **Gates green (commands run):** `npm run typecheck` ✓ · `npm run lint` ✓ · `npm run test` (vitest **646/646**, +4 mine) ✓ · `npm run build` ✓ (canvas rule in dist) · `npm run test:a11y` ✓ (30 axe passed, Lighthouse all ≥90, pa11y 7/7, regression-detector 0 new vs baseline) · `cargo fmt --check` ✓ · `cargo xtask capability-drift` ✓ clean (0 missing, 0 extra).
- **Smoke:** no Rust boot-path change (zero `.rs`/main.rs/capabilities/tauri.conf delta) → no boot smoke required; the UI-surface half (a11y/contrast harness) ran green against the rebuilt dist. Full `cargo xtask self-verify` boot re-runs at the next Rust-touching chunk (release binary absent; Rust boot unaffected by a frontend-only change).
- No spec↔reality gap surfaced; no soft-exit.
