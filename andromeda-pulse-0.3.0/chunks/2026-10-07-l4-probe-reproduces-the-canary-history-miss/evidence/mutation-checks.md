# Mutation checks

One-shot mutations, one per new discriminating pin. Each was applied with the Edit tool, confirmed present by a
fixed-string grep before the run (the count is in the row), run with
`cargo test -p pulse-app --example l4_decision_probe`, then reverted. A mutation and its run were never fired in one
batch.

## Steps 2-4 (the probe before the first reading)

Baseline before any mutation: 105 passed, 0 failed (91 at the chunk base plus 14 new pins). Every run below exited
101. After the last revert: each mutation's text counts 0 in the file, `cargo fmt --check` exit 0, 105 passed again.

| # | Mutation (`pulse-app/examples/l4_decision_probe.rs`) | Present | Result | Red pins |
|---|---|---|---|---|
| A | `selected_corpus_incidents` passes no scope (`&scopes[..0]`), so only the fingerprint arm selects | 1 | 97 passed, 8 failed | `s1_to_s8_keep_their_corpus_blocks_through_the_product_selection` (S4: `[]` against its one line) · `the_shipped_arm_composes_s4_s7_and_s8_at_their_recorded_sizes` (S4: 7453 against 7654) · `each_new_shape_holds_exactly_the_lines_its_row_names_in_order` (S12) · `each_new_shape_differs_from_s7_in_the_corpus_block_alone` (S12: 0 framing notes) · `s16_holds_two_lines_for_the_triggering_service_and_three_for_the_sibling` (`(2, 2, 0)` against `(5, 2, 3)`) · `the_position_pairs_hold_the_same_lines_in_a_different_order` · and two pre-existing S4 pins, `nf_arm_strips_exactly_the_three_framing_lines` and `s4_renders_a_framed_corpus_match_beside_the_trigger` |
| B | the shape id `"S16"` in `shapes()` renamed `"S16x"` | 1 | 100 passed, 5 failed | `the_new_shape_ids_parse_and_select_in_shapes_order` · `every_arm_and_shape_composes_within_the_production_bound` (the id list) · the three pins that look S16 up |
| C | `sibling_shape` gives S9 alone a one-row services table | 1 | 104 passed, 1 failed | `each_new_shape_differs_from_s7_in_the_corpus_block_alone` (S9) |
| D | the 12-minute slot of `FIVE_LINE_SLOTS_MIXED_SCOPE` made `Resolved` | 1 | 104 passed, 1 failed | `each_new_shape_holds_exactly_the_lines_its_row_names_in_order` (S16) |
| E | S15 built with `specs.rotate_left(0)`, S14's order | 1 | 103 passed, 2 failed | `the_position_pairs_hold_the_same_lines_in_a_different_order` (`S15 reorders S14`) · `each_new_shape_holds_exactly_the_lines_its_row_names_in_order` (S15) |
| F | S16's 9-minute line scoped to the sibling, its text unchanged | 1 | 104 passed, 1 failed | `s16_holds_two_lines_for_the_triggering_service_and_three_for_the_sibling` (`(5, 1, 4)` against `(5, 2, 3)`) |
| G | `SIBLING_SHAPES` widened to `["S7", "S8", "S9"]` | 1 | 104 passed, 1 failed | `the_new_shapes_count_in_neither_half_of_the_bar` (`sibling_both: 20, sibling_n: 20` against the zero counts) |
| H | `reproduce_request`'s arm check replaced by `arms.is_empty()` | 1 | 104 passed, 1 failed | `the_reproduction_reading_takes_the_shipped_arm_a_new_shape_and_no_other_verdict_flag` (`shipped,ns`: `None` against the refusal) |
| I | `reproduce_shape` requires `counts.misses > k` | 1 | 103 passed, 2 failed | `a_shape_reproduces_at_k_misses_and_is_clean_only_below_k_with_its_unparsed_rows` (`2 0`: `UNREAD` against `REPRODUCES`) · `the_reproduction_verdict_reads_the_eight_new_shapes` (shapes `S14` against `S9,S14`) |
| J | `fold_miss` counts `unparsed` as a miss | 1 | 104 passed, 1 failed | `an_unparsed_generation_is_counted_apart_and_never_as_a_miss` (`misses: 4, unparsed: 0` against `misses: 2, unparsed: 2`) |
| K | `reproduction_verdict` reads every shape that ran (`shapes.keys()`), controls included, in map order | 1 | 103 passed, 2 failed | `a_control_changes_no_reproduction_verdict` (`REPRODUCED · shapes S7,S8` against `NOT REPRODUCED · shapes none`) · `the_reproduction_verdict_reads_the_eight_new_shapes` (shapes `S14,S9` against `S9,S14`) |
| L | `reproduce_line` prints `unread` for `unparsed` | 1 | 104 passed, 1 failed | `the_reproduce_line_takes_its_printed_form` |

What the runs show about the pins:

- Mutations C, D, F, G, H, J and L each reddened exactly one pin, the one written for that property.
- The recorded-sizes pin and the selection-equivalence pin both went red under A through S4 alone. S8's two lines carry
  the cue's fingerprint, so the fingerprint arm kept them without any scope: S8 does not discriminate the scope arm.
- Under A the S16 block kept its two `conductor` lines and lost the three sibling lines: only the scope arm brings a
  sibling line with another fingerprint into the digest.
- The single-variable pin (`each_new_shape_differs_from_s7_in_the_corpus_block_alone`) stayed green under D, E and F,
  which change the block alone. It is the pin for what lies outside the block; the lines pin holds the block.

A separate reading, not a mutation: the dry run over all 18 arms and S1-S8 printed 144 lines at the chunk base and 144
after Steps 2-4, and `cmp` of the two outputs exited 0 (every arm and shape at the same prompt bytes).

## Steps 6-9 (the replay input, the section reader, the block edits, the two reading rules)

Forty-four mutations, run 2026-10-07 from 12:44:47Z to 12:46:21Z (the script's first and last file writes, read from
the files), after the first green gate block of this run. They were applied by a script, one at a time, each step
gated on the one before:

1. the anchor text is asserted to occur exactly once in its file;
2. the anchor is replaced, in binary mode, every other byte kept;
3. the mutated line is counted in the file as written (the "Present" column: occurrences added, 1 in every row); a
   count other than 1 aborts before any run;
4. `cargo test -p pulse-app --example l4_decision_probe` runs;
5. the original bytes are written back and the file's sha256 is compared with the one read before step 2.

A run that did not compile would have been recorded as such; none occurred. Baseline before any mutation: 146 passed,
0 failed (105 on the tree the stopped run left, plus 41 new pins: 25 in `replay.rs`, 16 in the probe). Every run
below exited 101. After the last restore both files hash as before the first mutation (`l4_decision_probe.rs`
`c113a170…a533`, `replay.rs` `6ef4aa7a…882f`), and the gate block re-run on that tree read `cargo fmt --check` exit 0
and 146 passed.

`R` is `pulse-app/examples/l4_decision_probe/replay.rs`, `P` is `pulse-app/examples/l4_decision_probe.rs`. A pin
named without a module is in the probe's `tests`; the others are in `replay::tests`.

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 1 | R | a `shipped` replay passes the prompt with its last newline trimmed | 1 | 145 passed, 1 failed | `a_shapes_own_composition_replays_to_that_prompt_with_the_same_argv` |
| 2 | R | `argv_difference` compares the recorded grammar path | 1 | 139 passed, 7 failed | `a_recorded_model_and_grammar_path_are_not_compared` · and the six other pins that load a capture written with another grammar path |
| 3 | R | `argv_difference` checks a recorded operand for presence only | 1 | 145 passed, 1 failed | `a_recorded_argv_that_differs_is_refused_at_the_first_differing_flag` |
| 4 | R | `read_capture_file` accepts a directory under a capture file's name | 1 | 145 passed, 1 failed | `a_missing_directory_or_file_is_refused` |
| 5 | R | the capture file bound raised to 1 MiB | 1 | 145 passed, 1 failed | `an_oversized_or_non_utf8_file_is_refused` |
| 6 | R | the work-tree refusal inverted: under `target/` refused, elsewhere in the tree accepted | 1 | 145 passed, 1 failed | `a_capture_inside_the_work_tree_is_refused_outside_its_target` |
| 7 | R | a second `-p` in `argv.nul` is no longer refused | 1 | 145 passed, 1 failed | `argv_nul_needs_exactly_one_p_with_the_prompt_after_it` |
| 8 | R | `read_capture` drops the production bound's verdict | 1 | 145 passed, 1 failed | `a_prompt_the_production_bound_rejects_is_refused` |
| 9 | R | the scope check accepts any `scope_id` the cue line carries | 1 | 145 passed, 1 failed | `a_prompt_without_a_digest_a_known_first_cue_or_the_given_scope_is_refused` |
| 10 | R | a repeated `--replay` label is no longer refused | 1 | 145 passed, 1 failed | `replay_labels_are_short_ascii_words_and_none_is_repeated` |
| 11 | P | `row_json` writes the prompt into every row | 1 | 144 passed, 2 failed | `a_replay_row_and_run_line_hold_labels_and_no_prompt_text` · `tests::an_s_shape_row_and_run_line_hold_labels_and_no_model_text` (its key set) |
| 12 | R | the section reader files the corpus block under `digest.attention-cues` | 1 | 143 passed, 3 failed | `s8_against_s16_differs_in_the_corpus_block_alone` · `s7_against_s8_reads_the_block_absent_on_one_side` · `tests::the_sections_flag_is_a_mode_of_its_own` |
| 13 | R | `compare` counts an absent section as one line | 1 | 144 passed, 2 failed | `s7_against_s8_reads_the_block_absent_on_one_side` · `s8_against_s16_differs_in_the_corpus_block_alone` |
| 14 | R | an unknown `# ` header no longer opens `other` | 1 | 145 passed, 1 failed | `a_section_under_an_unknown_header_lands_in_other` |
| 15 | R | the reader no longer knows the `# Conventions` header | 1 | 144 passed, 2 failed | `every_shape_is_cut_whole_with_nothing_under_other` · `a_section_under_an_unknown_header_lands_in_other` |
| 16 | R | `--sections` accepts a single source | 1 | 145 passed, 1 failed | `section_sources_are_captures_or_shape_ids_two_or_more_and_none_repeated` |
| 17 | R | `CX` keeps the other lines and removes the own ones | 1 | 142 passed, 4 failed | `each_edit_equals_the_same_change_made_before_rendering` · `exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope` · `the_restate_edit_adds_its_line_once_and_is_identity_after` · `a_digest_that_ends_without_a_newline_gains_none` |
| 18 | R | every edit of a block also rewrites the `TRIGGER:` line | 1 | 141 passed, 5 failed | `an_edit_changes_nothing_outside_the_block_and_never_the_trigger_line` · `each_edit_equals_the_same_change_made_before_rendering` · and three pins that compare an edited prompt whole |
| 19 | R | `CC` keeps three lines | 1 | 144 passed, 2 failed | `cap_and_reorder_change_no_lines_text` · `each_edit_equals_the_same_change_made_before_rendering` |
| 20 | P | `own_lines` loses the fingerprint arm: only the cue's scope makes a line own | 1 | 144 passed, 2 failed | `exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope` · `each_edit_equals_the_same_change_made_before_rendering` |
| 21 | R | an edit appends a newline to a prompt without a block | 1 | 143 passed, 3 failed | `a_prompt_without_a_block_is_left_as_it_is` · `each_edit_equals_the_same_change_made_before_rendering` · `an_edit_changes_nothing_outside_the_block_and_never_the_trigger_line` |
| 22 | R | `corpus_block` forgets a restate line it read | 1 | 145 passed, 1 failed | `the_restate_edit_adds_its_line_once_and_is_identity_after` |
| 23 | R | `CO` and `CX` run with unknown own lines, as if none were own | 1 | 145 passed, 1 failed | `a_replay_takes_its_own_lines_by_position_and_skips_the_two_edits_without_them` |
| 24 | R | `--own-lines` accepts a repeated position | 1 | 145 passed, 1 failed | `own_lines_parse_as_unknown_none_or_distinct_positions` |
| 25 | R | an edit always ends the block on a newline | 1 | 145 passed, 1 failed | `a_digest_that_ends_without_a_newline_gains_none` |
| 26 | P | `--replay` without `--shapes` keeps the default S1-S4 | 1 | 144 passed, 2 failed | `tests::the_replay_flags_parse_and_a_replay_brings_no_s_shape_of_its_own` · `tests::the_replay_reading_takes_the_shipped_arm_and_a_replay_labelled_miss` |
| 27 | P | `--own-lines` applies to every replay | 1 | 145 passed, 1 failed | `tests::the_replay_flags_parse_and_a_replay_brings_no_s_shape_of_its_own` |
| 28 | P | the replay reading no longer needs a replay labelled `miss` | 1 | 145 passed, 1 failed | `tests::the_replay_reading_takes_the_shipped_arm_and_a_replay_labelled_miss` |
| 29 | P | `replay_verdict` reads the source with the most misses, a control included | 1 | 145 passed, 1 failed | `tests::the_replay_verdict_reads_the_miss_replay_and_no_control` |
| 30 | P | the remedy reading no longer needs `--own-lines` | 1 | 145 passed, 1 failed | `tests::the_remedy_reading_parses_its_flags_and_selects_its_own_shapes` |
| 31 | P | `--sections` accepts an unknown shape id | 1 | 145 passed, 1 failed | `tests::the_sections_flag_is_a_mode_of_its_own` |
| 32 | P | the section rows name the first source second | 1 | 145 passed, 1 failed | `tests::the_sections_flag_is_a_mode_of_its_own` |
| 33 | P | `remedy_n` rounds 100 / m down | 1 | 145 passed, 1 failed | `tests::the_remedy_prompt_runs_at_the_n_that_expects_five_baseline_misses` |
| 34 | P | the known-positive holds at one miss over the allowance | 1 | 144 passed, 2 failed | `tests::the_known_positive_holds_at_the_allowance_plus_two_and_not_at_plus_one` · `tests::the_remedy_reading_prints_its_lines_in_order_and_exits_by_its_verdict` |
| 35 | P | the bar counts service misses only, so an `unparsed` generation passes | 1 | 144 passed, 2 failed | `tests::a_candidates_bar_needs_the_allowance_and_both_guard_minimums` · `tests::the_remedy_reading_prints_its_lines_in_order_and_exits_by_its_verdict` |
| 36 | P | the guard trips on an equal sibling count | 1 | 145 passed, 1 failed | `tests::the_remedy_guard_trips_only_below_shipped_on_a_guard_half` |
| 37 | P | the selection follows the order the candidates were listed in | 1 | 145 passed, 1 failed | `tests::the_remedy_selection_takes_the_first_candidate_in_order_that_is_selectable` |
| 38 | P | a candidate takes `shipped`'s counts on a replay it composes identically | 1 | 145 passed, 1 failed | `tests::a_candidate_composing_as_shipped_is_skipped_or_takes_shippeds_counts` (the plan half) |
| 39 | P | `remedy_counts` reads a reused shape from the candidate's own, absent, generations | 1 | 145 passed, 1 failed | `tests::a_candidate_composing_as_shipped_is_skipped_or_takes_shippeds_counts` (the counts half) |
| 40 | P | `fold_tally` counts `service_only` as a service miss | 1 | 145 passed, 1 failed | `tests::a_generation_folds_into_its_tally_by_its_identifies_label` |
| 41 | P | the derived shape is covered at two generations that are not `both` | 1 | 145 passed, 1 failed | `tests::the_derived_shape_reproduces_at_two_misses_and_is_covered_within_one_not_both` |
| 42 | P | a remedy verdict of NONE exits 0 | 1 | 144 passed, 2 failed | `tests::the_remedy_reading_prints_its_lines_in_order_and_exits_by_its_verdict` · `tests::the_derived_shape_is_recorded_and_moves_no_selection_and_no_exit` |
| 43 | P | the derived shape joins the bar | 1 | 145 passed, 1 failed | `tests::the_derived_shape_is_recorded_and_moves_no_selection_and_no_exit` |
| 44 | P | m is read off every arm's `replay:miss` rows | 1 | 145 passed, 1 failed | `tests::the_remedy_reading_takes_m_from_the_replay_readings_shipped_miss_rows` |

What the runs show about the pins:

- Each of the 41 new pins went red under at least one mutation, and 30 of the 44 mutations reddened exactly one pin.
- Under 17 and under 20, `a_replay_takes_its_own_lines_by_position_and_skips_the_two_edits_without_them` stayed green.
  It compares a replay's edit with the same shape's edit, so a wrong edit moves both sides alike: it holds that
  positions and incidents give the same result, and 23 is what it guards alone. What an edit does is held by
  `each_edit_equals_the_same_change_made_before_rendering`, red under 17, 18, 19, 20 and 21.
- Under 36 the reading-lines pin stayed green: none of its candidates ties `shipped` on a guard half. The tie is held
  by the guard pin alone.
- Under 43 (the shape the operator withdrew, inputs#I17) the selection moved from `CX` to `none` and only the pin
  written for that property went red.

## Step 11 (the replay verdict over every replay but the control, and the second count)

Two edits of the probe made at Step 11, before any capture entry fired, on the operator's decision that the replay
reads three prompts and records a second count (inputs#I18, inputs#I19): `replay_verdict` reads every replay but the
one labelled `control`, and `--count-naming ID` adds the closed per-generation label `names_other`. Four new pins.

Seven mutations, run 2026-10-07 from 13:10:56Z to 13:11:13Z by a script of the same five steps as above (unique
anchor, binary write, presence count, `cargo test -p pulse-app --example l4_decision_probe`, restore and re-hash).
Baseline before any mutation: 150 passed, 0 failed (146 plus the four new pins). Every run below compiled and exited
101. After the last restore the file hashes as before the first mutation (`l4_decision_probe.rs` `0c0aa2d1…93cd`;
`replay.rs` untouched, `6ef4aa7a…882f`), and `cargo fmt --check` exited 0.

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 45 | P | `replay_verdict` reads the control as a candidate | 1 | 148 passed, 2 failed | `tests::the_replay_verdict_reads_every_replay_but_the_control` · `tests::the_replay_verdict_reads_the_miss_replay_and_no_control` |
| 46 | P | `replay_verdict` reads the replay labelled `miss` alone | 1 | 149 passed, 1 failed | `tests::the_replay_verdict_reads_every_replay_but_the_control` |
| 47 | P | `names_other` matches the id as a substring, with no word boundary | 1 | 149 passed, 1 failed | `tests::the_second_count_reads_the_first_statement_for_the_other_id` |
| 48 | P | `names_other` reads the last hypothesis | 1 | 149 passed, 1 failed | `tests::the_second_count_reads_the_first_statement_for_the_other_id` |
| 49 | P | `fold_named` counts a `not_named` generation as named | 1 | 149 passed, 1 failed | `tests::the_second_count_folds_prints_and_rides_a_row_as_a_closed_label` |
| 50 | P | `--count-naming` accepts a value with a character outside an id | 1 | 149 passed, 1 failed | `tests::count_naming_parses_as_an_id_and_moves_no_reading` |
| 51 | P | `names_other` reads the id in its given spelling only | 1 | 149 passed, 1 failed | `tests::the_second_count_reads_the_first_statement_for_the_other_id` |

What the runs show about the pins:

- Each of the four new pins went red under at least one mutation. Under 46, the earlier verdict pin
  (`the_replay_verdict_reads_the_miss_replay_and_no_control`) stayed green: it holds `miss` beside a control only, so
  the reading of a second candidate is held by the new pin alone.
- That the second count moves no verdict is structural and has no mutation: no verdict function takes it, and the
  parse pin holds that the reading's flags read the same with and without it.

## Step 13 (the derived shape `S17`, and the label the remedy reading sizes itself from)

Three edits made after the replay reading and before the remedy pre-registration: the `Shape` field that counts
active incidents on `OVERALL:`, the derived shape `S17`, and `--remedy-read-as LABEL`, which takes m from the rows
the replay reading wrote under that label (`d2` reproduced, not `miss`). Three new pins, one extended pin
(`the_remedy_reading_takes_m_from_the_replay_readings_shipped_miss_rows`), and three pins that list the shape set
moved from sixteen shapes to seventeen.

`S1`-`S16` compose as before: the dry run of all 22 arms over `S1`-`S16` (352 lines) and the section rows of `S16`
against each of `S1`-`S15` were written by the binary before the edits and by the binary after them, and `cmp`
exited 0 on both pairs.

Six mutations, run 2026-10-07 from 13:29:14Z to 13:29:27Z by the same script and steps. Baseline before any
mutation: 153 passed, 0 failed. Every run below compiled and exited 101. After the last restore the file hashes as
before the first mutation (`l4_decision_probe.rs` `64504b9e…dc0f`; `replay.rs` untouched by a mutation,
`db364e49…11dd`), and `cargo fmt --check` exited 0. The gate block then read 19 green, 0 red on that tree (entry 5:
153 passed).

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 52 | P | `S17`'s first corpus line carries the third fingerprint | 1 | 152 passed, 1 failed | `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` |
| 53 | P | `S17` counts no active incident | 1 | 151 passed, 2 failed | `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` · `tests::the_derived_shape_composes_at_its_recorded_size_and_differs_from_s16_in_three_sections` |
| 54 | P | `S17`'s services rows stand with the triggering service first | 1 | 152 passed, 1 failed | `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` |
| 55 | P | `replay_reading_misses` reads `replay:miss` whatever the label | 1 | 152 passed, 1 failed | `tests::the_remedy_reading_takes_m_from_the_replay_readings_shipped_miss_rows` |
| 56 | P | the remedy prompt line never names the label the prompt was read under | 1 | 152 passed, 1 failed | `tests::the_remedy_reading_takes_m_from_the_replay_readings_shipped_miss_rows` |
| 57 | P | `--remedy-read-as` is accepted without `--remedy-from` | 1 | 152 passed, 1 failed | `tests::remedy_read_as_names_a_replay_label_and_needs_the_remedy_reading` |

What the runs show about the pins:

- Each of the three new pins and the extended one went red under at least one mutation.
- Under 54 the size-and-sections pin stayed green: swapping two rows of equal values keeps the size and still
  differs from `S16` in the same three sections. The rows' order is held by the first pin alone.

### After `S17`'s titles were reworded (the operator's ruling, inputs#I23)

The three titles were rewritten in the builder's own sentence frame before any reading of `S17`, the title constant
renamed, and the size pin moved from 7815 to 7824 bytes; nothing else in the pins changed. Three mutations, run
2026-10-07 from 13:33:28Z to 13:33:37Z by the same script and steps. Baseline: 153 passed, 0 failed. Every run
compiled and exited 101; after the last restore the file hashes `e9b409d7…b2a7` and `cargo fmt --check` exited 0.
The dry run of all 22 arms over `S1`-`S16` again compared equal to the one written before Step 13, and the gate
block read 19 green, 0 red (entry 5: 153 passed).

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 58 | P | `S17`'s first two titles change places | 1 | 152 passed, 1 failed | `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` |
| 59 | P | `S17`'s third title loses a word | 1 | 152 passed, 1 failed | `tests::the_derived_shape_composes_at_its_recorded_size_and_differs_from_s16_in_three_sections` |
| 60 | P | `S17` counts no active incident | 1 | 151 passed, 2 failed | both `S17` pins |

Under 59 the lines pin stayed green: it reads each title from the same constant the shape is built from, so a
title's wording is held by the size pin and its position by the lines pin.

## Step 15 (the selected remedy in the product: the triggering scope's own corpus lines first)

After the remedy reading selected `CO`, `select_corpus_matches` took the triggering cue's `scope_id` and orders
the kept matches with that scope's own first; `assemble` passes it. Five new in-crate pins (three in
`retrieval.rs`, two in `assembler.rs`), one new probe pin (`a_candidate_arm_is_its_edit_of_the_shipped_composition`)
and the identity the plan asks for, inside `each_edit_equals_the_same_change_made_before_rendering`: on every
shape the tree's block is the block the reorder edit produces from the newest-first block, and `CO` composes as
`shipped`.

Five probe pins held the order the readings measured under `shipped`, newest first throughout, and went red on the
product change, as they should: two that state a shape's lines in order (`S16`, `S17`), and three that test the
block edits against a shape's composition. They were moved to the product's order, not relaxed: the two line pins
state the own lines first, and the three edit pins now start from the newest-first block (the same incidents sorted
by age), so the reorder edit is still tested on a block that needs it.

Four mutations, run 2026-10-07 from 14:34:47Z to 14:35:21Z by a script of the same steps, each followed by
`cargo test -p triage --lib digest::` and `cargo test -p pulse-app --example l4_decision_probe`. Baseline: 54 passed
in the digest module, 154 in the probe. Every run compiled. After the last restore the files hash as before
(`retrieval.rs` `e15b4f7c…473f`, `assembler.rs` `c24d12f2…ded1`), `cargo fmt --check` exited 0, and the gate block
read 19 green, 0 red (entry 4: 497 passed; entry 5: 154; entry 17: 2796).

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 61 | `retrieval.rs` | the product change reverted: the kept matches stay newest first under a triggering scope | 1 | digest 51 passed, 3 failed · probe 150 passed, 4 failed | `retrieval::tests::select_corpus_matches_puts_the_triggering_scopes_own_first` · `retrieval::tests::select_corpus_matches_orders_after_the_cap_and_keeps_the_same_matches` · `assembler::tests::assemble_puts_the_triggering_scopes_own_corpus_lines_first` · probe: `replay::tests::each_edit_equals_the_same_change_made_before_rendering` · `replay::tests::cap_and_reorder_change_no_lines_text` · `tests::each_new_shape_holds_exactly_the_lines_its_row_names_in_order` · `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` |
| 62 | `retrieval.rs` | own is the triggering scope alone: a fingerprint match from another scope is not put first | 1 | digest 53 passed, 1 failed · probe 154 passed | `retrieval::tests::select_corpus_matches_puts_the_triggering_scopes_own_first` |
| 63 | `retrieval.rs` | the order is applied before the cap, so the cap keeps other matches | 1 | digest 53 passed, 1 failed · probe 154 passed | `retrieval::tests::select_corpus_matches_orders_after_the_cap_and_keeps_the_same_matches` |
| 64 | `assembler.rs` | `assemble` passes no triggering scope to the selection | 1 | digest 53 passed, 1 failed · probe 154 passed | `assembler::tests::assemble_puts_the_triggering_scopes_own_corpus_lines_first` |

What the runs show about the pins:

- 61 is the check the plan asks for: reverting the product change alone reddens the new pins, in the crate and in
  the probe.
- Under 62 and 63 the probe stayed green. No shape holds a fingerprint match from another scope behind a line that
  is not own, and no shape holds more than five incidents, so those two properties are held by the in-crate pins
  alone.
- The pins that hold the unchanged behaviour (`select_corpus_matches_keeps_newest_first_without_a_triggering_scope`,
  `assemble_keeps_corpus_lines_newest_first_without_a_cue_scope`) stayed green under all four, as they must: none
  of the mutations touches the no-cue path. They are the accept side of the pair.

## Step 15 again (the founder's remedy in the product: other scopes' corpus lines dropped under a cue's scope)

The section above records `CO` in the product. The founder then chose `CX` (inputs#I29), and a re-entry of
/implement replaced that change. The five in-crate pins of the reorder and its four checks (61 to 64) no longer
exist in the tree; what they recorded stands as a record of the tree the third reading ran on.

`select_corpus_matches` now narrows its scope arm to the triggering cue's `scope_id`: under one, a match scoped to
another active service is dropped before the cap, and a match the fingerprint arm keeps stays whatever its scope.
`assemble` passes the scope as before. Five in-crate pins replace the five (three in `retrieval.rs`, two in
`assembler.rs`). In the probe: the baseline arm `nb` (the block before the remedy), the candidate arms re-based on
it, the product path (`--product-path`), three new pins, the flag's cases added to a fourth, and sixteen moved ones.

**The sixteen probe pins that moved, and why.** With the selection changed and nothing else, sixteen pins went red
(138 passed, 16 failed), as they should: each held a block that `shipped` no longer composes. None was relaxed.

- Eleven state what a shape's block holds, at what size, in what order, or what an edit makes of it: they now read
  the block under `nb`, which is the composition the readings measured, and seven of them state beside it what
  `shipped` keeps of it. `S16`'s lines and `S17`'s are back in the order the readings measured (newest first); the
  previous run had moved both to the reorder's order.
- Two read `S4`'s framed corpus line (`nf_arm_strips_exactly_the_three_framing_lines`,
  `s4_renders_a_framed_corpus_match_beside_the_trigger`): `S4`'s one line is another service's, so `shipped` drops
  it. The first now reads `S8`, whose block the product keeps; the second reads `S4` under `nb`.
- Two read `S16` under `shipped` as a five-line block (`a_shapes_own_composition_replays_to_that_prompt_with_the_same_argv`,
  `s8_against_s16_differs_in_the_corpus_block_alone`): the first replays `S16` under `nb` and pins `shipped` at two
  lines; the second pins the section reader's row at `S16`'s two lines (4 L against 4 L, the bytes differing).
- One is the identity the plan asks for, inside `each_edit_equals_the_same_change_made_before_rendering`: it was
  "`CO` composes as `shipped`" and is now "`CX` composes as `shipped`, and the tree's block is the block the exclude
  edit produces from the baseline block", on all 17 shapes.

Twelve mutations, run 2026-10-07 from 18:29:17Z to 18:30:30Z by a script of the same steps (apply by a unique
anchor, confirm the mutated text is present once, run `cargo test -p triage --lib digest::` and
`cargo test -p pulse-app --example l4_decision_probe`, restore the file's bytes). Baseline, read at 18:29:13Z: 54
passed in the digest module, 157 in the probe. Every run compiled. After the last restore the four files hash as
before (`retrieval.rs` `16b6fd31…f6e4`, `assembler.rs` `52433447…c801`, the probe `04082c4e…a466`, its module
`5c145e27…5575`) and `cargo fmt --check` exited 0.

| # | File | Mutation | Present | Result | Red pins |
|---|---|---|---|---|---|
| 65 | `retrieval.rs` | the product change reverted: the scope arm is not narrowed under a triggering scope | 1 | digest 51 passed, 3 failed · probe 145 passed, 12 failed | `assembler::tests::assemble_drops_other_scopes_corpus_lines_under_a_cue_scope` · `retrieval::tests::select_corpus_matches_drops_other_scopes_matches_under_a_triggering_scope` · `retrieval::tests::select_corpus_matches_narrows_before_the_cap_and_only_removes` · probe: `replay::tests::a_shapes_own_composition_replays_to_that_prompt_with_the_same_argv` · `replay::tests::cap_and_reorder_change_no_lines_text` · `replay::tests::each_edit_equals_the_same_change_made_before_rendering` · `replay::tests::exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope` · `replay::tests::s8_against_s16_differs_in_the_corpus_block_alone` · `replay::tests::the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` · `tests::each_new_shape_holds_exactly_the_lines_its_row_names_in_order` · `tests::s16_holds_two_lines_for_the_triggering_service_and_three_for_the_sibling` · `tests::s1_to_s8_keep_their_corpus_blocks_through_the_baseline_selection` · `tests::the_baseline_arm_composes_s4_s7_and_s8_at_their_recorded_sizes` · `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` |
| 66 | `retrieval.rs` | the fingerprint arm narrowed too: under a triggering scope a fingerprint match from another scope is dropped | 1 | digest 52 passed, 2 failed · probe 142 passed, 15 failed | `retrieval::tests::select_corpus_matches_drops_other_scopes_matches_under_a_triggering_scope` · `retrieval::tests::select_corpus_matches_narrows_before_the_cap_and_only_removes` · probe: `replay::tests::a_digest_that_ends_without_a_newline_gains_none` · `replay::tests::a_replay_row_and_run_line_hold_labels_and_no_prompt_text` · `replay::tests::each_edit_equals_the_same_change_made_before_rendering` · `replay::tests::exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope` · `replay::tests::s7_against_s8_reads_the_block_absent_on_one_side` · `replay::tests::s8_against_s16_differs_in_the_corpus_block_alone` · `replay::tests::the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` · `replay::tests::the_restate_edit_adds_its_line_once_and_is_identity_after` · `tests::each_new_shape_holds_exactly_the_lines_its_row_names_in_order` · `tests::nf_arm_strips_exactly_the_three_framing_lines` · `tests::s1_to_s8_keep_their_corpus_blocks_through_the_baseline_selection` · `tests::s8_carries_two_sibling_corpus_lines_and_s7_none` · `tests::the_baseline_arm_composes_s4_s7_and_s8_at_their_recorded_sizes` · `tests::the_sections_flag_is_a_mode_of_its_own` |
| 67 | `retrieval.rs` | the cap runs before the narrowing | 1 | digest 53 passed, 1 failed · probe 157 passed | `retrieval::tests::select_corpus_matches_narrows_before_the_cap_and_only_removes` |
| 68 | `retrieval.rs` | the narrowing is not gated on the active scopes: a triggering scope outside them adds its matches | 1 | digest 53 passed, 1 failed · probe 157 passed | `retrieval::tests::select_corpus_matches_narrows_before_the_cap_and_only_removes` |
| 69 | `assembler.rs` | `assemble` passes no triggering scope to the selection | 1 | digest 53 passed, 1 failed · probe 157 passed | `assembler::tests::assemble_drops_other_scopes_corpus_lines_under_a_cue_scope` |
| 70 | the probe | `nb` renders the product's selection under the cue, not the baseline one | 1 | digest 54 passed · probe 141 passed, 16 failed | `replay::tests::a_replay_takes_its_own_lines_by_position_and_skips_the_two_edits_without_them` · `replay::tests::a_shapes_own_composition_replays_to_that_prompt_with_the_same_argv` · `replay::tests::cap_and_reorder_change_no_lines_text` · `replay::tests::each_edit_equals_the_same_change_made_before_rendering` · `replay::tests::exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope` · `replay::tests::the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` · `tests::each_new_shape_differs_from_s7_in_the_corpus_block_alone` · `tests::each_new_shape_holds_exactly_the_lines_its_row_names_in_order` · `tests::s16_holds_two_lines_for_the_triggering_service_and_three_for_the_sibling` · `tests::s1_to_s8_keep_their_corpus_blocks_through_the_baseline_selection` · `tests::s4_renders_a_framed_corpus_match_beside_the_trigger` · `tests::the_baseline_arm_composes_s4_s7_and_s8_at_their_recorded_sizes` · `tests::the_derived_shape_composes_at_its_recorded_size_and_differs_from_s16_in_three_sections` · `tests::the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident` · `tests::the_position_pairs_hold_the_same_lines_in_a_different_order` |
| 71 | the probe's module | the product path selects its remedied block with no triggering scope | 1 | digest 54 passed · probe 155 passed, 2 failed | `replay::tests::the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` |
| 72 | the probe's module | the product path reads no citable fingerprint: the window's set is empty | 1 | digest 54 passed · probe 156 passed, 1 failed | `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` |
| 73 | the probe's module | a line read back is not checked against the product's line format | 1 | digest 54 passed · probe 156 passed, 1 failed | `replay::tests::the_product_path_refuses_what_it_cannot_read_back` |
| 74 | the probe's module | the product path scopes every line to the triggering service | 1 | digest 54 passed · probe 155 passed, 2 failed | `replay::tests::the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` |
| 75 | the probe | `--product-path` is accepted without a dry run or own lines | 1 | digest 54 passed · probe 156 passed, 1 failed | `tests::the_replay_flags_parse_and_a_replay_brings_no_s_shape_of_its_own` |
| 76 | the probe | a candidate arm edits the `shipped` composition, not the baseline | 1 | digest 54 passed · probe 154 passed, 3 failed | `replay::tests::a_candidate_arm_is_its_edit_of_the_baseline_composition` · `replay::tests::cap_and_reorder_change_no_lines_text` · `replay::tests::the_product_path_reads_differs_where_the_selection_and_the_edit_part` |

What the runs show about the pins:

- 65 is the check the plan asks for: reverting the product change alone reddens the new pins, in the crate and in
  the probe.
- 66 is the boundary the scope document sets (the fingerprint arm stays fed): a remedy that also dropped a
  fingerprint match from another scope reddens two in-crate pins and fifteen probe pins.
- Under 67 and 68 the probe stayed green: no shape exercises either property, so both are held by the one
  in-crate pin alone.
- Under 69 the probe stayed green: it calls the selection itself and never `assemble`. That call is held by the
  one assembler pin.
- Under 72 the first product-path pin stayed green: it gives every own line the cue's scope, so the scope arm
  keeps them with or without a window. The fingerprint arm of the product path is held by the second pin, whose
  three cases each need it.
- The pins that hold the unchanged behaviour (`select_corpus_matches_keeps_every_active_scope_without_a_triggering_scope`,
  `assemble_keeps_every_scopes_corpus_lines_without_a_cue_scope`) stayed green under 65 to 69, as they must: none
  of those mutations touches the no-cue path. They are the accept side of the pair.
