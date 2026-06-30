# Session Handoff

**Last Updated:** 2026-06-30T22:49:18Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-06-30-widget-to-dashboard-navigation` — widget-to-dashboard TOGGLE (button + unified Cmd/Ctrl+Shift+P, widget always stays) + P-061 geometry & P-063 close/toast corrections (P-066)

## Position
- Done: `2026-06-30-widget-to-dashboard-navigation` (P-066) — an in-widget "Toggle dashboard" titlebar button + a unified Cmd/Ctrl+Shift+P shortcut TOGGLE the dashboard window (open-if-hidden / hide-if-shown) with the glance widget always visible; folded in a P-061 geometry correction (off-screen remembered position → fixed margin-inset corner) + P-063 close corrections (per-window: widget ✕ = app-to-tray + toast; dashboard ✕ = silent collapse; toast now every-time). Master: **8 chunks complete, 0 pending**. **10/18 v0.3.0 caps verified** (P-061…P-066, P-072, P-073, P-074, P-078).
- Next: **P-067 — Live-only service truth** (Epoch 3; show only currently-live services, hide/mark persisted registry entries · intent F7) — the next markerless working-route entry → `/andromeda-phase`.

## Work done
4 new files (`icons/Expand.tsx`, `hooks/use-toggle-dashboard.{ts,test.ts}`) + 16 modified: `default.json` (+3 `core:window` perms); Rust `window.rs` (per-window close + every-time toast + fixed-corner geometry, no remembered widget pos), `main.rs` (boot wiring), `unit_close_signpost.rs`; webview `Titlebar`/`CompactWidget`/`router`/`use-keyboard-shortcuts` (+ tests) + icon registry ×4. Heavy 3-round dogfood (toggle UX → per-window close + every-time toast → fixed on-screen geometry); each round rebuilt + self-verified + **manually clicked through by the user + obs-log-confirmed**. Gates: webview typecheck/lint/**vitest 667**/build/**test:a11y** (0 regressions) ✓ · `clippy --workspace --all-targets --all-features` ✓ · **`nextest --workspace` 1718/1718 + 1 skip** ✓ (the P7 light gate; this is a Rust-touching chunk) · capability-drift/widening/verify-matrix ✓ · `self-verify` ×3 ✓ · cargo check (ACL valid) ✓.

## Drift resolved
P2 fan-out: 7 docs, 3 proposals. **a11y §P5 — APPLIED** (button accessible name "Expand to dashboard" → "Toggle dashboard" + focus note; cascaded to `a11y.md` rule + `a11y-summary.md` + 3 SR scripts + sidecar). **test-plan §3 weaken-the-gate — REJECTED + codified** (user-approved: the full nextest RAN green at the wrap light gate, so §3 is satisfied — the deferral was /implement→wrap timing, not a skip; new playbook rule appended, testing.md unchanged). **layout titlebar-button — routine-REJECT** (within-titlebar element; Investigate-button precedent → handoff note below). arch/security/design/obs clean. **0 escalations open · drift = 0.**

## Notes
- **Key decisions (dogfood-driven, user-directed):** toggle-keep-widget-visible (hiding the widget on "expand" stranded the user — found live, not by tests); unify Cmd/Ctrl+Shift+P onto the same `toggleDashboard()` mounted in BOTH windows (the old dashboard-only shortcut leaked to the WebView2 print dialog from the widget); per-window close (widget=primary/app-to-tray, dashboard=secondary/collapse); every-time toast (removed the first-close latch); fixed margin-inset corner geometry, drop the remembered widget position (clamp-to-work-area if ever reintroduced); first-party `expand` glyph over a Lucide dep.
- **Curation:** T2 ×2 (`frontend.md` — window-pair toggle + shared-shortcut-in-both-windows pattern; `verification-harness.md` — manual-verification-via-obs-log technique). T3 ×1 (`session-learnings.md` — window.rs corrections: geometry/per-window-close/every-time-toast + the clippy `collapsible_match`→match-guard note). 1 dedup-filtered (bindings.ts restore — covered by existing entries). 0 conflicts · 0 deferred.
- **HANDOFF — layout-templates titlebar enumeration gap:** §Custom titlebar component lists app-icon/title/settings/window-controls but NOT the action buttons (Investigate from #42, now Toggle from this chunk). Pre-existing completeness gap; routine-REJECTed this wrap per the within-surface precedent. A future targeted touch-up could enumerate the titlebar action-button cluster (both at once, for consistency).
- **HANDOFF — standing testing.md gate-deferral conflict (from last chunk) persists:** "may a ZERO-.rs webview chunk defer heavy Rust gates at /implement?" was NOT triggered by this chunk (it has a Rust delta + ran the full nextest). The new playbook rule covers the related "defer-then-run-at-wrap" case; the zero-.rs question is still open for a future webview-only chunk.
- bindings.ts canonical (mcp present — restored from HEAD after the debug-binary boots regenerated it; this chunk added no procedure). Branch local-only — **NOT pushed**. Last failed command: none.

## Session End Status
Wrapping at 2026-06-30 (session 9).
