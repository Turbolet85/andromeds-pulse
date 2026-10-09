
## 2026-10-06T21-47-06Z-wrap — §1 l4-latency-p99-ps1-run-coverage: pwsh is on the dev host; only the run is owed
**Section:** §1 Pending coverage triggers → `l4-latency-p99-ps1-run-coverage`
**Change:** Was "`pwsh` is absent on the dev host" and "installing `pwsh` needs the founder's sudo"; now `pwsh` 7.6.6 is on the Omarchy Linux dev host (`/usr/bin/pwsh`, as measured at this wrap) and only the run itself is owed. Unchanged: the `.ps1` half has never run; the owed run is the three fixtures under `pwsh`, reading the `.sh` half's exits and lines; its owner is the CARRY on the working-route entry "pre-push:linux runs natively on Linux".
**Why:** `pwsh` was installed on the host on 2026-10-06 and this wrap measured it; the overseer (founder-delegated) directed the correction taken in this wrap. The row's stated blocker stopped being true, so the run no longer waits on the founder.
**Ref:** .andromeda/runs/2026-10-06T21-47-06Z-wrap/
