# Scope — 2026-07-06-incidents-panel-dropdown-layout-bug

**Marker:** `2026-07-06-incidents-panel-dropdown-layout-bug`
**Version:** andromeda-pulse-0.3.0 · **Epoch 3 — State honesty & legibility**
**Working-route intent (val-1 anchor):** _"Incidents-panel dropdown layout bug — the incidents dropdown
(under the widget's unread badge) renders broken: it STRETCHES the window and overflows with a white /
mis-clipped background instead of sitting as a bounded popover. … Focused frontend chunk: bounded-popover
positioning/overflow (no window stretch; correct design-token background)."_ Operator-surfaced at the
2026-07-05-constellation-severity-live-wiring (P-079) live-verify.

## What this builds
Fix the compact-widget's **Findings (incidents) dropdown** so it renders as a **bounded popover** anchored to
the unread-count badge — no window/content stretch, no white / mis-clipped background — while preserving the
chunk #87 a11y disclosure contract (role/aria-expanded/Escape/click-outside/focus-return) and the p8 axe spec.

The "incidents dropdown" in the UI is the **Findings** surface:
- `pulse-app/ui/src/widget/FindingsDropdown.tsx` — the popover panel (the mis-rendering element).
- `pulse-app/ui/src/widget/FindingsCounter.tsx` — the unread-count badge trigger (`aria-haspopup`).
- mounted in `pulse-app/ui/src/widget/CompactWidget.tsx`'s bottom `findings-band` (`position: relative`).

## Observed defect (surface diagnosis — root-cause is Phase 3's job)
The `findings-band` sits at the **bottom** of the `<main height: calc(100vh - 32px)>` column
(`justifyContent: flex-end`, `flexShrink: 0`), and `FindingsDropdown` opens **downward**
(`position: absolute; top: calc(100% + var(--spacing-xs)); right: 0`) from that bottom-anchored trigger — so
the panel is laid out **below the widget's bottom edge**, off the painted viewport. Symptom cluster the
operator saw: the window/content stretches to try to contain it and the off-viewport region reads as a white /
mis-clipped background. The panel's own base style is already a bounded, opaque, design-correct popover
(`minWidth 240 / maxWidth 320`, `background: var(--color-raised-2)`), so the fix is about **where/how it is
anchored and bounded within the widget**, not the component's intrinsic width or its background token.

## Boundaries
- **Frontend-only** — `pulse-app/ui/**` (the `widget/` findings components + their co-located `*.test.tsx`,
  and the `tests-a11y/axe/p8-findings-dropdown.spec.ts` a11y spec if its fixtures/assertions need updating).
- **Zero backend**: no Rust, no TauRPC procedure, no `pulse-app/capabilities/` change, no schema, no new
  crate/dep. (This is NOT a P-079 regression — P-079 was 100% backend; this bug is pre-existing since ~chunk
  #91 and was merely EXPOSED once P-079 populated `list_active()`.)
- **Preserve** the chunk #87 disclosure a11y contract + the p8 axe spec (must stay green) + reduced-motion
  handling + the existing bounded row/list/footer content and "Mark all as read" behavior. Layout/positioning
  fix only — no change to what the dropdown lists or how rows behave.
- **Premise correction — RESOLVED at Phase 4 (operator-confirmed; research-corrects-intent):** the
  working-route line suggested a `--color-inset` background, but Phase 2/3 research + the design and a11y
  extracts established that **popovers/dropdowns use `--color-raised-2`** (which the code already uses;
  design-system §Surface Scale) and `--color-inset` (the input-field/canvas token) would regress the popover
  role + text contrast. The "white background" is a SYMPTOM of the off-viewport overflow (native window bg
  showing through), NOT a wrong token. Surfaced via the P4 AskUserQuestion; **operator confirmed: keep
  `--color-raised-2`**, fix via positioning (open upward) + height-bounding. The frozen working-route line +
  the historical `--color-inset` phrasing are left as-is (historical source); the resolution is recorded here
  + carried into the plan's Acceptance Criteria + the P-080 matrix `notes`.

## Acceptance intent (refined into criteria at Phase 4/5)
- Opening the Findings dropdown does NOT stretch the widget window/content and produces NO white / mis-clipped
  region; the panel renders fully within the widget as a bounded popover anchored to the badge.
- The panel keeps its opaque design-system popover background and bounded width; a11y disclosure + p8 axe stay
  green; reduced-motion respected.

## Not in scope
Dashboard-route incidents surfacing, the Traces auto-refresh bug (next markerless entry), any backend
workspace-key / incident-producer work, per-dot constellation labels, connection-status text.

## Carried annotations
None — the taken-up working-route line carried no `PREREQ:` / `CARRY:` annotations to fold in.
