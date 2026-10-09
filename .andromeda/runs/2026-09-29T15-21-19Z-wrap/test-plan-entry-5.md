
## 2026-09-29-p-025-hue-shift-observable-made-gradable — coverage measure excludes xtask TEMPORARILY
**Section:** §4 Coverage target · §9 Coverage report row · §10 Cumulative across workspace
**Change:** New: `xtask/` is excluded from the coverage measure via `COVERAGE_IGNORE_FILENAME_REGEX` in `cargo xtask test:coverage` — TEMPORARILY. Measured on the CI lcov: line 84.3 % / function 83.95 % with xtask, 88.6 % / 87.80 % without (the function gate failed with it). Review point: the next epoch-boundary code audit, covering coverage quality, thresholds above 85 % and xtask's re-inclusion. The thresholds (75 / 70 / 85) are unchanged.
**Why:** founder ruling 2026-09-29 — exclude now, but only temporarily; the 85 % bar itself is to be revisited upward.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
