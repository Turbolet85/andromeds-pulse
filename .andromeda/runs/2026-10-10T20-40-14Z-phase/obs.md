# obs extract

## Relevance
partial — the chunk builds test data and moves one test file's port picking; it adds no product record, but the data the factories produce is what the product's intake records, scrub counters and perf-samples log are read against.

## Constraints
- The chunk adds no product emit site. The scope bars product source changes, so no span, metric target, log target or allowlist leaf is expected. If research finds a product-side change is needed, that is the scope's P4 halt, and any record it brought would need its own exact leaf carrying every field the emit site emits, guarded by set equality under `pulse-app/tests/` (per obs-plan §8 PII Scrubbing & Compliance).
- The factories build on OTLP wire-format types only. Obs-plan §1 Obs Scope Summary (architectural choice) requires that no OTel SDK is linked into the product's self-observation runtime and names the wire-format crates as the only OTLP-touching ones. Whether a dev-dependency taken for the factories would reach a product binary's graph is research's question.
- The factories' settable service and version are the SENDER's OTLP resource attributes. Obs-plan §3 Service identity fixes the product's own log identity at compile time; the factories neither set it nor take it as their default.
- Factory defaults are scrub-neutral. Obs-plan §5 Metric Coverage (the `redactions_applied` row) requires one increment per redacted value at the four column sites and one per redacted label pair at the fifth, counted for persisted cells only; a default service name, event name, metric name, severity text or label that trips a scrubber arm would change the count a migrated test reads.
- Factory defaults mint a distinct `(trace_id, span_id)` per span unless the test sets them. Obs-plan §10 SLO Invariants & Telemetry Budgets (DuckDB Connection Isolation, defect 4) records that a replayed span identity rejects the whole batch, raising `append_rejections` and a `duckdb.append` ERROR record. Whether any current hand-built site reuses identities on purpose is research's question.
- Default-built records pass the post-decode identity invariants. Obs-plan §1 Obs Scope Summary (Telemetry triggers, chaos-instrumentation) requires a malformed `span_id` or `trace_id` length to be rejected with an `ingest.grpc.parse.error` record; a test that drives that path must still be able to build the malformed record, from the factories or from a hand-built site the report counts.
- The perf-samples producer keeps its readings. Obs-plan §10 SLO Invariants & Telemetry Budgets (CI gates) requires `cargo xtask perf:budget --require memory,snapshot` to fail on a required arm with no sample or only zero memory samples, over the log `pulse-app/tests/perf_budget_samples.rs` writes. Whether that producer hand-builds telemetry, and whether the retention sweep reads a record's event time (so that a default event time outside the window empties the memory gauge), are research's questions.

## Patterns to follow
- Attribute values pass through the factories unchanged, so a planted canary reaches the product byte-for-byte: obs-plan §1 Obs Scope Summary (Telemetry triggers, logging-sensitive Vector 1) requires the negative test that sends a secret in an attribute value and reads the log for its absence.
- A test that needs to know what landed reads the aggregate fields the engine already carries on `buffer.tick` (`rows_ingested`, `append_rejections`, `redactions_applied`) instead of asking for a per-record product log (per obs-plan §3 Heartbeat ticks).
- A test leg that boots the engine gives it its own data dir, so its log family sits under that dir and never under the user's (per obs-plan §3 Log file location; obs-plan §9 CI Integration, Pipeline integration, the `xtask test` row). The port-picking change in `pulse-app/tests/integration_engine_boot.rs` keeps that.
- A check over a log family that the chunk touches never passes over an input it cannot grade: an absent family or an empty required arm is `cannot-evaluate` or FAIL (per obs-plan §10 SLO Invariants & Telemetry Budgets, Load-profile constraints).

## Anti-patterns to avoid
- NEVER link an OTel SDK into the product's self-observation runtime, including by way of a factory dependency (per obs-plan §11 Obs Anti-Patterns, Universal).
- NEVER add per-record or per-row product logging to make factory data visible to a test: hot paths are not logged at `info`, and raw OTLP attribute values are never logged (per obs-plan §11 Obs Anti-Patterns, Logs).
- NEVER conflate the watched services' telemetry with the product's own log: factory output is external OTLP data, and no factory produces a self-observation record (per obs-plan §11 Obs Anti-Patterns, Project-specific).

## Contract bindings
- obs ↔ tests: the perf-samples producer's log is the input of the `lint-test` `perf:budget` step (obs-plan §10 SLO Invariants & Telemetry Budgets, CI gates ↔ test-plan §10). A migration of that producer is a change to a graded input.
- obs ↔ tests: the log families that engine-booting tests write are uploaded by no step; a failure of such a test leaves its log on the runner only (obs-plan §9 CI Integration, CI failure → artifact triage workflow ↔ test-plan §3).
- obs ↔ security: the five ring-buffer scrub sites and the `redactions_applied` unit bind to the scrubber catalog (obs-plan §5 Metric Coverage ↔ security-plan §Logging & Monitoring). Factory defaults and canary inputs sit on that boundary.

## Acceptance criteria contributions
- (obs) The diff from the chunk base `50216d7e` adds no `tracing` emit site, target or allowlist leaf in product source (per obs-plan §8 PII Scrubbing & Compliance).
- (obs) No OTel SDK crate enters the workspace's dependency graph for the factories (per obs-plan §11 Obs Anti-Patterns, Universal).
- (obs) A batch of default-built records taken in by an engine reads `redactions_applied` 0 and `append_rejections` 0 on `buffer.tick`, with `rows_ingested` equal to the rows sent (per obs-plan §5 Metric Coverage; obs-plan §10 SLO Invariants & Telemetry Budgets, DuckDB Connection Isolation).
- (obs) If the perf-samples producer is among the migrated tests, `cargo xtask perf:budget --require memory,snapshot` over its log still reads PASS with a non-zero memory sample and its snapshot samples present (per obs-plan §10 SLO Invariants & Telemetry Budgets, CI gates).
