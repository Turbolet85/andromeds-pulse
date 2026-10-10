
## 2026-10-10-engine-harness-proofs-completed — the engine check is also read by the pre-push stage, its arms are mutation-read, and three test binaries start an engine
**Section:** §9 CI Integration → Pipeline integration (the `xtask test` row) · §9 → CI failure → artifact triage · §10 → CI gates (the heartbeat-stall bullet) · §10 → Load-profile constraints
**Change:**
- §9, the `xtask test` row and the triage bullet: was "one test binary of the run, `integration_console_engine`, spawns the console program"; now three do — it, and the two `xtask/tests` binaries `agent_run_shell` and `engine_cycle_refusals`, through the committed script and the built cycle verb — each under a cleared environment, each engine's log family under its test's own data dir, uploaded by no step. A failed assertion of the xtask suites prints the script's output with the scratch path and the passphrase replaced.
- §10 CI gates: outside CI, `check:engine-log` and then `ci-gates` are also read on the dev host by the sixth stage of `cargo xtask pre-push:linux`, over the log family of a real console engine cycle (335 records in each of four readings), where the stage had read `ci-gates` over a seed of three records it wrote itself. PROVISIONAL: the operator's reading, the founder's own word owed. It is no CI step.
- §10 Load-profile constraints: each of the check's seven arms, mutated by one line, reddened pins (6, 3, 7, 9, 7, 8, 8 of 45) and changed its own line over one real kept log; all stood again after the reverts. Limit stated: one mutation per arm, the passing side, n = 1; one heartbeat pin stayed green under the inverted gap comparison.
**Why:** The plan counted one engine-spawning test binary and named the `boot` job as the check's only reader; the mutation reading is what the pending row on the check's arms owed.
**Kept:** No log record, allowlist leaf or redaction shape changed at the chunk; §3, §6 and §8 are untouched.
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
