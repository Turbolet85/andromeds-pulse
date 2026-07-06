# Session Handoff

**Last Updated:** 2026-07-06T19:44:31Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-07-06-incidents-panel-dropdown-layout-bug` — bounded upward incidents (Findings) dropdown popover (P-080)

## Position
- Done: `2026-07-06-incidents-panel-dropdown-layout-bug` (P-080) — the compact-widget Findings (incidents) dropdown now renders as a **bounded, opaque `#2D3139` popover fully within the widget** (opens UPWARD, `overflowY:auto`, no window stretch, no white/off-viewport). **Operator visual-verify PASSED** (screenshot). **P-080 verified → 15/20 v0.3.0 caps.**
- Next (first markerless): **Traces table auto-refresh** → `/andromeda-phase`. **BUT NOTE:** the operator-directed **Incidents floating-window disclosure** chunk was added to the END of the Epoch-3 tail as "future work" — reorder it forward if you want it before Traces auto-refresh (see Notes).

## Work done
Webview-only (2 files): `FindingsDropdown.tsx` `PANEL_BASE_STYLE` `top:`→`bottom: calc(100% + var(--spacing-xs))` (open upward) + `maxHeight: calc(100vh - 32px - var(--spacing-lg))` + `overflowY:"auto"`, kept `--color-raised-2` · `FindingsDropdown.test.tsx` +3 DOM-shape layout-lock tests · matrix P-080 → verified. Gates: webview green (vitest 696/696 +3 · lint · typecheck · build · test:a11y p8 axe + regression 0-new). Deferred (zero-`.rs`-delta): `clippy`/`nextest --workspace`/`capability-drift`. Smoke: warm re-embed boot (0 panics, deterministic-L4 storm → 11 incidents) + operator visual verify PASSED.

## Drift resolved
none — all 7 spec-source detectors returned `proposals: []` (webview-only, zero new arch resource/dep/API/crate/schema/telemetry; tokens✓; existing a11y-covered surface). 0 amendments · 0 escalations · cascade no-op.

## Notes
- **Operator re-opened after accepting the upward popover** → wants the incidents disclosure as a **SEPARATE floating window docked BELOW the widget** ("drops under the widget"; the upward popover can't extend past the fixed webview edge — that was the original bug). Landed this chunk as the interim bug-fix; **added "Incidents floating-window disclosure" to the Epoch-3 markerless tail** (operator-directed "future work"; placed at the tail END so it doesn't jump the Traces-auto-refresh priority — **reorder if you want it sooner**). `/andromeda-phase` it (new Tauri window + position-below-widget/off-screen + cross-window focus/a11y + capability — de-risk with a proper plan).
- **Badge-reactivity bug found (operator):** `use-findings.ts` fetches incidents only at mount + on window-focus, never re-polls → the Findings badge stays hidden until a widget RESIZE forces a focus refetch (same class as the Traces auto-refresh follow-up). **CARRY-folded** onto the floating-window chunk (which reuses `use-findings.ts`): add a periodic re-poll (constellation ~1s precedent) or the `pulse://stream/incidents` subscription.
- **Premise correction (operator-confirmed at /phase P4):** kept `--color-raised-2` (design-system popover token), NOT the working-route's suggested `--color-inset` — the "white bg" was the off-viewport-overflow symptom, not a wrong token.
- **Curation:** Tier 2 ×3 — testing.md (jsdom 26 stores `calc()`/`var()` inline-style values → assert `element.style.*` directly) · frontend.md ×2 (fixed-webview popover can't extend past the window; webview live-data hooks that fetch-once-don't-re-poll go stale). Filtered: 2 dedup + 1 → this handoff.
- **Deferred gates (zero-`.rs`-delta):** re-run `clippy`/`nextest --workspace`/`capability-drift` at the next `.rs`-touching chunk.
- **Design "no unstyled scrollbar" ban (pre-existing project-wide gap):** the new `overflowY:auto` matches the Modal/CommandPalette bare-`overflowY` precedent; a project-wide scrollbar-styling pass is a future concern (not this chunk).
- Branch local-only — **NOT pushed**. Last failed command: none.

## Session End Status
Wrapped `2026-07-06-incidents-panel-dropdown-layout-bug` at 2026-07-06T19:44:31Z (session 14).
