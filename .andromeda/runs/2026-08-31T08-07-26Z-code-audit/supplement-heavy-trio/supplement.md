# Mutation supplement — heavy trio · andromeda-pulse · Epoch 4 — Polish & ship: verification
supplement of **audit record #1** · HEAD `83d4060` · same boundary, source tree verified clean · 2026-08-31

> Completes the three units record #1 declares as `skips[declined]` (buffer / triage / pulse-app — 375 / 908 / 903 mutants, 40–95 min each). **The ledger is untouched by design**: record #1 already carries the declaration, and the next boundary cites this file as the first-absolute reading for these units. Recipe pinned to collectors.md C1 — `--test-tool=nextest` pass-through, jobs=1, scratch copy, 100-min operator ceiling per unit. Obligation-free: nothing here is applied, and no source was edited.

## Scores

| unit | mutants tested | caught | missed | unviable | timeout | score | duration | status |
|---|---|---|---|---|---|---|---|---|
| buffer | 375 of 375 | 233 | 105 | 21 | 16 | **68.9%** | 4163s (69m) | complete |
| triage | 672 of 908 | 389 | 177 | 105 | 1 | **68.7%** | 6000s (100m) | **budget-exhausted** (672/908 mutants (74.0%)) |
| pulse-app | **0** of 903 | — | — | — | — | **no score** | 232s | **cannot-evaluate** (baseline build failed) |

**Weighted across the trio:** 622 caught / 282 missed = **68.8%** (formula `caught/(caught+missed)`; a partial unit's contribution is its tested subset only).

**Against record #1's 7 light units** (64.2% weighted, line coverage 84.35%): the trio adds the three largest units in the workspace — buffer (the DuckDB ring buffer), triage (the L1 distillation pipeline) and pulse-app (the binary + its routers). Read the two together as the epoch's mutation picture; neither half is the whole.

## pulse-app — cannot-evaluate (no score exists for this unit)

**What happened:** the unmutated baseline failed to build after 232s, so **0 of 903 declared mutants** were built or run. `outcomes.json` carries `total_mutants: 0` and a single `Baseline / Failure` outcome.

**Mechanism (measured, not inferred):** cargo-mutants' baseline build is PACKAGE-SCOPED (`cargo nextest run --no-run --package=pulse-app@0.1.0`) and failed in a PRISTINE scratch tree with `crate {libduckdb_sys, wasmtime, cranelift_codegen} required to be available in rlib format, but was not found in this form` — across the bin target and ~25 integration test targets. No mutant was ever built or run.

**Why this is not a stale-target artifact:** cargo-mutants builds in a pristine scratch copy with its own fresh target dir (`%TEMP%\cargo-mutants-andromeda-pulse-*.tmp`), so there is no inherited `target/.fingerprint` state — and the same source at this same sha built and ran 2364/2364 tests green under workspace-scoped `cargo llvm-cov nextest --workspace` earlier in this session. The failure tracks the **package-scoped** build shape, not the tree's condition. `.claude/rules/testing.md` documents this error string but attributes it to "environmental (target/.fingerprint inconsistency between feature combinations)" with a clean-and-retry / `--bin`-scoping remedy — that characterization does not explain this occurrence, and neither remedy is reachable from inside cargo-mutants' baseline invocation. Recorded as an observation about the documented root cause; no artifact was edited.

**Options for the next boundary** (costed, operator's call — none taken here):
- `--test-workspace=true`: replaces the package-scoped test command with the workspace-scoped one that is known to work. Cost: every mutant re-runs the full 2364-test suite (~19s) on top of its rebuild — 903 mutants ≈ 7–15 h, far past any per-unit ceiling. A 100-min slice would cover only the first ~5% of mutants in cargo-mutants' deterministic file order, which is a biased sample, not a unit score.
- Scope pulse-app by file (`--file`) into several bounded runs across boundaries, accepting per-file rather than per-unit scores.
- Leave pulse-app unmeasured and say so: its 903 mutants stay a declared gap in the record, which is the status quo record #1 already carries.

## Survivors — complete list (282 rows)

_Partial for: triage — the survivor list covers the tested subset, so ABSENCE of a site here is not evidence it is killed._

**triage partial boundary** — cargo-mutants proceeds in deterministic file order, so the untested remainder is a contiguous alphabetical tail, NOT a random sample. 25 of 36 files reached; **11 files never reached** (231 mutants declared, 0 tested):

| file never reached | mutants declared |
|---|---|
| `crates/triage/src/incident/registry.rs` | 46 |
| `crates/triage/src/incident/state_machine.rs` | 10 |
| `crates/triage/src/lifecycle/broadcast.rs` | 10 |
| `crates/triage/src/lifecycle/persistence.rs` | 10 |
| `crates/triage/src/lifecycle/registry.rs` | 38 |
| `crates/triage/src/lifecycle/state_machine.rs` | 2 |
| `crates/triage/src/pattern/broadcast.rs` | 10 |
| `crates/triage/src/pattern/detector.rs` | 26 |
| `crates/triage/src/pattern/persistence.rs` | 6 |
| `crates/triage/src/pattern/storm.rs` | 42 |
| `crates/triage/src/pattern/suppression.rs` | 31 |

Partially reached: `crates/triage/src/incident/persistence.rs` 14/19

| unit | site | mutation |
|---|---|---|
| buffer | `crates/buffer/src/appender.rs:50` | replace // with && in build_spans_record_batch |
| buffer | `crates/buffer/src/appender.rs:58` | replace / with % in build_spans_record_batch |
| buffer | `crates/buffer/src/appender.rs:265` | replace / with % in build_logs_record_batch |
| buffer | `crates/buffer/src/appender.rs:410` | replace hex_lower -> String with String::new() |
| buffer | `crates/buffer/src/appender.rs:410` | replace hex_lower -> String with "xyzzy".into() |
| buffer | `crates/buffer/src/appender.rs:448` | replace += with *= in encode_labels |
| buffer | `crates/buffer/src/appender.rs:496` | replace // with && in build_span_events_record_batch |
| buffer | `crates/buffer/src/appender.rs:530` | replace / with % in build_span_events_record_batch |
| buffer | `crates/buffer/src/appender.rs:891` | replace / with % in push_metric_row |
| buffer | `crates/buffer/src/appender.rs:900` | replace hash_resource -> Vec<u8> with vec![] |
| buffer | `crates/buffer/src/appender.rs:900` | replace hash_resource -> Vec<u8> with vec![0] |
| buffer | `crates/buffer/src/appender.rs:900` | replace hash_resource -> Vec<u8> with vec![1] |
| buffer | `crates/buffer/src/appender.rs:914` | replace hash_resource_logs -> Vec<u8> with vec![] |
| buffer | `crates/buffer/src/appender.rs:914` | replace hash_resource_logs -> Vec<u8> with vec![0] |
| buffer | `crates/buffer/src/appender.rs:914` | replace hash_resource_logs -> Vec<u8> with vec![1] |
| buffer | `crates/buffer/src/appender.rs:928` | replace short_err -> String with String::new() |
| buffer | `crates/buffer/src/appender.rs:928` | replace short_err -> String with "xyzzy".into() |
| buffer | `crates/buffer/src/broadcast.rs:51` | replace > with >= in encode_with_cap |
| buffer | `crates/buffer/src/consumer.rs:237` | replace observe_spans_for_baseline with () |
| buffer | `crates/buffer/src/consumer.rs:248` | replace // with && in observe_spans_for_baseline |
| buffer | `crates/buffer/src/consumer.rs:255` | replace / with % in observe_spans_for_baseline |
| buffer | `crates/buffer/src/consumer.rs:255` | replace / with * in observe_spans_for_baseline |
| buffer | `crates/buffer/src/drain.rs:491` | replace > with == in DrainMiner::assign_at_inner |
| buffer | `crates/buffer/src/drain.rs:491` | replace > with < in DrainMiner::assign_at_inner |
| buffer | `crates/buffer/src/drain.rs:491` | replace > with >= in DrainMiner::assign_at_inner |
| buffer | `crates/buffer/src/drain.rs:535` | replace > with == in DrainMiner::assign_at_inner |
| buffer | `crates/buffer/src/drain.rs:535` | replace > with >= in DrainMiner::assign_at_inner |
| buffer | `crates/buffer/src/drain.rs:604` | replace DrainMiner::take_lru_evictions_since_tick -> u64 with 0 |
| buffer | `crates/buffer/src/drain.rs:604` | replace DrainMiner::take_lru_evictions_since_tick -> u64 with 1 |
| buffer | `crates/buffer/src/drain.rs:670` | replace short_drain_err -> String with String::new() |
| buffer | `crates/buffer/src/drain.rs:670` | replace short_drain_err -> String with "xyzzy".into() |
| buffer | `crates/buffer/src/drain.rs:671` | replace > with == in short_drain_err |
| buffer | `crates/buffer/src/drain.rs:671` | replace > with < in short_drain_err |
| buffer | `crates/buffer/src/drain.rs:671` | replace > with >= in short_drain_err |
| buffer | `crates/buffer/src/drain.rs:705` | replace == with != in walk_down |
| buffer | `crates/buffer/src/drain.rs:738` | replace // with && in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace > with == in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace > with < in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace > with >= in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace && with // in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace == with != in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace < with == in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace < with > in find_best_match |
| buffer | `crates/buffer/src/drain.rs:738` | replace < with <= in find_best_match |
| buffer | `crates/buffer/src/drain.rs:753` | replace // with && in position_similarity |
| buffer | `crates/buffer/src/drain.rs:791` | replace / with % in drift_for |
| buffer | `crates/buffer/src/drain.rs:791` | replace / with * in drift_for |
| buffer | `crates/buffer/src/drain.rs:792` | replace > with >= in drift_for |
| buffer | `crates/buffer/src/drain.rs:801` | replace serialize_tree -> SerializableTree with Default::default() |
| buffer | `crates/buffer/src/drain.rs:811` | replace serialize_node -> SerializableNode with Default::default() |
| buffer | `crates/buffer/src/drain.rs:834` | replace deserialize_node -> TreeNode with Default::default() |
| buffer | `crates/buffer/src/drain.rs:849` | replace remove_id_from_tree with () |
| buffer | `crates/buffer/src/drain.rs:855` | replace remove_id_from_node with () |
| buffer | `crates/buffer/src/drain.rs:855` | replace != with == in remove_id_from_node |
| buffer | `crates/buffer/src/fingerprint.rs:174` | replace < with <= in is_hex_address_start |
| buffer | `crates/buffer/src/fingerprint.rs:176` | replace == with != in is_hex_address_start |
| buffer | `crates/buffer/src/fingerprint.rs:176` | replace + with - in is_hex_address_start |
| buffer | `crates/buffer/src/fingerprint.rs:176` | replace + with * in is_hex_address_start |
| buffer | `crates/buffer/src/fingerprint.rs:181` | replace += with *= in skip_hex_address |
| buffer | `crates/buffer/src/fingerprint.rs:202` | replace < with <= in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:202` | replace < with <= in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:202` | replace + with * in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:202` | replace + with * in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:208` | replace && with // in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:205` | replace < with <= in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:205` | replace + with * in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:208` | replace == with != in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:208` | replace + with - in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:208` | replace + with * in is_absolute_path_start |
| buffer | `crates/buffer/src/fingerprint.rs:219` | replace && with // in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:218` | replace && with // in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:217` | replace && with // in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:216` | replace < with <= in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:216` | replace + with * in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:219` | replace == with != in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:219` | replace + with - in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:219` | replace + with * in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:223` | replace += with *= in skip_absolute_path |
| buffer | `crates/buffer/src/fingerprint.rs:236` | replace is_line_number_suffix -> bool with true |
| buffer | `crates/buffer/src/fingerprint.rs:236` | replace && with // in is_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:236` | replace < with <= in is_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:240` | replace += with *= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:242` | replace += with -= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace && with // in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace < with > in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace == with != in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace < with == in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace < with > in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace < with <= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace + with - in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace + with * in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace + with - in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:244` | replace + with * in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:245` | replace += with -= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:245` | replace += with *= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:246` | replace < with <= in skip_line_number_suffix |
| buffer | `crates/buffer/src/fingerprint.rs:247` | replace += with -= in skip_line_number_suffix |
| buffer | `crates/buffer/src/retention.rs:50` | replace run_retention with () |
| buffer | `crates/buffer/src/retention.rs:50` | replace / with % in run_retention |
| buffer | `crates/buffer/src/retention.rs:50` | replace / with * in run_retention |
| buffer | `crates/buffer/src/retention.rs:202` | replace short_err -> String with String::new() |
| buffer | `crates/buffer/src/retention.rs:202` | replace short_err -> String with "xyzzy".into() |
| buffer | `crates/buffer/src/schema.rs:174` | replace sanitize_duckdb_error -> String with String::new() |
| buffer | `crates/buffer/src/schema.rs:174` | replace sanitize_duckdb_error -> String with "xyzzy".into() |
| buffer | `crates/buffer/src/state.rs:130` | replace > with >= in BufferState::record_redactions |
| triage | `crates/triage/src/baseline/mod.rs:227` | replace BaselineState::schema_version -> u32 with 1 |
| triage | `crates/triage/src/baseline/mod.rs:246` | replace BaselineState::operation_count -> usize with 0 |
| triage | `crates/triage/src/baseline/mod.rs:334` | replace BaselineState::swap_tdigest_pairs_on_tick -> usize with 0 |
| triage | `crates/triage/src/baseline/mod.rs:334` | replace BaselineState::swap_tdigest_pairs_on_tick -> usize with 1 |
| triage | `crates/triage/src/baseline/mod.rs:340` | replace += with *= in BaselineState::swap_tdigest_pairs_on_tick |
| triage | `crates/triage/src/baseline/mod.rs:357` | replace += with *= in BaselineState::swap_short_tdigest_pairs_on_tick |
| triage | `crates/triage/src/baseline/mod.rs:366` | replace BaselineState::total_centroid_count -> usize with 0 |
| triage | `crates/triage/src/baseline/mod.rs:366` | replace BaselineState::total_centroid_count -> usize with 1 |
| triage | `crates/triage/src/baseline/mod.rs:415` | replace BaselineState::scrubbed_clone -> Self with Default::default() |
| triage | `crates/triage/src/baseline/mod.rs:599` | replace > with >= in run_persist_cycle |
| triage | `crates/triage/src/baseline/mod.rs:608` | replace > with >= in run_persist_cycle |
| triage | `crates/triage/src/baseline/mod.rs:682` | replace run_persist_loop with () |
| triage | `crates/triage/src/baseline/mod.rs:749` | replace current_unix_nanos -> i64 with 1 |
| triage | `crates/triage/src/lifecycle/mod.rs:95` | replace start_lifecycle_heartbeat with () |
| triage | `crates/triage/src/lifecycle/mod.rs:139` | replace reevaluate_now -> usize with 0 |
| triage | `crates/triage/src/lifecycle/mod.rs:139` | replace reevaluate_now -> usize with 1 |
| triage | `crates/triage/src/lifecycle/mod.rs:208` | replace += with *= in emit_tick_observability |
| triage | `crates/triage/src/lifecycle/mod.rs:238` | replace current_time_unix_nano -> i64 with 0 |
| triage | `crates/triage/src/lifecycle/mod.rs:238` | replace current_time_unix_nano -> i64 with 1 |
| triage | `crates/triage/src/lifecycle/mod.rs:238` | replace current_time_unix_nano -> i64 with -1 |
| triage | `crates/triage/src/baseline/activity_floor.rs:124` | replace >= with < in ActivityFloor::observe |
| triage | `crates/triage/src/baseline/activity_floor.rs:154` | replace == with != in ActivityFloor::p95_historical_quiet_duration_seconds |
| triage | `crates/triage/src/baseline/activity_floor.rs:212` | replace ActivityFloor::flush_buffer with () |
| triage | `crates/triage/src/baseline/corpus.rs:20` | replace * with + |
| triage | `crates/triage/src/baseline/corpus.rs:20` | replace * with / |
| triage | `crates/triage/src/baseline/corpus.rs:76` | replace > with == in bootstrap_from_persistence |
| triage | `crates/triage/src/baseline/corpus.rs:76` | replace > with >= in bootstrap_from_persistence |
| triage | `crates/triage/src/baseline/ewma.rs:54` | replace EwmaTracker::alpha -> f64 with 0.0 |
| triage | `crates/triage/src/baseline/ewma.rs:54` | replace EwmaTracker::alpha -> f64 with 1.0 |
| triage | `crates/triage/src/baseline/ewma.rs:54` | replace EwmaTracker::alpha -> f64 with -1.0 |
| triage | `crates/triage/src/baseline/rolling_window.rs:39` | replace RollingWindow<T>::capacity -> usize with 0 |
| triage | `crates/triage/src/baseline/rolling_window.rs:39` | replace RollingWindow<T>::capacity -> usize with 1 |
| triage | `crates/triage/src/baseline/sql.rs:62` | replace cutoff_ns -> i64 with 0 |
| triage | `crates/triage/src/baseline/sql.rs:62` | replace cutoff_ns -> i64 with 1 |
| triage | `crates/triage/src/baseline/sql.rs:62` | replace cutoff_ns -> i64 with -1 |
| triage | `crates/triage/src/baseline/sql.rs:676` | replace run_q7_fallback_blocking -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> with Ok(vec![]) |
| triage | `crates/triage/src/baseline/sql.rs:713` | replace emit_query_telemetry with () |
| triage | `crates/triage/src/baseline/sql.rs:714` | delete match arm "q1" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:729` | delete match arm "q2" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:744` | delete match arm "q3" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:759` | delete match arm "q4" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:774` | delete match arm "q5" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:789` | delete match arm "q6" in emit_query_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:812` | replace emit_q7_telemetry with () |
| triage | `crates/triage/src/baseline/sql.rs:813` | delete match arm "q7" in emit_q7_telemetry |
| triage | `crates/triage/src/baseline/sql.rs:828` | delete match arm "q7-fallback" in emit_q7_telemetry |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:44` | replace >= with < in TDigestPair::insert |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:59` | replace // with && in TDigestPair::swap_on_tick |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:98` | replace && with // in TDigestPair::percentile_current_only |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:98` | replace == with != in TDigestPair::percentile_current_only |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:121` | replace TDigestPair::centroid_count -> usize with 0 |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:121` | replace TDigestPair::centroid_count -> usize with 1 |
| triage | `crates/triage/src/baseline/tdigest_pair.rs:121` | replace + with * in TDigestPair::centroid_count |
| triage | `crates/triage/src/cadence/broadcast.rs:103` | replace DigestTriggerBroadcast::subscriber_count -> usize with 0 |
| triage | `crates/triage/src/cadence/coordinator.rs:238` | replace cycle_window_for -> Duration with Default::default() |
| triage | `crates/triage/src/cadence/coordinator.rs:469` | replace current_unix_nanos -> i64 with 0 |
| triage | `crates/triage/src/cadence/coordinator.rs:469` | replace current_unix_nanos -> i64 with 1 |
| triage | `crates/triage/src/cadence/coordinator.rs:469` | replace current_unix_nanos -> i64 with -1 |
| triage | `crates/triage/src/cue/broadcast.rs:78` | replace CadenceTriggerChannel::subscriber_count -> usize with 0 |
| triage | `crates/triage/src/cue/classify.rs:45` | replace > with >= in dual_condition_bypass |
| triage | `crates/triage/src/cue/classify.rs:49` | replace > with >= in dual_condition_bypass |
| triage | `crates/triage/src/cue/classify.rs:50` | replace > with >= in dual_condition_bypass |
| triage | `crates/triage/src/cue/emitter.rs:198` | replace += with *= in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:199` | replace += with *= in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:219` | replace + with - in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:219` | replace + with * in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:219` | replace + with - in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:219` | replace + with * in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:231` | replace && with // in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:231` | delete ! in run_one_emit_cycle |
| triage | `crates/triage/src/cue/emitter.rs:357` | replace cue_scope_label -> &'static str with "" |
| triage | `crates/triage/src/cue/emitter.rs:357` | replace cue_scope_label -> &'static str with "xyzzy" |
| triage | `crates/triage/src/cue/emitter.rs:445` | replace drain_restart_events with () |
| triage | `crates/triage/src/cue/emitter.rs:466` | replace current_unix_nanos -> i64 with 0 |
| triage | `crates/triage/src/cue/emitter.rs:466` | replace current_unix_nanos -> i64 with 1 |
| triage | `crates/triage/src/cue/emitter.rs:466` | replace current_unix_nanos -> i64 with -1 |
| triage | `crates/triage/src/cue/evaluate.rs:38` | replace < with == in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:38` | replace < with <= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:41` | replace // with && in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:46` | replace < with <= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:49` | replace > with >= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:54` | replace / with * in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:90` | replace < with == in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:90` | replace < with <= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:100` | replace match guard l.is_finite() with true in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:100` | replace match guard l.is_finite() with false in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:103` | replace * with + in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:104` | replace < with <= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:107` | replace > with >= in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:108` | replace / with * in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:112` | replace / with % in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:112` | replace / with * in evaluate_thresholds |
| triage | `crates/triage/src/cue/evaluate.rs:181` | replace == with != in evaluate_service_went_silent |
| triage | `crates/triage/src/cue/evaluate.rs:184` | replace / with * in evaluate_service_went_silent |
| triage | `crates/triage/src/cue/thresholds.rs:46` | replace != with == in resolve_bootstrap_window_seconds |
| triage | `crates/triage/src/cue/thresholds.rs:53` | replace warn_bootstrap_window with () |
| triage | `crates/triage/src/cue/thresholds.rs:252` | replace // with && in Thresholds::validate |
| triage | `crates/triage/src/cue/thresholds.rs:253` | replace > with >= in Thresholds::validate |
| triage | `crates/triage/src/cue/thresholds.rs:281` | replace // with && in Thresholds::validate |
| triage | `crates/triage/src/cue/thresholds.rs:282` | replace > with >= in Thresholds::validate |
| triage | `crates/triage/src/digest/assembler.rs:132` | replace <impl std::fmt::Debug for Assembler>::fmt -> std::fmt::Result with Ok(Default::default()) |
| triage | `crates/triage/src/digest/assembler.rs:191` | replace Assembler::count_tokens -> usize with 0 |
| triage | `crates/triage/src/digest/assembler.rs:191` | replace Assembler::count_tokens -> usize with 1 |
| triage | `crates/triage/src/digest/assembler.rs:223` | replace && with // in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:223` | replace != with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:359` | replace > with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:359` | replace > with >= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:376` | replace && with // in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:376` | replace > with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:376` | replace > with < in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:376` | replace > with >= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:376` | delete ! in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:392` | replace && with // in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:392` | replace > with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:392` | replace > with < in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:392` | replace > with >= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:392` | delete ! in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:420` | replace && with // in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:420` | replace > with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:420` | replace > with < in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:420` | replace > with >= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:430` | replace < with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:430` | replace < with > in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:430` | replace < with <= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:441` | replace > with == in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:441` | replace > with < in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:441` | replace > with >= in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:461` | replace - with + in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:461` | replace - with / in <impl DigestAssembler for Assembler>::assemble |
| triage | `crates/triage/src/digest/assembler.rs:556` | replace cue_kind_label -> &'static str with "" |
| triage | `crates/triage/src/digest/assembler.rs:556` | replace cue_kind_label -> &'static str with "xyzzy" |
| triage | `crates/triage/src/digest/assembler.rs:567` | replace priority_tier_label -> &'static str with "" |
| triage | `crates/triage/src/digest/assembler.rs:567` | replace priority_tier_label -> &'static str with "xyzzy" |
| triage | `crates/triage/src/digest/assembler.rs:580` | replace severity_at_least_suggested -> bool with true |
| triage | `crates/triage/src/digest/assembler.rs:580` | replace severity_at_least_suggested -> bool with false |
| triage | `crates/triage/src/digest/assembler.rs:593` | replace / with % in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:593` | replace / with * in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:596` | replace / with % in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:596` | replace / with * in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:597` | replace / with % in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:597` | replace / with * in compose_services_from_q1 |
| triage | `crates/triage/src/digest/assembler.rs:652` | delete ! in render_payload |
| triage | `crates/triage/src/digest/assembler.rs:664` | delete ! in render_payload |
| triage | `crates/triage/src/digest/assembler.rs:685` | replace lowest_priority_cue_index -> usize with 0 |
| triage | `crates/triage/src/digest/assembler.rs:685` | replace lowest_priority_cue_index -> usize with 1 |
| triage | `crates/triage/src/digest/assembler.rs:693` | replace priority_tier_rank -> u8 with 0 |
| triage | `crates/triage/src/digest/assembler.rs:693` | replace priority_tier_rank -> u8 with 1 |
| triage | `crates/triage/src/digest/assembler.rs:701` | replace lowest_anomaly_service_index -> usize with 0 |
| triage | `crates/triage/src/digest/assembler.rs:701` | replace lowest_anomaly_service_index -> usize with 1 |
| triage | `crates/triage/src/digest/assembler.rs:704` | replace + with - in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:704` | replace + with * in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:704` | replace / with % in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:704` | replace / with * in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:705` | replace + with - in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:705` | replace + with * in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:705` | replace / with % in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/assembler.rs:705` | replace / with * in lowest_anomaly_service_index |
| triage | `crates/triage/src/digest/damper.rs:52` | replace generate_reason_label -> &'static str with "" |
| triage | `crates/triage/src/digest/damper.rs:52` | replace generate_reason_label -> &'static str with "xyzzy" |
| triage | `crates/triage/src/digest/damper.rs:115` | replace GenerationDamper::generations_run_total -> u64 with 1 |
| triage | `crates/triage/src/digest/damper.rs:199` | delete ! in GenerationDamper::record_generated |
| triage | `crates/triage/src/digest/damper.rs:213` | replace < with <= in GenerationDamper::evict_stale |
| triage | `crates/triage/src/digest/queue.rs:150` | replace LwwQueue::active_incident_depth_for -> usize with 0 |
| triage | `crates/triage/src/digest/queue.rs:150` | replace LwwQueue::active_incident_depth_for -> usize with 1 |
| triage | `crates/triage/src/digest/queue.rs:163` | replace LwwQueue::default_slot_occupied -> bool with true |
| triage | `crates/triage/src/digest/queue.rs:171` | replace kind_label -> String with String::new() |
| triage | `crates/triage/src/digest/queue.rs:171` | replace kind_label -> String with "xyzzy".into() |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with + |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with / |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with + |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with / |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with + |
| triage | `crates/triage/src/digest/retrieval.rs:27` | replace * with / |
| triage | `crates/triage/src/incident/persistence.rs:215` | replace += with *= in run_incident_persist_cycle |
| triage | `crates/triage/src/incident/persistence.rs:220` | replace += with *= in run_incident_persist_cycle |
| triage | `crates/triage/src/incident/persistence.rs:221` | replace += with -= in run_incident_persist_cycle |
| triage | `crates/triage/src/incident/persistence.rs:221` | replace += with *= in run_incident_persist_cycle |

## Non-terminating mutants (17) — timeouts, neither caught nor survived

_Given a slot deliberately: the collector table records that a timeout with no home stays invisible across boundaries. These are excluded from `caught/(caught+missed)` — a hang is evidence the mutation breaks termination, not evidence a test asserts the behaviour._

| unit | site |
|---|---|
| buffer | `crates/buffer/src/fingerprint.rs:164:11: replace += with *= in normalize_frame` |
| buffer | `crates/buffer/src/fingerprint.rs:181:5: replace skip_hex_address -> usize with 0` |
| buffer | `crates/buffer/src/fingerprint.rs:181:5: replace skip_hex_address -> usize with 1` |
| buffer | `crates/buffer/src/fingerprint.rs:181:7: replace += with -= in skip_hex_address` |
| buffer | `crates/buffer/src/fingerprint.rs:183:11: replace += with *= in skip_hex_address` |
| buffer | `crates/buffer/src/fingerprint.rs:216:5: replace skip_absolute_path -> usize with 0` |
| buffer | `crates/buffer/src/fingerprint.rs:216:5: replace skip_absolute_path -> usize with 1` |
| buffer | `crates/buffer/src/fingerprint.rs:221:11: replace += with -= in skip_absolute_path` |
| buffer | `crates/buffer/src/fingerprint.rs:223:11: replace += with -= in skip_absolute_path` |
| buffer | `crates/buffer/src/fingerprint.rs:226:11: replace += with -= in skip_absolute_path` |
| buffer | `crates/buffer/src/fingerprint.rs:226:11: replace += with *= in skip_absolute_path` |
| buffer | `crates/buffer/src/fingerprint.rs:240:5: replace skip_line_number_suffix -> usize with 0` |
| buffer | `crates/buffer/src/fingerprint.rs:240:5: replace skip_line_number_suffix -> usize with 1` |
| buffer | `crates/buffer/src/fingerprint.rs:240:7: replace += with -= in skip_line_number_suffix` |
| buffer | `crates/buffer/src/fingerprint.rs:242:11: replace += with *= in skip_line_number_suffix` |
| buffer | `crates/buffer/src/fingerprint.rs:247:15: replace += with *= in skip_line_number_suffix` |
| triage | `crates/triage/src/baseline/rolling_window.rs:28:32: replace >= with < in RollingWindow<T>::push` |

## Notes
- buffer: completed with rc=3 (cargo-mutants signals timeouts, not failure) — 16 NON-TERMINATING mutants, excluded from the score by the formula; listed separately
- triage: budget-exhausted at the 100-min ceiling — PARTIAL: 672/908 mutants tested; score 68.7% is over the tested subset only, NOT the unit
- pulse-app: CANNOT-EVALUATE under the pinned recipe — baseline build failed in 232s; 0 of 903 mutants tested

## Provenance
- Recipe: `timeout 6000 cargo mutants -p {unit} --test-tool=nextest -o {supplement_dir}/mutants-{unit} (pinned collectors.md C1: nextest pass-through, jobs=1, scratch copy; 100-min operator ceiling)`
- Ledger: UNTOUCHED by design — record #1 already declares these three as skips[declined]; the next boundary cites this supplement as first-absolute for these units
- Evidence twin: `c-mutation-heavy-trio.json` beside this file; per-unit logs `mutants-{unit}.log`; timing/disk trail `progress.txt`.