
## 2026-09-29-p-025-hue-shift-observable-made-gradable — pending coverage triggers: boot failure branch, perf-slo-check arms
**Section:** §1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage` · `perf-slo-check-arm-coverage` (new)
**Change:**
- Widened: the sh `boot` failure-diagnosis branch also ships with no committed test; it is the instrument of the open Linux-boot watch. Owed now: an assertion per verdict arm and per boot failure-reason arm, plus the ps1 boot leg.
- New row: `xtask/ci/perf-slo-check.sh` reads an empty metric stream as NEUTRAL (it died under `pipefail` on an empty `grep | sort`), and none of its arms has a committed test. Owed: empty ⇒ NEUTRAL, in-budget ⇒ PASS, over-budget ⇒ FAIL. It points to the CI vacuity owned by "Perf-budget gate reads real samples".
**Why:** both branches changed in the chunk's operator pass with CI exit behaviour as their only evidence.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
