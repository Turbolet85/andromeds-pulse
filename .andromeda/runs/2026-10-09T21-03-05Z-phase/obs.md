# obs extract

## Relevance
partial — obs-plan.md names neither the pre-push check nor a second-system hop (no `pre-push` and no `wsl` string anywhere in the plan), so it sets no contract for the verb or its verdict; it binds the chunk only through two surviving stages that produce or grade the product's own log (`test`, `ci-gates`), through where that log lands, and through its CI and SLO bans.

## Constraints
- The gates the `ci-gates` stage runs are the ones obs-plan §10 CI gates defines (no `app.panic.fatal` record, no heartbeat gap over 45 s per tick target, the perf-budget grader), and §10 CI gates requires each to fail the build; a native stage keeps all of them. Which gates the stage runs today over its seeded data dir, and what the seed holds, is research's question.
- obs-plan §10 CI gates requires the perf-budget grader to be never PASS over empty input, and describes `ci-gates` as grading with no arm required and printing `perf-budget NEUTRAL`; the native stage must keep that reading and must not turn an unrequired empty arm into a pass or into a failure. Whether the pre-push stage reads the same line as the CI boot job is research's question — the plan states it for the boot-smoke log only.
- obs-plan §10 (the always-required invariant, zero unlogged panics) applies to every log the native stages produce or grade: a panic record in the graded log family is red, with no retry.
- obs-plan §3 Log file location requires the product's log at `<data_dir>/logs/agent-latest.jsonl`, with `<data_dir>` taken from `ANDROMEDA_PULSE_DATA_DIR` when set and from the per-user default otherwise. With the clone and its emptied environment gone, every stage that starts product code or reads a log family must be given its own data dir explicitly; a stage left on the default would write into, or grade, the developer's own log. What data dir each stage gets today is research's question.
- obs-plan §9 Pipeline integration requires `xtask test` to produce JSON logs with spans serialized as `tracing` events, and obs-plan §10 CI gates requires a failure when `xtask test` leaves zero spans in the log file. Whether the `test` stage's native run writes that log, where, and whether anything checks it is research's question.
- obs-plan §6 Log Coverage (the `warn` row) requires `corpus.keychain.fallback` once per boot when the passphrase branch serves the corpus key instead of the OS credential store. If a native stage is given the session bus, the credential-store legs stop clean-skipping and this record's two arms become reachable inside the check; that reach is the scope's own boundary question, and the plan here only fixes what the record may carry.
- obs-plan §10 Performance budgets holds no budget for the check's own wall time. A timing the chunk records is an observation with its load stated, not a graded arm; the measured values the plan itself records name their build profile and sample count beside the number (§10 Performance budgets, the snapshot row).

## Patterns to follow
- A verdict vocabulary that tells "could not read" apart from pass and from fail: the perf-budget grader's PASS / FAIL / NEUTRAL / `cannot-evaluate` readings, with a named cause on an empty arm (obs-plan §10 Performance budgets, the frame row). The check's own `cannot-evaluate` for a missing tool is the same shape.
- Grade a log FAMILY, not one file: the grader reads every `agent-latest.jsonl*` member, because the sink rotates daily (obs-plan §10 CI gates; obs-plan §3 Log file location, Rotation).
- A dev-host leg is a recorded reading, not a CI gate, and the plan says so in the row that carries it (obs-plan §10 Performance budgets, the frame row: `perf:frame-sample` runs on a dev host and is "not a CI gate").
- Each measured value is written with the run or chunk that produced it and its sample size (obs-plan §10 Performance budgets; obs-plan §9 Telemetry artifact handling).

## Anti-patterns to avoid
- NEVER rely on colored terminal output for parsing, and never gate on a human reading a log: the check's result stays structured and machine-readable (obs-plan §11 Obs Anti-Patterns → CI).
- NEVER define a soft budget with no enforcement, and never add a retry that masks a panic: a timing of the check is not written as a budget, and a red gate inside `ci-gates` is not re-run to green (obs-plan §11 Obs Anti-Patterns → SLO).
- NEVER log a full path, a raw attribute value or a query parameter in the product's log (obs-plan §11 Obs Anti-Patterns → Logs). This governs the product records the native stages produce; the plan sets no rule for what the xtask verdict prints, so whether the native verdict or `report.json` would carry a host path is research's question, answered against the security plan, not this one.

## Contract bindings
- obs ↔ tests: the `ci-gates` stage consumes the log format and the tick records the obs plan defines (obs-plan §3 Log format JSON schema; obs-plan §3 Heartbeat ticks; obs-plan §10 CI gates). The harness that owns the verb, its seeded data dir and its five-command discipline is test-plan §3.
- obs ↔ security: the per-stage data dir is the same variable the path rule governs (obs-plan §3 Log file location ↔ security-plan §Security Anti-Patterns → Input); `corpus.keychain.fallback` may carry only `backend_kind`, `reason`, `consequence` as bounded static strings, never the key, the passphrase or a presence flag (obs-plan §8 PII Scrubbing ↔ security-plan §Logging & Monitoring). Reaching the session bus at all is a boundary question for the founder, per the scope.
- obs ↔ arch: obs-plan §9 CI Integration describes GitHub Actions jobs only and defers the CI/CD approach to architecture §Infrastructure Patterns. The obs plan carries no description of a WSL-hosted check, so the wrap owes it no correction on that account; a row for the native check in §9 Pipeline integration would be a new entry, the wrap's to decide.

## Acceptance criteria contributions
- (obs) The native `ci-gates` stage still fails on an `app.panic.fatal` record and on a heartbeat gap over 45 s in the log family it grades, shown by a red reading over a log seeded with each (per obs-plan §10 CI gates)
- (obs) Over its seeded data dir the native `ci-gates` stage prints the perf-budget reading as NEUTRAL with no arm required, never PASS over empty input (per obs-plan §10 CI gates)
- (obs) No native stage writes to or reads `logs/agent-latest.jsonl*` under the developer's default data dir: every stage that starts product code or grades a log is given an explicit `ANDROMEDA_PULSE_DATA_DIR` under `target/` (per obs-plan §3 Log file location)
- (obs) The check's result is one structured, machine-readable verdict an agent can read without a terminal, and any wall time recorded beside it states the load it was taken under and is not written as a budget (per obs-plan §11 Obs Anti-Patterns → CI)
