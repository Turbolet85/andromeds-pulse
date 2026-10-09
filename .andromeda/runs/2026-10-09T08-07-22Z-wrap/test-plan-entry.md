
## 2026-10-09T08-07-22Z-wrap — the .ps1 grader run is dropped; the native pre-push port left the route for the residuals
**Section:** §1 `l4-latency-p99-ps1-run-coverage` · §3 → Per-chunk gate discipline (the key file `registries/contracts/test-plan/per-chunk-gate-discipline.md`)
**Change:**
- §1 row. Was: `pwsh` is on the dev host "so only the run itself is owed", owned by the CARRY on the working-route entry "pre-push:linux runs natively on Linux". Now: the run was never made and was dropped at the 2026-10-09 version close; the `.ps1` half stands unrun and nothing owns a run. The row's measured `.sh` readings and its statement that the `.ps1` half has never run are unchanged.
- Key file. Was: porting `pre-push:linux` to native Linux "is its own route entry", with its placement. Now: the port was not done in 0.3.0; its route entry was removed unbuilt at the 2026-10-09 version close on the founder's ruling and carried to the next version as a residual (`.andromeda/residuals.md`: the local pre-push check without WSL).
- A partial retirement of `2026-10-06T21-47-06Z-wrap — §1 l4-latency-p99-ps1-run-coverage: pwsh is on the dev host; only the run is owed`: the run is no longer owed; that `pwsh` is on the dev host still stands.
**Why:** The founder closed 0.3.0 as it stands on 2026-10-09 (his own pick, relayed verbatim by the pc overseer), and the option he picked carries the pre-push entry to the next version, so the entry both passages named as owner is gone from the route. Dropping the `.ps1` run is the operator's word at this wrap: the grader grades the local model's latency and Windows is not a target host. Standing fact: the `.ps1` half of the L4 latency grader has never run on any host.
**Ref:** .andromeda/runs/2026-10-09T08-07-22Z-wrap/
