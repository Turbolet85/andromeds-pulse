
## 2026-10-10-no-gate-stands-while-reading-nothing — the 70 % branch threshold retired (PROVISIONAL); an empty coverage report fails
**Section:** §4 Unit Test Strategy → Coverage target · §9 → Pipeline structure (Quality gates row) · §9 → Build failure conditions · §10 → Coverage thresholds table · §10 → Build failure conditions · §3 → Bootstrap phases (item 8)
**Change:**
- Was three thresholds, line ≥ 75 % · branch ≥ 70 % · function ≥ 85 %; now two, line ≥ 75 % and function ≥ 85 %, at all six sites. The branch threshold is RETIRED as never measured, PROVISIONAL: retired on the operator's reading (the pc overseer, 2026-10-10), awaiting the founder's own word.
- §4 states the cause and the return condition: the arm read `Branch: 0/0 = 100.0%` on every run read, because branch instrumentation is a nightly-only compiler feature (`-Z coverage-options=branch`, `cargo llvm-cov --branch`), the project pins `1.95.0`, and `cargo xtask test:coverage` asks for no branch count (144 `BRF:0` and 144 `BRH:0` records, no `BRDA:` line, in the `coverage-linux` artifact of `ci#38038281709`). The arm returns when that feature is stable on the pinned channel, or when the founder admits a second channel for the `coverage` job, with a measured first reading before any threshold is stated.
- The Quality gates row and both Build failure conditions lists gain: a report tracking 0 lines or 0 functions fails the step (exit 1), and an absent `lcov.info` fails it. Before, a report tracking nothing exited 0.
- The table's branch cell reads "not enforced"; the key file's item 8 names function ≥ 85 % as the second enforced threshold.
**Why:** A threshold that has never read a number is a false statement, and a second unpinned toolchain is not its repair (the operator's answer at the plan's dialog). The coverage step passed over an empty report; the chunk made that an exit 1, pinned by four witness tests that run the step's own script.
**Kept:** The §11 ban "NEVER lower coverage threshold to pass build" stands unedited: no measured threshold was lowered. The TEMPORARY `xtask/` exclusion stands as the founder ruled it on 2026-09-29.
**Ref:** .andromeda/runs/2026-10-10T10-27-16Z-wrap/
