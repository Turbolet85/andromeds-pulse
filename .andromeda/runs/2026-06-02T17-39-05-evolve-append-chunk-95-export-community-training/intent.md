# Intent — append-chunk-95-export-community-training

_Captured by /andromeda-evolve Phase 1 (Phase 1b brief + Phase 1c depth)._

## Phase 1b brief intent (verbatim)

"§93 Export for community training"

## Phase 1c depth (derived from source plan + confirmed at Phase 2/Phase 5)

User requested appending the next v0.2.0 chunk — `docs/v0_2_0/pulse-v0_2_0-route.md` §93 "Export for community training" — to the route. Route is 94/94 implemented, so this becomes chunk #95, a Form 1 append to the existing Epoch 9 — Foundation v0.2.0 (where chunks #57–#94 live).

Grounded scope from §93:

- Settings → Storage → "Export for community training" action → anonymized JSONL dump from corpus (trigger context, model interpretation, user feedback, resolution outcome per record).
- Pre-write preview dialog (count by category, date range, anonymization confirmation) before file write.
- Default target `~/Downloads/pulse-corpus-export-{timestamp}.jsonl`.
- No automatic submission to external endpoints — user manually shares the file.
- Capability P-046 (Export for Community Training); distillation layer L5 channel.
- TauRPC delta: +1 procedure `storage.export_for_training(target_path)`.
- Crates touched (future impl): `crates/corpus/`, `pulse-app/ui/settings/Storage.tsx`.
- Depends on #69 (corpus), #78 (incident records), #47 (PII scrubbing) — all landed.

Privacy note: local-first / privacy non-negotiables are preserved (anonymized, opt-in, no network transmission, user-initiated local file export) — clears Refuse 2.

## Final slug

append-chunk-95-export-community-training

## Classification (Phase 2, user-confirmed)

Type 7 — Route registry update (Form 1 — chunk append to existing Epoch 9 — Foundation v0.2.0). New chunk #95 at terminal position (after #94). Confirmed by user at Phase 2 ("yes") and Phase 5 ("yes").
