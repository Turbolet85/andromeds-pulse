# Red before green — the pins of steps 4, 5, 6 and 8

Read 2026-10-10, 02:46Z, by /implement, step 1 of the plan.

## The tree at the red reading
- The pins were written first: the in-file tests of `xtask/src/harness_witness.rs` and
  `xtask/src/harness_series.rs`, the moved member pin in `xtask/src/harness_ready.rs`, the three workflow pins in
  `pulse-app/tests/quality_gate_workflow.rs`.
- The two new modules held their types, their constants and their tests; every decision, reader and payload
  function was a stub returning one fixed value (`unset`, `None`, `no-record`, `cannot-evaluate`, an empty object).
  The two modules were declared in `xtask/src/main.rs`; the verb was not.
- `scripts/exit-witness.c` did not exist. `scripts/agent-run.sh`, `.github/workflows/ci.yml` and `settled_payload`
  were as at the chunk base.

## The red reading
`cargo nextest run --workspace --profile ci --no-fail-fast`, once: exit 100, summary
`2876 tests run: 2838 passed, 38 failed, 0 skipped`.

The 38 failed tests, all of them pins this chunk wrote:

| where | failed | why red |
|---|---|---|
| `harness_ready::tests::settled_payload_carries_the_closed_field_set` | 1 | the object held seven members, the pin names eight |
| `harness_series::tests::*` | 15 | the stubs: every decision read `cannot-evaluate`, every label `no-record`, no file kept, an empty payload |
| `harness_witness::tests::*` (reader and decision) | 12 | the stubs: the decision read `unset` for every input, the reader read no line |
| `harness_witness::tests::built_library::*` | 7 | `cc` found no `scripts/exit-witness.c` |
| `quality_gate_workflow` (`ci_workflow_boot_job_builds_the_exit_witness_before_the_smoke`, `ci_workflow_boot_series_runs_after_the_smoke_whatever_it_returned_and_before_ci_gates`, `ci_workflow_boot_series_carries_no_soft_fail`) | 3 | the `boot` job held neither step |

Three new tests read green at the red reading, each for a stated reason:
- `harness_witness::tests::no_witness_file_is_unset`: the stub's one fixed value is this arm's value. It does not
  discriminate alone; the other five arms' pins do.
- `harness_witness::tests::the_reader_refuses_a_byte_outside_printable_ascii`: a rejection pin, green under a reader
  that refuses everything. Its accepting half is `the_reader_takes_the_three_kinds_and_nothing_else`, red above.
- `harness_series::tests::the_cycle_script_runs_the_smoke_sequence_and_always_cleans_up`: it pins a constant, which
  was written in its final form with the pins.

One test changed between the red and the green reading beyond its subject: the member pin's call of
`settled_payload` gained its seventh argument (`"exit-call"`) and one assertion on the member's value, when the
function gained the parameter.

## The green reading
After steps 2 to 8, in the gate block's last full call (02:53Z to 02:57Z; `evidence/local-legs.md`):
- the workspace entry, `cargo nextest run --workspace --profile ci`: exit 0,
  `2876 tests run: 2876 passed, 0 skipped`. The same 2876 tests as the red reading, the 38 among them.
- the targeted entry, `cargo nextest run -p xtask --profile ci -E 'test(/harness_witness::|harness_series::/)'`:
  exit 0, `37 tests run: 37 passed`, the seven controls on the built library among them.
