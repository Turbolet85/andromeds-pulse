# Selection — the pre-registered rule applied in writing (plan Step 9)

**Rule:** `preregistration.md` (2026-10-05T06:49:18Z) as amended by `preregistration-addendum.md`
(2026-10-05T07:23:28Z): the `gb` arm, each model on its authors' sampling. **Legs read:** re-run Leg 1
`load-cuda-gb-20261005T072350Z`, re-run Leg 2 `cpu-only-gb-20261005T072447Z`, selection
`selection-gb-20261005T075309Z`, each fired once. The original-argv legs (`series.md` §Leg 1 and §Leg 2) are
recorded beside them. Under that argv every candidate hit the b9305 sampler-init failure: no generation, so no
quality or latency reading.

## The table — every model, the baseline and the disqualified included

| model | load CUDA / CPU (G1) | `nr` thinking | `gb` thinking (all legs) | CPU-only elapsed_ms p50 / max | CPU rows > 30 s · timeouts | CPU-route peak RSS (KiB) | CUDA peak VRAM (MiB) | CUDA peak RSS (KiB) | GPU elapsed_ms p50 / max | selection rank1/40 (S1 S2 S3 S4) | disqualification |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M (baseline) | ok / ok | absent 2/2 | present 0/50 | 25654 / 33968 | 2 · 0 | 4603264 | 3382 | 2373308 | 2569 / 3257 | **36** (10 10 6 10) = B | — (baseline; its own G3 reading would fail) |
| Qwen3.5-2B-Q4_K_M | ok / ok | not measurable¹ | present 0/50 | 28920 / 35313 | 3 · 0 | 2321320 | 2128 | 1676632 | 3601 / 4192 | 37 (9 9 9 10) | **G3**: 3 CPU rows > 30000 ms |
| Qwen3.5-4B-Q4_K_M | ok / **timeout ×8** | not measurable¹ | present 0/50 (8 unread) | 60001 / 60002 | 8 · 8 | 4834216 | 3716 | 3102696 | 5518 / 6561 | 40 (10 10 10 10) | **G1** (8 timeouts) and **G3** |
| gemma-4-E2B-it-Q4_K_M | ok / ok | not measurable¹ | present 0/50 | 31908 / 36689 | 7 · 0 | 4667264 | 2308 | 3494456 | 4253 / 4795 | 40 (10 10 10 10) | **G3**: 7 CPU rows > 30000 ms |
| gemma-4-E4B-it-Q4_K_M | ok / **timeout ×5** | not measurable¹ | present 0/50 (5 unread) | 60001 / 60002 | 8 · 5 | 7822352 | 3898 | 5320908 | 6072 / 7201 | 39 (10 9 10 10) | **G1** (5 timeouts) and **G3** |

¹ The `nr` arm was measured only under the original argv (Leg 1), where every candidate failed sampler init before
generating, so its `absent` rows are no reading. Under `gb` the `nr` reading is not taken (addendum §The rule).

Notes:
- CUDA peak RSS / VRAM and GPU elapsed are over re-run Leg 1 plus selection (42 generations per model). The CPU-route
  columns are over re-run Leg 2 (8 per model). `unread` thinking rows are timeouts (no stdout).
- `names_trigger_stem` in selection (recorded only): Llama 36, Qwen3.5-2B 39, Qwen3.5-4B 40, gemma-4-E2B 40,
  gemma-4-E4B 40.
- Every selection row parsed (0 `unparsed` for every model).

## Applying the rule

- **G1 (loads):** Qwen3.5-4B and gemma-4-E4B are disqualified (CPU-only timeouts). Qwen3.5-2B and gemma-4-E2B pass.
- **G2 (no thinking):** all pass (0 `present` on any `gb` row).
- **G3 (every CPU-only generation ≤ 30000 ms, none timed out):** every candidate is disqualified: Qwen3.5-2B (max
  35313), gemma-4-E2B (max 36689), Qwen3.5-4B and gemma-4-E4B (timeouts).
- **Survivors:** none, so the Order is empty.
- **S:** B = 36 (Llama's selection rank1/40 under its own sampling, same slot, same argv shape). No survivor exists
  to compare against B.

pick: none

## Consequences, as pre-registered

- The confirmation and held-out entries (plan entries 22 and 23) are recorded **not fired — no candidate survives
  the gates** (here G1/G3, not the quality floor; every candidate's selection rank1 is ≥ B).
- **Ship:** Llama stays the L4 model. `-c 8192` and `-rea off` ship either way. The product argv stays
  `--json-schema-file` with llama.cpp's sampling (the `gb` arm and per-model sampling are measurement-side only).
- **Not decided by this rule, surfaced for the overseer:**
  - Every model, the baseline included, misses the borrowed 30 s cpu_primary budget on this host's CPU route at
    `-c 8192` with a ~7.2 KB prompt. On the GPU, every candidate holds rank1 ≥ B: Qwen3.5-2B 37 at the smallest
    footprint (1.68 GB RSS / 2.13 GB VRAM), Qwen3.5-4B and gemma-4-E2B 40.
  - The rule's footprint-first order put G3 ahead of quality, as the founder ordered. Whether the G3 bound, the CPU
    route or the context size should change is a new pre-registration's question, never a re-threshold of this one
    (Discipline).
