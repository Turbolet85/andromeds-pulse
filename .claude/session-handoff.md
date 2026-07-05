# Session Handoff

**Last Updated:** 2026-07-05T16:21:51Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-07-05-anomaly-surfacing` — anomaly-first Traces ordering + "Errors only" filter + viz/mcp completion-order fix (P-068)

## Position
- Done: `2026-07-05-anomaly-surfacing` (P-068) — Traces anomaly-first ordering + "Errors only" filter, **operator-verified on a live boot**; and the viz/mcp **completion-order fix** (`ORDER BY end_time_unix_nano DESC`) that let the slow erroring spans actually reach the table. **P-068 verified → 12/18 v0.3.0 caps.**
- Next: **P-069 — Legible labeled constellation** (Epoch 3; per-dot service names + health/severity via design-system color + Halo · intent F9) — the next markerless working-route entry → `/andromeda-phase`.

## Work done
Frontend: `sort.ts` anomaly-first unsorted baseline (`anomalyFirst` hoist) + `TraceTable.tsx` `aria-pressed` "Errors only" toggle (filter-then-sort + announce) + their tests (683 webview green). Backend (operator-surfaced on a LIVE boot): `viz/query.rs` + `mcp-server/tools.rs` `ORDER BY ts_unix_nano DESC` → `end_time_unix_nano DESC` — the `ts_unix_nano`=span-START-time + LIMIT-100 was excluding the 2500ms-slow payment error spans, so P-068's frontend logic had no errors to surface (viz 39/39 incl. a completion-order regression; mcp 78/78). Verified twice on a real warm boot (re-embed → inject_demo → obs-log), clean zero-orphan shutdown.

## Drift resolved
1 layout proposal (D-layout-surface: document the "Errors only" toolbar in the Traces wireframe) **routine-REJECTED** per the playbook 2026-06-28 within-surface-refinement rule (a filter control within the already-documented Traces surface, below wireframe granularity) — homed to the new Traces-layout-polish route entry. 0 amendments applied · 0 escalations · cascade no-op. (arch/security/design/tests/obs/a11y all `proposals: []`.)

## Notes
- **Boot-smoke directive (operator, standing):** run a FULL warm boot smoke at /implement P3 for EVERY user-visible-surface chunk INCL. webview-only — the boot ≠ the deferred gate (the deferral is the COLD workspace rebuild). Curated to `testing.md` + project memory (`boot-smoke-webview-warm-reembed.md`); the HOW = warm `cargo build -p pulse-app` re-embed (Tauri compile-time-embeds the frontend).
- **Deferral CLOSED:** P-068 became Rust-touching (viz+mcp), so the source-delta-proportional deferral resolved — the full Rust light-gate re-ran at P7 (`clippy --workspace` · `nextest --workspace` · `xtask self-verify` boot half · `xtask capability-drift`).
- **Route:** CARRY #2 (lint:a11y Windows quoting bug) pinned to the P-077 housekeeping entry; new Epoch-3 markerless entry **"Traces table layout polish"** (internal scroll + wireframe-toolbar doc) for CARRY #1.
- **CARRY #3 (handoff — no near-term owner):** the viz `next_cursor` still keys on `ts_unix_nano` (start) after the ORDER BY→`end_time` change — **latent only** (Traces route uses `cursor=null`, single page); align the cursor with the completion-ordering when pagination is next touched.
- **Curation:** T2 ×1 (`testing.md` boot-smoke-webview-only) + T3 ×1 (`session-learnings.md` viz start-vs-completion ordering). 0 conflicts · 0 deferred.
- Two trace-query copies (viz + mcp) — grep both when touching trace ordering. Branch local-only — **NOT pushed**. Last failed command: none.
