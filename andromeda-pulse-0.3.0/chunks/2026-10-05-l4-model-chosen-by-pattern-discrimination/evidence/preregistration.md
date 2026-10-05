# Pre-registration — L4 model chosen by pattern discrimination

Written before any real-model generation of this chunk. Every real-model leg's out dir carries a later UTC stamp
(listed in `series.md`). Nothing below is re-thresholded after a run; a keyword fix after a failed audit is an
addendum and re-grades the stored outputs, never re-generates (design inputs#I1 §3).

Written (UTC): 2026-10-05T10:39:23Z
audit seed: 418507

## What is frozen, and how

- The shapes, their numbers, the ground truth and the cause-word sets live in code:
  `pulse-app/examples/l4_decision_probe/patterns.rs`, sha256 at this write
  `d597e7ccef151ce4413cf46f2a0628c8d3a19ba41c289da7346bf57d1040229d`. The table below is a reading of that file; the
  file is the authority.
- The scorer, the re-grade, the audit draw and the recommendation rule are the functions of the same file, pinned
  by the 32 pattern pins and the mutation checks (`mutation-checks.md`).
- Every digest is rendered through the product's `render_payload`, composed with `build_primary_tier_prompt` and
  checked by `validate_prompt_bounded`. The window is 60 s. The largest composed prompt is 7,575 B (A6, today's
  render), under the 16,384 B bound.

## Shape table

Mode label: `tier3` for every cueless shape (the 60 s cueless channel); `tier1` for A7, the one cued shape.
Expected decision class: `surface` for A and C; not `surface` (dismiss or watch) for B. Ground-truth service and
cause words: A and C only; B is not cause-scored. Matching (Step 2): case-insensitive, over the FIRST hypothesis's
statement only. A service is named by its full name, its name with spaces for hyphens, or its first segment at a word
start. A cause word matches at a word start (so `saturat` matches "saturation", `recur` matches "recurring").

| shape | family | mode | expected | ground-truth service(s) | cause words | what the digest carries |
|---|---|---|---|---|---|---|
| A1 | A | tier3 | surface | cart-service | deploy, release, commit, change, rollout, rollback | cart-service at 18 % errors (baseline 0.4 %); a commit "cart-service: new price cache client" 3 min ago |
| A2 | A | tier3 | surface | payment-service | latency, slow, timeout, timed out, time out, p99, response time | checkout-api at 30 % errors and p99 5150 ms; payment-service at 2 % errors but p99 5000 ms |
| A3 | A | tier3 | surface | search-api, inventory-service (either) | shared, common, dependency, downstream, database, infrastructure, network | two unrelated services both at p99 1250-1400 ms with errors under 1 % |
| A4 | A | tier3 | surface | fraud-scorer | error, fail, exception, broken, crash | fraud-scorer at 0.2/s and 50 % errors; everything else healthy |
| A5 | A | tier3 | surface | auth-service | saturat, dependency, downstream, database, capacity, overload, contention, queue, pool, exhaust, bottleneck | auth-service at p99 2400 ms (baseline 42 ms), 0 % errors |
| A6 | A | tier3 | surface | inventory-service | recur, again, repeat, previous, before, known, history, past, prior | inventory-service at 6 % errors; three resolved corpus matches carrying ONE fingerprint, 1, 3 and 5 days old |
| A7 | A | tier1 | surface | payment-service (the origin Y) | error, fail, timeout, timed out, unavailable, down, latency, slow, origin, root cause | a retry-storm cue on checkout-api (X, 12 % errors); payment-service (Y) at 40 % errors, the origin by the numbers |
| B1 | B | tier3 | not surface | — | — | five healthy services |
| B2 | B | tier3 | not surface | — | — | 0.3 % errors at 120/s; a batch job whose p99 8200 ms sits at its own 8000 ms baseline |
| B3 | B | tier3 | not surface | — | — | two recent commits (4 and 20 min ago); every number healthy |
| C1 | C | tier3 | surface | notification-worker | drop, declin, fell, fall, traffic, throughput, volume, stall, stopp, dead, idle, consum | notification-worker at 4.1/s, 0 % errors (baseline 40/s; trend 40 to 4) |
| C2 | C | tier3 | surface | cart-service | creep, rising, rise, increas, grow, gradual, degrad, trend, climb, leak | cart-service at p99 460 ms (baseline 180 ms; trend 185 to 460, window over window) |
| C3 | C | tier3 | surface | search-indexer | periodic, cron, schedul, gc, garbage, batch, interval, every, cycle, recur | search-indexer at p99 2100 ms (baseline 140 ms; trend: a spike every third window) |

The baselines and the trend reach the model only in the C shapes' enriched render, a probe-side text transform: each
service row gains its baselines and a short six-window trend block follows the services. Today's render prints no
baseline and no trend (`render_payload`, unchanged).

## The series

- n = 10 per shape-render. A and B run today's render; C runs `today` and `enriched`. 16 shape-renders × 10 × 6
  models = 960 generations.
- Every generation runs the `gb` arm (`--grammar-file l4-output.gbnf`) with the shipped `-c 8192 -rea off`, on the
  CUDA route, one llama-cli spawn each, with the production 60 s timeout.
- The six models and their authors' sampling, copied from the predecessor chunk's
  `preregistration-addendum.md` (Llama, Qwen, gemma) and `preregistration-addendum-4.md` (Nemotron):

| model (GGUF basename) | sampling | thinking off |
|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M (baseline) | `--temp 0.6 --top-p 0.9 --top-k 40 --min-p 0` | `-rea off` (no thinking template) |
| Qwen3.5-2B-Q4_K_M | `--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 --chat-template-kwargs {"enable_thinking":false}` | `-rea off` + the kwarg |
| Qwen3.5-4B-Q4_K_M | the same as Qwen3.5-2B | the same |
| gemma-4-E2B-it-Q4_K_M | `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0` | `-rea off` |
| gemma-4-E4B-it-Q4_K_M | the same as gemma-4-E2B | the same |
| NVIDIA-Nemotron-3-Nano-4B-Q4_K_M | `--temp 0.6 --top-p 0.95 --top-k 40 --min-p 0 --chat-template-kwargs {"enable_thinking":false}` | `-rea off` + the kwarg |

## Scoring

Per generation, from the model's JSON only, three closed label sets:

- **valid** ∈ {valid, parse_failed, thinking, timeout, exit_failure, output_too_large, stdout_utf8_invalid}.
  `valid` needs captured stdout, a bounded JSON object extracted from it, `parse_bounded` Ok (which runs the schema's
  `validate`) and no thinking marker. A slower generation is cut by the runner's own 60 s timeout and reads
  `timeout`; latency is its own axis, never a validity atom.
- **detect** ∈ {hit, miss, false_alarm, quiet, no_reading}: `surface` on A or C is `hit`, anything else `miss`;
  `surface` on B is `false_alarm`, anything else `quiet`; a row that is not `valid` is `no_reading`.
- **cause** ∈ {hit, service_only, word_only, none, not_scored, no_reading}: the first hypothesis's statement names a
  ground-truth service AND a cause word (`hit`), one of them alone, or neither; B is `not_scored`. A model-authored
  identity (the output's fingerprint, its title) is never read.

**Denominators.** A row that is not `valid` is `no reading`: it is excluded from every A, B and C percentage and
shown per model as `no_reading n/N`. A zero-generation row (exit 0, no JSON) is `parse_failed`, so it is never `valid`
and never a correct B dismissal. A cell with 0 readable rows prints `cannot-evaluate` with its named cause, never a
percentage.

## The audit

- The seed above. ceil(10 %) of the series' rows (96 of 960), drawn by a seeded SplitMix64 partial Fisher-Yates over
  every model's rows, with the model hidden behind an opaque row id. The sample and its key are written under the
  series root in the gitignored `target/`.
- The overseer reads each drawn row's digest and output against the ground truth and records `agree` or `disagree`
  with the keyword labels.
- If the disagreement is above 10 % of the sample, the keyword sets are fixed in code, recorded as an addendum to this
  file, and the WHOLE series is re-graded from its stored outputs. It is never re-generated.

## The rule

The lightest model whose A-cause is within 10 points of the best A-cause and whose B false alarm is no worse than the
best B false alarm + 10 points. "Lightest" orders by the model's max `peak_vram_mib` in this series, then its max
`peak_rss_kib`. A model with either cell `cannot-evaluate` does not qualify. The table also draws, per model, GPU
latency p50 and max and the share of generations over 5000 ms (the gpu-primary budget), on the footprint axis beside
VRAM and RSS. The rule only recommends; the founder decides.
