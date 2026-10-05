# Session Handoff

**Last Updated:** 2026-10-05T13:46:13Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-05-l4-model-chosen-by-pattern-discrimination — chunk wrap (six small models scored on pattern discrimination; the founder picked gemma-4-E4B-it-Q4_K_M)

## Position
- **Done:** `2026-10-05-l4-model-chosen-by-pattern-discrimination`, flipped `complete`.
  - 960 generations: six models over the pre-registered A / B / C shapes, the `gb` arm, each model's authors'
    sampling. The blind audit read 9/96 `agree`.
  - The rule's pick, `recommendation: gemma-4-E4B-it-Q4_K_M` (A-cause 49 %, a lower bound; B false alarm 0 %), is
    also the heaviest and slowest model (GPU p50 5.3 s). A record-only QAT q4_0 face-to-face read within run-to-run
    noise.
  - **Founder's pick 2026-10-05:** the unsloth `gemma-4-E4B-it-Q4_K_M.gguf` (sha256 `85a896a0…ab87`).
  - No product change; the shipped GGUF stays Llama-3.2-3B. CI on the pre-CI commit `c5c3e94`: `verdict: green ·
    checks 13/13`.
- **Next:** entry A, **"L4 runs the founder's pick with its authors' settings"** (`working-route.md:174`).
  - It ships the pick, with its authors' sampling, a GBNF and thinking off. The GBNF needs the founder's own word at
    its phase.
  - **One founder question for its phase:** the pick's GPU p50 (5.3 s; 5.2 s fresh) exceeds the 5 s gpu-primary
    budget.
  - Conductor v3-09 waits on it. Real-model runs need the operator's model slot.
- Then the newly minted "Model observations without a cue surface quietly" (founder ruling 2026-10-05), "Without a GPU,
  L4 analysis is programmatic", and pre-push:linux, which closes Epoch 4 (65 entries; the no-split ruling holds).

## Work done
- Implement, the operator-directed QAT leg and the operator pass (pre-CI commit, push, CI) ran in one window; then
  this wrap.

## Drift resolved
- 5 amendments across 3 docs, 0 escalations. The other four docs read `proposals: []`.
  - arch [LLM Inference Runtime] Model and the [Fault Identity] tail: the model is chosen, the founder's pick named.
  - security-plan: the observed L4 prompt maximum re-based 7,405 → 7,575 B at two sites, in lockstep.
  - test-plan §1: the probe's pins moved 29 → 61.
- One leaf re-derived: `docs/services/interpretation.md`.

## Notes
- **Founder rulings, applied at this wrap's route-resolve:** the cueless-observation entry is minted; the "digest
  carries baselines and a short trend" entry is NOT minted (slow patterns sit in the 0.4.0 incubator).
- **Host artifacts outside the repo:** the QAT GGUF is in the gitignored `AI-Model/`; the QAT run dir was copied to
  `~/dev/data/l4-model-comparison-2026-10/`.
- **Ports:** 4317/4318 are shared with conductor-builder; ask the operator for the model slot before any real-model
  run.
- **Host:** Omarchy Linux. The cwd guard blocks a leading `cd`; use absolute paths. grep is ugrep.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines 52–125;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - `matrix.py` `UNPARSED: P-072 — legacy notes placement`.
- **Still open, carried:**
  - the env-var registry-completeness playbook rule proposal;
  - obs-plan §8 has no row for `interpretation.hardware.detect`;
  - the `contract.jointly-contradictory-instructions` evolve record;
  - the `sidecar.py` Ref defect relayed to overseer1.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:** none deferred — four candidates rejected below the bar (`.andromeda/runs/2026-10-05T13-31-12Z-wrap/curation.md`).
- **Still open from prior wraps:**
  - the evidence path-scan sweep hazard;
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
Completed normally at 2026-10-05 16:22:29
