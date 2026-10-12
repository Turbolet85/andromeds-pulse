# tests extract

## Relevance
relevant — the chunk edits the CI workflow that test-plan §9 specifies; it adds no product test surface, so §4–§8 contribute nothing beyond the standing gate set.

## Constraints
- test-plan §9 Pipeline structure specifies `lint-test` and the A11y suite as a Linux/macOS/Windows matrix, a macOS/Windows `release` job, and a §9 Matrix builds block over three systems. The chunk departs from that target state on P-113; the plan is not amended in phase, so these rows are expected wrap amendments, not mandates the plan holds against the chunk (per test-plan §9 Pipeline structure · §9 Matrix builds).
- test-plan §9 requires the Linux-only steps of `lint-test` to keep running on the Linux leg: `cargo xtask perf:slo-load`, the `perf-samples` nextest profile, `cargo xtask perf:budget ... --require memory,snapshot`, and the `if: always()` upload of the perf-sample logs. Dropping an always-true `runner.os == 'Linux'` condition must leave every one of these steps in place and in order (per test-plan §9 Pipeline structure, Lint + tests row).
- test-plan §9 requires the Linux workspace release build to be witnessed by the `supply-chain` job's auditable build, and names that build as Linux's release smoke; the `release` job leaving must not take that witness with it (per test-plan §9 Pipeline structure, Release build row and Supply chain row).
- test-plan §9 requires the cache keys that other jobs restore read-only to stay resolvable: `lint-test-${{ runner.os }}` (restored by `mcp-test` and `a11y`), `boot-Linux` (restored by `supply-chain`), and `coverage`. A rewrite of a key expression must resolve to the same string on Linux (per test-plan §9 Pipeline structure, Cache column).
- test-plan §9 requires every job to export `ANDROMEDA_PULSE_DATA_DIR` to `$GITHUB_ENV` from a step directly after harden-runner, pinned by the self-lint guards `workflow_env_references_no_step_only_context` and `data_dir_export_precedes_every_consumer`. The chunk edits that step (its `shell: bash` line) in seven jobs; whether those guards, or any other test, read the step's shape, a job name, or a matrix value is research's question (per test-plan §9 Pipeline structure, Boot smoke row).
- test-plan §3 Per-chunk gate discipline requires the standard gate set in the plan's `## Test Commands` unconditionally, whatever the chunk's scope; the webview gates are excluded only when no `pulse-app/ui/**` path is touched, and the close for a chunk adding no procedure is the bindings diff named against the chunk base, never HEAD (per test-plan §3 Per-chunk gate discipline).
- test-plan §10 tolerates no flaky test and no retry policy. The before/after wall-clock readings and the boot-smoke verdicts the chunk records must come from first-attempt runs; a re-run to obtain green is not a reading (per test-plan §10 Zero-flakiness budget).

## Patterns to follow
- Order the local gates as the plan fixes them: `capability-drift` before the default-features workspace nextest, the `--features mcp-server` `emit_taurpc_bindings` regen as the last cargo-adjacent step, then the bindings diff against the chunk base (per test-plan §3 Per-chunk gate discipline).
- The workspace tests need a Python 3 interpreter on the runner with no setup step; a missing interpreter fails the test, never skips it. The Linux leg keeps that property (per test-plan §9 Pipeline structure, Lint + tests row).
- The boot smoke stays as the plan describes it: release build with `mcp-server`, `boot` → `status` → `cleanup` inside one `xvfb-run`, `cargo xtask ci-gates` over the log the smoke wrote, and the `if: always()` log upload; the job is gating, with no `continue-on-error` (per test-plan §9 Pipeline structure, Boot smoke row).
- A gate that cannot evaluate reports that verdict by exit code 2, never green; none of the xtask gate steps the Linux legs keep may be weakened into an unconditional pass while their conditions are edited (per test-plan §9 Pipeline structure, Supply chain row and Staged artifacts row).
- The conditional boot-smoke gate of the chunk plan does not attach: its trigger list names boot/setup paths and no workflow file. Whether the plan's file list touches one of those paths is research's question (per test-plan §3 Per-chunk gate discipline, Boot-smoke gate (conditional)).

## Anti-patterns to avoid
- NEVER reference a third-party GitHub Action by floating tag; any action line the edit rewrites keeps its 40-char SHA pin (per test-plan §11 CI).
- NEVER lower a coverage threshold, add a retry, or ignore a flake to pass the build; the `coverage` job's thresholds and the zero-flake rule are untouched by a runner removal (per test-plan §11 CI · §11 Quality).
- NEVER include a manual smoke step; the after-reading is taken from the run itself, by the same machine read as the before-reading (per test-plan §11 Universal (agent-driven specific)).

## Contract bindings
- tests ↔ obs: `cargo xtask ci-gates` reads the log the boot smoke wrote, and `cargo xtask perf:budget` grades the obs-plan §10 budgets over the perf-sample log family; both stay on Linux jobs and bind to the obs log format (per test-plan §9 Pipeline structure, Boot smoke row and Lint + tests row · test-plan §10 Performance budgets).
- tests ↔ a11y: the A11y suite runs in the same workflow; its gate conditions (axe critical/serious = 0, Lighthouse ≥ 90, pa11y 0 errors, zero new baseline tuples) and its PR-only base-summary download must hold unchanged on the remaining Linux leg (per test-plan §9 Pipeline structure, A11y suite row).
- tests ↔ security: the `supply-chain` job shares the workflow file; its gate order (Cranelift assertion → npm gate → install → auditable build) is specified here and is outside this chunk's edit (per test-plan §9 Pipeline structure, Supply chain row · §9 Cranelift-only WASM enforcement).
- tests ↔ harness: `cargo xtask pre-push:linux` is specified as reading ci.yml's Node major and apt list; whether it also reads a matrix value or a job this chunk removes is research's question (per test-plan §3 Per-chunk gate discipline).
- tests ↔ host: the plan's integration tier uses the loopback receivers on `:4317`/`:4318`; whether the workspace run binds those fixed ports on the dev host, which the scope gates on the operator's word, is research's question (per test-plan §2 Agent-runnable invariants).
- Expected wrap amendments in this plan, for the plan's writer: §9 Pipeline structure (Lint + tests, Release build, A11y suite, E2E rows) · §9 Matrix builds · §4 coverage-tool line ("all 3 CI matrix runners") · §1 `multi-platform-compat` trigger row · §3 Per-chunk gate discipline, Process-end witness form ("CI lint-test Linux/macOS").

## Acceptance criteria contributions
- The standard gate set passes locally, in the fixed order, closing with `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` at exit 0 (per test-plan §3 Per-chunk gate discipline).
- `cargo nextest run --workspace --profile ci` passes with the workflow self-lint guards `workflow_env_references_no_step_only_context` and `data_dir_export_precedes_every_consumer` green against the edited `ci.yml` (per test-plan §9 Pipeline structure, Boot smoke row).
- The chunk's CI run is green on every remaining job with no test failure, no coverage threshold below 75% line / 70% branch / 85% function, and no step retried; the Linux `lint-test` leg shows the perf steps and `perf:budget --require memory,snapshot` as run, not skipped (per test-plan §9 Build failure conditions · §10 Coverage thresholds).
- The `supply-chain` job's auditable workspace release build reads `success` in that run, standing as the Linux release-build witness after the `release` job is gone (per test-plan §9 Pipeline structure, Supply chain row).
