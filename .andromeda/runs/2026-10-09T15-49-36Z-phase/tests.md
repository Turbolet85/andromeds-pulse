# tests extract

## Relevance
partial — the chunk edits the `ci` workflow's `supply-chain` job (test-plan §9 territory) and may touch the tests that read `ci.yml`; it adds no product surface, no fixture, no E2E path.

## Constraints
- The chunk plan's `## Test Commands` must carry the standard gate set whole, unconditionally, in the listed order — `capability-drift` before the workspace nextest, the `--features mcp-server` `emit_taurpc_bindings` regen as the last cargo-adjacent step, then the bindings close against the chunk base (per test-plan §3 Per-chunk gate discipline). The scope's base sha `b3e5859` is what that close names.
- The webview gates (`npm run lint` / `typecheck` / `test`) may be left out only when the plan's Files-to-modify / New-files lists touch zero `pulse-app/ui/**` paths (per test-plan §3 Per-chunk gate discipline).
- The conditional boot-smoke gate applies only when the plan touches one of the named boot/setup paths; a plan confined to `.github/workflows/` and workflow-reading tests does not trigger it — whether the final file list stays clear of those paths is the plan's to confirm (per test-plan §3 Per-chunk gate discipline → Boot-smoke gate (conditional)).
- Every verification layer must end in a machine-readable signal (exit code, structured stdout, a log line); no human-in-the-loop checkpoint. The push-event witness and the "still red on an advisory" control must therefore be read from a run's conclusion and step log by command, never accepted by eye (per test-plan §2 Agent-runnable invariants).
- A committed test must be deterministic and use no real network except loopback stubs. A control that needs the live RustSec database or the GitHub checks API cannot live in `cargo nextest run --workspace`; it is a CI-run reading or an offline, fixture-fed check (per test-plan §2 Agent-runnable invariants).
- No retry stands in for a verdict: a red CI reading is not re-run to obtain green, and no auto-retry is added to the step (per test-plan §10 Zero-flakiness budget).
- If the repair adds or swaps a third-party action it is pinned by 40-char commit SHA; a plain `run:` step carries no such obligation (per test-plan §11 Test Anti-Patterns → CI; per test-plan §9 Cranelift-only WASM enforcement for the plain-`run:` reading).

## Patterns to follow
- Workflow-shape claims are pinned by tests that read `ci.yml`: test-plan §9 names `ci_workflow_runs_on_linux_only`, `ci_workflow_keeps_the_linux_release_build_witnesses` (`pulse-app/tests/quality_gate_workflow.rs`) and the self-lint guards `workflow_env_references_no_step_only_context` / `data_dir_export_precedes_every_consumer`. A structural property this chunk establishes (the audit step's shape, the job's permissions) follows that form. Whether any existing pin reads the audit step, its token or the `permissions:` block is research's question; `ci_workflow_keeps_the_linux_release_build_witnesses` requires the job's `cargo auditable build --workspace --release` to stay (per test-plan §9 Pipeline structure, rows Release build and Boot smoke; §9 Matrix builds).
- A gate step that separates "finding" from "could not evaluate" uses the three-exit contract as a plain `run:` step — exit 0 green · 1 findings · 2 cannot-evaluate — as the npm gate and the staged-artifacts gate do. That is the plan's existing shape for "fails on a finding, never on its own reporting", should the repair move off the action (per test-plan §9 Pipeline structure, rows Supply chain and Staged artifacts).
- The staged-artifacts step landed with "NO third-party action added, workflow `permissions:` untouched": the plan's precedent for adding a CI check without widening the token (per test-plan §9 Pipeline structure, row Staged artifacts).
- A verdict proves its own preconditions: a leg whose precondition is unmet reports INCONCLUSIVE, never PASS, and a CI wiring asserts the leg RAN, never just its exit code. The control for "the step still turns red on an advisory" takes this form — it must show the audit ran and reached a finding (per test-plan §3 Per-chunk gate discipline for the scenario legs' precondition rule; per test-plan §9 Pipeline structure, row E2E tests, for "assert the leg RAN").
- New `pulse-app` test code lives in `pulse-app/tests/*.rs`, not in lib sources; new xtask code is pinned by xtask unit tests (per test-plan §2 Test directory + naming conventions).

## Anti-patterns to avoid
- NEVER reference a third-party GitHub Action by floating tag (per test-plan §11 Test Anti-Patterns → CI).
- NEVER add a retry-once policy, and never treat a one-off red as noise — quarantine-and-fix is the only path (per test-plan §11 Test Anti-Patterns → Quality; §11 → CI).
- NEVER use real network calls in a committed test, and NEVER add a manual verification step ("someone checks the run") (per test-plan §11 Test Anti-Patterns → Universal (agent-driven specific)).

## Contract bindings
- tests §9 ↔ security-plan §Dependency Security (CI integration): the `supply-chain` job's audit/deny steps are security-owned gates that run inside the tests-owned workflow; the action name, the token and any `permissions:` change are security's to state, the step's pass/fail contract and its pins are tests'. A permissions widening is the scope's boundary question, not a tests decision.
- tests §9 ↔ architecture §Infrastructure Patterns → CI/CD approach: test-plan §9 defers the cache readings to that key; the scope's cache carry lands there at the wrap, not in test-plan.
- Gap in this plan, for the wrap: test-plan §9's Supply chain row lists setup-node, the Cranelift assertion, the npm gate and the auditable build, and test-plan.md's body names no `cargo audit` step or its action at all; §9's Build failure conditions list `cargo deny` bans and no advisory finding. The plan therefore states no test-domain contract for the step this chunk repairs — what ships may need a §9 row amendment at the wrap.

## Acceptance criteria contributions
- (tests) The standard gate set passes in order, closing with `git diff --quiet b3e5859 -- pulse-app/ui/src/bindings/index.ts` exit 0 (per test-plan §3 Per-chunk gate discipline).
- (tests) Every test that reads `ci.yml` passes inside `cargo nextest run --workspace --profile ci` after the workflow edit, `ci_workflow_runs_on_linux_only` and `ci_workflow_keeps_the_linux_release_build_witnesses` among them (per test-plan §9 Pipeline structure, row Release build; §9 Matrix builds).
- (tests) The "red on an advisory" control is recorded with evidence that the audit ran and reported a finding, and the "green when the report cannot be published" reading with the step's own log line; neither is accepted from an exit code alone or from a run where the step was skipped (per test-plan §9 Pipeline structure, row E2E tests — "assert the leg RAN"; per test-plan §2 Agent-runnable invariants).
- (tests) The push-event and pull-request-event readings are each a first run's conclusion read by command, with no re-run used to obtain green (per test-plan §10 Zero-flakiness budget).
