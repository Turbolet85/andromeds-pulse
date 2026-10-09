# Report — 2026-08-30-diagnostics-un-muting-harness-truth-sweep

**Chunk:** Diagnostics un-muting + harness-truth sweep — the deferred targeted-cleanup cluster gets an
owner, so a diagnostic's silence stops being unreadable
**Date:** 2026-08-30T10:05:00Z
**Commits:** (this wrap's commit; last_wrap was `feat(2026-08-30-npm-advisory-coverage)`)

## Changes (structured — detectors read this)

- **Files:** `pulse-app/src/{observability.rs, main.rs, llamacli_inference.rs, heartbeat.rs}` ·
  `crates/buffer/src/{state,consumer,contract,schema,retention}.rs` ·
  `crates/corpus/src/{schema,disposition,contract}.rs` ·
  `crates/triage/src/{contract.rs, cadence/{broadcast,coordinator}.rs, cue/{broadcast,classify,emitter,evaluate,mod,thresholds}.rs, pattern/{mod,storm,suppression}.rs, lifecycle/*}` (rename + fmt) ·
  `xtask/src/{main.rs, harness_status.rs(NEW), smoke.rs, self_verify.rs, webview_drive.rs}` ·
  `scripts/agent-run.{sh,ps1}` · `pulse-app/ui/tests-e2e/webview-drive.mjs` ·
  `pulse-app/tests/{observability_pins.rs(NEW), unit_observability_allowlist_sweep.rs(NEW), unit_observability_allowlist_generation_damper.rs, unit_observability_allowlist_l4_path_guard.rs}` ·
  `pulse-app/ui/tests-a11y/axe/p9-report-copy-states.spec.ts(NEW)`
- **Symbols / APIs:**
  - `cargo xtask harness:status` CONTRACT REPLACED: was an in-xtask-process `ui_bridge::health::current_health()`
    envelope (exit 0 unconditionally); now a real-process verdict JSON
    `{verdict: running-healthy|stale|not-running|cannot-evaluate, pid, log_file_basename, last_write_age_seconds, stale_after_seconds}`,
    exit 0/1/1/2, derived from the registered PID file + the log family's mtime (`xtask/src/harness_status.rs`).
    Callers: exactly the two harness scripts (boot poll + status verb), both updated in-chunk and now
    threading `ANDROMEDA_PULSE_{DATA_DIR,PIDFILE,LOGFILE}` into the xtask child — no other caller exists (grep-verified).
  - Allowlist leaves (obs registry, `pulse-app/src/observability.rs`): ADDED exact leaves
    `metric.pipeline.l1a.{query_count_total, query_latency_p99_milliseconds, q7_timeout_count_total, q7_fallback_count_total}`
    + `triage.incident.auto_resolve.tick`; COMPLETED `interpretation.model.load` →
    `{model_identity, tier, load_status, inference_mode}` (never-emitted `file_size_bytes` REMOVED);
    REMOVED the bare `interpretation` prefix key (`:1934` breach — obs-plan §8 invariant now holds in code);
    `buffer` set + `append_rejections` (buffer.tick 14→15 fields);
    `app.boot.tracing.init` → `{log_dir_basename, default_fields_active}`;
    `app.boot.pid` → `{pid, path_basename, error, run_dir_basename, data_dir_basename}` (full-path fields GONE);
    `triage.cue.{suppression_check, emit}` field `persistence_seconds` → `persistence`.
  - Emit sites: `model_identity = semantic_name` now EMITTED at the loaded site (llamacli_inference.rs);
    the two boot records emit basename-only via `observability::log_basename` (now `#[doc(hidden)] pub`);
    per-cue `triage.cue.suppression_check` demoted `info!` → `debug!` (rate fix; the `:451` lagged WARN kept).
  - Triage rename (crate-local, no external consumer — 0 hits outside `crates/triage`, 0 in `ui/src`):
    `AttentionCue.persistence_seconds` → **`persistence`** (per-family units documented: spike families =
    EWMA samples, silence = quiet seconds; `>= 30` gate VALUES unchanged);
    `SuppressionParams.persistence_cutoff_seconds` → `persistence_cutoff_samples`;
    `Thresholds.suppression_persistence_cutoff_seconds` → `suppression_persistence_cutoff_samples`;
    `DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS` → `_SAMPLES` (spike-only comparison — samples is fully true there).
  - `corpus::update_incident_status` now records ONE `incident_events` row per Applied status-VALUE-change
    (event_kind = new status; same transaction; DeclinedStale + same-status refresh write nothing) — covers
    all seven production writers incl. the cross-process sidecar from the one choke point.
  - `BufferState::record_append_rejection` + `BufferStateSnapshot.append_rejections` +
    `BufferHeartbeat.append_rejections` (fold once per FAILED batch at run_consumer's two error arms;
    `StormEvidence`/`duckdb.append` ERROR record byte-unchanged).
  - Headful leg: 17th stage `boot-geometry` (DOM-only; driver reads `outer_position`/`outer_size`/
    `current_monitor` for `compact-widget`; Rust verdict re-derives the app formula
    `mon_w − round(480×scale) − 24`); `xtask::self_verify::launch_pulse` gains `current_dir(data_dir)`
    (kills the stray root-level bindings emission).
  - Test-only visibility widenings in `pulse-app/src/observability.rs` (`#[doc(hidden)] pub`):
    `SERVICE_NAME`, `DefaultFields{,Result}` + fields, `JsonWithDefaults` + fields,
    `validate_ci_run_id`, `validate_git_commit_sha`, `resolve_deployment_environment`, `log_basename`.
  - No TauRPC procedure changes · no new env vars · no port/socket changes · no new stream topics.
- **Crates / modules:** no crates added/removed; `xtask` gains module `harness_status`;
  `pulse-app` src-level `mod tests` DELETED (migrated).
- **Dependencies:** none added · none bumped (both ecosystems untouched).
- **Schema / config:**
  - Corpus SQLite: **SCHEMA_VERSION 1→2** — `baseline_state` DROPPED (ladder migration: fresh-create at v2;
    v1→v2 `DROP TABLE IF EXISTS`; newer-version reject unchanged). Disposition enumerations follow (payload
    tables 5→4).
  - DuckDB ring buffer: reserved tables **8→5** (`span_links`, `resources`, `instrumentation_scopes`
    CREATEs deleted — zero producers, measured 2026-08-23); retention `DELETE_BY_CUTOFF` **7→4**.
  - No config keys changed; no violation-schema changes.
- **Spec-master edits:** none at authoring — the plan's `Expected amendments (wrap)` list (all six touched
  masters + arch registries) is this wrap's P2 floor.
- **Counts / qualifiers moved:** DuckDB reserved tables 8→5 (stated: arch §Occupied Resources + §Conventions;
  security-plan §Logging "three of seven DELETEs" wording) · corpus tables 6→5 + SCHEMA_VERSION 1→2 (stated:
  arch §Occupied Resources → Corpus; security-plan §Threat Model + §Data Protection 6-table enumerations) ·
  retention sweeps 7→4 · headful stage count 16→**17** (stated: arch §Stack Role cell; test-plan §6;
  rules/testing.md) · `buffer.tick` fields 14→**15** (stated: obs-plan §1/§5/§8 triple-site;
  rules/observability.md) · the obs-plan §8 muted-target census **4 remaining → 0** (plus the two
  research-found q7 targets, also un-muted) · workspace test count 2085 → **2239** (+157 added-by-name,
  −3 removed rstest cases for the dropped tables).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** the boot-geometry verdict's first draft (outer-width back-computation)
  was replaced before commit — Windows DWM invisible borders make the realized outer width a wrong formula
  input (+16 px measured); the shipped verdict re-derives the app's own scaled-default-width formula.
- **Spec claims disproved by measurement:**
  1. The route entry's "130+ `#[test]` fns in `pulse-app/src/observability.rs`" names the WHOLE class as one
     file — the new mechanical guard's first firing measured **~101 MORE dead lib-src tests across 14 files**
     (heartbeat 25 · window 13 · diagnostics_router 11 · tray 10 · plugins_router 7 · snapshot_runtime 7 ·
     storage_router 7 · mcp_router 6 · connection_router 4 · restart_observer 4 · digest_runtime 2 ·
     baseline_observer 2 · storm_observer 2 · streams 2); `main.rs` exempt (bin-target tests genuinely run).
     Disposition: guard shipped as a per-file RATCHET; the migration needs a route entry (P5 owns minting it).
  2. layout-templates §Component — Settings modal states the widget snap position is a NINE-value setting
     (corners + edges + center); the shipped `WidgetPosition` enum is FOUR corners
     (`crates/ui-bridge/src/contract.rs:76-82`, default TopRight, test-pinned). Amendment candidate (same
     spec-truth family as D2).
  3. test-plan §3's status caveat ("`agent-run.sh status` exits 0 and emits a full payload … when nothing is
     running, measured 2026-08-23") is now FALSE — B1's rewrite makes a no-app run exit non-zero
     (`not-running`), live-proven this chunk. §3's `status` description must move to the new verdict contract.
  4. The C4 fork's stated basis ("the assignment is a raw EWMA sample count") held for 2 of 3 producers —
     the silence family assigns genuine quiet SECONDS (`cue/evaluate.rs:187`). The rename landed as bare
     `persistence` + per-family unit docs; `docs/v0_2_0/pulse-capability-spec.md`'s "spike duration" wording
     sits OUTSIDE the 7-master flow — recorded here for the operator; the working entry's C4 item is
     discharged by the rename (values unchanged).
- **Coverage of new surfaces:**
  - `xtask harness:status` verdict → validation ✓ (trim+is_file env guards) · instrumentation n/a (dev tool) ·
    PII redacted✓ (basename-only fields) · tests unit (7 pins incl. all arms + family-mtime + garbage-pid) ·
    a11y n/a · tokens n/a
  - `boot-geometry` stage → validation n/a · instrumentation n/a (window-API reads; obs half None by design) ·
    PII n/a · tests unit (3 committed verdict pins on the live-measured fixture) + live 17/17 GREEN + RED arm ·
    a11y n/a (no DOM anchors — window API) · tokens n/a (asserts the Rust `WIDGET_EDGE_MARGIN` constant)
  - `incident_events` lifecycle writes → validation n/a (internal, prepared statements) · instrumentation ✓
    (rides `triage.incident.persist` obs) · PII redacted✓ (empty encrypted payload; kind = bounded status
    label) · tests unit (3 pins: change/refresh/declined) + live (`created×2, resolved×2` read back)
  - `append_rejections` counter → instrumentation ✓ (own field on buffer.tick's exact-resolved set) · PII n/a
    (aggregate count) · tests unit (reject/success conditional pair) + wire-proven (renders 0 on 49 ticks;
    arithmetic pinned by the pair — a live rejection is deliberately absent from this leg)
  - p9 copy-state audits (rejected + in-flight) → a11y ✓ (axe 0 critical/serious; SC 4.1.2/4.1.3/1.4.1
    asserted; 0 new baseline tuples) · tests e2e-tier playwright (3 specs, collected + green)
  - corpus v2 migration → tests unit (fresh-create + v1→v2 converge) + live (user_version=2, 5 tables)

## Deviations from intent
1. **C4 rename shape** — bare `persistence` + per-family unit docs instead of the fork's example
   `persistence_samples`: the silence family's genuine-seconds site (hidden from research by a
   head-truncated grep) made `_samples` a new lie; the fork's commitment (rename-to-truth, values
   unchanged) is honored, and `_samples` is used where fully true (the spike-only cutoff chain).
2. **Mechanical guard = ratchet, not flat zero** — first firing found the 101-test finding above; migrating
   them mid-chunk is a scope explosion (fix-loop T3 class). Ratchet: zero for any new file, ≤ baseline for
   the 14 named files, growth reds, `main.rs` exempt. The plan's zero-src acceptance lands in this adjusted
   form; the migration is surfaced for a route entry.
3. **B6 discrimination via committed pins** (3 tests over the live-measured fixture, accept + wrong-inset +
   missing-fields arms) instead of the planned one-off mutation check — stronger: survives re-runs.
4. Two pre-existing bare-`interpretation` discriminator tests STRENGTHENED to the removed-key truth
   (assert `is_none()`), never relaxed.
5. `agent-run.{sh,ps1}` gained the env-threading edits (the plan's conditional fired); the sh boot poll is
   now honestly meaningful — a cold `cargo run` build can exceed the 10 s readiness default, which is
   truthful behavior (`HARNESS_STATUS_TIMEOUT` overrides).
6. No-op reconciliations executed as planned and verified: A3 (`triage.cue.tick` already complete),
   A9 (`buffer.tick` trio fine at HEAD), C3 fixture-sweep (zero references in viz/triage fixtures).

## Decisions & corrections
- Operator fork resolutions (phase P4/P5): DROP both dead-schema sets · RENAME persistence to truth ·
  PERSIST lifecycle events at the corpus choke point · RECORD `pulse://stream/incidents` producer-only.
- Plan leans executed: A7 basename-only (obs §11 unconditional ban) · A5 `file_size_bytes` removed from the
  leaf · A8 demote-to-debug · B1 boot-verb `--release` documented-not-changed.
- Self-caught harness traps (curation candidates): `set -o pipefail` + `grep -q` fails the pipeline ON the
  match (SIGPIPE to the producer) — a wait predicate so keyed never fires; post-hoc `grep -c` reads were the
  unaffected evidence · the migrated-test visibility recipe (glob-import + compiler-driven
  `#[doc(hidden)] pub` widening) handled a 3,431-line migration in 3 error rounds.
- The `interpretation.incident.created` field-completeness guard — documented for months as sitting in the
  dead src mod — now RUNS (migrated, collected by name).

## Outcome
All acceptance criteria met (the zero-src-test criterion in ratchet form per deviation 2; the chunk claims
0 matrix capabilities as planned). Gates green: `cargo fmt --check` · `clippy --workspace --all-targets
--all-features -D warnings` · `cargo nextest run --workspace --profile ci` **2239/2239 + 1 skip** ·
`npm --prefix pulse-app/ui run {lint, typecheck, test}` · `cargo xtask capability-widening-check` ·
`cargo xtask check:ingest-progress` · `cargo deny check bans licenses sources` exit 0 + `advisories`
exit 0 (owned set EMPTY, re-derived) · a11y chain **40/40, 0 new violation tuples** (pa11y 7/7) ·
`cargo xtask webview-drive` **17/17 GREEN** + `--no-inject --expect-absent traces-populate` RED arm PASS ·
mcp bindings regenerated · `cargo xtask capability-drift` **clean (0/0), run LAST**. 3 fix-loop iterations.
Boot smoke (direct-binary, fresh data dir, det-L4, default finite storm): every un-muted field renders
unredacted (150× l1a pair, 24 auto-resolve ticks numeric, `inference_mode` present, 0 `<redacted>` on the
un-muted set) · basename-only boot records, 0 full paths · corpus user_version=2 with exactly 5 tables ·
`event_kinds=[created, created, resolved, resolved]` · C8 workspace-key read-back CLEAN (one workspace ==
published key) · truthful `harness:status` exit 0 against the live pid · clean specific-pid kill, ports
released · 0 ERROR / 0 panics. Smoke artifacts: `%LOCALAPPDATA%/Temp/pulse-sweep-smoke-1a2W`.

## Pin #22 — session-60 between-point (recorded at the wrap)

probe skipped per ratified interval (next: 61). Basis + overlap re-verified first-hand anyway, per the
standing compact form: `cargo audit` still cannot load the RustSec DB — `error loading advisory database:
parse error: duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 read DIRECTLY (no pipe) — the pin's
signature reproduced byte-identically. Overlap: `cargo deny check advisories` exit 0; the owned
upgradeable set re-enumerated from scratch as DISTINCT `RUSTSEC-` ids = **EMPTY (0 ids)**, set identical
to the prior enumeration (empty since `2026-08-29-advisory-backlog`). `cargo deny check bans licenses
sources` exit 0 (`bans ok, licenses ok, sources ok`). No ordinal, per the 2026-08-28 ruling. Session 61 is
the next FULL-FORM interval point.
