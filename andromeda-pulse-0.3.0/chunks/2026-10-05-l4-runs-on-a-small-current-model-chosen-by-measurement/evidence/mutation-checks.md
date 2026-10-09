# Mutation checks (plan Step 5) — one-shot controls, never listed as gates

Run 2026-10-05 by /implement, after the gate block read green. Each mutation was applied with an anchored Edit,
confirmed present by a fixed-string grep before the run, run through
`cargo nextest run --workspace --profile ci --no-fail-fast -E 'binary(unit_llamacli_inference) + binary(l4_decision_probe)'`
(85 tests), then restored. The restored tree re-ran 85/85 green, and `cargo fmt --check` was clean.

| id | mutation | grep confirm | result | pins red |
|---|---|---|---|---|
| (a) | `build_llama_cli_args` drops the `-c` / `LLAMA_CLI_CTX_SIZE` pair | `LLAMA_CLI_CTX_SIZE.to_string()` count 0 | 84 passed, 1 failed | `spawn_args_contain_fixed_context_size` |
| (b) | `build_llama_cli_args` drops the `-rea` / `LLAMA_CLI_REASONING` pair | `LLAMA_CLI_REASONING.to_string()` count 0 | 83 passed, 2 failed | `spawn_args_contain_reasoning_off` · `nr_arm_removes_exactly_the_reasoning_switch` |
| (c) | `thinking_label` reads `absent` on the marker | `THINKING_MARKER) => "absent"` count 1 | 83 passed, 2 failed | `thinking_label_set_is_exactly_the_closed_set` · `thinking_reads_present_on_the_b9305_marker` |
| (d) | `compose_argv` drains an empty range (the `nr` removal a no-op) | `args.drain(at..at.min(end))` count 1 | 84 passed, 1 failed | `nr_arm_removes_exactly_the_reasoning_switch` |
| (e) | `parse_vm_hwm_kib` returns `None` (`.and(None)`) | `.and(None)` count 1 | 84 passed, 1 failed | `vm_hwm_parser_reads_the_peak_resident_set` |

Added with the pre-registration addendum's probe changes (the `gb` arm and the `--sampling` flag), same procedure,
over the 87- and then 89-test set:

| id | mutation | grep confirm | result | pins red |
|---|---|---|---|---|
| (f) | `compose_argv` never appends `--grammar-file` (`false && prepared.grammar_file`) | count 1 | 86 passed, 1 failed (of 87) | `gb_arm_swaps_exactly_the_schema_file_for_the_grammar_file` |
| (g) | `parse_sampling` skips the allowlist check (`false && !SAMPLING_FLAGS…`) | count 1 | 88 passed, 1 failed (of 89) | `sampling_flag_refuses_anything_outside_the_allowlist` |

Both restored trees re-ran green (87/87, then 89/89), and the probe is clippy-clean (`-D warnings`).

Readings:
- Every mutation turned only the pins of its own property red. (b) reds two pins by design: the `nr` pin first
  asserts that the shipped argv carries `-rea off`, so a `-rea`-less builder reds it (plan Step 4). (c) reds the
  closed-set pin and the reads-present pin, both pins of the `thinking` label.
- The first application of (e) did not compile (`.and(None)` left the `parse` target type uninferred, E0284). It
  measured nothing and is not counted. It was re-applied as `.parse::<u64>().ok().and(None)`, and that run is the
  reading in the table.
- The `None`-returning pins (`vm_hwm_parser_reads_none_without_the_line`, the absent-marker pin) stay green under
  every mutation. They are the conditional halves: each passes under both the correct parser or label and the
  mutated one, and the present-side pins carry the guard (testing.md 2026-08-17).
