
## 2026-09-30-perf-budget-gate-reads-real-samples — agent-run.ps1 boot mirrors the spawn/exit recorder
**Section:** §3 `boot` · §3 `status` · §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`
**Change:**
- `boot`: was "`agent-run.ps1` does not mirror this recorder"; now a hidden `powershell -EncodedCommand` wrapper writes `run/andromeda-pulse.spawn` and `run/andromeda-pulse.exit` (`exit N`, ASCII, no BOM); ≤ 5 s spawn poll with the no-spawn-record exit 1; `app ended:` / still-running diagnosis on a failed poll.
- `status`: was "`ended` always null under ps1"; now real under both scripts (measured `exit -1` after `Stop-Process -Force`).
- The trigger is WIDENED, not discharged: the ps1 `ended` leg ran once by hand, a one-time proof; owed: a committed ps1 boot-cycle leg over the recorder's arms. The ps1 cleanup live leg is still unmeasured.
**Why:** the chunk closed the CARRY that left `ended` empty on Windows. By this trigger's own one-time-proof standard, a by-hand run does not discharge a committed-test obligation.
**Kept:** the verb set, exit semantics and status/cleanup fields — unchanged.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
