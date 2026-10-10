
## 2026-10-10-engine-harness-proofs-completed — a second separate tests directory, what the workspace run needs for it, and what per-process isolation does not give
**Section:** §2 Test Strategy → Test directory + naming conventions (Directory pattern; Test function naming) · §4 Unit Test Strategy → Conventions (Test file location, Rust) · §9 CI Integration → Pipeline structure (the `lint-test` row; the Integration tests row)
**Change:**
- Was "no separate `tests/` directory EXCEPT the `pulse-app` binary crate"; now a second: `xtask/tests/`, two integration binaries (`agent_run_shell.rs`, `engine_cycle_refusals.rs`) over a shared `common/mod.rs`, because they drive the committed script, the built `harness:engine-cycle` verb and the built engine as processes. xtask's unit pins stay co-located in `xtask/src`. `check:english-sources` (a sixth root) and `quarantine-tracking` (a fifth search dir) read it, and its absence fails both. Framework and runner unchanged; no dependency added.
- §4: `pulse-app/tests/*.rs` still counts 109 (no file added; three arms to `integration_console_engine.rs`, 7 in all).
- §9 `lint-test`: the workspace run also needs `bash`, a Linux `/proc`, an advisory file lock under `target/` and the engine built in the test profile; the quarantine pins are six, was five; green on `ci#38079305603`, 3112 tests (3064 on the previous verdict run); the same tests run in `mcp-test` and `coverage`.
- §9 Integration tests, the Parallel cell: was "per-process isolation prevents port conflicts"; now it separates processes, not port numbers — a port bound to 0, read and released can be handed to another test starting at the same instant (one of seven hand runs failed; 14 of 14 green after the change, a count, not an exclusion), so three binaries claim their ports under an advisory file lock in `target/test-ports/`.
**Why:** The convention named one exception and the chunk made a second; the runner's isolation was read as preventing a collision it was measured not to prevent.
**Kept:** `pulse-app/tests/integration_engine_boot.rs` still picks by bind-and-release; no owner was named at the amendment.
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
