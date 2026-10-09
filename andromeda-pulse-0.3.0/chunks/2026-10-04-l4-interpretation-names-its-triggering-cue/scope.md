# Scope — L4 interpretation names its triggering cue

**Marker:** `2026-10-04-l4-interpretation-names-its-triggering-cue` · **Version:** andromeda-pulse-0.3.0 · **Taken up:** 2026-10-04 · **Mode:** taken up on a STANDING block (gated)

## Intent (working entry, verbatim title + hint)
L4 interpretation names its triggering cue — the L4 prompt marks corpus matches as past or other incidents and names
the triggering cue, measured on the restored real model.

## What this chunk builds
1. **Prompt framing — corpus matches are past or other incidents.** The digest's CORPUS MATCHES lines
   (`crates/triage/src/digest/assembler.rs:687-692`, rendered by `render_payload` inside `payload_summary`) are
   presented to the model as PAST or OTHER incidents, never as the incident being interpreted — by the digest text,
   by prompt template text describing it, or both (a P4 fork). `[premise-corrected: production passes "" as
   corpus_retrieval at every tier call (inference_runtime.rs:412-427) and at investigate_router.rs:248, so the
   prompt.rs "# Corpus Retrieval" sites :239/:329/:417 never render in production — corpus matches reach the model
   ONLY as the digest's CORPUS MATCHES section]`
2. **Prompt framing — the triggering cue is named.** The prompt names the cue that triggered this interpretation
   (its kind; the cue the incident's identity is taken from) as the subject the title, symptom and rank-1 hypothesis
   are about. The triggering cue is `digest.attention_cues.first()` (`pulse-app/src/inference_runtime.rs:815`); a
   cue-triggered digest carries exactly that one cue (`assembler.rs:259-272`); the composer receives it only as the
   digest's ATTENTION CUES text line plus its fingerprint among the citable ids — no structured parameter.
   Reflection digests carry no cue.
3. **A bounded names-the-trigger label on the dev probe.** `pulse-app/examples/l4_decision_probe.rs` records bounded
   labels only; it gains one bounded label saying whether the generated interpretation names the triggering cue —
   carrying no title text (the CONTEXT's stated need). [val-1 2026-10-04, intent-incomplete: the measurement the
   P4 forks settled (overseer, founder-delegated) also needs a d3-like corpus-carrying retry shape, a no-framing
   counterfactual arm and a `--min-rank1` verdict flag on the probe — the label alone cannot test the CONTEXT
   hypothesis without a counterfactual.]
4. **The measurement on the restored real model** — the probe run against the real GGUF, producing the
   names-the-trigger label for the retry-storm shape. **This is the gated part** (gate below).

## Gate
- gate: a Linux llama.cpp CUDA binary — none on this host yet (overseer measurement 2026-10-04 ~19:30; a founder desk
  act pending); since 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts this host reads GPU-present, so L4
  real mode routes to `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` (measured: `l4_decision_probe --arms A0 --n 1` prints
  `INCONCLUSIVE - ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH is unset`); the GGUF model itself is restored (CONTEXT); block
  wording per the wrap directive (overseer, founder-delegated, 2026-10-04) — **makes impossible now:** item 4, the
  real-model measurement of the names-the-trigger label (and any acceptance that reads a real generation). Items 1–3
  are doable now.
- Premise re-checked at take-up (2026-10-04T18:12Z): STANDING. No `llama-cli` on PATH; both
  `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` unset; `AI-Model/llama-b9305-cuda/` exists but every file in it is a
  Windows PE32+ image (`llama-cli.exe`, `ggml-cuda.dll`, `cublas64_13.dll` …) — the Windows-host build from
  2026-05-24 (`AI-Model/RESUME-NOTE.md`), not runnable here. The GGUF `AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf`
  is present.

## Folded freight (from the working entry)
- **CONTEXT (hypothesis, verbatim marker kept):** `[inferred]` "hypothesis: on Conductor's d3 run (2026-10-01) the
  model restated an older, differently-kinded active incident's title that the digest's CORPUS MATCHES put in front of
  it, so its rank-1 interpretation named no retry (inferred at 2026-10-04-retry-storm-interpretation-names-its-cause
  research §Half 1 from the Timeline's line shape and timestamps, n = 1 recorded generation; not measured — the digest
  and prompt were verified to carry the retry)". This chunk's remedy is premised on it; the real-model measurement
  (item 4) is what tests it.
- **CONTEXT (prior chunk's reach):** `2026-10-04-retry-storm-interpretation-names-its-cause` made the incident TITLE
  name the cause deterministically (`{cue_cause_label(kind)}: {model title}`), which leaves the model-authored
  symptom, timeline and ranked hypotheses — and Conductor v3-09, which grades the rank-1 hypothesis — unmoved. This
  chunk targets exactly those model-authored fields.
- **CONTEXT (model restore):** the real L4 model was absent on the Linux dev host until its restore from the backup
  disk on 2026-10-04 (overseer measurement ~19:30: `AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf`, sha256
  `6c1a2b41…28ff`, equal to `.andromeda/runs/2026-05-24T19-00-12-step0-spike/spike-result.md:7`, gitignored).
- **CONTEXT (probe constraint):** the dev probe `pulse-app/examples/l4_decision_probe.rs` records bounded labels only,
  so it needs a bounded names-the-trigger label (no title text) — item 3.
- **CONTEXT (ruling):** founder ruling 2026-10-04 (relayed by the overseer): its own WHAT-only entry, placed directly
  before "The declared Rust floor matches the code"; it is what Conductor v3-09 needs.
- **BLOCKED-ON:** folded as the Gate above.

## Boundaries
- The incident identity tuple `(kind, scope, scope_id)` and the deterministic title prefix are decided (arch
  §Established Decisions [Fault Identity]; CLAUDE.md Critical Warnings) — not reopened. This chunk changes what the
  model is TOLD, not how incidents coalesce or how the title prefix is grounded.
- No new port, TauRPC procedure, capability JSON, corpus table, MCP tool or env var is expected (verified at P3 for
  every remedy branch: template text, digest text and a probe label need none).
- Prompt template text stays ASCII (argv transport rule; `composed_prompt_templates_are_ascii_clean_for_argv_transport`
  in `crates/interpretation/src/prompt.rs`). The prompt byte ceiling (`MAX_PROMPT_BYTES` 16,384 B,
  `pulse-app/src/llamacli_inference.rs:76`) holds with the added framing — largest recorded primary prompt 6,979 B
  (d3); the exact delta is measured at implement.
- The probe label is bounded (a closed label set), never model text — NEVER-log discipline for model output / raw
  attribute values holds.
- Obtaining or building the Linux llama.cpp binary is the founder's desk act, outside this chunk.

## CI read at take-up (Setup 5a)
- `5d6e344` (the last wrap's flip commit = HEAD): **verdict not yet available** — ci#37223307977 (pull_request)
  in progress, checks 13/13 registered, 11 running, oldest the coverage gate at 216 s; secret-scan#37223308013
  completed/success. Not folded, not read as green.
