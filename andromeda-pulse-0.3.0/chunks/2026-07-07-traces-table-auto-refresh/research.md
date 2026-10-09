# Codebase Research — 2026-07-07-traces-table-auto-refresh

## Scope
- **Depth:** moderate · **Reads:** 5 files · **Globs/Greps:** 3 (traces dir glob + useTraces consumers + viz cursor)
- **Verdict:** webview-only chunk. The primary fix is one hook (`use-traces.ts`); the CARRY (viz `next_cursor` keying) **re-defers** — confirmed by code, not assumed.

## Files inspected
- `pulse-app/ui/src/dashboard/routes/traces/use-traces.ts` (full) — **the bug + primary modify target.** One `useEffect` keyed `[options.timeWindowSeconds, options.limit]` (`:48,82`) fetches `getClient().traces.query({..., cursor: null})` (`:52-59`) exactly once, with a `let cancelled` cleanup guard (`:49,79-81`). No interval, no re-poll, no focus listener. `INITIAL_STATE.isLoading = true` (`:17-22`); success sets `isLoading:false` (`:62-67`); error sets `rows:[], error` (`:71-76`). Test seam `__setProxyForTest` (`:34-38`) mirrors the `canvas/frame-metrics.ts` pattern.
- `pulse-app/ui/src/hooks/use-service-constellation.ts` (full) — **the precedent to mirror.** `poll()` fn (`:32-41`) + `poll()` on mount (`:43`) + `window.setInterval(poll, 1000)` (`POLL_INTERVAL_MS=1000`, `:22,44`) + `window.addEventListener("focus", …)` (`:45-48`) + cleanup returns `clearInterval` + `removeEventListener` (`:50-53`). Every proxy call `.catch(() => {})` (`:38-40`) keeps the last value silently in jsdom / pre-init Tauri. Its header docstring names the PULL-only posture + the future `pulse://stream/*` push path (not wired).
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (full) — receives `rows: readonly TraceRow[]` + `isLoading` props (`:16-19,28`). Filter/sort state is component-local `useState`: `sortState` (`:29`) + `errorsOnly` (`:30`). The rendered view is `useMemo(() => sortRows(errorsOnly ? rows.filter(...) : rows, sortState), [rows, sortState, errorsOnly])` (`:34-37`) → **a `rows`-prop change re-derives WITHOUT resetting filter/sort**. `aria-pressed={errorsOnly}` (`:77`), `aria-sort={ariaSortFor(...)}` (`:110`). Empty cell `sorted.length === 0` → "No traces yet" (testid `trace-table-empty`, `:158-173`); loading cell `isLoading && rows.length === 0` → "Loading…" (`:141-157`). `announce = useStatusAnnouncer()` (`:31`) already fires on sort/filter changes (`:43-45,53`) — the existing polite live-region hook.
- `pulse-app/ui/src/dashboard/routes/TracesRoute.tsx` (full) — the consumer. `const { rows, isLoading } = useTraces({ timeWindowSeconds: 60, limit: 100 })` (`:21-24`) → `<TraceTable rows={rows} isLoading={isLoading} />` (`:67`) with **no `key` prop** → TraceTable is NOT remounted when `rows` changes (focus + local state preserved by construction). The SAME route renders `<ConstellationCanvas items={useServiceConstellation()} />` (`:25,66`) which already polls 1s — the visible asymmetry (constellation live, table frozen) the operator hit.
- `pulse-app/ui/src/dashboard/routes/traces/use-traces.test.ts` (full) — **test modify target.** `__setProxyForTest({ traces: { query: queryFn } })` + `renderHook(() => useTraces(...))` + `waitFor`. Asserts loading→resolved + `queryFn` called with `{time_window_seconds, limit, cursor: null}` + AppError capture. Extend with fake timers for the re-poll.

## Graph impact (code-graph)
- **Primary surface is TypeScript** — the Rust SCIP code-graph (`tree.db`) indexes Rust symbols only (rust-analyzer), so `useTraces` / `TraceTable` are not in it. Code-graph query **skipped for the primary** (per cookbook: query only when the changed symbols are Rust). Zero Rust blast radius.
- **CARRY symbol (deferred):** `crates/viz/src/query.rs::compute_next_cursor` (`:424-428`) keys `next_cursor` on `ts_unix_nano` (start time). `query.rs:17-18` ALREADY documents the caveat verbatim: *"next_cursor keys on start-time too — a latent pagination caveat only (the Traces route uses a single page, cursor=null)."* The re-poll keeps `cursor: null` (page 1) → the mis-keyed cursor is never exercised → **CARRY not touched this chunk; re-defers with note.** No Rust file changes.

## Patterns detected
- **Constellation poll lifecycle** (`use-service-constellation.ts:29-54`): mount-poll + `setInterval(1000)` + focus-refetch + `clearInterval`/`removeEventListener` cleanup + silent `.catch` keeping last value. Direct template for the `use-traces.ts` re-poll.
- **rows-prop re-derive, no remount** (`TraceTable.tsx:34-37` + `TracesRoute.tsx:67`): updating the `rows` prop re-runs the `useMemo`; `sortState`/`errorsOnly`/focus survive. Focus-preservation + filter-survives-refresh acceptances hold **by construction** — the chunk must NOT introduce a `key` or remount.
- **Silent-keep-last on re-poll error** (constellation `.catch(() => {})`): a transient poll failure must keep the last-good rows (no mid-poll blank/flicker), distinct from the FIRST-fetch error path which legitimately surfaces `error` + empty.
- **Existing polite announcer** (`TraceTable.tsx:5,31` → `dashboard/StatusLiveRegion::useStatusAnnouncer`): reuse for the empty→populated SC 4.1.3 announcement — no new live region.

## Conventions to follow
- **Fake-timer determinism** (`.claude/rules/testing.md` 2026-07-02): `vi.useFakeTimers({ toFake: ["setInterval","clearInterval",...] })` faking ONLY what's needed; advance with `await vi.advanceTimersByTimeAsync(1000)` so the pending `traces.query` promise flushes; restore in `try/finally` (vitest does not auto-restore timers).
- **React-19 effect double-invoke** (`.claude/rules/testing.md` 2026-05-08): assert `queryFn` re-invocation with `toHaveBeenCalled()` / `≥ N`, not exact `toHaveBeenCalledTimes(N)`.
- **jsdom is layout-blind** (`.claude/rules/testing.md` 2026-07-05): `getBoundingClientRect` = 0×0; "table visibly populates on screen" is NOT vitest-assertable — assert DOM/state (rows present, `trace-table-empty` gone, `aria-pressed` retained, `document.activeElement` unchanged) + an operator warm-boot leave-running visual verify at P3.
- **No new IPC/capability** (`.claude/rules/frontend.md` 2026-07-06 + arch extract): the re-poll reuses `traces.query` under `pulse:default` — zero capability-drift, zero backend change.

## New files to create
- none.

## Files to modify
- `pulse-app/ui/src/dashboard/routes/traces/use-traces.ts` — add a periodic re-poll (interval + focus, mirroring `use-service-constellation.ts`): factor a `poll()` fn; keep the first-fetch loading→loaded + error-on-fail semantics; on SUBSEQUENT polls do a silent background refresh (do NOT re-flip `isLoading` true; on a re-poll error keep last-good rows, branch on `prev.isLoading`); `clearInterval` + `removeEventListener` on cleanup (no leaked timer).
- `pulse-app/ui/src/dashboard/routes/traces/use-traces.test.ts` — fake-timer tests: `queryFn` re-invoked ≥2× across ticks; `rows` update lands; timer cleared on unmount (no further calls after unmount); a re-poll rejection keeps the last-good rows (no blank).
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — emit a ONE-shot polite `announce(...)` on the empty→populated transition (track prev `rows.length` via a ref; fire when `0 → >0`), reusing `useStatusAnnouncer` (SC 4.1.3). Filter/sort preservation needs NO change (already via `useMemo`).
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` — assert: empty→populated announces once (polite), not per subsequent row change; focus on the Errors-only button is preserved across a `rows` update (re-render); Errors-only + sort survive a `rows` update.

## Open questions
- **CARRY** — RESOLVED: re-defers (viz `cursor=null` single-page confirmed at `query.rs:17-18`); no Rust touched. Hand it forward to a future pagination-touching chunk (it stays a documented latent caveat).
- **a11y empty→populated announcement** — the a11y extract lists it as this chunk's acceptance (the chunk actualizes the "P4 live trace list" surface, so SC 4.1.3 applies). Included by default as the a11y-domain contribution; **surface at P5 for operator veto** if a strictly-minimal re-poll-only chunk is preferred.
- Poll interval — RESOLVED: 1s, matching the `use-service-constellation.ts` precedent + the scope's "~1s" + the layouts "one coherent live surface" note (`traces.query` p99 <150ms budget comfortably absorbs a 1s cadence).
