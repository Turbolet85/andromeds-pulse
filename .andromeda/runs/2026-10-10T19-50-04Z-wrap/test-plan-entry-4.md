
## 2026-10-10-engine-harness-proofs-completed — what three pending rows and the Integration tests row leave open has an owner from the route-resolve card
**Section:** §1 Test Scope Summary → Pending coverage triggers (`harness-cleanup-verdict-and-boot-spawn-shell-coverage`; `engine-log-arm-mutation-and-engine-cycle-glue-coverage`; `pre-push-engine-cycle-stage-glue-coverage`) · §9 CI Integration → Pipeline structure (the Integration tests row)
**Change:**
- The engine-log row's remainder (a runner red cycle, the `run_check` / `run_settled` pins, the heartbeat pin that stayed green under inversion) and the whole of `pre-push-engine-cycle-stage-glue-coverage`: owned by the route entry `Engine end-to-end gate reachable`.
- The shell-coverage row's two behaviours pinned as they stand: on `cleanup` reading an unreaped child as alive nothing is owed (under `boot` the waiting wrapper reaps the app); `boot` with no spawn record leaving its engine running is owned by the route entry `One place on a node`.
- §9: was "no owner named" for `integration_engine_boot.rs`'s bind-and-release picking; now owned by the route entry `Shared telemetry test data`.
**Why:** A pending row with no owner on the route is drift, not sequencing; the operator gave each its owner or its nothing-owed at the card (the pc overseer, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T19-50-04Z-wrap/
