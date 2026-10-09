# Red before green — the workflow self-lint against `ci.yml`

Chunk `2026-10-09-ci-on-linux-alone`, plan Steps 2 and 4. One call, run twice at /implement on 2026-10-09 (UTC),
once before any edit to `.github/workflows/ci.yml` and once after:

    cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow)'

The test file (`pulse-app/tests/quality_gate_workflow.rs`) carried this chunk's edits in both runs: the release-job
test deleted, the job count 6, the two new pins added.

## Reading 1 — the workflow as it stands at the chunk base (`0b61bfb`), before Step 3

Exit 100. `Summary: 19 tests run: 17 passed, 2 failed, 0 skipped`.

Failing, 2:

- `ci_workflow_runs_on_linux_only` — panicked at `pulse-app/tests/quality_gate_workflow.rs:116:13`:

      ci.yml:20: the workflow MUST name no other system, matrix value or system condition; found `macos` in:
      # lint-test OS, one per release OS (macOS, Windows) and one for boot; a11y,

  Line 20 is the first of the 20 lines the base carries with an other-system token; the assertion stops at the first.

- `data_dir_export_precedes_every_consumer` — panicked at `pulse-app/tests/quality_gate_workflow.rs:464:9`:

      assertion `left == right` failed: ci.yml: expected 6 jobs, found 7
        left: 7
       right: 6

Passing, 17:

- `ci_workflow_keeps_the_linux_release_build_witnesses` (both builds are already in the workflow at the base, as the
  plan forecast; its control is the mutation in `mutation-checks.md`)
- `ci_workflow_clippy_uses_deny_warnings`
- `ci_workflow_downloads_coverage_baseline_artifact`
- `ci_workflow_downloads_criterion_baseline_artifact`
- `ci_workflow_env_includes_ci_run_id`
- `ci_workflow_invokes_ci_gates`
- `ci_workflow_invokes_coverage_regression_check`
- `ci_workflow_invokes_criterion_regression_check`
- `ci_workflow_invokes_quarantine_tracking_check`
- `ci_workflow_test_gates_no_continue_on_error`
- `ci_workflow_uploads_criterion_artifact_unchanged`
- `ci_workflow_uploads_logs_artifact_unchanged`
- `nextest_ci_profile_enforces_zero_retries`
- `nextest_default_profile_excludes_load_profiles_suite`
- `nextest_default_profile_excludes_perf_budget_samples_producer`
- `nextest_load_profiles_profile_preserves_zero_flake_posture`
- `workflow_env_references_no_step_only_context`

## Reading 2 — after Step 3's edit of `ci.yml`

Exit 0. `Summary: 19 tests run: 19 passed, 0 skipped`.

Read beside it on the edited file: `grep -c -i -E 'macos|windows|matrix\.os|runner\.os ==' .github/workflows/ci.yml`
prints 0 (20 at the base), and `grep -n 'runs-on:'` prints six lines, each `runs-on: ubuntu-22.04`.

## What the pair shows, and what it does not

`ci_workflow_runs_on_linux_only` and the job count of 6 discriminate: each read red on the three-system workflow and
green on the edited one, with no change to the test file between the two runs. The pair says nothing about the witness
pin, which passes in both readings by construction.
