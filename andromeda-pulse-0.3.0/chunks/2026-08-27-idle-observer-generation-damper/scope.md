# Scope — 2026-08-27-idle-observer-generation-damper

**Chunk:** Idle-observer generation damper
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Promoted:** 2026-08-27 (phase run `.andromeda/runs/2026-08-27T17-17-08Z-phase/`)
**Priority:** HIGH (operator ruling 2026-08-27)

**Working-entry intent (verbatim anchor):** a quiet system stops burning standing GPU: unchanged conditions re-analyze rarely, and hanging-incident churn stops defeating auto-resolve.

## Product bar (operator, 2026-08-27, for the record)
Idle-observer GPU uptime must be MINIMIZED — a workday of wattage on zero ingest defeats the product's own footprint promise. This is the bar the chunk's acceptance is judged against, not a nice-to-have.

## Problem (measured 2026-08-27 over the preserved 11h08m record)
Evidence: `D:/dev/evidence/pulse-l4run-20260827-064312/` (`analysis/attribution-11h.txt`; boot 04:43:20.712Z) — dir and file verified present at promotion; log re-read first-hand at P3 research. The numbers were re-derived first-hand at the 2026-08-27 wrap (handoff §Notes: 7,360 records / 3,680 generations exact; first silence cue 05:43:36.480Z).

- **3,680 real generations · 4.35 GPU-busy hours · 39.1% duty on ZERO ingest after minute 8.**
- Driver attribution: `service_went_silent` cues → `cadence_tier1` digests = 3,011/3,680 (82%) at ~5/min. The rate TRIPLE-LOCKS: 3,011/11.13h ≈ 5/min = 5 injector services × the 60s CueLatch refractory — rate × services × latch all agree. *(Verified at P3: `CUE_LATCH_REFRACTORY_NANOS = 60s`, `crates/triage/src/cue/emitter.rs:28`; admission = new condition / tier escalation / refractory elapse, `emitter.rs:96-127`; cleared conditions evicted per cycle so a recurrence admits at once.)* Kind split for the remainder: tier3 666 (60s baseline cadence ≈ 60/h), tier2 1.
- Onset = the first evaluation tick after boot+3600s bootstrap elapse (first cue 05:43:36.480Z) — the `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` default.
- **The CueLatch is working AS DESIGNED** (it was the `2026-08-26-cadence-runaway-blocking-pool` bound). The defect is one layer up: a PERSISTING-UNCHANGED condition re-admits forever at refractory rate, and **each re-admission spawns a full generation** — the L4 subscriber (`pulse-app/src/inference_runtime.rs::spawn_l4_inference_subscriber`) generates for EVERY digest, with the degraded-mode backoff as its only skip.
- **Auto-resolve is defeated by CHURN, not by held-open ids:** 59 incidents over the day (36 resolved / ~5 active population permanent). *(Mechanism verified at P3: the dedupe path calls `registry.observe_reemission`, which sets `updated_at_unix_nano = now` (`crates/triage/src/incident/registry.rs:313`); `should_auto_resolve` keys on `updated_at` + the 120s window (`DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120`, `crates/triage/src/incident/persistence.rs:41`; 30s sweep in `pulse-app/src/incident_observer.rs`). The "gap" auto-resolve sneaks through is the MODEL's own verdict variance — a `Decision::Dismiss` or `severity: None` parse returns early WITHOUT bumping `updated_at`, the window elapses, then the next admitted cue recreates a fresh incident.)*
- Falsified in the same record (do not re-derive): dedupe-attach produces no generations (it is downstream of each one). The second falsified hypothesis moved — see the CARRY correction below.

## Outcomes (what this chunk delivers)
1. **Unchanged conditions re-analyze rarely** — a persisting-unchanged condition (the 11h shape: the same 5 services silent, nothing else moving) no longer spawns a full L4 generation at refractory rate. On the measured scenario, generation count over a comparable idle window drops from ~5/min to rare (the concrete bound is set at plan time per the chosen design direction).
2. **Hanging-incident churn stops defeating auto-resolve** — a quiet system's incident population converges (resolved incidents stay resolved; unchanged re-emissions stop resetting the benign window into permanent ~5-active churn at 59/day).
3. **The damper's decisions are observable** — a suppressed/reused-generation outcome is visible at the wire (per obs-plan discipline: an exact allowlist leaf, no bare prefix key), so "quiet because damped" is distinguishable from "quiet because dead" — this chunk must not create the next silence-unreadable diagnostic. *(Verified need at P3: the `cadence.tick` heartbeat is hardcoded `tier="tier3"` with ONE cumulative cycle counter and `last_executed_at_ms = 0`, so per-tier visibility does not exist today; the backoff-skip precedent `interpretation.inference.skipped` has its own exact leaf at `pulse-app/src/observability.rs:2122`.)*

## Design directions (operator-open — carried NOT decided; the plan presents them and the operator decides)
- **Content-hash damper** — unchanged digest → reuse the attached brief, no generation.
- **Growing refractory** for UNCHANGED conditions (60s → 5m → 30m).
- **Stopped-source-vs-dead-service semantics** — analyze a silence once, not 180 times.
- **No benign-window reset on unchanged re-emission** — a re-emission of an UNCHANGED condition must not reset the auto-resolve benign window.

These are directions, not a menu of exclusives — the plan may compose them; the operator rules on the composition at P5 (or via AskUserQuestion at P4 if the fork shapes the plan structurally).

## CARRY (folded from the working entry) — premise corrected at P3
**Working-entry hypothesis:** "reflection cadence emitted 0 digests in 11h — dead or LWW-starved; the 30-min background reflection never reached a single generation while tier1 ran 3,011."

**[premise-corrected: measured first-hand over the preserved 11h log, 2026-08-27 P3]** Reflection is NEITHER dead NOR LWW-starved, and it DID emit digests — the "0 digests" claim holds only for GENERATIONS:
- The coordinator's reflection arm fired **22/22** times (every 1800s; `DEFAULT_CADENCE_REFLECTION_SECONDS = 1800`, `crates/triage/src/cadence/config.rs`): 22 `cadence.trigger` mode=reflection → 22 `digest.runtime.cadence_tick` → 22 assembled → **22 persisted to the corpus** (`digest.runtime.persist` digest_kind=reflection; `digest_archive` rows 3702 = 3680 + 22 exactly) → 22 `interpretation.prompt.assemble` at `v1.1-reflection`.
- **All 22 reflection GENERATIONS then failed**: exactly 22 `interpretation.inference.error` records in the whole log, all `(error_category: inference_failed, recovery_action: skip_digest, model_tier: primary)`, each ~3.8s after its prompt assembled (a real subprocess run, not an instant rejection). Zero reflection rows in `interpretation.inference.request` (successes: tier1 3011 / tier3 666 / tier2 1 = 3678 = parse-ok count).
- The failing site is one of the two SILENT `InferenceError::InferenceFailed` returns in `pulse-app/src/llamacli_inference.rs` — `io_error` (`:816`) or `stdout_utf8_invalid` (`:845`) — because every other failure path (prompt-rejection, timeout, non-zero exit) emits its OWN categorized warn first and none appears; the extractor is exonerated (it returns `JsonParseFailed`, a different label). Both silent sites drop `InferenceFailed.reason` on the floor — an observability gap in its own right. The reflection-specific input is `build_reflection_tier_prompt` (still prompt v1.1 while cadence runs v2.2).
- The LWW queue is push-only at HEAD (nothing drains it since `drain_all` was removed as dead); each reflection digest LWW-replaced its predecessor in the reflection slot (21 replaces / 22 pushes) — bookkeeping, not the block.

**This chunk owes the CARRY a disposition on the corrected premise:** fix the reflection generation failure in-chunk (root-cause the silent site with one live repro, and give the two silent sites their `reason` on the wire), or give the corrected finding a named owner — never leave it implied. Note the interaction with outcome 1: an UNFIXED reflection cannot serve as the "rare deep analysis" a damped tier1 would lean on.

## PREREQ (folded — wrap-time obligation)
- **`cargo audit` interval point 49: the probe RUNS at this chunk's wrap.** Standing deferral since `2026-08-15-corpus-key-persistence` (ratified pin #15, every-3rd-wrap interval; re-pinned here from `2026-08-26-interpretation-brief-completeness`, origin preserved; basis: upstream RustSec DB duplicate-advisory-id parse error; overlap signal: `cargo deny check advisories` designed-red at the eight owned IDs 0189/0190/0194/0195/0204/0222/0253/0258, re-verified at the session-48 wrap with `bans licenses sources` exit 0). Session 48 was an interval SKIP; this wrap's probe is a full-form run: true exit read directly, basis reproduced, overlap re-enumerated as DISTINCT ids (never error blocks).

## Boundaries (what this chunk does NOT do)
- Does not absorb the **Diagnostics un-muting + harness-truth sweep** (its own route entry owns the muted-leaf backlog — including `triage.incident.auto_resolve.tick`, whose three fields are live-confirmed redacted in the 11h log; if the damper adds a new observable it carries its OWN exact leaf and does not touch the owned backlog).
- Does not fix the **ingest consumer initiating freeze** (own route entry; detector armed by design).
- Does not decide the **Halo canvas disposition**, the advisory backlog, or any other tail entry.
- Does not change the L4 inference runtime's invocation discipline, the prompt contract for CADENCE tiers, or the incident/report wire shapes — the damper sits at verified seams UPSTREAM of generation: the L4 subscriber boundary (`inference_runtime.rs`), the CueLatch (`emitter.rs`), and the dedupe/benign-window seam (`observe_reemission` / `create_incident_from_l4_output`); reuse-of-attached-brief (if chosen) rides the existing `resolution_summary_text` attach mechanism from `2026-08-26-interpretation-brief-completeness`. *(The CARRY's reflection-generation fix, if taken in-chunk, touches `llamacli_inference.rs` error surfacing and/or the reflection prompt builder — a bounded exception to "runtime untouched", stated here rather than implied.)*
- No new workspace deps; no corpus DDL / `SCHEMA_VERSION` change. *(Verified consistent at P3: no finding requires either.)*

## Surfaces touched (verified at P3)
- `crates/triage/src/cue/emitter.rs` — CueLatch (`LatchEntry`, `admit_cycle`) if the growing-refractory direction lands; `triage.cue.tick` leaf completion if new counters ride it.
- `crates/triage/src/incident/registry.rs` (+ `state_machine.rs` semantics) — `observe_reemission` if the no-reset-on-unchanged direction lands.
- `pulse-app/src/inference_runtime.rs` — the subscriber seam (`spawn_l4_inference_subscriber` / `handle_digest_outcome`) for a content-gate; `create_incident_from_l4_output` dedupe path.
- `pulse-app/src/observability.rs` — the damper observable's exact leaf (precedent: `interpretation.inference.skipped` at `:2122`).
- If the CARRY is fixed in-chunk: `pulse-app/src/llamacli_inference.rs` (silent-site reason surfacing) + `crates/interpretation/src/prompt.rs` (reflection builder) as bounded.
- Tests: in-crate `crates/triage` unit pins + `pulse-app/tests/` allowlist/integration pins; direct-binary idle smoke legs per test-plan §3.
