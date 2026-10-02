# Scope — Fault-identity semantics decided

**Marker:** `2026-08-16-fault-identity-semantics-decided`
**Version:** andromeda-pulse-0.3.0 · **Epoch 4** — Polish & ship: verification
**Promoted:** 2026-08-16 · operator-directed 2026-08-16 at the 0-pending adaptation wrap

---

## What this chunk is

A **DECIDE chunk**. Its deliverable is a *decided semantic* plus the code and/or documentation
changes that make one statement true across the codebase — not a new feature and not a bug fix
chosen in advance. The working-route entry deliberately states the question and does **not**
answer it; answering it is this chunk's work.

One question, asked at two layers: **what makes two faults the same fault?**

> **[premise-corrected: P3 research]** The entry's framing — *"fingerprint normalization and incident
> dedupe agree on when two faults are one"* — presupposes the two layers range over ONE identity.
> **They do not.** There are two unrelated fingerprint namespaces: **A** = `ExceptionFingerprint`
> (`buffer/fingerprint.rs:37`, blake3 over the normalized stack, what Layer 1 governs) and **B** =
> `L4Output.fingerprint` (`interpretation/schema.rs:170`, an LLM-authored free string copied onto
> `Incident.fingerprint` at `inference_runtime.rs:687`). A never reaches the incident path — it is
> dropped at `synthesize_cue` (`storm.rs:309-319`), and `AttentionCue` has no fingerprint field. B
> never reaches the storm path. `prompt.rs` mentions fingerprint **zero** times, so nothing bridges
> them. The two layers are therefore not two views of one semantic; deciding them is still one
> chunk's work, but "make them agree" is not a one-predicate change. See `research.md` §Headline
> finding.

Both layers currently answer it, independently and without a recorded decision. Neither answer was
chosen; each is what the code happens to do. The chunk's job is to choose, then make impl and docs
agree with the choice — **never to infer the intent from current behaviour.**

## Layer 1 — Fingerprint identity (normalization semantics)

**The surface:** `crates/buffer/src/fingerprint.rs` — `normalize_stacktrace` / `normalize_frame`
feed the blake3 preimage that becomes `ExceptionFingerprint`, the L1c identity fed to the retry-storm
detector and written to the `span_events.fingerprint` column.

**The gap (stated by the working entry, confirmed by read at promotion):**
`is_absolute_path_start` (`fingerprint.rs:186`) returns true for **any** `/` followed by a path
character — not only a leading one. Because `is_path_char` (`fingerprint.rs:216-218`) itself
includes `/` and `\`, `skip_absolute_path` then consumes the entire remaining path token in one
sweep. Net effect: **only the leading path segment survives normalization.**

Worked example (verified against the implementation at promotion):
`src/a.rs` → `src` and `src/b/c.rs` → `src` — identical normalized frames, therefore identical
fingerprints, therefore one storm identity for two distinct faults.

**The contradiction:** the module doc (`fingerprint.rs:28`) says normalization "strips absolute
paths", and the `normalize_frame` doc (`fingerprint.rs:134-137`) says "absolute file paths (Unix
`/...` or Windows `C:\...`)". Both describe an absolute-only semantic. The implementation is greedy.
Impl and docs are in direct conflict; distinct faults can over-coalesce into a single storm.

**The decision:** choose the intended semantic on **product merits**, then align the losing side.
- **Greedy (status quo, re-documented)** — coarser identity; `src/a.rs` and `src/b/c.rs` are one
  fault; fewer distinct fingerprints; more aggressive storm coalescing.
- **Token-leading (absolute-only, impl fixed)** — preserves relative path structure; finer
  resolution; more distinct fingerprints; storms separate along module boundaries.

**Cross-project switching cost (operator-supplied, pre-verified 2026-08-16 — belongs in the
decision record, not in the merits):** the external Conductor verification harness has *just*
aligned to the current greedy behaviour — its P-017 spec clause (leading-segment precision), its
`FingerprintVariant` tests (below-leading-segment changes insignificant), and
`scenarios/fingerprint-storm.toml` all encode the measured any-slash semantics. Fixing the impl to
token-leading means Conductor re-aligns a **third** time (tests + toml + spec clause — bounded,
known cost). Re-documenting the leading-segment semantic as intended means Conductor stands as-is.
Decide on product merits; record the cost.

## Layer 2 — Incident identity (dedupe semantics)

**The surface:** `pulse-app/src/inference_runtime.rs::create_incident_from_l4_output` (`:626`).

**The gap (stated by the working entry, mechanism sharpened by read at promotion):** the working
entry says dedupe "keys on an OPEN incident, NOT on the fingerprint". The predicate at
`inference_runtime.rs:664` is more specific than that phrasing:

```rust
.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)
```

It searches `registry.list_active(&digest.workspace)` and matches on the **cue identity tuple**
`(kind, scope, scope_id)` — derived at `:648-654` from the triggering attention cue. `Incident.fingerprint`
IS populated (`:687`, from `parsed.fingerprint`) but is **not a component of the key**. A match takes the
re-emission branch (`observe_reemission` → `emit_incident_outcome(created:false, deduped:true)`, `:661-682`);
only a miss creates.

Consequence: for storm-derived incidents the tuple is effectively per-service, so **two distinct
fingerprints from the same service collapse into one incident by construction** — which is the
measured canary behaviour the entry records (a deliberately unique-typed canary deduped anyway,
`created:false` / `deduped:true`), with the auto-resolve window the only same-data-dir cure.

- **VERIFIED (P3)** The canary deduped *because* its storm carried the same `(kind, scope, scope_id)`
  tuple as the open incident. Confirmed **by construction**: `synthesize_cue` (`storm.rs:309-319`)
  hardcodes `kind: RetryStorm, scope: Service` and sets `scope_id = service`, so *every* storm cue
  from one service is tuple-identical no matter which fingerprint stormed — even though the detector
  itself keys its state per-fingerprint (`DashMap<fingerprint, StormState>`). Two distinct
  fingerprints storming one service emit **two cues that are indistinguishable at the predicate**.
- **[premise-corrected: `persistence.rs:41` = 120s, pinned to spec P-022 by the test at `:377`]**
  The auto-resolve window is **120 seconds (2 minutes)**, not the ~5 minutes the working entry
  states.

**The decision:** should a distinct-fingerprint storm open a **second concurrent incident** while
one is open? Equivalently: does `fingerprint` join the dedupe key?
- **Coalesce-per-service (status quo)** — one incident per service at a time; quieter incident list;
  a second, unrelated fault during an open incident is invisible until auto-resolve.
- **Incident-per-identity** — a distinct fingerprint opens a second concurrent incident; faithful to
  fault identity; more incidents, and the downstream surfaces must cope.

**Cross-project constraint (operator-supplied, pre-verified):** either direction is **safe** for
Conductor — fresh-dir-per-leg holds regardless, and per-fingerprint dedupe would only make
back-to-back legs easier. This layer stands **purely on product merits**.

## Open research questions — ANSWERED at P3 (see `research.md`)

1. **What would the L4 digest do with a second concurrent incident?** → **Nothing changes.** The
   assembler reads `list_active` only as a boolean (`has_critical_active = …any(severity ≥
   Suggested)`, `assembler.rs:221-224`) to pick the LWW mode. `any()` over 1 vs 2 is identical.
2. **What would the UI incident list do?** → **Already N-safe.** The per-service constellation join
   reduces by `max_by_key(tier_rank)` over all matching active incidents
   (`services_router.rs:97-104`) — design's max-not-count invariant holds by construction. The
   Findings list renders per-incident rows sorted by tier, so a second row is ordinary content.
3. **Layer-1 blast radius** → **Contained.** The normalization chain has zero callers outside
   `fingerprint.rs`; the fingerprint VALUE reaches only `appender.rs`, `consumer.rs`,
   `storm_observer.rs`, `main.rs:648` and 2 e2e tests. **No viz / MCP / corpus / UI consumer**, and
   `span_events.fingerprint` is never SELECTed. A Layer-1 change moves storm grouping and nothing
   else.

_(Original framing, retained for the record:)_

The operator directs that the P4 decision asks reach them **recommended-first and grounded in
research**, not as bare design polls. Specifically, before recommending on Layer 2:

1. **What would the L4 digest do with a second concurrent incident?** Does the digest assembler /
   cadence path assume at most one open incident per service, or is it indifferent? The answer may
   already constrain the choice.
2. **What would the UI incident list do?** The Findings window, the unread badge, the per-service
   constellation severity join (which keys on `scope_id`) — does a second concurrent incident for one
   service render sensibly, double-count, or collide?
3. **Layer 1 blast radius** — which consumers besides the storm detector read the fingerprint
   (`span_events.fingerprint` column readers, MCP surfaces, evidence refs), and does a resolution
   change any persisted-data interpretation?

If a downstream surface already forecloses one option, that is a **decisive material lean** and
belongs in the plan as a stated lean, not an operator question.

## Boundaries

**In scope**
- Deciding both layers, recording each decision with its rationale and its switching cost.
- Making impl and docs agree with each decision — on Layer 1 that is *either* a `fingerprint.rs`
  normalization change *or* a doc correction (module doc + `normalize_frame` doc + the inline
  comment), never both and never neither.
- Test coverage that pins the decided semantic so it cannot silently drift back.
- **[intent-incomplete, amended at P5 val-1 — operator-chosen at P4; then PREMISE-CORRECTED at the
  wrap, see below]** Narrowing the dead `fingerprint_match` arm in `select_corpus_matches`
  (`crates/triage/src/digest/retrieval.rs:86-94`). Research found it compares hex blake3 digests
  against LLM-authored prose, so it cannot fire in production and only `scope_match` ever does; its
  unit tests pass only because they supply symbolic strings on both sides. The scope as promoted did
  not know this existed. It is a fault-identity mechanism that does not work, so it belongs here
  rather than to the Diagnostics sweep entry — the operator chose in-chunk disposition at the P4
  review.
  - **[premise-corrected at wrap P2 val-3: the arm is NOT dead — it is correct-per-contract and
    STARVED, and the producer is the defect]** The "narrow" half was attempted and **fully
    reverted**. `Incident.fingerprint` is documented at `crates/triage/src/contract.rs:340` as an
    *"Anonymized fingerprint hash for cross-incident grouping"* and consumed as lowercase hex by the
    assembler (fixtures `"aa".repeat(16)` / `"ffff0000…"` / `"bb".repeat(16)`), so both sides of the
    comparison are MEANT to be the same hex namespace; it is the producer
    (`pulse-app/src/inference_runtime.rs:700`, writing the model-authored `L4Output.fingerprint`)
    that breaks it. Two `assembler.rs` tests caught the removal. What SHIPPED for this bullet is the
    "document" half only: the arm is intact, byte-identical to HEAD apart from a doc block carrying
    the guard *"The mismatch is the producer's; do not simplify this arm away."* The producer repair
    is owned by a dedicated route entry minted at this wrap's P5 — the correct fix is the
    `AttentionCue` threading this chunk deliberately rejected.

**Out of scope**
- P-075 / P-076 acceptance authoring. This chunk exists so those are written **once** against
  decided semantics; it does not write them.
- Any change to Conductor. `conductor/conductor-0.2.0/chunks/2026-08-15-*/` + `2026-08-16-*/` are
  **evidence, READ-ONLY from here — cite, never copy.** Re-alignment there is a consequence recorded
  in the decision, executed in that repo.
- The dead Tier-1 `LwwQueue` drain path, the `agent-run.sh status` truthfulness item, and the other
  Epoch-4 sweep entries — each has its own owner on the route.

## Why this precedes P-075 / P-076

Both later entries assert on **incident formation and read-back**, so their acceptances encode
whatever fault identity means. Settling it first writes those acceptances once against decided
semantics, instead of freezing today's measured-but-undecided behaviour into two suites.

## Folded annotations

**PREREQ (from the working entry) — `cargo audit` standing deferral, pin #3.**
Ratified at the 2026-08-16 0-pending adaptation wrap; origin `2026-08-15-corpus-key-persistence`;
re-pinned here from `2026-08-16-baseline-family-reachability`, which absorbed it and recorded
`probe skipped per ratified interval (next: 28)`.
- **Basis:** the upstream RustSec DB cannot LOAD — `parse error: duplicate advisory ID:
  RUSTSEC-2026-0244` — measured first-hand at session 25 and byte-identical on a second project the
  same day, so ONE upstream event, not a per-project fault.
- **Named overlap:** `cargo deny check advisories`, four classes, runs every chunk.
- **Ratified interval:** probe every 3rd wrap — ran at session 25, next at **session 28**. This
  chunk's wrap is session 27, so the expected disposition is `probe skipped per ratified interval
  (next: 28)` **with basis + overlap re-verified and recorded in the report — never a silent skip.**
- **Ends** the first time `cargo audit` loads.
- Full rationale: the `2026-08-15-corpus-key-persistence` report.

## Evidence

- `conductor/conductor-0.2.0/chunks/2026-08-15-*/` + `2026-08-16-*/` — all measured against this
  repo's HEAD `d090314`. **READ-ONLY: cite, never copy.**
- Confirmed by direct read at promotion: `crates/buffer/src/fingerprint.rs:186,216-218` (Layer 1
  greedy consumption) · `pulse-app/src/inference_runtime.rs:626,648-654,661-682,687` (Layer 2 dedupe
  tuple, and `fingerprint` populated-but-unkeyed).
