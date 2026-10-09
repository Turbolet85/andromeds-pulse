# Mutation checks — plan Step 5

One-shot controls, never gates. Each guard was neutralized with the anchored Edit tool, the mutation was
grep-confirmed present before the run (count 1), the pin was run with
`cargo test -p pulse-app --example l4_decision_probe {filter}`, and the guard was restored and grep-confirmed. A green
run would have been a failed mutation; every one went RED.

Before the first mutation and after the last restore, both sources hashed identically (byte-identical restore):

| file | sha256 before | sha256 after |
|---|---|---|
| `pulse-app/examples/l4_decision_probe/patterns.rs` | `d597e7ccef151ce4413cf46f2a0628c8d3a19ba41c289da7346bf57d1040229d` | the same |
| `pulse-app/examples/l4_decision_probe.rs` | `1b9f94d96eb9a01c11fecdea1eb9f3f0b9459210b00f1a225f23ca0f45d2c289` | the same |

After the restore: `test result: ok. 61 passed; 0 failed`.

| | guard neutralized | mutation | pin(s) | reading |
|---|---|---|---|---|
| (a) | the B `false_alarm` branch of `detect_label` | `(Family::B, true) => "quiet"` | `scorer_detect_pairs_split_surface_from_the_rest` | RED — `left: "quiet"` / `right: "false_alarm"` (exit 101) |
| (b) | the cause service atom of `cause_label` | `let service = true;` | `scorer_cause_needs_the_service_and_a_cause_word`, `scorer_cause_reads_the_first_hypothesis_only` | RED ×2 — `left: "hit"` / `right: "word_only"` (A7, the cue service X named) and `left: "service_only"` / `right: "none"` |
| (c) | the no-reading branch of `detect_label` | the `if valid != "valid" { return "no_reading"; }` lines removed | `scorer_zero_generation_or_parse_failed_row_is_no_reading` | RED — `left: "miss"` / `right: "no_reading"` (the first case, A1; a B row reads `quiet` under the same mutation) |
| (d) | the audit draw's seed | `SplitMix64::new(0)` | `audit_draw_is_seeded_and_takes_ceil_ten_percent` | RED — the draw for seed 20261005 equals the draw for seed 7 (the `assert_ne`) |
| (e) | the out-dir guard, lexical half | `let absolute = cwd.join(out);` in `texts_root` | `out_dir_guard_accepts_only_paths_under_target` | RED — "nothing created outside target/" (the resolved half still refused, after creating the dir) |
| (e') | the out-dir guard, resolved half | `if false && !resolved.starts_with(&target)` | `out_dir_guard_refuses_a_symlink_out_of_target` | RED — a symlink under `target/` leading out was accepted |
| (f) | the rule's VRAM order | the VRAM sort key replaced by `0u64` | `recommend_picks_the_lightest_qualifying_model_by_vram` | RED — `left: Some("heavy-best")` / `right: Some("light-close")` (RSS alone picks the heavier model) |

Pins that stayed green under a mutation, as designed: under (d) the mode-parse pin; under (e) the symlink pin; under
(e') the lexical pin; under (f) the RSS-tiebreak, none/cannot-evaluate and table pins (their fixtures do not order
VRAM against RSS).

Note on (f): the pin's fixture was written so that VRAM and RSS order the two qualifying models differently (the
heavier-VRAM model carries the lower RSS), the shape the predecessor measured between gemma-4-E2B and Llama. Without
that, neutralizing the VRAM key would have left the pin green.
