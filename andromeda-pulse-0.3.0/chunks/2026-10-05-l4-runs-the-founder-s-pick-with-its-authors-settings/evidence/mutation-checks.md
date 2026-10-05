# Mutation checks (step 8) — one-shot, each applied, grep-confirmed, run, reverted

Each mutation was applied with an anchored Edit (or a single-line `sed`), confirmed present by `grep -c` before
the run, and reverted afterwards (`git diff --stat 7663dd9` on the two sources was back to the chunk's own delta,
and the GBNF re-hashed to `7b5cc276…ac03`). Runs: `cargo nextest run --workspace --profile ci --no-fail-fast
-E '{selection}'`.

| # | mutation | selection | result | red pins |
|---|---|---|---|---|
| 1 | `build_llama_cli_args` emits `--json-schema-file` in place of `--grammar-file` | `binary(unit_llamacli_inference) + binary(l4_decision_probe)` | 126 run: 123 passed, 3 failed | `spawn_args_contain_model_path_and_grammar_file_and_no_schema_file` · `spawn_args_retain_reasoning_off_and_the_fixed_context_beside_the_grammar` · probe `shipped_argv_carries_the_grammar_file_and_no_schema_file` |
| 2 | the `--min-p` pair dropped from the argv | same | 126 run: 124 passed, 2 failed | `spawn_args_carry_the_authors_sampling_after_n_and_the_prompt_last` · probe `sampling_replaces_the_production_pairs_rather_than_duplicating` |
| 3 | `hardware_profile` removed from both latency emits (field renamed `hardware_profile_dropped`) | `binary(unit_inference_runtime)` | 17 run: 14 passed, 3 failed | `latency_sample_carries_the_runner_profile_on_the_parse_success_arm` · `…_on_the_runtime_error_arm` · `latency_sample_reads_unknown_for_a_runner_without_a_profile` |
| 4 | `grammar_for_schema` maps every schema to the grammar | `binary(unit_llamacli_inference)` | 65 run: 64 passed, 1 failed | `grammar_for_schema_refuses_any_other_schema` |
| 5 (extra) | one rule appended to the committed `l4-output.gbnf` | `binary(unit_l4_grammar)` | 2 run: 1 passed, 1 failed | `committed_grammar_is_the_converter_output_for_the_shipped_schema` (the discrimination pin stays green: it compares against the committed file, which still differs from the edited-schema output) |

## Grader controls (no source mutation; the rank and unlabeled rules)
- The chunk-base grader (`git show 7663dd9:xtask/ci/l4-latency-p99.sh`) with `L4_GPU_PRIMARY_BUDGET_MS=10000` on
  `xtask/ci/fixtures/l4-latency-tail.jsonl`: `profile=gpu_primary p99=1000ms ≤ 10000ms (sample_count=150) PASS`,
  exit 0 — the floor index reads 1000; the new grader reads 20000 and exits 1 (gate 7).
- The chunk-base grader on `l4-latency-unlabeled.jsonl`: `no … records found (… gate trivially passes)`, exit 0;
  the new grader prints `::error::l4-latency-p99: 1 sample(s) carry no hardware_profile or duration_ms`, exit 1
  (gate 9).

## Collected counts
- Gate 5 selection: 341 tests (332 at P5's baseline run + 9 new: 5 llamacli, 3 runtime, 1 interpretation).
- Gate 6, `binary(l4_decision_probe)`: 61 collected (61 at HEAD; 2 retired — `gb_arm_swaps_exactly_the_schema_file_for_the_grammar_file`,
  `gbnf_flag_takes_a_path_and_is_unset_by_default` — and 2 added).
- Gate 4, `binary(unit_l4_grammar)`: 2 run, 0 skipped.
