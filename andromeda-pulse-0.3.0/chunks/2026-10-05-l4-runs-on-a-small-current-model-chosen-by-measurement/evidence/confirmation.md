# Confirmation — the pick under addenda 2–4, and the six-row record

`selection.md` stays as it read under the rule then in force: `pick: none`, by G3. This file records what came after:
- addendum 2 (08:35:02Z): G3 retired by the founder's hardware ruling, inputs#I6; PASS iff rank1 ≥ 36;
- addendum 3 (08:35:46Z): gemma-4-E2B as a record-only comparator;
- addendum 4 (08:43:36Z): Nemotron 3 Nano 4B added, inputs#I7.

Every run below fired once, after its addendum's write time, on the CUDA route with the `gb` arm (GBNF), each model
on its own authors' sampling, and `-rea off` (plus `enable_thinking:false` for Qwen and Nemotron). No run was
concurrent with another.

## The verdict

**`PASS · rank1 37/40`: Qwen3.5-2B-Q4_K_M** (S1 9 · S2 9 · S3 9 · S4 10). Fresh generations,
`target/l4-decision-probe/confirm-gb-20261005T083515Z`. The probe printed
`l4-decision-probe: names-trigger verdict: PASS · rank1 37/40` and exited 0. Held-out S5/S6: 19/20 (S5 9 · S6 10),
recorded only (`confirm-heldout-gb-20261005T083743Z`).

## The rule over six rows (addendum 2, read verbatim; Nemotron per addendum 4)

- **G1** (no failure label on a load or CPU-only row): Qwen3.5-4B (8 CPU timeouts) and gemma-4-E4B (5 CPU timeouts)
  are disqualified. Qwen3.5-2B, gemma-4-E2B and Nemotron pass. Nemotron's CPU-only max is 57551 ms, under the 60 s
  timeout.
- **G2** (no `thinking present`): every model passes (0 present on every row of every leg).
- **G3:** retired (addendum 2). CPU readings stay in the table as data.
- **Order** (CPU-only-leg max `peak_rss_kib`): Qwen3.5-2B 2321320 KiB, Nemotron 3849416 KiB, gemma-4-E2B
  4667264 KiB.
- **S** (first in order with selection rank1 ≥ B = 36): Qwen3.5-2B (37). Nemotron (40) and gemma-4-E2B (40) also
  clear B but come later in the order, so the pick is unchanged by the sixth model.
- **C:** Qwen3.5-2B PASS 37 ≥ 36. Nemotron and gemma-4-E2B ran fresh confirmations as RECORD-ONLY comparators
  (no verdict flag): 40/40 each.

## The combined table (computed from every `runs.json`, no hand-copied number)

| model | selection rank1/40 | confirmation rank1/40 | S5/S6 /20 | GPU elapsed_ms p50 / max | peak RSS MiB (CUDA) | peak VRAM MiB | CPU-only elapsed_ms p50 / max (timeouts) | CPU-only peak RSS MiB | thinking present | licence | rule outcome |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Llama 3.2 3B Instruct (baseline) | 36 = B | — | — | 2569 / 3257 | 2317 | 3382 | 25654 / 33968 (0) | 4495 | 0 | Llama 3.2 Community (not re-read in this chunk) | baseline |
| **Qwen3.5-2B** | 37 | **37 PASS** | 19 | 3671 / 4348 | **1637** | **2128** | 28920 / 35313 (0) | **2266** | 0 | Apache-2.0 | **pick, confirmed** |
| Qwen3.5-4B | 40 | — | — | 5518 / 6561 | 3029 | 3716 | 60001 / 60002 (8) | 4720 | 0 | Apache-2.0 | G1 (CPU timeouts) |
| Gemma 4 E2B | 40 | 40 (record-only) | 20 | 4337 / 4795 | 3412 | 2308 | 31908 / 36689 (0) | 4557 | 0 | Apache-2.0 | survivor, after the pick |
| Gemma 4 E4B | 39 | — | — | 6072 / 7201 | 5196 | 3898 | 60001 / 60002 (5) | 7639 | 0 | Apache-2.0 | G1 (CPU timeouts) |
| Nemotron 3 Nano 4B | 40 | 40 (record-only) | 20 | 3720 / 4997 | 3157 | 3284 | 52610 / 57551 (0) | 3759 | 0 | NVIDIA Nemotron Open Model | survivor, after the pick |

Column bases:
- GPU latency, CUDA RSS and VRAM cover every CUDA row of the model: re-run Leg 1 + selection + confirmation +
  held-out. The CPU columns cover the CPU-only leg (8 rows; a timeout row reads its 60 s).
- MiB = KiB / 1024, floored. Licences are from the HF cards read at download (`acquisition.md`); Llama's was not
  re-read in this chunk.
- The predecessor's 34/40 for Llama was read under a different argv (llama.cpp defaults, `--json-schema-file`). B =
  36 is Llama under its own sampling and the `gb` grammar.

## What this confirms, and what it does not

- **Confirmed:** Qwen3.5-2B is the L4 model choice under the pre-registered rule as amended. It is the lightest
  model by RAM on both routes and by VRAM, with fresh rank1 37/40 ≥ B.
- **Not shipped by this chunk** (inputs#I6 §4): the product still invokes `--json-schema-file` with llama.cpp's
  sampling. Under that argv every qwen35 and gemma4 model fails sampler init on b9305 (`series.md` §Leg 1
  §Diagnosis). So pointing `ANDROMEDA_PULSE_MODEL_PATH` at Qwen3.5-2B today produces NO interpretation. The swap
  (GBNF via `--grammar-file`, the authors' sampling, `enable_thinking:false`) is route entry A, to be minted at the
  wrap. Until it lands, the shipped model stays Llama 3.2 3B.
- **For the founder's choice:** Nemotron and gemma-4-E2B each read 40/40 fresh, against Qwen3.5-2B's 37, at about
  1.9–2.1× its CUDA RSS (3157 / 3412 vs 1637 MiB) and 1.1–1.5× its VRAM (2308 / 3284 vs 2128 MiB). Nemotron's licence is not Apache-2.0. The rule picks by footprint first,
  as ordered; this table is what a different weighting would read from.
