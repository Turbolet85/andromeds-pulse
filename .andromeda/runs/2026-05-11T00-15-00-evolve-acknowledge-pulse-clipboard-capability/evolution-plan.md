# Evolution Plan — acknowledge-pulse-clipboard-capability

**Run timestamp:** 2026-05-11T00:15:00Z
**Skill invocation:** `/andromeda-evolve --allow-arch-registry`
**Status:** Applied (Phase 6 atomic write succeeded)

## User intent (verbatim)

User wanted to add the `pulse:clipboard` capability identifier to arch §Occupied Resources Tauri capability identifiers reserved list (current set: pulse:default, pulse:tray, pulse:notification, pulse:updater, pulse:plugin-fs). Chunk #43 partial implementation introduced `pulse-app/capabilities/clipboard.json` with `clipboard-manager:allow-write-text` permission scope; arch had not acknowledged the new identifier. This is the recommended Priority 1 next-action from session 51 wrap-session handoff.

## Skill classification

- **Type:** Type 6 — Architecture registry update
- **Flag used:** `--allow-arch-registry`
- **Eligibility:** flag set + Phase 2 indicators matched (arch.md §Occupied Resources target, capability-drift class, code already implements per Check 7.3 code_evidence)

## Files touched

- `.andromeda/architecture.md` — §Occupied Resources Tauri capability identifiers list addition (`pulse:plugin-fs` → `pulse:plugin-fs, pulse:clipboard` + cross-reference note) + §Architecture Registry Updates entry append
- `.andromeda/runs/2026-05-11T00-15-00-spec-amendment-acknowledge-pulse-clipboard-capability/amendment.md` — marker file (NEW)
- `.andromeda/state.yaml` — spec_amendments.active gains 1 Type 6 entry (now 2 active total: this Type 6 + prior Type 7 from earlier this session)
- `.andromeda/runs/2026-05-11T00-15-00-evolve-acknowledge-pulse-clipboard-capability/evolution-plan.md` — this file (NEW)

## Cross-references

- Single-marker amendment (no sibling cross-references this invocation)
- Sibling Type 6 precedents (archived): `2026-05-09T11-45-00Z-legitimize-streams-namespace`, `2026-05-09T11-45-00Z-legitimize-telemetry-namespace` — same pattern (additive capability/namespace acknowledgment)
- Originating chunk: chunk #43 partial commit 6e2d398 (session 51, wrap commit)
- Code evidence file: `pulse-app/capabilities/clipboard.json`
- state.yaml.spec_amendments.active entry: `2026-05-11T00-15-00-acknowledge-pulse-clipboard-capability`
- state.yaml.drift_warnings D3 entry: will clear at next /andromeda-wrap-session Phase 6 re-detection (since `pulse:clipboard` now appears in arch §Occupied Resources, the D3 condition no longer holds)
- Marker: `.andromeda/runs/2026-05-11T00-15-00-spec-amendment-acknowledge-pulse-clipboard-capability/amendment.md`

## Validation results

All Phase 3 checks passed:

- ✓ Check 1 (spec-amendment-protocol compliance)
- ✓ Check 2 (state.yaml schema compliance — Type 6 fields present: flag_used, registry_section, registry_additions, code_evidence)
- ✓ Check 3 (Decisions Log format consistency — matches arch.md §Architecture Registry Updates 2026-05-09 entries)
- ✓ Check 4 (cross-reference integrity — single marker, trivially satisfied)
- ✓ Check 5 (vision document principles compliance — clipboard write-only is defensive; no principle conflict)
- ✓ Check 6 (refuse taxonomy double-check — Type 6 confirmed; no other refuse pattern)
- ✓ Check 7 (arch registry flag verification — all 4 sub-checks pass; see amendment.md Verification section)

## Suggested next steps

1. **`/andromeda-setup-project --delta`** — propagates the active Type 6 + Type 7 amendment lifecycles (2 active entries: this Type 6 + prior Type 7 from same session). For both with empty `expected_propagation`, --delta runs lifecycle progression only (sets `propagated_by_run`); no Tier 2/3 file regeneration. (Note: arch registry additions + route additions do not cascade through Tier 2/3 in the same way specialist plan amendments do.)

2. **Future `/andromeda-wrap-session`** — will note + archive both active Type 6 and Type 7 amendments per standard lifecycle (active → archive) once `propagated_by_run` is set. D3 pulse:clipboard drift_warning will clear at Phase 6 re-detection (the condition `pulse:clipboard NOT in arch §Occupied Resources` no longer holds after this amendment lands).

3. **Future `/andromeda-implement`** against the new chunk #44 (added by prior Type 7 amendment in same session) — picks up the chunk #43 deferred runtime + UI scope.

## Notes

- This is the THIRD use of `--allow-arch-registry` flag (Type 6) on this project, after 2026-05-09 streams.* + telemetry.* additive precedents (both archived in state.yaml).
- Combined with the prior Type 7 (chunk #44 addition via --allow-route-append) earlier in this same session, this represents two distinct Type-6-class evolutions in one wrap cycle: route additive (Type 7 — first ever use of --allow-route-append flag, which was added to evolve in this same session) + arch registry additive (Type 6 — third use of --allow-arch-registry on this project).
- D3 pulse:clipboard drift_warning persists in state.yaml.drift_warnings until next wrap-session Phase 6 re-detects (the file is updated via this amendment but state.yaml.drift_warnings is derived state written by wrap-session, not by evolve per Refuse 5 / state.yaml.drift_warnings constraint).
