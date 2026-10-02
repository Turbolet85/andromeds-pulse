# Scope — 2026-08-16-baseline-family-reachability

**Working-route entry (verbatim intent):**
> Baseline-family reachability — the silence and activity-floor families become reachable for a fresh
> service within a bounded warm-up, so Conductor's Epoch-3 family proofs are not gated on an hour of
> wall-clock (operator-directed 2026-08-14; evidence: Conductor two-launch-verdict.md §Re-run)
> · PREMISE CORRECTED 2026-08-14-fingerprint-feed-capture-repair: the storm path never touches the Ready
> gate — it runs buffer fingerprint hook → StormObserverAdapter → detector → cue broadcast, never reading
> BaselineState — so storm→cue→incident flows in seconds on a fresh dir (measured); the 3600s gate binds
> ONLY the baseline-derived families, which is what this entry now targets

---

## The outcome this chunk owes

A **fresh service** (no prior baseline state on disk) must be able to produce **baseline-derived** attention
cues within a **bounded, declared warm-up** — short enough that an external verification harness can prove a
family end-to-end inside one scenario run, instead of waiting out an hour of wall-clock.

The chunk is about **reachability**, not detection quality: the families already detect correctly once warm.
What is unreachable today is the *warm* state itself, on any fresh data dir.

## What binds today (the measured mechanism)

- `DEFAULT_BOOTSTRAP_WINDOW_SECONDS = 3_600` (`crates/triage/src/cue/thresholds.rs`), documented as mirroring
  `baseline::BOOTSTRAP_WINDOW_SECONDS` "so config-path / hot-reload stay consistent".
- The bootstrap gate is **time-since-first-observation**, not sample-count: the cue evaluator's own tests
  describe "requires now ≥ first + 3600s" and "Bootstrap done: now - first = ~4740s > 3600s"
  (`crates/triage/src/cue/evaluate.rs`).
- Two **sample-count** gates sit beside it and are a different axis: `MIN_EWMA_SAMPLES = 10` (gates
  `ErrorRateSpike`, `evaluate.rs:38`) and `MIN_LATENCY_SAMPLES = 50` (gates `LatencyRegression`,
  `evaluate.rs:88`). **VERIFIED** — and the verification narrows the chunk sharply: those two families are
  gated by SAMPLE COUNT only and never consult the hour. The 3600s wall-clock gate has exactly ONE consumer:
  `evaluate_service_went_silent` at `evaluate.rs:164` (`if snapshot.bootstrap_state != BootstrapState::Ready
  { continue }`).
- **A second, independent floor binds the silence family end-to-end:** a cue fires only when quiet duration
  exceeds `max(learned_p95, MIN_QUIET_SECONDS = 30)` (`evaluate.rs:174`), and only once
  `p95_historical_quiet_duration_seconds` is `Some` (which needs ≥2 observations, since the gap is recorded
  on the second `observe()`). So even with the bootstrap window driven to zero, the shortest possible
  silence proof is still **>30s of quiet**. Any "bounded warm-up" claim must state this floor.
- The storm path is **out of scope by measurement** — the premise correction above establishes it never reads
  `BaselineState`, so nothing here should touch storm→cue→incident.

## In scope

1. **The two families the entry names.**
   - *silence* — the `ServiceWentSilent` cue family (`evaluate_service_went_silent`, with
     `DEFAULT_QUIET_DURATION_PERCENTILE` / `MIN_QUIET_SECONDS` / `QUIET_TO_SILENT_FALLBACK_SECONDS`).
   - *activity-floor* — the per-service `ActivityFloor` tracker (`crates/triage/src/baseline/activity_floor.rs`,
     capabilities P-013 / P-014) and its `Learning` lifecycle state.
2. **A bounded warm-up that is declared, not incidental** — whatever mechanism lands (a shortened window, a
   sample-count-satisfied early exit, a config-surfaced override, or an explicit test/verification affordance),
   the bound must be a stated contract a harness can rely on.
   `[premise-corrected: the two constants are not merely un-single-sourced — the cue-side one is INERT.
   `cue::thresholds::DEFAULT_BOOTSTRAP_WINDOW_SECONDS` + its `Thresholds.bootstrap_window_seconds` field are
   defaulted and validated (rejects 0, `thresholds.rs:243`) but read by NO evaluation path; the effective gate
   is the hardcoded `activity_floor::BOOTSTRAP_WINDOW_SECONDS` (`activity_floor.rs:33`, applied at `:178`),
   and `evaluate_service_went_silent` takes `_thresholds` underscore-prefixed — deliberately unused. So
   "make the same value reach both constants" is INSUFFICIENT as stated: the cue-side field must actually be
   CONSULTED, or the baseline-side const must become the single source. A change to the config field alone
   would be a silent no-op.]`
3. **Reachability proven for a fresh service** — the acceptance is observed on a data dir with no prior
   baseline state, since that is the condition the external harness runs under.
4. **PREREQ folded from the working entry (absorbed by this chunk):** re-check `cargo audit` — a standing
   external-decay deferral since `2026-08-15-corpus-key-persistence`, re-ratified at pin #3 on 2026-08-16 with
   a **re-run interval of every 3rd wrap**. It ran at session 25; **the next interval point is session 28**.
   If this chunk's wrap is not an interval point, its report must record
   `probe skipped per ratified interval (next: 28)` — never a silent skip — and must re-verify the basis
   (upstream `duplicate advisory ID: RUSTSEC-2026-0244` DB-load failure) plus the named overlap
   (`cargo deny check advisories`).

## Boundaries (explicitly NOT this chunk)

- **The storm path** — measured never to touch the Ready gate; untouched here.
- **Detection quality / threshold tuning** — the families' correctness once warm is not in question.
- **Fault-identity semantics** — fingerprint normalization and incident dedupe are the *next* entry's
  decision; this chunk must not pre-empt it.
- **Conductor-side scenario changes** — that repo is READ-ONLY from here; this chunk changes Pulse so the
  harness's existing family proofs become runnable, and cites Conductor evidence by path only.
- **The 7 upgradeable RustSec advisories** — owned by the Advisory backlog entry, not absorbed here.

## Open scope questions (P3 outcome)

- **How many families?** `[premise-corrected: ONE, not two or three. The 3600s wall-clock gate has a single
  consumer — the silence family (`ServiceWentSilent`). *activity-floor* is not a separate family but the
  SUBSTRATE that computes silence's gate: `ActivityFloor` owns `first_observed_unix_nanos` → `BootstrapState`
  AND the quiet-duration t-digest that supplies the p95 threshold. *error-baseline-spike* (`ErrorRateSpike`)
  is gated by `min_ewma_samples = 10`, never by the hour, so it is already reachable in seconds under load
  and needs nothing from this chunk — the wrap dialogue's "immediate consumer" note is about scenario
  sequencing, not about this gate.]`
- **Which mechanism** delivers the bound — shorten the window, satisfy-by-samples, config override, or a
  declared verification affordance — remains a genuine design choice for P4, but is now informed: whichever
  is chosen must make the value ACTUALLY CONSULTED at `activity_floor.rs:178`, since that is the only site
  the gate reads. Blast radii still differ (a shortened default changes production detection for every user;
  an env/config override does not).
- **Whether the sample-count gates also bind these families** — `[premise-corrected: they do not bind
  silence at all. Silence is purely time-gated plus the `>max(p95, 30s)` quiet floor; `MIN_EWMA_SAMPLES` /
  `MIN_LATENCY_SAMPLES` gate the error-rate and latency families respectively, which this chunk does not
  target.]`

## Surfaces likely touched

- `crates/triage/src/baseline/activity_floor.rs` — the effective gate (`BOOTSTRAP_WINDOW_SECONDS` at `:33`,
  applied at `:178`). **VERIFIED as the load-bearing site.**
- `crates/triage/src/cue/thresholds.rs` (the currently-inert `bootstrap_window_seconds` field + its default)
  and `crates/triage/src/cue/evaluate.rs:157-166` (`evaluate_service_went_silent`, whose `_thresholds`
  parameter is unused today) — **VERIFIED**, and both are implicated by the premise correction above.
- `crates/ui-bridge/src/contract.rs::Settings` + `crates/config-watcher` — `[premise-corrected: NOT touched
  today. `Settings` carries no bootstrap/warm-up field (it has theme / widget_position / retention_seconds /
  mcp_server_enabled / notifications_enabled / always_on_top / snapshot_* / lifecycle_* / drain_* /
  cadence_*). Touching `ui-bridge` is therefore OPTIONAL and has a cost the plan must weigh: it trips the
  test-plan §3 boot-smoke gate trigger and regenerates the TauRPC TS bindings.]`
- Tests: co-located `#[cfg(test)]` in `crates/triage` per test-plan §4, with a paused-clock reachability
  assertion. **VERIFIED as the right location** (library crate).
- Observability: **no new emission needed** — `[premise-corrected: a positive warm-state signal ALREADY
  EXISTS and is already allowlisted. `crates/triage/src/cue/emitter.rs:66-86` counts
  `services_in_bootstrap` / `services_ready` and emits `triage.baseline.service_went_silent.evaluate`,
  registered at `pulse-app/src/observability.rs:1365-1384` with a guarding field-set + PII-ban test at
  `:3522`. The harness's reachability probe already exists; this chunk needs that counter to MOVE, not a new
  leaf. Confirmed no collision with the diagnostics-sweep entry, which owns the different target
  `triage.cue.tick`.]`

## Capability provenance

No P-0NN capability is named by this entry. **VERIFIED** — the version's three unclaimed caps
(P-075 / P-076 / P-077) are owned by later entries and none covers baseline reachability; this chunk claims
**none**.
