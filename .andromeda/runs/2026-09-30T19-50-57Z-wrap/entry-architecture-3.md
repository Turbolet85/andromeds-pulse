
## 2026-09-30-perf-instruments-measure-their-budgets — release cache saves on failure
**Section:** §Infrastructure Patterns → CI/CD (Rust cache budget)
**Change:** `release-{os}` saves on failure too (`cache-on-failure: true`), like the `lint-test` and `boot` owning keys. Re-read after the change on `ci#36765040464`: byte-identical, 8 entries, 132 246 071 B (1.23 %) headroom, no key evicted — still a watch.
**Why:** a red release round had saved nothing, so the next rebuilt cold; the same key re-saves and no lockfile moved, so the change adds no cache growth.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
