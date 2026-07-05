# Report — 2026-07-05-anomaly-surfacing

**Chunk:** Anomaly surfacing — errors/anomalies sorted/flagged to the top of Traces + semantic error tokens + filterable (P-068)
**Date:** 2026-07-05
**Commits:** none since last_wrap (chunk work uncommitted until this wrap)

## Changes (structured — detectors read this)
- **Files:**
  - `pulse-app/ui/src/dashboard/routes/traces/sort.ts` — anomaly-first *unsorted baseline* (`anomalyFirst` helper; `direction:"none"` stably hoists `error_count > 0` rows; explicit sorts unchanged).
  - `pulse-app/ui/src/dashboard/routes/traces/sort.test.ts` — updated the `none`-baseline test to anomaly-first + a stability test.
  - `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — `errorsOnly` state + `aria-pressed` "Errors only" toggle (toolbar above the table, modeled on `LogFilter`) + filter-then-sort + announce.
  - `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` — default anomaly-first-view + real-DOM-event filter + announce tests.
  - `crates/viz/src/query.rs` — `SELECT_TRACES` `ORDER BY ts_unix_nano DESC` → `ORDER BY end_time_unix_nano DESC` (completion-order) + regression test `query_traces_orders_by_completion_so_slow_erroring_spans_surface`.
  - `crates/mcp-server/src/tools.rs` — `SELECT_SPANS_RECENT` same completion-order change (consistency with viz).
- **Symbols / APIs:** NO new IPC procedure, endpoint, export, port, socket, or env var. `TraceRow` DTO unchanged (`error_count` consumed as-is). No TauRPC/bindings change → `xtask capability-drift` unaffected. Internal only: `anomalyFirst()` (sort.ts private), 2 new Rust `#[cfg(test)]` tests, 3 new webview tests.
- **Crates / modules:** changed — `viz` (traces query ordering), `mcp-server` (recent-spans query ordering), `pulse-app/ui` (Traces table feature). None added/removed.
- **Dependencies:** none added / bumped.
- **Schema / config:** none. No DuckDB table/column change, no config key, no violation schema. The change is query ORDER BY only (`spans` schema unchanged; `end_time_unix_nano` column already existed).
- **Coverage of new surfaces:**
  - `Traces "Errors only" filter toggle` (webview UI element) → validation n/a (client-side toggle, no new input boundary) · instrumentation n/a (client filter emits no backend log; the toggle announce is a11y not telemetry) · PII n/a (no attribute values logged) · tests webview (real `userEvent.click` DOM event + default-order + announce) · a11y ✓ (native `<button aria-pressed>`, Tab/Enter/Space, `--border-focus`; non-color-only flag preserved; jsx-a11y via `npm run lint`) · tokens ✓ (`--color-accent`/`--color-raised-2`/`--color-inset`, no hardcoded hex).
  - `viz traces query ordering (end_time DESC)` (backend read-path) → validation n/a (prepared stmt `?` unchanged, no new param) · instrumentation ✓ (existing `viz.query.traces` span; no new log) · PII ✓ (no query-param values logged, existing discipline) · tests unit (viz 39/39 incl. new completion-order regression) · a11y n/a · tokens n/a.
  - `mcp query_traces ordering (end_time DESC)` (feature-gated read-path) → validation n/a · instrumentation ✓ (existing) · PII ✓ · tests unit (mcp-server 78/78 with `--features mcp-server`) · a11y n/a · tokens n/a.

## Deviations from intent
- **Scope expanded webview-only → +viz +mcp query fix (RUNTIME-CORRECTS-INTENT; operator-approved).** The plan/scope assumed "the anomaly signal already exists in the data spine — surface it read-side." The operator's LIVE boot revealed the read-side query time-excluded the errors: `ts_unix_nano` is the span **start** time (`crates/buffer/src/appender.rs:54`), and `traces.query` ordered `ts_unix_nano DESC LIMIT 100`, so the 2500 ms-slow `payment-service` error spans (start ~2.5 s behind fast healthy spans) were ranked "old" and cut off by LIMIT → never reached the frontend → P-068's anomaly-first + filter had nothing to act on (filter returned empty). Fixed the real root cause: order recent traces by **completion** (`end_time_unix_nano DESC`) in both `viz` and `mcp-server` (operator chose this via AskUserQuestion; "also affects MCP query_traces" was in the approved option). Justification: without it P-068 is inert for exactly its target scenario (a slow erroring service).
- **`handleToggleErrorsOnly` written announce-before-`setState`** (not the functional-updater shape the first draft used) — avoids introducing a new instance of a pre-existing React `setState-in-render` warning in `handleSort`. In-scope quality choice.
- **Existing `sort.test.ts` "returns insertion order" test replaced** with the anomaly-first contract — a planned baseline-redefinition, surfaced at /implement Setup, not a silent test edit.
- **Gate deferral RESOLVED/CLOSED.** The chunk began webview-only (Rust gates deferred at /implement P2). It is now Rust-touching (viz + mcp), so the source-delta-proportional deferral closes: the full Rust light-gate re-runs at this wrap (`clippy --workspace`, `nextest --workspace --profile ci`, `xtask self-verify` boot half, `xtask capability-drift`).

## Decisions & corrections
- **Operator directive (standing):** run a FULL warm boot smoke at /implement P3 for EVERY user-visible-surface chunk INCLUDING webview-only (`pulse-app/ui/**`-only) — the `boot-smoke-coverage` path-scoping is EXTENDED to webview-only; the source-delta deferral targets the COLD full-workspace rebuild, NOT the warm app boot. Saved to project memory (`boot-smoke-webview-warm-reembed.md`).
- **Operator directive:** at that P3 — automated boot-confirm, then LEAVE the app running for operator-driven inspection, emit a short location-first look-here list + UX-judgment flags, STOP + WAIT (don't wrap until told).
- **Fix decision:** operator chose "order recent traces by completion time" (AskUserQuestion) over limit-bump / server-side-errors-first.
- **Learnings (for P3 curation):**
  - Tauri 2 compile-time-**embeds** the frontend (`generate_context!`), so a webview-only chunk needs a warm `cargo build -p pulse-app` re-embed (~43 s, deps cached) to boot-smoke the new UI; the existing binary + `self-verify` show the OLD frontend (verify via the embedded `index-<hash>.js`). `agent-run boot` = `cargo run --release` = COLD.
  - Windows cleanup: Git Bash `kill <winpid>` fails ("No such process" — MSYS≠Windows PID); use `powershell Stop-Process -Id <PID> -Force`.
  - viz gotcha: `ts_unix_nano` = span **start** time; `ORDER BY ts_unix_nano DESC LIMIT N` ranks slow spans (long duration = early start) as "old" and cuts them off — order recent-traces views by `end_time_unix_nano` (completion) instead.
  - Pre-existing `handleSort` `setState-in-render` warning (announce inside the `setSortState` updater) surfaced but left unfixed (out of chunk intent).
- **Operator-surfaced CARRY findings (→ P5 route-resolve):**
  1. Traces table internal scroll — the table scrolls the whole dashboard; it should have its own scroll (max-height + `overflow-y:auto`) so constellation+toolbar stay fixed. Pre-existing layout (NOT P-068); does not block P-068. CARRY → Traces-layout/polish home.
  2. `lint:a11y` Windows harness bug — `npm run lint:a11y --prefix pulse-app/ui` fails on Windows (inline `--rule 'jsx-a11y/…: error'` quoting); jsx-a11y IS covered by `npm run lint`; harness-script bug not a code violation. CARRY → harness/tooling-fix home.
  3. viz pagination cursor latent caveat — the completion-order fix changed ORDER BY to `end_time_unix_nano` but `next_cursor` still keys on `ts_unix_nano` (start); unused by the Traces route (cursor=null) → latent-only. CARRY → viz follow-up home.
- **Accepted as-is (no CARRY):** constellation dots single-color regardless of error = EXPECTED (health/severity color is P-069, already routed). C1–3 UX-judgment accepted live by the operator: silent anomaly-first baseline (no ▲/▼), toggle off-state opacity 0.6, "No traces yet" empty-state under errors-only.

## Outcome
- **Acceptance MET (operator-verified on a LIVE boot):** with real mixed data (4 healthy services + `payment-service` 100% errors), the payment error rows surface at the TOP of the Traces table (accent border + `✗` + count) and the "Errors only" toggle narrows to erroring rows / restores — after the completion-order fix let the error spans reach the table.
- **Gates green (per-crate at /implement):** webview `typecheck` + `lint` + **683 tests** + production build; `verify:contrast` 12 pairs; `viz` **39/39** (incl. the completion-order regression); `mcp-server` **78/78** (`--features mcp-server`); `viz` clippy clean. Full Rust light-gate (`clippy --workspace` · `nextest --workspace --profile ci` · `xtask self-verify` boot half · `xtask capability-drift`) re-runs at P7 (deferral closed).
- **Smoke:** real WARM boot verified TWICE (pre-fix + post-fix) via re-embed → boot → `inject_demo` (1377+ spans / 246+ payment exceptions) → obs-log confirm (0 panics · 0 ERROR · `app.boot.webview.init` · `duckdb.append` · `viz.query.traces` · incidents · heartbeats) → operator-driven live inspection → clean shutdown, zero orphan.
