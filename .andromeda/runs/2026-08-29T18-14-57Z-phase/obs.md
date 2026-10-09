# obs extract

## Relevance
Partial — the chunk is doc-side defer + Rust test pins with no new obs target, but obs owns one restating site of the halo non-render claim and the P-025 observable it distinguishes, and the det-L4 pins sit in the `interpretation.*` emit family obs governs.

## Constraints
- obs-plan §8 (delegated-timing leaves) requires `metric.constellation.hue_update_ms` to remain an EXACT leaf carrying exactly `duration_ms` + `severity_tier`, and requires its registered subject to be the constellation DOT's `severityToHueFraction`, explicitly NOT a Halo State Pulse canvas. The defer-disposition wording must not re-point, widen or narrow this observable; whether the code still emits it against the dot is research's question.
- obs-plan §8's `hue_update_ms` narrative is a RESTATING SITE of the halo claim — it asserts "that canvas has no production render site (measured 2026-08-21)" and cross-references design-system §Brand Identity Signature element by name. Per the dual/triple-site amendment discipline the plan enforces repeatedly (obs-plan-amendments 2026-08-24, 2026-08-26), a design-system flip to DEFERRED obliges checking this obs site at wrap; whether its own sentence stays true under the new disposition is a P3 determination, not a settled fact.
- obs-plan §8 (Default-deny posture) requires that ANY new `tracing` target land as an EXACT leaf enumerating every field its emit site emits, with the guard under `pulse-app/tests/` — a src-level `mod tests` guard compiles but never runs under `[lib] test = false`. The scope asserts no new obs target; if P3/P4 finds the pins emitting anything, this rule binds.
- obs-plan §10 (Standard+ invariants → Trace context propagation) and §6 (required fields → optional `trace_id` / `span_id` "when available") require correlation fields on cross-surface calls. The chunk pins `EvidenceRefs.trace_id` / `span_ids` / `timestamps_unix_nano` as hardcoded empty at the producer — the pin must record by-construction emptiness at the source, not a scrub outcome; whether that emptiness constitutes a §10 propagation gap is research's question.
- obs-plan §8 warns that a leaf naming fewer fields than the emit site emits leaves a target PARTLY redacted — a shape no resolver probe can see. The same failure mode governs the blind-spot pins: a probe asserting only that a value resolves/reads empty passes vacuously.
- obs-plan §9 + §10 (CI gates) require the verification legs to satisfy the zero-span, `app.panic.fatal`, and 45s heartbeat-gap gates; §10's frame-budget row mandates the two-state NEUTRAL-when-headless posture with `write_run_window_log` run-window scoping, which applies since this chunk boots no webview.

## Patterns to follow
- The exact-leaf-plus-guard pattern at `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` (obs-plan §8) — the file that owns the halo/dot distinction; it is the nearest precedent for where a halo-adjacent pin belongs and how it is mutation-checked.
- Fallback-discrimination: obs-plan §8 records the mutation proof that a probe asserting only `for_target(...).is_some()` passes while every field silently redacts. The pins should assert the constructive impossibility (unreachable constructor / hardcoded-empty producer), not merely an observed empty read.
- Applied-as-measured disposition shape (obs-plan-amendments 2026-08-15, 2026-08-28): record measured truth, name the owner and the target, never claim the impl half is done — the same shape the defer record and the 0.4.0 residual take.
- The observable-scoping narrative pattern in obs-plan §8: an entry names both the surface it measures and the surface it does NOT, which is precisely the distinction the defer wording must preserve.
- obs-plan §11 (hot-path ban) + §8 tick-aggregation precedent: aggregate once per transition/cycle, never per decision — binds only if a diagnostic is added, which the scope says it is not.

## Anti-patterns to avoid
- obs-plan §11 (Metrics): never leave a `metric.*` target without its own exact leaf — the bare `metric` key legitimately exists and keeps `value` while silently redacting every label (the mechanism behind the still-open `metric.pipeline.l1a.*` backlog).
- obs-plan §11 (Logs): never skip `trace_id` / `span_id` when available — so the EvidenceRefs pin must be phrased as by-construction emptiness rather than as evidence the correlation requirement is satisfied.
- obs-plan §11 (Telemetry Strategy / Spans): never add spans or emissions to hot paths for the sake of a pin — the pins are assertions about existing artifacts, not new instrumentation.

## Contract bindings
- obs §3 Harness contract ↔ tests §3: the JSON log schema is stated verbatim as obs-aligns-to-tests; obs's contribution to this chunk's test placement is the `[lib] test = false` rule (guards under `pulse-app/tests/`, not src-level `mod tests`).
- obs §8 `metric.constellation.hue_update_ms` ↔ design-system §Brand Identity Signature element (cross-referenced by name in obs-plan §8) and ↔ verification-matrix P-025, whose observable this is.
- obs §8 ↔ security (PII vectors 1–6): the defer record and the pins touch no user data; the empty EvidenceRefs fields must not be reported as a redaction result.
- obs §9/§10 CI gates ↔ tests harness: zero-span build-fail, `app.panic.fatal` gate, `xtask/ci/heartbeat-gap-check.sh`, and the NEUTRAL-tolerant frame-budget posture all run over this chunk's legs.

## Acceptance criteria contributions
- (obs) The chunk introduces no new `tracing` target and no `AllowList::production()` delta; if P3/P4 finds one, it ships as an EXACT leaf enumerating every emitted field with its guard under `pulse-app/tests/` (per obs-plan §8 Default-deny posture).
- (obs) `metric.constellation.hue_update_ms` keeps its two-field exact leaf and keeps the constellation DOT (`severityToHueFraction`) as its named subject — no re-point to the halo canvas, no leaf narrowing (per obs-plan §8 delegated-timing leaves, §5/§2 metric naming).
- (obs) obs-plan §8's halo cross-reference is checked at wrap as a duplicate-occurrence site of the design-system §Brand Identity claim, and either amended or recorded as verified-still-true (per obs-plan §8 + the dual-site discipline in obs-plan-amendments).
- (obs) The EvidenceRefs pin records `trace_id` / `span_ids` / `timestamps_unix_nano` as empty BY CONSTRUCTION at the producer, so absence checks over them are marked vacuous and §10's trace-propagation invariant is not read as satisfied by them (per obs-plan §10 Standard+ invariants, §11 Logs).

## Relevant amendment history
- **2026-08-21-delegated-timing-observables** (§5, §8) — the amendment that registered `metric.constellation.hue_update_ms` and wrote the halo-vs-dot distinction into obs-plan §8, including the "no production render site (measured 2026-08-21)" clause this chunk's defer disposition supersedes on the design side. Its guard lives where it runs and its fallback-discrimination test was mutation-checked. It also set the precedent this chunk's scope invokes: the plan's expected-amendments list is the coverage floor when no detector proposes an amendment.
- **2026-08-26-l4-runtime-security-residuals** and **2026-08-27-idle-observer-generation-damper** (§6, §8) — the two nearest amendments in the `interpretation.*` family whose emit sites (`pulse-app/src/inference_runtime.rs`) the det-L4 pins sit beside. They establish the exact-leaf + guard-where-it-runs + mutation-discrimination discipline and record that a bare `interpretation` key exists in code at `pulse-app/src/observability.rs:1934` in violation of the stated invariant — latent, owned by the "Diagnostics un-muting + harness-truth sweep" route entry.
- **2026-08-16-fault-identity-semantics-decided** (§8) — `interpretation.incident.created` registered APPLY-AS-MEASURED at the same producer file, with the dead-src-guard sub-case codified; relevant to H5 (does a guard already exist, and does it actually run).
- **2026-08-28-ingest-consumer-block-under-gap-resume** / **2026-08-28-ingest-consumer-initiating-freeze** (§10) — the record-a-measured-OPEN-defect rule: record the state with a named owner and never claim the impl half is done. This is the disposition shape the defer record and the 0.4.0 residual follow.
