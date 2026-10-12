
## 2026-10-10-agent-harness-drives-the-console-engine — the heartbeat-gap, zero-records and memory readings are CI-enforced for a console engine's log
**Section:** §3 → Heartbeat ticks (Stall detection) · §9 (Telemetry artifact handling: the Log file and Snapshot rows; Pipeline integration; the triage workflow) · §10 (Standard+ invariants: Heartbeat ticks; Performance budgets: the memory row and the frame row; CI gates: the zero-records, heartbeat and perf-budget bullets; Load-profile constraints)
**Change:**
- Heartbeat bullet: was "NO CI step makes the gap check: its CI enforcement stands unmet and is carried on the working route"; now the `heartbeat-gap` arm of `cargo xtask check:engine-log`, run by the `boot` job's `Console engine cycle` step, grades a console engine's log (the three engine ticks; over 45 000 ms FAIL; fewer than two records cannot-evaluate), and no CI step grades the window app's ticks.
- Zero-records bullet: was "made by no CI step and stands unmet"; now made for the console engine by the check's `family` and `progress` arms; the test-run form is made by no step.
- `cargo xtask check:ingest-progress` is named as a verb that reads NEUTRAL, exit 0, over a log with no `buffer.tick`; the readings that cannot be neutral are the engine check's (§3 and §10).
- Perf: was "`perf:budget` is the whole perf gate"; now two perf gates, `perf:budget` in `lint-test` and the engine check's `budget` arm in `boot` (memory, required); the memory row and the grader's caller list say so; the frame row's "no frame line" is the perf-budget `frame:` line.
- §9: three log uploads, not two; a run keeps two app logs; `logs-engine-Linux` holds the engine's log family plus `boot.log` and `build.log`, whose lines carry the runner's checkout paths; the triage list and the Snapshot row's owner clause name the engine's kept log.
**Why:** The chunk built the engine reading of two CI gates this plan stated as unmet, and a second caller of the perf grader; the plan said nothing enforced them.
**Kept:** The two §11 bans (snapshot and log on a test failure) stay unmet under `Engine end-to-end gate reachable`. Whether the upload's class covers its two harness files is stated as the operator's word at this wrap's card.
**Ref:** .andromeda/runs/2026-10-10T16-47-34Z-wrap/
