# Red before green — the pins of steps 1, 2 and 4 against the untouched gates

Run at /implement, step 0, on the chunk base `7419496b` plus the pins alone. The gates were untouched: `ci.yml`,
both quarantine scripts and the `ci-gates` verb as at the base; in `xtask/src/main.rs` the four nextest argument
lists had been lifted into constants so the pin could read them, each still holding its base flag.

Command: `cargo nextest run --workspace --profile ci --no-fail-fast -E 'binary(quality_gate_workflow) + package(xtask)'`

Summary line: `341 tests run: 333 passed, 8 failed, 0 skipped` (2 binaries; exit 100).

## The eight red pins, each with its failing assertion

| pin | file:line of the assertion | what it read |
|---|---|---|
| `coverage_thresholds_step_fails_on_a_report_tracking_nothing` | `pulse-app/tests/quality_gate_workflow.rs:764` | case "an empty report": `left: Some(0)`, `right: Some(1)`; the step printed `Coverage gate: 0 lines / 0 functions tracked (Foundation epoch — gate trivially passes; activates at chunk #15 first tests)` |
| `coverage_job_reads_no_branch_count` | `pulse-app/tests/quality_gate_workflow.rs:835` | "the coverage job MUST hold no `BRF`" |
| `ci_workflow_nextest_runs_fail_on_an_empty_selection` | `pulse-app/tests/quality_gate_workflow.rs:866` | "every `cargo nextest run` of ci.yml MUST spell `--no-tests=fail`"; line: `run: cargo nextest run --workspace --profile perf-samples` |
| `empty_input_tests::every_nextest_argument_list_fails_on_an_empty_selection` | `xtask/src/main.rs:1123` | verb `test`: `left: ["--no-tests=pass"]`, `right: ["--no-tests=fail"]` |
| `empty_input_tests::quarantine_check_fails_when_no_search_dir_exists` | `xtask/src/main.rs:1167` | `left: Some(0)`, `right: Some(1)`; printed `quarantine-tracking-check: NEUTRAL (zero #[ignore] in source; gate establishes convention for future quarantines)` |
| `empty_input_tests::quarantine_check_fails_on_a_scan_of_no_source_file` | `xtask/src/main.rs:1179` | `left: Some(0)`, `right: Some(1)`; the same NEUTRAL line |
| `empty_input_tests::quarantine_check_says_how_many_files_it_scanned` | `xtask/src/main.rs:1189` | exit 0, and the NEUTRAL line where `PASS (0 quarantine(s) across 1 file(s))` is asserted |
| `empty_input_tests::quarantine_check_passes_an_ignore_with_its_issue_url` | `xtask/src/main.rs:1214` | exit 0, printed `quarantine-tracking-check: PASS (1 quarantine(s) tracked via GitHub issue URL)`: no file count |

## The four pins that were green at the base

They pin arms the base already had, and stay as guards of what the chunk keeps:
`coverage_thresholds_step_fails_without_a_report`, `coverage_thresholds_step_passes_a_report_over_both_thresholds`,
`coverage_thresholds_step_fails_a_report_under_either_threshold`,
`empty_input_tests::quarantine_check_fails_an_ignore_without_an_issue_url`.

## Limits of this reading, and one direct read beside it

- A looped pin stops at its first failing case. The tracking-nothing witness reached only "an empty report"; the
  argument-list pin reached only the verb `test`; the workflow pin reached only the `perf-samples` line.
- The other two zero-total cases of the witness were therefore read directly, by cutting the untouched step's body
  out of `ci.yml` and running it with `bash -e` over the same constructed reports in a scratch dir:

  ```
  empty report: exit 0
      Coverage gate: 0 lines / 0 functions tracked (Foundation epoch — gate trivially passes; activates at chunk #15 first tests)
  lines and no function: exit 0
      Line:     90/100 = 90.0% (threshold 75%)
      Branch:   0/0 = 100.0% (threshold 70%)
      Function: 0/0 = 100.0% (threshold 85%)
  functions and no line: exit 0
      Line:     0/0 = 100.0% (threshold 75%)
      Branch:   0/0 = 100.0% (threshold 70%)
      Function: 9/10 = 90.0% (threshold 85%)
  ```

- The `ci-gates` unit's pins are not in this run: they are written with the unit (step 3), and their red reading
  is the mutation of step 5 (`mutation-checks.md`).
