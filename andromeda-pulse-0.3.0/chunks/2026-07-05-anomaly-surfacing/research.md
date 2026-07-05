# Codebase Research — 2026-07-05-anomaly-surfacing

## Scope
- **Depth:** deep (mature surface; P-068 is a PARTIAL-capability-exists chunk) · **Reads:** 9 · **Globs/Greps:** 3
- **Headline:** the Traces surface is already substantially built. **Error flagging is DONE**; the real gaps are **(1) default anomaly-first ordering** and **(2) an errors-only filter control.** This is the 2026-06-01 "inverse-of-hybrid" pattern (plan assumes it must CREATE X; part of X already ships).

## Files inspected
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (full) — the table component. `TraceRowView` ALREADY flags error rows: `const isError = row.error_count > 0` → `borderLeft: "3px solid var(--color-accent)"` + a `✗` glyph (`aria-hidden`) + the numeric count, `data-testid={isError ? "trace-row-error" : "trace-row-ok"}`. Non-error → `0` in `--color-text-tertiary`. **Non-color-only flagging is already satisfied.** Sort state is local `useState<SortState>(SORT_STATE_NONE)`; `sorted = useMemo(() => sortRows(rows, sortState), …)`. No filter control. Sort changes already announce via `useStatusAnnouncer`.
- `pulse-app/ui/src/dashboard/routes/traces/sort.ts` (full) — the sort model. `SortColumn = "trace_id" | "service" | "duration_ms" | "error_count"` (error_count IS already a sortable column). `nextSortState` cycles none→asc→desc→none. **`SORT_STATE_NONE = { column: "trace_id", direction: "none" }`; `sortRows` returns `rows.slice()` (unchanged query order) when `direction === "none"`.** So the DEFAULT view is query order, which does not surface errors first — the core gap.
- `pulse-app/ui/src/dashboard/routes/traces/use-traces.ts` (full) — data hook. Calls `traces.query({ time_window_seconds, limit, cursor })` → `PaginatedResponse<TraceRow>`; `TraceRow.error_count` already present. No change needed (consume as-is).
- `pulse-app/ui/src/dashboard/routes/TracesRoute.tsx` (full) — composes header (h1 + Investigate button) → `<ConstellationCanvas>` → `<TraceTable>`. **No footer / no filter toolbar exists** (layouts' "footer control band" is a template ideal, not built here). Query is `QUERY_WINDOW_SECONDS=60`, `QUERY_LIMIT=100`.
- `crates/viz/src/query.rs` (full) — `SELECT … ORDER BY ts_unix_nano DESC, trace_id LIMIT ?` (newest-first, NOT latency-ascending — the intent's "latency ascending" is stale/effective description). `error_count: if status_code == 2 { 1 } else { 0 }` → **`error_count` is a per-span 0/1 error FLAG, not an aggregate count.** `TraceRow` DTO already `Serialize + specta::Type` with `error_count: u32`. **No Rust change planned** (the anomaly signal already crosses the bridge).
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` (full) — 11 tests; `row()` factory already includes `error_count`; `renderWithProvider` wraps in `StatusLiveRegionProvider` + `InvestigationProvider`; uses `@testing-library/user-event` real DOM events; has the error-row flag test + sort-cycle + announce tests. This file is a MODIFY target (add filter + default-order tests).
- `pulse-app/ui/src/dashboard/StatusLiveRegion.tsx` (full) — `useStatusAnnouncer()` → `announce(message)`; single polite `role="status" aria-live="polite"` region; no-op outside provider. Reuse for the filter announcement (SC 4.1.3).
- `pulse-app/ui/src/dashboard/routes/logs/LogFilter.tsx` (full) — **the established in-repo filter pattern.** Severity chips are `<button type="button" aria-pressed={enabled} onClick=…>` inside `<div role="group" aria-label="Severity filters">`, styled with design tokens (enabled → `--color-raised-2` + `1px solid #4A90E2`; disabled → `--color-inset` + subtle border + opacity 0.6), `data-testid`. **Exact model for the "Errors only" toggle.**
- `pulse-app/ui/tests-a11y/helpers/v02-fixtures.ts` (grep hit only) — carries `error_count` service fixtures (mixed healthy+erroring) for the a11y suite.

## Graph impact (code-graph)
- **Not queried — webview-only chunk.** The change is entirely TypeScript/React (`sort.ts` + `TraceTable.tsx` + their `*.test.*`); the Rust SCIP `tree.db` indexes Rust symbols only, so a query for these TS symbols returns nothing by construction. The Rust `TraceRow` / `query_traces` are consumed **unchanged** (additive-read only), so there is zero Rust blast radius. (Per cookbook: not-applicable is a real finding, recorded as such — no re-probe.)

## Patterns detected
- **Sort baseline** (`sort.ts:20,38-48`): `direction:"none"` returns `rows.slice()` verbatim — the hook point to redefine the unsorted baseline as anomaly-first.
- **Error flag render** (`TraceTable.tsx:178,228-245`): `isError` → accent left-border + `✗` + count; already non-color-only. Keep.
- **Filter control** (`LogFilter.tsx:88-110`): `<button aria-pressed>` toggle chips + `role="group"` + tokens + `data-testid` — the a11y-correct toggle pattern to mirror.
- **Announcer** (`TraceTable.tsx:38-44`): sort changes call `announce(...)`; the filter should announce likewise (SC 4.1.3).
- **Test harness** (`TraceTable.test.tsx`): `renderWithProvider` + `userEvent` real events + `row()` factory — extend for filter + default-order tests.

## Conventions to follow
- **Toggle = native `<button aria-pressed={on}>`** modeled on `LogFilter.tsx:88-110` (a11y §4 Button/Switch; NEVER `role=button` on a div). Focus ring `--border-focus` on `:focus-visible`; target ≥24px.
- **No new TauRPC / no new capability** — reuse `traces.query`; `TraceRow.error_count` consumed as-is (arch extract; frontend rule "capability discipline"). `xtask capability-drift` unaffected.
- **Design tokens only** (`design-tokens.md`): error hue `--color-accent` for border/icon/badge (non-text), body text `--color-text-primary`; toggle states per LogFilter tokens; no invented hex; no new motion.
- **Announce filter changes** via `useStatusAnnouncer` (`StatusLiveRegion.tsx`).
- **Webview tests** = co-located `*.test.tsx`, DOM-shape, REAL DOM event on the filter (affordance honesty); full standard gate set (webview gates fire — `pulse-app/ui/**` touched).

## New files to create
- **(none required)** — the errors-only toggle lands inline in `TraceTable.tsx` (single control; a separate `TraceFilter.tsx` mirroring `LogFilter` is optional over-structure for one toggle; plan defaults to inline).

## Files to modify
- `pulse-app/ui/src/dashboard/routes/traces/sort.ts` — redefine the `direction:"none"` baseline to hoist `error_count > 0` rows to the top (stable, preserving query order within each group); explicit column sorts still override.
- `pulse-app/ui/src/dashboard/routes/traces/sort.test.ts` — add anomaly-first-baseline tests.
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — add `errorsOnly` filter state + an `aria-pressed` "Errors only" toggle (in a small toolbar in the card, above the table) + apply filter in the `useMemo` (filter → sort) + announce on toggle.
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` — add: (a) default view puts an erroring row first with a mixed dataset; (b) real-DOM-event click on the toggle narrows to erroring rows only and back (affordance honesty — a dead toggle must fail).

## Open questions
- **Delta scope (P4 AskUserQuestion):** flagging already ships → confirm scope = **both default anomaly-first ordering + errors-only filter** (recommended; fully addresses F8's HIGH-PAIN "buried" intent + the matrix "filterable") vs filter-only (satisfies the literal "flagged AND filterable" but leaves the default view burying errors) vs ordering-only.
- **Ordering mechanism (resolve in plan; lean recommended):** anomaly-first as the *unsorted baseline* (no misleading sort indicator; click-to-sort still overrides) vs a default `error_count`-desc sort state (shows a ▼ on the Error column at load). Baseline approach recommended.
- **Filter shape (resolve in plan; lean recommended):** inline single `aria-pressed` "Errors only" toggle in the TraceTable card toolbar (mirrors LogFilter) vs a separate `TraceFilter.tsx` component. Inline recommended for one control.
