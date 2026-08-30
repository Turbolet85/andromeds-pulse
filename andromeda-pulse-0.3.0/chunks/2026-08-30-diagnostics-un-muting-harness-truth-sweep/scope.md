# Scope — 2026-08-30-diagnostics-un-muting-harness-truth-sweep

## Intent (verbatim from the working entry)

Diagnostics un-muting + harness-truth sweep — the deferred targeted-cleanup cluster gets an owner, so a
diagnostic's silence stops being unreadable (operator-directed 2026-08-15 at the
tier-1-incident-path-investigation wrap).

## What this chunk is

A truth-in-diagnostics sweep. Every item shares one class: a diagnostic, harness, schema, or spec surface
that READS as live/true while being muted, mock, dead, or disproven — so silence or greenness is
unreadable. The sweep gives each item its fix or its recorded disposition. It is deliberately many small
items, accumulated by prior chunks' explicit deferrals; the working entry names this entry as their owner,
and two arch amendments are load-bearing on it (§Occupied Resources reserved-tables measurement; obs-plan §8
muted-target census).

All named coordinates below fold as HYPOTHESES per promotion.md — P3 re-verifies each at HEAD before it
shapes the plan. Items known to have moved since first recording carry their corrections inline.

## Family A — obs un-muting (the redacted/incomplete allowlist targets)

Core SCOPE, as corrected by the 2026-08-26-cadence-runaway CARRY (a): the census's "resolve to no allowlist
entry" mechanism holds for exactly ONE of the remaining targets; the repair for the others is COMPLETE the
leaf, not add one. Each target gets its own EXACT leaf — no bare `triage`/`incidents`/`metric` prefix keys
(obs-plan §8 names this entry as owner).

- A1. `metric.pipeline.l1a.query_count_total` → `query_name` — falls back to the bare `metric` key (keeps
  `value`, loses labels; ~100 redactions per run).
- A2. `metric.pipeline.l1a.query_latency_p99_milliseconds` → `duration_ms`/`query_name`/`row_count_returned`
  — same fallback mechanism, same volume.
- A2b. [research-extended 2026-08-30] TWO MORE l1a targets share the identical bare-`metric` fallback and
  are also unregistered: `metric.pipeline.l1a.q7_timeout_count_total` (`baseline/sql.rs:484` — fields
  `value`/`timeout_ms`/`rejection_reason`) and `metric.pipeline.l1a.q7_fallback_count_total` (`:494` —
  fields `value`/`fallback_query_kind`/`cause`). Same mechanism, same fix shape; the obs-plan §8 census
  closure should name all four l1a targets, not two.
- A3. `triage.cue.tick` → `bypass_triggered`/`cues_suppressed` — HYPOTHESIS: already un-muted at
  `2026-08-26-cadence-runaway-blocking-pool` (its master record says "Un-muted `triage.cue.tick` — a leaf
  COMPLETION"). Re-verify at HEAD; expected no-op reconciliation, reported per the 2026-06-01
  already-exists discipline.
- A4. `triage.incident.auto_resolve.tick` → `duration_ms`/`evaluated_count`/`resolved_count` — the ONE
  target that genuinely resolves to no allowlist entry (new exact leaf). Its muting has a MEASURED COST
  (2026-08-28 CARRY): a boot-smoke wait keyed on `resolved_count` rendered `"<redacted>"` and ran a 300s
  ceiling — silence that read exactly like "no auto-resolution occurred". Un-muting removes a live
  false-negative source for every future harness leg.
- A5. `interpretation.model.load` — measured full shape at HEAD: the exact leaf (`observability.rs:1962`)
  registers `[model_identity, tier, file_size_bytes, load_status]`; the llamacli emit sites (now
  `llamacli_inference.rs:239` loading / `:262` loaded — line drift from the recorded :216/:239) emit only
  `tier` + `load_status`; TWO MORE emit sites at `main.rs:514`/`:523` emit `inference_mode`
  ("deterministic"/"real") on this same target, and `inference_mode` is NOT in the leaf → renders
  redacted (the muted half). So three disposition classes on one target: `inference_mode`
  emitted-but-not-in-leaf (add to leaf); `model_identity` in-leaf-but-never-emitted (emit it at the
  loaded site — `semantic_name` is in scope there, a file_stem, basename-safe); `file_size_bytes`
  in-leaf-but-never-emitted with NO emit candidate (leaf-equals-emit-set discipline: remove or emit —
  plan decision). `ModelLoadEvent` broadcast (`contract.rs:82-92`) is a separate wire and unaffected.
- A6. A bare `interpretation` allowlist key EXISTS at `pulse-app/src/observability.rs:1934` carrying a
  populated set — violating the invariant obs-plan §8 and `.claude/rules/observability.md` both state.
  Latent today (every live `interpretation.*` target has an exact leaf; exact match wins); it widens the
  next target added without one. Fix the violation.
- A7. Two PRE-EXISTING boot records emit FULL filesystem paths — `app.boot.tracing.init` (`log_dir`) and
  `app.boot.pid` (`path`), both the resolved data dir (2026-08-26 operator-directed CARRY; security-plan +
  rules/security.md name them as the KNOWN CARRIED EXCEPTION owned here). Truth-alignment decision: adopt
  basename-only like every sibling, or state the exception AT THE EMIT SITES rather than only in prose.
- A8. `triage.cue.suppression_check` emits ONE RECORD PER RAW CUE PER TICK (4,599 in the wedge log) —
  obs-plan §11 hot-path concern, measured at cadence-runaway and deliberately left to this sweep.
  [research-refined: the target IS registered (`observability.rs:1406`) — this is a RATE defect only, not
  muting; the per-cue emit is `tracing::info!` at `emitter.rs:229` (fields cue_kind/persistence_seconds/
  restart_window_active/suppression_bypassed/bypass_reason), and the same target carries a separate
  legitimate lagged-subscription WARN at `:451`. `triage.cue.tick` already aggregates `cues_suppressed` +
  `bypass_triggered`.] Disposition owed (demote-to-debug / remove-per-cue / document).
- A9. NO-ACTION guard (intake #7, measured 2026-08-21, recorded so it is not re-derived): the `buffer.tick`
  trio does NOT reproduce at HEAD — `buffer.tick` resolves through the `.tick`-strip fallback to the
  populated `"buffer"` set, which contains `span_events_seen`/`fingerprints_computed`/`observer_invocations`.
  If seen redacted on a live leg, the cause is elsewhere — re-measure before acting.

## Family B — harness truth (the harness stops lying)

- B1. `agent-run.sh status` TRUTHFULNESS — root cause diagnosed 2026-08-16, mechanism
  [premise-corrected: measured at HEAD — no `tauri::test::mock_builder` involved; `harness_status()`
  (`xtask/src/main.rs:278`) calls `ui_bridge::health::current_health()` IN XTASK'S OWN PROCESS, returning
  the resolver's envelope about xtask itself]. Same conclusion stands: it is STRUCTURALLY incapable of
  reporting on the process under test (exit 0, well-formed envelope, with NO pulse-app running). Both
  `agent-run.sh` (`:49` boot-readiness poll + `:72` status verb) and `agent-run.ps1` (`:37`) consume it —
  the boot-readiness poll is equally vacuous. Fix so `status` reports on the REAL process (the registered
  `run/andromeda-pulse.pid` resource + the app's own log family) or fails honestly. Secondary observation,
  disposition owed: the `boot` verb runs `cargo run --bin pulse-app --release` (`agent-run.sh:42`),
  forcing a release rebuild — chunks route around it via the test-plan §3 direct-binary variant; align
  the harness or the doc so the documented path is the used path (verified at HEAD).
- B2. The `agent-latest.jsonl*` bare-name family — [premise-corrected: measured at HEAD, the impl half
  LANDED at `2026-08-25-demo-injector-formalized-api-surface-retire`: `xtask/src/smoke.rs::read_jsonl_lines`
  (`:516-519`) resolves the daily-rotated family, `scripts/agent-run.sh` logs verb glob-loops the family
  with precedence-ordered bases, and test-plan §3 already documents the rotated form. What remains is the
  OWED DISCRIMINATING TEST — test-plan §1 pending trigger `harness-log-family-resolution-coverage`: the 6
  existing `assert_log_invariants` tests pass with a bare name that still matches the prefix, so the
  family branch (only a date-suffixed file present) is never entered.] Land that test.
- B3. test-plan §3 warm-boot doc-fix family — [premise-corrected: the term "warm" appears NOWHERE in
  test-plan.md at HEAD, and no enumerable warm-boot defect survives in §3 (re-synced 2026-08-25 rotated
  family + direct-binary variant + status caveat); "warm-boot" in the route archive describes operator
  VERIFICATION style, not a §3 defect]. Narrows to: at wrap, verify §3's harness-invocation lines still
  match B1/B2's landed fixes and amend only what measurement shows stale (folds into D3).
- B4. 130+ `#[test]` fns in `pulse-app/src/observability.rs` NEVER EXECUTE — `[lib] test = false`
  (`pulse-app/Cargo.toml:12`, the WebView2 workaround) makes a src-level `mod tests` (opens `:2637`, with
  an in-file comment at `:127` admitting "compiles but never executes") compile, pass clippy, and never
  run (`grep -c` = 130 at HEAD; the entry's 132 has drifted by 2). Includes the guard on
  `triage.baseline.service_went_silent.evaluate` — a guard that cannot fail protects nothing. Precedent:
  the 2026-05-20 migration (47 tests → `pulse-app/tests/`, workspace count jumped, proving they had never
  run). The discipline is curated THREE times in `rules/testing.md` (:25, :185, :226) and was still
  violated — so prefer a MECHANICAL guard (count assertion or lint) over a fourth restatement, alongside
  the migration.
- B5. `xtask::self_verify::launch_pulse` spawns the app with NO `current_dir`, so TauRPC dev-mode
  `export_types()` writes a stray `<root>/ui/src/bindings/index.ts` every run — papered over by
  `.gitignore:51 /ui/`. One directory level different and it would CLOBBER the real bindings. Fix: point
  the launch CWD at the per-run temp data dir, as `xtask::webview_drive::run_driver` does (note
  `self_verify.rs:254` already sets `current_dir` for the a11y leg — the omission is launch-site-specific).
- B6. Boot-geometry observation (P-061 residual; re-homed here 2026-08-25 by operator ruling with its OWN
  disposition — the drag-decline reason does not apply to it, and the same probe measured
  `geometry_readable: true`). Scope: assert the compact widget boots at the margin-inset corner
  (`WIDGET_EDGE_MARGIN = 24`, `compute_snap_position` at `pulse-app/src/window.rs`) by reading
  `outer_position` through the driver's existing `windowInvoke` transport — a cheap DOM-half-only stage on
  the headful leg (entry says 15-stage; VERIFIED at HEAD: 16 stages in `xtask/src/webview_drive.rs` STAGES
  (`:55`-`:138`), and the driver already reads `outer_position` via `windowInvoke` for the main window at
  `webview-drive.mjs:408` — the transport needs only the `compact-widget` label). Also strengthens P-061's matrix ref: affordance half closed by operator manual check
  2026-08-25; the geometry half is the piece that can be automated.
- B7. `pulse-app/ui/tests-a11y/` carries ZERO copy-state coverage (measured 2026-08-27): the p9
  diagnostic-report axe spec audits the settled surface only, so the REJECTED and IN-FLIGHT copy states are
  unaudited even though `__mockReject` / `__mockDelayMs` sentinels exist for exactly this (a11y-plan §3
  "Adding a surface"). Add the coverage.
- B8. Source comment at `crates/buffer/src/schema.rs:320-323` states a DISPROVED cause ("duplicate-INSERT
  path was observed to hang in libduckdb-sys 1.10502") — measured FALSE at
  `2026-08-28-duplicate-span-replay-fails-loudly` (plain prepared duplicate INSERT errors in 0.05s at that
  same version; the hang was Arrow `Appender::flush()`-specific, closed at 1.10505). test-plan §4 was
  corrected at that wrap; the COMMENT was not (wrap does not edit source). Delete the disproved cause from
  the comment; the `information_schema` test route itself may stay (it asserts the contract).
- B9. Misleading comment at `crates/triage/src/cadence/coordinator.rs` — the non-Autonomous arm claims
  Suggested cues "flow through their dedicated channels": TRUE for BaselineState-derived cues (`emit_cue`
  forwards to `CadenceTriggerChannel`), FALSE for storm-derived ones (`observe_and_dispatch_storm` never
  forwards — a Suggested storm is dropped outright; measured 2026-08-15). Fix the comment to the measured
  truth. (Whether the DROP itself is a defect is NOT this sweep's call — the comment is; note the seam for
  the route if research shows it product-visible.) Comment-only treatment verified appropriate: the
  comment sits at `coordinator.rs:416-417` at HEAD, and the drop behavior is unchanged.

## Family C — pipeline truth counters + dead-schema dispositions

- C1. Append-reject AGGREGATE counter — an `append_rejections` fold on `BufferState` behind its OWN exact
  allowlist leaf, mirroring `record_redactions` (once per batch, never per row). Two silences it closes
  with ONE counter: (i) the span-side PK-reject → fingerprint-observer coupling is SILENT — a span rejected
  on the composite `(trace_id, span_id)` key never reaches the fingerprint observer, so a colliding-id
  producer yields an UNDETECTABLE storm whose detector counters stay flat and read EXACTLY like a dead
  feed (measured in BOTH directions on live legs, 2026-08-16; the 2026-05-14 fixture rule in
  rules/testing.md is the consumer-side half); (ii) the general count gap — the boundary DOES log the
  rejection at ERROR on `duckdb.append` with `reject_reason` (obs-plan §10 satisfied; not a silence
  defect), but `rows_appended` counts rows REQUESTED (`record_batch.num_rows()` computed BEFORE the append)
  and can never witness a partial or total non-landing, while ingest's counter advances upstream.
- C2. Corpus `baseline_state` TABLE is DEAD SCHEMA — zero readers, zero production writers: created by DDL
  (`corpus/schema.rs:27,36`), listed in the orphan-disposition enumerations (`disposition.rs:25,37`),
  written by exactly ONE `INSERT` under `#[cfg(test)]` (`disposition.rs:355`), `SELECT`ed nowhere. Real
  baseline persistence round-trips through `pipeline_metrics` under `metric_name = "baseline_state"` /
  `layer = "l1b"` (`pulse-app/src/baseline_persistence.rs:47,78,95`). Its 0-row reading is the CORRECT
  value — and the same-name collision is exactly what makes a reader take the empty table for a broken
  path. DECIDE drop-vs-document; dropping is a corpus DDL change needing a `SCHEMA_VERSION` decision
  (measured 2026-08-16 at operator request). OPERATOR FORK at P4/P5.
- C3. THREE reserved DuckDB tables have NO PRODUCER — `resources`, `instrumentation_scopes`, `span_links`:
  declared in `crates/buffer/src/schema.rs`, swept by `crates/buffer/src/retention.rs` (`:25`,`:28`,`:29`),
  written by nothing (`append_record_batch_to_table` is the only write path; fires for exactly `spans`,
  `metrics_points`, `log_records`, `span_events` — measured 2026-08-23). Three of seven retention DELETEs
  sweep permanently-empty tables. Binary disposition: a producer lands (and
  `instrumentation_scopes.scope_name`/`.scope_version` re-enter scrub coverage — security-plan §Logging
  clause reopens) OR all three are dead schema whose DELETEs go with them. Arch §Occupied Resources +
  §Conventions were amended AS MEASURED and NAME THIS ENTRY as owner of the impl half — load-bearing, not
  optional cleanup. OPERATOR FORK at P4/P5.
- C4. `persistence_seconds` THREE-WAY DIVERGENCE — the field NAME says seconds, the assignment is a raw
  SAMPLE count (`let persistence_seconds = snapshot.samples;`, `crates/triage/src/cue/evaluate.rs`,
  verified 2026-08-21), the capability spec says spike duration; the tier boundary
  `persistence_seconds >= 30` (`crates/triage/src/cue/classify.rs:23`) therefore gates on 30 SAMPLES —
  product-visible, not merely naming. Corroborated by Conductor's measured 60s rotation, persistence
  2456→68 reset (intake #6). Fix decision informed by P3 research (rename-to-truth vs convert-to-seconds —
  changes tier behavior).
- C5. `incident_events` records ONLY `created` — 59 incidents × exactly one `created` each over the
  preserved 11h corpus; zero `resolved`/`acknowledged`/`read` kinds ever persisted, so the corpus-side
  lifecycle is unreadable from the events table alone (the day's 36 resolutions left no event trail).
  An events table that looks like a lifecycle ledger and records one kind misleads every forensic read.
  Disposition owed: persist the other kinds, or document the table's actual contract. FORK.
- C6. `pulse://stream/incidents` is a REGISTERED topic with PRODUCERS BUT NO CONSUMER (measured
  2026-08-29): `IncidentLifecycleBroadcast` constructed at boot, the auto-resolve observer and the
  `incidents.*` router `send()` into it, nothing calls `.subscribe()`, nothing bridges it to the webview.
  Arch §Occupied Resources lists it among live channels, so a reader assumes a consumer exists (the
  reconciliation chunk deliberately emitted NO lifecycle event because of this). Decision owed: wire a
  consumer, or record the topic as producer-only in arch. FORK.
- C7. intake-#12 residual (mechanism FOUND and FIXED at `2026-08-27-incident-persist-vs-resolve-write-race`):
  the remaining half is re-reading `incidents.list_active.request` → `item_count` CLEAN post-fix.
  Coordinate corrected 2026-08-28: the exact leaf is at `pulse-app/src/observability.rs:2321` (not the
  `:2242` first recorded); the emit site `incidents_router.rs:221` holds. Verification task — rides the
  chunk's live leg.
- C8. Second incident (18:49:56) invisible to the data-dir workspace key (intake #5, relay 2026-08-21;
  dedupe-precedent sibling) — an OPEN INVESTIGATION whose outcome is unknown until driven, not an
  assertion with a known shape (why it could not be scoped in the P-076 chunk). The assembled headful leg
  now makes it reachable: drive the path and read back.

## Family D — spec-truth doc fixes (wrap-owned amendments)

These are MASTER-side corrections. Phase/implement are read-only on the spec masters; the chunk plans them
as WRAP amendments carried by this chunk's report (the established per-chunk amendment channel), never
implement-time edits. [inferred: process framing — the entry names the fixes, not the mechanism]

- D1. security-plan §Input Validation MCP row + §Threat Model Summary MCP vector enumerate FOUR tools while
  EIGHT have shipped since chunk #94 — align both sites (surfaced 2026-08-29 by the security
  drift-detector, correctly declined as pre-existing).
- D2. layout-templates :17 + :145 name "shadcn/ui (Radix UI primitives)" as the component stack while
  react-aria-components is the shipped library and the radix family is lockfile-absent AND denylisted by
  `pulse-app/ui/npm-policy.json` (measured 2026-08-30) — align both sites to the shipped stack.
- D3. test-plan §3 alignment (= B3, premise-corrected above): no warm-boot family exists at HEAD; the owed
  §3 amendments are whatever B1's status/boot fixes make stale (the §3 `status` caveat "exits 0 against a
  dead system" becomes FALSE once the fix lands and must be updated to the new contract; the `boot` verb
  line if changed). Enumerated at wrap against the landed shape.

## PREREQ (rides the chunk, discharged at wrap)

- Pin #22 — `cargo audit` standing deferral: **session 60 is a BETWEEN-POINT** — re-verify basis + overlap
  first-hand (`cargo audit` load failure byte-comparable; `cargo deny check advisories` + `bans licenses
  sources`), re-enumerate DISTINCT `RUSTSEC-` ids from scratch (owned set was EMPTY at session 59), record
  `probe skipped per ratified interval (next: 61)` in the chunk report. No "Nth consecutive" ordinal.
  Session 61 is the next FULL-FORM interval point — NOT this chunk.

## Boundaries (explicitly NOT this chunk)

- The staged-bindings assertion (its own next route entry) and ACL-rejection logging (the entry after) —
  even though both share the truth class.
- Fixing the Suggested-storm DROP itself (B9 fixes the comment; the seam's behavior is route material if
  product-visible).
- The `cargo audit` FULL-FORM probe (session 61, next chunk's wrap).
- Any scrub-coverage expansion for `instrumentation_scopes` UNLESS the C3 fork chooses "producer lands"
  (then the security-plan clause reopens as that fork's cost, sized at P4).

## Surfaces / contracts touched (planning signal, subject to P3)

`pulse-app/src/observability.rs` (allowlist leaves + never-run tests migration) ·
`pulse-app/src/llamacli_inference.rs` (model_identity/inference_mode emit) · `crates/buffer` (schema
comment, append_rejections fold, possibly dead-table DDL + retention) · `crates/corpus` (possibly
baseline_state DDL + SCHEMA_VERSION) · `crates/triage` (coordinator comment, cue evaluate/classify naming,
suppression_check disposition) · `xtask` (harness:status truthfulness, smoke.rs reader, self_verify CWD) ·
`scripts/agent-run.{sh,ps1}` (status + reader) · `pulse-app/ui/tests-e2e/webview-drive.mjs` (+ xtask
webview_drive stage table: boot-geometry stage) · `pulse-app/ui/tests-a11y/` (copy-state coverage) · wrap
amendments: security-plan, layout-templates, test-plan §3, obs-plan §8 census closure, possibly arch
(stream topic + reserved tables + corpus schema).
