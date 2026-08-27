# Session Handoff

**Last Updated:** 2026-08-27T22:11:01Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 35 ahead after this wrap's commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-27-report-window-copy-affordance): the report Copy control copies, and a headful gate presses it`

## Position
- Done: **2026-08-27-report-window-copy-affordance** — the report window joined `clipboard.json`'s `windows` list (one label, no permission added, read-ban preserved) and a 16th headful stage `report-copy` presses the REAL control. Proven red→green on the same stage: RED at HEAD `copy_pressed:true / copy_state:"error"` → GREEN `copy_state:"copied"` with the live region announcing it. The RED reading is the dead-affordance signature — the control was FOUND and PRESSED successfully and only the effect field showed the write was dropped, so a "click didn't throw" check passes there.
- Next (first markerless): **Incident persist-vs-resolve write race** — carries the audit PREREQ (pin #17, next interval point **52**; session 51 is a between-point: re-verify basis+overlap, record `probe skipped per ratified interval (next: 52)`).
- Then: Ingest consumer initiating freeze · Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting + harness-truth sweep · Staged-bindings assertion · Metrics label surface · **ACL-rejection logging** (NEW, appended last this wrap).

## Work done
All 12 acceptance criteria MET. nextest **2026/2026 + 1 skip** (+2 = the two new xtask predicate pins) · vitest **821/821 across 79 files** (+8) · clippy 0 · a11y 37 passed with **0 new violation tuples**, Lighthouse 7/7 ≥90, pa11y 7/7 · self-verify PASS · capability-widening-check clean · check:ingest-progress PASS · `deny bans licenses sources` exit 0 · webview-drive **16/16 exit 0** · RED arm `--no-inject --expect-absent report-copy` exit 0 · capability-drift clean LAST after bindings regen. Boot smoke (capability JSON is a boot-path trigger): fresh data dir, release binary, feed asserted **0→3996 rows**, 0 panics / 0 ERROR, clean pid shutdown, ports released.

## Drift resolved
**8 applied (7 detector + 1 orchestrator-raised) across 2 masters · 0 escalations · drift = 0.** arch ×1 (§Stack GUI-harness Role cell 15 → 16 + `report-copy` enumerated) · test-plan ×7 (six count sites: §1 P5 · §2 invariants · §6 driver row · §6 P5 Status · §6 full-coverage · §9 E2E row; PLUS the orchestrator-raised completion of the §6 P5 Status enumeration, still the 13-id list and still calling `widget-close` TERMINAL — a claim retired at 2026-08-24, which a count-only fix would have left self-contradictory). Cascade: `docs/stack.md` · `docs/tests-summary.md` · `rules/testing.md` ×2. Five docs clean.

## Notes
- **Curation:** T1 0 · T2 3 · T3 0 · deferred 1 · rejected 1. T2 = one EXTENDED in place (`rules/testing.md` 2026-07-05 boot-smoke entry gains the release-preference correction) + two new (`rules/security.md` strings-vs-ACL probe; `rules/verification-harness.md` browser.execute serialisation). Separately applied and NOT capped: the cascade-routed extension of `rules/verification-harness.md:117` (preserve-verbatim "leg is now FIFTEEN stages" → SIXTEEN). CLAUDE.md untouched at 156/200.
- **Coverage:** chunk claimed 0 caps; version stays **21/22 verified, P-075 pooled** (Conductor's).
- **Audit PREREQ (session 50):** BETWEEN-point — probe NOT re-run; basis + overlap re-verified first-hand (`bans licenses sources` exit 0; `advisories` exit 1 at the same **8 DISTINCT ids** 0189/0190/0194/0195/0204/0222/0253/0258, sixth consecutive identical). Recorded `probe skipped per ratified interval (next: 52)`.
- **THE finding worth carrying forward:** `xtask` resolves `target/release/pulse-app.exe` BEFORE `target/debug/…`, so `cargo build -p pulse-app` alone leaves `self-verify` and `webview-drive` measuring a stale binary — silently and invertedly (the leg runs, the log is clean, and it reports the OLD behaviour as current). Cost three post-fix legs here. Now curated into `rules/testing.md`.
- Audit trail: `.andromeda/runs/2026-08-27T21-51-58Z-wrap/` (+ phase run dir `2026-08-27T20-41-09Z-phase/`).
- Last failed command: none.

## Deferred learnings
- **Cap-deferred this wrap:** binding a headful selector to `data-testid` rather than the accessible name is correct when the control's accessible NAME CHANGES WITH STATE (the Copy button cycles "Copy markdown" / "Copying…" / "Copied" / "Copy failed — retry", so a name-bound selector is unstable by construction) — the sanctioned test-plan §6 exception. Deferred on coverage-by-existing-text, not on value: §6 already states the accessible-name-first rule and the driver carries the nuance as a code comment.
- The pin-numbering chain on the audit PREREQ is now #17; re-derive the count from the route line, never carry it from memory.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes).
