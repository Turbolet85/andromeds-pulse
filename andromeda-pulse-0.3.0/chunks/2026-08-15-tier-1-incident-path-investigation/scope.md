# Scope — 2026-08-15-tier-1-incident-path-investigation

**Version:** andromeda-pulse-0.3.0 · **Epoch 4 — Polish & ship: verification**
**Taken up:** 2026-08-15, out of working-route order by explicit operator direction (it gates
preflight-green and the regression trail is freshest now). The preceding markerless entry
(*Baseline-family reachability*) stays markerless and mutable.

## The outcome

**A detected storm produces an incident again.** Today a storm IS detected and zero incident rows
exist; the whole downstream surface (incidents panel, per-service severity join, MCP read-back,
the P-075 canary) is starved by that one break.

## What is measured, and what is not

**MEASURED (from `2026-08-15-corpus-key-persistence/evidence/arm-zero-classification.md`, and
reproduced in both smoke boots of that chunk):**

| Stage | Observation | How it was read |
|---|---|---|
| Feed | `span_events_seen 6 / fingerprints_computed 6 / observer_invocations 6` | `buffer.tick` counters, 12 ticks |
| Storm detector | `storms_detected_total` 0 → **1** | `triage.pattern.storm.tick` |
| Cue emission | **0** `triage.cue.emit` in the whole run (`triage.cue.evaluate` ran 192×, `triage.cue.tick` 192×) | log grep |
| Incident rows | **0** in `incidents`, 0 in `incident_events` | keyless read-only `sqlite3` of `corpus.db` (plaintext columns) |
| MCP read-back | `tool dispatch ok`, `{"items": [], "total": 0}` | live JSON-RPC `query_incident_list` |

So the zero originates **at or before cue emission**, and the read-back's zero is truthful rather
than a read failure (the key-persistence fix removed the decryption barrier that previously masked
this).

**NOT MEASURED — the two open premises this chunk must resolve rather than inherit:**

1. **"Regressed within the last two chunks' window" is plausible, not proven.** The capture-chunk
   era demonstrably created incidents (10 in its run; 2 active read independently at the
   workspace-key before-evidence), but that run also differed environmentally. The window
   hypothesis is a lead, not a finding.
2. **A missing `triage.cue.emit` does not by itself prove no cue was carried.** The 2026-06-28
   Tier-1 work introduced a separate internal storm→digest carrier (`DigestTriggerBroadcast`); the
   cue may travel off the PII-free topic. Absence of one log target is not absence of the signal.

   **[closed 2026-08-15 P3 — the caution was right, for a nearer reason than expected.**
   `triage.cue.emit` has exactly ONE writer (`crates/triage/src/cue/emitter.rs:195`), fed only by
   the BaselineState-derived emit cycle. The storm detector is a SEPARATE producer that broadcasts
   its cue on the shared `AttentionCueBroadcast` and logs `triage.pattern.storm.detected` /
   `.emit` instead. So `triage.cue.emit == 0` is the EXPECTED reading for a storm-only run and
   carries no information about the storm path. (`DigestTriggerBroadcast` sits one hop further
   downstream than the note assumed — it carries the coordinator's trigger, not the detector's
   cue.) **A competing explanation for premise 1 also emerged and is now what §1 must
   discriminate:** against production thresholds (suggested 5 / autonomous 10,
   `crates/triage/src/pattern/storm.rs:73-78`) the arm's 6 fingerprints with
   `storms_detected_total` 0→1 fits the *Suggested* branch exactly — and the coordinator's Tier-1
   arm accepts **only** `PriorityTier::Autonomous` (`crates/triage/src/cadence/coordinator.rs:390`),
   so zero incidents would follow BY DESIGN from a 6-event run. See research.md §Findings.]**

## Work in scope

### 1. Premise-check BEFORE bisect (operator directive 1 — runs first)

Re-run the capture procedure on HEAD — `inject_demo`, **fresh data dir**, deterministic L4
(`ANDROMEDA_PULSE_L4_DETERMINISTIC=true`) — reading incident count by keyless `sqlite3`.

- **Zero incidents** → the in-window regression is confirmed; proceed to bisect the three points
  `b08e10a` (fingerprint-feed-capture-repair) → `2961f4e` (workspace-key-alignment) → `3ac3d9d`
  (corpus-key-persistence).
- **Ten incidents** → the break is environmental/config, the window hypothesis **dies here** before
  it can misdirect the chunk, and the investigation re-aims at the environmental delta between the
  capture-era run and the arm/smoke runs.

This discriminator is cheap and decisive, and it is the chunk's first action either way.

### 2. Locate the break in the storm→cue→incident seam

- **First check (named):** the `DigestTriggerBroadcast` carrier — establish whether a cue/trigger is
  carried on it while `triage.cue.emit` stays silent, or whether nothing is carried at all. This
  discriminates "signal lost" from "signal carried but unobserved".
- **Ranked suspect — HYPOTHESIS, to be tested, never assumed** (operator directive 3; two verified
  facts, the inference is research's to test):
  - FACT: the workspace-key-alignment chunk modified `pulse-app/src/digest_runtime.rs`.
  - FACT: `resolve_workspace_for_incidents`' own doc-comment declares that the FILTER key
    (services/incidents) and the stamped workspace are byte-equal — the invariant the per-service
    severity join depends on (P-079).
  - HYPOTHESIS: if the published-key change moved what gets *stamped* relative to what the join
    *filters*, incident creation dies exactly downstream of cues.
  - Rank it in the bisect; do not let it short-circuit the bisect.

### 3. Fix the located break

The route entry's headline is a restored behavior, not a diagnosis — the fix lands in this chunk
once the break is located. Nature and blast radius are unknown until §1–§2 complete, so the
implementation shape is deliberately not pre-committed here.

### 4. Obs repair — the muted-diagnostic backlog this entry OWNS

The default-deny allowlist redacts exactly the fields that would have made the diagnosis immediate,
which is what forced the out-of-band `sqlite3` read during the arm-zero classification:

- `incidents.list_active.request` → `item_count`
- `triage.incident.persist` → `incident_count`, `persist_kind`
- `triage.incident.corpus_restore` → `kind`, `restored_incident_count`

Same muted-diagnostic class the corpus-key chunk repaired for `corpus.open.error`, on
`triage.*` / `incidents.*` targets. These are counts and bounded enum labels — no telemetry
content — so they are addable under the project's NEVER-log invariants. **[verified 2026-08-15 P3:
all three targets are ABSENT from `pulse-app/src/observability.rs`; none landed with the corpus-key
chunk. The `incidents` family already carries `incidents.mark_all_read.request` +
`incidents.get_report.request` as sibling EXACT leaves and no bare prefix key, so the additions
follow that established shape. All four field additions are this chunk's work.]**

### 5. Evidence discipline (operator directive 4)

- Keyless `sqlite3` reads of `corpus.db` are the **ground truth** for the incidents table — never a
  log line, never an IPC response, when the question is "does a row exist".
- **Counters over gauges** — a monotonic total that can be differenced across a run beats a
  point-in-time reading.
- **Every claim names its query** (the exact command / grep / SQL that produced it).
- All evidence lands in **this chunk's** folder
  (`andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/`).
  Conductor, if used, is the INSTRUMENT: run its preflight from its own repo root, launch
  `pulse-app` outside that repo, never write into Conductor's tree.

## Boundaries — out of scope

- **P-075 (Conductor e2e verification closure)** — this chunk *unblocks* it (the canary needs an
  incident to FORM) but does not claim it.
- **Baseline-family reachability** — the 3600s baseline gate binds the baseline-derived families
  only; the storm path never reads `BaselineState` (measured at
  `2026-08-14-fingerprint-feed-capture-repair`). Separate markerless entry, untouched.
- **The advisory backlog** and the `LwwQueue` dead-path removal — separate route entries.
- **Conductor's own tree** — cross-repo, never written here. Its `architecture.md:174` still
  encodes a now-false workspace-column belief; recorded, not actioned.
- **No new capability surface.** This is a regression repair plus an observability repair; it adds
  no TauRPC procedure, no corpus table, and no env var. **[verified 2026-08-15 P3: every candidate
  fix site is a decision point inside an existing in-process signature (threshold arithmetic, a
  match arm, an allowlist leaf) — no new TauRPC procedure, broadcast topic, env var, corpus table
  or capability is implied. The `DigestTriggerBroadcast` carrier is confirmed internal, correctly
  outside arch §Occupied Resources per the 2026-06-28 PII-free-topic rule.]**

## Surfaces touched [closed 2026-08-15 P3 — see research.md]

- `pulse-app/src/observability.rs` — the three EXACT `triage.*` / `incidents.*` leaves of §4
  (verified absent on HEAD) + their resolution unit test. **Certain.**
- **Provisional, decided by the §1 premise-check + bisect** (the located break selects among them):
  `crates/triage/src/pattern/storm.rs` (tier selection at l.246-288; the missing
  `CadenceTriggerChannel` forward at l.352) · `crates/triage/src/cadence/coordinator.rs` (the
  Tier-1 `PriorityTier::Autonomous`-only gate at l.390) · `pulse-app/src/inference_runtime.rs`
  (the hop-7 creation gates at l.636-654).
- **[premise-corrected: `pulse-app/src/digest_runtime.rs` is NOT expected to change.** The
  code-graph shows `resolve_workspace_for_incidents` has exactly ONE production caller
  (`main.rs:735`) which destructures both halves of a single return value, so filter-key and
  stamped-workspace cannot diverge there by construction; and a workspace mismatch at the hop-7
  dedup lookup would yield DUPLICATE incidents, not zero. The P-074 cue threading is present and
  correct at l.148-156. The ranked suspect is ranked DOWN — still tested at the bisect per operator
  directive 3, not assumed.**]

## Acceptance shape (concretized at P4/P5)

1. The premise-check ran and its outcome is recorded with the query that produced it — either
   branch is a result.
2. The break is located and named with evidence, not inferred.
3. A storm produces an incident: a reproducible capture on a fresh data dir shows `incidents` rows
   > 0 by keyless `sqlite3`, where the same procedure showed 0 before the fix. **[concretized
   2026-08-15 P4/P5 — made CONDITIONAL on a break existing.** Research showed the arm's zero has a
   competing by-design explanation (6 events never reach the autonomous threshold of 10), so if
   Steps 1-3 locate no break there is no "before the fix" to improve on; the criterion then becomes
   the demonstrated-healthy proof plus the arithmetic accounting for every measured zero. An
   unconditional form would be unachievable in that branch and would pressure the chunk toward
   manufacturing a fix.]**
4. The muted diagnostics of §4 are emitted (or shown already-present), so the next such
   investigation reads them from the log rather than out-of-band.
