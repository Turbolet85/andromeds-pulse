# /andromeda-evolve --allow-arch-registry — Intent

**Invocation:** `/andromeda-evolve --allow-arch-registry`
**Flag:** `--allow-arch-registry`
**Date:** 2026-05-24T15:58:15Z
**Session:** 140 (next; current is mid-evolve)
**Slug:** `chunk-82-arch-registry`

## User intent (Phase 1b confirmed)

File the chunk #82 Type 6 follow-up amendment registering 3 items in arch §Occupied Resources:

1. **`interpretation` crate** in §Occupied Resources Cargo workspace crate names sub-section (mirror chunk #58 curation crate single-item Type 6 precedent at 2026-05-16)
2. **`model.current_profile`** in §Occupied Resources Tauri IPC routes sub-section (mirror chunk #59 connection.current_state precedent at 2026-05-16 + chunks #67 services / #68 storage / #69 diagnostics multi-item precedents)
3. **`pulse://stream/model-status`** in §Occupied Resources Tauri IPC events (broadcast channels) sub-section (mirror chunks #62 attention-cues / #63 restart-events / #80 cadence-events / #81 digests single-item broadcast topic Type 6 precedents)

Plus a single compact §Architecture Registry Updates Decisions Log entry per Proposal 8 Phase 1 canonical compact template citing all 3 additions.

## Why now

- Chunk #82 implementation landed this session (commit cf6686b — "chore(implement): chunk #82 Hardware profile detection + model loading + tokenizer substrate"); 3 D3 drift entries fired в session 139 wrap-session Phase 6 drift detection.
- Code evidence already in place for all 3 items:
  - `crates/interpretation/Cargo.toml` + `crates/interpretation/src/{lib.rs, contract.rs, hardware.rs, broadcast.rs}` (new workspace member)
  - `pulse-app/src/model_router.rs::ModelApi::current_profile` (TauRPC procedure) + `xtask/src/main.rs::EXPECTED_PROCEDURES` registered
  - `crates/interpretation/src/broadcast.rs::STREAM_NAME_MODEL_STATUS = "pulse://stream/model-status"` (broadcast topic constant)
- Chunk-then-amendment Type 6 precedent applies (chunks #62/#63/#67/#68/#78/#80/#81 all followed this pattern).
- Single-coordinated multi-item amendment per multi-item precedents (chunks #67 services-namespace + #68 corpus-additions + #78 incidents-namespace) — items are tightly related (all from chunk #82) and a single Decisions Log entry is more readable than three separate entries.

## Expected output

- 1 amendment marker file (single coordinated multi-item Type 6 amendment)
- 1 Decisions Log entry в arch.md §Architecture Registry Updates (compact template per Proposal 8 Phase 1)
- arch.md §Occupied Resources updates (3 sub-sections touched)
- state.yaml.spec_amendments.active +1 entry
- Suggested next step: `/andromeda-setup-project --delta` к propagate

## Classification preview

- Refuse: NO match (purely additive registry-section additions under flag exception)
- Type: 6 (Architecture registry update)
- Impact:
  - Plans touched: 1 (arch.md)
  - Decisions Log entries: 1 (in arch §Architecture Registry Updates)
  - Marker files: 1 (single coordinated)
