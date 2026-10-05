# Session Handoff

**Last Updated:** 2026-10-05T16:20:19Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — chunk wrap (L4 ships gemma-4-E4B with its authors' sampling, a committed GBNF via `--grammar-file` and thinking off; the gpu-primary SLO raised to 10000 ms p99)

## Position
- **Done:** `2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings`, flipped `complete`.
  - The product argv carries `--grammar-file` (the committed `pulse-app/src/l4-output.gbnf`) and the authors' sampling;
    `unit_l4_grammar` runs the vendored b9305 converter and pins the GBNF.
  - RED on the chunk base: 4/4 `json_parse_failed`. GREEN: 7/7 parse ok, the real round trip PASS, the latency grader
    (now gradable) PASS at p99 6716 ms and FAIL at a 1 ms budget. CI on the pre-CI commit `bc1946d`:
    `verdict: green · checks 13/13`, `lint-test` green on all three runners (the Python proof).
  - **Conductor v3-09 is unblocked by this entry** (overseer, 2026-10-05); its marker is the overseer's to move.
- **Next:** the head entry **"L4 generation records render unredacted"** (minted this wrap, overseer direction):
  complete the `interpretation.constrained.generate` allowlist leaf (`raw_output_bytes`, `extracted_bytes`) and pin it.
  - The founder pauses the pair after this wrap; the head entry is the next phase's first take-up.
- Then "Model observations without a cue surface quietly", "Without a GPU, L4 analysis is programmatic", and
  pre-push:linux (now carrying CARRY (d): the `.ps1` grader run), which closes Epoch 4.

## Work done
- Implement, the RED/GREEN real-model legs, the operator pass (pre-CI commit, push, CI) and this wrap, in one window.

## Drift resolved
- 25 amendments across 4 docs (arch 10 · security 6 · tests 6 · obs 3), 1 escalation resolved: the boundary widening
  (the `--grammar-file` crossing + four sampling operands), ratified by the founder live 2026-10-05 («Утверждаю
  целиком», relayed verbatim). design / layout / a11y: `proposals: []`.
- The masters now count THREE product-written locations outside the data dir (the L4 grammar temp file, since #84).
- obs-plan §8's muted backlog re-opened for `interpretation.constrained.generate` (owner: the head entry).
- Leaves re-derived: CLAUDE.md overview + modules, `docs/stack.md`, `docs/services/interpretation.md`, the three
  summaries, `rules/{security,testing,observability}.md`.

## Notes
- **Owed, carried honestly:**
  - the `.ps1` grader run — CARRY (d) on pre-push:linux (installing `pwsh` needs the founder's sudo);
  - the redaction fix — the head entry above.
- **Host:** the overseer's `l4-env.sh` (inputs#I3) still names Llama-3.2-3B as `ANDROMEDA_PULSE_MODEL_PATH`; moving it is
  the overseer's. On a host that keeps it, Llama runs with the pick's sampling.
- **Ports:** 4317/4318 are shared with conductor-builder; ask the operator for the model slot before any real-model
  run. Host: Omarchy Linux; the cwd guard blocks a leading `cd`; a Write-tool hook blocks paths under `vendor/`.
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines 52–125;
  `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y); `matrix.py` `UNPARSED: P-072 — legacy notes placement`.
- **Still open, carried:** the env-var registry-completeness playbook rule proposal; obs-plan §8 has no row for
  `interpretation.hardware.detect`; the `contract.jointly-contradictory-instructions` evolve record; the `sidecar.py`
  Ref defect relayed to overseer1.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:** none deferred — four candidates rejected below the bar.
- **Still open from prior wraps:** the evidence path-scan sweep hazard; the selection-optimism reading; the
  plan-authoring operator-pass CHECK; the `producer | grep -q` under pipefail CHECK; the scope guard omitting new files;
  mutation applied?; run-dir hygiene trip; bindings clobber; a writer census at the wrong layer; targeted nextest
  `timeout` sizing; the implement report-step CHECK; the bindings-regen PIPELINE half; macOS `SystemTime` µs ticks;
  Windows `.ico` vs palette PNG; the deferral-destination generalization; `inject_demo --sustained` cannot form an
  incident.
