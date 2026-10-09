# Codebase Research — 2026-08-14-fingerprint-feed-capture-repair

## Scope
- **Depth:** moderate · **Reads:** 6 · **Globs/Greps:** 5 · **Graph queries:** 1 (77 rows)

## Headline

The wiring premise the hand-off could only assert from source is **confirmed at HEAD**, and the region it named is **narrower than stated**. The buffer-crate path from `run_consumer` to `on_fingerprint` is intact AND is exercised end-to-end by an existing e2e test, so the algorithm is not the suspect. What research newly surfaced is a **boot-time branch whose `None` arm silently discards every ingest batch while leaving ingest's own counter incrementing** — a mechanism that reproduces the measured signature (`span_count` 1→2→3→9 stable, every fingerprint field zero) exactly.

This does **not** settle the cause. It converts an unbounded region into a small set of discriminable branch points, each of which the live capture can separate by reading one already-emitted log target.

## Files inspected
- `crates/buffer/src/consumer.rs` (full) — `run_consumer` takes `fingerprint_observer: Option<Arc<dyn FingerprintObserver>>` (l.40), clones it per batch (l.59), hands it to `dispatch_batch` (l.68), which passes it into `build_span_events_record_batch(&s, fingerprint_observer)` (l.119) for every `Batch::Spans`. No filter, no early return on the observer path.
- `crates/buffer/src/appender.rs` (l.340-406) — the per-span-event loop reads all three `exception.*` keys from `event.attributes` (l.346-350), computes the fingerprint from the RAW type + stacktrace (l.358) **before** the PII scrub (l.366-367), and fans out to the observer at l.394-400 for every row whose fingerprint is `Some`.
- `crates/buffer/src/fingerprint.rs` (l.60-85) — `compute_exception_fingerprint` returns `None` only when `exception_type` is `None` or empty. `NoopFingerprintObserver` exists but is not on the production path.
- `pulse-app/src/main.rs` (l.590-604, l.1102-1135, l.1471-1500) — observer construction, the consumer-spawn branch, and `init_buffer`.
- `pulse-app/tests/e2e_storm_detection.rs` (structure) — `chunk_66_storm_detector_emits_suggested_at_5th_autonomous_at_10th` drives a real `ExportTraceServiceRequest` through `run_consumer` with a real `StormObserverAdapter` and asserts the Suggested/Autonomous thresholds.

## Graph impact (77 rows, `refs` over the four fingerprint symbols)
- **`FingerprintObserver#on_fingerprint`** — exactly two implementors: `NoopFingerprintObserver` (`fingerprint.rs:392`) and `StormObserverAdapter` (`storm_observer.rs:72`). One production call site: `appender.rs:396`.
- **`StormObserverAdapter`** — constructed at `main.rs:600` (production) and `e2e_storm_detection.rs:144` (test). No other production construction.
- **`compute_exception_fingerprint`** — one production call site (`appender.rs:358`); the remaining 20 refs are all inside `fingerprint.rs`'s own test module.
- **Zero production references outside `crates/buffer` and `pulse-app`** — the feed has no other consumer, so the blast radius of a repair here is confined to those two crates.

## Patterns detected
- **Observer-as-`Option`, injected at the binary boundary** (`main.rs:600` → `consumer.rs:40`): the trait lives in the lower crate, the concrete adapter at the root — the project's standing cross-crate state-delivery shape.
- **Fingerprint-before-scrub ordering is deliberate and commented** (`appender.rs:352-357`): content-stable fingerprints preserve cross-occurrence dedupe; scrubbing first would collide unrelated exceptions on the redaction marker. **This closes the security extract's P3 hypothesis — the mandated scrub cannot starve the fingerprint.**
- **Silent-drain fallback on a failed subsystem** (`main.rs:1130-1135`): when the buffer connection is absent the receiver is drained to keep the ingest channel from saturating. Deliberate backpressure hygiene — but it is indistinguishable, from ingest's counters alone, from a healthy path.
- **Boot outcomes are already observable** (`main.rs:1471-1500`): `init_buffer` emits `buffer.schema.init` (info, on success) or `buffer.schema.init.error` (error, with `error_type` = `open_in_memory_failed` | `schema_create`) and records `BufferConnectionStatus` on the heartbeat. No new instrumentation is needed to discriminate this branch.

## Conventions to follow
- **Test-time telemetry injection over the wire** — synthetic spans enter through the real receiver on `:4317`/`:4318`; no in-process bypass (`e2e_storm_detection.rs` is the worked example).
- **Aggregate-only self-observation on triage targets** — counts, enum tags, and numeric values; never `service_name` / `scope_id` / per-span identity in `agent-latest.jsonl` (`.claude/rules/observability.md`, 2026-05-17 + 2026-06-28 entries).
- **No per-span-event emission at `info` in the appender loop** — it is a named hot path; level-gate anything expensive.
- **pulse-app tests belong in `pulse-app/tests/*.rs`**, never `#[cfg(test)] mod tests` in `pulse-app/src/` — `[lib] test = false` means source-level tests compile under clippy but never run.

## Candidate branch points for the live capture (ranked, each discriminated by an existing log target)
1. **`buffer_conn == None`** → consumer never spawns; batches drained at `main.rs:1133`. Discriminator: `buffer.schema.init.error` present, or `buffer.schema.init` absent, or `duckdb.append` absent entirely.
2. **`buffer_conn == Some` but appends failing** → `duckdb.append` present at ERROR with `reject_reason` (`consumer.rs:79-91`).
3. **Spans append but events do not** → `duckdb.append` present with `table_name="spans"` but never `"span_events"`; means `build_span_events_record_batch` returned `Ok(None)` (l.386-388: no event rows survived the loop).
4. **Events append but carry no fingerprint** → `span_events` rows present with `fingerprint IS NULL`; means `exception_type` arrived `None`/empty — i.e. the attributes did not survive ingest, the one premise research could not close.
5. **Fingerprints fan out but the detector's counters do not move** → the defect is detector-side, downstream of this chunk's named region.

Branches 1-3 are answerable from `agent-latest.jsonl` alone; 4 needs a `span_events` query; 5 needs the storm-tick series.

## Files to modify
Provisional — the repair's site is the live capture's output, not a prediction. Instrumentation-side (deliverable 1) is knowable now:
- `pulse-app/src/main.rs` — the `match buffer_conn` branch at l.1102: the `None` arm currently drains silently. At minimum it warrants a one-time `warn` naming the consequence (the fingerprint feed and DuckDB appends are both inert this boot), so the condition is self-announcing rather than inferable only by absence.
- `crates/buffer/src/consumer.rs` and/or `crates/buffer/src/appender.rs` — a bounded, aggregate-only boundary counter making the feed's throughput observable (events seen / fingerprints computed / observer invocations), so "nothing reached the observer" is a positive reading rather than an inference from silence. Must be level-gated out of the per-event hot path.
- `pulse-app/src/observability.rs` — any new field is a deliberate `AllowList` extension; default-deny redacts silently otherwise.

## Open questions
- Do the canary's `exception.*` attributes survive ingest's post-`prost` invariant checks intact? → blocks: **implementation-scope** (it separates branch 4 from branches 1-3 and decides whether the repair sits in `ingest` or `buffer`).
- Does `e2e_storm_detection` still pass at HEAD? → blocks: **plan-decision**. The last full workspace run was 2026-06-28 (1681 tests); the Rust gate has been deferred across the five chunks since, and two hygiene commits (`6fbbd30`, `de00fbc`) touched Rust source with no wrap gate over them. The absorbed `PREREQ` closes exactly this — running the workspace suite tells us whether the guarding e2e is green, which decides whether this chunk starts from a working or a broken in-process path.
