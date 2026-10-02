# Scope — metrics_points labels

**Marker:** `2026-08-23-metrics-points-labels`
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Working entry:** `andromeda-pulse-0.3.0/working-route.md` line 76
**Claim expectation:** NOTHING (operator directive) — fourth consecutive chunk on this posture.

## Intent (from the working entry, verbatim outcome)

> A metric point's label set survives ingestion and reads back, so dimensioned metrics stop
> arriving indistinguishable.

This is the declared other half of `2026-08-23-metrics-points-identity`. That chunk's own working
entry asked for survival *"with their labels"* and delivered survival ONLY, by operator decision at
its phase P4 gate. Identity is closed; **information loss is open**, and closing identity made the
loss SILENT — pre-fix the colliding pair failed the whole batch with a `duckdb.append` ERROR, which
was the only signal the dimension was being dropped.

## What this chunk builds

A metric data point's attribute set (its label set) is carried from the OTLP payload through the
appender into the ring buffer, scrubbed at the write boundary, and is readable back out — so two
data points of one metric differing only by label set are distinguishable in the stored row and in
what a reader gets back.

**The representation is the decision, and it is deferred to P4** (the operator directive asks for
the schema's own precedent to be reused or explicitly declined). Two shapes are on the table:
a hashed label-set key (cheap identity, opaque read-back) vs stored label pairs (real read-back,
scrub surface grows). See §Representation precedent below — the evidence is already gathered.

## Boundaries

- **In scope:** the label half only — carriage, persistence, scrub, and read-back of a data point's
  attribute set.
- **Out of scope:** identity/PK work (landed at `2026-08-23-metrics-points-identity`; the `seq`
  ordinal and the 4-column PK are not revisited). Resource-level attributes (`resource_hash`) are a
  *different* attribute set and are not being changed — but they ARE the precedent (below).
- **Out of scope:** the three producer-less tables (`resources`, `instrumentation_scopes`,
  `span_links`) — owned by the "Diagnostics un-muting + harness-truth sweep" entry. This chunk must
  only avoid *adding a fourth* one.

## Verified premises (directive coordinates + entry claims, re-derived at HEAD `d4b432b`)

Per the external-relay / annotation discipline, each was re-derived first-hand before shaping scope.
None are taken on relay.

| # | Claim | Verdict |
|---|---|---|
| 1 | No external witness — `conductor-emit` emits no metrics signal; `conductor-verify` reads nothing from `metrics_points` | **VERIFIED** at Conductor HEAD `fb8a8a5`: `conductor-emit/src/` = client · error · exception · latency · logs · message · pii · rate · span_tree · topology (no metrics module); grep `metrics_points\|MetricsService\|ResourceMetrics\|export_metrics\|v1/metrics` across all 9 `conductor/crates` → **0 files** |
| 2 | `SELECT_METRICS` (`crates/viz/src/query.rs:24`) selects five columns, no attributes | **VERIFIED** — `metric_name, ts_unix_nano, resource_hash, value, data_point_kind` |
| 3 | `push_metric_row` (`crates/buffer/src/appender.rs:768`) never reads `p.attributes` | **VERIFIED, and stronger** — the fn has no attributes parameter at all; all five `collect_metric_points` arms (Gauge 0 · Sum 1 · Histogram 2 · ExponentialHistogram 3 · Summary 4) hold `p` in scope with `p.attributes` available and never pass it |
| 4 | `metrics_points` stores no attributes column | **VERIFIED** across all THREE DDL representations (`crates/buffer/src/schema.rs:70` per-table const · `:168` concatenated `SCHEMA_DDL` · plus `viz`'s structurally-unguarded fixture copy) |
| 5 | The scrub posture flip (`cbb316c`) makes scrubbing a new label column non-optional | **VERIFIED** — commit body: "The four client-controlled DuckDB columns now scrub at the write boundary" |
| 6 | `MetricsRoute.tsx` shows "No metrics received yet" (not an error) when a batch is rejected | **VERIFIED** — `error !== null` → "Couldn't load metrics"; else `rows.length === 0` → "No metrics received yet" + exporter hint. A rejected batch is not a query error, so it lands in the empty branch |

## Representation precedent (the directive's item 2 — evidence gathered, decision at P4)

The schema **does** carry a precedent for "attribute set → identity", and it is a **cautionary** one:

- **The hash shape is live:** `resource_hash` is produced by `hash_resource`
  (`crates/buffer/src/appender.rs:789`). It hashes **only `kv.key`**, never the value, with
  `DefaultHasher` (std SipHash — documented as *not stable across Rust releases*). So the existing
  precedent for a hashed attribute set (a) discards values, (b) is not a stable identity across
  toolchain bumps, and (c) is opaque at read-back.
- **The side-table shape is declared and DEAD:** `resources` exists with `PRIMARY KEY (resource_hash)`
  (`schema.rs:100`) — the "hash → full record" companion — and has **no producer**. It is one of the
  three producer-less tables measured 2026-08-23, and one of the three retention DELETEs that sweep
  permanently-empty tables.
- **A new table has a mechanical guard:** `DELETE_BY_CUTOFF: [&str; 7]`
  (`crates/buffer/src/retention.rs:22`) plus the drift-guard test at `retention.rs:369-372` that
  iterates `RESERVED_TABLES` — so a new reserved table MUST be added to the sweep or to
  `RETENTION_EXCLUDED_TABLES`, or the guard trips.

**VERIFIED at P3, and sharper than stated** — the read-back proof surface is not merely the only
*external* witness, it is the only witness of any kind. `SELECT_METRICS` + `MetricRow`
(`crates/viz/src/query.rs:24,81`) is the sole place label content could be observed:
`MetricsChart` aggregates only `ts_unix_nano` and `value` into time buckets, and there is no
metrics table, legend, or per-row surface anywhere in the webview. A representation whose
read-back is opaque therefore cannot be shown to work by any surface this project has.

## Surfaces and contracts this reaches

- **Storage:** `crates/buffer/src/schema.rs` — all THREE DDL representations must stay in sync
  (the shipped `ddl_constants_match_concatenated_schema` test forces the first two to agree; `viz`'s
  fixture copy is structurally unguarded — `viz` has no `buffer` dep).
- **Ingestion:** `crates/buffer/src/appender.rs` — `collect_metric_points` (5 arms) ·
  `push_metric_row` · the metrics record-batch builder (which, since
  `2026-08-23-metrics-points-identity`, RETURNS its redaction tally rather than folding it, with the
  fold at the table's own post-append site).
- **Security:** every persisted label value is a client-controlled OTLP attribute value and MUST pass
  `security::scrubber::scrub_attribute` at the write boundary — the ring buffer is unencrypted, so
  redaction at write is the control, not confidentiality at rest. Label KEYS are also
  client-controlled and still need a stated disposition (scrub / bound / allow) — **P3 narrowed it
  to a STORAGE question only**: `crates/ingest/src/invariants.rs` already bounds keys at 256 B,
  values at 4096 B and count at 128 per data point (so the bound half is inherited, not built), and
  obs-plan bars keys from telemetry under either answer. P4 decides the storage disposition.
- **Read side:** `SELECT_METRICS` + `MetricRow` (`crates/viz/src/query.rs:24,81` — 5 fields,
  `specta::Type`, crosses the TauRPC bridge) · the `metrics.*` IPC response · MCP `query_metrics`
  (`crates/mcp-server/src/tools.rs`) · `MetricsRoute.tsx` / `MetricsChart`.
- **Specs:** arch §Occupied Resources (a new column, or a new reserved table that must get a producer
  AND a retention sweep) · security-plan §Anti-Patterns → Logging (the scrub-coverage claim currently
  enumerates four columns) · obs-plan §5 (`redactions_applied` persisted-cells-only counting rule).
- **Bindings:** a new `MetricRow` field regenerates `pulse-app/ui/src/bindings/index.ts`.
  **VERIFIED at P3** — no new TauRPC procedure, and `capability-drift` cannot fire on a DTO field:
  `xtask/src/main.rs:1070-1078` parses the `ARGS_MAP` procedure-name keys out of `bindings.ts` and
  set-diffs them against `EXPECTED_PROCEDURES`, which is a list of name strings. The gate still runs
  unconditionally per test-plan §3; it simply has nothing to catch here.

## Folded annotations

- **PREREQ (from the working entry) — `cargo audit` re-check, pin #8.** Standing deferral since
  `2026-08-15-corpus-key-persistence`, ratified at the 2026-08-16 adaptation wrap with an
  every-3rd-wrap INTERVAL. Probe points ran at sessions 25/28/31/34, **DISCHARGED at 34**; the next
  point is **37**. This chunk wraps as **session 36**, so the probe is **skipped in the ratified
  form** — but the basis and overlap must be re-verified (not echoed) and the PREREQ re-pinned
  forward to the next markerless entry at wrap. Signature for probe-auto-satisfy: exit `1` +
  `error: error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`,
  with `cargo deny check advisories` overlap green at the eight owned IDs
  (0189/0190/0194/0195/0204/0222/0253 + RUSTSEC-2026-0258).
- **No CARRY, no BLOCKED-ON** on this entry.

## Premise correction (P5 validation-1 — intent-incomplete)

The working entry's **NOTE** claims: *"the user-facing surface is ALREADY misleading and this entry is what
fixes it — a rejected batch is not a query error, so `MetricsRoute.tsx` falls to its empty branch and shows
'No metrics received yet' plus an exporter hint naming ports the user is exporting to successfully."*

**That claim is FALSIFIED for this chunk, while the entry's OUTCOME stands unchanged.** The misleading empty
state is caused by a REJECTED BATCH — zero rows persist, the query succeeds returning nothing, and
`MetricsRoute` takes its empty branch. Batch rejection was the *collision* defect, and
`2026-08-23-metrics-points-identity` closed it: rows now persist, so the chart populates. Measured at HEAD:
`MetricsChart` consumes only `ts_unix_nano` and `value`, and there is no metrics table, legend, or per-row
surface anywhere in the webview — so labels reach **no rendered surface at all**.

Consequence: this chunk fixes information loss in storage and read-back; it does **not** change the
user-facing surface, and no acceptance criterion should claim it does. The NOTE appears to have been carried
into the entry at the predecessor's wrap without re-checking against the state that wrap had just created.

Per the RESEARCH-CORRECTS-INTENT discipline the working-route line and its NOTE are left as the historical
source; this scope carries the correction, and a user-visible Metrics label surface is a candidate for its own
future entry rather than a silent extension here.

## Acceptance shape (outcome level — concretized at P4)

1. A metric batch carrying two data points of one metric that differ **only** by label set is
   ingested on the real OTLP path, and both rows persist **distinguishably** — the read-back shows
   which is which.
2. A label value of credential shape is **redacted** at the write boundary, and the redaction is
   counted under the persisted-cells-only rule.
3. RED-before-GREEN on the real OTLP path, per the three preceding chunks' standard: the
   indistinguishability is measured at HEAD first, then measured closed.
4. No new producer-less table and no unswept table is introduced.
