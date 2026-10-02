
## 2026-09-30-perf-budget-gate-reads-real-samples — agent-run.ps1 writes the boot end-status records
**Section:** §Occupied Resources → xtask CLI surfaces (`scripts/agent-run.{sh,ps1}`, `harness:status` `ended`) · §Occupied Resources → Filesystem locations (`run/andromeda-pulse.spawn` + `.exit`)
**Change:** was "`agent-run.ps1` does not mirror the recorder" and "`ended` null under `agent-run.ps1`"; now the ps1 `boot` hidden `powershell -EncodedCommand` wrapper writes the app pid to `.spawn` and `exit N` (ASCII, no BOM) to `.exit`, with the ≤ 5 s spawn poll, the no-spawn-record exit 1 and the ended / still-running diagnosis; `ended` is real under both scripts (`exit -1` after `Stop-Process -Force`). Both files remain harness-written, never by the product.
**Why:** the chunk discharged the CARRY that left Windows `harness:status` unable to say how the app ended.
**Kept:** the five verbs, exit semantics and status/cleanup field set.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
