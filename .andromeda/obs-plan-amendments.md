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
