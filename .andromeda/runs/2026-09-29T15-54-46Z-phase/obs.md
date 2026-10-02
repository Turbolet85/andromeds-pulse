# obs extract

## Relevance
partial — no new product instrumentation (CI/dev-loop infrastructure, obs tier Standard per obs-plan §1), but the job split (A), rebuild de-duplication (C) and the WSL pre-push check (D) move the steps that produce and gate on the self-observation log, so obs-plan §9 CI Integration and §10 CI gates bind.

## Constraints
- obs-plan §9 CI Integration → Telemetry artifact handling requires the log file (`logs/agent-latest.jsonl`) to be uploaded as a CI artifact from every CI job that produces one, plus the curated snapshot on integration-test failure. After the split, each new parallel job that boots the app or runs log-producing tests owns its own upload. Whether ci.yml uploads per job today, and under what artifact names, is research's question.
- obs-plan §10 CI gates requires three log-derived gates to fail the build: zero `app.panic.fatal`, heartbeat tick gap ≤ 45 000 ms per `{ingest,buffer,viz,plugins}.tick`, and the perf-budget p99 assertion. Moving the boot leg or the `ci-gates` step into its own job must keep each gate reading the log of the run it gates, in the same job or through an artifact hand-off. The scope's Boundaries rule (no gate weakened or removed) is the obs half of this.
- obs-plan §10 Performance budgets (WebGPU frame row) requires check scripts to stay NEUTRAL-tolerant: a missing metric stream is not a failure, and `write_run_window_log` scopes the scripts to the run's time window. A split that hands a gate a log from a different job, or a merged log from several jobs, must keep that run-window scoping so a cross-job heartbeat gap cannot read as a false stall FAIL.
- obs-plan §10 CI gates (perf-budget bullet, "measured VACUOUS in CI") names its owner as the working-route entry "Perf-budget gate reads real samples". Per the scope's Boundaries, this chunk may move the perf step but must not claim to fix or change its vacuousness.
- obs-plan §3 Heartbeat ticks + §10 Standard+ invariants set the liveness/progress pair: tick presence is liveness, and `check:ingest-progress` / `buffer.consumer.stalled` is progress. A local pre-push boot leg (D) that reuses the harness boot gets the same two-signal reading, never tick presence alone. Whether D includes a boot leg at all is a P4 decision under the host constraint (no `4317`/`4318` bind).
- obs-plan §9 CI-specific default fields sets `deployment.environment` / `ci.run.id` / `git.commit.sha` at subscriber init from GitHub env vars. Splitting jobs must not strip the `GITHUB_*` env those fields read. Whether the subscriber wires these fields today is research's question, and a local WSL run has no `GITHUB_RUN_ID`, so the pre-push check must tolerate their absence rather than invent values.

## Patterns to follow
- obs-plan §9 CI failure → artifact triage workflow: `gh run download <run_id> -n logs` + `jq` over JSON-per-line. Artifact naming across the split jobs should keep that one-command retrieval workable, with names that tell the jobs apart.
- obs-plan §3 Log file location: `<data_dir>/logs/agent-latest.jsonl` resolved via `resolve_data_dir()` (`ANDROMEDA_PULSE_DATA_DIR` override). Each parallel job and the WSL check should point the data dir at a job-local path so concurrent legs never share a log file.
- obs-plan §10 CI gates heartbeat-gap enforcement lives in a named script (`xtask/ci/heartbeat-gap-check.sh`). New local or CI gate logic belongs on the existing harness/xtask verbs, not in ad-hoc inline shell. The scope already routes D the same way.
- obs-plan §3 Heartbeat ticks + the scope's watch item: `agent-run.sh boot`'s failure diagnosis, which names the signal or exit status, is the instrument for the Linux silent-death watch (1/3). Keep that diagnosis output intact and reachable in whichever job hosts the boot leg.

## Anti-patterns to avoid
- obs-plan §11 CI: NEVER lose telemetry artifacts. A split job that drops the `agent-latest.jsonl` upload, or loses the snapshot on failure, violates this.
- obs-plan §11 CI + §11 Logs: NEVER rely on colored terminal output for CI parsing, and NEVER use unstructured text as the agent surface. The pre-push check's verdict must be machine-readable: a structured result plus exit code, not colour or prose alone.
- obs-plan §11 SLO: NEVER add retry-once policies for panic handling. A parallel-job or pre-push design must not add blind step retries that could mask an `app.panic.fatal` or a boot death.

## Contract bindings
- obs ↔ tests: the §3 Heartbeat ticks, log-format schema and boot-harness liveness bind to test-plan §3 (5-command harness `boot/run/status/cleanup/logs`) and the registered scenario smokes. The CARRY (E) registering `smoke:gap-resume` / `smoke:external-resolve` touches the same harness surface, and those smokes consume the §3 log contract.
- obs ↔ security: every new CI job inherits `harden-runner` first, SHA-pinned actions and `contents: read` (security.md §Supply chain + CI). An artifact upload step added for obs-plan §9 is a third-party action and falls under the same SHA-pin rule.

## Acceptance criteria contributions
- (obs) Every post-split CI job that boots the app or runs log-producing tests uploads its `agent-latest.jsonl` as a distinctly named artifact, including on failure (per obs-plan §9 Telemetry artifact handling).
- (obs) The zero-panic, heartbeat-gap and perf-SLO gates still run on every round after the split. Each reads the log of the run it gates, within its run window, and a deliberately injected `app.panic.fatal` line still reddens the round (per obs-plan §10 CI gates).
- (obs) The perf-budget gate's NEUTRAL/vacuous status is unchanged and still attributed to its owner entry: this chunk neither fixes it nor claims a fix (per obs-plan §10 CI gates, perf-budget bullet).
- (obs) The WSL pre-push check emits a machine-readable verdict plus exit code and, if it runs a boot leg, keeps `agent-run` boot's named-signal/exit-status diagnosis (per obs-plan §11 CI and §3 Heartbeat ticks).
