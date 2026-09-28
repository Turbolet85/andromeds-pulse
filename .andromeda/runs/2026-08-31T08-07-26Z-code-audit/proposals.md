# Code Audit — andromeda-pulse · Epoch 4 — Polish & ship: verification · 2026-08-31
mode **baseline** · HEAD `83d4060` · baseline none · span — · ledger record #1 appended

> Operator-ruled epoch boundary: the one markerless route entry (Conductor return) is administrative/external with no Pulse code surface, so HEAD is Epoch 4's final code state. **Trend judgments begin at the next boundary** — everything below is absolute-only. Founder-facing and obligation-free; fixes route through the normal route/intent channels. Evidence twins: the `c-*.json` files beside this document; reproducibility = {sha, tool_versions, commands} in the ledger record.

## Baseline findings

### B1 — mutation score vs line coverage — the hollow-test gap, quantified
**Numbers:** line coverage **84.35%** / function 84.06% (2364 tests, all green) against a weighted mutation score of **64.2%** over the 7 scoped units (274 caught / 153 missed / 582 total mutants incl. unviable). Formula: caught/(caught+missed).

| unit | mutants | caught | missed | unviable | score | duration |
|---|---|---|---|---|---|---|
| curation | 105 | 58 | 39 | 8 | **59.8%** | 370s |
| security | 9 | 6 | 0 | 3 | **100.0%** | 82s |
| snapshot | 98 | 50 | 45 | 3 | **52.6%** | 420s |
| workspace-detector | 32 | 27 | 3 | 2 | **90.0%** | 143s |
| interpretation | 127 | 69 | 43 | 15 | **61.6%** | 951s |
| viz | 56 | 40 | 3 | 13 | **93.0%** | 427s |
| config-watcher | 55 | 24 | 20 | 11 | **54.5%** | 639s |

**Suspected shapes (survivor families):** curation's `anomaly.rs` arithmetic (operator swaps in `detect_latency_outliers` / `detect_error_correlation` / `detect_cardinality_spikes` survive — tests assert presence, not values); snapshot's `markdown.rs` rendering (whole render fns replaced with `()` survive — structure asserted, content not); interpretation's `schema.rs::validate` bounds (~27 `>`→`>=`/`==` swaps survive — the L4 output validator is barely pinned, and the same fn is a complexity top-offender at cognitive 34); config-watcher lifecycle (`apply`, `record_rejection`, `emit_event`, `ConfigWatchTask::run` survive whole-body deletion at 54.5%).
**Direction:** killing tests for the four families above, highest-leverage first (schema.rs `validate`; curation arithmetic with value-level asserts). Complete survivor list (153 rows):

| unit | site | mutation |
|---|---|---|
| curation | `crates/curation/src/aggregation.rs:91` | replace - with + in percentiles |
| curation | `crates/curation/src/aggregation.rs:91` | replace - with / in percentiles |
| curation | `crates/curation/src/anomaly.rs:63` | replace kind_ordinal -> u8 with 0 |
| curation | `crates/curation/src/anomaly.rs:63` | replace kind_ordinal -> u8 with 1 |
| curation | `crates/curation/src/anomaly.rs:74` | replace < with == in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:74` | replace < with <= in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:79` | replace / with % in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:80` | replace / with % in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:80` | replace - with + in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:80` | replace - with / in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:90` | replace / with % in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:90` | replace / with * in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:90` | replace - with + in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:91` | replace > with >= in detect_latency_outliers |
| curation | `crates/curation/src/anomaly.rs:119` | replace += with *= in detect_error_correlation |
| curation | `crates/curation/src/anomaly.rs:135` | replace * with + in detect_error_correlation |
| curation | `crates/curation/src/anomaly.rs:135` | replace * with / in detect_error_correlation |
| curation | `crates/curation/src/anomaly.rs:135` | replace / with % in detect_error_correlation |
| curation | `crates/curation/src/anomaly.rs:135` | replace / with * in detect_error_correlation |
| curation | `crates/curation/src/anomaly.rs:177` | replace < with == in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:177` | replace < with <= in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:183` | replace / with % in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:187` | replace * with + in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:193` | replace < with <= in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:196` | replace / with % in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:196` | replace / with * in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:197` | replace * with + in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:197` | replace * with / in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:197` | replace - with + in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:197` | replace - with / in detect_cardinality_spikes |
| curation | `crates/curation/src/anomaly.rs:225` | replace < with == in span_duration_ms |
| curation | `crates/curation/src/anomaly.rs:225` | replace < with <= in span_duration_ms |
| curation | `crates/curation/src/anomaly.rs:225` | replace / with * in span_duration_ms |
| curation | `crates/curation/src/critical_path.rs:45` | replace += with *= in extract_critical_path |
| curation | `crates/curation/src/critical_path.rs:80` | replace match guard !c.is_empty() with true in longest_path_from |
| curation | `crates/curation/src/critical_path.rs:109` | replace == with != in prefer_longer_or_lex |
| curation | `crates/curation/src/critical_path.rs:109` | replace < with == in prefer_longer_or_lex |
| curation | `crates/curation/src/critical_path.rs:109` | replace < with <= in prefer_longer_or_lex |
| curation | `crates/curation/src/critical_path.rs:121` | replace < with <= in span_duration_ms |
| snapshot | `crates/snapshot/src/attribute_filter.rs:80` | replace > with == in truncate_value |
| snapshot | `crates/snapshot/src/attribute_filter.rs:80` | replace > with < in truncate_value |
| snapshot | `crates/snapshot/src/attribute_filter.rs:80` | replace > with >= in truncate_value |
| snapshot | `crates/snapshot/src/attribute_filter.rs:83` | replace += with *= in truncate_value |
| snapshot | `crates/snapshot/src/markdown.rs:53` | replace > with >= in format_markdown |
| snapshot | `crates/snapshot/src/markdown.rs:104` | replace <= with > in format_markdown |
| snapshot | `crates/snapshot/src/markdown.rs:148` | replace && with // in finish_ok |
| snapshot | `crates/snapshot/src/markdown.rs:282` | replace > with == in truncate_to_byte_cap |
| snapshot | `crates/snapshot/src/markdown.rs:282` | replace > with < in truncate_to_byte_cap |
| snapshot | `crates/snapshot/src/markdown.rs:282` | replace > with >= in truncate_to_byte_cap |
| snapshot | `crates/snapshot/src/markdown.rs:285` | replace += with *= in truncate_to_byte_cap |
| snapshot | `crates/snapshot/src/markdown.rs:294` | replace span_id_hex -> String with String::new() |
| snapshot | `crates/snapshot/src/markdown.rs:294` | replace span_id_hex -> String with "xyzzy".into() |
| snapshot | `crates/snapshot/src/markdown.rs:326` | replace == with != in render_section_anomalies |
| snapshot | `crates/snapshot/src/markdown.rs:344` | delete ! in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:347` | replace > with == in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:347` | replace > with < in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:347` | replace > with >= in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:359` | delete ! in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:364` | replace > with == in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:364` | replace > with < in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:364` | replace > with >= in render_anomaly_subsection |
| snapshot | `crates/snapshot/src/markdown.rs:435` | replace render_service_block with () |
| snapshot | `crates/snapshot/src/markdown.rs:450` | replace && with // in render_section_unique_spans |
| snapshot | `crates/snapshot/src/markdown.rs:472` | replace / with % in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:472` | replace / with * in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:475` | delete match arm 0 in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:476` | delete match arm 1 in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:477` | delete match arm 2 in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:486` | replace && with // in render_span_block |
| snapshot | `crates/snapshot/src/markdown.rs:513` | replace render_truncation_marker_attributes with () |
| snapshot | `crates/snapshot/src/markdown.rs:513` | replace == with != in render_truncation_marker_attributes |
| snapshot | `crates/snapshot/src/markdown.rs:526` | replace render_truncation_marker_spans with () |
| snapshot | `crates/snapshot/src/markdown.rs:526` | replace == with != in render_truncation_marker_spans |
| snapshot | `crates/snapshot/src/markdown.rs:539` | replace classify_alert_services -> HashSet<String> with HashSet::new() |
| snapshot | `crates/snapshot/src/markdown.rs:539` | replace classify_alert_services -> HashSet<String> with HashSet::from_iter([String::new()]) |
| snapshot | `crates/snapshot/src/markdown.rs:539` | replace classify_alert_services -> HashSet<String> with HashSet::from_iter(["xyzzy".into()]) |
| snapshot | `crates/snapshot/src/markdown.rs:541` | replace == with != in classify_alert_services |
| snapshot | `crates/snapshot/src/markdown.rs:559` | replace collect_high_priority_span_ids -> HashSet<[u8; 8]> with HashSet::new() |
| snapshot | `crates/snapshot/src/markdown.rs:559` | replace collect_high_priority_span_ids -> HashSet<[u8; 8]> with HashSet::from_iter([[0; 8]]) |
| snapshot | `crates/snapshot/src/markdown.rs:559` | replace collect_high_priority_span_ids -> HashSet<[u8; 8]> with HashSet::from_iter([[1; 8]]) |
| snapshot | `crates/snapshot/src/markdown.rs:588` | replace total_attribute_count -> usize with 0 |
| snapshot | `crates/snapshot/src/markdown.rs:588` | replace total_attribute_count -> usize with 1 |
| snapshot | `crates/snapshot/src/markdown.rs:592` | replace section_h2_count -> usize with 0 |
| snapshot | `crates/snapshot/src/markdown.rs:592` | replace section_h2_count -> usize with 1 |
| workspace-detector | `crates/workspace-detector/src/contract.rs:117` | replace > with >= in read_published_workspace_key |
| workspace-detector | `crates/workspace-detector/src/vcs.rs:50` | replace < with == in sha_basename |
| workspace-detector | `crates/workspace-detector/src/vcs.rs:50` | replace < with <= in sha_basename |
| interpretation | `crates/interpretation/src/hardware.rs:108` | replace >= with < in detect_real |
| interpretation | `crates/interpretation/src/hardware.rs:132` | replace detect_gpu_present -> bool with true |
| interpretation | `crates/interpretation/src/hardware.rs:132` | replace detect_gpu_present -> bool with false |
| interpretation | `crates/interpretation/src/hardware.rs:173` | replace available_cpu_cores -> usize with 1 |
| interpretation | `crates/interpretation/src/hardware.rs:186` | delete match arm "cpu_fallback" / "cpu-fallback" in parse_profile_override |
| interpretation | `crates/interpretation/src/markdown.rs:344` | replace hex_lower -> String with String::new() |
| interpretation | `crates/interpretation/src/markdown.rs:344` | replace hex_lower -> String with "xyzzy".into() |
| interpretation | `crates/interpretation/src/markdown.rs:352` | replace incident_status_label -> &'static str with "" |
| interpretation | `crates/interpretation/src/markdown.rs:352` | replace incident_status_label -> &'static str with "xyzzy" |
| interpretation | `crates/interpretation/src/markdown.rs:360` | replace severity_label -> &'static str with "" |
| interpretation | `crates/interpretation/src/markdown.rs:360` | replace severity_label -> &'static str with "xyzzy" |
| interpretation | `crates/interpretation/src/markdown.rs:369` | replace l4_confidence_label -> &'static str with "" |
| interpretation | `crates/interpretation/src/markdown.rs:369` | replace l4_confidence_label -> &'static str with "xyzzy" |
| interpretation | `crates/interpretation/src/prompt.rs:65` | replace > with == in push_citable_ids_section |
| interpretation | `crates/interpretation/src/prompt.rs:65` | replace > with < in push_citable_ids_section |
| interpretation | `crates/interpretation/src/prompt.rs:65` | replace > with >= in push_citable_ids_section |
| interpretation | `crates/interpretation/src/schema.rs:53` | replace * with + |
| interpretation | `crates/interpretation/src/schema.rs:195` | replace // with && in validate |
| interpretation | `crates/interpretation/src/schema.rs:195` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:195` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:200` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:205` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:205` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:210` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:210` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:224` | replace // with && in validate |
| interpretation | `crates/interpretation/src/schema.rs:224` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:224` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:229` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:229` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:244` | replace // with && in validate |
| interpretation | `crates/interpretation/src/schema.rs:244` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:244` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:249` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:249` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:255` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:255` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:264` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:264` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:270` | replace // with && in validate |
| interpretation | `crates/interpretation/src/schema.rs:270` | replace > with == in validate |
| interpretation | `crates/interpretation/src/schema.rs:270` | replace > with >= in validate |
| interpretation | `crates/interpretation/src/schema.rs:323` | replace > with >= in parse_bounded |
| viz | `crates/viz/src/query.rs:447` | replace < with <= in compute_window |
| viz | `crates/viz/src/query.rs:488` | replace short_err -> String with String::new() |
| viz | `crates/viz/src/query.rs:488` | replace short_err -> String with "xyzzy".into() |
| config-watcher | `crates/config-watcher/src/event.rs:35` | replace kind_label -> &'static str with "" |
| config-watcher | `crates/config-watcher/src/event.rs:35` | replace kind_label -> &'static str with "xyzzy" |
| config-watcher | `crates/config-watcher/src/event.rs:80` | replace ConfigEventBroadcast::subscriber_count -> usize with 0 |
| config-watcher | `crates/config-watcher/src/partition.rs:26` | replace ChangedKeys::is_empty -> bool with true |
| config-watcher | `crates/config-watcher/src/partition.rs:26` | replace && with // in ChangedKeys::is_empty |
| config-watcher | `crates/config-watcher/src/partition.rs:26` | replace && with // in ChangedKeys::is_empty |
| config-watcher | `crates/config-watcher/src/watcher.rs:146` | replace == with != in ConfigWatchHandle::reload_now |
| config-watcher | `crates/config-watcher/src/watcher.rs:163` | replace ConfigWatchHandle::apply with () |
| config-watcher | `crates/config-watcher/src/watcher.rs:170` | replace += with -= in ConfigWatchHandle::apply |
| config-watcher | `crates/config-watcher/src/watcher.rs:170` | replace += with *= in ConfigWatchHandle::apply |
| config-watcher | `crates/config-watcher/src/watcher.rs:173` | delete ! in ConfigWatchHandle::apply |
| config-watcher | `crates/config-watcher/src/watcher.rs:192` | replace ConfigWatchHandle::record_rejection with () |
| config-watcher | `crates/config-watcher/src/watcher.rs:212` | replace ConfigWatchHandle::status_snapshot -> ConfigStatus with Default::default() |
| config-watcher | `crates/config-watcher/src/watcher.rs:223` | replace emit_event with () |
| config-watcher | `crates/config-watcher/src/watcher.rs:265` | replace ConfigWatchTask::run with () |
| config-watcher | `crates/config-watcher/src/watcher.rs:266` | delete ! in ConfigWatchTask::run |
| config-watcher | `crates/config-watcher/src/watcher.rs:309` | replace == with != in start_config_watcher |
| config-watcher | `crates/config-watcher/src/watcher.rs:329` | replace now_unix_nano -> i64 with 0 |
| config-watcher | `crates/config-watcher/src/watcher.rs:329` | replace now_unix_nano -> i64 with 1 |
| config-watcher | `crates/config-watcher/src/watcher.rs:329` | replace now_unix_nano -> i64 with -1 |

### B2 — dead-code candidates — 162 zero-ref symbols (upper bound), FP classes dominate
**Numbers:** 2949 raw zero-ref symbols → 1125 after symbol-path `tests/` exclusion → **162** after the DUAL symbol+file filter (pinned this baseline: the symbol-only recipe missed integration-test-target fns — 1125 vs 162, the filter-drift class the collector table warns about). Counts are **candidates, never "dead"**: the FP classes below are structural.
**Top-40 classed** (8/40 candidate-rate — extrapolated, most of the 162 are FP classes):

| # | symbol | file | class |
|---|---|---|---|
| 0 | `pulse-app::streams/StreamsApi#subscribe_spans().` | pulse-app/src/streams.rs | fp-runtime-invoked (TauRPC) |
| 1 | `pulse-app::observability/impl#[`JsonFieldVisitor<'_>`][Visit]record_u64().` | pulse-app/src/observability.rs | fp-trait-dispatch (Visit) |
| 2 | `pulse-app::observability/impl#[`JsonFieldVisitor<'_>`][Visit]record_f64().` | pulse-app/src/observability.rs | fp-trait-dispatch (Visit) |
| 3 | `pulse-app::llamacli_inference/impl#[LlamaCliInference]configured_model_path().` | pulse-app/src/llamacli_inference.rs | CANDIDATE |
| 4 | `pulse-app::incidents_router/impl#[IncidentsApiImpl][IncidentsApi]get_report().` | pulse-app/src/incidents_router.rs | fp-trait-dispatch (TauRPC impl) |
| 5 | `pulse-app::corpus_retrieval/impl#[CorpusBackedIncidentSource][CorpusIncidentSource]load_candidates().` | pulse-app/src/corpus_retrieval.rs | fp-trait-dispatch |
| 6 | `pulse-app::cadence_runner/impl#[CadenceSqlRunner][SqlQueryRunner]run_q2().` | pulse-app/src/cadence_runner.rs | fp-trait-dispatch |
| 7 | `ui-bridge::health/runtime/IntrospectionApi#health().` | crates/ui-bridge/src/health.rs | fp-runtime-invoked (TauRPC) |
| 8 | `mcp-server::tools/default_token_budget().` | crates/mcp-server/src/tools.rs | fp-serde-default |
| 9 | `triage::incident/persistence/_force_serde_imports_used().` | crates/triage/src/incident/persistence.rs | fp-anchor (deliberate) |
| 10 | `corpus::contract/impl#[Corpus][CorpusWriter]save_digest().` | crates/corpus/src/contract.rs | fp-trait-dispatch (CorpusWriter) |
| 11 | `xtask::bundle_format/impl#[BundleFormat][FromStr]from_str().` | xtask/src/bundle_format.rs | fp-trait-dispatch (FromStr) |
| 12 | `pulse-app::streams/StreamsApi#subscribe_logs().` | pulse-app/src/streams.rs | fp-runtime-invoked (TauRPC) |
| 13 | `pulse-app::model_router/ModelApi#current_profile().` | pulse-app/src/model_router.rs | fp-runtime-invoked (TauRPC) |
| 14 | `pulse-app::incidents_router/IncidentsApi#mark_resolved().` | pulse-app/src/incidents_router.rs | fp-runtime-invoked (TauRPC) |
| 15 | `pulse-app::incident_persistence/impl#[CorpusIncidentPersistence][IncidentPersistence]mark_read().` | pulse-app/src/incident_persistence.rs | fp-trait-dispatch |
| 16 | `pulse-app::config_router/impl#[ConfigApiImpl][ConfigApi]reload().` | pulse-app/src/config_router.rs | fp-trait-dispatch (TauRPC impl) |
| 17 | `pulse-app::cadence_runner/impl#[CadenceSqlRunner][SqlQueryRunner]run_q7().` | pulse-app/src/cadence_runner.rs | fp-trait-dispatch |
| 18 | `config-watcher::watcher/impl#[ConfigWatchHandle]status_snapshot().` | crates/config-watcher/src/watcher.rs | CANDIDATE |
| 19 | `mcp-server::tools/default_time_window().` | crates/mcp-server/src/tools.rs | fp-serde-default |
| 20 | `pulse-app::observability/impl#[`JsonFieldVisitor<'_>`][Visit]record_debug().` | pulse-app/src/observability.rs | fp-trait-dispatch (Visit) |
| 21 | `pulse-app::llamacli_inference/impl#[LlamaCliInference]binary_kind().` | pulse-app/src/llamacli_inference.rs | CANDIDATE |
| 22 | `pulse-app::diagnostics_router/MetricHistoryPoint#snapshot_unix_nano.` | pulse-app/src/diagnostics_router.rs | CANDIDATE (unread field) |
| 23 | `pulse-app::diagnostics_router/DiagnosticsApi#template_distribution().` | pulse-app/src/diagnostics_router.rs | fp-runtime-invoked (TauRPC) |
| 24 | `ingest::main().` | crates/ingest/build.rs | fp-entry-point (build.rs) |
| 25 | `ingest::main().` | crates/ingest/examples/load_profiles.rs | fp-entry-point (example) |
| 26 | `ingest::main().` | crates/ingest/examples/inject_scrub_canaries.rs | fp-entry-point (example) |
| 27 | `ingest::main().` | crates/ingest/examples/inject_demo.rs | fp-entry-point (example) |
| 28 | `ingest::main().` | crates/ingest/examples/inject_colliding_metrics.rs | fp-entry-point (example) |
| 29 | `ingest::main().` | crates/ingest/examples/inject_colliding_logs.rs | fp-entry-point (example) |
| 30 | `snapshot::contract/FormatError#AnchorEncodingFailed#` | crates/snapshot/src/contract.rs | CANDIDATE (unconstructed error variant) |
| 31 | `triage::digest/DigestError#TokenBudgetExceeded#limit.` | crates/triage/src/digest/mod.rs | CANDIDATE (unread field) |
| 32 | `triage::digest/queue/impl#[LwwQueue]active_incident_depth_for().` | crates/triage/src/digest/queue.rs | CANDIDATE |
| 33 | `triage::cue/emitter/impl#[CueLatch][Default]default().` | crates/triage/src/cue/emitter.rs | fp-trait-dispatch (Default) |
| 34 | `ingest::grpc/impl#[MetricsServiceImpl][MetricsService]export().` | crates/ingest/src/grpc.rs | fp-runtime-invoked (tonic service) |
| 35 | `ingest::connection/STREAM_NAME_CONNECTION_STATE.` | crates/ingest/src/connection.rs | CANDIDATE (unused const) |
| 36 | `ingest::connection/impl#[ConnectionBroadcast][Default]default().` | crates/ingest/src/connection.rs | fp-trait-dispatch (Default) |
| 37 | `pulse-app::streams/impl#[StreamsApiImpl][StreamsApi]subscribe_spans().` | pulse-app/src/streams.rs | fp-trait-dispatch (TauRPC impl) |
| 38 | `pulse-app::services_router/ServicesApi#list_with_states().` | pulse-app/src/services_router.rs | fp-runtime-invoked (TauRPC) |
| 39 | `pulse-app::plugins_router/PluginsApi#list().` | pulse-app/src/plugins_router.rs | fp-runtime-invoked (TauRPC) |

**Corroborated candidates** (two independent signals): `ConfigWatchHandle::status_snapshot` (zero-ref AND its mutation survived); `LwwQueue::active_incident_depth_for` (sibling of the graph-proven-dead `drain_all` removed at 2026-08-25 — the rest of the queue's read surface may be following it); `MetricHistoryPoint#snapshot_unix_nano` (unread field of the `diagnostics.history` validated stub); `FormatError::AnchorEncodingFailed` (unconstructed variant); `STREAM_NAME_CONNECTION_STATE` (unused const); `LlamaCliInference::{configured_model_path, binary_kind}`.
**Direction:** a sweep chunk over the corroborated set (delete or wire a consumer), leaving the FP classes untouched; the 162 number is the trend anchor, not a work list.

### B3 — unused dependencies — 12 declarations across 8 manifests (cargo-machete)

- `corpus: chrono`
- `corpus: security`
- `curation: thiserror`
- `ingest: bytes`
- `ingest: tonic-prost`
- `ingest: tower`
- `security: thiserror`
- `security: tracing`
- `snapshot: chrono`
- `ui-bridge: tauri`
- `viz: tracing-error`
- `xtask: ui-bridge`

**FP caveat:** machete misses macro-only usage — `ui-bridge: tauri` is plausibly taurpc-macro-referenced; verify before removing. **Suspected real:** `xtask: ui-bridge` (the 2026-08-30 harness-truth chunk replaced the in-xtask `ui_bridge::health` call with `harness_status`); `corpus: security` (resolver-side scrubbing moved to pulse-app per the 2026-05-26 defense-in-depth pattern); `ingest: {bytes, tonic-prost, tower}`.
**Direction:** per-dep remove or `[package.metadata.cargo-machete] ignored` with provenance — the same disposition discipline the deny/advisory gates use.

### B4 — duplication starting table — 7.09% of lines (716 clones: 382 src / 293 test / 41 mixed)
**Numbers:** 8669 duplicated lines / 122295 scanned (466 sources; min-tokens 50; token-level 9.19%).

| lines | class | file A | file B |
|---|---|---|---|
| 158 | src | pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.tsx | pulse-app/ui/src/widget/ConstellationCanvas.tsx |
| 74 | src | crates/triage/src/baseline/mod.rs | crates/triage/src/pattern/storm.rs |
| 70 | src | crates/triage/src/baseline/mod.rs | crates/triage/src/pattern/detector.rs |
| 64 | mixed | crates/triage/src/baseline/mod.rs | pulse-app/tests/unit_baseline_persistence.rs |
| 64 | test | pulse-app/tests/security_l4_parse_failure_does_not_leak_output.rs | pulse-app/tests/unit_degraded_mode_runtime.rs |
| 55 | src | crates/triage/src/baseline/mod.rs | crates/triage/src/lifecycle/mod.rs |
| 49 | test | pulse-app/tests/integration_constellation_severity_workspace_key.rs | pulse-app/tests/integration_tier1_storm_one_incident.rs |
| 49 | test | pulse-app/tests/security_path_env_var_canonicalization.rs | pulse-app/tests/security_plugin_path_basename_only.rs |
| 48 | mixed | crates/buffer/src/appender.rs | pulse-app/tests/integration_investigate_actions.rs |
| 48 | test | pulse-app/tests/integration_constellation_severity_workspace_key.rs | pulse-app/tests/integration_deterministic_l4_mode.rs |

**Suspected shape:** the 158-line src clone `MetricsChart.tsx` ↔ `ConstellationCanvas.tsx` is a shared-chart-primitive extraction waiting to happen; the `triage` trio (`baseline/mod.rs` ↔ `pattern/storm.rs` ↔ `pattern/detector.rs`, 70–74L) duplicates window/EWMA plumbing across the detector family.
**Direction:** extract the shared webview chart primitive; consider a shared window-state helper for the triage trio. Test-class clones (293) are the usual fixture repetition — cheaper to leave.

### B5 — complexity starting table — 32 functions over ceiling (cognitive>15 rust · cyclomatic>15 ts)
**Numbers:** 5770 rust + 2361 ts functions; cyclomatic p50/p90 1/4.0; cognitive p50/p90 0/1.0 (rust only — lizard carries no cognitive).

| value | metric | function | file |
|---|---|---|---|
| 90.0 | cognitive | `main` | pulse-app/src/main.rs |
| 53.0 | cognitive | `<anonymous>` | pulse-app/src/main.rs |
| 52.0 | cognitive | `verify_capability_matrix` | xtask/src/main.rs |
| 49.0 | cognitive | `main` | crates/ingest/examples/inject_demo.rs |
| 37.0 | cognitive | `parse_bindings` | xtask/src/main.rs |
| 35.0 | cognitive | `security_workspace_wide_loopback_only_no_non_loopback_bind_literals` | pulse-app/tests/e2e_security_negative_canaries.rs |
| 35.0 | cognitive | `capability_widening_check` | xtask/src/main.rs |
| 34.0 | cognitive | `validate` | crates/interpretation/src/schema.rs |
| 34.0 | cognitive | `walk` | pulse-app/tests/e2e_security_negative_canaries.rs |
| 33 | cyclomatic | `SettingsModalForm` | pulse-app/ui/src/dashboard/routes/SettingsModalForm.tsx |

**Suspected shape:** the top is boot wiring and harness verbs (`main` 90 — the known God-boot; `verify_capability_matrix` 52, `parse_bindings` 37 in xtask; `inject_demo::main` 49) — high-fan-in glue, not algorithmic rot. The one corroborated hotspot is `interpretation/schema.rs::validate` (cognitive 34 AND the largest mutation-survivor family).
**Direction:** none urgent at baseline; if `main.rs` keeps growing (1547 lines, cog 90) a boot-module split is the natural cut. Watch `validate` via B1's killing tests rather than a refactor.

### B6 — size starting table — 98573 LOC / 484 files / 16 units · 22 files >800 lines
**Numbers:** file p50/p90/max 105/465/3057.

| code lines | file |
|---|---|
| 3057 | pulse-app/tests/observability_pins.rs |
| 2366 | crates/buffer/src/appender.rs |
| 2157 | pulse-app/src/observability.rs |
| 2115 | crates/ui-bridge/src/contract.rs |
| 1547 | pulse-app/src/main.rs |
| 1510 | xtask/src/main.rs |
| 1262 | crates/corpus/src/contract.rs |
| 1258 | xtask/src/webview_drive.rs |
| 1171 | crates/triage/src/cue/emitter.rs |
| 1095 | crates/viz/src/query.rs |

**Note:** the max (`observability_pins.rs`, 3057) is the deliberate 2026-08-30 dead-test migration target; `appender.rs` 2366 / `observability.rs` 2157 / `contract.rs` 2115 are the src heavyweights to watch for growth.

### B7 — dependency graph — **0 cycles** · 35 cross-unit edges
**The headline absolute check passes plainly: no crate cycle exists.** Starting tables for the trend:

fan-out per unit (instability proxy): pulse-app 13 · mcp-server 8 · ui-bridge 7 · buffer 2 · interpretation 2 · config-watcher 1 · xtask 1 · snapshot 1

| fan-in (distinct callers) | symbol |
|---|---|
| 215 | `ui-bridge::contract/AppError#` |
| 183 | `triage::contract/CueKind#` |
| 151 | `triage::crate/` |
| 127 | `triage::baseline/BaselineState#` |
| 115 | `triage::contract/CueScope#` |
| 111 | `triage::contract/CueKind#ErrorRateSpike#` |
| 111 | `pulse-app::observability/AllowList#` |
| 108 | `pulse-app::observability/impl#[AllowList]production().` |
| 107 | `pulse-app::observability/impl#[AllowList]for_target().` |
| 106 | `triage::contract/` |
| 106 | `triage::contract/Incident#` |
| 101 | `triage::contract/PriorityTier#` |
| 97 | `triage::contract/IncidentStatus#` |
| 95 | `corpus::error/Error#` |
| 90 | `buffer::state/BufferState#` |
| 84 | `triage::baseline/impl#[BaselineState]new().` |
| 82 | `ui-bridge::contract/Settings#` |
| 79 | `pulse-app::capture_json_lines().` |
| 79 | `triage::lifecycle/state_machine/ServiceLifecycleState#` |
| 79 | `triage::contract/CueScope#Service#` |

_ts plane crate_edges = 0 (single-package plane — module-level cycle metric not applicable)_

### B8 — coverage vs the project's own stated gates
**Numbers:** line **84.35%** (stated floor ≥75% — comfortably above) · function **84.06%** against the stated ≥85% function floor — **0.94pt under**, stated without verdict (the gate's own tooling basis may differ) · branch **unmeasurable on this toolchain**: `--branch` maps to `-Z coverage-options=branch`, nightly-only on stable 1.95.0 — which also means the test-plan's ≥70% branch floor cannot currently be produced by the documented `cargo llvm-cov` command on this host. That last fact is a founder-facing observation about the GATE, not the code.

## Informational
- `crates/triage-experimental/` exists on disk **untracked** (src/lib.rs + src/main.rs; not a workspace member, not git-tracked) — fs-walk collectors leaked it into complexity until population-filtered; surfaced for disposition (adopt, relocate, or delete — operator's call).
- `interpretation` mutation ran 951s — over the 15-min budget, under the 25-min ceiling; complete, not truncated.
- Tool-substitution note for the trend: this host's `jscpd` is the **Rust `cpd 5.0.16` reimplementation** (flags differ from node jscpd); the recorded command is the firing form.
- Disk: pre-authorized `cargo clean` freed 291.0 GiB in 78s before the instrumented build; ~187 GB free at run end.
- Coverage suite: 2364/2364 passed + 1 skip under instrumentation — matches the known suite exactly.
- Audit side effect, caught by the exit self-check: the coverage run's default-features workspace suite CLOBBERED `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape (the documented taurpc dev-mode export class, worktree 0 vs HEAD 1 `"mcp":`); restored to HEAD byte-identical before exit — the tree gained nothing outside the run dir + ledger.

## Below threshold — no action
Baseline mode: no prior record exists, so no movements — this section starts reporting at the next boundary.

## Skips
- **churn** — no-baseline
- **hotspots** — no-baseline
- **coverage-branch** — tool-missing (-Z coverage-options=branch nightly-only on stable 1.95.0; line+function recorded)
- **mutation:buffer** — declined (operator-declared: 375 mutants ~40 min/unit - overnight-run / next-boundary candidate)
- **mutation:triage** — declined (operator-declared: 908 mutants ~93 min - overnight-run / next-boundary candidate)
- **mutation:pulse-app** — declined (operator-declared: 903 mutants ~92 min - overnight-run / next-boundary candidate)
- **mutation:xtask,ingest,mcp-server,ui-bridge,corpus,plugins** — declined (outside the operator-chosen 7-unit subset (56-725 mutants, 8-74 min each))

_Generated by compose_audit.py from the c-*.json evidence twins; ledger record #1 in `.andromeda/code-metrics.ndjson` is the trend anchor._