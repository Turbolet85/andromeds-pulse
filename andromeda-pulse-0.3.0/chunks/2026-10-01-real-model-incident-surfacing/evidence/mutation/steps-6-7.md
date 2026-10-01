# Mutation checks - plan Steps 6 and 7

Both run with `cargo test -p pulse-app --test <binary>` (parallel libtest, one process),
each mutation applied with the anchored Edit tool, then reverted; `grep -c MUTATION`
over both source files read 0 after the revert.

## Step 6 - the `interpretation.incident.skipped` leaf narrowed by one field
Mutation: `pulse-app/src/observability.rs`, the skip leaf
`["skip_reason", "decision", "severity", "digest_kind"]` -> `["skip_reason", "decision", "severity"]`.

`cargo test -p pulse-app --test unit_observability_allowlist_incident_skip` -> exit 101:
- `incident_skip_leaf_equals_the_emitted_field_set_in_both_directions` ... FAILED
  (panicked at `unit_observability_allowlist_incident_skip.rs:38`, the missing-field arm)
- `incident_skip_target_resolves_to_an_exact_leaf` ... ok
- `incident_skip_leaf_admits_no_banned_field` ... ok
- `incident_skip_has_no_interpretation_fallback` ... ok

The set pin went RED while the resolve pin stayed green, as the plan requires: a
resolver-only probe cannot see a partly-redacted leaf.

## Step 7 - two skip emits deleted
Mutation: `pulse-app/src/inference_runtime.rs`
- the routing-seam emit (`is_resolution_summary` on a non-resolution digest) replaced by a no-op;
- the no-cue emit removed, its `return` kept.

`cargo test -p pulse-app --test unit_incident_producer incident_skip` -> exit 101, 5 passed / 2 failed:
- `incident_skip_records_a_cue_less_digest` ... FAILED (`unit_incident_producer.rs:1050`)
- `incident_skip_records_a_model_resolution_summary_on_a_storm_digest` ... FAILED (`unit_incident_producer.rs:1245`)
- `incident_skip_records_a_dismiss_decision` ... ok
- `incident_skip_records_a_none_severity` ... ok
- `incident_skip_records_a_model_resolution_summary_at_the_predicate` ... ok
- `incident_skip_is_absent_when_an_incident_is_created` ... ok
- `incident_skip_is_absent_for_a_resolution_summary_digest` ... ok

Each deleted emit reddened exactly its own reason's pin and no other. The five greens
ran under parallel libtest beside the two reds, which also shows the per-thread capture
filter keeps sibling tests' skip records out of each assertion.
