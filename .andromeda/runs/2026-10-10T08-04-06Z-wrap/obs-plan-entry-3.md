
## 2026-10-10-no-ci-step-reads-nothing — owners named at the card: the unmet bans, and the engine's readings on its node
**Section:** §9 Telemetry artifact handling (the Snapshot markdown row) · §10 CI gates (the perf-budget bullet)
**Change:**
- Snapshot markdown row: the §11 bans that name it stand, unmet, and are now owned by the route entry `Engine end-to-end gate reachable`, which keeps the engine's log in CI. Was, in this pass's earlier entry: unmet with no owner.
- §10 CI gates: after "nothing compares a run with an earlier one", the route's owners of the engine's readings on its node are named: its memory by `Engine memory measured` (P-090), its behaviour under load by `Load profiles re-based on the engine` and `Disk store measured under load` (P-090, P-091); no route entry owns a comparison of one run with an earlier one.
**Why:** The operator ruled that a ban standing unmet has an owner from now, a carry on that entry, and asked that the corrected sentence name the entry that owns a performance or memory reading of the engine, or say none (the operator, the pc overseer, at this wrap's route-resolve card, 2026-10-10).
**Kept:** §11's two bans are unchanged in the body. A machine-read test report, perf regression against an earlier run and a pull-request comment with new a11y violations stand corrected as measured with no owner.
**Ref:** .andromeda/runs/2026-10-10T08-04-06Z-wrap/
