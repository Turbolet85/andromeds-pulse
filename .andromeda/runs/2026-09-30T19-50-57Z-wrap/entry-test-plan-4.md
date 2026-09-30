
## 2026-09-30-perf-instruments-measure-their-budgets — release row saves on failure
**Section:** §9 Release build row
**Change:** `release-${{ runner.os }}` saves on failure too (`cache-on-failure: true`); post-change re-read on `ci#36765040464`: 10 605 172 169 B, 8 entries, 1.23 % headroom, no eviction — a watch.
**Why:** the owning keys save on a red round; the release key had not, and a red release round rebuilt Windows cold.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
