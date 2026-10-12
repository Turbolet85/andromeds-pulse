
## 2026-10-10-engine-harness-proofs-completed — the xtask package's integration tests and the two lints that read them
**Section:** §Stack and Technologies → Gate-script pins (test-only) · §Occupied Resources → xtask CLI surfaces (`cargo xtask check:english-sources`; `cargo xtask quarantine-tracking`)
**Change:**
- Gate-script pins: the `xtask` package has its first integration tests, two binaries in `xtask/tests/` (`agent_run_shell`, 19 tests; `engine_cycle_refusals`, 6) over `xtask/tests/common/mod.rs`, running the committed `scripts/agent-run.sh` and the built `harness:engine-cycle` verb with `bash` on real console engines. With three new spawned arms of `integration_console_engine.rs`, a host that runs the workspace tests needs `bash`, a Linux `/proc`, an advisory file lock under `target/` and the engine built in the test profile. No dependency, manifest or lockfile change. They run in `lint / test`, `mcp-server tests` and `coverage gate`.
- The quarantine script's pins: was five; now six.
- `check:english-sources`: was five roots; now six, `xtask/tests` added. `quarantine-tracking`: was four search dirs; now five, `xtask/tests` added, in the `.sh` and the `.ps1` mirror. An absent `xtask/tests` fails both.
**Why:** The shell-level and refusal pins drive processes, which a co-located test module cannot; the two lints had to read the new directory in the same commit or it would stand unlinted.
**Kept:** The `.ps1` mirror's new entry is parsed, never run.
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
