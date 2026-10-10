# Mutation checks — do the pins discriminate

One-shot checks at /implement, on this host, 2026-10-09 between 16:36Z and 16:38Z. Each mutation edited
`.github/workflows/ci.yml` alone, was confirmed present by a fixed-string count before the run, and was reverted
before the next. The run is the same for all:

```
cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow)'
```

The binary holds 21 tests at this point (the 20 of `red-before.md` plus the build-only trigger pin).

## The two the plan asks for

| # | Mutation | Exit | Summary | Tests that failed |
|---|---|---|---|---|
| 1 | `continue-on-error: true` added to the audit step | 100 | 19 passed, 2 failed | `ci_workflow_audit_step_is_a_plain_run_step` · `ci_workflow_test_gates_no_continue_on_error` |
| 2 | `  workflow_dispatch: {}` added to the trigger block | 100 | 20 passed, 1 failed | `ci_workflow_triggers_are_pull_request_and_push_on_main` |

- Mutation 1, the two panics: `the audit step MUST hold no `continue-on-error`: no action, no token and no soft-fail`
  (`quality_gate_workflow.rs:324:9`) and `ci.yml step block contains gate command AND `continue-on-error: true``
  (`:293:9`). The second shows the new `"cargo audit"` gate substring is what brings the audit step under the
  soft-fail ban: that test passed in `red-before.md`'s red reading, when the step carried no such line.
- Mutation 2, the panic: `assertion `left == right` failed: ci.yml MUST run on a pull request and on a push to main,
  and on nothing else` (`:163:5`). No other test failed, the audit pin included.

## Three more, one per remaining assertion of the audit pin

`red-before.md`'s red reading stops at the pin's first assertion. These read the others; they are beyond the plan's
two and cost one run each.

| # | Mutation | Exit | Summary | Failing assertion of `ci_workflow_audit_step_is_a_plain_run_step` |
|---|---|---|---|---|
| 3 | the run line made `run: cargo audit \|\| true` | 100 | 20 passed, 1 failed | `:317:5`, the step MUST be the plain step `run: cargo audit` |
| 4 | `permissions:` / `checks: write` added to the `supply-chain` job | 100 | 20 passed, 1 failed | `:330:5`, the job MUST carry no `permissions:` key of its own |
| 5 | the action re-added as a separate step after the audit step | 100 | 20 passed, 1 failed | `:337:5`, ci.yml MUST NOT use `rustsec/audit-check` |

- Mutation 3 fails on the exact-line assertion before the `||` ban is reached, so the `||` entry of the banned list
  is not read alone here; it would be reached by a multi-line `run:` block that keeps a `run: cargo audit` line.
- In mutations 3, 4 and 5 `ci_workflow_test_gates_no_continue_on_error` stayed green, as it should: none adds
  `continue-on-error`.
- Mutation 4 is the rejected widening itself (plan, Constraints). It is not caught by
  `ci_workflow_preserves_workflow_level_contents_read_permission`, which reads the workflow's first 40 lines only.

## After the last revert

- `git diff b3e5859 -- .github/workflows/ci.yml` equals the first 15 lines of `main-hotfix.patch` byte for byte (`cmp`,
  exit 0).
- `git apply --check --reverse evidence/main-hotfix.patch`: exit 0.
- The same nextest command: exit 0, `21 tests run: 21 passed, 0 skipped`.
- A count after each revert read the mutation gone: `continue-on-error: true` 3 lines (3 at `b3e5859`),
  `workflow_dispatch` 0, `|| true` 0 (0 at `b3e5859`), `checks: write` 0.
