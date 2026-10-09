# obs extract

## Relevance
partial — the chunk adds no span, metric, log or error path and obs-plan names the npm supply-chain gate nowhere; obs binds only through the webview dependency tree the lockfile change moves, and through the obs CI gates that re-run on the chunk's commits.

## Constraints
- No new instrumentation is owed: the chunk adds no module, surface or traceable operation, so the Standard-tier depth asks nothing of it (per obs-plan §1 Obs Scope Summary; §2 Telemetry Strategy). The plan carries no span, metric or log row for `check:npm-supply-chain`, and the chunk is not the place to add one.
- The webview dependency tree must stay free of a browser OTel SDK and of `web-vitals`: obs-plan §1 (frontend module row + desktop-webview surface row) and §3 Logging stack → Frontend bridge require that all frontend telemetry goes through the hand-rolled TauRPC `telemetry.frontend.*` bridge and record `web-vitals` as retired with 0 hits in `package.json` / `package-lock.json`. An upgrade or an `overrides` entry must not bring either in, directly or transitively. Whether the lockfile at HEAD still reads 0 hits is research's question.
- A runtime webview dependency moves (`seroval` under `@tanstack/react-router`, prod-reachable per the scope): obs-plan §4 Span / Trace Coverage (instrumentation table, Desktop-webview row) requires the frontend emit sites behind `telemetry.frontend.record_*` to keep working unchanged. The chunk adds and removes none of them.
- No obs CI gate witnesses webview-originated records: obs-plan §10 (WebGPU frame row) and §1 (the `ui.webgpu.adapter` note in the multi-platform row) state that the CI boot job stops the app before the webview issues any IPC, and that the frame gate is a dev-host leg, not CI. So a green CI run on the changed lockfile says nothing about the frontend bridge; that reading comes from the UI unit tests and build the scope already re-runs.
- The obs CI gates stay hard on the chunk's commits: zero `app.panic.fatal`, heartbeat gap ≤ 45 s, and `perf:budget --require memory,snapshot` must read green, with no softened budget and no retry (per obs-plan §10 CI gates; §11 SLO).
- If the Node-major comparison leads to a `ci.yml` edit, the log-artifact uploads stay as specified: per-OS `lint-test` logs, the Linux `boot` log and the perf-samples log, each `if: always()` under distinct names (per obs-plan §9 CI Integration → Telemetry artifact handling).

## Patterns to follow
- Dependency-absence claims are made by measurement on the committed manifests: the plan's own form is a hit count over `package.json` / `package-lock.json` / `ui/src` (per obs-plan §3 Logging stack → Frontend bridge). Use the same probe after the change.
- CI verdicts are read from structured, machine-readable output: the gate's exit code and printed arm, never terminal colour (per obs-plan §9 CI Integration; §11 CI).
- obs-plan §9's artifact table names log artifacts for the `lint-test` and `boot` jobs only and none for the supply-chain job, so the plan gives no artifact route to that job's verdict; it is read from the step's own output. Whether `ci.yml` uploads anything from that job is research's question.

## Anti-patterns to avoid
- NEVER link an OTel SDK or a vendor APM package into the self-observation runtime, the webview included: an advisory is not closed by swapping in a dependency that carries one (per obs-plan §11 Universal).
- NEVER rely on coloured terminal output for CI parsing when confirming the gate reads green (per obs-plan §11 CI).

## Contract bindings
- obs ↔ tests: the frontend telemetry bridge has no CI obs witness (obs-plan §10 frame row), so the UI unit tests and build on the changed lockfile are the only pre-merge reading of it; the tests extract owns those commands.
- obs ↔ security: the gate, `npm-policy.json` and the exception form are security's (security-plan §Dependency Security). Obs adds one condition to any chosen disposition: no telemetry SDK enters the tree.
- obs ↔ tests harness (obs-plan §3 Heartbeat ticks, §10 CI gates): unchanged by this chunk; the boot-smoke and heartbeat checks run as they stand on its commits.

## Acceptance criteria contributions
- After the change, `web-vitals` and any browser OTel SDK package read 0 entries in `pulse-app/ui/package.json` and `pulse-app/ui/package-lock.json` (per obs-plan §3 Logging stack → Frontend bridge; §11 Universal)
- The chunk's diff adds or removes no telemetry emit site: no Rust `tracing` target and no webview `telemetry.frontend.*` call (per obs-plan §1 Obs Scope Summary; §4 Span / Trace Coverage instrumentation table)
- On the chunk's CI run the obs gates read green: zero `app.panic.fatal`, no heartbeat gap over 45 s, `perf:budget --require memory,snapshot` PASS (per obs-plan §10 CI gates)
- If `ci.yml` is edited, the three log-artifact uploads are still present with `if: always()` and distinct names (per obs-plan §9 CI Integration → Telemetry artifact handling)
