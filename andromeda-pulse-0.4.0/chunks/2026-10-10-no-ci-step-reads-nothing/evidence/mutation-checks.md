# Mutation checks — 2026-10-10-no-ci-step-reads-nothing

Plan step 7. Run by /implement on 2026-10-10, between 2026-10-10T07:21:52Z and 2026-10-10T07:23:16Z, each by hand on
the working tree: the mutation applied with an anchored edit, confirmed present by `grep` before the run, the pins
run, the mutation removed, the file's sha256 read equal to its value before any mutation. One at a time.

**The run, every time:**
`cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow) + binary(a11y_perf_workflow)'`.

**Before any mutation:** exit 0; `Starting 36 tests across 2 binaries`; `36 tests run: 36 passed, 0 skipped` (the 41
of the red-before run less the five pins plan step 4 deleted).

**The two files mutated, sha256 before any mutation:**
- `.github/workflows/ci.yml` — `48bd97b6c7a129bad9324369e1873180f8ca5067529fb08e6e44dccfa89b2b16`
- `pulse-app/ui/tests-a11y/regression-detector.mjs` — `75096f4c2898fd517049b590acdfb47f92d5ccc5dba4b2e62fe9b1238cec39aa`

## 1. One `download-artifact` step restored

- **Mutation:** in `ci.yml`, job `a11y`, the step `Download base-branch a11y-violations baseline (PR only)` put back
  before `cargo xtask test:a11y`, in its form at the chunk base `6df9e95c` (seven lines, its `continue-on-error: true`
  among them).
- **Present before the run:** `grep -n -E 'download-artifact@|a11y-violations-base'` printed `:257` and `:260`.
- **Run:** exit 100; 36 tests run, 34 passed, 2 failed.
- **Red:**
  - `ci_workflow_makes_no_download_without_a_producer` (the plan's pin) — ``ci.yml downloads `a11y-violations-base`,
    which no workflow uploads under that name``.
  - `ci_workflow_test_gates_no_continue_on_error` — the every-step arm, on the same restored step: the base form of
    the step carries the soft-fail key.
- **Removed:** `grep -c 'download-artifact@'` reads 0; sha256 equal to the value above.

The plan forecast the first pin red. The guard went red with it, because the step was restored whole.

## 2. One upload's key set back to `ignore`

- **Mutation:** in `ci.yml`, job `coverage`, step `Upload coverage artifact`: `if-no-files-found: error` →
  `if-no-files-found: ignore`.
- **Present before the run:** `grep -n 'if-no-files-found'` printed six lines, five `error` and `:541` `ignore`.
- **Run:** exit 100; 36 tests run, 35 passed, 1 failed.
- **Red:** `ci_workflow_uploads_fail_when_they_find_no_file` — the block it printed is `Upload coverage artifact`.
- **Removed:** `grep -c 'if-no-files-found: ignore'` reads 0; sha256 equal to the value above.

## 3. `continue-on-error: true` on a step the command list never read

- **Mutation:** in `ci.yml`, job `lint-test`, the line `continue-on-error: true` added under the `name:` line of
  `cargo xtask typecheck (tsc --noEmit against TauRPC bindings)`.
- **Present before the run:** `grep -n -B1 -A1 'continue-on-error'` printed `:91` between the step's `name:` and
  `run:` lines, and no other hit.
- **Run:** exit 100; 36 tests run, 35 passed, 1 failed.
- **Red:** `ci_workflow_test_gates_no_continue_on_error`, panicked at
  `pulse-app/tests/quality_gate_workflow.rs:268:9`, the every-step arm; the block it printed is the typecheck step.
  The arm the test had before this chunk did not fire: the step holds none of the fourteen listed commands.
- **Removed:** `grep -c 'continue-on-error'` reads 0; sha256 equal to the value above.

## 4. The detector's absent-baseline exit

Two forms were run. The plan's wording is the first; the second is the one that restores the defect.

### 4a. The new exit line removed, nothing else (the plan's wording)

- **Mutation:** in `regression-detector.mjs`, the line `process.exit(1);` taken out of the absent-baseline block; the
  message line stays.
- **Present before the run:** the block read back with the `console.error` line directly followed by `}`.
- **Run:** exit 0; 36 tests run, 36 passed. **The pin stays green under this form.**
- **Why, measured:** run by hand under the mutation with an absent baseline and a current summary holding no tuple,
  the script exits 1 and prints two lines on stderr, `regression-detector: baseline not found at {tmp}/absent.json`
  and `regression-detector: Error: ENOENT: no such file or directory, open '{tmp}/absent.json'`. Step 5 also took out
  the arm that read an absent baseline as an empty one, so with the exit gone the read of the file fails and the
  script still ends non-zero. The property the pin holds (the comparison fails when its baseline is absent) is true
  under this mutation; the mutation does not restore the defect.

### 4b. The block put back in its base form

- **Mutation:** the absent-baseline block and the baseline read put back as they stand at the chunk base: the
  message ends `; treating as empty baseline`, no exit, and the baseline is `{ per_surface: {} }` when the file is
  absent.
- **Present before the run:** `git diff --quiet 6df9e95c -- pulse-app/ui/tests-a11y/regression-detector.mjs` exit 0
  (the file equal to its base form); `grep -n` printed `:63` and `:69`.
- **Run:** exit 100; 36 tests run, 35 passed, 1 failed.
- **Red:** `a11y_regression_detector_fails_when_its_baseline_is_absent`, panicked at
  `pulse-app/tests/a11y_perf_workflow.rs:136:5` with `status: exit status: 0` and the stderr line
  `regression-detector: baseline not found at {tmp}/absent-baseline.json; treating as empty baseline`. The same
  reading as the red-before run.
- **Removed:** `grep -c 'treating as empty baseline'` reads 0; sha256 equal to the value above.

## After the last mutation

Both files' sha256 equal to the values above. The run: exit 0; 36 tests run, 36 passed, 0 skipped.

## What the checks say about the pins

- The download pin, the upload pin and the guard's every-step arm each go red on the mutation the plan named for
  them.
- The detector pin goes red on the defect as it stood (4b) and not on the removal of the exit line alone (4a). The
  plan forecast red for the wording of 4a. The absent-baseline failure is held in two places in the script after
  step 5: the exit, and the read with no empty-baseline arm.
- `a11y_regression_baseline_is_committed_in_the_tree` was not mutated: the plan lists no mutation for it.

A side effect, on the host only: each run that reached the comparison (the red-before run and 4b) wrote
`pulse-app/ui/tests-a11y/regression-set.json`, which git ignores (`.gitignore:124`).
