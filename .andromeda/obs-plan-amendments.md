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
