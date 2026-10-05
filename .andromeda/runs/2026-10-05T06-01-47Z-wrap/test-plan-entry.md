
## 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape — prompt lineage v2.5 and the first-hypothesis obligation pin; probe pins 8 → 16
**Section:** §4 Unit Test Strategy → interpretation crate · §1 Pending coverage triggers → `l4-decision-probe-arg-parse-unit-coverage`
**Change:**
- §4: the lineage was v2.4 / v1.3-*; now v2.5 / v1.4-fallback / v1.4-reflection. `TRIGGER_FRAMING_INSTRUCTION` is restated with its two new obligations, and `every_tier_obliges_the_first_hypothesis_to_name_the_trigger_signal` pins the obligation clause exactly once in every tier's composed prompt (mutation-checked: reverting the product text alone reads `left: 0, right: 1`); `-p interpretation` 128 pins.
- §1: the probe was 8 collected pins; now 16 — the `--shapes` trio through the `parse_args_from(iter)` seam, the `names_trigger_stem` label set and its asymmetric strict-vs-stem pair, and three candidate-arm pins (text exactly once, identity once shipped, R2 schema change confined to the hypotheses description); the bound pin also asserts S1–S6. The flag parse of every flag but `--shapes` stays owed.
**Why:** The chunk shipped the reworded framing with its lineage bump, and the probe gained the arms, flag and grader the pre-registered series needed; each new pin was mutation-checked where the plan named one.
**Ref:** .andromeda/runs/2026-10-05T06-01-47Z-wrap/
