# Scope — Traces table auto-refresh

**Marker:** `2026-07-07-traces-table-auto-refresh`
**Version:** andromeda-pulse-0.3.0 · Epoch 3 — State honesty & legibility
**Promoted:** 2026-07-07
**Provenance:** first markerless working-route entry; originally filed at the 2026-07-05-legible-labeled-constellation (P-069) leave-running verify, PRIORITY-BUMPED at the 2026-07-05-constellation-severity-live-wiring (P-079) wrap when the operator hit it live.

**Working-route intent (verbatim):**
> Traces table auto-refresh — the Traces table's `viz.query.traces` runs once at mount (row_count 0 before data lands) and never re-polls, so it reads "No traces yet" / stale even as spans flow (the constellation polls every 1s; the table does not). Add a periodic re-poll / refresh. PRIORITY-BUMPED at the 2026-07-05-constellation-severity-live-wiring (P-079) wrap — operator hit it live (Traces stays empty while the constellation + incidents show live data; reads as broken for an observability app), same visible-brokenness class as the incidents-dropdown bug; originally filed at the P-069 leave-running verify. · CARRY: fold in the latent viz `next_cursor` keying (still `ts_unix_nano`=start after P-068's ORDER BY→`end_time` change) when pagination is touched.

---

## What this builds

The Traces table (dashboard route) refreshes its data on a live cadence so it reflects spans as they arrive, instead of showing the mount-time snapshot forever. On a fresh boot the table currently queries `viz.query.traces` once — when the buffer is still empty — reads `row_count 0` → renders the "No traces yet" empty state, and then never re-queries, so it stays empty/stale even while telemetry flows and the constellation + incidents surfaces show live data. This chunk adds a periodic re-poll (the constellation's live-refresh cadence, ~1s, is the in-app precedent) so the table transitions out of the empty state and keeps its rows current.

## Observed gap

- Fresh boot with telemetry flowing → Traces tab reads **"No traces yet"** indefinitely; the constellation hero + incidents panel on the same screen show live data. For an observability app this reads as broken.
- Root cause: the Traces table's data fetch (`viz.query.traces`, the P-068 anomaly-first aggregated query) runs **once at mount** and never re-polls. The initial query resolves before spans land (`row_count 0`), and there is no refresh trigger — no interval poll, no stream subscription, no manual refresh — so the snapshot is frozen at boot.
- Contrast: the constellation refreshes live (~1s); the table does not. Same **visible-brokenness class** as the incidents-dropdown bug (P-080) — a surface that silently never updates.

## In scope

- A live-refresh mechanism for the Traces table so its rows reflect current buffer state: re-invoke the existing `viz.query.traces` aggregation on a periodic cadence (mirroring the constellation ~1s precedent), transitioning the table out of the "No traces yet" empty state once data lands and keeping rows current as spans flow.
- Preserve the P-068 anomaly-first behavior across refreshes: default order still hoists erroring rows to the top; the "Errors only" filter state and `aria-pressed` survive a refresh (a re-poll must not reset user filter/sort state or cause row-flicker that defeats the anomaly-first read).
- Honest empty-vs-live states: "No traces yet" shows only while genuinely empty; once rows arrive the table populates without a manual reload.
- **A11y live-surface completion (planning-surfaced — /phase P5 val-1 intent-incomplete amendment):** actualizing the live trace list triggers the a11y-plan "P4 Live trace list" obligations the /phase a11y extract surfaced — a ONE-shot polite (`aria-live="polite"`) announcement on the empty→populated transition via the existing StatusLiveRegion (never per-tick, never assertive; SC 4.1.3), plus focus preserved across re-polls (SC 2.1.1/2.4.3, held by construction — no remount). Added because the a11y domain lists it as this chunk's acceptance; the strictly-minimal "re-poll only" reading remains available as an operator veto at the /phase P5 review.

## Out of scope

- **Metrics/Logs empty states** — that is P-071 (self-explaining empty states), a separate markerless chunk. This chunk touches the **Traces** table only.
- **Constellation refresh** — already polls; untouched.
- **Findings badge fetch-once-no-repoll bug** (`use-findings.ts`) — a sibling of this bug class, but CARRY-folded onto the separate Incidents floating-window disclosure chunk (per the 2026-07-06 handoff); not this chunk.
- New TauRPC procedures / new capabilities — `viz.query.traces` and its `pulse:default` capability already exist; a re-poll reuses them.

## Surfaces & contracts touched

- **Webview (primary):** the Traces table React component + its data-fetching hook (`pulse-app/ui/src/dashboard/routes/traces/` — the P-068 `TraceTable.tsx` surface and whatever hook drives its `viz.query.traces` call). The refresh cadence, lifecycle (start/stop on mount/unmount, no leaked timers), and filter/sort-state preservation live here.
- **TauRPC contract:** `viz.query.traces` (existing, read-only query router in the `viz` crate) — re-invoked, not modified.
- **CARRY (conditional, Rust `viz` crate):** see below — only if the refresh work touches the pagination/cursor path.

## Folded CARRY (conditional — Rust `viz` traces query)

The working entry carries: *fold in the latent viz `next_cursor` keying (still `ts_unix_nano`=start after P-068's ORDER BY→`end_time` change) when pagination is touched.*

- Latent bug: P-068 changed the traces query `ORDER BY` to `end_time`, but the pagination `next_cursor` is still keyed on `ts_unix_nano` (start time). A start-time cursor against an end-time ordering can skip/duplicate rows during pagination.
- **Condition:** the CARRY's own guard is "when pagination is touched." A minimal re-poll re-runs the first-page aggregation and does **not** touch pagination, so the default expectation is that this CARRY **re-defers with an explicit note** (it stays homed here / on the next pagination-touching chunk). It is folded into scope so it is not lost — the plan must **consciously decide** to fix it now (if the refresh design touches the cursor) or re-defer it, never silently drop it.

## Capability / matrix note

No P-NNN in the working-route line (this is an operator-surfaced bug, like P-079/P-080). Expect a **new capability minted at P5** (next id `P-081`) — "Traces table live refresh" — linked to this chunk in `verification-matrix.json`, with an affordance-honest acceptance (the table transitions empty→populated under live telemetry without a manual reload). Final id/acceptance decided at P5.

## Gate note

This is the **next `.rs`-touching chunk** *iff* the CARRY activates (Rust `viz` change). If the fix is webview-only (re-poll in the React hook), it is a frontend-only chunk like P-080 — but the deferred `clippy` / `nextest --workspace` / `capability-drift` re-runs (handed off from the P-080 wrap as due at the next `.rs`-touching chunk) come due here if any Rust is touched. Decided at plan time.
