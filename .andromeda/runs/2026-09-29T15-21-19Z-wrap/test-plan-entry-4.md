
## 2026-09-29-p-025-hue-shift-observable-made-gradable — smoke:hue-shift registered as the third scenario leg
**Section:** §3 Per-chunk gate discipline → scenario legs
**Change:** New: `cargo xtask smoke:hue-shift` (`xtask/src/hue_shift.rs`) — not a standard gate, dev-host only, not CI-wired. Grades `metric.constellation.hue_update_ms`: the rise against `interpretation.incident.created`, the fall against the first `triage.incident.auto_resolve.tick` with `resolved_count ≥ 1`, each on anchor error ≤ 1000 ms (the ≤ 2000 ms budget is context; Conductor grades P-025). The storm stops once the rise paints; both samples are graded. Exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE (a precondition unmet); artifact under `target/hue-shift/`.
**Why:** the chunk's live leg, RED at the pre-fix HEAD and GREEN after.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
