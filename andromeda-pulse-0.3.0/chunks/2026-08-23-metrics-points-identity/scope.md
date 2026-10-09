# Scope — 2026-08-23-metrics-points-identity

**Version:** andromeda-pulse-0.3.0 · **Epoch:** Epoch 4 — Polish & ship: verification
**Working entry:** "metrics_points identity — two data points of one metric differing only by label set both survive ingestion, with their labels"

## The claim under test

The `metrics_points` primary key is `(metric_name, ts_unix_nano, resource_hash)`. The table stores no
attributes column at all, so two data points of ONE metric that differ only by their label set — the
ordinary shape of any dimensioned metric (`http.server.duration{route=/a}` vs `{route=/b}`) — are
identical on all three key columns. The claim is that the pair **collides by construction**, and that the
label set is **lost outright even when a point does land**.

Unlike its sibling, this chunk starts from a **measured** failure mode rather than a theorised one.
`2026-08-22-log-records-identity` established on the real OTLP path that the Arrow `Appender` enforces the
primary key at `flush()`, that the resulting loss is **whole-batch** (not a silent per-record drop), and
that it surfaces as `tracing::error!(target: "duckdb.append", …)` with `record_rows_appended` skipped and
the broadcast emit suppressed. That mechanism is inherited as established, not re-litigated. What is NOT
inherited is whether it reproduces on the metrics builder — a different builder, a different key.

**This chunk is MEASURE-FIRST.** The qualifying measurement is end-to-end over the real OTLP path: emit
colliding metric points, then read back what the buffer holds. A schema reading demonstrates the
mechanism; it does not prove the loss.

**A premise correction remains a first-class good outcome.** If the metrics write path does not actually
collide, the finding dies and the chunk records that. Do not reshape the chunk to preserve the finding.

## The collision has TWO sources — both in scope

**(1) Label-set collision — structural, present since the table was minted.** `collect_metric_points`
(`crates/buffer/src/appender.rs:658`) walks all five OTLP data shapes (Gauge / Sum / Histogram /
ExponentialHistogram / Summary) and calls `push_metric_row` (`:764`) with exactly
`name, p.time_unix_nano, resource_hash, value, kind`. **`p.attributes` is never read** — verified by
sweeping every `.attributes` access in the file: `:133` and `:786`/`:800` are RESOURCE attributes (hashing),
`:403-407` are span-event exception attributes. There is no metric-point attribute read anywhere. So the
label set is discarded at the push site, and two label-differing points arrive at the appender as
byte-identical key tuples.

**(2) Redacted-name collision — added 2026-08-23 by `2026-08-23-ingestion-scrub-coverage`.**
`build_metrics_record_batch` now scrubs the metric name once per metric
(`crates/buffer/src/appender.rs:163`, `scrub_otlp_field(&m.name, …)`, deliberately hoisted above the point
loop since every point of one metric shares the name). Two DISTINCT credential-shaped metric names redact
to the same placeholder, so they collide at the same `ts_unix_nano` + `resource_hash`. That chunk stated
this as an accepted, LOUD failure.

The RED measurement must exhibit **both** pairs. Source (2) is the newer and less obvious of the two, and
nothing in the codebase demonstrates it today.

## Verified at HEAD (cbb316c) — hypotheses that survived re-derivation

Directive- and annotation-supplied coordinates were re-derived first-hand before entering this scope (the
2026-08-21 external-relay rule). All coordinates the working entry named survived; two counts are refined.

- `crates/buffer/src/schema.rs:77` — PK `(metric_name, ts_unix_nano, resource_hash)` on the named const
  (`CREATE TABLE IF NOT EXISTS metrics_points` at `:70`). **Confirmed verbatim.**
- `crates/buffer/src/schema.rs:174` — the SAME PK inside the live `SCHEMA_DDL` concat (`:167`).
  **Confirmed verbatim.** As with `log_records`, this is ONE table in TWO representations, not two tables,
  and `ddl_constants_match_concatenated_schema` (`crates/buffer/src/schema.rs:420`) forces the pair to
  agree — a PK edit cannot land one-sided. Not a finding; the plan must not build around it.
- **`crates/viz/src/query.rs:488` — REFINED, not falsified.** `:488` is the PK LINE; the fixture's
  `CREATE TABLE IF NOT EXISTS metrics_points` starts at `:481`. Same site, one coordinate sharpened.
- **"THREE sites" — VERIFIED and correctly counted this time.** A workspace-wide sweep for
  `CREATE TABLE … metrics_points` returns exactly three: `schema.rs:70` · `schema.rs:167` ·
  `viz/query.rs:481`. **`crates/triage/src/baseline/sql.rs` carries NO `metrics_points` fixture** — the
  fourth-site under-count that bit the `log_records` chunk is specific to that table and does not recur
  here. Checked explicitly rather than assumed.
- **"NO attributes/labels column at all" — VERIFIED.** Columns are `metric_name`, `ts`, `ts_unix_nano`,
  `resource_hash`, `value`, `data_point_kind`. Note `value` and `data_point_kind` are stored but correctly
  outside the key (payload, not identity).
- **The `seq` precedent — VERIFIED and reusable as-is.** `log_records` took `seq BIGINT NOT NULL` from a
  buffer-global `AtomicU64` (`crates/buffer/src/state.rs:32`, `reserve_log_seq_block` at `:124`), consumed
  once per batch at `crates/buffer/src/appender.rs:277`, with the viz fixture mirroring it via
  `next_log_seq()` (`crates/viz/src/query.rs:511`). PK became four columns (`schema.rs:93`, `:187`).
  This is the shape to reuse, not redesign.
- **`andromeda-pulse-0.3.0/verification-matrix.json` — all 22 caps swept; NONE covers ingestion identity,
  PK collision, or metric-point label fidelity.** P-075 is pooled by operator decision, P-076/P-077 are
  owned by later working entries. Confirmed against the operator directive.

**Verified gap in the inherited record — this chunk must derive the generalization itself.** The sibling's
`scope.md` listed "the report's statement on whether the fix generalizes to `metrics_points`" as in-scope,
and its `report.md` does not contain that statement (the sole `metric` occurrence, `report.md:44`, is the
quoted arch PK-convention text about `log_records`). The promised hand-off was not delivered, so the
generalization is established here first-hand rather than inherited. Recorded as a fact, not a grievance.

## Premise closure (P3 — each bullet VERIFIED or premise-corrected; see `research.md`)

- **The write path** — **VERIFIED**: production metrics writes go through `consumer::dispatch_batch`'s
  `Batch::Metrics` arm → `build_metrics_record_batch` → `append_table_traced(&guard, "metrics_points", rb)`
  (`crates/buffer/src/consumer.rs:126`) → `append_record_batch_to_table` (`appender.rs:535-556`), which is
  the SAME `conn.appender()` → `append_record_batch` → `flush()` helper every other table uses. The
  sibling's flush-time enforcement therefore transfers by construction. Note `append_metrics_batch`
  (`appender.rs:585`) is `#[cfg(test)]` only and is NOT the production path.
- **The failure mode reproduces on metrics** — **VERIFIED STRUCTURALLY; the behavioural leg is still
  owed.** The rejection path is identical and constant-valued: `dispatch_batch`'s `?` propagates to the
  consumer loop's `Ok(Err(e))` arm → `tracing::error!(target: "duckdb.append", reject_reason =
  %describe_error(&e), …)` (`consumer.rs:78-83`), with `record_rows_appended` (`consumer.rs:161`) skipped
  because it sits after the append. Nothing metric-specific short-circuits it. The RED measurement still
  runs — a shared code path is not a substitute for observing the behaviour — but it is no longer the
  premise most worth being wrong about.
- **Blast radius of a key change** — **VERIFIED, and NARROWER than stated on readers, WIDER on fixtures.**
  Every production reader names its columns or keys on `ts_unix_nano` alone (`SELECT_METRICS`
  `viz/query.rs:25`; `COUNT_METRICS` `:37`; retention `crates/buffer/src/retention.rs:26`), so an added
  ordinal is invisible to all of them. What the bullet under-counted: `push_metric_row` is called from
  **five** sites inside `collect_metric_points` (`appender.rs:671, 688, 705, 722, 739` — one per OTLP data
  shape), so threading the ordinal touches all five; and BOTH `viz` fixture `INSERT`s (`:526`, `:608`) name
  their columns, so a `NOT NULL` ordinal with no DEFAULT breaks both.
- **`crates/viz`'s DDL copy is structurally unguarded** — **NEWLY MEASURED, not anticipated in this
  scope.** `viz` does not depend on `buffer` (`crates/viz/Cargo.toml:9-17`), so it cannot import the DDL,
  and no test cross-checks the two. Of the three sites, only the `schema.rs` pair is pinned (by
  `ddl_constants_match_concatenated_schema`, `schema.rs:420`). The third must be edited by hand and nothing
  will catch a miss.
- **A duplicate-`INSERT` PK probe is NOT an available measurement** — VERIFIED: `crates/buffer/src/schema.rs`
  (~`:311`) documents that path as "observed to hang in libduckdb-sys 1.10502 on this build". The
  measurement must run through the Appender path, under an explicit timeout.

## The design tension — named here, DECIDED in the plan

**Identity half — the ordinal.** The `seq` shape generalizes mechanically and is the entry's own stated
expectation. It closes BOTH collision sources at once, and it satisfies the scrub-coverage chunk's
hand-off constraint ("whatever key shape this entry lands must keep it loud rather than reintroducing
silent loss") **by eliminating the loss rather than by keeping it loud** — with an ordinal in the key,
neither colliding pair is rejected, so there is no dropped row for a signal to be silent about. That
reading is stated explicitly here so it is not re-argued downstream.

**Label half — the entry itself calls this "the larger half".** The working entry's own title demands the
points survive *"with their labels"*, so the label question is inside the entry's remit by its own wording,
not an outside addition. It is nonetheless a materially different piece of work — it requires choosing a
representation (a JSON/`MAP` column vs a side table), and it touches the OTLP data-model coverage, the
`viz` response shape, and arch §Conventions.

**RESOLVED at P4 (operator-selected 2026-08-23): the label half CARRIES.** This chunk ships identity only.
Three options were presented — CARRY, storage-only-this-chunk (an `attributes` column written and scrubbed
but never SELECTed), and the full half through the `viz` response shape — and CARRY was selected. The
deciding evidence: P3 measured the identity half **fully separable** (the ordinal closes both collisions
without `p.attributes` ever being read), while the label half needs a representation decision spanning arch
§Occupied Resources, security's scrub remit and the `viz`/MCP response shape, with the arch extract warning
that a producer-less side table reproduces an already-named defect class. Consequence carried honestly: the
entry's stated outcome is **under-delivered until the follow-up lands**, and the CARRY must be minted at
wrap with this chunk's evidence attached (see the silent-loss paragraph below).

**A second decision, taken at the same gate: the `redactions_applied` fold moves IN-CHUNK.** This was not
in the entry and not in this scope as first written — P3 measured it. `state.record_redactions(…)` fires
*inside* all four builders (`crates/buffer/src/appender.rs:73`, `:184`, `:273`, `:468`), i.e. before
`append_table_traced` can fail, so a rejected batch counts cells that never persisted and obs-plan §5's
persisted-cells-only rule does not hold. Pre-existing and common to all four builders, so not this chunk's
defect — but this chunk discovers it and owns the harness that proves it, and the operator selected fixing
it here over CARRYing it to the Diagnostics sweep. The fold moves to each table's own post-append site.
**Sequencing consequence:** the RED measurement must run at HEAD *before* this fix, or the over-count can
no longer be witnessed.

The decision carries one consequence that must be stated wherever it lands: **the ordinal makes the label
loss SILENT — but only in the log, because it is already silent to the user.** P3 measured the
user-facing half and it corrects this scope's first framing. Today a label-differing pair fails the whole
batch loudly at ERROR *in the log sink*; the Metrics view sees nothing of it, because a rejected batch is
not a query error — `viz.query.metrics` succeeds and returns zero rows, so `MetricsRoute.tsx` falls to its
empty branch and renders "No metrics received yet" plus an exporter hint naming the very ports the user is
already exporting to successfully. The error-before-empty rule is correctly implemented and structurally
cannot help, since no rejection signal reaches the frontend.

So the fix does not convert a loud symptom into a silent one at the surface the user actually looks at —
that surface is misleading *today*, and the fix makes it correct (both rows land and render). What the fix
removes is the LOG-side ERROR. After it, both rows land carrying no labels and indistinguishable from one
another, with nothing anywhere announcing that a dimension was dropped. Nothing is newly lost — the labels
were never stored — but the last remaining signal goes quiet. If the label half becomes a CARRY, that CARRY
must be minted with this evidence attached.

## Capability claim posture

**Expect to claim NOTHING in the 0.3.0 matrix, and that is the correct outcome** (operator directive,
verified above by sweeping all 22). The coverage gate no-ops; proof lives in the tests and the report, as
at both preceding chunks. **Do not hunt for a capability to attach this to.** If P3 concludes a 0.3.0 entry
is genuinely warranted, surface it to the operator — never mint one silently.

## Folded annotations

- **PREREQ — `cargo audit` standing deferral (pin #7).** Ratified 2026-08-16, origin
  `2026-08-15-corpus-key-persistence`, every-3rd-wrap re-run INTERVAL. Probe points ran at sessions 25, 28,
  31, and **DISCHARGED at 34** (2026-08-23: probe RAN, exit read DIRECTLY as `1`, first diagnostic line
  reproduced byte-identical). **Next point is 37; this chunk's wrap is session 35 — BETWEEN points**, so
  the ratified form applies: record `probe skipped per ratified interval (next: 37)`, never a silent skip,
  and still re-verify basis + overlap. Basis is UNCHANGED and upstream (`parse error: duplicate advisory
  ID: RUSTSEC-2026-0244` — no repo change can clear it). Overlap `cargo deny check advisories` stands at
  **eight** owned upgradeable IDs — 0189/0190/0194/0195/0204/0222/0253 plus RUSTSEC-2026-0258 (`h2` 0.4.14,
  safe upgrade `>=0.4.16`) — owned by the Advisory-backlog entry and **never ignore-listed**. A
  probe-auto-satisfy SIGNATURE was recorded at 34 (exit 1 + that exact diagnostic line + overlap green at
  the eight).
- **NOTE (from the working entry) — do not conflate with the scrub chunk.** `metrics_points.metric_name`
  was named in "Ingestion scrub coverage" as an un-scrubbed column; that is a related table and an
  **unrelated defect**, and it COMPLETED 2026-08-23. What this entry inherits from it is one constraint,
  addressed under "The design tension" above.

## Boundaries

**In scope:** the `metrics_points` primary key and the `seq` ordinal the fix requires, in all three DDL
representations (`schema.rs` const + live concat, and the unguarded `viz/query.rs` fixture copy); the
end-to-end measurement that BOTH colliding pairs (label-set and redacted-name) survive — including the
small OTLP **producer** that measurement requires, modelled on the sibling's
`crates/ingest/examples/inject_colliding_logs.rs`; confirmation that the whole-batch failure mode
reproduces on the metrics builder before any fix; **the `redactions_applied` fold moving to each table's
post-append site across all four builders** (P4 decision above); regression coverage in `buffer`, including
a DIRECT unit assertion for both ordinal allocators so open trigger
`buffer-log-seq-allocator-unit-coverage` is closed rather than re-minted; a mutation check proving the new
pins discriminate; the generalization statement the sibling promised and did not deliver; the label-half
decision with its silent-loss consequence recorded.

**Out of scope:** **the label half's IMPLEMENTATION** — no attributes/labels column, no `viz` response-shape
change, no UI (P4 decision: CARRY, to be minted at wrap with this chunk's evidence); the `log_records` and
`spans` identity work (both settled); the span-side drop-observability CARRY (owned by the "Diagnostics
un-muting + harness-truth sweep" entry); the three producer-less tables (`resources`,
`instrumentation_scopes`, `span_links`) and their dead retention DELETEs (same sweep entry); a drift guard
for the `viz` fixture DDL (recorded for a future guard chunk — closing it would add a `viz → buffer`
dependency edge for a test fixture); the missing `MockMetricPoint` factory that test-plan §3 mandates
(surfaced for wrap to disposition); the Advisory-backlog upgrades (own entry); a 0.3.0 matrix mint
(operator-surfaced only); any corpus-side change — the corpus `pipeline_metrics` table is a
product-internal namespace in a DIFFERENT database and is explicitly not this table.
