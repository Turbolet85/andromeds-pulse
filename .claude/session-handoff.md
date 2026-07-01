# Session Handoff

**Last Updated:** 2026-07-01T23:15:06Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-07-01-live-only-service-truth` — recency-gated live-only constellation + honest corpus-restore last_seen (P-067)

## Position
- Done: `2026-07-01-live-only-service-truth` (P-067) — the constellation shows only currently-live services (recency of honest `last_seen`, 60s window); corpus-restored/stale services hidden on BOTH surfaces (widget + dashboard); `set_state_on_corpus_restore` no longer clobbers `last_seen` to boot time. **P-067 verified → 11/18 v0.3.0 caps.**
- Next: **P-068 — Anomaly surfacing** (Epoch 3; errors/anomalies to the top of Traces, semantic error tokens, filterable · intent F8 · design-system) — the next markerless working-route entry → `/andromeda-phase`.

## Work done
Backend: `registry.rs::set_state_on_corpus_restore` preserves persisted `first_seen`/`last_seen` (+3 unit tests). Frontend: `LIVE_RECENCY_WINDOW_NANOS` + `isServiceLive` + `visibleDots`/`constellationSummary` now-gated (both `ConstellationCanvas` copies + tests, incl. a fake-timers quiet-drop test). Real-boot confirmed all 3 states (obs-log `item_count:7` constant while visible dots went 0→~6→0; 3 clean boots, 0 panics).

## Drift resolved
none — all 7 doc-agents returned `proposals: []` (behavior refinement within existing surfaces; no new IPC / deps / schema / surfaces). 0 amendments · 0 escalations · cascade no-op.

## Notes
- **Light-gate deferral (source-delta-proportional):** `nextest --workspace --profile ci` was NOT re-run at wrap — green at /implement (1721+1 skip), ZERO Rust-source change since (only 2 webview quiet-drop test files), triage delta re-proven by `nextest -p triage`, full compile by `clippy --workspace`, the binary re-exercised by 3 boot-observe boots + `self-verify`. Re-runs at the next Rust-source-touching chunk. All other chunk gates re-ran green.
- **HANDOFF — prior chunk gate-list gap:** `window.rs` carried a pre-existing `cargo fmt --check` drift from `2026-06-30-widget-to-dashboard-navigation` (its Test Commands omitted `fmt --check`); fixed in-chunk (fmt-only). testing.md 2026-05-10 already mandates the full gate set — the prior chunk just didn't list `fmt --check`.
- **Route:** P-070 got a `CARRY` — the ConnectionDot "Listening · just now"-on-zero-telemetry honesty, deferred from P-067's premise correction (that phrasing was intent F7's evidence but is the connection-status surface, not the constellation).
- **Curation:** T2 ×2 — `frontend.md` (two `ConstellationCanvas` copies → grep all copies before a shared-signature change) + `testing.md` (`vi.useFakeTimers({toFake:["Date"]})` + `rerender` proves now-recompute-each-render). 1 deferred (obs-log-count-vs-visual divergence, conf < 0.6). 0 conflicts.
- **Excluded from commit:** `andromeda-pulse-0.4.0-incubator/` (untracked out-of-chunk 0.4.0 planning material — left untracked). `free-disk.ps1` + routine committed separately (`ad7030f`).
- bindings.ts regenerated with mcp as the last pre-commit step (capability-drift clean). Branch local-only — **NOT pushed**. Last failed command: none.
