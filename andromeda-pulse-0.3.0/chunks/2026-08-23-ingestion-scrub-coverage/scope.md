# Scope — Ingestion scrub coverage

**Marker:** `2026-08-23-ingestion-scrub-coverage`
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Promoted:** 2026-08-23

---

## The working-route entry (verbatim intent)

> Ingestion scrub coverage — the five client-controlled columns that never reach `scrub_attribute`

CONTEXT (from the entry): measured 2026-08-22 at the `2026-08-22-pii-scrubber-recall` chunk. A **COVERAGE**
gap, distinct from the RECALL gap that chunk closed — these columns never reach the scrubber at all, and the
DuckDB ring buffer is unencrypted, so PII placed in them by an instrumented host app is stored and read in
plaintext.

SCOPE (from the entry): `security-plan.md` §Security Anti-Patterns → Logging was amended AS MEASURED at that
wrap and **NAMES THIS ENTRY as the owner of the impl half**, so this entry is load-bearing for that amendment,
not optional cleanup. `span_events.exception_type` is a separate DELIBERATE exclusion (class identifier, not
user content) and is **NOT in scope**.

NOTE (from the entry): operator-decided 2026-08-22 as a standalone entry rather than a CARRY on the
diagnostics sweep — a security-grade coverage hole should not inherit a housekeeping entry's priority.

---

## What this chunk builds

PII scrubbing reaches the client-controlled DuckDB ring-buffer columns that currently bypass it — with the
**treatment decided per column class and justified**, not applied uniformly by default.

The deliverable is threefold:
1. **Coverage** — the columns a host app can fill with arbitrary text stop being stored verbatim.
2. **A stated per-class treatment rule** — identifier/key columns and free-text columns are handled by a rule
   the plan argues for, because the naive treatment has a measured failure mode (below).
3. **A measured RED→GREEN proof on the real OTLP path** — matching the precedent both immediately preceding
   chunks set (`pii-scrubber-recall`, `log-records-identity`): assert the leak before the fix, assert its
   absence after, at the wire rather than in a unit fixture alone.

## Verified target set (re-derived at HEAD, 2026-08-23)

The entry says "five columns". **Measured at HEAD the live target set is FOUR**; the fifth is vacuous.

| # | Column | Class | Write site (verified) | Consumer significance (verified) |
|---|---|---|---|---|
| 1 | `spans.service_name` | **identifier / key** | `appender.rs` — `extract_service_name(rs.resource…)` → `service_names.push` → `StringArray::from` → `Field::new("service_name")`; no `scrub_otlp_field` on the path. **[premise-corrected 2026-08-23: this is ONE of THREE consumers of a single extractor, not a single write path. `extract_service_name` (`appender.rs:107`) is called at `appender.rs:44` (→ this column), `appender.rs:359` (→ NOT a column — `span_events` has no `service_name` field; it feeds `FingerprintObserver::on_fingerprint` at `:425`) and `consumer.rs:232` (→ `SpanObserver::observe_span`, the baseline tap). Scrubbing the column alone leaves two raw paths into `triage` AND makes DuckDB disagree with the baseline/registry identity — see research.md F2]** | `GROUP BY` in Q1, Q2, Q5; `GROUP BY` **and** a JOIN-side comparison predicate (`caller.service_name != callee.service_name`) in Q4 — `crates/triage/src/baseline/sql.rs`. Also reconstructed from a composite key by `BaselineState::iter_operations` (`key.split('/').next()`, `baseline/mod.rs`). Reaches the webview via `viz::query::SELECT_TRACES` → constellation dot labels + table Service column |
| 2 | `span_events.name` | **predicate column** (not pure free text) | `appender.rs::build_span_events_record_batch` — `names.push(event.name.clone())`; `Field::new("name", …)` | `WHERE name = 'exception'` gates Q3_EXCEPTION_FINGERPRINTS (`baseline/sql.rs`) — the storm/fingerprint feed |
| 3 | `metrics_points.metric_name` | **identifier / key** | `appender.rs` — `metric_names.push(name.to_string())` → `StringArray::from` → `Field::new("metric_name")` | Corpus lookup key: `SELECT payload FROM pipeline_metrics WHERE metric_name = ?1 AND layer = ?2`, and `GROUP BY metric_name, layer` in the purge query (`crates/corpus/src/contract.rs`) |
| 4 | `log_records.severity_text` | **bounded-vocabulary text** | `appender.rs` — `severity_texts.push(log.severity_text.clone())`; `Field::new("severity_text", …)` | **VERIFIED 2026-08-23**: SELECTed by `viz::query::SELECT_LOGS` into `LogItem.severity_text` and rendered in the Logs view. Not in any PK. (The a11y not-color-alone precedent from 2026-06-10 is on the incident `ReportRenderer` path, which reads the `triage` severity enum — a different source than this column.) |
| 5 | ~~`instrumentation_scopes.scope_name` / `.scope_version`~~ | **VACUOUS at HEAD** | **none — the table has no producer** | — |

**Finding 5 (proven, not inferred):** `append_record_batch_to_table` is the only DuckDB write path in
`crates/buffer`, and it is called for exactly four tables — `spans`, `metrics_points`, `log_records`,
`span_events`. `instrumentation_scopes` appears only in DDL (`schema.rs`, both representations) and in a
retention `DELETE` (`retention.rs`). Nothing writes it, so nothing can leak through it, and a scrub attached
there would be an emitter that never fires (the producer-never-ran vacuous class — CLAUDE.md session-learning
2026-08-21). Its **disposition is a scope question for the plan**, not silent inclusion or silent omission.
`resources` is in the same never-written state (noted, out of scope).

## The design tension this chunk must resolve (not assume)

`scrub_attribute`'s stated posture is **recall over precision** — "false positives are acceptable
(over-redaction); false negatives leak secrets" (`crates/security/src/scrubber.rs`, verified). That stance was
written for **values**. Three of the four live targets are **keys or predicates**:

- A false positive on `spans.service_name` does not merely over-redact one cell — it **forks one service into
  two identities** across every `GROUP BY service_name` aggregate, the Q4 self-join, and the composite-key
  parse in `iter_operations`.
- A false positive on `metrics_points.metric_name` breaks the corpus `WHERE metric_name = ?1` lookup for that
  series.
- A false positive on `span_events.name` could disturb the `WHERE name = 'exception'` predicate that feeds the
  storm/fingerprint path.

So **"apply the same scrub everywhere" is a choice this chunk must argue for, not a default**. The plan must
state the treatment per column class and why. The candidate treatments (uniform scrub / key-safe
reject-not-rewrite / bounded-vocabulary validation / drop-and-count) are P4's decision; this scope fixes only
that the decision must be made explicitly and recorded.

**Research sharpened the tension into two decisive facts (research.md F1/F2):**
- **Only `metrics_points.metric_name` sits in a PRIMARY KEY** — `(metric_name, ts_unix_nano, resource_hash)`.
  The other three targets are outside their tables' keys. Two distinct metric names redacting to the same
  `[REDACTED:{category}]` string at the same ts + resource_hash therefore COLLIDE, and the preceding chunk
  measured that a colliding pair fails the **ENTIRE batch** at `flush()`. This is the chunk's sharpest
  constraint and the one genuine fork.
- **`service_name` has a single choke point** — scrubbing inside `extract_service_name` covers all three
  consumers consistently; scrubbing at the column alone splits identity between DuckDB and `triage`.

## Boundaries (what this chunk does NOT do)

- **Not** `span_events.exception_type` — a deliberate exclusion (class identifier, not user content), per the
  entry and `security-plan.md` §Logging.
- **Not** a change to the 8-category catalog — the RECALL gap was closed by `2026-08-22-pii-scrubber-recall`;
  this is the COVERAGE gap. No new pattern arms unless the per-class treatment demands one.
- **Not** the corpus / Drain / BaselineState / ServiceRegistry / RetryStormState persist paths — already
  covered (`security-plan.md` §Logging enumerates them).
- **Not** encrypting the DuckDB ring buffer — the plaintext-at-rest property is the *reason* the gap matters,
  not the thing being fixed here.
- **Not** the spec amendment — `security-plan.md` §Logging already names this entry as the impl owner, so the
  wrap's amendment is a **posture flip** (MEASURED-reality clause → holds), not new prose. Phase is read-only
  on specs.

## Capability claim expectation

**Claims nothing — third chunk in a row.** Verified at HEAD: `P-047` (the PII-scrubber capability) lives in
the **v0.2.0** ledger and is absent from `andromeda-pulse-0.3.0/verification-matrix.json`; the three unclaimed
0.3.0 caps are P-075 (Conductor e2e), P-076 (Integration UX e2e), P-077 (demo injector), none of which covers
ingestion scrubbing, and no 0.3.0 capability mentions scrub / PII / redaction at all. The chunk's proof
obligation therefore lives in its own tests + smoke, not in a matrix row.

## Folded annotations

**PREREQ (from the entry) — `cargo audit` re-check.** Standing deferral since
`2026-08-15-corpus-key-persistence`, ratified at the 2026-08-16 0-pending adaptation wrap as **pin #6** with an
every-3rd-wrap re-run INTERVAL. Probe points ran at sessions 25 / 28 / 31; sessions 32 and 33 were both
between points and each recorded `probe skipped per ratified interval (next: 34)`. **This session is 34 — the
probe is DUE and must actually run this chunk**, not be skipped again. Basis to re-verify: the RustSec DB
cannot LOAD (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`), which no repo change can clear; the
named overlap signal is `cargo deny check advisories`, last re-derived STABLE at **eight** owned upgradeable
IDs (0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 + RUSTSEC-2026-0258 `h2` 0.4.14, safe upgrade `>=0.4.16`),
owned by the separate **Advisory backlog** entry and never ignore-listed.

No `CARRY:` and no `BLOCKED-ON:` annotation on this entry.

## Touchpoints (anticipated — plan refines)

- `crates/buffer/src/appender.rs` — the four record-batch builders + `extract_service_name` (`:107`, the
  three-consumer choke point) + `scrub_otlp_field` (`:319`, the existing boundary helper) + the two
  `redactions_applied` folds (`:240`, `:432`). **Note (research F-set): the spans and metrics builders have NO
  fold today** — covering them adds counter plumbing those two lack.
- `crates/buffer/src/consumer.rs` — only if the treatment lands at call sites instead of the choke point.
- `crates/security/src/scrubber.rs` — **VERIFIED conditional**: needed only if the treatment requires a key-safe
  entry point. The `buffer → security` crate edge already exists (26 `scrub_attribute` refs, `buffer` among
  them), so no manifest change either way.
- `crates/buffer/src/state.rs` / `contract.rs` — **VERIFIED not-required**: `redactions_applied` already exists
  and is folded once per batch (landed as CARRY #10 at the previous chunk).
- `crates/ingest/examples/inject_scrub_canaries.rs` — **NEW, added at P5 val-1 as intent-incomplete**: this
  scope's own acceptance shape demands a RED→GREEN proof "on the real OTLP path", which requires a producer to
  put canaries on the wire. `agent-run.sh run` is the nextest suite, not an injector, so the live leg needs a
  dedicated dev-only example following the `inject_colliding_logs.rs` precedent the previous chunk set in the
  same directory. Dev-only; ships no product surface.
- Consumer-side read paths (`crates/triage/src/baseline/sql.rs`, `crates/corpus/src/contract.rs`,
  `crates/viz/src/query.rs`) — read for the consumer map; **not expected to change**.

## Acceptance shape (plan concretizes)

- A credential-shaped value placed in each covered column by a host app does not reach DuckDB verbatim —
  asserted RED before the fix and GREEN after, on the real OTLP path.
- The per-class treatment is stated and its identity-preservation property is asserted for key columns (a
  legitimate service/metric name is not forked or altered).
- `cargo audit` probe RUN and its result recorded (PREREQ, pin #6, point 34).
- Workspace gates green; the smoke run is discriminating (a feed precondition asserted before the absence
  claims, so an absence cannot pass for the wrong reason).
