
## 2026-10-10-capability-record-re-based — pending trigger for the capability verb's glue
**Section:** §1 Test Scope Summary → Pending coverage triggers (documented gaps)
**Change:** A new row, `verify-capability-matrix-verb-glue-coverage`. The capability record's verdict is pinned
in-crate by 36 co-located pins in `xtask/src/capability_record.rs` (one per arm over constructed records under
`tempfile::TempDir`, plus one over the committed record and route). The verb function `verify_capability_matrix`
in `xtask/src/main.rs` has no pin: the JSON event line (target `xtask.verify_capability_matrix`), the report twin
`target/capability-matrix/report.json` (`state` · `capability_count` · `claimed_count` · `retired_count` ·
`violation_count` · `reason` · `violations` · `generated_at`) and the process exit taken from
`Verdict::exit_code`. The verb ran on one arm only, clean and exit 0 (the dev host; the `lint / test` job log of
`ci#38049792921`); its exit 1 and exit 2 never ran through the verb, which takes no path. Owed: a pin over the
twin's member set and the verb's exit per state. Was: no row.
**Why:** The verdict is tested at the unit tier and the function that prints it, writes the twin and returns the
exit is not; the gap is recorded so that it is not read as covered by the 36 pins.
**Ref:** .andromeda/runs/2026-10-10T12-20-47Z-wrap/
