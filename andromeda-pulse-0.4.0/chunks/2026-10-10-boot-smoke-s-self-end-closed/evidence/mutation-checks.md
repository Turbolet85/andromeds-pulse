# Mutation checks — 2026-10-10-boot-smoke-s-self-end-closed

Run by /implement on 2026-10-10, each by hand on the working tree: the mutation applied with an anchored edit,
confirmed present by `grep` before the run, the pins run, the mutation removed, its absence confirmed by `grep`.
One at a time; the gate block was fired only after the third was removed and `cargo fmt --check` read exit 0.

Before any mutation (gate entries 5 and 6, first targeted run): `harness_series::` 25 tests run, 25 passed (the 16
pins the base held and 9 new); `unit_xlib_threads` 6 tests run, 6 passed.

## 1. The own-end decision made to return false (plan step 2)

- **Mutation:** `ended_by_itself` in `xtask/src/harness_series.rs` returns `false && !matches!(…)`.
- **Present before the run:** `grep -n -A3 '^fn ended_by_itself'` printed the `false` line under the signature.
- **Run:** `cargo nextest run -p xtask --profile ci -E 'test(/harness_series::/)'` — exit 100; 25 tests run, 21
  passed, 4 failed.
- **Red:**
  - `a_boot_that_ended_by_itself_before_ready_is_ended_with_its_record_and_label` (the plan's first pin)
  - `another_signal_is_an_end_the_app_made_itself` (the plan's third pin)
  - `the_smokes_own_end_is_listed_as_ordinal_1_and_stays_out_of_the_counts`
  - `a_witness_file_outside_the_grammar_reads_unreadable_and_the_boot_is_still_ended`
- **Green under the mutation, as they must be:** the `TERM` / `KILL` pin, the no-exit-record pin, the fixture read
  pin, the member-set pin and the 16 pins of the base. They assert nothing the decision's positive arm carries.
- **Removed:** `grep` shows `!matches!(` directly under the signature.

The plan forecast the first and third pins red. Two more went red: the smoke's listing and the still-`Ended`
half of the unreadable-witness pin both pass through the same decision.

## 2. The call moved below the runtime build (plan step 5)

- **Mutation:** in `pulse-app/src/main.rs`, `xlib_threads::init();` taken from under the render-posture statement
  and placed after `.expect("failed to build tokio runtime");`.
- **Present before the run:** `grep -n` printed the call on the line after the `expect` line.
- **Run:** `cargo nextest run --workspace --profile ci -E 'binary(unit_xlib_threads)'` — exit 100; 6 tests run, 5
  passed, 1 failed.
- **Red:** `main_calls_it_second_after_the_render_posture_step_and_before_the_runtime`, on `the second statement
  of main`: left `let runtime = tokio::runtime::Builder::new_multi_thread()`, right `xlib_threads::init();`.
- **Green under the mutation:** the three arm pins, the soname pin and the one-call-site pin (the call still
  stands once, in `main.rs`).

## 3. The call moved above the render-posture step (plan step 5)

- **Mutation:** `xlib_threads::init();` placed as the first statement of `main`, the render-posture statement
  second.
- **Present before the run:** `grep -n` printed the call one line above `apply_linux_default();`.
- **Run:** the same command — exit 100; 6 tests run, 5 passed, 1 failed.
- **Red:** the same pin, on `the first statement of main`: left `xlib_threads::init();`, right
  `let render_posture = render_posture::apply_linux_default();`.
- **Removed:** `grep -n` prints `apply_linux_default();` on line 281, `xlib_threads::init();` on line 282 and the
  runtime's `expect` on line 293.

## Limits

- The place pin reads the text of `fn main()`: its first two lines that are neither blank nor a comment, compared
  whole. A statement spread over several lines ahead of the call, or the call written through another path
  (`pulse_app::xlib_threads::init()`), reads red, not green.
- No mutation was run on the three arm pins: each asserts one returned value of one input.
- The raw nextest logs of the three runs are in the session scratch directory and are not kept.
