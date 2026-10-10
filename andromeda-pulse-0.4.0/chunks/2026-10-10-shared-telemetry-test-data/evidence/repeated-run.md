# The repeated run — 2026-10-10-shared-telemetry-test-data (plan step 8)

Gate entry 8 of the plan's `## Test Commands`, fired once by the gate tool in /implement's run of the whole block
(run dir `.andromeda/runs/2026-10-10T21-06-49Z-implement/`, gate v1.14). The four port-using test binaries
(`integration_engine_boot`, `integration_console_engine`, `agent_run_shell`, `engine_cycle_refusals`) in one
nextest invocation, seven invocations in a row.

The entry's line: `8 integration green · exit 0 · 118.98s · 69020 B → 8.log`.

**Count: 7 green runs of 7.** The seven `Summary` lines of the entry's own log, in order:

```
Summary [  16.203s] 38 tests run: 38 passed, 0 skipped
Summary [  16.103s] 38 tests run: 38 passed, 0 skipped
Summary [  16.142s] 38 tests run: 38 passed, 0 skipped
Summary [  16.108s] 38 tests run: 38 passed, 0 skipped
Summary [  16.049s] 38 tests run: 38 passed, 0 skipped
Summary [  16.093s] 38 tests run: 38 passed, 0 skipped
Summary [  16.097s] 38 tests run: 38 passed, 0 skipped
```

## What the count is and is not
- A count, not an exclusion: seven runs with no failure do not show that a collision cannot happen.
- Not a discriminator of the carry: the plan's baseline read seven of seven green on the untouched tree too, with
  bind-and-release still in `integration_engine_boot.rs`. The property is held by the two census entries of the
  same block, both green in this run: entry 6 (files defining `claim_port`, `PortClaim` or `pick_port`) reads `1`,
  entry 7 (binds of port 0 in the three release files) reads `0` at exit 1.
- One firing of the entry. The host was running nothing else of this session beside it; other projects' builds
  share this host's CPU and were not measured.
- The same four binaries ran once more inside entry 18 (the whole workspace, `3132 tests run: 3132 passed, 0
  skipped`) and once more under a cleared environment inside the pre-push check's `test` stage (entry 21, green).
