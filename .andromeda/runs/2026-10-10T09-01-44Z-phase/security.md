# security extract

## Relevance
partial — the chunk edits `ci.yml` steps, `xtask` gate arms and the workflow-reading test files; the security plan
binds the workflow's permission and action-pinning posture, the security gates' fail conditions and their pin, and
the harness class in the boot job. It says nothing on coverage thresholds, perf budgets, heartbeat or zero-span
checks themselves (not security's).

## Constraints
- The workflow-level `permissions:` of every workflow is required to be `contents: read`, with `write` only per job
  on a publish step (per security-plan §Secret Management → GitHub Environment scoping). No repair of the five gates
  or of the a11y upload may add a job-level `permissions:` key or a token input; the scope's own halt ("a new
  permission, token, trigger or third-party action") is this rule's boundary.
- Any `uses:` line the chunk adds or rewrites (for example an a11y upload split into one step per path member) is
  required to name a 40-char commit SHA with a version comment, never a tag (per security-plan §Security
  Anti-Patterns → Secrets). The plan's convention is worded for third-party Actions; whether every `uses:` in
  `ci.yml` at `7419496b` already carries a SHA, first-party `actions/*` included, is research's question.
- `step-security/harden-runner`, SHA-pinned, is required as the FIRST step of every job in `ci.yml` (per
  security-plan §Bootstrap phases → dep-security-ci-gate). A step the chunk inserts, moves or deletes in `lint-test`,
  `mcp-test`, `a11y`, `boot` or `coverage` must leave that ordering intact; whether each job has it first today is
  research's question.
- The `supply-chain` job's fail conditions are required to stand whole: `cargo audit` red on exit 1 and exit 2,
  `cargo deny check bans licenses sources`, `check:npm-supply-chain` red on 1 and on 2, `check:staged-artifacts` red
  on 1 and on 2 (per security-plan §Dependency Security → CI integration). None of them is among the chunk's five
  gates; the chunk's edits to shared files must not loosen any.
- The `cargo audit` step's shape is required to be pinned by `ci_workflow_audit_step_is_a_plain_run_step` in
  `pulse-app/tests/quality_gate_workflow.rs`, and `cargo audit` is required to sit in the soft-fail ban's gate list
  (per security-plan §Dependency Security → CI integration). That test file is one the chunk edits: its pin and that
  list survive the edit. The security plan names the soft-fail ban without defining it; where the ban and its list
  live is research's question.
- A Python 3 interpreter is required on every runner that runs the workspace tests, and a missing interpreter is
  required to FAIL `pulse-app/tests/unit_l4_grammar.rs`, never skip it (per security-plan §Dependency Security →
  Vendored test-only channel). That test is the vendored converter's only executed carrier and reaches CI through the
  suite steps gate (3) names; whether a `--no-tests=pass` selection can come up empty on a run, leaving it unrun and
  green, is research's question.
- The pinned channel is required to stay at or above 1.85.0 with the declared `rust-version` held equal to the pin by
  the xtask test `declared_floor_equals_the_pinned_channel` (per security-plan §Security Anti-Patterns → Universal).
  The plan is silent on a CI job running a second, unpinned channel; if producing a branch count for gate (2) needs
  one, that is outside what the plan covers and belongs in the P4 dialog as a cost, not in the plan as a default.

## Patterns to follow
- A gate that cannot read its input is its own red arm, never a pass: the npm gate's exit contract is 0 green ·
  1 findings · 2 cannot-evaluate, with registry-unreachable "NEVER a findings pass" (per security-plan §Dependency
  Security → CI integration, the npm channel bullet). The same three-way shape is the plan's precedent for a kept gate
  whose input is empty or absent.
- A plain named `run:` step with no `uses:`, no `with:`, no token, no `continue-on-error` and no `||`, its shape
  pinned by a test that reads the workflow file (per security-plan §Dependency Security → CI integration, the
  `cargo audit` bullet). A kept gate's pin can follow that form.
- Harness readers return `cannot-evaluate` on an unusable input and name what is missing, "never green" (per
  security-plan §Input Validation, the harness-only readers of the path-env-var row: `harness:ready`,
  `harness:settled`, `pre-push:linux`, `harness:boot-series`).
- Harness output is closed labels, counts, a pid and bounded records: no environment value and no path (per
  security-plan §Security Anti-Patterns → Input, the harness-only class). A repaired `ci-gates` arm that prints what
  it read prints in that vocabulary.

## Anti-patterns to avoid
- NEVER reference a GitHub Action by `@v2` or any floating tag, and NEVER grant `contents: write` at the workflow
  level (per security-plan §Security Anti-Patterns → Secrets).
- NEVER print or log raw OTLP attribute values, span / log / metric payloads or a full product-consumed filesystem
  path, basename only (per security-plan §Security Anti-Patterns → Logging). Gates (4) and (5) read the app's own log
  family in the boot job, and the job log of a public repository is an egress: a gate made to "read something" must
  not echo what it read.
- NEVER add a new harness-written kept file, a new uploaded member or a new environment read to the boot job as
  routine: each existing member of that class carries a dated classification, and the exit-witness arm's is
  PROVISIONAL, awaiting the founder's own word (per security-plan §Security Anti-Patterns → Input). The scope leaves
  the boot smoke, its series and the witness untouched; a gate (4) / (5) repair that needs a new member is a
  classification question for the operator, not a plan step.

## Contract bindings
- security ↔ tests: `pulse-app/tests/quality_gate_workflow.rs` holds security's `cargo audit` shape pin and the
  soft-fail gate list beside the pins the chunk re-points (security-plan §Dependency Security → CI integration ↔
  test-plan §3 / §10).
- security ↔ tests (CI): the `supply-chain` job and the `secret-scan` workflow are the security gate inside the one
  CI surface (security-plan §Bootstrap phases → dep-security-ci-gate and → secret-scanning-ci-gate). The witness run
  read "gate by gate" on the chunk's pre-CI commit includes them as unchanged and green, not as edited.
- security ↔ obs: obs-plan §10 owns the perf-budget, heartbeat and zero-span readings; security owns what a gate may
  print from the log family it reads (security-plan §Logging & Monitoring → What NEVER to log) and the class of any
  file it keeps (security-plan §Security Anti-Patterns → Input).
- security ↔ arch / tests: the toolchain pin and its equality test (security-plan §Security Anti-Patterns →
  Universal) bind to the coverage job's toolchain if gate (2)'s branch arm is produced rather than retired.
- wrap-time, expected amendment site only if the named members change: security-plan §Threat Model Summary →
  Infrastructure → CI/CD describes `ci.yml` by its members ("xtask test", six jobs). The scope removes no job.

## Acceptance criteria contributions
- `ci.yml` at the chunk's commit keeps workflow-level `permissions: contents: read` and gains no job-level
  `permissions:` key and no token input, read as a diff against `7419496b` (per security-plan §Secret Management →
  GitHub Environment scoping)
- every `uses:` line added or changed against `7419496b` names a 40-char SHA and no action absent from `ci.yml` at
  `7419496b` appears; `harden-runner` is still the first step of each job the chunk edits (per security-plan
  §Security Anti-Patterns → Secrets; per security-plan §Bootstrap phases → dep-security-ci-gate)
- `ci_workflow_audit_step_is_a_plain_run_step` passes, `cargo audit` is still a member of the soft-fail ban's gate
  list, and the `supply-chain` job's steps read unchanged against `7419496b` (per security-plan §Dependency Security
  → CI integration)
- `rust-toolchain.toml` is unchanged against `7419496b` and `declared_floor_equals_the_pinned_channel` passes (per
  security-plan §Security Anti-Patterns → Universal)
