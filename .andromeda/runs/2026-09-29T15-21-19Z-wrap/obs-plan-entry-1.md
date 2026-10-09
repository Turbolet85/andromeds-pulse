
## 2026-09-29-p-025-hue-shift-observable-made-gradable — hue_update_ms measures tier-effective → paint
**Section:** §8 PII Scrubbing → DELEGATED TIMING leaves → `metric.constellation.hue_update_ms`
**Change:** Was "measures span-arrival → the severity-driven hue the constellation DOT renders"; now `duration_ms` is the paint instant minus the service's `ServiceListItem.tier_effective_at_unix_nano` (rise = the opening incident's `opened_at_unix_nano`, fall = the last max-tier holder's `resolved_at_unix_nano`, Acknowledged keeps its tier), one record per service whose tier changed, emitted by `hueShiftSamples` only for changes witnessed after mount, graded by `cargo xtask smoke:hue-shift`. Fields, the dot-hue / not-a-Halo clause and the metric-fallback clause unchanged.
**Why:** the old interval did not measure the quantity the P-025 ≤ 2 s budget bounds (Conductor's measurement contract).
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
