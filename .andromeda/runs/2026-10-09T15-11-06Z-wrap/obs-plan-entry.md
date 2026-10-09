
## 2026-10-09-ci-on-linux-alone — §9 Log file row: the `lint-test` job uploads from one runner system
**Section:** §9 CI Integration → Telemetry artifact handling → Log file row
**Change:** Was: "the `lint-test` job per OS (`logs-${{ runner.os }}`)". Now: the `lint-test` job, on one runner system (`ubuntu-22.04`) since chunk 2026-10-09-ci-on-linux-alone. The rest of the row stands as written.
**Why:** The chunk removed the Windows and macOS runners from the `ci` workflow.
**Kept:** The row's statement that `lint-test` uploads `logs-${{ runner.os }}`, with the Criterion bench JSON row, the Pipeline integration consumer cells and the triage workflow's `-n logs`: found standing false on the two runs read (no `logs-Linux`, `nextest-Linux` or `criterion-Linux` artifact exists; the upload steps find no file). Not amended here: the family has no named owner yet and goes to route-resolve.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
