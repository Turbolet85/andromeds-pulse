
## 2026-10-06T21-47-06Z-wrap — §10: pwsh is on the dev host; only the .ps1 grader run is owed
**Section:** §10 SLO Invariants & Telemetry Budgets → Performance budgets (the L4 constrained inference gpu-primary row)
**Change:** Was "The `.ps1` half has not run (no `pwsh` on the dev host)"; now the `.ps1` half has not run, `pwsh` 7.6.6 is on the Omarchy Linux dev host (`/usr/bin/pwsh`, as measured at this wrap), and only the run itself is owed. The budgets, the nearest-rank rule and the measured p99 are unchanged. Leaves re-derived: `.claude/rules/observability.md` and `.claude/docs/obs-summary.md`, each of which restated the absence.
**Why:** `pwsh` was installed on the host on 2026-10-06 and this wrap measured it; the overseer (founder-delegated) directed the correction taken in this wrap. The stated reason for the unrun half stopped being true; the run stays owned by the CARRY on the working-route entry "pre-push:linux runs natively on Linux".
**Ref:** .andromeda/runs/2026-10-06T21-47-06Z-wrap/
