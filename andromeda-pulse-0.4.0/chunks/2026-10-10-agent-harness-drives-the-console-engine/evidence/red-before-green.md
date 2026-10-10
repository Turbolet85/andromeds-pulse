# Red before green — gate entry 4 (the pins of `engine_log` and `engine_cycle`)

Entry 4 of the plan's gate block, fired twice through the gate tool in the /implement run
`.andromeda/runs/2026-10-10T15-24-06Z-implement/`:
`cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(xtask) and test(/^(engine_log|engine_cycle)::/)'`

## Firing 1 — the pins over stubbed cores (clock read after it: 2026-10-10T15:37:35Z)

The two modules existed with every pin written and every decision stubbed: each arm of the log check returned
PASS whatever it read, the overall verdict was PASS, the settle decision never returned, the cycle's refusal
returned none, its child environment was empty, its verdict was `pass`.

```
  4 unit        red · exit 0 ✗ (exit 100) · exit 100 · 3.42s · 102330 B → 4.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
entries 22 · green 0 · red 1 (4) · recorded 0 · timeout 0 · not-run 21
```

nextest's own line in `4.log`: `Summary [   0.031s] 66 tests run: 11 passed, 55 failed, 367 skipped`

The 55 pins that read FAIL, by what the acceptance names:

- **panic** — `engine_log::tests::a_panic_record_at_error_fails_by_file_name_and_line`
- **heartbeat gap** — `…::a_gap_over_45_seconds_between_two_ticks_of_one_target_fails_with_no_error_record`,
  `…::a_gap_of_exactly_45_seconds_passes` (its detail assertion), `…::fewer_than_two_ticks_of_a_target_cannot_be_evaluated`,
  `…::a_tick_with_no_readable_timestamp_cannot_be_evaluated`
- **progress** — `…::a_stall_announcement_fails`, `…::a_run_in_which_no_span_landed_fails`,
  `…::a_log_with_no_buffer_tick_cannot_be_evaluated_for_progress`,
  `…::buffer_ticks_with_no_numeric_row_count_cannot_be_evaluated_for_progress`
- **process end** — `…::a_run_ended_by_sigkill_with_no_exit_record_is_unloggable_end`,
  `…::a_run_with_no_exit_record_otherwise_is_end_not_recorded`, `…::two_exit_records_fail`,
  `…::a_record_after_the_exit_record_fails`
- **budget** — `…::a_memory_sample_over_budget_fails`, `…::a_log_with_no_memory_sample_fails_the_budget`,
  `…::only_zero_memory_samples_fail_the_budget`, `…::a_non_numeric_memory_value_fails_the_budget`
- **family and program** — `…::an_absent_log_dir_cannot_be_evaluated`,
  `…::a_log_dir_with_no_family_member_cannot_be_evaluated`, `…::members_that_hold_no_record_fail`,
  `…::a_boot_record_that_reads_window_fails`, `…::a_log_with_no_boot_record_cannot_be_evaluated`,
  `…::two_boots_in_one_family_cannot_be_evaluated`,
  `…::a_boot_record_naming_no_known_program_cannot_be_evaluated`
- **the whole check** — `…::a_complete_console_log_passes_every_arm_in_order`,
  `…::a_fail_outranks_a_cannot_evaluate_and_the_exit_follows_the_verdict`,
  `…::no_line_holds_a_record_s_text_or_a_path`
- **the settle read** — `…::a_settle_window_shorter_than_a_tick_interval_and_its_margin_is_refused`,
  `…::a_live_console_run_with_a_gradeable_log_is_settled`, `…::a_dead_pid_is_ended_whatever_the_log_holds`,
  `…::a_log_whose_boot_record_reads_window_is_wrong_program`, `…::a_log_short_of_a_reading_polls_until_the_timeout`,
  `…::an_absent_or_unprobeable_pid_cannot_be_evaluated`,
  `…::settle_evidence_counts_the_ticks_and_the_populated_memory_samples`,
  `…::settle_evidence_reads_the_family_and_no_file_beside_it`, `…::the_settle_exit_is_zero_for_settled_alone`,
  `…::the_settle_payload_carries_the_closed_member_set`
- **the cycle** (`engine_cycle::tests::…`) — `a_host_that_is_not_linux_is_refused`,
  `a_shared_port_in_either_place_is_refused`, `a_data_dir_that_already_holds_a_log_family_is_refused`,
  `a_log_family_is_read_under_the_data_dir_s_logs_alone`, `the_child_environment_is_exactly_the_fixed_set`,
  `the_witness_variable_is_passed_on_only_when_the_verb_s_own_environment_holds_it`,
  `any_verb_that_exits_one_fails`, `an_error_record_fails`, `a_witness_file_fails`,
  `a_failed_boot_fails_with_the_verbs_between_skipped`,
  `a_verb_that_could_not_evaluate_is_cannot_evaluate_unless_another_failed`,
  `a_cycle_that_did_not_run_cannot_be_evaluated`, `a_cycle_that_did_not_run_reports_nulls_and_no_witness_file`,
  `the_exit_follows_the_verdict`, `the_verdict_carries_the_closed_member_set`,
  `the_verdict_text_is_what_the_gate_entry_reads`, `the_default_data_dir_is_named_by_the_utc_second`,
  `the_passphrase_is_made_for_each_run`

The 11 pins that read PASS over the stubs are the ones whose input is healthy or whose subject was not stubbed,
so a stub that always passes cannot fail them: `engine_log::tests::` `a_kill_record_does_not_excuse_a_log_that_holds_its_end`,
`a_memory_sample_at_the_budget_passes`, `an_unrelated_file_beside_the_family_changes_nothing`,
`a_panic_target_record_below_error_passes`, `a_recovery_announcement_passes`,
`one_line_says_frame_and_snapshot_are_not_graded`, `the_console_label_is_the_one_the_settle_read_waits_for`,
`viz_and_plugins_ticks_are_never_graded`; `engine_cycle::tests::` `a_child_sees_the_constructed_set_and_nothing_of_this_process`,
`five_zero_exits_no_error_record_and_no_witness_file_pass`,
`the_default_ports_on_linux_over_a_fresh_data_dir_are_not_refused`. Each has a partner above that carries the
guard on the failing side.

## Firing 2 — the same pins over the written code (clock read after it: 2026-10-10T15:40:49Z)

```
  4 unit        green · exit 0 · 3.05s · 63521 B → 4.2.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

nextest's own line in `4.2.log`: `Summary [   0.035s] 66 tests run: 66 passed, 367 skipped`

## What changed between the two firings

Only the decision bodies of the two modules. One assertion of one pin was reworded after firing 1:
`a_complete_console_log_passes_every_arm_in_order` asserted that an arm's detail was not the stub's word and
now asserts that it is not empty. No pin was added, removed or weakened.

## Limits

- The stub read every arm PASS, so firing 1 shows each failing-input and each cannot-evaluate pin red against
  a check that grades nothing. It does not show a pin red against a check that is wrong in one arm only; no
  mutation of the written code was run.
- The pins of steps 1 to 3 (the injector's port, the status and readiness program reading) and the workflow
  pins of step 9 are outside this entry's filter and were not read red first; they run in the workspace entry.
