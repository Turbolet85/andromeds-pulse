# One mutation per arm of `check:engine-log` — 2026-10-10-engine-harness-proofs-completed

Run once at implement, 2026-10-10, in `xtask/src/engine_log.rs`. Nothing of it is committed: after the seventh
revert `git diff --quiet 514fbe575d0446decebc98296192c52f30019428 -- xtask/src/engine_log.rs` exits 0 and the
file's blob id equals the base's (`493d908e`).

Each mutation was applied with the anchored Edit tool, read back from `git diff -U0` against the chunk base (the
one changed line, shown below) before anything was run, and reverted the same way before the next one. Under
each, two readings:

- **Pins:** `cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(xtask) & test(engine_log::)'`
  (45 tests: the module's own). Unmutated: `45 tests run: 45 passed`.
- **The real log:** `cargo xtask check:engine-log` with the data dir of the engine cycle this chunk's gate block
  ran, `target/engine-cycle/20261010T184357Z` (one console engine, 335 records, a healthy feed over real OTLP on
  loopback port 24317, ended by the cycle's own cleanup; `"verdict": "pass"`).

Six of the module's pins read the whole verdict (its PASS, or its exit 2 where another arm cannot be evaluated),
so a mutation that makes any arm FAIL reddens them whatever arm it is: `a_complete_console_log_passes_every_arm_in_order`,
`an_unrelated_file_beside_the_family_changes_nothing`, `a_log_with_no_boot_record_cannot_be_evaluated`,
`a_log_with_no_buffer_tick_cannot_be_evaluated_for_progress`, `fewer_than_two_ticks_of_a_target_cannot_be_evaluated`,
`two_boots_in_one_family_cannot_be_evaluated`. They are named "whole-verdict" below. No pin that reads another
arm's own line alone reddened under any mutation.

## Before — unmutated, over the real log (exit 0)

    engine-log: family PASS (335 records across 1 file(s))
    engine-log: program PASS (one app.boot.engine record, reading console)
    engine-log: panic PASS (no app.panic.fatal record at ERROR)
    engine-log: heartbeat-gap PASS (ingest.tick max gap 15001 ms (n=2); buffer.tick max gap 15001 ms (n=2); connection.tick max gap 15001 ms (n=2))
    engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
    engine-log: process-end PASS (one app.exit record, the last of the family)
    engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
    engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
    engine-log: PASS

## 1 — `family`

- Changed line (`family_arm`, the arm for a family that holds records): `State::Pass,` → `State::Fail,`
- Pins: `45 tests run: 39 passed, 6 failed`. The six whole-verdict pins, the first of which also reads this arm's
  own PASS.
- Real log, the arm's line: `engine-log: family FAIL (335 records across 1 file(s))`; last line `engine-log: FAIL`;
  exit 1. Every other arm's line as before.

## 2 — `program`

- Changed line (`program_arm`, the guard of the passing arm): `[program] if *program == Program::Console.label() =>`
  → `[program] if *program == Program::Window.label() =>`
- Pins: `45 tests run: 42 passed, 3 failed`: `a_boot_record_that_reads_window_fails` (the arm's own), and
  whole-verdict `a_complete_console_log_passes_every_arm_in_order`, `an_unrelated_file_beside_the_family_changes_nothing`.
- Real log, the arm's line: `engine-log: program cannot-evaluate (the app.boot.engine record names no known program)`;
  last line `engine-log: cannot-evaluate`; exit 2. Every other arm's line as before.

## 3 — `panic`

- Changed line (`panic_arm`, the arm for a family with no panic record):
  `ArmReading::new(arm, State::Pass, "no app.panic.fatal record at ERROR")` → `State::Fail` in the same call
- Pins: `45 tests run: 38 passed, 7 failed`: `a_panic_target_record_below_error_passes` (the arm's own) and the
  six whole-verdict pins.
- Real log, the arm's line: `engine-log: panic FAIL (no app.panic.fatal record at ERROR)`; last line
  `engine-log: FAIL`; exit 1. Every other arm's line as before.

## 4 — `heartbeat-gap`

- Changed line (`tick_gap`, the comparison): `if largest > MAX_TICK_GAP_MS {` → `if largest < MAX_TICK_GAP_MS {`
- Pins: `45 tests run: 36 passed, 9 failed`: the arm's own `a_gap_of_exactly_45_seconds_passes`,
  `a_tick_with_no_readable_timestamp_cannot_be_evaluated`, `viz_and_plugins_ticks_are_never_graded`, and the six
  whole-verdict pins (one of them, `fewer_than_two_ticks_of_a_target_cannot_be_evaluated`, is this arm's too).
  `a_gap_over_45_seconds_between_two_ticks_of_one_target_fails_with_no_error_record` stayed green under this
  inversion: its log holds ordinary gaps on the two other targets, which the inverted comparison fails, so the
  arm still reads FAIL there.
- Real log, the arm's line:
  `engine-log: heartbeat-gap FAIL (ingest.tick gap 15001 ms > 45000 ms; buffer.tick gap 15001 ms > 45000 ms; connection.tick gap 15001 ms > 45000 ms)`;
  last line `engine-log: FAIL`; exit 1. Every other arm's line as before.

## 5 — `progress`

- Changed line (`progress_arm`, the pattern of the no-span arm): `Some(0) => ArmReading::new(` →
  `Some(1..) => ArmReading::new(`
- Pins: `45 tests run: 38 passed, 7 failed`: the arm's own `a_run_in_which_no_span_landed_fails` and
  `a_recovery_announcement_passes`, and five whole-verdict pins (all but
  `a_log_with_no_buffer_tick_cannot_be_evaluated_for_progress`, whose log this arm cannot grade either way).
- Real log, the arm's line:
  `engine-log: progress FAIL (no span landed, largest rows_ingested 0 over 2 buffer.tick record(s))`; last line
  `engine-log: FAIL`; exit 1. Every other arm's line as before.

## 6 — `process-end`

- Changed line (`process_end_arm`, the guard of the passing arm): `[only] if only + 1 == lines.len() =>` →
  `[only] if only + 2 == lines.len() =>`
- Pins: `45 tests run: 37 passed, 8 failed`: the arm's own `a_record_after_the_exit_record_fails` and
  `a_kill_record_does_not_excuse_a_log_that_holds_its_end`, and the six whole-verdict pins.
- Real log, the arm's line: `engine-log: process-end FAIL (0 record(s) after the app.exit record)`; last line
  `engine-log: FAIL`; exit 1. Every other arm's line as before.

## 7 — `budget`

- Changed line (`budget_arm`, the arm for a graded sample within the budget): `State::Pass,` → `State::Fail,`
- Pins: `45 tests run: 37 passed, 8 failed`: the arm's own `a_memory_sample_at_the_budget_passes` and
  `one_line_says_frame_and_snapshot_are_not_graded`, and the six whole-verdict pins.
- Real log, the arm's line: `engine-log: budget FAIL (memory max 138240 B <= 512000000 B, n=2, populated 1)`;
  last line `engine-log: FAIL`; exit 1. Every other arm's line as before.

## After — every mutation reverted

- Pins: `45 tests run: 45 passed`.
- Real log: the nine lines of "Before", byte for byte; exit 0.

## What this shows, and what it does not

- Under each of the seven mutations at least one pin reddened and the mutated arm's line over the real log
  differed from its unmutated line. No mutation left the suite green.
- No arm, mutated or not, read PASS over an input it cannot grade in these readings; the one input every reading
  used is a log all seven arms can grade.
- One mutation per arm, chosen to turn the arm's passing reading: a mutation that weakens an arm's FAIL side (a
  missed panic, a missed gap) was not run here and is pinned only by the module's own tests.
- n = 1 real log. It is a healthy cycle on the dev host; no red cycle was read on a runner in this chunk.
