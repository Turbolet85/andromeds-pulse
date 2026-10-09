# Report — 2026-06-29-window-size-constraints

**Chunk:** Window size constraints — min inner-size + aspect-ratio constraint for the glance widget (P-062)
**Date:** 2026-06-30
**Commits:** (pending — wrap commit) feat(2026-06-29-window-size-constraints): min-size + debounced aspect-band clamp for the glance widget (P-062), + folded-in allow-close (P-063 residual) + widget scrollbar fix

## Changes (structured — detectors read this)
- **Files:**
  - `pulse-app/tauri.conf.json` — `minWidth`/`minHeight` on BOTH windows: `compact-widget` 400×225 (16:9), `main` 800×600.
  - `pulse-app/src/window.rs` — pure `clamp_to_aspect_bounds` helper + aspect-band consts + `WindowEvent::Resized` debounced clamp arm in `on_window_event`.
  - `pulse-app/src/main.rs` — captures an `Arc<AtomicU64>` resize-generation into the `on_window_event` closure (debounce state; replaced the earlier re-entrancy guard).
  - `pulse-app/tests/unit_window_constraints.rs` — NEW, 6 unit tests for the clamp helper.
  - `pulse-app/capabilities/default.json` — **folded-in fix:** grants `core:window:allow-close` (P-063 residual).
  - `pulse-app/ui/src/widget/CompactWidget.tsx` — **folded-in fix:** `<main>` `minHeight` → `height` (kills the vertical-overflow scrollbar).
- **Symbols / APIs:**
  - New `pub` (`#[doc(hidden)]`) `window::clamp_to_aspect_bounds(size, min_aspect, max_aspect) -> Option<(u32,u32)>` + `pub const WIDGET_MIN_ASPECT = 1.4` / `WIDGET_MAX_ASPECT = 2.1` (exposed for the integration test; `[lib] test = false` makes src `mod tests` non-running).
  - `window::on_window_event` signature: `aspect_guard: &AtomicBool` → `resize_gen: &Arc<AtomicU64>` (single caller = `main.rs` setup closure).
  - **No** new TauRPC procedure, IPC route, broadcast topic, env var, or port.
  - New Tauri capability permission `core:window:allow-close` (a CORE-window perm, not a TauRPC procedure → no `capability-drift` impact; not one of the 3 NEVER-widen caps → no `capability-widening-check` impact).
- **Crates / modules:** none added/removed; `pulse-app` changed.
- **Dependencies:** none added/bumped.
- **Schema / config:** `tauri.conf.json` window config (min-size, both windows); `default.json` permissions += `core:window:allow-close`. No corpus/DuckDB schema change.
- **Coverage of new surfaces:**
  - `window::clamp_to_aspect_bounds` (internal pure fn) → validation n/a · instrumentation: warn-on-`set_size`-failure, aggregate/label-only (log✓) · PII: no coordinate/dimension values logged (redacted✓) · tests: unit✓ (6) · a11y n/a · tokens n/a.
  - `Resized` aspect clamp (debounced via `tauri::async_runtime::spawn` + `tokio::time::sleep`) → no new external surface; runtime-verified (boot smoke + manual resize) · tests: helper unit-tested, live headful resize-delta CARRIED to P-076.
  - `core:window:allow-close` grant → tests: capability-widening-check✓ + tauri-build ACL-validated · a11y n/a · tokens n/a.
  - `CompactWidget.tsx` layout fix → tests: webview vitest✓ (642 passed) · a11y: no new violations (self-verify Playwright 30/30, pa11y 7/7) · tokens: unchanged.

## Deviations from intent
1. **Tests in `pulse-app/tests/unit_window_constraints.rs`, not `window.rs mod tests`** — plan Step 5's explicitly-anticipated fallback. Confirmed `[lib] test = false` (Cargo.toml:12) makes src-level `mod tests` compile-but-never-run under nextest (2026-05-20 trap). Verified the 6 tests run (1713→1719). Helper + 2 consts exposed `pub #[doc(hidden)]`.
2. **Min-size value 320×180 → 400×225** (compact-widget). The plan left exact values to implement; the initial 320×180 was tuned UP after the user's live screenshot showed the titlebar wrapping (title + 5 buttons can't fit on one row at ~320px → overflow). In-scope value refinement.
3. **Aspect clamp: debounced (settle-then-clamp), not per-event** — the plan specified a re-entrancy-guarded per-`Resized` `set_size`. Live testing surfaced that per-frame `set_size` during a Windows modal resize fights the cursor and flickers badly. Re-implemented as a generation-counter debounce (clamp once after ~150ms of no new Resized). Better UX + eliminates the fighting; the re-entrancy guard became the generation counter.
4. **`core:window:allow-close` grant — OUT of P-062's plan scope (folded in).** The frameless custom-titlebar ✕ (`use-window-controls.ts` `getCurrentWindow().close()`) was silently rejected (negative-default ACL) → "nothing happens on click". **P-063 residual:** P-061 granted drag/minimize/maximize but deferred close to P-063; P-063 added the close *signpost* on the Rust `CloseRequested` handler but never granted the webview permission to *reach* it. P-063 is marked `verified`, yet its primary close affordance was dead — the self-verify boot-quit never clicked the real button. Folded in per the user's standing in-chunk-fix preference. **wrap should record this corrects P-063.**
5. **Widget scrollbar fix (`CompactWidget.tsx` `minHeight`→`height`) — OUT of P-062's plan scope (folded in).** Pre-existing widget-shell CSS overflow: `<main minHeight: calc(100vh - 32px)>` let the element GROW past the viewport → vertical scrollbar (design-banned on the glance widget). Fixed to `height` (locks to viewport; could not use `overflow:hidden` — the findings dropdown pops downward and would be clipped). Frontend/CSS, separate from window-sizing. Folded in per user preference.

## Decisions & corrections
- **P4 scope decisions (user-selected, recorded in scope.md):** Q1 → min-size + aspect *band* (not hard 16:9 pin), both delivered in-chunk; Q2 → constrain the widget AND give the `main` dashboard a min-size.
- **Min-size values must be validated against the real titlebar/content layout, not just aspect math** — 320×180 satisfied the aspect band but broke the titlebar (wrap → overflow). Needed live visual feedback to set 400×225.
- **A per-event `set_size` aspect clamp during an interactive (modal) window resize fights the cursor → flicker;** debounce-on-settle (clamp once the resize stops) is the fix. The `tauri::async_runtime::spawn` + `tokio::time::sleep` debounce ran cleanly at runtime (no "no timer" panic — Tauri's runtime has timers).
- **A frameless custom-titlebar window's ✕/minimize/maximize each need their own `core:window:allow-*` grant** (extends the P-061 lesson to `allow-close`). A capability marked "verified" can still hide a DEAD affordance when the verification (boot-quit self-verify) never exercises the real user-facing button — manual click-testing caught what 1719 tests + the boot smoke did not.
- **`minHeight` on a viewport-fill flex container lets it grow → scrollbar; use a fixed `height`.** And `overflow:hidden` is unusable as the scrollbar cure when a descendant popover (findings dropdown, `top: calc(100% + …)`) must render outside the container.
- **User preference reconfirmed:** fix adjacent bugs in-chunk rather than deferring (multiple folded-in fixes accepted this session) — consistent with the saved `fix-in-chunk-preference` memory.

## Outcome
- **Acceptance MET:** resize below the min-size is clamped (OS-enforced via `tauri.conf.json`); aspect stays within the configured band (pure-helper unit tests + live-verified). Live headful resize-delta e2e CARRIED to P-076 (P-061 precedent).
- **Gates green (final full run, all changes present):** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -D warnings` (clean) · `cargo nextest --workspace --profile ci` (**1719/1719 + 1 skip**) · webview `eslint` + `tsc --noEmit` + `vitest run` (**642 passed / 69 files**) · `cargo xtask capability-drift` (clean 0/0) · `cargo xtask capability-widening-check` (clean 0/3).
- **Smoke:** `cargo xtask self-verify` PASSED (real binary boot, shell health, zero panics, zero orphan, a11y 30/30 + pa11y 7/7) **+ extensive hands-on user verification** — min-size floor holds, aspect snaps smoothly (no flicker), ✕→tray works with signpost, no scrollbar.
