# Real-model series — L4 model selection

Pre-registration: `preregistration.md`, written 2026-10-05T06:49:18Z. Every leg below started later.
Host: RTX 3090, 62 GiB RAM, 32 threads; llama-cli b9305 (CUDA build, `-ngl 99` on the CUDA route, `-ngl 0` on the CPU
route); argv `-c 8192 -rea off` (Step 1). Model and binary paths are printed as basenames only.

## Precondition — plan entry 18 (`leg = 'operator'`, driven by hand 2026-10-05T07:03:46Z)

`test "$(ss -ltnH | grep -cE ':(4317|4318) ')" = 0 && nvidia-smi --query-compute-apps=pid --format=csv,noheader | wc -l`
→ exit 0, last line `1`. **Red** against `expect = ['exit 0', 'last line 0']`.

- The one compute process is `voxtype-osd-gtk4` (pid 2213, up about 11.7 h, 10 MiB VRAM). It is the founder's
  desktop voice-typing overlay, not a model process. GPU utilization read 0 %, device memory 987 MiB used (P3 read
  971 MiB idle). Ports 4317/4318 were free.
- **Operator word (overseer, founder-delegated, 2026-10-05, asked at /implement P2):** "voxtype is the founder's own
  desktop tool and is not ours to close. The precondition exists to keep a competing model load off the GPU; a 10 MiB
  overlay at 0 % util does not compete, and the footprint is per-child-PID. Record entry 18 red with that reason and
  run the legs."
- The legs ran on that word. `peak_vram_mib` is read per child pid (`nvidia-smi --query-compute-apps`), so the
  overlay's 10 MiB enters no footprint reading.

## Leg 1 — load + no-thinking, CUDA route (plan entry 19, driven once by hand)

Out dir `target/l4-decision-probe/load-cuda-20261005T070517Z` (after the pre-registration). Exit 0; every model
printed `binary llama-cli (cuda, -ngl 99)`, both footprint summary lines, and no INCONCLUSIVE. All expect atoms hold.
`--arms shipped,nr --shapes S1 --n 2 --footprint`, so 4 generations per model.

| model | arm | decisions | thinking | elapsed_ms | peak_rss_kib max | peak_vram_mib max | rank1 |
|---|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M | shipped | surface 2 | absent 2 | 2636, 2681 | 2373716 | 3382 | 2/2 |
| Llama-3.2-3B-Instruct-Q4_K_M | nr | surface 2 | absent 2 | 3322, 2330 | 2371632 | 3382 | 2/2 |
| Qwen3.5-2B-Q4_K_M | shipped | parse_failed 2 | absent 2 | 1093, 1084 | 1674976 | 1468 | 0/2 |
| Qwen3.5-2B-Q4_K_M | nr | parse_failed 2 | absent 2 | 1095, 1081 | 1675040 | 1468 | 0/2 |
| Qwen3.5-4B-Q4_K_M | shipped | parse_failed 2 | absent 2 | 1232, 1173 | 3103036 | 2860 | 0/2 |
| Qwen3.5-4B-Q4_K_M | nr | parse_failed 2 | absent 2 | 1177, 1278 | 3100804 | 2860 | 0/2 |
| gemma-4-E2B-it-Q4_K_M | shipped | parse_failed 2 | absent 2 | 1510, 1534 | 3494516 | 1664 | 0/2 |
| gemma-4-E2B-it-Q4_K_M | nr | parse_failed 2 | absent 2 | 1535, 1554 | 3492808 | 2280 | 0/2 |
| gemma-4-E4B-it-Q4_K_M | shipped | parse_failed 2 | absent 2 | 1885, 1829 | 5319228 | 3140 | 0/2 |
| gemma-4-E4B-it-Q4_K_M | nr | parse_failed 2 | absent 2 | 1812, 1866 | 5319844 | 3348 | 0/2 |

Readings:
- **Baseline:** Llama passes G1 and G2 under the new argv (`-c 8192 -rea off`), so there is no product regression
  from Step 1. On Llama, `nr` also reads `thinking absent`: `-rea off` is a no-op there, as predicted.
- **Every candidate loads** (exit 0, nonzero RSS and VRAM) **and generates nothing:** every row is `parse_failed` at
  about 1–2 s. No G1 failure label fires, because the process exits 0 (below).
- `thinking` reads `absent` for every candidate. With no tokens generated, that is the absence of a marker in a
  zero-generation stdout, not evidence that the model does not think.

### Diagnosis — the candidates' JSON-schema grammar fails to initialize on b9305 (outside the series)

Four bounded manual llama-cli invocations, outside the series (not counted, not slotted): the shipped argv,
`-n 64` or `-n 32`, the prompt `Reply with a JSON object.`. Stdout and stderr went to a session scratch dir and were
read for the error lines only. None is in this record.

| model | grammar | result |
|---|---|---|
| Qwen3.5-2B-Q4_K_M | the shipped `schema-shipped.json` | stdout `Error: Failed to initialize samplers: std::exception`, exit 0; stderr `E common_sampler_init: error initializing grammar sampler for grammar:` |
| Qwen3.5-2B-Q4_K_M | none (no `--json-schema-file`) | generates; no sampler error |
| gemma-4-E2B-it-Q4_K_M | the shipped schema | the same sampler error as Qwen, exit 0 |
| Llama-3.2-3B-Instruct-Q4_K_M (control) | the shipped schema | generates; no sampler error |
| Qwen3.5-2B-Q4_K_M · gemma-4-E2B-it-Q4_K_M | a one-field schema `{"type":"object","properties":{"a":{"type":"string"}},"required":["a"]}` | the same sampler error for both |

**Finding.** On the pinned llama-cli b9305, ANY `--json-schema-file` grammar fails sampler initialization for the
`qwen35` and `gemma4` candidates, while the same grammar initializes for Llama 3.2. llama-cli reports the error on
STDOUT and exits 0, so the L4 runner sees a clean exit with no JSON object, and the probe records `parse_failed`.
On b9305, under the product's schema-constrained D1 path, the four candidates load but cannot produce an L4
generation at all. The scope boundaries keep b9305 (no bump, no runtime swap) and keep chat-template or grammar
handling out of the shared prompt, so this is SURFACED as a finding, not fixed. The pre-registered G1 label set does
not name this failure (exit 0 + an error line), so under the rule as written the candidates survive G1 and fail
on rank1 at selection.

Footprint caveat: a candidate's `peak_rss_kib` / `peak_vram_mib` here measures load plus the 8192-token context
allocation without a single generated token, and its `elapsed_ms` is load-to-error time, not generation latency.

## Leg 2 — CPU-only, GPU hidden (plan entry 20, driven once by hand)

Out dir `target/l4-decision-probe/cpu-only-20261005T070724Z`. Exit 0; every model printed `binary llama-cli (cpu,
-ngl 0)` and its footprint summary line, and no INCONCLUSIVE. All expect atoms hold. `--arms shipped --shapes
S1,S2,S3,S4 --n 2 --footprint`, `ANDROMEDA_PULSE_HARDWARE_PROFILE=cpu_primary`, `CUDA_VISIBLE_DEVICES=` (empty).

| model | rows | decisions | thinking | elapsed_ms p50 / max | rows > 30000 ms | peak_rss_kib max | peak_vram_mib | rank1 |
|---|---|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M | 8 | surface 8 | absent 8 | 27787 / 39981 | 3 | 4602944 | - (no device) | 7/8 |
| Qwen3.5-2B-Q4_K_M | 8 | parse_failed 8 | absent 8 | 958 / 989 | 0 | 2214028 | - | 0/8 |
| Qwen3.5-4B-Q4_K_M | 8 | parse_failed 8 | absent 8 | 1255 / 1293 | 0 | 4669100 | - | 0/8 |
| gemma-4-E2B-it-Q4_K_M | 8 | parse_failed 8 | absent 8 | 1449 / 1495 | 0 | 4543604 | - | 0/8 |
| gemma-4-E4B-it-Q4_K_M | 8 | parse_failed 8 | absent 8 | 1992 / 2050 | 0 | 7613168 | - | 0/8 |

Readings:
- **The baseline exceeds G3 on the CPU route under the new argv:** 3 of 8 Llama generations are over 30 000 ms
  (max 39 981 ms), none timed out at 60 s. The pre-registered stop clause covers a baseline failing G1 or G2 only.
  G3 is a candidate gate, so this is recorded, not acted on: Llama-on-CPU does not meet the cpu_primary budget the
  rule borrows (`xtask/ci/l4-latency-p99.sh:38`).
- **Every candidate row is the Leg 1 sampler-init failure class:** `parse_failed` at about 1–2 s with no tokens
  generated. These `elapsed_ms` values are load-to-error times, NOT CPU generation latencies, and they are never
  read as a G3 pass. Their `peak_rss_kib` is load plus the 8192-token context on CPU, with no generation.
- `peak_vram_mib` reads `-` for every row, as it should with the device hidden.

## Status after Leg 2 — the overseer's hold

- Overseer (founder-delegated), on the qwen35 rows: "a parse_failed on every row in ~1.2 s is not a quality
  reading; no text was generated. Do not let it enter selection as 0 rank1 yet." The cause was then named (Leg 1
  §Diagnosis; `preregistration-addendum.md` §Why), and the overseer chose a probe-side `gb` arm for all five models
  under one argv.
- HOLD (overseer, relaying the founder): before any re-run leg, each candidate's official usage is researched and
  folded into the addendum. No real-model leg fires, and the addendum is not stamped, until those findings arrive.
- The selection slot (plan entry 21) has NOT fired under either argv. No selection rule has been applied, and
  `selection.md` does not exist yet.

## Upstream — does a newer llama.cpp tag fix the prefill? (read 2026-10-05; no bump made)

- Upstream `master` HEAD, read through the GitHub API: `common/common.h:221-224` still returns
  `common_grammar_needs_prefill` true for `COMMON_GRAMMAR_TYPE_OUTPUT_FORMAT`, and `common/sampling.cpp:300-305` still
  feeds the generation-prompt tokens to such a grammar. The mechanism is unchanged at HEAD.
- `common/sampling.cpp` has 12 commits since b9305 (2026-05-24). None names the output-format prefill. The nearest
  are reasoning-budget and lazy-grammar changes (`910196f6`, `5254a799`, `da6c28eb`).
- Issue #23990 ("`Failed to initialize samplers` on Gemma4 with `response_format: json_schema` prefill") was CLOSED
  2026-07-17 as NOT_PLANNED, by the stale bot, with no fix.
- Issue #29006 ("JSON Schema grammar fails on chat-template control tokens while equivalent GBNF succeeds", Qwen3) is
  OPEN since 2026-09-17, labelled `bug-unconfirmed`; a commenter says a patch is coming, and none is merged.
- **Reading:** as of 2026-10-05 no llama.cpp tag fixes this. A tag bump is not an alternative to the GBNF path
  today; #29006 is the upstream item to watch.

# Under the addendum (`preregistration-addendum.md`, written 2026-10-05T07:23:28Z)

Every re-run leg: `--arms gb --gbnf l4-output.gbnf --sampling "$(samp $m)" --footprint`. Each model's run printed
its `l4-decision-probe: sampling …` line, matching the addendum's per-model table byte for byte. Precondition re-read
at 07:23:42Z: ports free, the one GPU compute process still the 10 MiB voxtype overlay (the ruled state), no
llama-cli running.

## Re-run Leg 1 — load + no-thinking, CUDA route (`gb`)

Out dir `target/l4-decision-probe/load-cuda-gb-20261005T072350Z` (after the addendum). Exit 0, no INCONCLUSIVE.

| model | decisions | thinking | elapsed_ms | peak_rss_kib max | peak_vram_mib max | rank1 |
|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M | surface 2 | absent 2 | 2501, 2288 | 2371184 | 3382 | 2/2 |
| Qwen3.5-2B-Q4_K_M | surface 2 | absent 2 | 3852, 3945 | 1674492 | 2128 | 2/2 |
| Qwen3.5-4B-Q4_K_M | surface 2 | absent 2 | 5105, 6385 | 3100572 | 3712 | 2/2 |
| gemma-4-E2B-it-Q4_K_M | surface 2 | absent 2 | 4472, 4417 | 3494216 | 2308 | 2/2 |
| gemma-4-E4B-it-Q4_K_M | surface 2 | absent 2 | 6297, 6064 | 5320908 | 3898 | 2/2 |

Every model loads AND generates a parseable L4 object under `gb`, so the original legs' sampler-init failure is
gone (the fix measured live). G1 and G2 hold for all five on the CUDA route.

## Re-run Leg 2 — CPU-only, GPU hidden (`gb`)

Out dir `target/l4-decision-probe/cpu-only-gb-20261005T072447Z`. Exit 0, no INCONCLUSIVE. S1–S4 × 2 per model.

| model | decisions | thinking | elapsed_ms p50 / max | rows > 30000 ms | timeouts (60 s) | peak_rss_kib max | rank1 |
|---|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M | surface 8 | absent 8 | 25654 / 33968 | 2 | 0 | 4603264 | 7/8 |
| Qwen3.5-2B-Q4_K_M | surface 8 | absent 8 | 28920 / 35313 | 3 | 0 | 2321320 | 8/8 |
| Qwen3.5-4B-Q4_K_M | timeout 8 | unread 8 | 60001 / 60002 | 8 | 8 | 4834216 | 0/8 |
| gemma-4-E2B-it-Q4_K_M | surface 8 | absent 8 | 31908 / 36689 | 7 | 0 | 4667264 | 8/8 |
| gemma-4-E4B-it-Q4_K_M | surface 3 · timeout 5 | absent 3 · unread 5 | 60001 / 60002 | 8 | 5 | 7822352 | 3/8 |

Gate readings (the rule verbatim):
- **G1 (no failure label on a shipped row):** Qwen3.5-4B FAILS (8 `timeout`) and gemma-4-E4B FAILS (5 `timeout`).
  Qwen3.5-2B and gemma-4-E2B pass.
- **G2 (no `thinking present`):** every row reads `absent` or `unread`, so all pass.
- **G3 (every CPU-only `elapsed_ms` ≤ 30000, none timed out):** EVERY candidate FAILS. Qwen3.5-2B has 3 rows over
  (max 35.3 s), gemma-4-E2B 7 (max 36.7 s), and Qwen3.5-4B and gemma-4-E4B time out.
- **Survivors: none.** Under S, `pick: none` follows before any selection reading. The selection slot fires anyway,
  once as registered, so the table carries every model's selection rank1 and B.
- The baseline also exceeds 30 s on the CPU route (2 of 8 rows, max 34.0 s), as in the original Leg 2. The G3 bound
  (the cpu_primary budget, `xtask/ci/l4-latency-p99.sh:38`) is not met by ANY model on this host's CPU route at an
  8192-token context and a ~7.2 KB prompt. That is a reading about the bound as much as about the models; it is
  recorded, not re-thresholded (Discipline).

## Selection slot — CUDA route (`gb`, each model's own sampling), fired once

Out dir `target/l4-decision-probe/selection-gb-20261005T075309Z`. Exit 0, no INCONCLUSIVE; S1–S4 × 10 = 40 per model.
Every row parsed, and thinking was present 0/40 for every model.

| model | rank1/40 | per shape S1 S2 S3 S4 | stem rank1 | GPU elapsed_ms p50 / max | peak RSS KiB | peak VRAM MiB |
|---|---|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M | **36 = B** | 10 10 6 10 | 36 | 2598 / 3257 | 2373308 | 3382 |
| Qwen3.5-2B-Q4_K_M | 37 | 9 9 9 10 | 39 | 3595 / 4192 | 1676632 | 2128 |
| Qwen3.5-4B-Q4_K_M | 40 | 10 10 10 10 | 40 | 5518 / 6561 | 3102696 | 3716 |
| gemma-4-E2B-it-Q4_K_M | 40 | 10 10 10 10 | 40 | 4246 / 4795 | 3494456 | 2308 |
| gemma-4-E4B-it-Q4_K_M | 39 | 10 9 10 10 | 40 | 6072 / 7201 | 5319988 | 3898 |

(The p50/max and peak values in this table are over the 40 selection rows only. `selection.md` gives the Leg 1 plus
selection figures.)

- **B = 36.** That is the baseline under its OWN sampling (temp 0.6 · top_p 0.9 · top_k 40 · min_p 0) and `gb`. The
  predecessor's confirmed reading was 34/40 under llama.cpp's default sampling (min_p 0.05) and `--json-schema-file`.
  The argv differs, so the two are not a like-for-like comparison of the same text.
- Every candidate reads rank1 ≥ B on the GPU route. Under the pre-registered rule, though, none is a survivor
  (re-run Leg 2: G1 and G3), so the rule gives **`pick: none`** (`selection.md`). Confirmation and held-out are not
  fired.

# Under addenda 2–4 (G3 retired; comparators; the sixth model)

- **Addendum 2** (08:35:02Z, inputs#I6): the founder's hardware ruling retires G3. `selection.md` stays
  `pick: none`. On the recorded data the rule now picks Qwen3.5-2B, so it must confirm fresh at rank1 ≥ 36.
- **Addendum 3** (08:35:46Z): gemma-4-E2B joins the confirmation slot as a record-only comparator.
- **Addendum 4** (08:43:36Z, inputs#I7): NVIDIA Nemotron 3 Nano 4B runs the same three legs, then the six-row rule.

| run | out dir | started (UTC) | exit | reading |
|---|---|---|---|---|
| Qwen3.5-2B confirmation | `confirm-gb-20261005T083515Z` | 08:35:15 | 0 | **PASS · rank1 37/40** (9 9 9 10), thinking 0/40, p50 3698 ms |
| Qwen3.5-2B held-out | `confirm-heldout-gb-20261005T083743Z` | 08:37:43 | 0 | 19/20 (S5 9 · S6 10) |
| gemma-4-E2B confirmation (record-only) | `confirm-gemma-e2b-gb-20261005T083900Z` | 08:39:00 | 0 | 40/40, thinking 0/40, p50 4372 ms |
| gemma-4-E2B held-out | `confirm-heldout-gemma-e2b-gb-20261005T084155Z` | 08:41:55 | 0 | 20/20 |
| Nemotron load (CUDA) | `load-cuda-gb-nemotron-20261005T084809Z` | 08:48:09 | 0 | `nemotron_h` loads; 2/2, thinking 0/2 |
| Nemotron CPU-only | `cpu-only-gb-nemotron-20261005T084817Z` | 08:48:17 | 0 | 8/8 rank1; p50 52610 / max 57551 ms; 0 timeouts; RSS 3849416 KiB |
| Nemotron selection | `selection-gb-nemotron-20261005T085527Z` | 08:55:27 | 0 | 40/40, thinking 0/40, p50 3755 ms |
| Nemotron confirmation (record-only) | `confirm-nemotron-gb-20261005T085813Z` | 08:58:13 | 0 | 40/40, thinking 0/40 |
| Nemotron held-out | `confirm-heldout-nemotron-gb-20261005T090045Z` | 09:00:45 | 0 | 20/20 |

No run printed INCONCLUSIVE. Every run started after its addendum's write time, and none overlapped another. The
six-row rule application and the combined table are in `confirmation.md`.
