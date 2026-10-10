
## 2026-10-10-no-gate-stands-while-reading-nothing — every nextest invocation fails on an empty selection; bash and awk as runner needs; the untested .ps1 quarantine mirror
**Section:** §9 → Pipeline structure (MCP-feature tests row · Lint + tests row) · §1 → Pending coverage triggers
**Change:**
- MCP-feature tests row: was `… --profile ci --no-tests=pass`; now `--no-tests=fail`, a selection that matches no test fails the job, pinned by `ci_workflow_nextest_runs_fail_on_an_empty_selection`; the runner's cargo-nextest 0.9.133 took the flag and ran 2946 tests on `ci#38042949735`.
- Lint + tests row: the perf-samples line spells `--no-tests=fail`; `cargo xtask test` and `cargo xtask perf:slo-load` pass nextest the same flag from argument lists a pin reads (with `test:coverage` and `perf:load-profiles`); nextest's own exit is 4 and the verb maps a failed status to exit 1; the three steps ran 2908, 1 and 1 tests on that run; the step is named `cargo xtask test`.
- The same row's list of what the workspace tests need on the runner gains `bash` and `awk` on PATH: four witness tests run the `Enforce coverage thresholds` step's script with `bash -e`, five xtask pins run `xtask/ci/quarantine-tracking-check.sh` with `bash`; a missing tool fails them, never skips them; green in the `lint / test` job of `ci#38042949735`, neither version read.
- New pending-trigger row `quarantine-tracking-ps1-mirror-coverage`: the `.ps1` mirror gained the absent-input arms with no committed test (parsed by `pwsh` 7.6.6, 0 parse errors, never run); owed a pin per arm, or the mirror leaves with the PowerShell scripts.
**Why:** Five invocations passed a run that selected no test; the chunk made each fail and spelled the behaviour instead of leaning on a default the runner's older nextest was not read for. The new pins run the gates' own scripts, which is what makes the two tools a runner need.
**Ref:** .andromeda/runs/2026-10-10T10-27-16Z-wrap/
