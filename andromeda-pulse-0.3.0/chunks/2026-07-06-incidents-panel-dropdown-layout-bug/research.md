# Codebase Research — 2026-07-06-incidents-panel-dropdown-layout-bug

## Scope
- **Depth:** moderate (single-surface webview fix; the extracts narrowed it to 3 components + 1 test + 1 a11y spec + tokens) · **Reads:** 7 (FindingsDropdown.tsx, FindingsCounter.tsx, CompactWidget.tsx, FindingsDropdown.test.tsx, p8-findings-dropdown.spec.ts, tokens.css grep, cookbook) · **Globs/Greps:** 4

## Files inspected
- `pulse-app/ui/src/widget/FindingsDropdown.tsx` (full) — the mis-rendering popover. `PANEL_BASE_STYLE` = `position: absolute; top: calc(100% + var(--spacing-xs)); right: 0; minWidth 240 / maxWidth 320; background: var(--color-raised-2); border 1px rgba(74,144,226,0.3); borderRadius var(--radius-md); zIndex 10; transition opacity var(--duration-standard) var(--easing-out)`. **No `maxHeight` / `overflowY`** → an unbounded-height panel. The `top: calc(100% + …)` opens it DOWNWARD from its offset parent.
- `pulse-app/ui/src/widget/FindingsCounter.tsx` (full) — the trigger badge (`<button aria-haspopup aria-expanded aria-controls={FINDINGS_DROPDOWN_PANEL_ID}>`); hidden when count 0. Correct disclosure wiring; NOT the bug.
- `pulse-app/ui/src/widget/CompactWidget.tsx` (full) — mounts both under `<div data-testid="findings-band" style={{ position: "relative", display: flex, justifyContent: "flex-end", padding: "0 md xs", flexShrink: 0 }}>` which is the LAST child of `<main style={{ display:flex, flexDirection:"column", height:"calc(100vh - 32px)" }}>` (constellation `flex:1` above it). So `findings-band` is the offset parent, pinned at the widget's BOTTOM edge.
- `pulse-app/ui/src/widget/FindingsDropdown.test.tsx` (full) — 6 describe blocks (visibility / empty-state / row-rendering / footer-action / Escape+focus-restoration / click-outside). `makeTriggerRef()` appends a real `<button>` to `document.body`; `afterEach` clears `document.body.innerHTML`. All DOM-shape assertions via `getByTestId("findings-dropdown" | "-list" | "-empty" | "-row" | "-mark-all-read")`. This is where new layout-lock assertions extend.
- `pulse-app/ui/tests-a11y/axe/p8-findings-dropdown.spec.ts` (full) — opens via `[data-testid="findings-band"] button[aria-haspopup]` → `getByTestId("findings-dropdown").waitFor("visible")` → asserts 3 rows + severity-in-accessible-name + dots `aria-hidden`. Fixtures from `helpers/v02-fixtures.ts::v02WidgetOverrides` + `installTauriIpcMock(page, …, "compact-widget")`. Selectors are testid-based → a positioning-only change should NOT require spec edits (the panel keeps `data-testid="findings-dropdown"` + its rows).
- `pulse-app/ui/src/styles/tokens.css` (grep) — confirms `--color-raised-2: #2D3139` (opaque), `--color-inset: #0F1117`, `--color-base: #1A1D24`, `--spacing-xs: 4px`, `--radius-md: 6px`, `--duration-standard: 200ms`, `--easing-out`; and `@media (prefers-reduced-motion: reduce)` zeroes `--duration-standard: 0ms` (line 77-80) → the panel fade already degrades for free.

## Graph impact (from the code-graph query)
- **Code-graph query returned `[]` (0 rows)** for `%FindingsDropdown%`/`%FindingsCounter%` — CONSULTED-AND-CONFIRMED: the code-graph is a rust-analyzer SCIP graph (Rust symbols only; 6632 nodes / 32554 edges, all Rust); the webview TS layer is not indexed. Impact for a TS-only chunk therefore comes from grep, NOT the graph. Adoption trace written to `runs/2026-07-06T16-15-03Z-phase/tree-query-2026-07-06-incidents-panel-dropdown-layout-bug.json`.
- **Grep impact (the real blast radius):** `FindingsDropdown` + `FindingsCounter` are consumed ONLY by `CompactWidget.tsx` (+ their co-located `*.test.tsx`). **Widget-only — no dashboard copy** (unlike `ConstellationCanvas`, which has widget + dashboard copies per the 2026-07-02 two-copies learning). So the fix is single-surface; no sibling-copy fan-out.

## Patterns detected
- **Downward-open popover from a bottom-anchored trigger** (`FindingsDropdown.tsx:44` `top: calc(100% + var(--spacing-xs))` inside `CompactWidget.tsx:80` bottom `findings-band`): the exact defect — the panel is laid out below the widget's bottom edge, off the painted viewport. Called out verbatim in `frontend.md` 2026-06-30 ("the findings dropdown is `position: absolute; top: calc(100% + …)` (pops DOWN, below the bottom-anchored trigger)").
- **Static inline design-token styles** (`FindingsDropdown.tsx` `*_STYLE` consts): all styling is first-party-static CSSProperties objects using `var(--token)` — the security-safe / CSP-clean pattern; the fix stays in this shape (no untrusted-input CSS).
- **useEffect document-level Escape/click-outside listeners** (`FindingsDropdown.tsx:129-156`) with `triggerRef.current?.focus()` restore — the canonical click-outside pattern (a11y-plan §11 / rules a11y 2026-05-10). Preserve unchanged.
- **DOM-shape test discipline** (`FindingsDropdown.test.tsx`): render-with-props + `getByTestId` assertions; jsdom has no layout engine so tests lock STRUCTURE, not geometry (test-plan §4).

## Conventions to follow
- **Design token surface scale** (`tokens.css:9`): popovers/dropdowns = `--color-raised-2` (opaque). Keep it; do NOT swap to `--color-inset` (that is the input-field/canvas token — a contrast + role regression). Resolves the scope's token tension (design + a11y both confirm).
- **First-party static inline styles only** (`FindingsDropdown.tsx`): extend `PANEL_BASE_STYLE` with token-valued positioning/bounding; no hardcoded hex/px (design-tokens rule).
- **Fixed-viewport compact widget** (`CompactWidget.tsx:71` `height: calc(100vh - 32px)`; frontend.md 2026-06-30 fixed-height-not-minHeight): the widget must not reflow/stretch (layout-templates §IA no-reflow). Any bounded height uses the same viewport-derived approach.
- **testid-anchored a11y spec** (`p8-findings-dropdown.spec.ts`): keep `data-testid="findings-dropdown"` + `-row` + `findings-band button[aria-haspopup]` so the p8 spec needs no selector change.

## New files to create
- (none) — the fix edits existing files only.

## Files to modify
- `pulse-app/ui/src/widget/FindingsDropdown.tsx` — anchor the panel to open UPWARD from the bottom trigger (`bottom: calc(100% + var(--spacing-xs))` in place of `top: …`) so it renders within the widget viewport over the constellation area; add a bounded `maxHeight` (viewport-derived) + `overflowY: auto` so a long list scrolls internally rather than stretching the widget; keep `--color-raised-2` bg + border + radius + z-index + the 200ms fade. Positioning/bounding only — no change to rows/footer/a11y/handlers.
- `pulse-app/ui/src/widget/FindingsDropdown.test.tsx` — add DOM-shape layout-lock assertions (e.g. the panel style carries the upward `bottom`-anchor + `overflowY: auto` + a bounded `maxHeight`; keeps `--color-raised-2`, not `--color-inset`); all existing tests stay green.
- **Possibly** `pulse-app/ui/src/widget/CompactWidget.tsx` — only if the upward-open needs the `findings-band` positioning context adjusted (it is already `position: relative`, so likely no change; confirm at implement).
- **Not expected:** `p8-findings-dropdown.spec.ts` (testid selectors unchanged) — touch only if the reposition changes visibility timing.

## Open questions
- **Premise correction (surface at P4 via AskUserQuestion):** the working-route line says "correct design-token `--color-inset` background," but design-system + a11y both assign popovers `--color-raised-2` (already in the code), and `--color-inset` would regress contrast. Research shows the "white background" is the off-viewport-overflow symptom (native window bg showing through), not a wrong token. Recommend KEEP `--color-raised-2` + fix via positioning/bounding; confirm with the operator (who wrote the `--color-inset` hint from a live observation) rather than silently override — per the RESEARCH-CORRECTS-INTENT discipline.
- Open-direction (up vs. down) is NOT ambiguous — the trigger is at the widget bottom, so upward is the only on-screen direction; stated as the plan's approach, not a question.
