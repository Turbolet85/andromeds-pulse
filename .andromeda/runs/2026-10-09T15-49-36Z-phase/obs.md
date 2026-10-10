# obs extract

## Relevance
partial — the chunk repairs one step of the `supply-chain` job in `ci.yml` and adds no product code path; obs-plan carries no mandate for that job or for the audit step, and binds only through §9 (the log artifacts other jobs of the same file upload, and the artifact the folded boot-smoke watch is read from) and §11 CI (how a CI verdict is read).

## Constraints
- obs-plan §9 Telemetry artifact handling requires the self-observation log artifacts to be uploaded from the `lint-test` job and the `boot` job, each under `if: always()`, under names distinct within a run. The chunk edits the file that holds those steps; they stay as they stand at the chunk base (scope keeps both jobs untouched).
- obs-plan §9 Pipeline integration lists the stages that produce telemetry and their consumers; it has no row for a supply-chain stage. Obs therefore asks the repaired step for no log artifact, no upload step and no default field. A repair that adds none owes obs nothing.
- obs-plan §9 CI failure → artifact triage workflow requires a CI failure to be diagnosable by an agent from a downloaded, machine-parseable artifact. For the folded boot-smoke watch this is the route a red reading is diagnosed by: the boot smoke's application log is the `boot` job's log artifact, not the job's console output.
- obs-plan §9 Artifact retention sets a 14-day window on those artifacts. A boot-smoke reading the chunk wants to diagnose from the application log has to be read inside that window.
- obs-plan §11 CI requires CI results an agent parses to be structured and machine-readable, and bans analysis gated on a human review surface. This applies to how the chunk's two witnesses are read (the run that stays green when the report cannot be published, and the control that turns red on an advisory): each is read from the run's conclusion and the job log, not from a UI-only surface.
- obs-plan §1 Obs Scope Summary scopes instrumentation to the product's modules and surfaces. A workflow-only repair touches none of them, so §4, §5 and §6 attach to nothing here. Whether the chosen repair touches any Rust source at all is the plan's question; obs-plan names no instrumentation for an xtask or workflow step.

## Patterns to follow
- Distinct artifact names per job within one run (per obs-plan §9 Telemetry artifact handling). It matters only if the repair publishes the audit report as an artifact instead of a check run; the new name must then collide with none already in the run.
- A lint stage reports through the job's own output stream, consumed as workflow annotations (per obs-plan §9 Pipeline integration, the `fmt` / `clippy` row). This is the plan's one stated precedent for a step whose report travels in the job log rather than through a separate publication. Whether that channel suits the audit step, and what it needs from the token, is the plan's and the security extract's to settle, not obs's.
- Download the run's log artifact and filter it by structured fields when a job's failure has to be explained (per obs-plan §9 CI failure → artifact triage workflow). This is the reading path for the boot-smoke watch on every CI run the chunk reads.

## Anti-patterns to avoid
- Never lose a telemetry artifact: do not drop, rename or condition the `lint-test` and `boot` log uploads while editing `ci.yml` (per obs-plan §11 CI).
- Never let a CI verdict rest on colored terminal output or on a surface only a human reviews: the finding and the report-publication outcome of the audit step must be readable by an agent from the job log (per obs-plan §11 CI).

## Contract bindings
- obs ↔ tests: the `boot` job's log artifact is the log the `ci-gates` reading in that job consumes (obs-plan §9 Telemetry artifact handling). The chunk does not touch that job; the folded watch reads its verdict. Whether the two test files and the xtask module that read `ci.yml` (named in scope) pin any of the obs upload steps is research's question.
- obs ↔ security: the token's permissions and the audit step's action are the security plan's subject. Obs-plan states nothing about either and adds no constraint on which repair ships.
- obs ↔ architecture: obs-plan §9 takes the CI platform from the architecture's CI/CD decision; the scope's wrap amendment to that key file is architecture's record, not obs's.
- No §3 keyed contract (tracing init, service identity, logging stack, log format, log file location, snapshot integration, trace context propagation, heartbeat ticks) is turned by this chunk; none was read.

## Acceptance criteria contributions
- A diff of `.github/workflows/ci.yml` against the chunk base `b3e5859` shows no change to the log-artifact upload steps of the `lint-test` and `boot` jobs: their names, their paths and their `if: always()` condition (per obs-plan §9 Telemetry artifact handling)
- Each witness the chunk records for the audit step names a run id and the job-log text an agent read to reach the verdict, for the green run and for the red control alike; none rests on a check-run page or another UI-only reading (per obs-plan §11 CI)
- If the repair adds an artifact upload to the `supply-chain` job, its artifact name differs from every other artifact name the workflow produces in one run; if it adds none, this check is recorded as not applicable (per obs-plan §9 Telemetry artifact handling)
