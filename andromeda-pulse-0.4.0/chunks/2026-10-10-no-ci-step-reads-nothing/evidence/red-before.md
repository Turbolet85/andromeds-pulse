# Red before — 2026-10-10-no-ci-step-reads-nothing

Plan step 1. Read by /implement on 2026-10-10 (run ended 2026-10-10T07:19:29Z), on the working tree with the five
pins written and nothing else of the chunk edited: `.github/workflows/ci.yml`,
`pulse-app/ui/tests-a11y/regression-detector.mjs` and `xtask/` were byte-identical to the chunk base `6df9e95c`
(`git diff --quiet 6df9e95c -- .github xtask pulse-app/ui`: exit 0, read after the run).

**Run:** `cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow) + binary(a11y_perf_workflow)'`
— exit 100; `Starting 41 tests across 2 binaries (128 binaries skipped)`;
`Summary 41 tests run: 37 passed, 4 failed, 0 skipped`.

The plan predicted four red and one green. Measured: four red, one green, the same five.

## Red

- `quality_gate_workflow::ci_workflow_makes_no_download_without_a_producer` — panicked at
  `pulse-app/tests/quality_gate_workflow.rs:372:9`: ``ci.yml downloads `criterion-${{ runner.os }}-base`, which no
  workflow uploads under that name``. The block it printed is the step
  `Download base-branch criterion baseline (PR only)`.
- `quality_gate_workflow::ci_workflow_uploads_fail_when_they_find_no_file` — panicked at
  `pulse-app/tests/quality_gate_workflow.rs:393:9`: ``every upload step of ci.yml MUST carry `if-no-files-found:
  error` ``. The block it printed is the first upload of the file, `Upload perf-samples logs artifact`, holding
  `if-no-files-found: ignore`.
- `quality_gate_workflow::ci_workflow_test_gates_no_continue_on_error` — panicked at
  `pulse-app/tests/quality_gate_workflow.rs:296:9`, the new arm: ``ci.yml: no step may carry `continue-on-error` ``.
  The block it printed is `Download base-branch criterion baseline (PR only)`, holding `continue-on-error: true`. The
  arm the test already had did not fire: that step holds none of the sixteen listed commands.
- `a11y_perf_workflow::a11y_regression_detector_fails_when_its_baseline_is_absent` — panicked at
  `pulse-app/tests/a11y_perf_workflow.rs:136:5`: `the detector MUST fail, not pass, when its baseline is absent`,
  with `status: exit status: 0` and the detector's own line on stderr,
  `regression-detector: baseline not found at {tmp}/absent-baseline.json; treating as empty baseline` (`{tmp}` is
  the test's scratch directory). The pin's first assertion, that stderr says the baseline was not found, held; the
  exit assertion is the one that failed.

## Green

- `a11y_perf_workflow::a11y_regression_baseline_is_committed_in_the_tree` — PASS: the baseline file is present and
  parses with a `per_surface` object.
- The 36 other tests of the two binaries, the five pins step 4 deletes among them (their steps were still in the
  workflow).
