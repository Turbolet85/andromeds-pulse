# The repeated run — entry 6 of the gate block

`for i in 1 2 3 4 5 6 7; do cargo nextest run --workspace --profile ci --no-tests=fail -E 'binary(integration_door) |
binary(integration_engine_boot) | binary(integration_console_engine) | binary(agent_run_shell) |
binary(engine_cycle_refusals)' || exit 1; done`, fired through the gate tool at /implement, twice (once in each
firing of the whole block), on the Linux dev host.

| firing | runs green | each run's nextest summary |
|---|---|---|
| first (2026-10-10, block 1) | 7 of 7 | `48 tests run: 48 passed, 0 skipped`, 16.285 s to 16.450 s |
| second (2026-10-11, block 2, the final tree) | 7 of 7 | `48 tests run: 48 passed, 0 skipped`, 16.217 s to 16.519 s |

The counts are read from each run's own `Summary` line in the entry's log, never from a line grep of `PASS`.

Limits: fourteen runs on one host, under whatever else the host was doing; a count, not an exclusion of a collision
between tests that start at the same instant. On this host the other-account arm of `integration_door` printed its
skip sentence in every run (`other-account-control.md`).
