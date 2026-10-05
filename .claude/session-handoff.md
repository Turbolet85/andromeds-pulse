# Session Handoff

**Last Updated:** 2026-10-05T09:55:30Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement — chunk wrap (argv `-c 8192 -rea off`; Qwen3.5-2B confirmed PASS 37/40; CPU route founder-retired)

## Position
- **Done:** `2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement`, flipped `complete`.
  - The product argv gained `-c 8192 -rea off`. The shipped model stays Llama-3.2-3B-Instruct-Q4_K_M.
  - The naming series confirmed Qwen3.5-2B `PASS · rank1 37/40` (bar 36, held-out 19/20). gemma-4-E2B and Nemotron
    were record-only at 40/40 + 20/20. The b9305 json-schema grammar-prefill trap blocks qwen35/gemma4 under the
    shipped argv; `--grammar-file` works.
  - CI on `5ac259e`: `verdict: green · checks 13/13`.
- **Next:** three entries minted by the founder's ruling of 2026-10-05, ahead of "pre-push:linux runs natively on
  Linux", in order:
  1. **"L4 model chosen by pattern discrimination"** (`working-route.md:172`). The founder-approved design is
     snapshotted verbatim (D7) at `.andromeda/runs/2026-10-05T09-41-57Z-wrap/relay-l4-pattern-discrimination-design.md`.
     Phase P1 snaps that copy with `inputs.py snap --message-file`. Real-model runs need the operator's model slot.
  2. **"L4 runs the founder's pick with its authors' settings"** — the founder's pick from P, not automatically
     Qwen3.5-2B, with sampling, GBNF and thinking off. Conductor v3-09 waits on it. The GBNF needs the founder's own
     word at its phase.
  3. **"Without a GPU, L4 analysis is programmatic"** — implements the CPU-route retirement that arch now records
     as founder-ruled.
  - Then pre-push:linux (three CARRY blocks), which closes Epoch 4. Epoch 4 now has 64 entries; the no-split ruling
    holds.

## Work done
- Resumed at P2 after the prior window ended past P1.
- Arch and test-plan amended; two leaves re-derived (`docs/stack.md`, `docs/services/interpretation.md`).
- Three route entries minted.

## Drift resolved
- 7 amendments across 2 docs, 0 escalations. The other five docs read `proposals: []`.
  - arch [LLM Inference Runtime]: the argv constants, the model, and the grammar trap.
  - arch: the CPU route marked founder-retired at four sites, owned by entry B.
  - arch [Fault Identity]: the remainder is now owned by the P and A entries.
  - test-plan §1: the probe pins moved 16 → 29.
- Expected amendments with no site, so no amendment was owed:
  - the `CUDA_VISIBLE_DEVICES` registry;
  - the security-plan argv row;
  - test-plan §4.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder; ask the operator for the model slot before any
  real-model run.
- **Host:** Omarchy Linux.
  - The cwd guard blocks a leading `cd`; use absolute paths.
  - grep is ugrep, and it rejects long `-o` context regexes, so use python windows.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines 52–125;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y).
- **Still open, carried:**
  - the env-var registry-completeness playbook rule proposal;
  - obs-plan §8 has no row for `interpretation.hardware.detect`;
  - the `contract.jointly-contradictory-instructions` evolve record;
  - the `sidecar.py` Ref defect relayed to overseer1.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:**
  - the evidence path-scan sweep hazard (`AI-Model/` with a trailing slash in prose trips the `/home/`-class scan),
    which scored 0.4 (task-specific);
  - the zero-generation vacuous-reading trap, which went to the P entry's CARRY instead.
- **Still open from prior wraps:**
  - the selection-optimism reading;
  - the plan-authoring operator-pass CHECK;
  - the `producer | grep -q` under pipefail CHECK;
  - the scope guard omitting new files;
  - mutation applied?;
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

## Session End Status
Completed normally at 2026-10-05 12:22:04
