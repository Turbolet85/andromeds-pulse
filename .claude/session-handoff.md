# Session Handoff

**Last Updated:** 2026-10-05T06:14:08Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape — chunk wrap (the L4 framing reworded by measurement; confirmation FAIL rank1 34/40)

## Position
- **Done:** `2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape`, flipped `complete`. The two-slot
  pre-registered series ran on Llama 3.2 3B through llama-cli b9305 CUDA `-ngl 99`.
  - **Selection** (Slot 1, untouched tree), rank1/40: v2.4 baseline 28 · R1 (framing reworded) 37 · R3 (conventions
    sentence) 36 · R2 (schema-description sentence) 33. Under the fixed rule, **R1 shipped**; no combination ran.
  - **Confirmation** (Slot 2, fixed tree, v2.5): **`FAIL · rank1 34/40`** (S1 9 · S2 9 · S3 7 · S4 9). The held-out
    S5/S6 read 20/20; the record-only stem reading is 36/40. Recorded as measured, never as passed.
  - The evidence is the chunk's `evidence/series.md` and three `*-runs.json` files, recounted by the overseer.
- **Next:** "L4 runs on a small current model chosen by measurement" (`working-route.md:170`), minted this wrap on the
  FOUNDER RULING of 2026-10-05, relayed by the overseer.
  - Candidates: Qwen3.5-4B · Qwen3.5-2B · Gemma 4 E4B · Gemma 4 E2B (unsloth GGUF Q4_K_M, Apache-2.0), against the
    Llama 3.2 3B baseline. Small footprint, no thinking mode; bigger models are rejected. LoRA on Conductor scenarios
    comes later.
  - Its CONTEXT carries the baseline's 34/40, the selection-optimism reading (37 at selection, 34 on confirmation) and
    the Conductor v3-09 dependency. That "b9305 loads qwen35 + gemma4" is the ruling's statement, unmeasured.
  - Any series is a new pre-registration, written before its first run.
  - Then: "pre-push:linux runs natively on Linux" (three CARRY blocks), which closes Epoch 4 (61 entries; the no-split
    ruling holds).

## Work done
- `TRIGGER_FRAMING_INSTRUCTION` reworded and the prompt lineage moved to v2.5 / v1.4-fallback / v1.4-reflection.
- The probe gained the R1/R3/R2 arms and their combinations, `--shapes`, the held-out S5/S6 and a stem grader.
- Workspace nextest is 2639 (+9 pins). The clippy deferral PREREQ is closed.
- The operator pass committed `febe375`; its CI read `verdict: green · checks 13/13`.

## Drift resolved
- 6 amendments across 3 docs, 0 escalations.
  - arch §Established Decisions [Fault Identity]: the reworded framing, the lineage, and the two-slot measured effect;
    the owner is now the model-replacement entry.
  - security-plan :139 / :461: the observed maximum moved 7,185 → 7,405 B (~2.21×), in lockstep.
  - test-plan §1 (probe pins 8 → 16) and §4 (lineage plus the obligation pin).
- The other four docs read `proposals: []`. The sweep ran 11 patterns, every control fired, and no leaf changed.

## Notes
- **Conductor:** v3-09 now waits on the model-replacement entry; with the FAIL its BLOCKED-ON does not clear here. The
  Conductor-side marker is the overseer's to move.
- **Ports:** 4317/4318 are shared with conductor-builder; ask the operator for the model slot before any real-model
  run. This chunk's slot was used 23:53–00:08Z and released, with the GPU back to idle.
- **Host:** Omarchy Linux.
  - The Bash tool's cwd persists, and the cwd guard blocks only a LEADING `cd`. A `cd` inside a loop moves the
    session cwd (curated this wrap, host-win32.md).
  - grep is ugrep. Use the Write/Edit tools for documents.
- **Plan defects recorded in the report (no amendment owed):** the plan listed only the hygiene entry as an operator
  entry. The pre-push stages, the regen and the close were fired in the precedent plan's exact text
  (`2026-10-04-declared-rust-floor-matches-the-code` entries 25–33).
- **Not measured, carried as such:** the fallback and reflection prompt sizes after the +220 B rewording (the probe
  composes primary only).
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines 52–125;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - matrix `P-072` UNPARSED.
- **Still open, carried:**
  - the env-var registry-completeness playbook rule proposal;
  - obs-plan §8 has no row for `interpretation.hardware.detect`;
  - the `contract.jointly-contradictory-instructions` evolve record;
  - the `sidecar.py` Ref defect relayed to overseer1.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:**
  - the selection-optimism reading (best of four single-run arms 37 → 34 on confirmation) scored exactly 0.6 at
    curation. It is carried instead by the new route entry's CONTEXT;
  - a plan-authoring CHECK: list the operator pass's pre-push stages, regen and close as plan entries.
- **Still open from prior wraps:**
  - the `producer | grep -q` under pipefail CHECK;
  - the scope guard omitting new files;
  - mutation applied?;
  - the sweep hazard;
  - run-dir hygiene trip;
  - bindings clobber;
  - a writer census at the wrong layer;
  - targeted nextest `timeout` sizing;
  - the implement report-step CHECK;
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
