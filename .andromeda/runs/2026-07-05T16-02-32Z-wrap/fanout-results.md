# Fan-out drift results — 2026-07-05-anomaly-surfacing

7 doc-agents, one per spec source. 6 clean, 1 proposal (routine-rejected).

| doc | result |
|---|---|
| arch | `proposals: []` — no new IPC/endpoint/event/port/env/crate; deps none; ORDER BY swap on existing `duckdb` SQL path honors [Database]/[ORM none] decisions; `query_traces` pre-existing tool. |
| security | `proposals: []` — no new input boundary (prepared stmt `?` unchanged, no new param); no auth/crypto/secret; deps none. |
| design | `proposals: []` — the one new UI element (Errors-only toggle) is `tokens ✓` (accent/raised-2/inset/border-focus, all existing tokens; no hardcoded hex). |
| layouts | **1 proposal — D-layout-surface (warning):** document the "Errors only" filter toolbar (above the table) in the Traces wireframe. |
| tests | `proposals: []` — all new paths tested at the unit tier (sort/TraceTable/viz/mcp); runners match §4 (nextest/vitest); harness/status/log unchanged. |
| obs | `proposals: []` — ORDER BY edits to EXISTING queries (existing `viz.query.traces` span); no new logger/OTel; no new logging of user data; PII vectors untouched. |
| a11y | `proposals: []` — the Errors-only toggle is a textbook native-button toggle covered by existing §4/§5 Button + §6 focus/not-color-alone + §7 aria-pressed/live-region; no schema change. |

## Resolution of the layouts proposal — routine-REJECT (playbook line 30–32)
The "Errors only" toggle is a **filter CONTROL added WITHIN the already-documented Traces surface** (the wireframe already documents the Traces view as filterable via the footer time-range/service filters). Per the playbook 2026-06-28 within-surface-refinement rule (generalized to all detectors): a control/refinement within an already-documented surface is **routine-REJECT** (its exact toolbar placement is below the wireframe's screen-region granularity — header/hero/table/footer — not a new wireframe *surface*). Precedent: 2026-06-28-investigate-actions-functional rejected documenting result/error panels as new wireframe surfaces (within-surface states).

**Handoff/CARRY note (not amended under this marker):** the Traces wireframe could be refreshed to show the "Errors only" filter toolbar above the table — bundle with the operator's Traces-layout CARRY #1 (internal-scroll) → the Traces-layout/polish home.

## Reconcile outcome
- **0 amendments applied · 0 escalations · 1 proposal routine-REJECTED (playbook-matched) · cascade no-op** (no spec body changed).
- No new playbook rule needed — the existing 2026-06-28 within-surface-refinement rule covers this.
