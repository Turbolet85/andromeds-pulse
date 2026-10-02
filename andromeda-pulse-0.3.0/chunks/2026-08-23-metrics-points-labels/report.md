# Report — 2026-08-23-metrics-points-labels

**Chunk:** metrics_points labels — a metric point's label set survives ingestion and reads back, so dimensioned metrics stop arriving indistinguishable
**Date:** 2026-08-23
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:** `crates/buffer/src/schema.rs` (+2) · `crates/buffer/src/appender.rs` (+257/−…) · `crates/viz/src/query.rs` (+87) · `crates/ingest/examples/inject_colliding_metrics.rs` (+57) · `pulse-app/ui/src/bindings/index.ts` (regenerated) · `pulse-app/ui/src/dashboard/routes/MetricsRoute.test.tsx` · `.../metrics/MetricsChart.test.tsx` · `.../metrics/use-metrics.test.ts` — **8 files, +403/−6**.

- **Symbols / APIs:**
  - NEW crate-internal (`buffer`, private): `encode_labels(&[KeyValue], &mut u64) -> String` · `render_any_value(&any_value::Value) -> String` · `hex_lower(&[u8]) -> String`. All three are `fn` (not `pub`) — no new public surface in `buffer`.
  - CHANGED private signatures (`buffer::appender`): `collect_metric_points` (+`labels: &mut Vec<String>`, +`redactions: &mut u64`) · `push_metric_row` (+`labels: &mut Vec<String>`, +`encoded_labels: String`). **Caller facts:** the code-graph impact query (rust plane, `db_state: fresh`, 10 rows) shows the write-side caller set is ONE closed in-crate chain — `consumer/dispatch_batch() @ consumer.rs:131` → `append_metrics_batch()` → `build_metrics_record_batch()` → `collect_metric_points() @ appender.rs:681-748` (5 sites, one per OTLP data kind) → `push_metric_row`, plus the test caller `redaction_counter_folds_once_per_batch_across_all_four_builders() @ appender.rs:1229`. **All callers are in-crate and all were updated**; no caller outside `crates/buffer` exists for these symbols.
  - CHANGED public DTO (`viz`): `MetricRow` gained `pub labels: String` (already `Serialize + Deserialize + specta::Type`). This is the ONE public-surface change. It KEEPS all existing consumers: `crates/mcp-server` (`dispatch_query_metrics` serialises the whole `viz` response — **unchanged, verified**) and the webview via generated bindings.
  - CHANGED SQL constant (`viz`): `SELECT_METRICS` selects a 6th column `labels`.
  - **NO new IPC method, endpoint, event, socket, port, or env var.** `metrics.query` keeps its name and arity; only its response row shape widened.

- **Crates / modules:** none added, none removed. Changed: `buffer` (schema + appender), `viz` (query), `ingest` (dev-only example).

- **Dependencies:** **none added, none bumped.** No `Cargo.toml` / `Cargo.lock` delta (verified — `git diff --name-only HEAD` over the manifests is empty).

- **Schema / config:**
  - `metrics_points` gained `labels VARCHAR NOT NULL DEFAULT ''` in **all THREE DDL representations** — `crates/buffer/src/schema.rs:70` (per-table const), `:168` (concatenated `SCHEMA_DDL`), `crates/viz/src/query.rs:481` (the structurally-unguarded fixture copy; `viz` has no `buffer` dep).
  - **PRIMARY KEY unchanged** — still exactly `(metric_name, ts_unix_nano, resource_hash, seq)`, 4 columns. `labels` is NOT part of it.
  - No migration (ring buffer is in-memory, created at startup). No config key. No violation-schema change.

- **Spec-master edits:** none yet — this report is the input to P2. Expected amendments the plan recorded: `architecture.md` §Conventions · `security-plan.md` §Anti-Patterns → Logging (two distinct halves) · `obs-plan.md` §5 · `test-plan.md` §4.

- **Counts / qualifiers moved:**
  - **security-plan §Security Anti-Patterns → Logging** states scrub coverage as **FOUR** client-controlled ring-buffer columns (`spans.service_name` · `span_events.name` · `metrics_points.metric_name` · `log_records.severity_text`). This chunk adds a **FIFTH** scrubbed cell: `metrics_points.labels`. The stated count is now stale.
  - **obs-plan §5 `redactions_applied` row** states the counter's registered scope as those same **four** write boundaries → now **five**. Additionally the counter's **unit** is refined: a data point can carry several redacted label pairs inside ONE persisted cell, and the implementation counts **per redacted label PAIR**, not per cell.
  - **test-plan §4 buffer crate** enumerates the per-column redaction pin set (four columns) and names the tally pin `redaction_counter_folds_once_per_batch_across_all_four_builders` — the builder count is still four, but the **scrub-site count** is not.
  - **arch §Conventions → Database entity naming** — a new `metrics_points` column. (§Occupied Resources enumerates TABLE names, not columns — the predecessor chunk ruled this explicitly; disposition still owed.)
  - Workspace test count **1870 → 1877** (+7, exactly the pins added).

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** two mutation-check mutations were applied and **fully reverted** (both restored + re-verified green): (a) neutralising the label scrub in `encode_labels`; (b) replacing the read-path `row.get(5)` with `String::new()`. Neither shipped. No surface was written-then-abandoned.

- **Spec claims disproved by measurement:**
  1. **The working entry's NOTE is falsified for this chunk** (recorded in `scope.md` §Premise correction at P5 validation-1, classified intent-incomplete). It claims this chunk fixes the misleading "No metrics received yet" surface. Measured: that empty state arises from a REJECTED BATCH, which `2026-08-23-metrics-points-identity` already closed. `MetricsChart.tsx:180-195` consumes only `ts_unix_nano` and `value`; there is no metrics table, legend, or per-row surface in the webview, so labels reach **no rendered surface at all**. The chunk's OUTCOME stands; the collateral claim does not. Disposition: the working-route line + NOTE are left as historical source (route-resolve territory), scope carries the correction.
  2. **The P4 dialogue-card rationale was backwards and is corrected in `plan.md` §Constraints & rejected approaches.** It claimed `secret_kv`/`api_key` being key-name-anchored meant a credential under a key name "is caught through the VALUE path regardless". Measured at `crates/security/src/scrubber.rs:77,83`: those arms require the key name INSIDE the matched string, so a split `{password: "hunter2"}` matches nothing. Operator caught it at the P5 review. No spec states the false claim — it lived only in the dialogue — so no spec amendment is owed for it; it is recorded here for the curation trail.

- **Coverage of new surfaces:**
  - `metrics_points.labels` (persisted client-controlled cell) → validation **✓** (ingest already bounds data-point attributes at 128/point, 256 B/key, 4096 B/value — `crates/ingest/src/invariants.rs`; inherited, not added) · instrumentation **✓** (`redactions_applied` on `buffer.tick`, folded per-table post-append) · PII **redacted ✓** (joined `key=value` ∪ bare value through `scrub_attribute`; keys verbatim) · tests **unit ✓** (5 in-crate `buffer` pins + 2 `viz` pins) + **e2e ✓** (real OTLP path, RED→GREEN) · a11y **n/a** (no rendered surface) · tokens **n/a** (no UI).
  - `MetricRow.labels` (cross-bridge DTO field) → validation **n/a** (read-only projection of an already-validated+scrubbed cell) · instrumentation **✓** (`viz.query.metrics` already emits `query_id`/`param_count`/`row_count`/`latency_ms`; no per-row emission added) · PII **redacted ✓** (the stored cell is already scrubbed; no second raw path) · tests **unit ✓** (`query_metrics_returns_labels_distinguishing_otherwise_identical_points`, `query_metrics_returns_empty_labels_for_attributeless_points`, serde round-trip) · a11y **n/a** (nothing renders it) · tokens **n/a**.

### Scrub posture — the fact `D-security-logging` binds to

- **Coverage set** is now **FIVE** persisted client-controlled cells (the four above + `metrics_points.labels`). Still NOT covered, by construction and unchanged: `instrumentation_scopes.*` (no producer), `span_events.exception_type` (deliberate exclusion — class identifier).
- **A THIRD scrub SHAPE now exists.** The plan records two: choke-point (`extract_service_name`, feeding three consumers) and push-site (`scrub_otlp_field` on a bare column value). Labels use neither: they scrub the **JOINED `key=value` form, unioned with the bare value**, because `secret_kv` (`scrubber.rs:83`) and `api_key` (`:77`) are key-name-anchored and are **structurally unreachable on a split value**. Keys are stored verbatim; only the value becomes `[REDACTED:{category}]`.
- **Stated failure mode of the treatment:** over-redaction when the KEY alone trips a match (`password=1` redacts a benign value), accepted per the catalog's own recall-over-precision posture (`scrubber.rs:56-58`). Two distinct keys never collapse into one placeholder — the key survives — so the collision class the predecessor chunk closed is **not** reintroduced.
- **Ring buffer remains UNENCRYPTED** — write-boundary redaction is the control, unchanged.

## Deviations from intent

1. **Step order inverted deliberately.** Producer extension (Step 8) and the RED leg (Step 9) ran BEFORE Steps 1–7, because Step 9 requires the measurement at HEAD. Stated at the time, not silently reordered. Justification: the plan's own text mandates "RED at HEAD".
2. **Two pins added beyond the plan's list, both in listed files.** (a) `query_metrics_returns_labels_distinguishing_otherwise_identical_points` in `viz` — acceptance criterion 1 says "read back through `metrics.query`", but the plan's buffer pin reads raw SQL, leaving the acceptance under-pinned. (b) `label_encoding_is_attribute_order_independent` — the encoder sorts pairs; without this pin the distinguishability test could be measuring attribute ordering rather than label content.
3. **Three webview test fixtures edited, not in research's Files-to-modify.** `MetricsRoute.test.tsx`, `MetricsChart.test.tsx`, `use-metrics.test.ts` construct `MetricRow`; widening a DTO research DID list makes updating its fixtures unavoidable. Judged in-scope (same class as the `viz` `metric_row_round_trips_through_serde_with_new_fields` constructor research named as a companion), per fix-loop-protocol Trigger 3's "a listed file needing more edits than research predicted".
4. **`consumer.rs` and `mcp-server/src/tools.rs` confirmed unchanged**, as the plan predicted. The plan asked to verify rather than assume; verified.

## Decisions & corrections

- **Operator ruling (P4, both recommendations approved):** representation = a scrubbed `labels` column on `metrics_points` (side table and hashed key both rejected with reasons recorded in `plan.md`); label KEYS stored verbatim, only VALUES scrubbed.
- **Operator correction (P5 review) — load-bearing.** The scrub input must be the JOINED `key=value` form, because `scrub_attribute` takes the value alone while two of the eight catalog arms are key-name-anchored. My Q2 dialogue card had asserted the opposite. The ruling survived unchanged; the MECHANISM changed. Consequence beyond the encoder: the live-leg producer needed **two** canary classes (keyed + bare), since a RED leg carrying only a bare `sk_live_` value would have passed while the keyed class leaked.
- **Correction found while authoring the plan's Test Commands:** `agent-run.sh run` is `cargo nextest run --workspace`, NOT an OTLP send. The live leg is the `inject_colliding_metrics` example built outside the timed section and invoked by path. The first draft also called `mktemp -d` twice per leg, which would have split boot and the log read across different data dirs.
- **Shell-discipline recurrence (twice this session):** a heredoc-quoted Python edit script failed to parse because the target Rust line ends in a line-continuation backslash. The second occurrence was a MUTATION-CHECK script — the SyntaxError left the mutation unapplied while the test run printed PASS, which reads exactly like a pin failing to discriminate, i.e. the inverse of the finding. Caught only because the expected RED did not appear.
- **Bindings clobber recurred (8th occurrence)** — workspace nextest rewrote `bindings/index.ts` to the no-mcp shape. Recovered by regenerating last. **This wrap is the first where a bindings change is INTENDED**, so the staged check must assert BOTH the mcp entries AND `labels: string`.

## Outcome

**Acceptance criteria: met.**

- **RED → GREEN on the real OTLP path**, separate fresh `ANDROMEDA_PULSE_DATA_DIR` per leg, production binary both legs:

  | | RED (at HEAD) | GREEN (after fix) |
  |---|---|---|
  | `rows_ingested` | 8 | 8 |
  | `redactions_applied` | **2** (metric names only) | **4** (+2 label canaries) |
  | `duckdb.append` | `rows_appended: 8` | `rows_appended: 8` |
  | ERROR / `app.panic.fatal` | 0 / 0 | 0 / 0 |
  | canary literals in log | 0 | 0 (across 1417 lines) |

  The +2 delta is exactly the two label canaries; the benign `http.method=GET` did NOT increment — selectivity proven at the wire. As the plan predicted, this RED leg was **ERROR-free** (information loss, not batch rejection), so the RED evidence is the counter, not a `duckdb.append` failure.

- **Mutation check — both pin families discriminate.** Neutralising the label scrub turned `credential_label_values_redact_in_both_classes_and_keys_survive` RED with the defect verbatim (`left: "password=hunter2"`, `right: "password=[REDACTED:secret_kv]"`) while the distinguishability and benign pins stayed green. Dropping `labels` from the read turned `query_metrics_returns_labels_distinguishing_otherwise_identical_points` RED while `query_metrics_returns_empty_labels_for_attributeless_points` stayed green — the expected asymmetry for a conditional property, where the distinguishing half carries the guard. Both mutations reverted and re-verified.

- **Gates green (commands run):** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` → **1877 passed, 1 skipped** · `cargo nextest run -p buffer -p viz` → **225/225** · `npm run lint|typecheck|test --prefix pulse-app/ui` → **801 vitest passed** · `cargo xtask capability-drift` → **clean** · `cargo xtask capability-widening-check` → **clean (0/3)** · `cargo deny check bans licenses sources` → **ok**. `cargo deny check advisories` observed SEPARATELY and designed-RED at exactly the **8 owned IDs** (RUSTSEC-2026-0189/0190/0194/0195/0204/0222/0253/0258) — no new findings.

- **Smoke:** fired as the chunk's own RED and GREEN live legs (Direct-binary variant — `agent-run.sh run` is the test suite, not an OTLP send). Each booted the production binary, ingested over the real receiver, and had its obs log read from the date-suffixed family. Clean shutdown by specific PID both legs; `:4317`/`:4318` confirmed closed; **zero `pulse-app` orphans**. No user-visible surface changed, so the headful self-verify had no changed subject.

- **Gate deferrals:** none. Every gate in the plan's `## Test Commands` ran.

- **`cargo audit` PREREQ (pin #8):** session 36 sits BETWEEN ratified probe points (25/28/31/34, DISCHARGED at 34; next **37**) → **skipped in the ratified form**, never silently. Basis re-verified rather than echoed: the chunk added and bumped **zero** dependencies (no `Cargo.toml`/`Cargo.lock` delta), so the upstream RustSec DB load failure cannot be affected by anything here. Overlap re-derived this wrap: `cargo deny check advisories` stable at the **8** owned IDs. The next wrap (session 37) fires the probe in full form.
