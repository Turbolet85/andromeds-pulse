# Literal census — 2026-10-10-shared-telemetry-test-data (plan step 7)

What the hand-built form leaves behind. Research's scan, re-run at /implement on the chunk base `50216d7e` (read
through git) and on the working tree (tracked `.rs` files plus the untracked ones git does not ignore).

**The scan.** A line scan of every `.rs` file for a line that opens a struct literal `Span {`, `Event {`,
`NumberDataPoint {` or `LogRecord {`, the name not preceded by a letter, digit or underscore (so `ResourceSpans {`
is no `Span {`); lines holding `->`, `fn`, `impl`, `use` or a leading `//` are left out. The wrapper column counts
`Export…ServiceRequest {`, `Resource {`, `Resource(Spans|Metrics|Logs) {`, `Scope(Spans|Metrics|Logs) {` and
`Metric {` the same way.

**Control.** At the base the four record kinds read 25, 11, 10 and 14 in 26 files, 28 files with the two
wrapper-only ones: research's numbers, each one. The wrapper column reads 180 at the base where research states
170; research's exact wrapper pattern is not recorded and five variants of this one read 180, 180, 169, 161 and
148, none 170. The wrapper column is therefore this re-run's own reading, not research's.

## Totals
| kind | base | after | of which the factory's own literal | hand-built after |
|---|---|---|---|---|
| span | 25 | 20 | 1 | 19 |
| span event | 11 | 8 | 1 | 7 |
| number point | 10 | 8 | 1 | 7 |
| log record | 14 | 13 | 1 | 12 |
| wrapper (this re-run's pattern) | 180 | 155 | 10 | 145 |

Files holding a record literal: 26 at the base, 22 after (21 hand-built, and `crates/test-support/src/otlp.rs`,
the one place the factories build each kind). The plan predicted 19, 7, "9 or fewer" and 12 hand-built: measured
19, 7, 7 and 12.

## Files this chunk moved (span · span event · number point · log record · wrapper, base → after)
| file | span | event | point | log | wrapper |
|---|---|---|---|---|---|
| `pulse-app/tests/integration_engine_boot.rs` | 1→0 | 0→0 | 0→0 | 0→0 | 4→0 |
| `pulse-app/tests/integration_console_engine.rs` | 1→0 | 0→0 | 0→0 | 0→0 | 4→0 |
| `pulse-app/tests/e2e_drain_template_assignment.rs` | 0→0 | 0→0 | 0→0 | 1→0 | 4→0 |
| `pulse-app/tests/e2e_pii_span_masking_stored_field.rs` | 1→0 | 1→0 | 0→0 | 1→0 | 7→0 |
| `crates/buffer/src/consumer.rs` (test module) | 3→0 | 3→0 | 1→0 | 0→0 | 10→0 |
| `crates/buffer/src/appender.rs` (test module, the metric-point helper group) | 1→1 | 1→1 | 7→5 | 9→9 | 44→38 |
| `crates/test-support/src/otlp.rs` (new: the factories) | 0→1 | 0→1 | 0→1 | 0→1 | 0→10 |

Five of the six hold no record literal and no wrapper literal after. `appender.rs` holds what its other tests
build by hand; the group this chunk moved (`gauge_point`, `gauge_metric`, `labelled_metric_batch`, and the
wrapper inside `append_metrics_on_worker`) holds none.

## Files this chunk did not move, each with the route entry that owns it
Counts are unchanged base → after in every row. Owners are research's ("Owners of the sites this chunk does not
move"); each title was read on `andromeda-pulse-0.4.0/working-route.md` at /implement.

| file | span | event | point | log | wrapper | owning route entry |
|---|---|---|---|---|---|---|
| `crates/buffer/src/appender.rs` (what stays) | 1 | 1 | 5 | 9 | 38 | `Span keeps its identity` · `Span and resource attributes kept` · `Histogram metrics keep their shape` · `Logs and metrics name their service` |
| `crates/ingest/tests/backpressure.rs` | 1 | 0 | 0 | 0 | 3 | `Network OTLP receiver behind the token` · `Receiver refusals` · `Per-sender bounds` |
| `crates/ingest/tests/grpc_loopback.rs` | 1 | 0 | 0 | 0 | 6 | the same three |
| `crates/ingest/tests/http_loopback.rs` | 1 | 0 | 0 | 0 | 11 | the same three |
| `crates/ingest/tests/invariants.rs` | 1 | 0 | 0 | 0 | 3 | the same three |
| `crates/ingest/tests/proptest_invariants.rs` | 1 | 0 | 0 | 0 | 2 | the same three |
| `crates/ingest/tests/rate_limit.rs` | 1 | 0 | 0 | 0 | 3 | the same three |
| `crates/ingest/src/invariants.rs` | 1 | 3 | 0 | 0 | 2 | the same three |
| `crates/ingest/src/grpc.rs` | 0 | 0 | 0 | 0 | 6 | the same three |
| `crates/ingest/src/http.rs` | 0 | 0 | 0 | 0 | 6 | the same three |
| `crates/ingest/examples/inject_demo.rs` | 1 | 0 | 0 | 0 | 4 | `Scenario legs driven against the console engine` |
| `crates/ingest/examples/inject_colliding_logs.rs` | 0 | 0 | 0 | 1 | 4 | `Scenario legs driven against the console engine` |
| `crates/ingest/examples/inject_colliding_metrics.rs` | 0 | 0 | 1 | 0 | 5 | `Scenario legs driven against the console engine` |
| `crates/ingest/examples/inject_scrub_canaries.rs` | 1 | 1 | 1 | 1 | 13 | `Scenario legs driven against the console engine` |
| `crates/ingest/examples/load_profiles.rs` | 1 | 0 | 0 | 0 | 4 | `Load profiles re-based on the engine` |
| `pulse-app/tests/perf_budget_samples.rs` | 1 | 0 | 0 | 0 | 4 | `Load profiles re-based on the engine` |
| `pulse-app/tests/perf_load_profiles.rs` | 1 | 0 | 0 | 0 | 4 | `Load profiles re-based on the engine` |
| `pulse-app/tests/perf_slo_10k_spans.rs` | 1 | 0 | 0 | 0 | 4 | `Load profiles re-based on the engine` |
| `pulse-app/tests/e2e_storm_detection.rs` | 1 | 1 | 0 | 0 | 4 | `Detection baseline through the console engine` |
| `pulse-app/tests/e2e_p1_otlp_grpc_to_traces_query.rs` | 1 | 0 | 0 | 0 | 4 | `Detection parity after the removals` |
| `pulse-app/tests/e2e_p6_channel_arrow_ipc.rs` | 1 | 0 | 0 | 0 | 4 | `Display-only computation retired` |
| `pulse-app/tests/e2e_pii_bare_credential_stored_field.rs` | 1 | 1 | 0 | 1 | 7 | `Span keeps its identity` |
| `xtask/src/smoke.rs` | 1 | 0 | 0 | 0 | 4 | `Desktop distribution retired` |

Sums of the unmoved rows: span 19, span event 7, number point 7, log record 12, the hand-built totals above.

## Limits
- A line scan, not a parse: a literal opened on a line that also holds `->`, `fn`, `impl` or `use` is not counted,
  and a literal built through a type alias or `Default::default()` alone is not seen.
- The owners are research's proposal as shown on the P5 card; none of those entries has been planned, and a plan
  may move or remove a site otherwise.
- `crates/ingest/src` unit-test modules cannot take the member (the test build of `ingest` and the `ingest` the
  member links are two crates to the compiler), so those three files move only by another form.
