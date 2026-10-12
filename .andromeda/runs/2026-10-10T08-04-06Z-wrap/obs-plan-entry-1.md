
## 2026-10-10-no-ci-step-reads-nothing — no bench suite exists: the criterion claim retired at its eight sites
**Section:** §1 (perf-budget-instruments, the snapshot token budget row) · §2 (the Metrics row) · §5 (why `tracing` events instead of an OTel Meter) · §9 Telemetry artifact handling (the Criterion bench JSON row) · §9 Pipeline integration (the `xtask bench` row) · §10 Performance budgets (the snapshot row, the frame row) · §10 CI gates (the perf-budget bullet)
**Change:**
- Was, in each: "`criterion` 0.5 in `xtask benches/`" as an offline regression assertion, "`xtask benches/snapshot.rs` provides regression detection on stable estimators (median + slope)", "offline criterion bench for stable hardware regression detection", "the `xtask bench` half", a Criterion bench JSON artifact and an `xtask bench` stage.
- Now, in each: no bench suite exists — no `criterion` dependency, no `xtask benches/` directory, no `bench` verb. The snapshot p99 ≤ 500 ms assertion is the snapshot arm of `cargo xtask perf:budget` over the emitted events; SLO-critical budgets are asserted by `perf:budget` over the `metric.*` stream; the budgets are absolute, and nothing compares a run with an earlier one.
- §9: the Criterion bench JSON row reads "not produced"; the `xtask bench` stage row is gone, with one sentence under the table saying no step runs a bench.
- §10 CI gates: there is no `xtask bench` half; the `criterion-regression` verb, its CI step, the `criterion-Linux` upload and the base-branch criterion download left `ci.yml` and `xtask`, each having read nothing on `ci#38031822696`; `perf:budget` is the whole perf gate.
**Why:** The bench the plan described was never built, and the CI steps written for it compared nothing with nothing. The operator ruled them gone whole and asked that the absence be stated as measured (the operator, the pc overseer, at the P4 dialog and by the wrap directive, 2026-10-10).
**Kept:** The absolute budgets and their grader are unchanged. Whether the version still wants regression detection against an earlier run was put to the operator at this wrap's route-resolve card.
**Ref:** .andromeda/runs/2026-10-10T08-04-06Z-wrap/
