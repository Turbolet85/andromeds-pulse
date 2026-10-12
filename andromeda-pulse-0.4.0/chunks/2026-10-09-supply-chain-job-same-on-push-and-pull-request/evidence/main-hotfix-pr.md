## What this changes

Two files, nothing else:

- `.github/workflows/ci.yml` — the `cargo audit (RustSec advisory DB)` step of the `supply-chain` job becomes a plain
  `run: cargo audit`. The `rustsec/audit-check` action and its `token` input are removed.
- `pulse-app/tests/quality_gate_workflow.rs` — a new test, `ci_workflow_audit_step_is_a_plain_run_step`, holds the
  step in that shape (no action, no token, no soft-fail, no job-level `permissions:`), and `cargo audit` joins the
  list of gates that may not carry `continue-on-error`.

## Why

`main` has been red since the push that merged #39.

- Push run `37907730264` on `60ef43c`: the audit found no vulnerability and 10 informational warnings, then the step
  failed with `Resource not accessible by integration - …/checks/runs#create-a-check-run`. The nine steps after it
  were skipped.
- Pull-request run `37904682919` on `0e45d58`: the same audit result and the same denial, then "Posting audit report
  here instead" and a green step.

The action tries to publish its report as a check run. The workflow token is `contents: read`, so the call is denied.
The action swallows that denial on a pull-request event only; on a push it fails the step. So the job failed on its
own reporting, not on a finding, and only where a green pull request could not show it.

A plain `cargo audit` publishes nothing, receives no token and reads no event. It ends the same on both events:
exit 0 when clean, 1 on a vulnerability, 2 when it could not evaluate. No permission is widened. Informational
warnings still do not fail the job, as before.

## What it does not change

- Nothing else of 0.4.0 is here. `main` keeps its seven jobs and its three-system matrix.
- No other step of `supply-chain` changes, and no other job.
- `Cargo.lock`, `deny.toml` and every source file outside the one test file are untouched.

## How it was checked

- The new test was read red against the workflow as it stood (it printed the action's three lines), then green
  after the edit.
- This branch is the 0.4.0 build branch's own diff for these two files, applied to `main`, not typed twice. So the
  version's later merge takes the repair once, without a conflict (measured by trial merges).
- `main`'s test file with this patch, compiled and run alone against `main`'s workflow with this patch: 19 of 19
  pass.
- `cargo audit` on the project lockfile, cargo-audit 0.22.2 (the version the `ubuntu-22.04` runner image carries):
  exit 0, 10 allowed warnings.
- The red control, on a developer host and not on a runner: the same command on a two-package lockfile naming
  `time 0.1.43` exits 1 and names `RUSTSEC-2020-0071`.

## What witnesses the repair

A green run on this pull request does not: the old step was green on pull requests too. The witness is the push run
that merging this makes on `main`, in its `supply-chain` check.

The merge is the founder's. Nothing here merges it or pushes to `main`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
