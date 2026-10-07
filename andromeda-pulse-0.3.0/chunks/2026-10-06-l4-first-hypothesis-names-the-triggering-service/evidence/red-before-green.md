# Red before green — the scope-obligation pin (plan Step 2)

The pin `prompt::tests::every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope`
(`crates/interpretation/src/prompt.rs`) asserts that the clause
`must name that scope_id value exactly as written there` occurs exactly once in each of the three tiers' composed
prompts. It was written first and run against the untouched instruction (the chunk base's
`TRIGGER_FRAMING_INSTRUCTION`), then again after Step 3 appended the pre-registered sentence.

Command, both readings: `cargo nextest run -p interpretation -E 'test(every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope)'`
(the green reading is from the whole-crate run `cargo nextest run -p interpretation`).

## RED — the instruction as the chunk base carries it (exit 100)

```
    Starting 1 test across 1 binary (129 tests skipped)
        FAIL [   0.005s] (1/1) interpretation prompt::tests::every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope
    assertion `left == right` failed: primary: the scope obligation appears exactly once
      left: 0
     right: 1
     Summary [   0.005s] 1 test run: 0 passed, 1 failed, 129 skipped
```

The loop stops at its first tier, so the red names `primary`; the other two tiers push the same const.

## GREEN — after the sentence was appended (exit 0)

```
        PASS [   0.008s] ( 66/130) interpretation prompt::tests::every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope
     Summary [   0.066s] 130 tests run: 130 passed, 0 skipped
```

The crate went from 129 tests to 130: the one new pin. The three lineage asserts in the same crate
(`v2.6` / `v1.5-fallback` / `v1.5-reflection`) are in that 130.

Recorded 2026-10-07T00:06:15Z.
