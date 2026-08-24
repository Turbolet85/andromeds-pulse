# obs-plan.md — Amendments

Append-only history of amendments to `obs-plan.md`. **The body holds only current truth; this sidecar holds the
change history** (also in git). Each entry's substantive current-truth has been folded into the `obs-plan.md` body;
these entries record when/why each change happened. `Marker:` paths under `.andromeda/runs/` hold the full record.

---

## 2026-05-02 — Initial obs plan (Phase 3 synthesis)
**Section:** all (new plan).
**Change:** generated initial obs plan via `/andromeda-obs` Phase 3 synthesis — Standard tier; `tracing` 0.1 +
`tracing-subscriber` 0.3 (JSON) + `tracing-appender` + `tracing-error` as the self-observation runtime; external
OTLP wire via `tonic`/`axum`/`prost`/`opentelemetry-proto` (parse-only, no SDK runtime); mandatory panic hook →
`tracing::error!`; optional opt-in Sentry; frontend via `web-vitals` + TauRPC bridge (no browser OTel SDK); metrics
as `tracing` events with `metric.*` target prefix (no OTel Meter).
**Why:** `/andromeda-obs` Phase 3 synthesis.

## 2026-05-02 — Pivot to tracing-only self-observation (Phase 3.5 review)
**Section:** 1, 2, 3, 4, 5, 7, 8, 9, 10, 11.
**Change:** dropped the OTel SDK + `opentelemetry-stdout` exporter from the self-observation runtime; replaced with
`tracing` + `tracing-subscriber` JSON formatter as the sole self-observation primitive. Product's external OTLP
receivers unaffected.
**Why:** user feedback — eliminate the "OTel SDK monitoring an OTel monitor" recursion *by construction* (no
exporter exists → no recursion path) rather than guarding against it. (User quote on file in the amendment record.)

## 2026-05-02 — Manual OTel mental-model residue cleanup (post-Phase 5)
**Section:** 1, 4, 10, 11.
**Change:** removed 6 residual OTel-SDK mental-model leaks that survived the convergence loop — WebView2 "browser
OTel SDK" → `web-vitals` + TauRPC; `SpanKind` enum → `span.kind` field; criterion p50-vs-p99 mislabel → `jq` p99;
"perf-budget histogram p99" → `jq` p99; tokio "blocks export" → context-bleed (no exporter); Meter refs → `tracing`
event convention.
**Why:** manual edit, post-Phase-5 convergence. (Diagnostic note for the `/andromeda-obs` skill recorded in the
marker: add "architectural-invariant compliance" + "snippet API correctness" refinement dimensions.)

## 2026-05-04 — Clarify PII grep heuristic UI-vocabulary exemption  →  folded to §8
**Section:** 8, 11.
**Change:** clarified that PII vectors 1–6 forbid leakage of *real* secrets/values, NOT the literal UI-label words
"token"/"password"/"API key" (e.g. the "Token budget" accessible label). PII grep heuristics must distinguish
secret *formats* (regex envelopes) from UI vocabulary. **Now current-truth in obs-plan §8 (UI-vocabulary exemption).**
**Why:** chunk #14 phase #11 obs PII grep returned false positives on a11y SR fixtures documenting the "Token budget"
slider label.
**Marker:** `.andromeda/runs/2026-05-04T20-02-04-spec-amendment-clarify-pii-grep-ui-vocab/amendment.md`

## 2026-05-08 — Heartbeat ticks vs health command complementarity  →  folded to §3
**Section:** 3, 10.
**Change:** clarified that heartbeat ticks (async, 15s, emitted to JSON log; stall = no tick >45s; for retroactive
analysis) and the TauRPC `health` command (sync polling; for active boot/runtime liveness) are complementary, not
redundant, and must not be conflated. **Now current-truth in obs-plan §3 (complementarity paragraph).**
**Why:** cross-plan vocabulary drift surfaced during cross-plan review.
**Marker:** `.andromeda/runs/2026-05-08T17-28-28-spec-amendment-cross-ref-heartbeat-vs-health/amendment.md`

## 2026-06-10 — Chunk #99 tag gate: frame-budget posture + L4 budgets + load-suite findings  →  folded to §10
**Section:** 10.
**Change:** three operational findings (no §10 budget-value changes), now folded into obs-plan §10 as current-truth:
- **Frame p99 ≤33ms two-state posture** — NEUTRAL when no webview (headless run) → check scripts report NEUTRAL not
  FAIL; ACTIVE when booted app measures real frames; `write_run_window_log` scopes scripts to the run window.
  Chunk #99 verified both states (ACTIVE: frame p99 = 27.3ms over 56,642 real frames, buffer max 152.4MB ≤ 512MB,
  zero panics/ERROR).
- **L4 per-hardware-profile latency budgets** in `xtask/ci/l4-latency-p99` confirmed canonical (no re-derivation).
- **DuckDB connection-isolation pattern + load constraints** — 50k spans/s exposed 3 production defects, fixed
  in-chunk: (a) retention-sweep mutex starvation → dedicated sweep `try_clone()` connection + `MissedTickBehavior::
  Delay`; (b) timeout couldn't cancel a mutex-holding `spawn_blocking` query → dedicated read connection +
  `interrupt_handle().interrupt()`; (c) L1a read starvation behind row-group maintenance → dedicated read
  connection. Topology: write/sweep/read isolation on the same `:memory:` DB via `Connection::try_clone()`.
  Characterized-not-fixed: append path can block >60s on a ~12.7M-row table (mpsc absorbs; zero L0 loss; in-spec).
  Load-profile: sustained drain maintenance-tolerant (420s cap); nextest ceiling 20min; check scripts NEUTRAL-tolerant.
**Why:** chunk #99 tag-gate load-suite verification (sibling entry in test-plan §12, same amendment record).
**Marker:** `.andromeda/runs/2026-06-10T00-35-00-spec-amendment-adopt-four-load-profiles/amendment.md`

## 2026-08-14-fingerprint-feed-capture-repair — Fingerprint-feed counters + degraded-boot warn
**Section:** §1 Obs Scope Summary → Heartbeat ticks; §5 Metric Coverage → conceptual instrument types; §6 Log Coverage → log-level mapping (`warn` row); §8 PII Scrubbing → default-deny per-module allowlist
**Change:** `buffer.tick`'s field enumeration extended with `span_events_seen` / `fingerprints_computed` / `observer_invocations`; §5 gained a tick-aggregated counter row describing them (folded once per batch by `BufferState::record_feed_counts`, ridden out as tick FIELDS rather than separate `metric.buffer.*` targets, per-span-event emission barred by the §11 hot-path rule); §6 records the new `app.boot.buffer.degraded` WARN target (`reason` + `consequence`, once per boot); §8's `buffer` allowlist gained the three counters (aggregate counts only, never identity) plus an explicit `app.boot.buffer.degraded` leaf entry.
**Why:** The chunk ships this instrumentation permanently (operator decision at /phase P4). Its purpose is recorded in §5: equality of `observer_invocations` with `fingerprints_computed` makes a live feed a POSITIVE reading, where previously "nothing reached the storm detector" could only be inferred from downstream silence — the condition that let a suspected dead feed go undiagnosed. The §8 leaf entry is load-bearing: `for_target`'s fallback would otherwise resolve the dotted boot target to an unrelated `app`-prefixed field set and redact both fields.

## 2026-08-14-workspace-key-alignment — `app.boot.workspace_key` allowlist leaf + P7 chain
**Section:** §8 PII Scrubbing default-deny whitelist; §4 Scenario P7; §1 Critical paths (P7 row)
**Change:** Registered the new `app.boot.workspace_key` emission — explicit leaf entry permitting `workspace_root_basename` + `key_bytes` (INFO publish) and `error_category` + `error_detail` (WARN failure), basename and byte-count ONLY; the full workspace root path, the key value, and the key-file path are never emitted. Added the event to P7's must-trace chain and required-log-field lists at both restating sites.
**Why:** the chunk added the emission and the smoke verified it live — `workspace_root_basename="andromeda-pulse"` unredacted (proving the leaf entry resolves) with zero full-path occurrences in the log. An explicit leaf entry is required for the same reason as `app.boot.buffer.degraded`: `for_target`'s prefix fallback would otherwise resolve an `app`-prefixed dotted target to an unrelated field set and redact both fields. Basename-only follows §5 Vector 5 and the chunk #43 `workspace.detect` precedent.

## 2026-08-15-corpus-key-persistence — four corpus allowlist leaves (one a repair), the `*_PASSPHRASE` redaction convention, and the muted-diagnostic backlog
**Section:** §8 PII Scrubbing → Data classification rules (env-var row) · §8 Default-deny posture → Whitelist per module · §6 Log Coverage → Log levels mapping (`warn` row)
**Change:** Added explicit §8 leaves for `corpus.open.error` (a REPAIR — emitting since chunk #68 with no
resolvable entry, so its `error_kind` was redacted), `corpus.keychain.fallback`, `corpus.read.undecryptable`
and `corpus.orphan.disposition`, together with the invariant that NO bare `corpus` prefix key may exist.
Extended the secret env-var naming convention with `*_PASSPHRASE` and named
`ANDROMEDA_PULSE_CORPUS_PASSPHRASE` as never-emitted (not even as a presence flag). Recorded the three new
WARN targets in the §6 `warn` row with their emission discipline (once per boot / once per query, never per
row). Recorded a **muted-diagnostic backlog** for three further targets whose fields are redacted today —
`incidents.list_active.request` · `triage.incident.persist` · `triage.incident.corpus_restore` — with their
owner named.
**Why:** The §8 whitelist carried no corpus entry at all, so the shipped default-deny config and the doc's
enumeration disagreed, and the widening ban the new tests enforce was unrecorded. The `*_PASSPHRASE` gap
mattered more: §8 classified `ANDROMEDA_PULSE_*` as OK-to-log and redacted only `*_SECRET` / `*_TOKEN` /
`*_KEY`, so the doc's stated posture sanctioned logging a live secret the implementation deliberately
withholds. The backlog is recorded as measured-not-fixed because the arm-zero classification was actively
obstructed by those redactions — an out-of-band sqlite3 read was required — and a gap that costs an
investigation should be visible to the next reader rather than rediscovered.

## 2026-08-15-tier-1-incident-path-investigation — incident-path leaves landed; a five-target backlog replaces the three
**Section:** §8 PII Scrubbing & Compliance → Default-deny posture / Whitelist per module
**Change:** The three muted-diagnostic backlog targets are promoted from *measured-not-fixed* to landed EXACT allowlist leaves, restated as a sibling of the corpus-leaf block: `incidents.list_active.request` → `item_count`; `triage.incident.persist` → `incident_count`, `persist_kind`, **`duration_ms`**; `triage.incident.corpus_restore` → `kind`, `restored_incident_count`. The no-bare-`incidents`/`triage`-prefix-key ban is stated with its guarding test (`pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs`). A NEW backlog bullet records five FURTHER targets measured redacted in the same census — `metric.pipeline.l1a.query_count_total` → `query_name`; `metric.pipeline.l1a.query_latency_p99_milliseconds` → `duration_ms`/`query_name`/`row_count_returned`; `triage.cue.tick` → `bypass_triggered`/`cues_suppressed`; `triage.incident.auto_resolve.tick` → `duration_ms`/`evaluated_count`/`resolved_count`; `interpretation.model.load` → `inference_mode` — owned by the new "Diagnostics un-muting + harness-truth sweep" route entry.
**Why:** The chunk was the named owner of the three-target backlog and discharged it; all three were live-verified emitting unredacted on a post-fix boot (`item_count` observable forming 0→1→2). `duration_ms` is a third field the plan's list omitted — the emit site emits it, so a two-field entry would have left the target partly redacted, the same defect in miniature. The five further targets were measured by a full redaction census over the log family while verifying the three; applying only the promotion would have left §8 asserting a clean backlog while five measured gaps went invisible — reintroducing exactly the dark-diagnostic condition the backlog bullet exists to prevent. Applied as measured, with the impl half named to its owner (playbook 2026-08-15 routine-APPLY-AS-MEASURED).

## 2026-08-16-baseline-family-reachability — register the cold-start-window override WARN
**Section:** §8 PII Scrubbing → Default-deny posture (whitelist) · §6 Log Coverage → Log levels mapping
(`warn` row)
**Change:** Registered `triage.baseline.bootstrap_window.override` as an EXACT allowlist leaf carrying ALL
THREE fields its emit site emits — `resolved_seconds`, `default_seconds`, `reason` (bounded static label) —
with the explicit note that neither a bare `triage` nor a bare `triage.baseline` key may exist, since either
would widen every sibling `triage.*` target to one field set. Also added it to §6's once-per-boot WARN
enumeration beside `app.boot.buffer.degraded` / `corpus.keychain.fallback`, with the reason it fires: an
env-shortened or env-rejected bound silently changes when services become eligible for silence detection.
**Why:** The chunk added a new logging target and §8's default-deny posture listed no entry for it, so the
doc claimed these fields were redacted while production emits them un-redacted (measured in the smoke:
emitted once, fields visible). D-obs-pii fired at escalate severity; escalated to the operator ONCE and ruled
routine WITH them, because the fields are two integers plus a bounded label with an EXACT leaf and three
guard tests — registry completeness, not a PII hole. The resolution is codified as a new playbook rule so the
class does not re-escalate. The §6 entry is the duplicate-occurrence half: every sibling once-per-boot WARN is
stated in BOTH §6 and §8, so a §8-only apply would have left §6 silently stale.

## 2026-08-16-fault-identity-semantics-decided — register the interpretation.incident.created leaf (as measured)
**Section:** §8 PII Scrubbing → Default-deny posture (whitelist per module)
**Change:** Added the EXACT leaf `interpretation.incident.created` → `created`, `deduped` (bools) + `severity`, `priority_tier` (bounded static labels) — all four fields its emit site emits — with the no-bare-`interpretation`-prefix-key ban. Recorded AS MEASURED: the leaf has shipped in `AllowList::production()` since before this chunk and is live-verified unredacted, so §8 was closing a DOC gap, not a production one; and its field-completeness guard does NOT run (it sits in `observability.rs`'s own `mod tests`, dead under `[lib] test = false`), with only the PII half live-guarded by `pulse-app/tests/unit_incident_producer.rs`. Guard migration named to the "Diagnostics un-muting + harness-truth sweep" route entry.
**Why:** The chunk measured the gap (report §Spec claims disproved by measurement #3) and its plan named the amendment as due. Fields are bounded (2 bools + 2 static labels), so this is registry-completeness, not a PII hole — the same class as the 2026-08-16 `triage.baseline.bootstrap_window.override` precedent. Disposition ESCALATED and resolved WITH the operator as **APPLY-AS-MEASURED** because playbook condition (b) (guard lives where it runs) does not hold for the completeness half, putting the 2026-08-16 and 2026-08-15 rules in collision; the operator also approved codifying the dead-guard sub-case as a playbook refinement so the next occurrence resolves without an interrupt. No duplicate-occurrence amendment: `interpretation` appears elsewhere in obs-plan only in the muted backlog (`interpretation.model.load`, a different target, unaffected), and §6 enumerates once-per-boot WARN targets only, so this INFO per-incident target has no restating site.

## 2026-08-21-delegated-timing-observables — three delegated-timing metric leaves registered
**Section:** §5 (metric event naming); §8 (PII scrubbing → allowlist leaves)
**Change:** Registered `metric.constellation.hue_update_ms` (`duration_ms`, `severity_tier`), `metric.constellation.discovery_ms` (`duration_ms`, `discovered_count`) and `metric.findings.counter_refresh_ms` (`duration_ms`) as EXACT leaves, each enumerating every field its emit site emits. Recorded that the guard differs from the triage/corpus/interpretation families: a bare `metric` key legitimately EXISTS (`value`/`unit`/`module`), so the invariant is per-target-own-leaf rather than no-bare-prefix — without a leaf, `for_target`'s fallback keeps `value` and silently redacts every label.
**Why:** Raised by the orchestrator at Validate check 5 — the chunk plan queued §5 + §8 as expected amendments and no detector proposed them, so the plan's list served as the coverage floor. Guard lives in `pulse-app/tests/` where it runs, and its fallback-discrimination test was mutation-checked.


## 2026-08-22-pii-scrubber-recall — redaction counter registered on `buffer.tick`
**Section:** §1 Heartbeat ticks · §5 Metric Coverage (tick-aggregated Counter row) · §8 PII Scrubbing default-deny `buffer` whitelist
**Change:** Registered `redactions_applied` — a PII-redaction counter folded once per batch by the new `BufferState::record_redactions` and ridden out as a field on the existing 15s `buffer.tick`, at all THREE sites that restate the `buffer.tick` field set (a §8-only apply would have left §1 and §5 stale). Recorded as a deliberately SEPARATE fold from `record_feed_counts`, since the log-record path applies redactions while producing no fingerprint-feed counts, and scoped to the OTLP persistence path (`scrub_otlp_field`) — the drain/template path calls `scrub_attribute` directly and carries its own tick fields. Qualified aggregate-count-only: never the matched value, its category, the attribute key, or a `service_name` label.
**Why:** Chunk shipped the counter to make P-047 scrubber recall gradeable from OUTSIDE the process (CARRY #10) rather than only by reading stored rows back. Its allowlist leaf is guarded by `pulse-app/tests/unit_observability_allowlist_redaction_counter.rs` — under `pulse-app/tests/` where it actually runs, not the dead src-level module — asserting the full 12-field completeness plus the never-admit PII bans, and mutation-discriminated (neutralizing the leaf turned 2 of 3 guards RED; restore returned 3/3). Live at the wire: present and UNREDACTED on `buffer.tick`, advancing 0 → 1 after a bare-credential canary, with the canary literal appearing 0 times in the obs log.

## 2026-08-23-ingestion-scrub-coverage — `redactions_applied` scope widened to all four appender builders

**Section:** §5 Metric Coverage — Conceptual instrument types table, the `redactions_applied` row

**Change:** The counter's registered scope widened from the OTLP persistence path (`scrub_otlp_field`: log body + exception message + exception stacktrace) to ALL FOUR `buffer::appender` record-batch builders, adding the `spans.service_name`, `span_events.name`, `metrics_points.metric_name` and `log_records.severity_text` write boundaries — the spans and metrics builders gained the once-per-batch fold they previously lacked. The row now also states the **persisted-cells-only counting rule**: a scrub applied purely for identity consistency on a non-stored path is deliberately UNCOUNTED, because `extract_service_name` takes an `Option<&mut u64>` and its two non-storing callers (the storm `FingerprintObserver`, the baseline `SpanObserver` tap) pass `None`. The separate-fold rationale was generalized (it read as log-record-path-specific, which is no longer the only redacting path). Field set, label posture, tick cadence and the §11 per-field-emission ban are UNCHANGED.

**Why:** chunk `2026-08-23-ingestion-scrub-coverage` extended scrub coverage to four write boundaries and folded their redactions into the existing counter. The §5 row was the ONLY site carrying the retired scope wording — §1 (Heartbeat ticks) and §8 (default-deny allowlist) name the field without any scope claim, verified at this wrap, so no allowlist leaf edit and no restating-site edit were required. The persisted-cells-only rule is what makes a canary in all four columns read 4 rather than 5, observed live at the wire.

## 2026-08-23-metrics-points-identity — `redactions_applied` fold moved to the post-append site; PERSISTED-cells-only is now structural
**Section:** §5 Metric Coverage → Conceptual instrument types, the `redactions_applied` Counter row
**Change:** The four `buffer::appender` record-batch builders now RETURN their per-batch tally (`Result<Option<(RecordBatch, u64)>, Error>`) and `BufferState::record_redactions` is folded at each table's OWN post-append site in `consumer::dispatch_batch`, so a batch the appender rejects at `flush()` contributes zero. Per-table granularity recorded as load-bearing (a `span_events` failure must not discard the count for rows `spans` already stored), and `build_spans_record_batch` consequently takes no `&BufferState`. The "Counts redactions to PERSISTED cells ONLY" qualifier is marked structural rather than aspirational, with the pre-chunk counter-example recorded (`redactions_applied: 2` with `rows_ingested: 0` on a rejected batch, measured live). The stale "since the spans and metrics builders gained the fold they previously lacked" clause is retired.
**Why:** the rule this row states was measured FALSE at HEAD (report Spec-claims item 3) — the fold fired inside the builders before the append could fail. The chunk moved it, making the stated rule true. Verified single-site: the other two `redactions_applied` mentions (obs-plan.md:101 tick field list, :517 `buffer` allowlist) assert only tick-membership and aggregate-only shape, both still true, so no restating site needed amendment.

## 2026-08-23-metrics-points-labels — redactions_applied scope and unit
**Section:** §5 Metric Coverage → Conceptual instrument types, the `redactions_applied` row
**Change:** Registered scope restated as **FIVE scrub sites across the four** `buffer::appender` record-batch builders — the four column boundaries plus `metrics_points.labels`. The BUILDER count is still four; only the scrub-site count moved, so the tally pin's name stays accurate. The row now also states the counter's **unit** explicitly: one increment per REDACTED LABEL PAIR, not per persisted cell, since a single data point can carry several redacted pairs inside one `labels` cell. The trailing `extract_service_name` `None`-caller sentence correspondingly reads that a canary in all five scrubbed cells reads 5 and not 6.
**Why:** Chunk `2026-08-23-metrics-points-labels` added a fifth scrub site inside an existing builder. The persisted-cells-only rule and its structural basis (each builder RETURNS its tally; `record_redactions` folds at each table's own post-append site) are UNCHANGED — only the granularity of "a cell" needed stating, because a labels cell can carry several redactions. Measured at the wire: `redactions_applied` 2 (RED) → 4 (GREEN) = 2 credential metric names + 2 redacted label pairs. Duplicate sweep: the other "ALL FOUR" occurrence in this master (§8, `interpretation.incident.created` field completeness) is an unrelated claim and was left untouched.

## 2026-08-23-headful-leg-extension — §8 exact leaf: tray.signpost.shown → window_label
**Section:** §8 PII Scrubbing → Default-deny posture (whitelist per module)
**Change:** registered the exact `tray.signpost.shown` leaf — `window_label` ONLY (bounded via `sanitize_window_label`), no bare `tray` key; guard `pulse-app/tests/unit_observability_allowlist_close_signpost.rs` (mutation-checked RED 3/3 → GREEN).
**Why:** the chunk gave the previously-fieldless P-063 signpost emission one bounded field so the harness can assert a FIELD rather than record presence (report §Changes → Symbols; live wire-proof in the widget-close stage).

## 2026-08-24-headful-mechanics-probe-race-disposition — boot navigation-check target registered (dual-site)
**Section:** §6 Log Coverage → Log levels mapping → `warn` row · §8 PII Scrubbing → Default-deny posture whitelist
**Change:** Registered the new self-observation target `app.boot.window.navigation` at BOTH sites per the dual-site rule. §6 records it as an at-most-once-per-window-per-boot diagnostic (INFO when the window navigated, WARN when it is still `about:blank`), post-settle and off any hot path, with the rationale that a webview losing its initial navigation is never re-navigated and is otherwise indistinguishable from a healthy boot. §8 registers the EXACT leaf with all three emitted fields (`window_label` · `navigated` · `reason`), the bounded provenance of each, the outright ban on URL / title / coordinates, the measured fact that no bare `app` key exists (so the fallback resolves nothing and would redact all three), and the guard test under `pulse-app/tests/` with its mutation result.
**Why:** The chunk shipped the navigation check as the instrument that converts the launch race's production exposure from unmeasured to measured; without the leaf every field is silently redacted and the measurement reads as all-zeroes.
