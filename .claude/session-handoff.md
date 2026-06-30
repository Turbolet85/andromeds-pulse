# Session Handoff

**Last Updated:** 2026-06-30T16:58:36Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-06-29-window-size-constraints` — min-size + debounced aspect-band clamp for the glance widget (P-062) + folded-in allow-close (P-063 residual) + widget scrollbar fix

## Position
- Done: `2026-06-29-window-size-constraints` — glance-widget min-size (400×225) + a debounced aspect-band clamp [1.4–2.1], main-window min-size (800×600). Master: **6 chunks complete, 0 pending**. **7/18 v0.3.0 caps verified** (P-061, P-062, P-063, P-072, P-073, P-074, P-078).
- Next: **P-064/P-065 — Browser-chrome suppression** (default WebView2 context menu + canvas image-save suppressed app-wide · intent F4/F5) — the first markerless working-route entry (Epoch 2) → `/andromeda-phase`.

## Work done
P-062 core: `tauri.conf.json` min-size both windows + a pure `clamp_to_aspect_bounds` helper wired into `on_window_event`'s `Resized` arm, **debounced** (generation-counter + `tauri::async_runtime::spawn`/`sleep`) so it snaps once on release instead of fighting the drag. **Two folded-in fixes** (user-driven, per the fix-in-chunk preference): `core:window:allow-close` grant — the custom ✕ was DEAD (silently-dropped IPC) → now hides-to-tray; `CompactWidget.tsx` `<main>` `minHeight`→`height` — killed a vertical-overflow scrollbar. **Extensively hands-on verified by the user** (min-size, aspect snap, ✕→tray + signpost, no scrollbar). Gates: nextest **1719/1719+1skip** · webview **642** · clippy/capability-drift/widening clean · self-verify PASS.

## Drift resolved
P2 fan-out: 7 docs, **1 proposal** (arch D-arch-resources: add `core:window:allow-close` to arch §225 "Tauri capability identifiers"). **Escalated → REJECTED WITH the user** — core:window perms live in `default.json` with rationale per arch §Webview-IPC-capability-policy; §225 lists `pulse:*` capability FILES, and P-061's 3 sibling core:window perms were never added to arch (the precedent). **Codified a playbook rule** so future D-arch-resources firings on a core:window perm auto-reject. **0 amendments applied.** Drift = 0.

## Notes
- **Key decisions:** debounce-on-settle for the aspect clamp (per-event set_size fights the Windows modal resize → flicker); min-size raised 320→400×225 after the user's screenshot showed the titlebar wrapping (aspect math ≠ layout-fits); arch-reject (core perms not in §225).
- **P-063 CORRECTION flagged:** P-063 was matrix-`verified`, yet its ✕ was dead — the boot-quit self-verify never CLICKS a button. The allow-close grant this chunk landed makes P-063's close affordance actually work. (Curated to verification-harness.md.)
- **CARRY recorded:** P-062 headful resize-delta e2e → the **P-076** working-route line (beside the P-061 drag-delta CARRY).
- **Optional future (not drift):** layout-templates §Responsive-behavior could record the concrete 400×225 widget min-size + 1.4–2.1 band someday.
- **Curation:** T2 ×2 (verification-harness — verified-cap-can-hide-dead-affordance; frontend — fixed `height` not `minHeight` for viewport-fill) + T3 ×1 (window resize-constraint impl); 1 dup filtered (allow-close fact already in security.md). 0 conflicts, 0 deferred.
- bindings.ts canonical (restored to HEAD before capability-drift; unmodified, mcp present — no commit churn). Branch local-only — **NOT pushed**. Last failed command: none.
