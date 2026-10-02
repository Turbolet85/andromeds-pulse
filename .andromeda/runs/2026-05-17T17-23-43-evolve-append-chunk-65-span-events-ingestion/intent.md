# Intent — append-chunk-65-span-events-ingestion

_Captured by /andromeda-evolve Phase 1c step 6._

## User input verbatim

### Phase 1b — brief intent (one sentence)

Append chunk #65 "Span events ingestion" to route.md §2 Epoch 9 — Foundation v0.2.0.

(Selected from a Phase 1b AskUserQuestion option set seeded by session-handoff
recommendations. Two candidates were offered: (a) Span events ingestion, (b) Corpus
SQLite scaffold. User selected (a).)

### Phase 1c — confirmation

User confirmed proposed draft (chunk text + motivation grounding) without
adjustments. Single follow-up question used (1 of 4 Phase 1c cap).

## Final slug

`append-chunk-65-span-events-ingestion`

## Synthesized intent

- **Plan targeted:** `.andromeda/route.md` §2 Roadmap (Epoch 9 — Foundation v0.2.0) + §3 Decisions Log
- **Change:** Append chunk #65 "Span events ingestion" after current chunk #64 (terminal position of Epoch 9 body). Mechanical §1 Total chunks 64 → 65 per Proposal 6 Form 1 Policy A.
- **Motivation:** L0 schema population layer per pulse-v0_2_0-route §Phase 2 line 225-238. Capability P-006 (Exception Event Capture) prerequisite for chunk #66 (exception fingerprinting + retry storm detection). Depends on nothing per source plan; storage-side only (`crates/buffer/appender.rs` extension to populate `span_events` table).
- **Form:** Type 7 Form 1 (chunk append to existing epoch — Epoch 9 already exists from chunk #57 evolve cycle).
- **Flag:** `--allow-route-append`

## Source plan reference

`docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 2 — Algorithmic detection layer, line 225-238 chunk #65 entry.
