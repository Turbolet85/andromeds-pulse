
## 2026-10-05-l4-model-chosen-by-pattern-discrimination — the decision probe's pins 29 → 61
**Section:** §1 → `l4-decision-probe-arg-parse-unit-coverage`
**Change:** The row adds the extension to 61 collected pins (32 new, 27 in the `#[path]`-wired `pulse-app/examples/l4_decision_probe/patterns.rs`): the pattern shapes A1–A7 / B1–B3 / C1–C3 (parse, no mixing with S shapes, every shape × render within `MAX_PROMPT_BYTES`, the cueless TRIGGER / OVERALL rendering, the enriched render's exactly-once transform, argv-only arms), the closed-label `valid` / `detect` / `cause` scorer with its asymmetric pairs and `no_reading` rows, the stored-output re-grade, the seeded audit draw and verdict grade, the target-only out-dir guard and the recommendation rule; mutation-checked (a)–(f) plus (e'). The STILL OWED list now reads `--renders` / `--audit-draw` / `--audit-seed` / `--audit-grade` / `--table` among the parse-pinned flags.
**Why:** The chunk's pins, measured collected by `cargo nextest run --workspace --profile ci -E 'binary(l4_decision_probe)'` (61 run, 61 passed); the plan's recorded expected amendment.
**Ref:** .andromeda/runs/2026-10-05T13-31-12Z-wrap/
