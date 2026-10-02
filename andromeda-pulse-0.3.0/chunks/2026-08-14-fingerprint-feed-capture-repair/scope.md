# Scope — 2026-08-14-fingerprint-feed-capture-repair

**Version:** andromeda-pulse-0.3.0 · **Epoch:** 4 — Polish & ship: verification
**Working entry:** _Fingerprint-feed capture and repair — the storm detector's feed observed live, then the dead region between ingest receipt and the fingerprint hook repaired (operator-directed 2026-08-14; evidence: Conductor fingerprint-feed-verdict.md)_

## Outcome

A storm of identical exception span-events, delivered over OTLP the way any instrumented app delivers them, reaches the retry-storm detector's feed. The dead region between OTLP ingest receipt and the per-span-event fingerprint hook is **first localized by live observation, then repaired**, and the repair is proven by the detector's own latched counters under a canary-shaped storm.

This is a diagnose-then-fix chunk: **the cause is unknown by the evidence's nature.** No prior run has seen into the region, because the artifact that would show it (a span table) does not exist in the corpus. Localization is therefore in scope as work, not as a foregone conclusion, and the repair's shape cannot be fixed in advance.

## What this chunk delivers

1. **Live observation of the feed.** A run in which the path from OTLP receipt to the fingerprint hook is observable at its intermediate boundaries — enough to say *where* the events stop, not merely *that* they stop. Whatever instrumentation this requires is part of the chunk. **The instrumentation SHIPS PERMANENTLY** (operator-directed at P4): three atomic counters on `BufferState` surfaced on the existing 15s `buffer.tick` — no new tick, no per-event emission — plus a one-time warn when the degraded-boot branch fires. Rationale: this defect survived because "nothing reached the observer" could only be inferred from silence; permanence makes it a positive reading and puts the regression class under the existing obs gates. Rejected at the same gate: diagnostic-only scaffolding (removes the very thing that made the diagnosis possible) and boot-warn-only (leaves branches 3-5 silence-inferred).
2. **The repair.** The localized defect is fixed at its cause.
3. **Proof under a canary-shaped storm.** Identical-fingerprint exception span-events over OTLP move the detector's latched counters off zero, and the repair is guarded by a test at the tier the defect occupies.

## Acceptance signal — the counters, not the gauge

`tracked_fingerprints_count` is a **windowed gauge of DISTINCT fingerprints, sampled after eviction**. A fully healthy six-occurrence storm of *identical* fingerprints reads `1`, never `6`, and legitimately reads `0` once the 60s retention window closes. It cannot serve as the acceptance signal.

The latched, window-immune discriminators are **`storms_detected_total`** and **`fingerprints_evicted_total`** (both cumulative, both reset per process). `fingerprints_evicted_total ≥ 1` proves a fingerprint was tracked and later aged out, whatever the gauge reads. Detection is inline, not tick-driven — a Suggested-threshold cue fires on the occurrence that crosses it, so tick cadence is not a confound.

## Boundaries — explicitly NOT this chunk

- **The per-service baseline bootstrap gate** (next markerless entry). No incident is required here: the storm tick fires on its own cadence and occurrences record inline, so a `ready:false` preflight is an expected, non-blocking outcome for this chunk's evidence.
- **Workspace-key alignment** app↔sidecar (third markerless entry). No corpus read-back is needed to prove this chunk.
- **Conductor-side emission.** Proven good at the wire against a real collector stub: non-empty `trace_id`+`span_id`, an `exception` event carrying non-empty `exception.type`/`.message`/`.stacktrace`, distinct span identity with one shared fingerprint, resource `service.name` present.
- **The DuckDB append-path stall** under ~10 min of sustained 10k/s storm (pre-existing chunk-#99 connection-contention class).
- **Any redesign of the storm detector's thresholds, window, or cue semantics.** This chunk restores a feed; it does not retune what consumes it.

## Surfaces and contracts in play

- `crates/ingest` — OTLP receipt, post-`prost` invariant checks, mpsc hand-off. Arrival here is already **proven** by measurement (`span_count` 1→2→3→9, stable, in all three arms).
- `crates/buffer` — `consumer.rs` (holds `Option<Arc<dyn FingerprintObserver>>`), `appender.rs` (the per-span-event extraction of the three `exception.*` keys and the observer call), `fingerprint.rs` (`compute_exception_fingerprint`, which returns `None` only on a `None`/empty `exception_type`).
- `pulse-app/src/main.rs` — construction and wiring of `StormObserverAdapter` over `RetryStormDetector`.
- Self-observation (`tracing` only — never the product's own OTLP ports).
- No new TauRPC procedure, no new capability grant, and no new corpus table. **VERIFIED for the instrumentation half** — the counters, tick fields, and allowlist entries are entirely `tracing`-side and cross no IPC or storage boundary. The repair half remains contingent on the capture; the plan carries an arch acceptance criterion requiring any such resource to be registered in §Occupied Resources at wrap if the repair forces one.

## Absorbed annotations

**`PREREQ: close Rust gate deferral (deferred since 2026-07-06-incidents-panel-dropdown-layout-bug).`** This chunk is Rust-touching, so the deferral closes here. It has ridden five consecutive chunks as prose, never as a route annotation, so its age trigger never fired. Two commits (`6fbbd30`, `de00fbc`) additionally touched Rust source outside the chunk loop with no wrap gate ever run over them. Memory, not disk, is the live constraint (D: has 124.7 GB): build first at bounded parallelism under `CARGO_INCREMENTAL=0`, then run the workspace suite. Closure means the full Rust gate set green at this chunk's wrap.

## Premises — closed in research (P3)

- The production observer is a real `StormObserverAdapter`, not the `NoopFingerprintObserver`, and is constructed as `Some(...)` and passed into the consumer. **VERIFIED** — `pulse-app/src/main.rs:600-604` constructs `Some(Arc::new(StormObserverAdapter::new(...)))`; `main.rs:1110` passes `fingerprint_observer.clone()` into `run_consumer`.
- ~~The dead region lies in the `buffer` crate's consumer→appender path rather than in `ingest`'s decode or hand-off.~~ **[premise-corrected: the buffer-crate path is exercised end-to-end by the existing `pulse-app/tests/e2e_storm_detection.rs` e2e (OTLP export → `run_consumer` → appender → observer → detector, asserting Suggested at the 5th and Autonomous at the 10th occurrence), so the algorithm is not the suspect; the newly-surfaced candidate is the BOOT-TIME branch `match buffer_conn` at `main.rs:1102`, whose `None` arm spawns a receiver-drain task that discards every batch]**
- ~~Nothing upstream of the per-span-event loop drops or empties span events.~~ **[premise-corrected: `main.rs:1130-1135` — the `buffer_conn == None` arm spawns `while rx.recv().await.is_some() {}`, which drains and discards every ingest batch while leaving ingest's own counter incrementing; this reproduces the measured signature exactly]**
- The three `exception.*` attributes are read from the **event's** attribute list. **VERIFIED** — `crates/buffer/src/appender.rs:346-350` reads all three via `extract_string_attribute(&event.attributes, …)`.
- The mandated PII scrub does not starve the fingerprint. **VERIFIED** (closes the security extract's P3 hypothesis) — `appender.rs:358` computes the fingerprint from RAW `exception_type` + `exception_stacktrace` BEFORE the scrub at 366-367, and `exception_type` is never scrubbed at all ("class identifier, not user content").
- The canary's exception events survive ingest's post-`prost` invariant checks intact. **STILL OPEN** — not answerable by reading; it is one of the discriminations the live capture must make.
- The storm-tick line's fields reflect live detector state. **STILL OPEN** — the gauge's windowed/post-eviction/distinct semantics are established by the handing-off party's §1, but not re-verified here.

## Evidence

- `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/fingerprint-feed-verdict.md` — §2 the wire is intact (measurement), §3 thirty-one tick lines across three arms with every field zero and twelve in-window samples (transcribed SUT record), §4 the hand-off, §5 the live-leg recipe.
- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` §Re-run.
