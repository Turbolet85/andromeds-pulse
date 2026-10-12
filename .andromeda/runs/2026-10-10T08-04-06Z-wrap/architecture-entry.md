
## 2026-10-10-no-ci-step-reads-nothing — node as a host need of the workspace tests
**Section:** §Stack and Technologies (one new row)
**Change:** A third test-only host-tool row, beside the Python 3 row and the `cc` row: `node` on PATH on every host that runs the workspace tests. `pulse-app/tests/a11y_perf_workflow.rs` (`a11y_regression_detector_fails_when_its_baseline_is_absent`) runs the existing `pulse-app/ui/tests-a11y/regression-detector.mjs` with an absent baseline and asserts a non-zero exit and a stderr holding `baseline not found`; a missing `node` fails the test, never skips it. No npm or Cargo dependency, no manifest or lockfile change. Dev host read at v26.8.2 (its default) and 24.21.0 (the `mise` install the pre-push check runs under), as measured on 2026-10-10; the runner's Node version was not read.
**Why:** The chunk made a Rust workspace test run Node, a tier §Stack had not stated. Node itself is not new to the stack.
**Kept:** §Infrastructure Patterns → CI/CD approach is not amended: its key file states none of the eight steps the chunk removed from `ci.yml`.
**Ref:** .andromeda/runs/2026-10-10T08-04-06Z-wrap/
