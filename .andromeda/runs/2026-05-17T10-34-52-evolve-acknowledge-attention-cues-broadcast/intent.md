# Intent — acknowledge-attention-cues-broadcast

**Captured:** 2026-05-17T10:34:52Z
**Skill invocation:** `/andromeda-evolve --allow-arch-registry`
**Slug:** acknowledge-attention-cues-broadcast

## Phase 1b — Sanity check (one-sentence intent, verbatim)

> Type 6 — add `pulse://stream/attention-cues` + `cadence-triggers` to arch §Occupied Resources Tauri IPC events

## Phase 1c — Clarifying-question answer

**Q:** Where should `cadence-triggers` (internal tokio broadcast, not Tauri-bridge) be registered in arch §Occupied Resources?

**A:** Only the pulse:// one — add only `pulse://stream/attention-cues` to "Tauri IPC events (broadcast channels)"; do NOT acknowledge `cadence-triggers` in arch (it is a true internal cross-crate channel, not occupying any registry-grade identifier).

## Resolved scope

- **Affected plan:** `.andromeda/architecture.md` (Type 6, flag-authorized)
- **Affected sub-section:** §Occupied Resources → "Tauri IPC events (broadcast channels)"
- **Registry addition (1 item):** `pulse://stream/attention-cues`
- **Code evidence:** `crates/triage/src/cue/broadcast.rs:8` (`STREAM_NAME_ATTENTION_CUES` const) — emitted by `AttentionCueBroadcast::send` from `crates/triage/src/cue/emitter.rs` per chunk #62 implementation
- **Originating chunk:** #62 "Attention cue emitter" (route §2 Epoch 9 Foundation v0.2.0 sixth chunk; committed 2026-05-17 commit `aeb4d7d`)
- **Drift reference:** state.yaml.drift_warnings[D3] anticipates this amendment (severity warning, first/last observed session 79)

## Motivation

Chunk #62 lands the attention cue emitter, which broadcasts cues over `pulse://stream/attention-cues` (the new Tauri IPC event topic). arch.md §Occupied Resources Tauri IPC events sub-section had not yet acknowledged the new topic — D3 capability-drift gap. Amendment closes the gap; mirrors chunk #59 precedent for `pulse://stream/connection-state`.

`cadence-triggers` is an internal tokio broadcast channel inside the triage crate (cue emitter → future cadence coordinator, chunk #72); it does not cross the Tauri bridge and therefore is NOT a registry-grade identifier for §Occupied Resources. Intentionally excluded from this amendment per Phase 1c clarifying-question answer.

## Anticipated next step after this evolve

`/andromeda-setup-project --delta` to propagate the amendment through Tier 2/3 + CLAUDE.md ecosystem (typically minimal for Type 6 registry-only additions — empty `expected_propagation`; --delta runs lifecycle progression only).

Then `/andromeda-wrap-session` to archive the amendment.

This is anticipated to be the first live P7 narrative-cascade test per session 78 Proposal 7 status (PROPOSED, awaiting live `--allow-arch-registry` cascade); Check 7.5 narrative-cascade scan runs against arch.md structural sections (Design Philosophy / Inherited Defaults / Cross-cutting Patterns / Project Intent) but the count-noun whitelist (`library crates` / `workspace members` / `capability identifiers` / `TauRPC procedures` / `crates`) does NOT include "Tauri IPC events" — so 7.5 is expected to surface zero warnings for this specific amendment.
