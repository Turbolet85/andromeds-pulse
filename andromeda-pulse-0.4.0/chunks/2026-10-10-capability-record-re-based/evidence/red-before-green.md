# Red before green — the per-arm pins against a stub (plan step 0)

Read 2026-10-10T11:33:42Z, before any other edit of the chunk. The tree held two edits: the new file
`xtask/src/capability_record.rs` with its 36 pins and a stub, and the `mod capability_record;` line in
`xtask/src/main.rs`. `docs/capability-record.json` did not exist yet.

**The stub.** `evaluate` returned a reading with no finding and zero counts for every input, whatever the files
under the root held:

```rust
pub fn evaluate(_root: &Path) -> Verdict {
    Verdict::Read(Reading::default())
}
```

**The run.** `cargo nextest run -p xtask -E 'test(capability_record::)' --no-fail-fast`, exit 100.

```
Summary 36 tests run: 1 passed, 35 failed, 316 skipped
```

**The reading, pin by pin** (names under `capability_record::tests::`):

| pin | read |
|---|---|
| `a_valid_record_reads_clean` | PASS |
| `the_counts_on_the_line_are_read_from_the_record` | FAIL |
| `the_committed_record_reads_clean_over_the_committed_route` | FAIL |
| `an_absent_record_cannot_be_evaluated` | FAIL |
| `an_unreadable_record_cannot_be_evaluated` | FAIL |
| `a_record_that_is_not_json_cannot_be_evaluated` | FAIL |
| `a_record_with_no_capabilities_array_cannot_be_evaluated` | FAIL |
| `an_absent_route_cannot_be_evaluated` | FAIL |
| `a_missing_id_is_a_finding` | FAIL |
| `a_duplicated_id_is_a_finding` | FAIL |
| `an_id_outside_the_range_is_a_finding` | FAIL |
| `an_unknown_disposition_is_a_finding` | FAIL |
| `a_legend_that_is_not_the_three_closed_sets_is_a_finding` | FAIL |
| `a_claimed_entry_without_carried_by_is_a_finding` | FAIL |
| `a_carrying_requirement_outside_the_range_is_a_finding` | FAIL |
| `an_empty_carried_by_needs_a_note` | FAIL |
| `a_provisional_entry_needs_a_note` | FAIL |
| `a_claimed_entry_with_no_scenario_is_a_finding` | FAIL |
| `source_evidence_alone_is_no_proof` | FAIL |
| `an_unknown_scenario_kind_is_a_finding` | FAIL |
| `an_unknown_verification_mode_is_a_finding` | FAIL |
| `a_dangling_file_ref_is_a_finding` | FAIL |
| `a_missing_anchor_is_a_finding` | FAIL |
| `a_scenario_with_no_file_needs_a_note` | FAIL |
| `a_retired_entry_carries_no_proof_of_its_own` | FAIL |
| `a_retired_entry_with_no_surface_is_a_finding` | FAIL |
| `an_unknown_surface_is_a_finding` | FAIL |
| `a_retired_entry_with_no_removing_entry_is_a_finding` | FAIL |
| `a_removing_entry_the_route_does_not_carry_is_a_finding` | FAIL |
| `a_title_is_read_past_its_marker_stamp_and_never_from_a_note_line` | FAIL |
| `a_kept_half_owner_the_route_does_not_carry_is_a_finding` | FAIL |
| `a_retired_entry_with_no_guard_is_a_finding` | FAIL |
| `an_unknown_guard_state_is_a_finding` | FAIL |
| `a_guard_that_runs_names_what_runs` | FAIL |
| `a_guard_short_of_running_names_what_does_not_run` | FAIL |
| `a_record_with_no_claimed_id_is_a_finding` | FAIL |

**What the reading says.** Every pin of a cannot-evaluate arm and of a finding arm reads red against a verdict
that passes everything, so none of them can pass without the arm it names. The one green is the clean pin, which
a pass-everything stub satisfies by construction. Two further pins are red for a reason other than an arm: the
counts pin and the committed-record pin assert the counts on the verb's line (`82 ids: 1 claimed, 81 retired` and
`82 ids: 36 claimed, 46 retired`), and the stub counts nothing; the committed record was also absent at this
reading.

**Limits.** One run, on the dev host. The pins as run here are the pins as first written; a pin changed after this
reading is not covered by it and is named in the implement report.
