# Pre-registration addendum — the `gb` arm with per-model authors' sampling

**Written (UTC):** 2026-10-05T07:23:28Z — before any re-run leg. Every re-run leg's out dir must carry a later timestamp.
Held as a draft from the overseer's HOLD until the research findings (inputs#I5) arrived; folded and stamped then.

**Base:** `preregistration.md` (written 2026-10-05T06:49:18Z). Everything it states stands unless amended here.
**Authority:** overseer (founder-delegated), 2026-10-05, /implement P2. The cause was named, then the option chosen
was "Probe-side GBNF arm, all": one argv for all five, written and timestamped before any re-run, no product change.

## Why

The first readings (`series.md` §Leg 1, §Leg 2) stand as measured. On the pinned llama-cli b9305 every candidate
loaded and generated nothing. The cause is read at b9305 source:
- `--json-schema-file` makes a `COMMON_GRAMMAR_TYPE_OUTPUT_FORMAT` grammar (`common/arg.cpp:1916`).
- That type is prefilled with the template's generation prompt (`common/common.h:204-206`;
  `common/sampling.cpp:279-292`). `llama-cli` sets that generation prompt only for a template with a thinking end
  tag (`tools/cli/cli.cpp:~100-106`).
- For qwen35 and gemma4 the prefix (`<think>\n</think>\n` under `-rea off`) is rejected by a grammar whose root
  opens with `{`. The result is `Failed to initialize samplers`, printed to stdout, exit 0.
- Llama 3.2's template has no thinking tag, so it gets no prefill.

A user grammar (`--grammar-file`, `COMMON_GRAMMAR_TYPE_USER`) is never prefilled. Measured outside the series on
the shipped S2 prompt, with only the failure class read: all five models generate a parseable L4 object under it.
`--no-jinja` was rejected because it aborts gemma4 (exit 134, no legacy template).

## The `gb` arm

- `gb` = `shipped` with the `--json-schema-file {schema}` pair removed and `--grammar-file {gbnf}` appended. The
  prompt, the schema embedded in the prompt, `-c 8192`, `-rea off`, `-n 1024`, the timeout and the extractor are
  unchanged. Pinned by `gb_arm_swaps_exactly_the_schema_file_for_the_grammar_file` (mutation (f) in
  `mutation-checks.md`) and `gbnf_flag_takes_a_path_and_is_unset_by_default`.
- The GBNF is `target/l4-decision-probe/l4-output.gbnf` (gitignored, host-local), sha256
  `7b5cc276b45d45a65c372c4cceb20d01bf6ab668d208f7bbd11a5125a8a8ac03`. It was produced by b9305's own
  `examples/json_schema_to_grammar.py` (sha256 `677553718afb2bc2…52a8`) from the shipped schema, which is
  byte-equal in JSON to `crates/interpretation/src/schema.json` (sha256 `3c1071c2…7a`). 44 rules.
- Equivalence to the grammar `--json-schema-file` builds in C++: the 8 rules visible in a captured b9305 stderr
  (`model-tier`, `schema-version`, `severity`, `space`, `symptom`, `timeline`, `title`, `prompt-version`) and the
  head of `root` match byte for byte. The other rules were not compared: no standalone C++ converter ships in
  b9305's `build/bin`.

## The re-run legs (in order; each fired once)

Every leg: `--arms gb --gbnf target/l4-decision-probe/l4-output.gbnf --sampling "$(samp $m)" --footprint`, all five
models in the original loop order (Llama, Qwen3.5-2B, Qwen3.5-4B, gemma-4-E2B, gemma-4-E4B), the env sourced as in
plan entries 19-21.

1. **Load + no-thinking, CUDA route.** `--shapes S1 --n 2`. Out dir `target/l4-decision-probe/load-cuda-gb-{UTC}`.
2. **CPU-only, GPU hidden.** `ANDROMEDA_PULSE_HARDWARE_PROFILE=cpu_primary CUDA_VISIBLE_DEVICES=` (empty),
   `--shapes S1,S2,S3,S4 --n 2`. Out dir `cpu-only-gb-{UTC}`.
3. **Selection, CUDA route.** `--shapes S1,S2,S3,S4 --n 10`. Out dir `selection-gb-{UTC}`. B = Llama's rank1/40 in
   this slot, under its own sampling (same slot, same argv shape).
4. **Rule applied in writing** (`selection.md`), then confirmation (`--shapes S1,S2,S3,S4 --n 10 --min-rank1 34`) and
   held-out (`--shapes S5,S6 --n 10`) on the pick, both with the pick's own `--sampling` and the `gb` arm.

## The rule under the addendum

G1, G2, G3, the Order, S, C, Ship, the Table, the Discipline and the Baseline clause apply verbatim, read from the
`gb` legs. One clarification fixed before the data: a row that reads `parse_failed` with a
`Failed to initialize samplers` cause is a load failure for G1. Under `gb` this cannot occur by construction, and
the original legs' rows of that class are recorded as such in the table, never as a quality reading. Under the
pre-registered argv the `nr` reading was taken, but under `gb` it is not: with a grammar applied from the first
generated token, a thinking pass cannot be emitted, so G2 under `gb` reads the marker with that mechanism stated.

## Per-model usage (from inputs#I5, the overseer's research relay, snapshotted at /implement P2)

What the models ran with before this addendum (inputs#I5 §1, MEASURED by the overseer over the GGUFs): only the
Gemma GGUFs embed `general.sampling.*` (temp 1.0 · top_p 0.95 · top_k 64). b9305 reads that metadata
(`common/common.cpp:1140`), and CLI flags take precedence over it. So Llama and Qwen ran on llama.cpp's hardcoded
defaults (temp 0.80 · top_k 40 · top_p 0.95 · min_p 0.05), Gemma ran on its authors' values, and all five ran on
min_p 0.05, which no author recommends. Under the addendum every model passes its authors' non-thinking sampling as
EXPLICIT flags, with `--min-p 0` for all.

| model | sampling flags passed | thinking off | structured output | source (inputs#I5) |
|---|---|---|---|---|
| Llama-3.2-3B-Instruct-Q4_K_M (baseline) | `--temp 0.6 --top-p 0.9 --top-k 40 --min-p 0` | n/a (no thinking template); `-rea off` stays (a no-op, measured in Leg 1) | `--grammar-file` GBNF | §2: generation_config.json. **top_k: the authors state none. This addendum CHOOSES 40**, llama.cpp's hardcoded default and the value of every prior Llama series, so Llama's only sampling change is the authors' stated temp/top_p plus min_p 0 |
| Qwen3.5-2B-Q4_K_M | `--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 --chat-template-kwargs {"enable_thinking":false}` | `-rea off` + the `enable_thinking:false` kwarg, passed explicitly (§3) | `--grammar-file` GBNF | §2: the unsloth Qwen3.5 guide, "non-thinking, general tasks" |
| Qwen3.5-4B-Q4_K_M | the same as Qwen3.5-2B | the same; the official 4B template defaults to thinking, so the kwarg is explicit | `--grammar-file` GBNF | §2, §3 |
| gemma-4-E2B-it-Q4_K_M | `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0` | `-rea off`; the prompt carries no `<\|think\|>` (measured: 0 occurrences in the composed S2 prompt) | `--grammar-file` GBNF | §2: google generation_config.json + model card |
| gemma-4-E4B-it-Q4_K_M | the same as gemma-4-E2B | the same | `--grammar-file` GBNF | §2 |

**One argv shape, per-model values.** Every generation of the re-run legs is:
`-m {gguf} -ngl {99|0} -c 8192 -rea off -st --simple-io --no-display-prompt --log-disable -n 1024 -p {prompt}
{sampling flags above} --grammar-file l4-output.gbnf`. That is the `gb` arm with the probe's `--sampling` flag:
whitespace-separated flag/value pairs, allowlisted to `--temp --top-p --top-k --min-p --presence-penalty
--chat-template-kwargs`, each value validated. Pinned by `sampling_flag_keeps_the_allowlisted_pairs_in_order` and
`sampling_flag_refuses_anything_outside_the_allowlist` (mutation (g)). Each probe run prints its sampling tokens on
a `l4-decision-probe: sampling …` line, which records the exact argv per model.

The leg loops pick each model's sampling with this mapping, fixed now:

```
samp() { case "$1" in
  Llama*) echo '--temp 0.6 --top-p 0.9 --top-k 40 --min-p 0' ;;
  Qwen*)  echo '--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 --chat-template-kwargs {"enable_thinking":false}' ;;
  gemma*) echo '--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0' ;;
esac; }
```

**Not applied, recorded:**
- The quantization notes (inputs#I5 §5). Every model stays on Q4_K_M (the founder's footprint priority). If a
  Gemma narrowly misses, its Q8 or QAT form is a named follow-up, never a mid-series swap.
- `--reasoning-budget 0`. Not passed: the relay reports it ineffective on Gemma 4, and under a grammar applied
  from the first token no thinking pass can be emitted.
- The confirmation threshold. C stays at the pre-registered `≥ 34`; it is not amended here. B in S is re-measured
  under Llama's own sampling, in the same slot.

## Shipping

Unchanged from the base: the product argv stays `--json-schema-file` with llama.cpp's sampling. If a candidate
PASSES confirmation, it cannot ship on this chunk: shipping GBNF and per-model sampling widens the product boundary
and needs the founder's live word. The follow-up route entry is minted next (before pre-push:linux). It ships the
model WITH its sampling and the GBNF route (inputs#I5 "What the addendum should do" 4), and Conductor v3-09 waits on
it.
