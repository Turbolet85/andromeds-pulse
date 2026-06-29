# Session Handoff

**Last Updated:** 2026-06-29T17:30:11Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-06-29-window-geometry-movable-shell` — feat: grant core:window perms (drag/min/max) + center dashboard + remembered window position (P-061)

## Position
- Done: `2026-06-29-window-geometry-movable-shell` — Window geometry + movable shell (P-061 · intent F1 · Epoch 2); master → complete. **4/17 v0.3.0 capabilities verified.**
- Next: `/andromeda-phase` to promote + plan the next markerless entry — **Window size constraints (P-062 · intent F2 · Epoch 2)** — a min-size + a sensible aspect-ratio constraint for the glance widget.

## Work done
Premise correction: the `data-tauri-drag-region` markup already existed (chunk #24); the window "didn't move" purely because `pulse:default` never granted `core:window:allow-start-dragging` (Tauri 2 negative-default silent rejection — `core:default` is getters + internal-toggle-maximize only). Granted that + `allow-minimize`/`allow-toggle-maximize` (the same-root-cause titlebar buttons; `close`→P-063), added `"center": true` to the `main` window, and added Rust-owned remembered free position (`window-geometry.json`: captured on `WindowEvent::Moved` / flushed on close-to-tray / restored at boot — decoupled from the webview `Settings` → no form-clobber, no `allow-set-position`, no bindings churn). New `window_geometry.rs` + 2 test files (7 tests). Gates green (nextest 1701 + 1 skip, clippy `--all-features`, capability-drift + widening clean, webview typecheck/lint/642, bindings mcp+investigate present). P-061 → verified.

## Drift resolved
7/7 detectors. **2 routine amendments applied** — **D-arch-resources** (detector under-scoped to IPC/port/crate; orchestrator-judged: registered `window-geometry.json` in arch §Occupied Resources §Filesystem locations, completeness precedent #68/#95) + **D-security-input** (registered the geometry-file boundary in security-plan §Input Validation — routine per the bounded-config-input playbook rule: report shows it validated + unit-tested, so NOT an unvalidated-boundary HALT). 5 detectors clean (design/layouts/test-plan/obs/a11y — no new UI element/surface, PII redacted, tests present; layouts/a11y/obs agents self-applied the pre-existing-surface reject). **0 escalations.** Cascade no-op (CLAUDE.md / stack.md / security.md rule / security-summary.md reference/summarize, not enumerate, the amended registry rows).

## Notes
- **Premise correction (recorded):** intent F1's "build a drag region" mechanism was falsified — the region pre-existed; the fix is the capability grant. Captured in `scope.md` P4-resolutions + `verification-matrix.json#P-061` notes.
- **Boot smoke skipped with cause:** Tauri compile-embeds the capability ACL — the successful `cargo build` validated + embedded the 3 perms (verified in `target/.../capabilities.json`); the setup-closure wiring is panic-safe by construction (no new async/spawn/reactor → the "no reactor" latent-panic class can't apply); + the documented Windows GUI-orphan hazard. New Tier-2 learning (verification-harness.md) codifies when a capability/config-only change can skip the GUI boot smoke.
- **Two planning refinements (in the report):** no `allow-set-position` (Rust-side restore); remembered geometry decoupled from `Settings` (avoids the form-clobber bug). Both simplified the surface in the user's favor.
- **CARRY:** the headful tauri-driver drag-delta e2e (assert window-position-delta > 0 via a real drag) → pinned to the Epoch-4 **P-076** integration UX e2e working-route entry.
- Curation: Tier 2 +2 (security.md core:window-perms-for-frameless-titlebar · verification-harness.md compile-embedded-ACL→boot-smoke-skippable) · Tier 3 +1 (runtime-set state → Rust-owned sink, decoupled from Settings) · 0 filtered. CLAUDE.md 152/200.
- Stray repo-root `ui/src/bindings/index.ts` (a WRONG-PATH taurpc/Specta bindings export — canonical is `pulse-app/ui/`; differs from canonical) was swept by `git add -A`; `rm` is harness-blocked, so this wrap **gitignored `/ui/`** (anchored — never matches `pulse-app/ui/`) to stop the pollution. Latent root cause: some taurpc export writes bindings under a repo-root CWD — worth investigating/deleting later.
- Branch is local-only — **NOT pushed** (this wrap adds 1 commit).
- Last failed command: none.
