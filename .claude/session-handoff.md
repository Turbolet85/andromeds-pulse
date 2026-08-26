# Session Handoff

**Last Updated:** 2026-08-26T10:12:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 28 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-26-ingest-consumer-stall-under-sustained-load): a wedged consumer stops reading as healthy`

## Position
- Done: **2026-08-26-ingest-consumer-stall-under-sustained-load** — the observable half shipped in full (drain-progress fields, transition WARN, exact allowlist leaf, a failing gate), plus the obs-plan §10 isolation fix that `crates/viz` had been violating. The ROOT CAUSE was attributed but **not fixed** — it lives outside the chunk's touchpoints.
- Next (first markerless, **new this wrap**): **Cadence runaway starves the blocking pool** — the unfinished half of this chunk. It carries the re-pinned `cargo audit` PREREQ, **next interval point 46** (i.e. the next wrap).
- Then: L4 runtime security residuals · Interpretation brief completeness · Halo State Pulse canvas disposition · Advisory backlog · npm advisory coverage · Diagnostics un-muting · Staged-bindings assertion · Metrics label surface.

## Work done
**Deliverable B complete.** `BufferState` gained `last_append_at_nanos` (refreshes every append, deliberately distinct from the latch-once `first_append_at_nanos`); `buffer.tick` went 12 → 14 fields with `rows_ingested_delta` + `last_append_age_seconds`; a once-per-transition `buffer.consumer.stalled` WARN fires after 30 non-draining ticks (450s, clearing obs-plan §10's 420s in-spec cap); `cargo xtask check:ingest-progress` fails a run on that record and is NEUTRAL-tolerant on an absent stream. The classifier is the point: `rows_ingested` alone cannot separate a stalled consumer from an idle producer — both leave it static, which is exactly why the wedge read healthy. Channel occupancy is the discriminator.

**Deliverable A attributed, on evidence, and it falsified this chunk's own hypothesis.** The 15-min RED leg at HEAD reproduced every stated precondition and stayed healthy (47,898 rows, channel 0.0 %, 0 ERROR). Attribution came from the predecessor's own wedge log, still on disk: both `spawn_blocking` users died (`duckdb.append` 17:20:50, `viz.query.traces` 17:24:13) while every async task ran 16 minutes longer — and they use DIFFERENT mutexes, so the shared resource is the tokio blocking pool, flooded by a **122× runaway** (26,821 L1a queries vs 220; overseer-verified). Max append duration 11 ms both runs, so DuckDB contention is out.

**Gates:** fmt · clippy all-features 0 warnings · **nextest 1949/1949 + 1 skip** (baseline 1931, +18 then +1 at wrap = 19, accounted exactly) · capability-drift clean (after the documented bindings regen) · capability-widening clean · deny bans/licenses/sources ok · `check:ingest-progress` PASS. Webview gates excluded — bindings content-identical to HEAD. **Two mutation checks, both discriminating:** the gate's 4 arms (`stalled` → exit 1; `recovered` correctly green) and the classifier (collapsing it reddened exactly the 2 pins guarding the collapsed branches).

**Smoke:** two Direct-binary legs, fresh data dir each, real L4. GREEN leg — 42/42 ticks carry both new fields unredacted, 0 `viz.query` fallback WARNs, gate PASS. No orphans; `:4317`/`:4318` released.

## Drift resolved
**12 amendments · 0 escalations · cascade closed · drift = 0.**
- **obs-plan** ×9 across §1/§3/§5/§6/§8/§10 — the `buffer.tick` field pair at all THREE restating sites, the WARN target at BOTH §6 and §8, the stall definition qualified at all three sites that restate it (tick presence is LIVENESS, not PROGRESS), and the connection-isolation topology extended to viz.
- **test-plan** ×3 — `check:ingest-progress` into the §3 standard gate set (with `capability-drift` moved LAST per the 2026-08-22 ordering rule), a new §1 trigger `viz-read-connection-router-wiring-coverage`, and the §4 viz bullet.
- **Cascade** — 8 leaf sites re-derived across 5 files (`rules/observability.md` ×4, `docs/obs-summary.md` ×3, `rules/verification-harness.md`, `docs/tests-summary.md`, `rules/testing.md`). Two near-miss sites (obs-plan `:620`, `docs/workflow.md:49`) were READ and deliberately left — they describe what the heartbeat-gap script does, which stays true. 5 of 7 detectors returned clean.

## Notes
- **The root cause is un-fixed and now owns the first tail entry.** Its research must answer the TRIGGER question per operator directive: what made THAT run's loop spawn 122× while a matched leg stayed at baseline — conditions, not just mechanism. Evidence preserved at the predecessor session's `scratchpad/l4run-171923/`.
- **Operator ruling (durable):** a chunk's own falsified premises have NO sanctioned writer at wrap — `research.md` is never mutated, `scope.md`'s windows are closed. Their home is the report's `Spec claims disproved by measurement` bullet. Neither artifact was edited; curated as Tier 1.
- **Two pre-existing arch inconsistencies surfaced by the detector and correctly NOT amended under this marker** (routine-HANDOFF class): arch's "single in-memory `:memory:` DuckDB connection" line (already inaccurate before this chunk — retention and L1a clone), and the "twelve library crates" count word vs the sixteen names in §Occupied Resources. Both want a targeted doc-truth touch-up.
- **A real coverage gap is recorded, not closed:** nothing pins that the three viz router constructors actually CALL `read_connection` — reverting them would silently restore the §10 violation with every test green. Tracked as the new test-plan §1 trigger; verified first-hand at wrap, deliberately not fixed in code (one wrap-time code edit was already a recorded process deviation).
- **Curation:** T1 ×1 (the falsified-premise ownership ruling) · T2 ×1 (the allowlist fallback's THIRD shape — a bare key carrying a POPULATED sibling set, so the target resolves and still loses every field) · T3 ×1 (attributing a defect that will not reproduce). Filters: 1 dup · 1 recurrence → below. CLAUDE.md **156/200**.
- Audit trail: `.andromeda/runs/2026-08-26T09-45-00Z-wrap/` (+ the phase run dir).
- Last failed command: none.

## Deferred learnings
- **`recurrence-despite-learning`** — the output-masking family fired TWICE more against my own probes, past both a rules entry and the previous handoff's identical note. (1) The gate-discrimination harness read `rc` from a pipeline ending in `tail`, so the RED arm reported exit 0 and printed "GATE PROOF FAILED" — the gate was correct and had printed FAIL; only the measurement was wrong. (2) A `grep -hc … | paste -sd+ | bc || echo 0` tick counter printed 0 while the ticks plainly existed (a trailing CR breaks `bc`; the `||` swallows it), which also made a bounded wait loop run all 24 iterations. Both caught by reading the substantive artifact rather than the derived number. Per Filter 1 this is logged, not re-curated — a fourth restatement is not the remedy; a check in the owning step's reference is.
- Still open from prior wraps: the **deferral-destination generalization** (a capability whose `ref` defers evidence to another cap must be re-checked when that destination completes — a destination can complete WITHOUT absorbing the deferred evidence, leaving a hollow `verified`).
