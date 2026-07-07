# Report — 2026-07-07-traces-table-auto-refresh

**Chunk:** Traces table auto-refresh — periodic re-poll of `viz.query.traces` so the table reflects live spans (no more stale "No traces yet")
**Date:** 2026-07-07
**Commits:** (uncommitted — wrap commits in P7; prior wrap's last commit was 48f1219 P-080)

## Changes (structured — detectors read this)
- **Files:** 4, all under `pulse-app/ui/src/dashboard/routes/traces/`:
  - `use-traces.ts` — factored `poll()`; added `window.setInterval(poll, 1000)` (const `POLL_INTERVAL_MS = 1000`, module-private) + a `window` `"focus"` listener + cleanup (`clearInterval` + `removeEventListener`). Silent background-refresh: on a post-load re-poll error the state is left unchanged (`prev.isLoading ? {error+empty} : prev`); first-fetch error path unchanged; `cursor: null` retained (page 1).
  - `TraceTable.tsx` — added a `useRef` + `useEffect` on `rows.length` emitting a ONE-shot polite `announce("Traces loaded")` (existing `useStatusAnnouncer`) on the empty→populated (0→>0) edge. `useMemo` filter/sort logic untouched.
  - `use-traces.test.ts` — +3 fake-timer tests (re-poll updates rows / interval cleared on unmount / last-good rows kept on re-poll fail) + a cursor-stays-`null` lock; `act` import added.
  - `TraceTable.test.tsx` — +4 tests (announces once / no re-announce / focus preserved / filter+sort survive a refresh); extracted a `treeFor` render+rerender helper.
- **Symbols / APIs:** NONE new/changed public. `useTraces(options)` signature unchanged. Re-poll reuses the existing `traces.query` (`viz.query.traces`) TauRPC procedure — no new procedure, no new capability JSON, no capability-drift. No new IPC methods, endpoints, exports, ports/sockets, or env vars. `POLL_INTERVAL_MS` is module-private (not exported).
- **Crates / modules:** NONE — webview-only; zero Rust crate touched. `crates/viz/**` untouched (the CARRY re-deferred).
- **Dependencies:** NONE added/bumped.
- **Schema / config:** NONE — no migration, no config key, no violation schema, no DuckDB table.
- **Coverage of new surfaces:**
  - `Traces table live-refresh` (existing P1/P4 surface, now live via 1s re-poll) → validation **n/a** (no new user input; reuses the typed `TracesQueryArgs`, `cursor: null`) · instrumentation **✓** (reuses the existing `viz.query.traces` span — proven at runtime: 190 spans at a clean 1s cadence during the boot smoke; no new frontend telemetry added, no browser OTel SDK) · PII **n/a** (no new logging; reuses the already-scrubbed `viz.query` path — `query_id` + `param_count` only) · tests **✓** (webview unit: `use-traces.test.ts` +3, `TraceTable.test.tsx` +4; a11y `p4-live-trace-list` + `p1-traces` axe) · a11y **✓** (SC 4.1.3 polite empty→populated announcement · focus preserved across re-poll · `aria-pressed`/`aria-sort` survive · error rows keep ✗ glyph + `--color-accent`; p4/p1 axe 0 critical/serious, regression 0/0 new) · tokens **n/a** (no new hex/px; Tables pattern + tokens unchanged).

## Deviations from intent
- **None substantive** — the 6 plan Implementation Steps landed as written; 0 fix-loop iterations.
- The **a11y SC 4.1.3 empty→populated announcement** was included per the **/phase P5 val-1 intent-incomplete amendment** (`scope.md` amended; the a11y extract surfaced it as this chunk's acceptance once the list becomes live) and operator-approved at the /phase P5 "Apply" gate — a recorded scope addition, not an implementation deviation.
- The working-route **CARRY** (viz `next_cursor` keyed on start-time vs P-068's `ORDER BY end_time`) **re-defers** per plan: the re-poll keeps `cursor: null` (page 1) so pagination is never exercised (`crates/viz/src/query.rs:17-18` already documents the latent caveat). Handed to a future pagination-touching chunk — no Rust change this chunk.

## Decisions & corrections
- Operator chose **Apply (Recommended)** at /phase P5 → include the a11y announcement (vs the "minimal re-poll only" option).
- Operator **leave-running visual verify PASSED** at /implement P3 — the Traces table populates (payment-service errors hoisted) + stays live, no longer stuck on "No traces yet".
- **Deferred gates** (zero `.rs` delta, continuing the P-080 webview-only deferral): `clippy` / `nextest --workspace` / `cargo xtask capability-drift` — due at the next `.rs`-touching chunk.
- **Pre-existing (NOT this chunk's fix), flagged for future cleanup:** `handleSort` / `handleToggleErrorsOnly` in `TraceTable.tsx` call `announce(...)` INSIDE the `setSortState`/`setState` updater — a "setState during render" React warning (pre-existing); and the existing "starts in loading state" test emits an act-warning (asserts pre-resolution state). My new announcement is correctly in a `useEffect` and does not share the anti-pattern. Candidate cleanup: move those two announces into their event handlers.
- **Smoke method:** used the operator-directed warm re-embed boot smoke (per `.claude/rules/testing.md` 2026-07-05) rather than `agent-run.sh`, because a Tauri compile-time-embedded webview chunk boots cold/stale under that harness.

## Outcome
- **Acceptance criteria: MET.** P-081 fully satisfied — re-poll updates rows (`traces.query` ≥2× on a fake timer), interval cleared on unmount (no leak), last-good rows on re-poll fail, empty→populated announces once (polite), focus + `aria-pressed`/`aria-sort` survive a refresh, `cursor: null` locked, p4/p1 axe green.
- **Gates green:** `npm run typecheck` ✓ · `npm run lint` ✓ · `npm run test` (vitest **703/703**, +7) ✓ · `npm run build` ✓ · `npm run test:a11y` ✓ (32 axe incl. p4-live-trace-list · lighthouse 7/7 ≥90 · pa11y 7/7 · regression-detector 0/0 new) · `cargo fmt --check` ✓. **Deferred (zero `.rs`):** `clippy` / `nextest --workspace` / `capability-drift`.
- **Smoke: PASSED** (warm re-embed boot). `cargo build -p pulse-app` re-embed 18.9s (`index-BAkjSWiR.js`) → boot 0 `app.panic.fatal` / 0 ERROR, `app.boot.webview.init`, 51,662 frames → **`viz.query.traces` = 190 at exactly 1s cadence (re-poll proven at runtime)** → storm/incident + all heartbeats healthy → operator visual verify PASSED → clean-quit, zero orphan, `:4317` released.
- **Matrix:** P-081 → `implemented` (ref: the vitest DOM locks + `p4-live-trace-list` axe + the operator P3 visual verify).
