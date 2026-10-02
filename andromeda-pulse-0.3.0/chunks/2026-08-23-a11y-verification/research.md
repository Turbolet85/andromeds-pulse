# Codebase Research — 2026-08-23-a11y-verification

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 14 · **Code-graph queries:** 2 (ts plane)

The extracts converged on a small number of sharp, answerable questions rather than a broad survey,
so research went narrow-and-deep on those. Every question the seven extracts marked "research's
question" is answered below; three of them retired a premise.

## Files inspected
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (full, 329 lines) — the CARRY-3 locus.
  Renders a **native `<table>`** at `:115` with `<thead>` `:124`, `<tbody>` `:167`, `<th scope="col">`
  carrying `aria-sort` `:137`, `<tr>`×4, `<td>`×6. Inline style sets only `width` /
  `borderCollapse: "collapse"` / `fontFamily` / `fontSize` — **no `display:` override**, so the
  implicit ARIA `table` role is intact. Also ships `aria-pressed` `:92` (Errors-only toggle),
  `aria-label` `:314` (per-row Investigate), `aria-hidden` `:228,:304`. 13 `data-testid` occurrences.
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (`:60-90`, `:222-262`) — wraps the
  `<canvas>` `:247` in `<section aria-label={summary} data-testid="constellation-canvas">` `:229-231`.
  A named `<section>` **is** `role="region"`. Gates motion via `useReducedMotion()` `:63` → threaded to
  the frame loop as `prefersReducedMotion` `:203` with `reducedMotion` in the effect deps `:219`.
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (`:32,:67,:284,:300`) — the widget twin; gates
  motion identically. Rendered by `CompactWidget.tsx:90`.
- `pulse-app/ui/src/widget/constellation-types.ts` (`:192-212`) — `constellationSummary()` is
  **aggregate-only**: filters to live/non-archived, counts by lifecycle state, tallies findings,
  returns e.g. `"Service constellation: no active services."`. **No service name reaches the
  accessible name.**
- `pulse-app/ui/src/dashboard/routes/TracesRoute.tsx` (`:25-76`) — mounts `ConstellationCanvas` `:74`
  and `TraceTable` `:76` as siblings inside `<section aria-labelledby="route-heading-traces">` `:28-30`.
- `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` (`:275-290`, `:376-390`) — both error states
  render `role="alert"` on `--color-text-primary` with the accent as a border only.
- `pulse-app/ui/src/components/InvestigateButton.tsx` (`:64`) — `aria-busy` shipped.
- `pulse-app/ui/src/widget/FindingsCounter.tsx` (`:66-67`) — `aria-expanded` + `aria-haspopup="dialog"`,
  deliberately no `aria-controls` (commented rationale: separate document, axe `aria-valid-attr-value`).
- `pulse-app/ui/tests-a11y/reduced-motion.spec.ts` (`:8-52`) — sweeps **six** surfaces.
- `pulse-app/ui/tests-a11y/axe/p13-empty-states.spec.ts` · `p2-investigation-modal.spec.ts` ·
  `p11-constellation-semantics.spec.ts` — existing coverage boundaries (see Patterns).
- `.github/workflows/ci.yml` (`:112-217`) + `xtask/src/main.rs` (`:78-79,:174`) — the a11y CI wiring.

## Graph impact (code-graph, ts plane; trace `tree-query-2026-08-23-a11y-verification.json`)
- **`HaloCanvas`** — `symbol` view resolves it at `pulse-app/ui/src/halo/HaloCanvas.tsx` (props
  `ariaLabel` `:45`, `cumulativeSeverity` `:47`, `activityState` `:48`). The `refs` view returns
  **only** self-references inside `HaloCanvas.tsx` (`:70-74`, its own props interface) and one test
  hit (`HaloCanvas.test.tsx:193`). Cross-checked by grep: every non-test mention elsewhere
  (`halo-pipeline.ts:4`, `halo-types.ts:2`, `severity-to-halo.ts:54`,
  `widget/ConstellationCanvas.tsx:10`) is a **comment**. → **Zero production render sites.** Both
  preconditions for trusting an empty result are met: the ts plane built (not cold-start) and the
  symbol IS indexed (query 4 returned rows). This is "no callers", not "not visible to the graph".
- **`useReducedMotion`** — canonical re-export at `hooks/use-reduced-motion.ts:5`; five production
  consumers (`CanvasContainer:48`, `InvestigationModalForm:160`, `MetricsChart:31`,
  `dashboard/…/ConstellationCanvas:63`, `widget/ConstellationCanvas:67`, plus `HaloCanvas:79` which
  never renders).

## Patterns detected
- **Semantic-HTML-first is already the house style** (`TraceTable.tsx:115-167`): native `<table>` +
  `scope="col"` + `aria-sort`, with zero redundant `role=`. This matches a11y-plan §11's own ban on
  ARIA duplicating inherited semantics — the plan's rule and the shipped code agree; only the plan's
  *claim about current state* was wrong.
- **Canvas-wrapper pattern** (`ConstellationCanvas.tsx:229`): `<section aria-label={aggregate summary}>`
  around the canvas, with a sibling DOM overlay `constellation-labels` `:258` carrying per-dot text.
  `p11-constellation-semantics.spec.ts` already asserts "every canvas must live inside a labelled
  section wrapper".
- **Motion gating is React-state-driven, not a vanilla rAF loop** — `useReducedMotion()` feeds
  `prefersReducedMotion` into the pipeline and sits in the effect deps, so a11y-plan §11's warning
  about vanilla loops needing their own `matchMedia` does not bite on either constellation surface.
- **Axe spec shape** (`p1`/`p2`/`p13`): `installTauriIpcMock(page)` in `beforeEach`, then
  `runAxeSweep(page, { surface, url, setup? })`. `p2` opens the modal via `setup` but performs **no
  run**, so result/error/progress states are never rendered — precisely the `p14` gap.
- **Test-only IPC mock** (`tests-a11y/helpers/mock-tauri.ts:14`, `:101-107`): intercepts
  `TauRPC__<router.path>`. The suite runs Playwright against the built Vite dist with no Tauri
  backend, so this is the only way to render the webview standalone.

## Conventions to follow
- **Spec-numbering is contiguous**: `p1`–`p13` all exist; `p13` is `p13-empty-states.spec.ts`. The
  next free slot is **`p14`** (the CARRY's stated `p13` is occupied).
- **Suite entry is `npm run test:a11y`** (`package.json`), a six-stage chain:
  `verify:contrast && playwright test --config=playwright-a11y.config.ts && lighthouse-runner.mjs &&
  run-pa11y.mjs && aggregator.mjs && regression-detector.mjs`. CI invokes it as
  `cargo xtask test:a11y` (`ci.yml:125`), which is a thin delegate (`xtask/src/main.rs:174`).
- **Focus ring is a token**, never per-component (design-system §Focus); any cell-nav focus affordance
  inherits it via `:focus-visible`.
- **Roving `tabindex="0"/"-1"`** is the only compliant shape for cell navigation — a11y-plan §11 bans
  `tabindex > 0` and nested focusables.

## New files to create
- `pulse-app/ui/tests-a11y/axe/p14-investigate-states.spec.ts` — axe sweep over the Investigate
  result / error / progress states, driven through the real `investigate.run_action` mock path
  (populated fixture, not empty arrays — per the 2026-08-17 empty-fixture lesson).

## Files to modify
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — add cell/row keyboard navigation
  (roving tabindex + arrow handling + scroll-into-view inside the existing `trace-table-scroll`
  region). **Measured absent:** zero `onKeyDown` / `onKeyUp` / `tabIndex` / `Arrow*` occurrences.
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` — colocated unit assertions for the
  new traversal (the crate-local companion; `vitest` + `@testing-library/react`).
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` — **only if** P4 fork 1 chooses
  "rename to the literal"; otherwise untouched.
- `pulse-app/ui/tests-a11y/keyboard-focus/widget-and-modals.spec.ts` — extend with the trace-list
  traversal assertion (`tabbable`-computed expected vs `document.activeElement`), if P4 puts it at the
  Playwright tier rather than the vitest tier.

*Caller threading:* `TraceTable` has exactly one production caller (`TracesRoute.tsx:76`) and it passes
only `rows` / `isLoading`; cell navigation is component-internal state, so no prop threading and no
registration site is implicated. No new export, so no `index` re-export is owed. The data pin that
rides this change is `TraceTable.test.tsx` (already listed) and the headful leg's Traces selectors in
`pulse-app/ui/tests-e2e/webview-drive.mjs` + `xtask/src/webview_drive.rs`, which bind
`trace-table` / `trace-row` / `trace-table-empty` — those `data-testid`s must **survive** any edit, or
the shipped 7-stage leg breaks.

*Load-bearing equality:* the design needs `<table>` **⇒ implicit `role="table"` exposed to AT**. Verified
for THIS case, not merely in general: the element is `<table>` (`:115`) and its computed display is not
overridden (inline style enumerated above sets no `display`; no `className` on the element, and
`tokens.css` has no `table { display: … }` rule). A `display:block`/`grid` on a `<table>` is the one
thing that would strip the role, and it is absent.

## Open questions
- **Cell-navigation tier and depth** → blocks: **plan-decision**. Full arrow-key grid traversal
  (a11y-plan §5's literal "Tab across cells, arrow-key drill into rows/columns") vs row-level
  Tab/Enter on a 4-column read-only table, and whether the assertion lands at the vitest tier or the
  Playwright tier. P4 resolves before synthesis.
- **Canvas region accessible name** → blocks: **plan-decision**. Literal `"Telemetry traces chart"`
  vs the shipped aggregate summary. P3 retired the security discriminator (summary is aggregate-only),
  so this turns purely on information value vs plan-conformance. P4 resolves before synthesis.
- **`cargo audit` probe outcome** → blocks: **implementation-scope**. The PREREQ must run in full form
  at point 40 and its result (basis reproduced vs deferral ended) plus the owned-ID enumeration —
  the plan body says **7**, the annotation says **8** — is measurable only at implement time.
