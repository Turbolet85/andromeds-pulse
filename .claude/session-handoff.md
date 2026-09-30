# Session Handoff

**Last Updated:** 2026-09-30T15:57:10Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-09-30-perf-budget-gate-reads-real-samples — chunk wrap (the perf-budget gate grades real samples; frame gate on the dev host)

## Position
- Done: `2026-09-30-perf-budget-gate-reads-real-samples`.
  - `xtask::perf_budget` (`cargo xtask perf:budget`) replaces the deleted `perf-slo-check` scripts. CI lint-test Linux
    feeds it the in-process `perf_budget_samples` producer and requires memory + snapshot (both PASS on CI).
  - Frame p99 is the dev-host `cargo xtask perf:frame-sample` (p99 2.6 ms, n = 7971). The hosted Windows runner gave
    0 frames (`ci#36723465727`), so on the operator's decision CI prints a named `frame: cannot-evaluate` line.
  - Release owns `release-${{ runner.os }}`; `agent-run.ps1` records `.spawn`/`.exit` (`ended` real on Windows);
    `app.boot.gpu.check` lost its hardcoded `gpu_available`.
  - Operator pass on the operator's word: pre-CI commits `d708ad7` (round 1 red — the frame reading) and `c6eb395`;
    `ci#36729367693` green both rounds; cache 10 605 172 169 of 10 737 418 240 B.
- Next (first markerless, minted this wrap on the relay's direction): **Perf instruments measure what their budgets
  name** — the snapshot timer (formatting only: 0 ms vs 61–76 ms) and an adapter-state record; it carries the
  release-cache `cache-on-failure` CARRY and the cache-headroom re-read.
- Then: Span-level redaction → Real-model incident surfacing → Conductor return (P-075).

## Work done
- New: `xtask/src/perf_budget.rs` (15 pins), `xtask/src/perf_frame.rs` (2 pins), `pulse-app/tests/perf_budget_samples.rs`
  (`--profile perf-samples`). Changed: `xtask/src/main.rs`, `ci.yml`, `.config/nextest.toml`, `scripts/agent-run.ps1`,
  `window.rs` + allowlist + pins. Workspace 2466/2466. No dependency, procedure or bindings change.
- Evidence: `chunks/2026-09-30-perf-budget-gate-reads-real-samples/evidence/` (red-at-base, slot-legs, operator-pass,
  frame-gate-decision, ci-cache).

## Drift resolved
35 detector proposals, all routine, 0 escalations (`.andromeda/runs/2026-09-30T15-36-30Z-wrap/fanout-results.md`).
- obs-plan (12): the perf gate is real; ONE p99 rule (nearest rank, quoted from the grader); frame gate on the dev
  host; memory gauge = rows × 256 B, no RSS gate; snapshot timer scope recorded as measured, owned by the minted
  entry; `app.boot.gpu.check` = `wgpu_backend` only.
- test-plan (13): `perf-slo-check-arm-coverage` DISCHARGED; the ps1 half of
  `harness-cleanup-verdict-and-boot-spawn-shell-coverage` WIDENED, not discharged (one by-hand run is a proof, not a
  committed test); §9 rows; `[profile.perf-samples]`.
- architecture (8): the two verbs, `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` (harness-set, child only), the ps1
  recorder, release cache ownership.
- security-plan (2): the `.spawn`/`.exit` state files join the harness-only class.
- Leaves re-derived: obs/tests/security summaries, rules `observability`/`security`/`testing`/`verification-harness`,
  `commands.md`, `workflow.md`, CLAUDE.md overview/modules/warnings.

## Notes
- Ports 4317/4318 are shared with conductor-builder: STOP and ask the operator for the slot before any window-opening
  run (`self-verify`, `perf:frame-sample`, the ps1 boot leg).
- The phase run's entry-#8 control fixture moved to `.andromeda/cache/phase-ctl-2026-09-30/` (gitignored) on the
  operator's instruction — a committed run dir cannot hold a `.rs` file under the hygiene check.
- Not re-run after the fallback edit: `self-verify` and `perf:frame-sample` (no product change; desktop held).
- Pre-existing tool verdicts, not this chunk's: `route.py` UNPARSED/INDETERMINATE on frozen lines (:52, :54 ×2, :60,
  :100, :116, :125); `matrix.py show` UNPARSED P-072 at `verification-matrix.json:161`.
- Epoch 4 grew by one entry; the operator's no-split ruling stands.
- Not this wrap (founder's hand): the `.gitattributes` re-checkout; the U35 door. PR #39 stays a draft.
- Still open: the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- Last failed command: none.

## Deferred learnings
- `recurrence-despite-learning`: the curated cache-allocation rule says owning keys save with `cache-on-failure`;
  the new release key omitted it and a red round saved nothing (fix pinned as a CARRY on the next entry).
- Still open from prior wraps: the implement report-step CHECK (unit-only claims vs a longer live run); the
  bindings-regen PIPELINE half; macOS `SystemTime` µs ticks; Windows `.ico` vs palette PNG; the deferral-destination
  generalization; `inject_demo --sustained` cannot form an incident.
