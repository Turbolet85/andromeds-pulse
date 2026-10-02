# Evolution Plan — acknowledge-restart-events-broadcast

**Generated:** 2026-05-17T14:15:00Z
**Skill:** `/andromeda-evolve --allow-arch-registry`
**Mode:** default (no --dry-run)

## User intent (verbatim)

(See `intent.md` in the same run-dir.)

Acknowledge the `pulse://stream/restart-events` Tauri broadcast topic in arch.md §Occupied Resources Tauri IPC events (broadcast channels) sub-section. Topic introduced by chunk #63 (commit `61ca564`); D3 drift first observed at session_count 81 per `state.yaml.drift_warnings`.

## Classification

**Type 6 — Architecture registry update** (--allow-arch-registry; narrow Refuse 1 exception per refuse-taxonomy.md §Refuse 1 Exception).

## Files touched

| File | Operation | Description |
|---|---|---|
| `.andromeda/runs/2026-05-17T14-15-00-spec-amendment-acknowledge-restart-events-broadcast/amendment.md` | Create | Type 6 marker file (Part A conformant) |
| `.andromeda/architecture.md` | Edit | §Occupied Resources inline list append + §Architecture Registry Updates entry append (compact form) |
| `.andromeda/state.yaml` | Edit | Append 1 entry to spec_amendments.active |
| `.andromeda/runs/2026-05-17T14-15-00-evolve-acknowledge-restart-events-broadcast/evolution-plan.md` | Create | This file |

Plus run-dir-local `intent.md` (already written at Phase 1c).

## Cross-references

- Originating commit: `61ca564` (chunk #63 "Restart event detector + dual-condition bypass"; route §2 Epoch 9 Foundation v0.2.0 seventh chunk; HEAD at 2026-05-17T13:20:00Z wrap commit).
- Precedent: 2026-05-17 chunk #62 `pulse://stream/attention-cues` amendment at `.andromeda/runs/2026-05-17T10-34-52-spec-amendment-acknowledge-attention-cues-broadcast/amendment.md` (mirrors structure — single registry addition, same sub-section, same flag authorization).
- D3 drift_warning closure: clears the only entry in `state.yaml.drift_warnings` (first_observed_session_count=81; will be removed by next /andromeda-wrap-session Phase 6 drift-detection re-run after propagation).
- session 81 handoff "Next Recommended Action": this amendment IS the recommended action.

## Validation results (Phase 3)

- Check 1: ✓ Spec-amendment-protocol Part A compliance
- Check 2: ✓ state.yaml Part B schema compliance
- Check 3: ✓ Decisions Log format consistency (compact P8 Phase 1 form)
- Check 4: ✓ Cross-reference integrity
- Check 5: ✓ Vision document principles compliance (local-first / OSS / privacy untouched by registry addition)
- Check 6: ✓ Refuse taxonomy double-check (no refuse pattern matched)
- Check 7 — Arch registry flag verification (all 6 sub-checks):
  - 7.1 (purely additive): ✓
  - 7.2 (registry section only): ✓ (§Occupied Resources Tauri IPC events (broadcast channels))
  - 7.3 (code evidence): ✓ (`crates/triage/src/pattern/broadcast.rs:8` STREAM_NAME_RESTART_EVENTS resolved)
  - 7.4 (no new concept): ✓ (sub-section has 7 prior entries; this is the 8th)
  - 7.5 (narrative-cascade staleness): ✓ no staleness detected (whitelist excludes "Tauri IPC events" / "broadcast channels")
  - 7.6 (compact-format conformance): ✓ 5-content-line canonical shape; no Authority paragraph

Result: 7 ✓ / 0 ⚠ / 0 ✗

## Suggested next steps

1. Run `/andromeda-setup-project --delta` to propagate.
   - Expected scope: empty (per Type 6 baseline + empty `expected_propagation`); lifecycle progression only (sets `propagated_by_run` on the amendment entry).
   - No Tier 2/3 regeneration; no CLAUDE.md pointer-table cascade (arch.md is not pointer-table-tracked in that way for registry additions).
2. After --delta succeeds, run `/andromeda-wrap-session` to archive the amendment (move from `spec_amendments.active` to `spec_amendments.archive`; clear the D3 drift_warning on Phase 6 re-detection).
