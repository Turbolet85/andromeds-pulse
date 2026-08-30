# Codebase Research — 2026-08-30-diagnostics-un-muting-harness-truth-sweep

## Scope
- **Depth:** deep · **Reads:** ~14 targeted file sections · **Globs/Greps:** ~38 · **Graph queries:** 3
  (trace at `.andromeda/runs/2026-08-30T08-13-20Z-phase/tree-query-2026-08-30-diagnostics-un-muting-harness-truth-sweep.json`)

## Files inspected
- `pulse-app/src/observability.rs` (registration sites + :1925-1950 + :2575-2600 + mod tests) — 206
  `by_target.insert(` registrations; bare `interpretation` key at **:1934** (5 fields, NO live emitter
  targets bare `interpretation` — pure fallback, safely removable with a discriminator); `triage.cue.tick`
  leaf **:1315** already carries all 9 fields (A3 = no-op reconciliation); `interpretation.model.load` leaf
  **:1962** = `[model_identity, tier, file_size_bytes, load_status]`; `app.boot.tracing.init` **:589**
  `[log_dir, default_fields_active]`; `app.boot.pid` **:596** `[pid, path, error, run_dir, data_dir]`;
  `triage.cue.suppression_check` **:1406** (registered — A8 is rate-only); `incidents.list_active.request`
  **:2321** (C7 corrected coordinate holds); NO leaves for the four l1a targets or
  `triage.incident.auto_resolve.tick`; `mod tests` opens **:2637**, 130 `#[test]` fns, in-file comment
  `:127` admits "compiles but never executes"; `[lib] test = false` at `pulse-app/Cargo.toml:12`.
- `crates/triage/src/baseline/sql.rs` (:480-500, :710-740) — l1a emit sites: `query_count_total`
  (`query_name`,`value`), `query_latency_p99_milliseconds` (`query_name`,`duration_ms`,
  `row_count_returned`), plus TWO more unregistered same-family targets: `q7_timeout_count_total` :484
  (`value`,`timeout_ms`,`rejection_reason`, WARN) and `q7_fallback_count_total` :494
  (`value`,`fallback_query_kind`,`cause`).
- `pulse-app/src/incident_observer.rs` (:29, :109-116) — `triage.incident.auto_resolve.tick` emits exactly
  `evaluated_count`, `resolved_count`, `duration_ms` (info, once per tick). New exact leaf = those 3.
- `pulse-app/src/llamacli_inference.rs` (:234-275, :600) — `interpretation.model.load` emits at :239
  (loading) and :262 (loaded), fields `tier`+`load_status` only; `semantic_name` (file_stem, basename-safe)
  is in scope at the loaded site for a `model_identity` emit; `interpretation.model.load.error` separate
  target :600.
- `pulse-app/src/main.rs` (:505-530, write_pid_file) — `inference_mode` emitted on `interpretation.model.load`
  at :514 ("deterministic") / :523 ("real"); `app.boot.pid` emits `run_dir`/`data_dir` (canonicalized, FULL)
  on one record and `pid`+`path` (FULL) at the written record; `app.boot.tracing.init` emits
  `log_dir = ?logs_dir` (FULL) at `observability.rs:~2585`.
- `xtask/src/main.rs` (:38-39, :206, :278-296) — `harness_status()` calls
  `ui_bridge::health::current_health()` IN-PROCESS (xtask's own process; comment admits "full IPC roundtrip
  lands at the integration-test chunk"). NOT `tauri::test::mock_builder` — scope premise corrected. Exit
  keyed on envelope status; always describes xtask.
- `scripts/agent-run.sh` (:36-72, :105-130) — `boot` runs `cargo run --bin pulse-app --release` (:42) then
  polls `cargo xtask harness:status` (:49) — a vacuous readiness poll; `status` (:72) delegates to the same;
  `logs` (:110+) ALREADY resolves the rotated `agent-latest.jsonl*` family (glob loop, precedence bases).
  `agent-run.ps1:37` same delegation.
- `xtask/src/smoke.rs` (:95, :414, :516-519, tests :619-743) — `read_jsonl_lines` handles the daily-rotated
  family; 6 existing tests all use the bare name (family branch never entered) — the owed
  `harness-log-family-resolution-coverage` test.
- `xtask/src/self_verify.rs` (:180, :254) — `launch_pulse` sets NO `current_dir`; the a11y leg at :254 does.
  Fix confined to the launch site.
- `crates/buffer/src/schema.rs` (:30-138, :315-330) — 8 CREATE TABLEs; the disproved hang comment sits at
  ~:317-323 ("observed to hang in libduckdb-sys 1.10502") inside
  `spans_primary_key_is_composite_trace_id_span_id`.
- `crates/buffer/src/retention.rs` (:20-35) — `DELETE_BY_CUTOFF: [&str; 7]` includes `span_links`,
  `resources`, `instrumentation_scopes` (the three producer-less sweeps); `log_templates` excluded (LRU).
- `crates/buffer/src/consumer.rs` / `state.rs` — reject ERROR with `reject_reason` at consumer.rs:81
  (`describe_error`), `record_redactions` folded per-table post-append (:116-150),
  `record_rows_appended` :168, `duckdb.append` info :193; `BufferStateSnapshot` at state.rs:47 (no
  `append_rejections` yet). `buffer.tick` emit at `pulse-app/src/heartbeat.rs:307-323` = **14 fields**.
- `crates/triage/src/cue/{evaluate,classify,emitter}.rs` — `let persistence_seconds = snapshot.samples;`
  at evaluate.rs **:55** AND **:113** (two families); `classify_priority` gate `>= 30` at classify.rs:23;
  consumers of the field: classify gate, suppression-eligibility cutoff (emitter.rs:916-924,
  `persistence_cutoff_seconds`), two log emits (emitter.rs:232 suppression_check, :337 cue.emit), fixtures
  (cue/broadcast.rs:100, cadence/broadcast.rs:211, coordinator.rs:581, contract.rs:658). NOT on any
  webview/TS surface (0 hits in `pulse-app/ui/src`). Contract field: `triage/src/contract.rs:323`.
- `crates/corpus/src/{schema,disposition,contract}.rs` — `SCHEMA_VERSION: u32 = 1` (schema.rs:24);
  `baseline_state` DDL :27/:36; disposition enumerations :25/:37; the ONLY insert under `#[cfg(test)]`
  (:308 opens, :355 INSERT) — scope coordinates confirmed exactly. `save_incident_event` INSERT at
  contract.rs:880 (generic `event_kind`).
- `pulse-app/src/{inference_runtime,incidents_router}.rs` — `save_incident_event(id, "created", …)` at
  inference_runtime.rs:900 is the ONLY production event write (graph-confirmed); `incidents_router.rs`
  records outcomes to tracing only; `item_count` emit at :221-227 confirmed.
- `xtask/src/webview_drive.rs` (:44-149) + `pulse-app/ui/tests-e2e/webview-drive.mjs` — STAGES table has
  **16** ids (`launch` :55 … `signpost-repeat` :138), `stage_ids()` :143 feeds `--expect-absent`
  validation; the driver's `windowInvoke` (:292) already reads `outer_position` (:408) for the `main`
  label — the boot-geometry stage needs only the `compact-widget` label + a record() call.
- `pulse-app/src/window.rs` (:373-392) — `WIDGET_EDGE_MARGIN: i32 = 24`, `compute_snap_position` :381 with
  existing corner unit tests; on a FRESH data dir (no `window-geometry.json`) the widget boots at the
  default snap corner — the harness always uses a fresh temp data dir, so the stage asserts the default
  margin-inset corner deterministically.
- `pulse-app/ui/tests-a11y/` + `pulse-app/ui/src/report/` — zero copy-state coverage (grep `copyState|
  copy-markdown|data-copy-state` = 0 in tests-a11y); sentinels live at `helpers/mock-tauri.ts:98-105`;
  the surface is `ReportRenderer.tsx:431` (`data-copy-state={copyState}`); p9 spec =
  `tests-a11y/axe/p9-diagnostic-report-modal.spec.ts`; `Report.tsx:102` has a `role="alert"`.
- `.andromeda/security-plan.md` :68 (§Threat Model entry point) + :140 (§Input Validation MCP row) — both
  4-tool enumerations confirmed present (D1).
- `.andromeda/test-plan.md` — `grep -i warm` = ZERO hits; §3 Boot-smoke gate at :314; B3 premise corrected
  (no enumerable warm-boot family at HEAD).

## Graph impact
- **classify_priority** — callers: `evaluate_error_rate_spike` + `evaluate_service_went_silent`
  (`cue/evaluate.rs:~140/:187`) + the `cue/mod.rs:28` re-export. A C4 rename is triage-crate-local plus
  the emitter log fields + the suppression cutoff comparison + 4 fixture sites + `contract.rs:323`; no
  cross-crate or TS consumer (name-bridge: 0 hits in `ui/src`).
- **save_incident_event** — production caller set is exactly `inference_runtime::create_incident_from_l4_output`
  (:899) via `CorpusIncidentPersistence` (:175) + one test impl (`integration_incident_write_guard.rs:116`).
  C5's single-kind producer is graph-confirmed; a choke-point event write at
  `corpus::update_incident_status` would cover all seven status writers with one edit.
- **crate_edges** — `buffer` inbound: `pulse-app`, `ui-bridge` (+ query trace holds the full set);
  `corpus` inbound: `pulse-app` (+ `mcp-server` per arch). A C2 DDL drop and C1 counter are inside
  already-connected crates — zero new edges.

## Patterns detected
- **Exact-leaf + both-directions field-equality guard + fallback discriminator** (obs precedents:
  `buffer.consumer.stalled`, `triage.cue.tick` 5→9, delegated-timing `metric.*` leaves) — the shape for
  every Family A leaf.
- **Tick-aggregated fold** (`record_redactions` per-table post-append in consumer.rs:116-150, ridden out
  on `buffer.tick`) — the C1 `append_rejections` shape; the reject site (consumer.rs:81 ERROR) is the fold
  point, once per failed batch.
- **StageHalves two-half verdict, DOM-half-only stages** (webview_drive.rs; `native-menu-suppressed` and
  `report-copy` are DOM-only precedents) — B6's shape; `--expect-absent` eligibility is free via
  `stage_ids()`.
- **`pulse-app/tests/` integration-target migration** (2026-05-20 precedent, 47 tests; test-plan §2/§4
  codified) — B4's destination; internals reachable via `pub` + `#[doc(hidden)]`.
- **Formalized xtask CLI contract** (`check:npm-supply-chain`: exit codes + named verdict arms + one JSON
  object) — the registry-worthy shape for a truthful `harness:status`.
- **Monotonic-guard choke point** (`corpus::update_incident_status`, seven writers) — where a C5
  event-persistence arm would attach (Applied-only, never DeclinedStale).

## Conventions to follow
- Allowlist guards live under `pulse-app/tests/`, never in `observability.rs::tests` (obs-plan §8;
  test-plan §2/§4).
- `-E`-under-`--workspace` for pulse-app test selection, never `-p` (test-plan §3, measured link failure).
- Basename-only path fields via the existing sanitizer vocabulary (`path_basename` / `root_basename` /
  `sanitize_window_label`) — A7 adopts, not invents.
- capability-drift runs LAST in the gate order; the three `pulse-app/ui` webview gates bind (chunk touches
  `pulse-app/ui/**`).
- Boot-smoke gate mandatory (`observability.rs` is a named boot-path trigger) — Direct-binary variant
  sanctioned (test-plan §3:314+).

## New files to create
- `pulse-app/tests/observability_allowlist_pins.rs` (name indicative) — B4's migrated guard tests (130
  fns, possibly split by theme) + the new Family-A leaf guards (field-set equality both directions +
  fallback discriminators) + the no-bare-`interpretation` discriminator.
- `pulse-app/ui/tests-a11y/axe/p9-report-copy-states.spec.ts` (or an extension inside the existing p9
  spec file) — B7's rejected + in-flight copy-state axe coverage via `__mockReject` / `__mockDelayMs`.
- (conditional, C2 drop arm) a `SCHEMA_VERSION 2` migration step in `crates/corpus/src/schema.rs` —
  `DROP TABLE IF EXISTS baseline_state` under the existing PRAGMA user_version ladder.

## Files to modify
- `pulse-app/src/observability.rs` — add exact leaves: 4× l1a (q1/q2 count+latency, q7 timeout+fallback),
  `triage.incident.auto_resolve.tick`; complete `interpretation.model.load` (add `inference_mode`;
  `file_size_bytes` disposition); REMOVE the bare `interpretation` key (:1934); complete `buffer.tick`
  leaf (+`append_rejections`); adjust `app.boot.pid`/`app.boot.tracing.init` field names if basename
  rename lands; DELETE the never-run `mod tests` (migrated).
- `pulse-app/src/main.rs` — basename-only emits for `app.boot.pid` (path/run_dir/data_dir) +
  `write_pid_file` fields.
- `pulse-app/src/observability.rs` (tracing.init emit ~:2585) — `log_dir` basename.
- `pulse-app/src/llamacli_inference.rs` — emit `model_identity` at the loaded site (:262 area).
- `pulse-app/src/heartbeat.rs` — `buffer.tick` emit +`append_rejections` (14→15 fields).
- `crates/buffer/src/state.rs` — `append_rejections` counter + snapshot field + fold method.
- `crates/buffer/src/consumer.rs` — record the rejection at the reject arm (:81 area), once per failed
  batch.
- `crates/buffer/src/schema.rs` — B8 comment correction (:317-323).
- `crates/buffer/src/retention.rs` — (C3 drop arm) remove 3 dead DELETEs; `schema.rs` remove 3 CREATEs +
  the retention test enumerations.
- `crates/triage/src/cadence/coordinator.rs` — B9 comment (:416-417).
- `crates/triage/src/cue/emitter.rs` — A8 demote per-cue `suppression_check` info → debug (:229 area);
  (C4 rename arm) field rename at :232/:337 + cutoff comparison :916-924.
- `crates/triage/src/cue/{evaluate,classify}.rs` + `crates/triage/src/contract.rs:323` + fixture sites
  (`cue/broadcast.rs:100`, `cadence/broadcast.rs:211`, `cadence/coordinator.rs:581`, `contract.rs:658`) —
  (C4 rename arm) `persistence_seconds` → samples-named field.
- `crates/corpus/src/schema.rs` + `disposition.rs` — (C2 drop arm) DDL + enumerations + cfg(test) INSERT;
  `contract.rs` (C5 persist arm) event INSERT at the `update_incident_status` Applied path.
- `xtask/src/main.rs` — `harness_status()` truthful rewrite (PID-file + log-family based; formalized
  verdict contract).
- `xtask/src/smoke.rs` — the owed family-branch test (date-suffixed-only fixture).
- `xtask/src/self_verify.rs` — `launch_pulse` gains `current_dir` (per-run temp data dir).
- `xtask/src/webview_drive.rs` — 17th stage `boot-geometry` in STAGES (+ Role-cell wrap amendment).
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — the stage's DOM half (windowInvoke `outer_position`,
  `compact-widget` label, margin-inset assertion vs `WIDGET_EDGE_MARGIN`).
- `scripts/agent-run.sh` (+ `.ps1`) — only if the `harness:status` contract shape changes what the verbs
  print/poll; boot-readiness poll inherits truthfulness from the xtask fix.
- Caller threading: `pulse-app/tests/` gains the migrated + new guards (collected-by-name evidence);
  `xtask/src/main.rs::EXPECTED_PROCEDURES` untouched (no TauRPC change on the leaned paths — C6
  document-arm adds no procedure); manifests untouched (zero new deps).

## Open questions
- **C2/C3 dead-schema dispositions** (baseline_state drop-vs-document; 3 DuckDB tables drop-vs-document)
  → blocks: plan-decision — operator fork at P4/P5.
- **C4 `persistence_seconds` fix side** (rename-to-samples-truth vs convert-to-true-seconds, which
  changes tier behavior and needs time tracking that does not exist) → blocks: plan-decision.
- **C5 `incident_events` disposition** (choke-point status-event persistence vs document created-only) →
  blocks: plan-decision.
- **C6 `pulse://stream/incidents`** (document producer-only vs wire a consumer) → blocks: plan-decision.
