# Mutation checks - plan Steps 13 and 14

Applied together by a scratchpad script (backup, then mutate; restore byte-checked by
sha256 against the backup). The two mutations reach disjoint test targets, so each red
is attributable to its own mutation.

## Step 13 - the OVERALL line: Tier1-reads-nominal restored
Mutation: `crates/triage/src/digest/assembler.rs`, the cue branch of the state word
`"anomalous"` -> `"nominal"` (a cue-bearing, bypass-free digest reads nominal again, as at HEAD).

- `cargo test -p triage --lib overall_line` -> exit 101:
  - `overall_line_reads_anomalous_for_a_cue_bearing_tier1_digest` ... FAILED
  - `overall_line_reads_nominal_without_cue_or_incident` ... ok
  - `overall_line_reads_degraded_with_an_active_incident` ... ok
- `cargo test -p pulse-app --test unit_digest_runtime_scrub` -> exit 101:
  - `pii_scrub_fixture_overall_line_matches_the_render` ... FAILED (the strengthened fixture pin)
  - the 5 scrub pins ... ok

## Step 14 - the selected fix (A5) reverted
Selection: A5, per the pre-registered order after the founder declined A1 (2026-10-01).
Mutation: `crates/interpretation/src/schema.json` and `crates/interpretation/src/schema.rs`
replaced by their HEAD blobs (`git show HEAD:<path>`): `decision` / `severity` back before the
analysis fields, prompt versions back to `v2.2` / `v1.1-fallback` / `v1.1-reflection`.

- `cargo test -p interpretation --lib` -> exit 101, 112 passed / 1 failed:
  - `primary_prompt_schema_lists_decision_and_severity_after_the_analysis` ... FAILED
- `cargo test -p pulse-app --test unit_inference_runtime` -> exit 101, 11 passed / 3 failed:
  - `handle_digest_emits_prompt_version_v2_2_for_primary_tier` (renamed `..._v2_3_...` after this check) ... FAILED
  - `handle_digest_emits_prompt_version_v1_fallback_for_fallback_tier` ... FAILED
  - `handle_digest_selects_reflection_prompt_for_reflection_digest` ... FAILED

Restore: all three files byte-identical to the backup (sha256 prefixes 65ca9b9b7101 ·
3c1071c2c3f5 · 4a28c1b66b44). Before the mutation the same suites read 113/113 (interpretation),
3/3 overall_line, 6/6 scrub and 14/14 runtime; the post-restore green is the full gate run's
workspace nextest (implement P2).
