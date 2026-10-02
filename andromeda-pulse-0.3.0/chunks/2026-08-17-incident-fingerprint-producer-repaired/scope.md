# Scope — Incident-fingerprint producer repaired

**Marker:** `2026-08-17-incident-fingerprint-producer-repaired` · **Version:** andromeda-pulse-0.3.0 ·
**Epoch:** Epoch 4 — Polish & ship: verification · **Promoted:** 2026-08-17

## The chunk in one line

`Incident.fingerprint` carries the anonymized grouping **hash** its contract and its consumers already
expect, instead of the model-authored string its producer writes today.

## What it builds

- **The producer stops writing a model-authored string into a hash field.** `Incident.fingerprint` is
  produced at `pulse-app/src/inference_runtime.rs:700` as `parsed.fingerprint.clone()` — the
  `L4Output.fingerprint` the model emits (a free-form 1–256-char `String`; a hardcoded constant
  `"deterministic-l4-fixture"` under `ANDROMEDA_PULSE_L4_DETERMINISTIC`). Its own contract at
  `crates/triage/src/contract.rs:340-342` documents it as *"Anonymized fingerprint hash for cross-incident
  grouping per capability spec P-047"*, and `crates/triage/src/digest/assembler.rs` consumes it as
  32-char lowercase hex fed by `hex_lower(Q3 blake3 bytes)`. The chunk makes the produced value match the
  contracted and consumed kind.
- **A real fault fingerprint reaches the incident site.** The value the field should carry already exists
  upstream — the `ExceptionFingerprint` (blake3 of `exception.type` + the normalized 3-frame stack,
  `crates/buffer/src/fingerprint.rs`), whose normalization semantics were settled TOKEN-LEADING at
  `2026-08-16-fault-identity-semantics-decided`. Today it is discarded at the cue boundary:
  `synthesize_cue` fixes `kind`/`scope` and sets `scope_id = service`, carrying no fingerprint, so
  namespace A never reaches the incident path. Closing that gap is this chunk's substance.
- **The starved retrieval arm gets fed.** `crates/triage/src/digest/retrieval.rs::select_corpus_matches`
  compares `Incident.fingerprint` against the assembler's `hex_lower(Q3 blake3)` set. It is
  correct-per-contract and STARVED — never dead. Its removal was written and FULLY REVERTED at the prior
  chunk when `assembler.rs` tests rejected it, leaving an in-code guard: *"The mismatch is the producer's;
  do not simplify this arm away."* This chunk repairs the producer so that arm can fire in production
  rather than only in its own tests.

## Likely shape (per the working entry)

The `AttentionCue` → `DigestCueRef` → producer threading that the fault-identity chunk deliberately
rejected as out-of-scope, and named as *the documented path — do not partially implement it*. Expect a
`triage::contract` type change plus the three pinned producer tests.

## Boundaries

- **Does NOT reopen Layer-2 incident identity.** Coalesce-per-cue-identity on `(kind, scope, scope_id)`
  stays DECIDED (`architecture.md` §Established Decisions [Fault Identity]). This chunk only records the
  dependency: per-fingerprint incident dedupe becomes *possible* once the field carries real hashes, and
  therefore becomes REVISITABLE — by a later entry, not here.
  `[premise-corrected: the pin's own helper builds a DigestCueRef literal at pulse-app/tests/unit_incident_producer.rs:111, and Rust requires every field named]`
  The existing pin `distinct_fingerprint_same_service_still_coalesces_to_one_incident` must still pass
  **semantically** unchanged — but its file DOES change mechanically. "Leave the file untouched" is the
  wrong reading; "do not weaken the assertion" is the right one.
- **Does NOT reopen Layer-1 normalization.** TOKEN-LEADING is settled and its blast radius measured; this
  chunk consumes that value, it does not redefine it.
- **Deliberately separate from the P-075 det-L4 `evidence_refs` CARRY.** Same L4-output→persisted-field
  data flow, different defect: that CARRY is a stub *not populating* a field; this is a producer populating
  one with the wrong KIND of value. Kept apart on purpose — do not fold them.
- **VERIFIED — no corpus DDL / `SCHEMA_VERSION` change.** `incidents.payload` is a bincode BLOB and
  `fingerprint` is a `String` on both sides. The `DigestCueRef` field addition rides the chunk-#92
  `#[serde(default)]` precedent (`crates/triage/src/contract.rs:458,465`), which kept archived digest BLOBs
  deserializable with no version bump.
- **VERIFIED — no new TauRPC procedure, MCP tool, port, or env var**, so no capability-JSON /
  `EXPECTED_PROCEDURES` / bindings quadruple-binding work. `fingerprint` appears in
  `pulse-app/ui/src/bindings/index.ts` only inside comments (`:129`, `:276`) — the webview `IncidentRecord`
  view drops it, and `interpretation::markdown::PreviouslySeenMatch` carries no fingerprint either, so no
  `pulse-app/ui/**` path is touched and no a11y fixture moves.
- **NEW BOUNDARY (research) — no `triage → buffer` crate edge is created.** `triage` deliberately carries
  none (`crates/triage/src/pattern/storm.rs:13-18`) and needs none: `ExceptionFingerprint` is a `[u8; 16]`
  alias and the bytes already arrive opaquely through the `FingerprintObserver` seam.

## Known consequence this chunk should resolve or restate

`select_previously_seen` compares `Incident.fingerprint` to `Incident.fingerprint` — same namespace, so it
is genuinely live, but the deterministic runner emits a **constant**, so under
`ANDROMEDA_PULSE_L4_DETERMINISTIC=true` every incident matches every other as "previously seen". The prior
chunk documented this as a property of the fixture needing exactly the threading this chunk performs.
**VERIFIED — repairing the producer closes it.** Storm-derived incidents gain per-fault hashes so matching
becomes fault-scoped; reflection/baseline-derived incidents carry no cue fingerprint, so the field is empty
and both selectors' `!fingerprint.is_empty()` guards (`crates/triage/src/digest/retrieval.rs:96,120`) drop
them to scope-only matching. The acceptance names this outcome.

## Why it precedes P-075 / P-076

Both assert on incident formation and read-back, so their acceptances encode whatever the fingerprint
field means. Authoring them after this lands writes them once against the settled field shape — the same
reason fault-identity preceded them.

## Folded annotations

- **PREREQ — `cargo audit` probe FIRES at this chunk's wrap.** Standing deferral since
  `2026-08-15-corpus-key-persistence`, ratified at the 2026-08-16 0-pending adaptation wrap (pin #3), with
  a ratified RE-RUN INTERVAL of every 3rd wrap. Session 25 ran it; sessions 26 and 27 recorded interval
  skips with the basis re-verified first-hand each time (`error loading advisory database: parse error:
  duplicate advisory ID: RUSTSEC-2026-0244` — upstream, reproduced) and the named overlap
  `cargo deny check advisories` re-observed (exactly the 7 owned upgradeable IDs). **Session 28 is the
  interval point: run `cargo audit` for real and record its result — a third silent skip breaches the
  ratification.** Full rationale: the `2026-08-15-corpus-key-persistence` report.

## The trap research found (carry into implementation)

`triage` already has a `fingerprint_to_hex_prefix` (`crates/triage/src/pattern/storm.rs:450`) — correctly
named, already in the file, and **8 characters wide** (first 4 bytes only; it exists for bounded-cardinality
tracing). The value the retrieval arm matches against is `hex_lower` of the **full 16 bytes** — 32 characters
(`crates/triage/src/digest/assembler.rs:283-288,686`). Reusing the adjacent helper would produce a value that
can never match, re-starving the arm while a shape-only assertion ("is lowercase hex") still passes. The
acceptance must pin the WIDTH, not just the shape.

## Verification-matrix relation

**VERIFIED** — no P-0NN capability becomes fully verifiable here. P-075 and P-076 are the caps asserting on
incident formation/read-back and both land after this; P-077 is the injector entry. All three stay
`chunk:null`, with this chunk cited as a partial advance in `plan.md` provenance.
