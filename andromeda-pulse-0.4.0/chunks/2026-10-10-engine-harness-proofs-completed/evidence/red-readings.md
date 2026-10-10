# Red before green — 2026-10-10-engine-harness-proofs-completed

Three readings of one command, taken at implement on 2026-10-10, each on the tree as it stood then:

    cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(xtask) | binary(integration_console_engine)'

The first two were taken with the tests written and the decisions they pin still holding the base behaviour.
Each stand-in was one line marked in the source and removed before the third reading; `grep -rn` for the mark
over `xtask`, `scripts` and `pulse-app/tests` read 0 hits before the third reading.

## Reading 1 — the decisions hold the base behaviour

What stood in for the new code:

| Subject | Held as |
|---|---|
| `harness_ready::bound_by_this_run` | always true: any accepted connection counts as the run's own (the base decision) |
| `engine_cycle::taken_port_refusal` | never refuses (the base has no such refusal) |
| `pre_push::decide_ci_gates` | always green |
| `source_lint::ROOTS`, the quarantine script's search dirs | without `xtask/tests` (the base sets) |
| the census's read of a process's fd table (test code) | empty |
| the rejected-port arm's variable (test code) | the digits alone, a valid port |
| the lock-file arm's environment (test code) | no temp-dir variable |

Summary line: `484 tests run: 465 passed, 19 failed, 0 skipped`; the command's exit 100.

The 19, with the assertion each failed on:

- `harness_ready::tests::a_held_port_with_no_bound_record_is_foreign_and_not_ready` — left `("accepting", "accepting")`, right `("foreign", "accepting")`
- `harness_ready::tests::a_failed_bind_record_of_the_same_target_does_not_count` — left `("accepting", "accepting")`, right `("foreign", "accepting")`
- `harness_ready::tests::a_bound_record_for_another_port_or_the_other_receiver_does_not_count` — left `"accepting"`, right `"foreign"`
- `harness_ready::tests::a_bound_record_before_the_last_boot_record_does_not_count` — left `("accepting", "accepting")`, right `("foreign", "accepting")`
- `harness_ready::tests::a_log_with_no_boot_record_holds_no_bound_record_of_this_run` — left `("accepting", "accepting")`, right `("foreign", "foreign")`
- `engine_cycle::tests::a_port_that_accepts_in_either_place_is_refused_as_taken` — left `None`, right `Some("port-taken")`
- `engine_cycle::tests::a_listener_on_a_cycle_port_is_read_as_taken_and_a_closed_one_is_not` — left `None`, right `Some("port-taken")`
- `pre_push::tests::a_taken_cycle_port_cannot_be_evaluated_and_never_reads_green` — the decision returned `Ok(())`
- `pre_push::tests::a_cycle_that_could_not_be_evaluated_stops_as_cannot_evaluate` — left `Ok(())`, right `Err(Stop { verdict: CannotEvaluate, reason: "engine-cycle-unevaluated" })`
- `pre_push::tests::a_failed_cycle_is_red` — left `Ok(())`, right `Err(Stop { verdict: Red, reason: "stage-failed:ci-gates" })`
- `pre_push::tests::a_passed_cycle_whose_gates_failed_or_did_not_run_is_red` — left `Ok(())`, right `Err(Stop { verdict: Red, reason: "stage-failed:ci-gates" })`
- `pre_push::tests::a_cycle_that_left_a_port_accepting_or_its_engine_alive_is_red_whatever_it_returned` — left `Ok(())`, right `Err(Stop { verdict: Red, reason: "stage-failed:ci-gates" })`
- `source_lint::tests::english_sources_reads_the_xtask_integration_tests` — exit left `0`, right `1`
- `empty_input_tests::quarantine_check_reads_the_xtask_integration_tests` — the check's exit was 0
- `integration_console_engine::a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening`
  — the census control: `the census reads this process's own listener on port …: Census { sockets: [], unlisted: 0 }`
- `integration_console_engine::the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else` — the same
  control, so an empty census did not pass
- `integration_console_engine::the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives` —
  `the lock file is the only entry: {}`, left `0`, right `1` (with no temp-dir variable the supplied directory
  stayed empty)
- `agent_run_shell::boot_never_ready_over_another_listener_prints_the_foreign_line` — exit left `Some(0)`, right
  `Some(1)`; the verb printed `boot: ready (PID=…, data_dir={scratch}/data)` with the test's own listener
  holding the gRPC port
- `engine_cycle_refusals::a_cycle_over_a_port_another_listener_holds_is_refused_as_port_taken` — exit left
  `Some(1)`, right `Some(2)`, after 62.5 s

The last two are the base behaviour read on a real engine, as research's hand reading had it (n = 1 there, n = 1
here). With the gRPC port held by the test's listener the built cycle verb printed, and this is the whole of
what it said about the cause:

    boot: ready (PID=…, data_dir={scratch}/data)
    "verdict": "not-settled"   ("memory_samples_populated": 0, five ticks of each target)
    "verdict": "running-healthy"
    cleanup: ports-lingering
    engine-log: progress FAIL (no span landed, largest rows_ingested 0 over 5 buffer.tick record(s))
    engine-log: budget FAIL (no memory sample to grade, populated 0 of 5)
    engine-log: FAIL
    {"boot": 0, "settled": 1, "status": 0, "cleanup": 1, "check": 1, "error_records": 2, "verdict": "fail", "witness_file": "absent"}

That cycle ran from the workspace root on the release engine already built there, on the two ports the test
picked; it ended its engine itself (`ps` over the host's process list read no `andromeda-pulse-engine` and no
`inject_demo` after the run).

Every other test of the two new xtask binaries read green in this reading (23 of 25): they pin behaviour the
chunk does not change. Their red reading is the plan's own: the gate entry that selects the two binaries read
exit 94 at P5 on the untouched tree, where neither binary exists.

## Reading 2 — the decisions implemented, one arm still on a valid port

Summary line: `484 tests run: 483 passed, 1 failed, 0 skipped`; exit 100. The one:

- `integration_console_engine::a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening`
  — `the HTTP receiver is the one listener`: the census read two TCP listeners on `127.0.0.1` (both receivers)
  and two unnamed unix sockets, not listening, `unlisted: 0`; left two listeners, right one.

## Reading 3 — the stand-ins removed

Summary line: `484 tests run: 484 passed, 0 skipped`; exit 0.

## One pin added after the first two firings of the gate block

`pre_push::tests::end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names` runs the stage's own teardown
(`end_cycle`, the script's `cleanup` on the stage data dir) against a real process the pin started and named in
that data dir's pid file. Command: `cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(xtask) & test(end_cycle_ends)'`.

- With `end_cycle` returning at once (one marked line, removed after): `1 test run: 0 passed, 1 failed`,
  `the process the pid file named is ended`, left `Some(true)`, right `Some(false)`; exit 100. The pin's own
  last step ended the process it had started (`ps` read no `sleep 600` after the run).
- As committed, with the module's other pins (`test(pre_push::)`): `28 tests run: 28 passed`; exit 0.
