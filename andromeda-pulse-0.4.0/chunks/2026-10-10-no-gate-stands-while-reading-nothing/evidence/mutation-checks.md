# Mutation checks — step 5, one shot each

Run at /implement after the pins were green on the edited gates (`347 tests run: 347 passed` over
`binary(quality_gate_workflow) + package(xtask)`). For each kept gate the BASE form of its block was put back, the
mutation was confirmed present by a grep before the run, the gate's pins were run, and the file was restored.

**Restore, for all four:** the four files touched (`.github/workflows/ci.yml`, `xtask/src/main.rs`,
`xtask/src/ci_gates.rs`, `xtask/ci/quarantine-tracking-check.sh`) were copied and checksummed before the first
mutation; after the last restore all four sha256 sums equal the kept ones (`diff` of the two sum lists: exit 0), the
script's mode is unchanged (644) and `cargo fmt --check` exits 0.

## (a) The coverage step's zero-total block → the witness

- **Put back:** the `&&` test with the "trivially passes" line and `exit 0`, and the two percentage lines that print
  `100.0` over a zero total. Present before the run: 3 lines match `trivially passes|if (t==0) print "100.0"`.
- **Run:** `-E 'binary(quality_gate_workflow) & test(/^coverage_/)'` → `5 tests run: 4 passed, 1 failed`.
- **Red:** `coverage_thresholds_step_fails_on_a_report_tracking_nothing`, case "an empty report": `left: Some(0)`,
  `right: Some(1)`, the step printing the "trivially passes" line.
- **Green under the mutation** (they pin arms the mutation leaves alone): the over-threshold case, the two
  under-threshold cases, the no-report case, and the no-branch pin.
- **The other two cases, read directly** (the looped pin stops at its first case): the mutated step's body, cut
  from `ci.yml` and run with `bash -e` in a scratch dir, read `lines and no function: exit 0` with
  `Function: 0/0 = 100.0%`, and `functions and no line: exit 0` with `Line: 0/0 = 100.0%`. After the restore the
  same three reports read exit 1 with `::error::lcov.info tracks {n} lines and {m} functions …`.

## (b) The retired flag in one argument list and on the `mcp-test` line → the two pins

- **Put back:** the base flag value in `TEST_ARGS` (`xtask/src/main.rs`) and on the `mcp-test` step's `run:` line
  (`ci.yml`). Present before the run: one line in each file.
- **Run:** `-E 'test(/empty_selection/)'` → `2 tests run: 0 passed, 2 failed`.
- **Red:** `ci_workflow_nextest_runs_fail_on_an_empty_selection` names the `mcp-test` line;
  `empty_input_tests::every_nextest_argument_list_fails_on_an_empty_selection` reads, for the verb `test`,
  `left` holding the base flag and `right: ["--no-tests=fail"]`.

## (c) The absent-log arm returning success with the NEUTRAL lines → the unit's exit-2 pins

- **Put back:** in `xtask/src/ci_gates.rs`, the no-member verdict maps to exit 0 and prints the base's four
  NEUTRAL lines. Present before the run: 4 NEUTRAL lines in the unit and the exit arm folded into the pass arm.
- **Run:** `-E 'package(xtask) & test(/^ci_gates::/)'` → `6 tests run: 4 passed, 2 failed`.
- **Red:** `ci_gates::tests::an_absent_log_dir_cannot_be_evaluated` and
  `ci_gates::tests::a_log_dir_with_no_family_member_cannot_be_evaluated`, both `left: 0`, `right: 2` on the exit.
- **Green under the mutation:** the four pins of the arms that read a log.
- **The verb itself, mutated**, over a data dir that does not exist: exit 0 and four NEUTRAL lines, the base
  reading.

## (d) The quarantine script's base absent-dir arm → its pins

- **Put back:** the base loop's silent skip of a missing dir and the base zero-match arm (the NEUTRAL line,
  exit 0); the missing-dir failure and the zero-file failure removed. A one-line removal of the missing-dir
  check alone would not have restored the defect: the zero-file check also fails an absent root.
- **Run:** `-E 'package(xtask) & test(/quarantine_check/)'` → `5 tests run: 2 passed, 3 failed`.
- **Red:** `quarantine_check_fails_when_no_search_dir_exists` and
  `quarantine_check_fails_on_a_scan_of_no_source_file` (both `left: Some(0)`, `right: Some(1)`, the NEUTRAL line
  printed), and `quarantine_check_says_how_many_files_it_scanned` (the NEUTRAL line where the PASS line with its
  file count is asserted).
- **Green under the mutation:** the two pins of an `#[ignore]` with and without its issue URL.

## Limits

- Each mutation was one shot: no pin was run twice under it.
- The `.ps1` mirror of the quarantine script was parsed with PowerShell's own parser (0 parse errors) and not run;
  no pin reads it.
