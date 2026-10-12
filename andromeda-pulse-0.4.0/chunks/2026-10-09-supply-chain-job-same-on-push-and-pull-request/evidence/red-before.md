# Red before, green after — the audit-step pin

Both readings are the same command, on this host, at /implement (2026-10-09, between 16:33Z and 16:35Z):

```
cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow)'
```

## Red — the pin written, the workflow as it stood at `b3e5859`

The tree held step 1's two insertions in `pulse-app/tests/quality_gate_workflow.rs` and the unedited
`.github/workflows/ci.yml`.

- Exit 100. `Summary [   0.034s] 20 tests run: 19 passed, 1 failed, 0 skipped`.
- The one failure: `FAIL [   0.007s] ( 4/20) pulse-app::quality_gate_workflow ci_workflow_audit_step_is_a_plain_run_step`.
- Its panic, whole:

```
thread 'ci_workflow_audit_step_is_a_plain_run_step' panicked at pulse-app/tests/quality_gate_workflow.rs:294:5:
the audit step MUST be the plain step `run: cargo audit`, whose exit is the same on a push and on a pull request (security-plan §Dependency Security, CI integration; P-120); step:
        uses: rustsec/audit-check@69366f33c96575abad1ee0dba8212993eecbe998 # v2.0.0
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

- `ci_workflow_test_gates_no_continue_on_error`, which gained the `"cargo audit"` substring in the same step, passed:
  the step carried no `continue-on-error`. Its discrimination is read in `mutation-checks.md`.

The test stops at its first failing assertion (the missing `run: cargo audit` line), so this reading shows one of
the pin's four assertions failing. The other three are read one by one in `mutation-checks.md`.

## Green — the same tree plus step 3's edit of the audit step

- Exit 0. `Summary [   0.019s] 20 tests run: 20 passed, 0 skipped`.

## One reading beside the plan's gates: `main`'s own test file, patched

The plan lists no local run of `main`'s tests (a second checkout would be a cold build). This reading costs no
build of the workspace, so it was taken; it is not a gate and nothing rests on it alone.

- A scratch directory outside the repository (`{scratch}/mainsim`) received `main`'s copies (`git show 60ef43c:…`)
  of `.github/workflows/*.yml`, `.config/nextest.toml` and `pulse-app/tests/quality_gate_workflow.rs`.
- `evidence/main-hotfix.patch` was applied there with `git apply -p1`: exit 0. The patched workflow holds
  `run: cargo audit` at `ci.yml:494`, under the name line at `:493`.
- `main`'s patched test file was compiled alone (`rustc --edition 2024 --test`, `CARGO_MANIFEST_DIR` pointing at the
  scratch `pulse-app`) and run: `test result: ok. 19 passed; 0 failed`, with
  `ci_workflow_audit_step_is_a_plain_run_step ... ok` and `ci_workflow_test_gates_no_continue_on_error ... ok` among
  them. `main`'s file has 19 tests where the build branch's has 20 at this point.
- A first run of that binary read 17 passed, 2 failed: the scratch directory then held `ci.yml` alone, and two tests
  read `release.yml`. That was the scratch copy's gap, not the patch's; the reading above is the run after the other
  three workflow files were added.
- What it does not show: the rest of `main`'s workspace tests, and anything about the runner. The second branch's
  own pull-request run is the gate for those.
