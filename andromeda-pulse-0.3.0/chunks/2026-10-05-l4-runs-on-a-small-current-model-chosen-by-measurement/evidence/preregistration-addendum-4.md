# Pre-registration addendum 4 — NVIDIA Nemotron 3 Nano 4B joins the series

**Written (UTC):** 2026-10-05T08:43:36Z — before the Nemotron download and every Nemotron generation. Its out dirs must carry a
later timestamp.

**Base:** `preregistration.md` + `preregistration-addendum.md` + `preregistration-addendum-2.md` +
`preregistration-addendum-3.md`. All of them stand unchanged.
**Authority:** founder ruling 2026-10-05, relayed by the overseer (founder-delegated), inputs#I7, verbatim:
«Давай все же еще немотрон потестим».

**Timing:** written after the Qwen3.5-2B confirmation and the gemma-4-E2B comparator had both finished (neither was
disturbed: the Nemotron download and its runs come after them), and before the Nemotron download and every
Nemotron generation.

## The sixth model

- **File:** `unsloth/NVIDIA-Nemotron-3-Nano-4B-GGUF` → `NVIDIA-Nemotron-3-Nano-4B-Q4_K_M.gguf`. HF API read
  2026-10-05: size 2,900,295,712 B, LFS sha256
  `e515c9ceb10ae503db22a201fade92167f757510d1247a65987f8b6ae9e296e7`, repo sha
  `8e81be55c5aa3d63bb82b6ceec62d50805d9e1bb`. Downloaded into the gitignored model dir and checked against that
  sha256 before any run (`evidence/acquisition.md`).
- **Licence:** `nvidia-nemotron-open-model-license` (card `license: other`), NOT Apache-2.0. Recorded in the table;
  per inputs#I7 it is a distribution question for the founder only if this model wins.
- **Architecture:** `nemotron_h` (hybrid Mamba-Transformer), listed in b9305 `src/llama-arch.cpp:88`. That is a name
  in a table, not a load: the CUDA load leg proves the load.
- **Thinking off:** ON by default per the model README (inputs#I7). Passed as `--chat-template-kwargs
  {"enable_thinking":false}` plus `-rea off` (shared argv). The `thinking` field measures it.
- **Sampling:** `--temp 0.6 --top-p 0.95 --top-k 40 --min-p 0`. The README gives no non-reasoning value. The
  tool-calling pair (0.6 / 0.95) is used as the closest class to grammar-constrained structured output (inputs#I7).
  min_p 0 as for every model. **top_k 40 is chosen** on the same basis as Llama's (the card states none; 40 is
  llama.cpp's hardcoded default).
- **Grammar:** the same `--grammar-file l4-output.gbnf` (`gb` arm).

## Its legs (each fired once, after this write time; `--arms gb --gbnf … --sampling … --footprint`)

1. **CUDA load + no-thinking:** `--shapes S1 --n 2`. Out dir `target/l4-decision-probe/load-cuda-gb-nemotron-{UTC}`.
2. **CPU-only, GPU hidden** (record-only for G3, which is retired; still read for G1's CPU half as addendum 2
   keeps it): `ANDROMEDA_PULSE_HARDWARE_PROFILE=cpu_primary CUDA_VISIBLE_DEVICES=` (empty), `--shapes S1,S2,S3,S4
   --n 2`. Out dir `cpu-only-gb-nemotron-{UTC}`.
3. **Selection:** `--shapes S1,S2,S3,S4 --n 10`. Out dir `selection-gb-nemotron-{UTC}`. B stays 36 (Llama's reading
   in the selection slot; the slot is not re-run).

## The rule over six rows

Addendum 2's rule applies verbatim over all six rows (G3 retired; G1 with its CPU half; G2; Order by CPU-only-leg
max `peak_rss_kib`; S with B = 36):
- If Nemotron becomes the pick (a survivor ordered before Qwen3.5-2B with selection rank1 ≥ 36), it needs its own
  fresh confirmation (S1–S4 × 10 with `--min-rank1 36`, plus S5/S6 × 10) before it can ship.
- Otherwise, if its selection rank1 ≥ 36, it runs a fresh RECORD-ONLY confirmation (S1–S4 × 10 without a verdict
  flag, plus S5/S6 × 10) beside Qwen3.5-2B and gemma-4-E2B, so the founder sees three confirmed rows.
- Below 36, it gets no confirmation run.

The final table carries: model · selection rank1 · confirmation rank1 · S5/S6 · GPU p50/max · RSS · VRAM · CPU
p50/max · licence (inputs#I7 §5).
