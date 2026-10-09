# Codebase Research — 2026-06-30-browser-chrome-suppression

## Scope
- **Depth:** moderate · **Reads:** 7 (main.tsx · App.tsx · index.html · use-window-label.ts · CanvasContainer.tsx · package.json · scope.md) · **Globs/Greps:** 6 (src tree · canvas enumeration · suppression-signal grep · roots · styles · code-graph)

## Files inspected
- `pulse-app/ui/src/main.tsx` (full) — bootstrap: `createRoot(#root).render(<App/>)`. Single SPA entry.
- `pulse-app/ui/src/App.tsx` (full) — **the single shared root**; routes `compact-widget` → `<CompactWidget>` else `<Dashboard>` via `useWindowLabel()`. Already holds a `useEffect(() => { document.title = … }, [])`. **Both windows render through App**, so a `document`-level effect here is the app-wide host (covers dashboard + widget in one place).
- `pulse-app/ui/index.html` (full) — single `<div id="root">` + `/src/main.tsx`; links `/tokens.css`. No inline `oncontextmenu`, no CSP `<meta>` here (CSP is set elsewhere per `csp.test.ts`).
- `pulse-app/ui/src/hooks/use-window-label.ts` (full) — confirms single-bundle multi-window: the label is read once at boot; both windows share the bundle. An App-level effect = app-wide by construction.
- `pulse-app/ui/src/canvas/CanvasContainer.tsx` (full) — canvas inside a `<section aria-label>` wrapper (a11y region pattern); `<canvas>` carries an inline `style` object, no `draggable`. Representative of the 5 canvas hosts.
- `pulse-app/ui/package.json` (full) — scripts: `test` = `vitest run`, `typecheck` = `tsc --noEmit`, `lint` = `eslint .`, `test:a11y` = contrast + playwright-a11y + lighthouse + pa11y. Deps: React 19, Vite 7, vitest 3, @testing-library/react 16 + user-event 14.
- 5 `<canvas>` elements enumerated (grep):
  1. `pulse-app/ui/src/canvas/CanvasContainer.tsx:146`
  2. `pulse-app/ui/src/halo/HaloCanvas.tsx:250`
  3. `pulse-app/ui/src/widget/ConstellationCanvas.tsx:239`
  4. `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx:236`
  5. `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.tsx:151`

## Graph impact (from the code-graph query)
- **Webview frontend is NOT in the Rust SCIP graph.** Query `SELECT symbol, crate, file FROM symbol WHERE file LIKE '%pulse-app/ui%' LIMIT 5` returned `[]` (trace at `runs/2026-06-30T18-25-41-phase/tree-query-2026-06-30-browser-chrome-suppression.json`). `tree.db` indexes Rust (rust-analyzer SCIP) only. **Consequence:** webview-only chunk → **zero Rust blast radius**, no cross-crate impact, no `cargo`-side symbol callers to consider.

## Patterns detected
- **Single shared App root** (`App.tsx:13-24`): a `document`-level `useEffect` in App fires for whichever window mounts → app-wide handler in ONE place. The existing `document.title` effect is the precedent shape.
- **Hook convention** (`src/hooks/use-*.ts(x)` — `use-window-label`, `use-reduced-motion`, `use-platform`, `use-window-controls`): a new `use-suppress-browser-chrome` hook with a colocated `*.test.tsx` fits the established shape and is unit-testable in isolation.
- **Global stylesheet** = `pulse-app/ui/src/styles/tokens.css` (Tailwind v4 `@theme`, built to `dist/tokens.css`, linked by `index.html`) — the single place for a global `canvas { … }` rule covering all 5 canvases + any future one.
- **NO existing suppression of any kind** (grep `contextmenu|oncontextmenu|import.meta.env|draggable|user-drag|user-select` over `pulse-app/ui/src` → only unrelated `preventDefault` in keyboard handlers; **zero** contextmenu / `import.meta.env` / draggable / user-select hits). **Correction to the P2 extracts:** arch + security claimed "the codebase already gates features on `import.meta.env.PROD` (mirror that pattern)" — that precedent does NOT exist; `import.meta.env.PROD` is standard Vite and available, but this chunk introduces its first use. Net-new, not a mirror. (No "capability already exists" surprise per the 2026-06-01 lesson — verified absent, so genuinely build it.)

## Conventions to follow
- **Vitest jsdom + colocated test** (testing.md §Framework; `vitest.config.mjs` `setupFiles: ["./src/test-setup.ts"]`): colocate `use-suppress-browser-chrome.test.tsx`; dispatch real events via `fireEvent.contextMenu` / `dragstart`; assert `event.defaultPrevented`.
- **`import.meta.env.PROD` gating + test stubs**: in vitest `PROD` is false by default → PROD-gated handlers are OFF in dev/test (so existing `App.test.tsx` is unaffected). Test the PROD path with `vi.stubEnv('PROD', true)` + `vi.unstubAllEnvs()` in `afterEach` (per the 2026-05-08 `vi.unstubAllGlobals`/env discipline).
- **A11y (a11y.md SC 3.3.2 / 4.1.2 / 2.1.2)**: exempt editable elements (`input`/`textarea`/`[contenteditable]`) from the contextmenu suppression so native copy/paste survives in the Settings form; the listener must NOT consume `Escape` (it only handles `contextmenu`/`dragstart`, so this holds by construction); no focus/Tab change (document listeners don't alter focus).
- **Design (design-tokens.md / frontend.md §Anti-patterns)**: "NEVER ship visible Chromium/WebView2 artifacts (default context menu, text selection on non-text)." This chunk directly satisfies that ban — design-sanctioned, no new tokens.
- **Obs (observability.md §PII / frame budget)**: the handlers call `preventDefault()` only — NEVER log the event (no target, no coords); no per-event telemetry (frame-budget cardinality).

## New files to create
- `pulse-app/ui/src/hooks/use-suppress-browser-chrome.ts` — app-wide suppression hook: installs a document `contextmenu` listener (preventDefault for **non-editable** targets) + a document `dragstart` listener (preventDefault when target is `<canvas>`/`<img>`), both PROD-gated, cleaned up on unmount.
- `pulse-app/ui/src/hooks/use-suppress-browser-chrome.test.tsx` — vitest: PROD path (contextmenu prevented on a non-editable node; NOT prevented on an `<input>`; dragstart prevented on a `<canvas>`) + DEV path (no listener installed → contextmenu not prevented).

## Files to modify
- `pulse-app/ui/src/App.tsx` — call `useSuppressBrowserChrome()` once (app-wide; both windows route through App). Mirrors the existing `useEffect` shape.
- `pulse-app/ui/src/styles/tokens.css` — append a global `canvas { -webkit-user-drag: none; user-select: none; }` rule (P-065 defense-in-depth on top of the dragstart handler; also satisfies the design "no text selection on non-text" ban). Always-on (harmless in dev; a canvas is never a saveable image).

## Open questions
1. **Blanket vs editable-exempt `contextmenu` suppression** — resolve at P4 (AskUserQuestion). Lean: **editable-exempt** (preserve native copy/paste in Settings inputs per a11y SC 3.3.2/4.1.2 + design "text selection on non-text"). Blanket is simpler but removes mouse copy/paste in form fields (keyboard Ctrl+C/V still works).
2. (resolved) **PROD-gate** — settled by intent/matrix wording "in a production build"; not ambiguous. Dev keeps right-click → Inspect.
3. (resolved) **Global CSS vs per-element** for the canvas — global CSS chosen (one rule covers all 5 canvases + future; "app-wide" intent).
