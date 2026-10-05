# Session Handoff

**Last Updated:** 2026-10-04T23:26:00Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-l4-framing-measured-on-the-real-model — chunk wrap (the pre-registered L4 framing series measured: FAIL rank1 30/40, nf 18/40)

## Position
- **Done:** `2026-10-04-l4-framing-measured-on-the-real-model`, flipped `complete`. The pre-registered series ran once
  each on the real model, through llama-cli b9305 CUDA `-ngl 99`, routed by detection.
  - **shipped: `FAIL · rank1 30/40`** against the bar of 36. Per shape: S1 9 · S2 6 · S3 5 · S4 10.
  - **nf: 18/40.** Per shape: S1 6 · S2 6 · S3 1 · S4 5.
  - Recorded as measured, never as passed (P4 ruling). The evidence is the chunk's `evidence/series.md` plus the two
    `runs.json` copies (40 rows each).
- **Next:** "L4 rank-1 hypothesis names the retry on every storm shape" (`working-route.md:168`). This is the
  remedy, minted inside 0.3.0 per the 2026-10-02 founder ruling.
  - The overseer placed it ahead of pre-push:linux so Conductor unblocks soonest; the founder may move it.
  - Its CONTEXT carries the measured shortfall: S2 6/10 and S3 5/10 carry NO corpus match, while S4 reads 10/10. So
    the corpus-match restatement hypothesis does not account for the FAIL.
  - Its CONTEXT also carries Conductor's v3-09 dependency, plus `PREREQ: close rust gate deferral (clippy)`.
  - The series is spent: a re-measurement is a NEW pre-registration, written before any run.
  - Then: "pre-push:linux runs natively on Linux" (three CARRY blocks). It closes Epoch 4, which is at 60 entries;
    the no-split ruling holds.

## Work done
- No source change. The chunk folder carries `evidence/` and `report.md`.
- Nextest 2630/2630 (voided by walk-class, ran green). Clippy is deferred on zero Rust delta; the PREREQ is pinned.
- `inputs.py verify`: I1 n/a (message), I2 unchanged. It is quoted verbatim in the report, as the first live D7
  verify.

## Drift resolved
- 1 amendment: arch §Established Decisions [Fault Identity]. The UNMEASURED clause was replaced by the measured FAIL
  30/40 with the nf distribution, scoped to its measured boundary. Applied on the plan's expected amendment, matched
  by D-arch-decisions.
- The other six docs read `proposals: []`. Escalations: 0.
- Sweep: 7 patterns, all controls fired. The test-plan `:144` and `:385` rows are true claims, so no change; no leaf
  changed.

## Notes
- **Conductor:** its fourth v3-09 series now waits on the remedy entry. The Conductor-side BLOCKED-ON lives in the
  Conductor repo (0 marker hits in its `.md`) and is the overseer's to move.
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches
  pulse-app, a window or the model. The slot granted for this chunk was used 23:10–23:14Z and released, with the GPU
  back to idle.
- **Host:** Omarchy Linux.
  - `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently, and `grep -c` on a binary prints nothing
    (use python).
  - A PreToolUse hook blocks `cat >> file <<EOF` and a leading `cd`; use the Write/Edit tools and absolute paths.
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps. Select it by `comm == cargo` +
  `--message-format=json`; never `pgrep -f`. None was running this session.
- **Plan defects recorded in the report (no amendment owed):**
  - acceptance 1's `--out …/evidence/shipped` versus the entry's `target/` out dir plus a copy;
  - the shipped entry's `cargo build` runs before the env is sourced, so the triage build fetched a tokenizer.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines 52–125;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - matrix `P-072` UNPARSED (legacy notes placement).
- **Still open, carried:**
  - The playbook rule proposed at an earlier wrap (registering a PRE-EXISTING product-consumed env var as registry
    completeness).
  - obs-plan §8 has no row for `interpretation.hardware.detect`.
  - The `contract.jointly-contradictory-instructions` evolve record.
  - The `sidecar.py` Ref defect relayed to overseer1.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:** none.
- **Still open from prior wraps:**
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the scope guard omitting new files (phase Test Commands CHECK);
  - mutation applied? (implement's mutation-step CHECK);
  - the sweep hazard (phase research's bare-value CHECK);
  - run-dir hygiene trip (operator-pass CHECK);
  - bindings clobber (operator-pass CHECK);
  - a writer census at the wrong layer;
  - targeted nextest `timeout` sizing from a measured cold build;
  - the implement report-step CHECK;
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.

## Session End Status
Completed normally at 2026-10-05 01:47:47
