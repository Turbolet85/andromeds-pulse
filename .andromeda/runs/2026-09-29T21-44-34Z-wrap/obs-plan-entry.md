
## 2026-09-29-ci-wall-time-and-round-trips — log artifacts per job
**Section:** §9 Telemetry artifact handling
**Change:** the log-file artifact (`logs/agent-latest.jsonl*`) is uploaded `if: always()` by the `lint-test` job per OS as `logs-${{ runner.os }}` and by the Linux `boot` job as `logs-boot-${{ runner.os }}` — the boot smoke's log that `ci-gates` reads in that job (was: "every CI job (fmt + clippy + xtask test + release)"); the names differ because upload-artifact v4 refuses a duplicate within a run.
**Why:** ci.yml split into seven jobs; `ci-gates` now runs only in `boot`, after the smoke that writes its log.
**Kept:** the perf-budget gate's VACUOUS status and its owner — `ci-gates` still reads only the boot-smoke log.
**Ref:** .andromeda/runs/2026-09-29T21-44-34Z-wrap/
