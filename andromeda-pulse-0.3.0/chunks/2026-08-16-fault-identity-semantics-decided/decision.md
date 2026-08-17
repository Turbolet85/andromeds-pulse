# Decision record — fault-identity semantics

**Chunk:** `2026-08-16-fault-identity-semantics-decided` · **Decided:** 2026-08-16 · **Decider:** operator, at the /andromeda-phase P4 review

The question: **what makes two faults the same fault?** Asked at two layers, decided at both, on product
merits, with switching costs recorded rather than hidden.

---

## The finding that reframed the question

The route entry asked that "fingerprint normalization and incident dedupe agree on when two faults are one."
They could not disagree, because they never met. There are **two unrelated fingerprint namespaces**:

| | **A — `ExceptionFingerprint`** | **B — `L4Output.fingerprint`** |
|---|---|---|
| Defined | `crates/buffer/src/fingerprint.rs` | `crates/interpretation/src/schema.rs` |
| Value | 16-byte blake3 of `(exception.type + normalized 3-frame stack)` | model-authored `String`, 1–256 chars |
| Reaches | `span_events.fingerprint`, the storm detector | `Incident.fingerprint` |

Namespace A never reaches the incident path: `synthesize_cue` fixes `kind`/`scope` and sets
`scope_id = service`, carrying no fingerprint, so the per-fingerprint distinction the detector tracks
internally is discarded at the cue boundary. Namespace B never reaches the storm path. `prompt.rs` mentions
fingerprint zero times, so nothing bridges them. Layer 1 governs A; Layer 2 could only ever have keyed on B.

---

## Layer 1 — fingerprint normalization: **TOKEN-LEADING**

**Decision.** Only a path token that *starts* a token (position 0, or preceded by a non-path byte) counts as
absolute and is stripped. Relative path structure is identity-significant and is preserved in full.

**Why.** Normalization exists for host-stability. Stripping *absolute* paths achieves that; stripping
*relative* structure does not serve it and destroys identity resolution. Both doc sites already claimed
absolute-only, and the pre-existing test `compute_strips_paths_and_line_numbers` exercises only genuinely
absolute paths (`(/abs/path/…` and `(C:\abs\path\…`) — so **docs and tests both encoded absolute-only, and
only the implementation was greedy.** This is the implementation moving to meet a stated intent, not a
reinterpretation of intent. For a diagnostic tool, over-coalescing is the worse failure: it hides distinct
faults, which is exactly what the route entry named.

**What changed.** `is_absolute_path_start` gained a token-boundary precondition, applied to both the Unix and
the Windows drive-letter arm. The guard also closes a latent false positive on the Windows arm (a mid-token
`x:/` shape can no longer be read as a drive root).

**Blast radius (measured, not assumed).** The normalization chain has zero callers outside `fingerprint.rs`.
The fingerprint value reaches only `appender.rs`, `consumer.rs`, `storm_observer.rs`, `main.rs` and two e2e
tests — never viz, MCP, corpus or UI, and `span_events.fingerprint` is never SELECTed. So this moves storm
grouping and nothing else. No persisted data changes meaning; no `SCHEMA_VERSION` decision arises.

**Switching cost — the external harness re-aligns a third time.** Conductor had just aligned to the greedy
behaviour: its P-017 spec clause (leading-segment precision), its `FingerprintVariant` tests (below-leading-
segment changes insignificant) and `scenarios/fingerprint-storm.toml` all encode the any-slash semantics as of
2026-08-16. Those three artifacts now need updating. Bounded and known. **Executed in that repo — Conductor is
read-only from here; this record is the citation, not a change.**

**Rejected — re-document greedy as intended.** Zero code change and zero Conductor cost, and the coarser-
identity argument is real (fewer fingerprints, less storm fragmentation). Rejected because it would take
current behaviour as evidence of intent, which the route entry explicitly warned against, and because it
permanently accepts that two different faults sharing a leading segment are indistinguishable to the detector.

---

## Layer 2 — incident identity: **COALESCE-PER-CUE-IDENTITY**

**Decision.** Incident identity stays the cue tuple `(kind, scope, scope_id)` — for storms, effectively
per-service. A storm carrying a different fingerprint on a service that already has an open incident is
absorbed into it **by design**. No behavioural change; the predicate is untouched.

**Why.** Incident-per-identity buys no correctness today, because every downstream surface is already N-safe:
the per-service constellation join reduces by `max_by_key(tier_rank)` over all matching active incidents, and
the digest assembler reads `list_active` only as a boolean ("is any active incident ≥ Suggested?"). A second
concurrent incident would therefore change no rendering and no L4 behaviour — it would only add rows. And the
real fingerprint is not available at the incident site to key on.

**What changed.** Documentation and one test. The predicate now states the decision and why fingerprint is
absent from the key, so a future reader does not "repair" it. A new pin,
`distinct_fingerprint_same_service_still_coalesces_to_one_incident`, converts the previously-surprising canary
behaviour into an intentional, guarded semantic.

**Rejected — key on `L4Output.fingerprint`.** The option the entry's wording most suggests, and a trap. The
field is model-authored, and the deterministic runner hardcodes it to `"deterministic-l4-fixture"`, so
per-fingerprint dedupe would be a **no-op under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`** — the exact mode
chosen for e2e verification *because* it is reproducible. Under a real model it is worse: the same storm can
yield different strings across invocations, opening spurious duplicate incidents. This is the same
vacuous-verification failure class already recorded against P-075.

**Rejected (for this chunk) — thread the real fingerprint through `AttentionCue`.** The honest way to make
incident-per-identity real: add a fingerprint field to `AttentionCue` (a `triage::contract` change), populate
it at `synthesize_cue`, carry it through `DigestCueRef` into the digest, and join it to the key. It would also
repair the retrieval arm below. Rejected here as a genuine scope expansion — a contract type change plus four
or more files, invalidating three pinned tests — not because it is wrong. **This is the documented path if
incident-per-identity is ever wanted; do not partially implement it.**

---

## Consequence — the unreachable retrieval arm, narrowed

`select_corpus_matches` compared `Incident.fingerprint` (namespace B) against a fingerprint set the assembler
derives as `hex_lower(Q3 blake3 bytes)` (namespace A). It could not be true in production; only `scope_match`
ever fired. It read as live because its own tests supplied symbolic strings on both sides — self-consistent,
but not the production shape.

The arm is removed and the function is documented as scope-based. The parameter is retained so the caller keeps
its shape (`assembler.rs` is outside this chunk's scope), and it becomes meaningful only if a real exception
fingerprint is threaded onto the incident. Three tests depended on the arm — two the plan predicted, plus
`select_corpus_matches_ranks_by_recency_and_caps_at_limit`, which selected purely via fingerprint — all
retargeted to the scope contract.

## Known consequence, documented not fixed

`select_previously_seen` compares `Incident.fingerprint` to `Incident.fingerprint` — same namespace, so it is
genuinely live. But because the deterministic runner emits a **constant** fingerprint, every incident created
under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` shares it, so incidents will match each other as "previously
seen" on the fingerprint arm in that mode. This is a property of the deterministic fixture, not a defect
introduced here, and fixing it needs the `AttentionCue` threading rejected above. Same vacuous-surface family
as the P-075 CARRY.

## Why this chunk preceded P-075 / P-076

Both later entries assert on incident formation and read-back, so their acceptances encode whatever fault
identity means. Settling it first lets those acceptances be written **once**, against decided semantics,
instead of freezing today's measured-but-undecided behaviour into two suites.
