# Scope — 2026-08-22-log-records-identity

**Version:** andromeda-pulse-0.3.0 · **Epoch:** Epoch 4 — Polish & ship: verification
**Working entry:** "log_records identity — two log records arriving in the same tick at the same severity both survive ingestion"

## The claim under test

The `log_records` primary key is `(ts_unix_nano, resource_hash, severity_number)`. None of those three
columns distinguishes two DIFFERENT log records emitted by the same resource, in the same nanosecond tick,
at the same severity — the overwhelmingly common shape for a burst of `INFO` lines from one service. The
claim is that the second record **does not survive ingestion**, and does so **silently**.

**This chunk is MEASURE-FIRST.** The claim is about SURVIVAL THROUGH INGESTION, so the qualifying
measurement is end-to-end: emit two distinct log records over the real OTLP path that collide on all three
key columns, then read back what the buffer holds. A schema reading demonstrates the MECHANISM; it does not
prove the loss. Both legs are required and neither substitutes for the other.

**A premise correction is a first-class good outcome.** If the write path never actually collides (batching,
timestamp resolution, an upsert semantic, or a discriminator applied upstream), the finding dies and the
chunk records that instead. Do not reshape the chunk to preserve the finding — the 2026-08-15 tier-1
investigation and the 2026-08-14 fingerprint-feed chunk both closed exactly this way, honourably.

## Verified at HEAD (70344d5) — hypotheses that survived re-derivation

Directive- and annotation-supplied coordinates were re-derived first-hand before entering this scope
(per the 2026-08-21 external-relay rule). One working-entry premise is FALSIFIED and corrected here.

- `crates/buffer/src/schema.rs:92` — PK `(ts_unix_nano, resource_hash, severity_number)` on
  `const CREATE_LOG_RECORDS` (declared `:81`, carrying `#[allow(dead_code)]`). **Confirmed verbatim.**
- `crates/buffer/src/schema.rs:185` — the SAME PK inside `const SCHEMA_DDL` (declared `:134`, a `concat!`
  of the eight table consts). **Confirmed verbatim.**
- **FALSIFIED — "the SAME PK shape recurs at `:185`, so check both tables."** There is **one** table with
  **two representations**, not two tables. `:81` is the named const; `:134` is the live concatenated DDL
  that the buffer actually executes. `ddl_constants_match_concatenated_schema`
  (`crates/buffer/src/schema.rs:380-396`) asserts `SCHEMA_DDL` equals the concatenation of the eight table
  consts, so a PK edit **must** land in both or the test goes red. The hand-sync hazard the entry implies is
  **already closed by a shipped test**; it is not a finding and the plan must not build around it.
  (Operator directive 2026-08-22, independently re-derived this run.)
- **Full PK inventory, both representations** (16 `PRIMARY KEY` lines = 8 tables x 2): `spans`
  `(trace_id, span_id)` · `span_events` `(..., event_index)` · `span_links` `(..., link_index)` ·
  `metrics_points` `(metric_name, ts_unix_nano, resource_hash)` · `log_records`
  `(ts_unix_nano, resource_hash, severity_number)` · `resources` `(resource_hash)` ·
  `instrumentation_scopes` `(resource_hash, scope_name, scope_version)` · `log_templates` `(template_id)`.
  `resources` and `instrumentation_scopes` are dedup-by-design and correct. The span family carries
  OTLP-native ids and is correct by construction.
- **`andromeda-pulse-0.3.0/verification-matrix.json` — all 22 caps swept (P-061 through P-082); NONE covers
  log-record identity, PK collision, or ingestion-time record loss.** The three unclaimed (P-075/P-076/P-077)
  are owned elsewhere and none of them fits. Confirmed against the operator directive.

**Found during scope re-derivation, not carried by the directive — adjacent, NOT taken into scope:**

- `metrics_points` PK `(metric_name, ts_unix_nano, resource_hash)` omits the point's ATTRIBUTES, so two
  data points of one metric differing only by label set, in the same tick from one resource, are the SAME
  same-class collision. **Not in scope** (this entry names `log_records`), but it is the natural sibling and
  the plan should say plainly whether the chosen fix generalizes, so a future entry can be minted with
  evidence rather than re-derived from scratch.

## Premise closure (P3 — each bullet VERIFIED or premise-corrected; see `research.md`)

- **The failure mode** — `[premise-corrected: the entry's "silently drops [the second record]" is falsified
  in SHAPE. The loss is WHOLE-BATCH and it IS logged — an appender error propagates via `?` out of
  `dispatch_batch` (`crates/buffer/src/consumer.rs:143`) and surfaces as
  `tracing::error!(target: "duckdb.append", reject_reason = …)` at `:78-83`, with `record_rows_appended`
  skipped and the broadcast emit suppressed. What remains open is whether the Appender raises at all —
  research Open question 1, and the measure-first leg's first task.]`
- **The write path** — **VERIFIED**: the Arrow `Appender` (`conn.appender` → `append_record_batch` →
  `flush`, `crates/buffer/src/appender.rs:494-507`), not a prepared `INSERT`.
- **Blast radius of a key change** — `[premise-corrected: WIDER on DDL, NARROWER on readers. FOUR
  `log_records` DDL sites exist, not two — `schema.rs:82` (const) · `schema.rs:175` (live) ·
  `crates/triage/src/baseline/sql.rs:877` · `crates/viz/src/query.rs:490` — and the sync test guards only
  the schema.rs pair; the two out-of-crate copies are test-only, already-divergent minimal subsets that
  both carry the OLD PK. Every PRODUCTION reader is safe: `viz` names its columns and already omits the
  nullable `template_id`, MCP `query_logs` delegates to `viz::query::query_logs`, and retention keys on
  `ts_unix_nano` alone.]`
- **The architectural consequence** — **VERIFIED, and already inaccurate at HEAD independent of this
  chunk**: `log_records` has no `name` column at all (`ts`, `ts_unix_nano`, `resource_hash`,
  `severity_number`, `body`, `severity_text`, `trace_id`, `span_id`, `template_id`), so arch §Conventions
  *Primary key convention* is stale as written. Wrap-time drift amendment, not a phase edit.

**Newly surfaced by research, not anticipated here:** a duplicate-INSERT PK probe is a **documented hang**
on this build (`crates/buffer/src/schema.rs:311-318` — "observed to hang in libduckdb-sys 1.10502 … not
behavioral PK enforcement"), so behavioural PK enforcement has never been verified anywhere in this
codebase and the obvious measurement is the one that wedges. The measurement must run through the Appender
path under an explicit timeout.

## The design tension — named here, DECIDED in the plan

Three directions were named; **P3 eliminated one on evidence and the fork is now (a) vs (b)**:
(a) **extend the key** with a per-batch discriminator (a sequence/index column) — the `event_index` /
`link_index` precedent is not merely conventional but **live in the same builder**
(`crates/buffer/src/appender.rs:354`, `:383`, `:448`, with a monotonicity test at `:1527`);
(b) **drop the primary key** on the grounds that an in-memory ring buffer needs no uniqueness constraint and
the PK buys nothing the retention sweep does not; ~~(c) widen the key with `body` / `trace_id` / `span_id`~~
— **ELIMINATED** from two independent directions: keying on `body` would make record identity depend on
scrubber output (`extract_log_body` → `scrub_otlp_field` is live at `appender.rs:290-307`), and untraced log
records carry empty `trace_id`/`span_id`, so widening with them does not separate the colliding case at all.

**RESOLVED at P4 (operator-selected 2026-08-22), and option (a) SPLIT in two on the way.** Composing the
question surfaced that a *per-batch* ordinal — the literal `event_index` mirror — closes only half the class:
`event_index` is safe because its key carries `(trace_id, span_id)`, a unique parent scoping the ordinal,
whereas `log_records` has no unique parent, so two same-nanosecond records arriving in DIFFERENT export
batches would both take index 0 and still collide. The fork was therefore presented three ways:

- **(a1) monotonic sequence column — SELECTED.** `seq BIGINT NOT NULL` from a buffer-global counter; PK
  becomes `(ts_unix_nano, resource_hash, severity_number, seq)`. Closes both the intra- and cross-batch
  halves; an ordinal, not a UUID/random surrogate, so arch's convention holds.
- **(a2) per-batch ordinal — REJECTED**: fixes the common case, not the class.
- **(b) drop the primary key — REJECTED**: would close the class, but leaves `log_records` the only one of
  8 tables without identity and makes true duplicate re-sends permanently undetectable.

Also operator-selected: the two out-of-crate fixture DDLs (`viz/query.rs:490`,
`triage/baseline/sql.rs:877`) **are updated** to the fixed key, so those suites stop exercising the old
identity. Both decisions remain downstream of research Open question 1 — if the Appender does not enforce
the PK, there is no defect and no option is taken.

## Capability claim posture

**Expect to claim NOTHING in the 0.3.0 matrix, and that is the correct outcome** (operator directive,
verified above by sweeping all 22). The coverage gate no-ops; proof lives in the tests and the report, as at
`2026-08-22-pii-scrubber-recall`. **Do not hunt for a capability to attach this to.** If P3 somehow concludes
a 0.3.0 entry is genuinely warranted, surface it to the operator — never mint one silently.

## Folded annotations

- **PREREQ — `cargo audit` standing deferral (pin #5).** Ratified 2026-08-16, origin
  `2026-08-15-corpus-key-persistence`, every-3rd-wrap re-run INTERVAL; probe points ran at sessions 25, 28,
  31. Session 32 was BETWEEN points and recorded `probe skipped per ratified interval (next: 34)`.
  **This chunk's wrap is session 33 — also between points**, so the same ratified form applies and the
  probe itself is due at 34. Basis UNCHANGED and upstream (`parse error: duplicate advisory ID:
  RUSTSEC-2026-0244` — no repo change can clear it). The wrap must still **re-verify basis + overlap** and
  record it explicitly, never silently. Overlap `cargo deny check advisories` stands at **eight** owned
  upgradeable IDs — 0189/0190/0194/0195/0204/0222/0253 plus RUSTSEC-2026-0258 (`h2` 0.4.14, safe upgrade
  `>=0.4.16`) — owned by the Advisory-backlog entry and **never ignore-listed**.
- **Lineage — intake #11, relayed 2026-08-21.** "The 2026-08-15 identity class landing on a second table."
  The FIRST table in that class is `spans`: a span rejected on the `(trace_id, span_id)` PK never reaches
  the fingerprint observer, so an id-colliding producer reads exactly like a dead feed (measured in both
  directions 2026-08-16). **That observability half is owned by the "Diagnostics un-muting + harness-truth
  sweep" working entry as an explicit CARRY and is NOT absorbed here.** This chunk owns the `log_records`
  identity itself. If the fix here suggests a shared drop-counter shape, say so for the sweep to consume —
  do not build the sweep's half.

## Boundaries

**In scope:** the `log_records` primary key and whatever discriminator the fix requires, in both DDL
representations; the end-to-end measurement that two colliding records both survive — including the small
OTLP **producer** that measurement requires (`crates/ingest/examples/inject_colliding_logs.rs`, modelled on
the existing `inject_demo` example; added at P5 val-1, which found the scope named the measurement but not
its means); the failure-mode determination that precedes it; the two out-of-crate fixture DDLs
(`viz/query.rs:490`, `triage/baseline/sql.rs:877`) per the P4 operator selection; regression coverage in
`buffer`; the report's statement on whether the fix generalizes to `metrics_points`.

**Out of scope:** the `metrics_points` sibling collision (adjacent, named for a future entry); the span-side
drop-observability CARRY (owned by the diagnostics sweep); the five un-scrubbed ingestion columns (owned by
the "Ingestion scrub coverage" entry, tail position 2); the Advisory-backlog upgrades (own entry); a 0.3.0
matrix mint (operator-surfaced only); any corpus-side or `log_templates` schema change beyond what the
`template_id` relation forces.
