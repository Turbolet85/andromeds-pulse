# Intent — append-chunk-66-exception-fingerprinting-retry-storm

_Captured by /andromeda-evolve Phase 1b+1c at 2026-05-17T22:01:44Z._
_Slug: append-chunk-66-exception-fingerprinting-retry-storm_
_Flag: --allow-route-append_

## Phase 1b (brief intent — derived from session-handoff Next Recommended Action; user invoked /andromeda-evolve --allow-route-append per the handoff suggestion)

Register chunk #66 "Exception fingerprinting + retry storm detector" in
route.md §2 Epoch 9 — Foundation v0.2.0, following the chunk #65 precedent.

## Phase 1c (full intent — synthesized from session-handoff + docs/v0_2_0/pulse-v0_2_0-route.md §Phase 2 line 240-251 per user no-pause directive; no interactive dialogue this invocation)

Chunk #66 is the natural successor to chunk #65 (span events ingestion;
landed at session 86 wrap commit 0bd0d76). Chunk #65's plan explicitly
declared the `span_events.fingerprint` BLOB NULL column as substrate for
chunk #66 population:

  > "Adds `span_events.fingerprint` column populated by #66."
  > — pulse-v0_2_0-route.md §65 line 238

Source plan for chunk #66 (pulse-v0_2_0-route.md §Phase 2 line 240-251):

  - Depends on: #65, #60
  - Capabilities enabled: P-017 (Exception Fingerprinting),
                          P-018 (Retry Storm Detection)
  - Distillation layer: L1c (fingerprint computation) + L2 (storm detection)
  - Crates touched: `crates/triage/pattern`, `crates/buffer/appender.rs`
  - Summary: ExceptionFingerprint = hash(exception.type + normalized first
    3 stack frames). Normalization strips paths, addresses, line numbers.
    Fingerprints written to `span_events.fingerprint` column in ingestion
    hot path. `DashMap<ExceptionFingerprint, RecentOccurrences>` with 60s
    window. ≥5 occurrences per 30s → emit `RetryStorm` AttentionCue with
    Suggested severity hint, ≥10 → Autonomous hint.

Chunk #60 (triage pattern module + 10 contract types in triage::contract)
landed at session 74 — provides the `crates/triage/pattern` substrate
chunk #66 will extend.

Form: 1 (chunk append to existing epoch). Existing Epoch 9 — Foundation
v0.2.0 currently has 9 chunks (#57-#65); chunk #66 appends as the 10th.
Insertion position 66 > last_completed_chunk.route_index=65 (Check 8.3
clean). in_progress=null (Check 8.4 confirmed_shift=false).

Following chunk #65's compact-format precedent for both §2 chunk text
(≤25 words; cites pulse-v0_2_0-route source plan for detail) and §3
Decisions Log entry (Insert/Why/Mechanical/Marker bullets per Proposal
9 Phase 1(b) compact template).

## Audit trail

- Evolve run-dir: this file
- Spec-amendment run-dir: `.andromeda/runs/2026-05-17T22-01-44-spec-amendment-append-chunk-66-exception-fingerprinting-retry-storm/`
- Marker file: see spec-amendment run-dir + amendment.md
- Route.md edits: §1 (Total chunks 65 → 66), §2 (Epoch 9 body chunk #66 append), §3 (Decisions Log new entry)
- State.yaml edit: spec_amendments.active +1 verbose entry
- Originating chunk: #65 (commit 0bd0d76 — span events ingestion + fingerprint substrate creator)
- Source plan: docs/v0_2_0/pulse-v0_2_0-route.md §Phase 2 line 240-251
