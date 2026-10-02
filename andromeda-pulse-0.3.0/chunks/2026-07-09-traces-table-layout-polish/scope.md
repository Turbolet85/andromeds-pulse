# Scope — Traces table layout polish

**Marker:** `2026-07-09-traces-table-layout-polish`
**Version:** andromeda-pulse-0.3.0 · **Epoch:** Epoch 3 — State honesty & legibility
**Working-route entry (verbatim intent):**
> Traces table layout polish — the trace table gets its own internal scroll (max-height + overflow-y:auto on the table container) so the constellation hero + filter toolbar stay fixed (today the whole dashboard scrolls); + refresh the desktop-webview Traces wireframe to show the "Errors only" filter toolbar above the table. Operator-surfaced on the 2026-07-05-anomaly-surfacing (P-068) live boot — the internal-scroll is PRE-EXISTING layout NOT introduced by P-068 (it does not block P-068's anomaly-first intent); the wireframe-toolbar doc is the D-layout-surface within-surface-refinement routine-REJECTED at the P-068 wrap, homed here. · CARRY: constellation per-dot label collision-avoidance — the P-069 labels can overlap when dots cluster (no avoidance yet); a layout refinement of the Traces constellation hero. · CARRY: while in `TraceTable.tsx`, move the PRE-EXISTING `handleSort` / `handleToggleErrorsOnly` `announce(...)` out of the `setState` updater into the event handler — calling `announce` (a StatusLiveRegion setState) INSIDE a `setSortState`/`setState` updater runs during render → a "Cannot update a component while rendering" React warning (pre-existing, NOT introduced by the auto-refresh chunk, whose empty→populated announce is correctly in a `useEffect`); surfaced at the 2026-07-07-traces-table-auto-refresh wrap.

---

## What this builds

A **frontend-only (webview) layout-polish + a11y-correctness** chunk on the full-dashboard **Traces route**. Four folded deliverables, all in `pulse-app/ui/**` — expected **zero `.rs` delta** (no TauRPC / capability / `pulse://` topic / data-fetch change):

### D1 — Traces table internal scroll (primary)
The trace-table container gets its **own** scroll region (`max-height` + `overflow-y: auto` on the table's scroll wrapper) so the **constellation hero** (top) and the **filter toolbar** (Errors-only toggle + column sort controls) stay **fixed and visible** while only the table body scrolls. Today the whole Traces route scrolls as one column, pushing the hero + toolbar off-screen once rows accumulate — the anomaly-first ordering (P-068) and the Errors-only control become unreachable exactly when there are many rows to triage. The change is layout/CSS on the Traces route + table container; it does **not** touch the query, the row data, the sort model, or pagination.

### D2 — Traces wireframe doc refresh
Refresh the **desktop-webview Traces wireframe** (design/layout documentation) so it shows the **"Errors only" filter toolbar above the table** (matching the shipped P-068 anomaly-first toolbar) and the fixed-hero / internal-scroll table region. Documentation artifact only — the D-layout-surface within-surface-refinement that was routine-REJECTED at the P-068 wrap and homed here.

### D3 — Constellation per-dot label collision-avoidance (CARRY, folded)
The P-069 always-on per-dot service-name labels on the **Traces constellation hero** can **overlap** when dots cluster (no avoidance yet). Add a layout refinement so labels do not overlap / occlude neighbouring dots when the constellation is dense. Scope is the **dashboard Traces constellation hero** only (the compact widget stays aggregate-glance per the P-069 layout boundary). Must preserve the P-069 verified contract: each dot keeps an always-on accessible name + a non-color severity token (SC 1.4.1 / 4.1.2).

### D4 — `announce(...)`-in-updater React-warning fix (CARRY, folded)
In `TraceTable.tsx`, the **pre-existing** `handleSort` / `handleToggleErrorsOnly` handlers call `announce(...)` (a `StatusLiveRegion` setState) from **inside** a `setSortState` / `setState` updater callback — which runs during render, triggering a React "Cannot update a component while rendering a different component" warning. Move each `announce(...)` **out of** the state-updater and **into** the event-handler body (compute the announcement from current state, call `announce` directly). This is a correctness fix for a pre-existing warning (NOT introduced by the P-081 auto-refresh chunk, whose empty→populated announce is correctly in a `useEffect`). Must preserve the existing announce semantics (sort-direction / Errors-only-toggle announcements still fire once per user action, politely).

> **Research amendment (2026-07-09 /phase P5 val-1 — intent-incomplete):** the CARRY named BOTH `handleSort` and `handleToggleErrorsOnly`, but the codebase (`TraceTable.tsx:47-63`) shows only `handleSort` calls `announce(...)` *inside* the `setSortState` updater; `handleToggleErrorsOnly` ALREADY announces *outside* its setter (announce → `setErrorsOnly(next)`). So D4 fixes `handleSort` only; `handleToggleErrorsOnly` is confirmed correct (no-op). Outcome (no render-phase announce warning) unchanged.

---

## Boundaries (what this chunk does NOT do)

- **No `.rs` / backend change.** No new TauRPC procedure, capability JSON, `pulse://` topic, arch §Occupied-Resources entry, or query change. (Consistent with the deferred cargo-gate note — but see "Gate note" below.)
- **No pagination / cursor work.** The latent viz `next_cursor` keying (still `ts_unix_nano`=start after P-068's `ORDER BY end_time`) is a SEPARATE deferred CARRY owned by a future pagination-touching chunk; the internal-scroll keeps `cursor=null` (single page) so it is untouched here.
- **No data-fetch / re-poll change.** The P-081 1s auto-refresh, the P-068 anomaly-first order, and the Errors-only filter behaviour are unchanged — this chunk only makes the hero + toolbar stay put and stops the labels overlapping.
- **No compact-widget constellation change.** D3 collision-avoidance is dashboard-hero-scoped; the widget remains aggregate-glance (P-069 boundary).
- **No new empty/error-state work** (that was P-071); no ConnectionStatusLine change (P-070).

## Surfaces / contracts touched

- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — internal-scroll container (D1) + the `announce`-in-updater fix (D4).
- The Traces-route layout shell that stacks constellation hero → toolbar → table (D1 fixed-hero behaviour) — likely the traces route index / a layout wrapper.
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (+ any label-layout module / `constellation-types`) — per-dot label collision-avoidance (D3).
- Design-token / CSS for the scroll region (`max-height`, `overflow-y`) — reuse existing tokens; no new token unless research shows a gap.
- `.andromeda/layout-templates.md` (Traces desktop wireframe) — D2 doc refresh (design artifact).
- Tests: `TraceTable.test.tsx` (scroll container DOM-shape + announce-not-in-updater), constellation label test(s) (`ConstellationCanvas.test.tsx` / `constellation-types.test.ts`), and the relevant a11y axe spec(s) (p1-traces / p11-constellation-semantics) for 0-new-violation regression.

## Verification intent (refined at P4/P5)

Pure-visual / render-layout + an a11y-correctness fix. Candidate NEW pure-visual cap (~P-082) for "Traces table internal scroll — fixed hero + toolbar" verified by webview DOM-shape (scroll wrapper has bounded `max-height` + `overflow-y:auto`; hero + toolbar outside the scroll region) + the p1/p11 axe sweeps (0-new) + the /implement P3 operator warm-boot VISUAL verify (jsdom `getBoundingClientRect`=0 is layout-blind). The D4 React-warning fix and D3 collision-avoidance are correctness/refinement; whether they each earn a distinct cap or fold under the layout cap is a P5 matrix-link decision. Affordance status: **pure-visual / render → affordance-EXEMPT** per verification-matrix-contract §Affordance honesty (no user-operated control is being newly wired; the Errors-only + sort controls already exist and are P-068-proven).

## Gate note (carried from the last wrap)

The prior frontend-only chunks deferred `cargo fmt`/`clippy`/`nextest --workspace`/`capability-drift` + the self-verify BOOT half. This chunk is also expected frontend-only (zero `.rs`), so the same deferral is expected to recur; the /implement P3 `cargo build -p pulse-app` re-embed is the de-facto compile check. If any `.rs` is unexpectedly touched, the full cargo gate + bindings.ts regen discipline re-engages.
