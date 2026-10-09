
## 2026-10-09-ci-on-linux-alone — §4 buffer crate: the stale-comment citation corrected at the first citation sweep
**Section:** §4 Unit Test Strategy → buffer crate bullet
**Change:** Was: the source comment in `crates/buffer/src/schema.rs` "still states the disproved cause and is owned by its own route entry". Now: the comment above `spans_primary_key_is_composite_trace_id_span_id` states the measured behaviour since chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep, while a sibling comment above the `metrics_points` key test still states the disproved cause (read 2026-10-09). The ownership clause is dropped.
**Why:** The first citation sweep listed the citation `changed`; the read found the cited comment rewritten and the claim false as stated. No route entry, residual or requirement names the comment that still stands. Trap for later chunks: a comment corrected above one test can leave its sibling's copy standing.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
