# Report — 2026-08-23-ingestion-scrub-coverage

**Chunk:** Ingestion scrub coverage — the client-controlled DuckDB columns that never reach `scrub_attribute` stop storing host PII verbatim, with the treatment decided per column class (key/predicate columns vs free text) rather than uniformly
**Date:** 2026-08-23
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/buffer/src/appender.rs` (modified) — scrub coverage + counter folds + 8 in-crate tests
  - `crates/buffer/src/consumer.rs` (modified) — 3 call sites updated for changed signatures
  - `crates/security/src/scrubber.rs` (modified) — identity-preservation corpus (tests only)
  - `crates/ingest/examples/inject_scrub_canaries.rs` (**new**) — dev-only live-leg producer

- **Symbols / APIs:**
  - `buffer::appender::extract_service_name` — signature CHANGED `(Option<&Resource>) -> String` →
    `(Option<&Resource>, Option<&mut u64>) -> String`, and it now SCRUBS its return value.
    **All three production callers updated, none left behind**: `appender.rs:44` (spans column, passes
    `Some`), `appender.rs:359` (fingerprint observer, passes `None`), `consumer.rs:232` (baseline
    `SpanObserver` tap, passes `None`). `pub(crate)` — no cross-crate surface.
  - `buffer::appender::build_spans_record_batch` — signature CHANGED, gained `state: &BufferState`.
    **Callers: 1 production (`consumer.rs:108`) + 1 `#[cfg(test)]` wrapper + 3 in-crate tests — all
    updated.** `pub(crate)`.
  - `buffer::appender::build_metrics_record_batch` — signature CHANGED, gained `state: &BufferState`.
    **Callers: 1 production (`consumer.rs:127`) + 1 `#[cfg(test)]` wrapper — both updated.** `pub(crate)`.
  - No new public API, no IPC method, no endpoint, no port/socket, no env var, no export.

- **Crates / modules:** none added or removed. Changed: `buffer`, `security` (tests only), `ingest`
  (example only). The `buffer → security` dependency edge already existed — no manifest change.

- **Dependencies:** none added, none bumped. `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:** none. No DDL change, no migration, no config key, no violation-schema change.
  The four target columns keep their names, types and primary keys.

- **Spec-master edits:** none by implement (implement is read-only on specs). Expected amendments are
  listed under Outcome → Expected amendments (wrap).

- **Counts / qualifiers moved:** **YES.** The scrubber-coverage gap was documented as **five** client-
  controlled columns; measured at HEAD it is **four live + one vacuous**. Docs stating the five-count:
  `security-plan.md` §Security Anti-Patterns → Logging (line 427, the MEASURED-reality clause) and the
  chunk's own working-route entry CONTEXT (frozen at promotion — historical record, correctly left as-is).
  Separately, the catalog count (**8** categories) is UNCHANGED by this chunk — no arm added, reordered
  or retuned.

- **Dev-tool versions:** none — no external CLI installed or upgraded.

- **Reverted / negative API facts:** none. (One temporary mutation of the four scrub sites was applied and
  fully restored as a verification act — see Outcome; no surface was written-then-reverted.)

- **Spec claims disproved by measurement:** **three.**
  1. **"five client-controlled columns"** — stated at `security-plan.md:427` and in the working-route
     entry. Measured FALSE: `append_record_batch_to_table` (the only DuckDB write path in `crates/buffer`)
     is called for exactly four tables — `spans`, `metrics_points`, `log_records`, `span_events`
     (`appender.rs:534/557/580/604` + the generic `consumer.rs:182`). `instrumentation_scopes` has **no
     producer**, so its two columns cannot leak and cannot be scrubbed. Live target set is **four**.
  2. **`spans.service_name` has a single write path** — implied by the entry's coordinate chain
     (`extract_service_name :45 → push :58 → StringArray :74 → column :83`). Measured FALSE via the
     code-graph: that extractor has **three** production call sites, and two never reach a column
     (`appender.rs:359` → `FingerprintObserver`, `consumer.rs:232` → baseline tap). Scrubbing the column
     alone would have left two raw paths into `triage` and desynchronised DuckDB identity from the
     baseline registry.
  3. **`resources` is in the same never-written state** (measured this wrap, adjacent to #1): declared in
     `schema.rs:16`, swept by `retention.rs:28`, never written. So **two of the seven retention DELETEs
     sweep permanently-empty tables**. Not stated as a claim anywhere yet — arch §Occupied Resources
     enumerates both tables as reserved without qualification, which is the site that should say it.

  *(A fourth premise, supplied in the phase directive rather than by a spec — "`metric_name` is the lookup
  key in corpus" as a reason scrubbing is risky — also measured false: that corpus column holds the app's
  OWN pipeline metric names (`save_pipeline_metric("drain_template_tree","l1c")`) in a separate SQLite
  database, not the client-controlled `metrics_points.metric_name`. Recorded for completeness; no spec
  states it, so nothing needs disposition.)*

- **Coverage of new surfaces:**
  - `spans.service_name` (write boundary) → validation n/a · instrumentation ✓ (`redactions_applied` folded once per batch) · PII **redacted✓** · tests unit✓ (in-crate, real DuckDB) + wire✓ · a11y n/a · tokens n/a
  - `span_events.name` (write boundary) → validation n/a · instrumentation ✓ · PII **redacted✓** · tests unit✓ + wire✓ · a11y n/a · tokens n/a
  - `metrics_points.metric_name` (write boundary, PK member) → validation n/a · instrumentation ✓ · PII **redacted✓** · tests unit✓ + wire✓ · a11y n/a · tokens n/a
  - `log_records.severity_text` (write boundary) → validation n/a · instrumentation ✓ · PII **redacted✓** · tests unit✓ + wire✓ · a11y n/a · tokens n/a
  - `crates/ingest/examples/inject_scrub_canaries.rs` (dev-only producer, not a product surface) → validation n/a · instrumentation n/a · PII n/a (emits synthetic canaries only) · tests — exercised as the live leg · a11y n/a · tokens n/a

## Deviations from intent

1. **`crates/buffer/src/consumer.rs` was modified; the plan predicted "not modified."** The choke-point
   signature change forced 3 call-site edits there. *Justification:* the file was on the plan's
   Files-to-modify list (annotated as conditional), so this is in scope; and it was unavoidable — the
   entire point of scrubbing inside `extract_service_name` is that all three consumers share one identity.
   Leaving `consumer.rs` unchanged was not an option once the signature moved.

2. **Counter threading resolved as `Option<&mut u64>`, counting only persisted cells.** Plan step 1
   explicitly deferred the signature shape to step 4 without prescribing it. *Justification:* the
   span-events builder's `service_name` is never written to a column (that table has no such field), so
   counting it would inflate `redactions_applied` beyond redactions that actually protected stored data.
   Measured consequence: the wire smoke shows exactly **4**, not 5.

3. **Direct-binary smoke instead of the plan's `scripts/agent-run.sh boot`.** *Justification:* that verb
   runs `cargo run --bin pulse-app --release` (a full release rebuild on top of an already-built debug
   workspace) and exports its own `ANDROMEDA_PULSE_DATA_DIR`, so it can serve neither a debug-build proof
   nor a caller-chosen fresh data dir. The Direct-binary smoke variant is the form `test-plan.md` §3
   sanctions for exactly this shape.

4. **Prefixed the workspace nextest with `cargo build --workspace --tests --jobs 4`.** *Justification:*
   the documented host remedy for the ~30-binary parallel-compile OOM and the cold-build rlib race. The
   plan's nextest line itself ran verbatim.

5. **Ran a mutation check (not in the plan).** *Justification:* the plan's step 8 asked for a RED half
   "so the GREEN half is known to discriminate"; running RED against the pre-fix build was impossible once
   the fix had landed, so the equivalent discharge was to neutralize the four scrub sites, confirm the
   pins go red, and restore. Recorded under Outcome.

## Decisions & corrections

- **Operator decision (phase P4): scrub all four uniformly**, accepting the `metrics_points.metric_name`
  PK-collision risk. Rationale recorded: a collision needs two *distinct* credential-shaped metric names
  redacting identically at the same `ts_unix_nano` + `resource_hash`; the failure is loud (ERROR at
  `flush()`, whole batch) rather than silent, and no corpus lookup is affected. Rejected alternatives: a
  collision-safe discriminator (mints a stable derived identifier of the secret) and leaving `metric_name`
  raw (leaves the named gap open so the posture clause could not flip).
- **Operator decision (phase P4): `instrumentation_scopes` dispositioned documented-vacuous, zero code.**
  Scrubbing a column nothing writes ships an emitter that never fires.
- **Correction to my own plan, caught by the phase P5 mechanical gate:** the live leg had been authored as
  `SCENARIO=scrub-canary agent-run.sh run`. No `SCENARIO` support exists in the harness and `run` is the
  nextest suite, not an injector — so the four log-artifact acceptance criteria had no producer. Replaced
  with the real firing form plus a new producer example.
- **Design rule established this chunk:** `redactions_applied` counts redactions applied to **persisted
  cells** only. A scrub applied purely for identity consistency on a non-stored path is deliberately
  uncounted.
- **`resources` finding recorded, not actioned** — same never-written state as `instrumentation_scopes`;
  out of this entry's scope, surfaced for disposition.

## Outcome

**Acceptance criteria: met.** All four covered columns reject a credential-shaped canary and preserve
legitimate values byte-identical; identity preservation proven for service names, metric names, the
`exception` event name and the standard severity vocabulary; the `cargo audit` PREREQ ran.

**Gates green** (commands run):
- `cargo fmt --check` ✓
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓
- `cargo nextest run --workspace --profile ci` → **1859 passed, 1 skipped** (baseline 1835 → +24, exactly
  the 8 buffer tests + 16 security rstest cases added)
- `cargo nextest run -p buffer` + `-p security` → **207/207** (in-crate, per test-plan §4)
- `cargo xtask capability-drift` → clean · `cargo xtask capability-widening-check` → clean (0/3)
- `cargo deny check bans licenses sources` → **ok** (pass/fail gate)
- `cargo deny check advisories` → designed-RED at exactly the **8 owned IDs** (0189 / 0190 / 0194 / 0195 /
  0204 / 0222 / 0253 / 0258) — **stable, no new finding**
- `cargo audit` → **PROBE RAN** (PREREQ pin #6, ratified point 34). Basis reproduced byte-identical and
  upstream: `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`.
  No repo change can clear it; the deferral stands correctly ratified.

**No gate deferrals** — the chunk carries Rust delta, so the full workspace clippy and nextest both ran
rather than riding the source-delta-proportional rule. Webview gates correctly omitted (zero
`pulse-app/ui/**` paths touched — verified against the trigger list itself, not the plan's prose).

**Mutation check (discrimination proof):** neutralizing all four new scrub sites turned **5 pins RED** —
the four column recall pins plus `redaction_counter_folds_once_per_batch_across_all_four_builders` —
and restoring returned them to green. The identity-preservation pins correctly stayed green under the
mutation (they guard over-redaction, not under-redaction). It also showed the pre-existing
`build_spans_record_batch_extracts_service_name_from_resource` test passes under BOTH semantics,
confirming the old suite never exercised the scrub path on these columns.

**Smoke: PASS and discriminating** (Direct-binary variant; boot-path unchanged, so this is the chunk's own
wire proof rather than a triggered gate). Feed precondition asserted BEFORE any absence claim:
`rows_ingested` **0 → 6**, `redactions_applied` **0 → 4**, `span_events_seen` **0 → 2**. All four tables
appended **2 rows each** (`spans` / `span_events` / `metrics_points` / `log_records`), so every canary AND
every control landed — no PK collision. Canary literal and the bare `sk_live_` format each appear **0
times across 21,846 log lines**; **0** `app.panic.fatal`, **0** ERROR, **0** PK/constraint violations; 6
tick families healthy; clean specific-PID shutdown with both `:4317`/`:4318` released and no orphan
process. The counter landing on exactly 4 (not 5) is itself the confirmation of the persisted-cell
counting rule.

**Capabilities claimed: none** (third chunk running) — verified at phase: `P-047` lives in the v0.2.0
ledger and is absent from the 0.3.0 matrix; the three pooled 0.3.0 caps (P-075 / P-076 / P-077) none cover
ingestion scrubbing. Matrix untouched; version coverage stays 19/22.

**Expected amendments (wrap):**
- `security-plan.md` §Security Anti-Patterns → Logging — POSTURE FLIP: the MEASURED-reality clause moves
  from "does NOT yet hold for five columns" to holding for the four live columns; the enumeration is
  corrected to four-live-plus-one-vacuous, recording that `instrumentation_scopes` has no producer.
- `obs-plan.md` §5 (and the §1 / §8 restating sites, per the three-site rule) — `redactions_applied`'s
  registered scope widens from the `scrub_otlp_field` log/exception paths to all four appender builders,
  since the spans and metrics builders gained the fold they previously lacked. Field set is UNCHANGED
  (no new tick field), so no allowlist leaf edit is required.
- `test-plan.md` §1 — `buffer-redaction-counter-unit-coverage` CLOSES: the new
  `redaction_counter_folds_once_per_batch_across_all_four_builders` asserts the fold arithmetic, that all
  four builders contribute to one tally, and that the fold stays separate from `record_feed_counts`.
- `architecture.md` §Occupied Resources (DuckDB reserved tables) — record once, in the master that owns
  the schema, that `resources` and `instrumentation_scopes` are declared and retention-swept but have no
  producer (measured 2026-08-23).
