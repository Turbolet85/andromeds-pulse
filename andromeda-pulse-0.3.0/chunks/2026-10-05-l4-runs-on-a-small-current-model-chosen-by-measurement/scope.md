# Scope — L4 runs on a small current model chosen by measurement

**Marker:** `2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement` · version `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification
**Working entry (`working-route.md:170`):** "L4 runs on a small current model chosen by measurement — the L4 interpretation
model is a small, efficient, non-thinking current model, selected by a pre-registered real-model series"

## Intent
Replace the L4 interpretation model (today Llama-3.2-3B-Instruct-Q4_K_M under llama-cli b9305) with a current SMALL, efficient
model that runs without a thinking mode. A real-model series chooses it, and its pre-registration is written before the first
run. Pulse is a background helper, so footprint comes first. The founder's order of criteria is footprint (RAM/VRAM, CPU-only
latency), then no thinking mode, then quality (inputs#I1).

## What the chunk builds
- **Candidate acquisition.** Qwen3.5-4B · Qwen3.5-2B · Gemma 4 E4B · Gemma 4 E2B, the unsloth GGUF Q4_K_M builds, placed beside
  the baseline in the gitignored `AI-Model/` dir. Each licence is recorded from the model card at download.
  All four cards read `license: apache-2.0` (verified at P3 from the Hugging Face API, inputs#I3, which also records each
  Q4_K_M file's size and LFS sha256: Qwen3.5-4B 2,740,937,888 B · Qwen3.5-2B 1,280,835,840 B · Gemma 4 E4B 4,977,171,584 B ·
  Gemma 4 E2B 3,106,738,272 B; the baseline GGUF is 2,019,377,696 B). The download is checked against those sha256 values.
  A candidate whose card says otherwise at download is surfaced, never silently kept.
- **Load proof, per candidate, on the pinned b9305 llama-cli.** Each candidate is driven to a schema-constrained generation
  through the CUDA binary (`-ngl 99`) AND the CPU binary (`-ngl 0`). The b9305 tree names the architectures `qwen35`
  (`llama-arch.cpp:42`) and `gemma4` (`:59`) (inputs#I2). That is a name in a table, not a load: the overseer's correction
  says so and asks for each load to be proven (inputs#I1). A candidate that does not load is disqualified and recorded,
  never fixed by bumping llama.cpp. Keeping b9305 is this scope's boundary: arch §Established Decisions [LLM Inference
  Runtime] makes a build-tag bump a deliberate chunk-scoped event (verified at P3, arch extract).
- **Footprint measurement, per candidate and the baseline:** peak RAM, peak VRAM on the CUDA path, and CPU-only latency per
  generation, plus GPU latency for context. Measured on the dev host (RTX 3090, 62 GiB RAM, 32 threads).
  `[premise-corrected: the shipped argv passes no -c, so b9305 runs the model's training context shrunk only to fit free
  memory (common.h:427,451-456), and its "CPU route" is the CUDA build at -ngl 0 with KV-cache and op offload still on the
  GPU (common.h:553,556; inputs#I4)]` A footprint reading under today's argv measures the context the model was trained
  with, not the model. A CPU-only reading on this host measures a GPU-assisted route. So the series holds the context fixed
  and hides the GPU for CPU-only latency. Whether the fixed context also ships in the product argv is a P4 decision.
- **No-thinking proof, per candidate.** Every generation carries no reasoning block, and the non-thinking configuration is
  the one the product actually passes. `[premise-corrected: the mechanism is now read at b9305 source, the outcome stays
  unmeasured]` b9305 `llama-cli` prints any reasoning to STDOUT between `[Start thinking]` and `[End thinking]`, before the
  content (`tools/cli/cli.cpp:158-171`). The product's extractor takes the FIRST `{` in stdout
  (`llamacli_inference.rs:328`), so a thinking pass is a parse hazard, not just a token cost. `-rea off` sets
  `enable_thinking=false` for a template that supports it (`common/arg.cpp:3159-3175`; `cli.cpp:219`), and is a no-op on
  Llama 3.2's template. Whether a candidate thinks by default, and whether the `--json-schema-file` grammar (applied through
  `defaults.sampling`, `cli.cpp:88`) forecloses reasoning, are measured per candidate in the load leg.
  The proof reads the `[Start thinking]` marker.
- **Quality series.** A new pre-registration is written before its first run: the arms, n, the shapes, the selection rule,
  the thresholds, the held-out confirmation and the disqualification order. It runs on `pulse-app/examples/l4_decision_probe.rs`
  (S1–S6 shapes, the frozen strict grader, the stem reading). Selection and confirmation are separate slots, because a
  best-of-candidates pick carries selection optimism into its confirmation (below).
- **Ship the chosen model as the L4 default.** `[premise-corrected: no compiled-in default model exists — the identity is
  the GGUF file stem (llamacli_inference.rs:254-257), and the model reaches the runtime only through
  ANDROMEDA_PULSE_MODEL_PATH]` "Shipping" is therefore:
  - the argv constants `-c 8192` and `-rea off`, which ship whichever model wins (P4, overseer founder-delegated:
    a pinned context cuts the shipped footprint for every model; the switch is a no-op on templates without thinking);
  - the service doc naming the model and its argv (`.claude/docs/services/interpretation.md`);
  - the arch §Established Decisions [LLM Inference Runtime] record at wrap;
  - the overseer's `l4-env.sh` (inputs#I4), which lives outside this repo and is the overseer's to move.

  `ModelIdentity`, the load events and the `interpretation.model.load` field set need no change: the value follows the file.
  If the measurement selects no candidate, the series result says so, and what ships is a question for P4, not a default.

## Boundaries
- Small models only: bigger models are rejected (founder ruling 2026-10-05). The candidate list is the four named; adding
  another is an operator decision.
- No LoRA or fine-tuning: LoRA on Conductor scenarios comes later (founder ruling).
- No llama.cpp version bump and no runtime swap. The trait surface (`interpretation::LlmInferenceRunner`) and the D1
  spawn-per-generation shape stay, and any code delta lives at the binary boundary (verified at P3: arch §Established
  Decisions [LLM Inference Runtime], "only the concrete impl changes").
- No change to the L4 prompt framing or the frozen grader. The v2.5 lineage is the input the candidates are measured
  against. Chat-template or thinking handling belongs in the runner's argv at the binary boundary, never in the shared prompt
  composition (verified at P3: the arch sidecar entry `2026-10-04-l4-interpretation-names-its-triggering-cue`, "framing
  lives in shared composition, never in a runner impl"). If a candidate needs a different chat-template handling to answer
  at all, that is surfaced as a finding, never folded silently into the prompt.
- Real-model runs only in daytime (the host is a bedroom PC; heavy runs at night are off), and only inside the operator's
  granted model slot (inputs#I1).

## Folded freight (the entry's four CONTEXT blocks)
- **CONTEXT 1 — founder ruling 2026-10-05** (relayed by the overseer at the
  `2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape` wrap): replace the L4 model with a current SMALL
  efficient one chosen by measurement. Pulse is a background helper, so small footprint and no thinking mode. Bigger models
  are rejected, LoRA on Conductor scenarios comes later, and this entry was minted next, before pre-push:linux.
- **CONTEXT 2 — candidates:** Qwen3.5-4B · Qwen3.5-2B · Gemma 4 E4B · Gemma 4 E2B (unsloth GGUF Q4_K_M, Apache-2.0), against
  the baseline Llama-3.2-3B-Instruct-Q4_K_M. The entry recorded "b9305 loads qwen35 + gemma4" as the ruling's statement,
  unmeasured. **Corrected by the directive** (inputs#I1): that claim is the overseer's own grep of `llama-arch.cpp`
  (inputs#I2, re-derived at take-up: `qwen35` at `:42`, `qwen35moe` at `:43`, `gemma4` at `:59`). It covers arch names only,
  so each load is proven above.
- **CONTEXT 3 — the baseline's measured standing** (a measured claim from a prior chunk; its evidence pointer spot-checked
  still true at P3: that chunk's `evidence/series.md:10-11` reads `FAIL · rank1 34/40`, S1 9 · S2 9 · S3 7 · S4 9; `:86`
  the stem 36/40; `:42` the selection R1 37/40):
  on Llama 3.2 3B the v2.5 framing reads `FAIL · rank1 34/40` against the pre-registered 36 (S1 9 · S2 9 · S3 7 · S4 9;
  held-out S5/S6 20/20; record-only stem 36/40). Measured at `2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape`
  (`evidence/series.md`). The probe carries the S1–S6 shapes, the frozen strict grader and the stem reading. At the same
  chunk, the best of four single-run arms read 37/40 at selection and the same text read 34/40 on fresh confirmation
  (n = 10 per shape), "so a best-of-candidates pick carries selection optimism into its confirmation". Any series is a new
  pre-registration written before its first run. Real-model runs need the operator's model slot, since ports 4317/4318
  are shared with conductor-builder.
- **CONTEXT 4 — Conductor dependency:** Conductor's fourth v3-09 series grades the L4 framing and waits on this entry
  (founder ruling 2026-10-05). The Conductor-side BLOCKED-ON is the overseer's to move, never this chunk's.

## Second fold source — the CI verdict read at Setup 5a
- `4e5b595` (the last wrap's flip, = HEAD): **`CI 4e5b5959: verdict not yet available`**. It read `in progress · checks
  13/13` with run `ci#37271600158` in progress (12 checks running, the oldest `lint / test (windows-latest)` at 130 s) and
  `secret-scan#37271600189` completed/success. No wall-clock is available for an unfinished run. This is not folded as
  green and no red is dispositioned, because none was read. **Re-read at P3:** still `in progress · checks 13/13`, with
  one check running (the coverage gate, 839 s at the read) and secret-scan completed/success. Still no verdict; P5 re-reads it.
  **Re-read at P5:** `verdict: green · checks 13/13 · wall 1088 s` (`ci#37271600158` and `secret-scan#37271600189`, both
  completed/success). Nothing to disposition.

## Operational facts (directive, inputs#I1)
- The model slot and ports 4317/4318 are granted for this chunk, and conductor-builder stays idle.
- Heavy runs are daytime only.

## Out of scope
- pre-push:linux runs natively on Linux (the next working entry).
- Moving the Conductor-side marker.
