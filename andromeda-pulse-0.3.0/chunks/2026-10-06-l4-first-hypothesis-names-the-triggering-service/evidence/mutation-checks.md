# Mutation checks (plan Step 9)

Three one-shot mutations. Each was applied with the Edit tool, confirmed present by a grep before the run, run, then
reverted and confirmed reverted. After the last revert both suites read green again:
`cargo nextest run -p interpretation` 130 passed, `cargo test -p pulse-app --example l4_decision_probe` 91 passed,
`cargo fmt --check` exit 0.

Baseline before any mutation: the probe binary 91 passed, 0 failed; the interpretation crate 130 passed.

## 1. The whole-word rule replaced by a substring match

- **Mutation** (`pulse-app/examples/l4_decision_probe.rs`, in `identifies`): the service half
  `names_whole_word(&lower, &id.to_ascii_lowercase())` became `lower.contains(&id.to_ascii_lowercase())`.
- **Confirmed present:** the mutated expression counted 1, the original 0.
- **Run:** `cargo test -p pulse-app --example l4_decision_probe` — exit 101, `89 passed; 2 failed`.
- **Red pins:**
  - `tests::identifies_reads_the_service_alone_where_a_joined_sibling_fails` — the Step 5 sibling pin:
    `assertion left == right failed: conductor-canary` · `left: "both"` · `right: "signal_only"`.
  - `tests::identifies_limit_a_joining_neighbour_rejects_wherever_it_stands`:
    `assertion left == right failed: Retry storm on pre-conductor` · `left: "both"` · `right: "signal_only"`.
- **Stayed green under the mutation** (they do not discriminate the whole-word rule, by design): the
  both-names pin, the space-separated-sibling limit, the negation limit and the retry-token pins.
- **Reverted:** the original expression counts 1 again.

## 2. The sentence removed from the const

- **Mutation** (`crates/interpretation/src/prompt.rs`): `TRIGGER_FRAMING_INSTRUCTION` cut back to end at
  `current signal.` (the chunk-base text).
- **Confirmed present:** the sentence's opening line counted 0 in the const; the const's last line read
  `current signal.";`.
- **Run:** `cargo nextest run -p interpretation -E 'test(every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope)'`
  — exit 100, `1 test run: 0 passed, 1 failed, 129 skipped`.
- **Red pin:** `prompt::tests::every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope` — the Step 2 pin:
  `assertion left == right failed: primary: the scope obligation appears exactly once` · `left: 0` · `right: 1`.
- **Reverted:** the probe pin `the_shipped_arm_carries_the_scope_sentence_once_and_the_baselines_do_not` asserts the
  const equals the v2.5 text, one space, and the pre-registered sentence, byte for byte; it is green after the revert.

## 3. The selection order reversed

- **Mutation** (`pulse-app/examples/l4_decision_probe.rs`): `SELECTION_ORDER` became `["LI", "L", "shipped"]`.
- **Confirmed present:** the const line read back reversed.
- **Run:** `cargo test -p pulse-app --example l4_decision_probe` — exit 101, `89 passed; 2 failed`.
- **Red pins:**
  - `tests::selection_takes_the_first_arm_in_order_whose_bar_is_met` — the Step 8 selection pin:
    `assertion left == right failed` · `left: "LI"` · `right: "shipped"`.
  - `tests::the_verdict_lines_print_the_selection_then_the_verdict_and_the_guard_last`: `assertion failed: pass`
    (with every arm meeting the bar, the reversed order selects `LI`, so the service verdict reads FAIL).
- **Reverted:** the const line reads `["shipped", "L", "LI"]`.

Recorded 2026-10-07T00:06:15Z.
