# arch extract

## Relevance
partial — the chunk edits one step of one CI job; arch owns the workflow's shape (§Infrastructure Patterns → CI/CD approach) and the xtask gate registry (§Occupied Resources), not the token-permission rule, which is security-plan's.

## Constraints
- architecture §Infrastructure Patterns → CI/CD approach requires `ci.yml` to run on every pull request and on every push to `main`. A repair that reaches a push-event reading by adding or changing a trigger (another branch, a manual dispatch) changes this keyed statement and is carried as a wrap amendment of that key file; phase amends nothing.
- architecture §Infrastructure Patterns → CI/CD approach requires six independent jobs on `ubuntu-22.04`, with no matrix and no `needs:` edge. A repair that moves the audit step, or its reporting, into a job of its own (for example to scope a permission to it) changes the job count and the no-`needs:` statement; the plan names that as a key amendment or keeps the step inside `supply-chain`.
- architecture §Infrastructure Patterns → CI/CD approach requires `supply-chain` to restore the `boot-Linux` cache read-only and to write no cache key of its own, under a budget its last recorded reading puts over the repository cap. The repaired step adds no cache write.
- architecture §Infrastructure Patterns → CI/CD approach requires `supply-chain`'s `cargo auditable build --workspace --release` to stay one of the two release-build witnesses, and the workflow to name no system other than Linux; both are stated there as pinned by self-lint tests. The repair leaves that step and the runner label as they are.
- architecture §Infrastructure Patterns → CI/CD approach requires shared CI logic that needs Rust to live in the `xtask` crate, runnable locally as `cargo xtask <task>`. architecture §Established Decisions [CI Task Runner] fixes `cargo-xtask` as the one task runner; a repair that replaces the third-party action with first-party logic puts it there, not in a shell script, a `just` recipe or a new crate.
- architecture §Occupied Resources → xtask CLI surfaces requires any new or re-contracted `cargo xtask` verb wired into CI to be registered there as a formalized CLI contract: named verdict arms, the exit shape of its siblings (0 green · 1 findings · 2 cannot-evaluate, the last never a pass), a machine-readable verdict. It names `Cmd::Audit` and `Cmd::DenyBans` as existing supply-chain verbs; whether `Cmd::Audit` exists in the code at the chunk base, what it runs and what it exits with is research's question.
- architecture §Occupied Resources → Cargo workspace crate names and → Environment variables reserve the sixteen member names and the registered variable set. The chunk adds no workspace member and no `ANDROMEDA_PULSE_*` variable; a new one would be an Occupied Resources entry at the wrap.

## Patterns to follow
- Plain named `run:` step, no third-party action, workflow `permissions:` untouched: the shape architecture §Occupied Resources → xtask CLI surfaces records for the staged-artifacts gate and for the npm gate inside the same `supply-chain` job. It is the widening-free alternative the scope asks the plan to state beside any permission change; whether it fits the audit step is the plan's to decide.
- Three-arm exit contract with "cannot evaluate" kept apart from "finding" (architecture §Occupied Resources → xtask CLI surfaces, the npm gate and the staged-artifacts gate). P-120's "never on its own reporting" covers the publishing of a report; it does not cover an advisory database that could not be read, which those sibling contracts never count as a pass. The plan keeps the two apart.
- Workflow claims are pinned by self-lint tests that read `ci.yml` (architecture §Infrastructure Patterns → CI/CD approach names `ci_workflow_runs_on_linux_only` and `ci_workflow_keeps_the_linux_release_build_witnesses`). A new workflow invariant the repair introduces follows that shape; whether an existing test pins the audit step's action, its token or the permissions block is research's question.
- Machine-parseable, deterministic gate output for agent-driven work (architecture §Cross-cutting Patterns → Development Style).

## Anti-patterns to avoid
- Folding the repair into a new job, a `needs:` edge or a matrix without naming the key amendment (architecture §Infrastructure Patterns → CI/CD approach).
- CI-only Rust logic outside `xtask`, or a second task runner, for the audit gate (architecture §Established Decisions [CI Task Runner]).
- An audit verb whose "could not evaluate" arm exits 0 so that the job "ends the same" on both events (architecture §Occupied Resources → xtask CLI surfaces).

## Contract bindings
- arch ↔ security: the workflow-level `permissions:` rule, the SHA-pinning of a third-party action and the `cargo audit` pass/fail posture are security-plan's (§Dependency Security, CI integration; its supply-chain rules). Arch records only the job's shape; a permission widening is decided there and by the founder, not here.
- arch ↔ tests: the self-lint tests over `ci.yml` named in architecture §Infrastructure Patterns → CI/CD approach are test-plan's gate inventory; a changed step name or action may touch them.
- arch ↔ `xtask/src/pre_push.rs`: architecture §Occupied Resources → xtask CLI surfaces records that `pre-push:linux` reads pins from `ci.yml` (the Node major, the first `apt-get install` list). The repair leaves both readable; whether it reads anything of the `supply-chain` job is research's question.
- arch ↔ wrap: two expected amendments of the CI/CD approach key file, neither implement's work: the cache-listing re-read the scope carries, and the description of the audit step or trigger set if the repair changes either. If a verb is added or re-contracted, a third in §Occupied Resources → xtask CLI surfaces.
- arch ↔ other workflows: architecture §Infrastructure Patterns → CI/CD approach describes `release.yml` and `update-channels.yml` without naming an audit step in either; whether the same step or token shape stands there is research's question.

## Acceptance criteria contributions
- `ci.yml` at the chunk's end still holds six jobs, each on `ubuntu-22.04`, with no `matrix:` and no `needs:`, or the plan names the CI/CD approach key amendment that changes the count (per architecture §Infrastructure Patterns → CI/CD approach)
- The `supply-chain` job writes no cache key, and its `cargo auditable build --workspace --release` step is unchanged against the chunk base `b3e5859` (per architecture §Infrastructure Patterns → CI/CD approach)
- Any Rust the repair adds lives in the `xtask` crate, with no new workspace member and no new environment variable (per architecture §Occupied Resources → Cargo workspace crate names)
- If the audit gate becomes or changes a `cargo xtask` verb, its exit contract separates finding (1) from cannot-evaluate (2), neither exits 0, and the verb is listed for registration at the wrap (per architecture §Occupied Resources → xtask CLI surfaces)
