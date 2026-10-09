# obs extract

## Relevance
partial — the chunk changes a scrubber-primitive result shape and its storage/digest consumers (security-owned), and touches obs at three seams: the `redactions_applied` counter's unit, the log-hygiene bans on scrubbed content, and the snapshot p99 perf gate.

## Constraints
- Obs tier is Standard, and self-observation is `tracing`-only (per obs-plan §1 Obs Scope Summary). Any new diagnostic this chunk adds rides the JSON-per-line sink. No OTel SDK is linked.
- obs-plan §5 Metric Coverage (the `redactions_applied` row) requires the counter to be an aggregate count, folded once per batch onto `buffer.tick` and counted only for persisted cells. Its stated unit is "one increment per REDACTED LABEL PAIR, not per persisted cell", and a canary across the five scrubbed cells reads 5. When one value can carry several masked spans, that unit becomes ambiguous: is it per redacted value or per matched span? P3/P4 must decide this and keep it consistent with §5. Whether the builders' per-batch tally counts values or spans today is research's question.
- obs-plan §8 PII Scrubbing (the `buffer` allowlist entry) requires `redactions_applied` never to carry the matched value, its category, or the attribute key. A span-level result that exposes categories or match spans to consumers must not turn into new per-category or per-span log fields.
- obs-plan §8 PII Scrubbing (data classification table, raw OTLP payloads = High, scrub-required) and §11 Logs (Vector 1) classify the whole value as OTLP content. That includes the non-secret remainder a span-masked value now keeps. A span-masked string is therefore not log-safe: it may reach stored cells, the digest, the report, the snapshot or MCP, but never a `tracing` field. Default-deny holds.
- obs-plan §11 Telemetry Strategy and §11 Metrics ban per-field or per-match emission on the append hot path. The §5 row names the §11 hot-path rule as the reason for per-batch folding. Span masking must not add per-redaction or per-span log records.
- obs-plan §10 SLO Invariants & Telemetry Budgets requires snapshot generation p99 ≤ 500 ms. The graded window spans load + curate + format (`GenerationTimer`; measured p99 94.0 ms, n = 50). It is enforced by `cargo xtask perf:budget --require memory,snapshot` on lint-test Linux. If span masking adds work on the curate/format path, that work falls inside the graded window. Which snapshot/markdown consumers call the scrubber inside it is research's question.
- obs-plan §8 PII Scrubbing (Integration points) names three layers: at source, the subscriber Layer, and the Sentry boundary, and requires them to "agree on the same scrubbing rules". Whether the subscriber-stage Layer consumes `ScrubbedValue` (which would make it a consumer of the new shape) is research's question.

## Patterns to follow
- Tick-aggregated counters: fold once per batch at each table's own post-append site in `consumer::dispatch_batch`, with the builder returning its per-batch tally (per obs-plan §5 Metric Coverage, the `redactions_applied` row). A span-count or value-count change goes through that same tally, never through a new emit.
- Scrubs applied only for identity consistency on non-storing paths stay uncounted through the `Option<&mut u64>` = `None` convention on `extract_service_name` (per obs-plan §5 Metric Coverage, the `redactions_applied` row). The drain/template path's direct `scrub_attribute` call keeps its own tick fields.
- Allowlist leaves are exact, and their field-set guards assert equality in both directions under `pulse-app/tests/`, because of the `[lib] test = false` rule (per obs-plan §8 PII Scrubbing, the leaf entries). This applies only if the chunk adds or changes an emitted field on an allowlisted target.
- Diagnostic content (digest text, condition keys) stays in memory and is never logged (per obs-plan §8 PII Scrubbing, the `interpretation.generation.damper` entry). The same applies to the now-partially-readable digest the model reads.

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values or MCP response bodies (per obs-plan §11 Logs Vector 1/4 and §11 Project-specific). A span-masked value or digest is still that content. Never add it to a log to show that the masking worked.
- NEVER add a per-redaction, per-span or per-category `tracing` record on the ingest/append path (per obs-plan §11 Telemetry Strategy "over-instrument hot paths" and §11 Logs "log in hot path at info level").
- NEVER use unbounded label cardinality in event fields (per obs-plan §11 Metrics). A span offset, span length or matched-substring field is out of bounds. Only a bounded category label could be considered, and even that is banned on `redactions_applied` by §8.

## Contract bindings
- obs ↔ security: obs-plan §8 PII Scrubbing binds to security-plan §Security Anti-Patterns → Logging, which the scope expects to amend. The §5 `redactions_applied` row states it "makes P-047 scrubber recall gradeable from outside the process". Any change to what the counter counts is a joint obs/security amendment target, together with the P-048 matrix entry.
- obs ↔ tests: the §5 canary expectation (five scrubbed cells read 5) and the §10 `perf:budget` snapshot arm bind to test-plan §3 and to the `perf_budget_samples` producer log. A counter-unit change moves the canary's expected value. A snapshot-path cost change is graded by the existing gate.

## Acceptance criteria contributions
- (obs) The `redactions_applied` unit is stated and pinned, whether per redacted value/pair or per matched span. The five-cell canary's expected count is re-derived to match, and a multi-span value inside one cell is counted exactly as the stated unit says (per obs-plan §5 Metric Coverage).
- (obs) A canary with a secret embedded in a larger value, driven through ingest and the digest/report path, leaves both the secret AND the non-secret remainder absent from `agent-latest.jsonl`: 0 occurrences of either substring on the wire (per obs-plan §8 PII Scrubbing and §11 Logs Vector 1).
- (obs) The chunk adds no new per-redaction or per-span log target. Any changed field on an allowlisted target keeps its exact leaf, with set equality asserted in both directions under `pulse-app/tests/` (per obs-plan §8 PII Scrubbing and §11 Telemetry Strategy).
- (obs) `cargo xtask perf:budget --require memory,snapshot` stays PASS, with snapshot p99 ≤ 500 ms, after span masking lands on any consumer inside the load + curate + format window (per obs-plan §10 SLO Invariants & Telemetry Budgets).
