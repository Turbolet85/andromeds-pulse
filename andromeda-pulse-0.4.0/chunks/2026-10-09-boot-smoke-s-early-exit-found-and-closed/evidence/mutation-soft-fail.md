# The soft-fail pin, shown to discriminate by a mutation

`ci_workflow_boot_smoke_carries_no_soft_fail` (`pulse-app/tests/quality_gate_workflow.rs`) was green on arrival:
the smoke step at the chunk base carries no soft-fail key, so the pin had no red reading before the edit
(`red-before-green.md`). A green-on-arrival guard proves nothing until it is shown to go red.

## The mutation — 2026-10-09T19:14:34Z

- One line added to the edited smoke step in `.github/workflows/ci.yml`, directly above its `run:` key:
  `continue-on-error: true`. Confirmed present by grep before the run (four lines of that key in the file; three
  at the base).
- `cargo test -p pulse-app --test quality_gate_workflow`: exit 101, `24 passed; 1 failed`.
- The one red: `ci_workflow_boot_smoke_carries_no_soft_fail`.
- Read beside it: `ci_workflow_test_gates_no_continue_on_error`, the file's older soft-fail guard, stayed green
  under the mutation. Its list of gate commands names no `agent-run.sh` verb, so it never read the smoke step.
  The new pin is the only test in the file that reads this step for a soft-fail key.

## Restored

- The line was removed. Grep reads three lines of the key again, as at the base.
- The same command: exit 0, `25 passed; 0 failed`.
- `git diff 277d65d -- .github/workflows/ci.yml` shows one hunk, inside the `Boot pulse-app smoke` step.
