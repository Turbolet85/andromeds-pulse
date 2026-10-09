# Scope — L4 framing measured on the real model

**Marker:** `2026-10-04-l4-framing-measured-on-the-real-model` · **Version:** andromeda-pulse-0.3.0 · **Taken up:** 2026-10-04T22:54Z (2026-10-05 local) · **Mode:** taken up with its block CLEARED (not gated)

## Intent (working entry, verbatim title + hint)
L4 framing measured on the real model — the trigger framing shipped at 2026-10-04-l4-interpretation-names-its-triggering-cue
is measured on the restored real model through a Linux CUDA llama-cli.

## What this chunk builds
1. **The pre-registered `shipped` run.** `l4_decision_probe --arms shipped --n 10 --min-rank1 36` on the real GGUF through
   the Linux CUDA `llama-cli`, under the leg env `inputs#I2`. PASS iff the rank-1 hypothesis names the retry in >= 36 of 40
   generations (shapes S1-S4, S4 carrying a d3-like corpus match); the probe prints
   `l4-decision-probe: names-trigger verdict: {PASS|FAIL} · rank1 r/n`. Its stdout and `runs.json` go to the chunk's
   `evidence/`. This is the chunk's acceptance: the gated criterion the predecessor chunk pre-registered and never
   recorded as passed.
2. **The pre-registered `nf` counterfactual.** `l4_decision_probe --arms nf --n 10` — the no-framing arm, record-only and
   unthresholded, run after the `shipped` run (which builds the example). Its stdout and `runs.json` go to `evidence/`.
3. **The reading of the CONTEXT hypothesis.** The `nf` vs `shipped` `names_trigger` distributions are recorded side by
   side as the measurement of the hypothesis below. A reading, not a second threshold — the pre-registered rule is
   item 1's alone.
4. **No product code changes.** The probe (`pulse-app/examples/l4_decision_probe.rs`, with S4, the `nf` arm,
   the `names_trigger` label and `--min-rank1`) and the trigger framing both shipped at
   `2026-10-04-l4-interpretation-names-its-triggering-cue`; this chunk runs and records them. [verified at P3: the
   probe at HEAD carries every flag, arm and shape the series names, both arms compose under the bound, and this host
   reads `GpuPrimary` (`/usr/lib/libcuda.so`) so real mode takes the CUDA binary at `-ngl 99` — research.md §Scope
   premise closure]
5. **A FAIL is a recorded outcome, never a re-run until it passes.** The rule was pre-registered before
   any run; a FAIL is recorded as measured, and any remedy is a new route entry, not a re-tuned threshold or a
   repeated series. (How a FAIL dispositions the chunk — complete-with-FAIL-recorded vs. held — is a P4 question.)
   [verified at P3 as a premise: the probe records a FAIL as exit 1 with its verdict line; the disposition stays P4's]
   [val-1 2026-10-04, decided at the P4 fork (overseer, founder-delegated): a FAIL COMPLETES the chunk with the FAIL
   recorded, never as passed; its remedy entry stays in 0.3.0 per the 2026-10-02 founder ruling; on FAIL the report
   names that Conductor's fourth v3-09 series grades this framing, so its BLOCKED-ON moves to the remedy entry; on PASS
   that series is unblocked]

## Gate
- gate: cleared — the block "a Linux llama.cpp CUDA binary" (BLOCKED-ON removed at the
  2026-10-04-declared-rust-floor-matches-the-code wrap on the overseer's directive). Evidence: the overseer's measurement
  and phase directive `inputs#I1` (binary `version: 9305 (63248fc)`); the leg env `inputs#I2`. Premise re-checked at
  take-up (2026-10-04T22:55Z), read-only: `~/dev/tools/llama.cpp-b9305/build/bin/llama-cli` is a Linux x86-64 ELF
  printing `version: 9305 (63248fc)` / `built with GNU 16.2.1 for Linux x86_64`, with `libggml-cuda.so` beside it;
  `AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf` is present, gitignored (`.gitignore:119`), sha256
  `6c1a2b41161032677be168d354123594c0e6e67d2b9227c84f296ad037c728ff` (= the `6c1a2b41…28ff` every prior series used).
- Model slot: the 4317/4318 slot is GRANTED for this chunk and conductor-builder stays idle until its wrap
  (`inputs#I1`, overseer, founder-delegated).

## Folded freight (from the working entry)
- **CONTEXT (pre-registered series)** — overseer, founder-delegated, 2026-10-04; written into the predecessor's plan
  before any run; never recorded as passed: first `--arms shipped --n 10 --min-rank1 36` (PASS iff rank-1 names the
  retry in >= 36/40, shapes S1-S4, S4 with a d3-like corpus match), then `--arms nf --n 10` record-only; both on the
  operator's model slot; stdout and `runs.json` into the chunk's evidence. Folded as items 1-2.
- **CONTEXT (hypothesis, verbatim marker kept):** `[inferred]` "it measures the CONTEXT hypothesis that the d3 model
  restated a corpus-match title" — the predecessor's hypothesis (inferred from n = 1 recorded generation, not measured).
  Folded as item 3.
- **CONTEXT (Conductor):** Conductor's fourth v3-09 series was blocked on the frozen line of
  2026-10-04-l4-interpretation-names-its-triggering-cue and now waits on this entry; it realizes the founder ruling of
  2026-10-02 (nothing moves to 0.4.0) without recording an unmeasured pass (overseer, founder-delegated, 2026-10-04).
- **CONTEXT (block cleared):** llama.cpp b9305 (63248fc) built with CUDA 13.3 at
  `~/dev/tools/llama.cpp-b9305/build/bin/llama-cli`; the overseer's smoke with this entry's exact argv: exit 0,
  schema-valid JSON, 155 t/s on the GPU; the leg's whole env is `inputs#I2` (exports `ANDROMEDA_PULSE_MODEL_PATH`,
  `_LLAMA_CUDA_BIN_PATH`, `_LLAMA_CPU_BIN_PATH`); since 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts this
  host reads GPU-present, so L4 real mode routes to the CUDA binary path. Folded as the Gate above; the routing claim
  is verified at P3 at HEAD (research.md §Scope premise closure).

## Boundaries
- The trigger framing, the digest, the probe's shapes/arms/labels and the pre-registered rule are not changed here. A
  change to any of them after seeing a result would void the pre-registration.
- Model output is never logged or committed as text: the evidence carries the probe's bounded labels and verdict lines
  (the probe records bounded labels only — NEVER-log discipline for model output).
- The real L4 paths are product-consumed env vars outside the data dir; their guard (`validate_path_input`, opt-in
  `ANDROMEDA_PULSE_L4_ALLOW_ROOT`) is unchanged. Committed evidence carries no full host path (hygiene) — basenames or
  `~`-relative spellings only.
- No new port, TauRPC procedure, capability JSON, corpus table, MCP tool or env var.
- The series runs once each, in the pre-registered order; any further run is a new pre-registration.

## CI read at take-up (Setup 5a)
- `4c9e05e` (the last wrap's flip commit = HEAD): **verdict not yet available** — ci#37241608600 (pull_request) in
  progress, checks 13/13 registered, 12 running, oldest lint / test (windows-latest) at 151 s; secret-scan#37241608626
  completed/success. Not folded, not read as green.
