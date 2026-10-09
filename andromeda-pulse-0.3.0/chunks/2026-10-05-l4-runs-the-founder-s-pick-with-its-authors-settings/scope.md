# Scope — L4 runs the founder's pick with its authors' settings

**Marker:** `2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings` · version `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification
**Working entry (`working-route.md:174`):** "L4 runs the founder's pick with its authors' settings — the shipped L4 model is
the founder's pick from pattern discrimination, run with its authors' sampling, a GBNF and thinking off"

## Intent
The predecessor chose the model; this chunk ships it. The product's L4 subprocess runs the founder's pick, the unsloth
`gemma-4-E4B-it-Q4_K_M.gguf` (sha256 `85a896a0…ab87`; the host copy in the gitignored `AI-Model/` re-read `85a896a047553e84…`
at take-up), with the sampling its authors publish, thinking off, and its output constrained by a committed GBNF passed through
`--grammar-file` in place of today's `--json-schema-file`. The founder also ruled that the L4 GPU latency SLO is raised to a
figure this phase proposes from the measurements (inputs#I1).

## What the chunk builds
- **A committed GBNF generated from the shipped schema** (inputs#I1 ruling 1). Source: `crates/interpretation/src/schema.json`
  (sha256 `3c1071c2…7a` at HEAD — the same bytes the probe GBNF was generated from, per the predecessor addendum). Generator:
  llama.cpp b9305's own `examples/json_schema_to_grammar.py`, the converter that produced the probe's 44-rule grammar
  (inputs#I2, sha256 `7b5cc276…ac03`).
  - A fresh generation at HEAD reproduces inputs#I2 byte for byte (verified at P3: the converter, inputs#I4, run on
    `schema.json` exits 0 with 44 lines, sha256 `7b5cc276…ac03`, `cmp`-identical).
- **A test pinning GBNF == the schema conversion** (inputs#I1 ruling 1, verbatim requirement). A schema edit without a
  regenerated grammar must fail it.
  - The converter is a Python script outside this repository (inputs#I4: stdlib-only, MIT, ASCII), and CI runs the
    workspace tests on Linux, macOS and Windows with no Python step, so the test's form is a P4 fork (verified at P3).
- **The argv swap: `--json-schema-file {schema}` out, `--grammar-file {gbnf}` in**, in `build_llama_cli_args`
  (`pulse-app/src/llamacli_inference.rs`). A user grammar is never prefilled, which is why it runs gemma4 where the
  json-schema form fails (`Failed to initialize samplers`, exit 0, no JSON — working-entry CONTEXT 4).
  - The GBNF reaches disk at spawn the way the schema file does today: a per-spawn RAII temp file in
    `std::env::temp_dir()` (`SchemaTempFile`, `llamacli_inference.rs:704-736`; verified at P3). That write predates this
    chunk (chunk #84), so the GBNF adds no product-written location outside the data dir. The arch and security-plan text
    counting exactly TWO such locations misses it, which is a spec gap for the wrap.
  - Every production caller passes `L4_OUTPUT_JSON_SCHEMA` (`inference_runtime.rs:443`, `investigate_router.rs:257`), so
    the Investigate path inherits the swap by construction.
  - The schema text embedded in the PROMPT is unchanged (the probe's `gb` arm changed only the flag pair).
- **The authors' sampling in the argv:** `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0` (google `generation_config.json` +
  model card, the predecessor addendum's per-model table; working-entry CONTEXT 5). Today the product passes no sampling flag.
  - Whether sampling is bound to the model (keyed by the GGUF file stem, the only identity the product has) or a fixed
    constant for the one shipped model is a P4 fork (verified at P3).
- **Thinking off:** `-rea off` stays (already in the argv). The addendum measured 0 `<|think|>` occurrences in the composed
  prompt for gemma4, so no `--chat-template-kwargs` is needed for it.
  - The shipped prompt also carries none (verified at P3: 0 case-insensitive `think` in `prompt.rs` and `schema.json`).
- **The L4 GPU SLO raised** (inputs#I1 ruling 2): the gpu-primary budget in `xtask/ci/l4-latency-p99.{sh,ps1}` (5000 ms)
  and its homes. [premise-corrected: obs-plan §10 holds no L4 row; the homes are the script pair,
  `docs/v0_2_0/pulse-distillation-architecture.md` `:317 :531 :864`, and the obs-plan-amendments 2026-06-10 record.] Figure
  proposed at P4 from the measurements: p50 5346 ms / max 7285 ms on the pattern series (n = 160), p50 5187 ms / max 7.5 s on
  the fresh re-run (inputs#I1); overseer lean < 10 s.
  - The script gates a p99, so the figure is set against the tail; the handoff's "p50 exceeds the budget" compared a p50
    to a p99 budget. [premise-corrected: the grader is VACUOUS at HEAD — the latency emit (`inference_runtime.rs:476`,
    `:523`) carries no `hardware_profile`, the grader drops every sample without one and prints "gate trivially passes" at
    exit 0; its p99 is a floor index, not the project's nearest-rank rule; and nothing calls it. Whether this chunk makes it
    gradable is a P4 fork.]
- **The shipped-model naming follows the swap** wherever the docs name the shipped GGUF (arch [LLM Inference Runtime] Model,
  at the wrap).
  - The model is SELECTED by `ANDROMEDA_PULSE_MODEL_PATH`, set on this host by the overseer's `l4-env.sh` (inputs#I3, today
    naming Llama-3.2-3B), a file outside this repository; moving it is the overseer's (verified at P3).
    [premise-corrected: the product names the model by the GGUF's file stem (`llamacli_inference.rs:270-273`), so
    `interpretation.model.load` names the pick by construction. The `"llama-3.2-3b-instruct-q4_k_m"` strings in
    `crates/interpretation/src/contract.rs` are a doc-comment example of the name form (`:67`) and a test fixture (`:212`),
    not claims about the shipped model, and need no change.]
- **A real-model end-to-end read** on the CUDA route with the pick: the product (not the probe) generates parseable L4 JSON
  under the new argv, and the latency samples land under the new budget. Inside the granted model slot, daytime only
  (inputs#I1).

## Boundaries
- No llama.cpp bump: b9305 stays (upstream #29006 / fix PR #29066 unmerged as read 2026-10-05; working-entry CONTEXT 4).
- No change to the L4 prompt framing or to `schema.json` itself.
- The GGUF is never committed (gitignored `AI-Model/`).
- The cueless-observation surfacing ("Model observations without a cue surface quietly") and the GPU-less programmatic path
  ("Without a GPU, L4 analysis is programmatic") are the next two entries, not this one.
  - The CPU route keeps running whatever model the env names; it is not re-tuned here (verified at P3: one model path for
    both routes, `binary_target_for_profile`).
- The Conductor-side v3-09 marker is the overseer's to move (working-entry CONTEXT 2).
- Real-model runs only in daytime and only inside the granted slot; conductor-builder stays idle (inputs#I1).

## Folded freight (the entry's six CONTEXT blocks)
1. **Founder ruling:** this entry ships the founder's pick from pattern discrimination, not automatically Qwen3.5-2B; the
   framing "L4 runs Qwen3.5-2B…" is superseded.
2. **Conductor v3-09 waits on this entry**; its marker is the overseer's.
3. **The GBNF widens the product's subprocess boundary** and needed the founder's own word at this phase. Given:
   ratified (inputs#I1 ruling 1).
4. **The json-schema argv cannot run qwen35 or gemma4 on b9305** (`Failed to initialize samplers`, exit 0, no JSON; `-rea off`
   does not prevent it); a `--grammar-file` grammar is never prefilled (predecessor `evidence/series.md` §Leg 1 §Diagnosis).
   - Mechanism claim, measured-marked at the predecessor: the output-format grammar is prefilled with the template's
     generation prompt and rejects it. Re-derived at P3 at b9305 source: `--grammar-file` builds a `USER` grammar
     (`common/arg.cpp:1889-1893`), `--json-schema-file` an `OUTPUT_FORMAT` one (`:1905-1918`), and only `OUTPUT_FORMAT` /
     `TOOL_CALLS` are prefilled (`common/common.h:204-207`).
5. **Founder pick «Давай брать обычную тогда»:** the unsloth Q4_K_M over Google's QAT q4_0 — the rule's recommendation
   (A-cause 49 %, a lower bound; B false alarm 0 %); the face-to-face read within run-to-run noise; rank1 39/40 on the
   predecessor's GPU slot under `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`.
6. **Founder question for this phase:** GPU p50 5346 ms / 5187 ms (100/160 and 93/160 over 5000 ms), above the gpu-primary
   budget; the heaviest candidate (peak VRAM 3898 MiB, peak RSS 5.32 GB). Answered by inputs#I1 ruling 2: raise the SLO.

## Directive (inputs#I1) — folded
- Ruling 1 → the GBNF bullets above (commit, `--grammar-file`, equality test). The widening is ratified.
- Ruling 2 → the SLO bullet; the phase proposes the figure.
- Model slot and ports 4317/4318 granted; conductor-builder idle; daytime only.

## Second fold source — the CI verdict read at Setup 5a
- `7663dd91c449` (the last wrap's flip, = HEAD): **verdict not yet available** — `ci#37319659032` in progress at take-up
  (13/13 checks registered, 9 running, the oldest `boot smoke (ubuntu-22.04)` at 431 s); `secret-scan#37319659009` completed
  success. Not read as green; nothing to disposition yet. The predecessor's pre-CI commit `c5c3e94` read green 13/13
  (handoff).

## Gate
- none — the entry carries no `BLOCKED-ON`.
