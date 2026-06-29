# Codebase Research — 2026-06-29-window-geometry-movable-shell

## Scope
- **Depth:** moderate · **Reads:** 9 (tauri.conf.json · Titlebar.tsx · Titlebar.test.tsx · window.rs · CompactWidget.tsx · contract.rs Settings · default.json · use-window-controls.ts · WindowControls.tsx) · **Globs/Greps:** 6 · **Code-graph queries:** 1

## Headline finding — the capability ALREADY exists; the *permission grant* does not
This is the 2026-06-01 "capability already exists" inverse + a partial **premise-correction** (2026-06-28). The intent's stated mechanism — "the custom titlebar IS a drag region so the window is movable" implying we must *build* a drag region — is **falsified**: the drag-region markup has existed since chunk #24. The REAL defect is a missing Tauri-2 capability permission.

- **`Titlebar.tsx`** (chunk #24): the `<header>` already carries `data-tauri-drag-region` (plus app-icon / title / grow spans); interactive buttons correctly excluded. **CompactWidget.tsx:55 renders `<Titlebar>`** — so the boot window (compact-widget) HAS the drag region on screen.
- **Why dragging "does nothing":** Tauri 2's `data-tauri-drag-region` invokes `getCurrentWindow().startDragging()` over IPC, which requires the `core:window:allow-start-dragging` permission. **`pulse-app/capabilities/default.json` grants only `["core:default", "updater:default"]`** (default.json:7-10). Tauri 2's `core:window:default` (bundled in `core:default`) is **read-only getters + `allow-internal-toggle-maximize` only** — it does NOT include `allow-start-dragging`. → the drag IPC is **silently rejected** (this project's negative-default model, CLAUDE.md §Critical Warnings). The official Tauri "Window Customization" guide requires exactly `core:window:allow-start-dragging` for `data-tauri-drag-region`.
- **Sibling discovery (same root cause):** the titlebar's three window-control buttons also call permission-gated APIs absent from `core:default`: `minimize()`→`allow-minimize`, `toggleMaximize()`→`allow-toggle-maximize`, `close()`→`allow-close` (use-window-controls.ts:17-19). So minimize / maximize / close-to-tray buttons are ALSO currently silent no-ops. (`close` overlaps P-063's "predictable close + honest tray"; minimize/maximize are orphaned — owned by no other chunk.)

## Files inspected
- `pulse-app/tauri.conf.json` (full) — two windows: `compact-widget` (480×270, `alwaysOnTop:true`, `decorations:false`, `visible:false`) + `main` (1280×800, `decorations:false`, `visible:false`). **Neither has `center` or an explicit position** → OS-default placement (top-left / cascade). Default sizes are deliberate, not "tiny" (480×270 is the design-spec quarter-screen glance widget).
- `pulse-app/src/window.rs` (full) — `apply_widget_settings()` snaps the compact-widget to `settings.widget_position` via `compute_snap_position` + sets `always_on_top`; on `current_monitor()`/`outer_size()` unavailable it **warns and SKIPS the snap**, leaving the window at the tauri.conf OS-default (top-left) — the likely "pinned top-left" path. `on_window_event` intercepts `CloseRequested`→`prevent_close()`+`handle_close_to_tray` (hides, not quits — the existing F3 close-to-tray plumbing). `show_compact_widget` shows the widget at boot. NO positioning applied to the `main` window anywhere.
- `crates/ui-bridge/src/contract.rs` Settings (70-244) — `widget_position: WidgetPosition` (**default `TopRight`**, contract.rs:78-79) + `always_on_top` (default `true`). **No free-form geometry fields** (no x/y/width/height) → "remembered" *free* position does not exist today; only the 4-corner snap enum. `Settings` derives `PartialEq + Eq` (the f32-blocks-Eq constraint per 2026-05-19 — any persisted geometry must be integer-typed).
- `pulse-app/ui/src/widget/CompactWidget.tsx` (full) — renders `<Titlebar onInvestigateClick=…/>` at top + `<main>` (constellation canvas + findings band). Confirms the drag region is present in the boot window.
- `pulse-app/capabilities/default.json` (full) — `windows:["compact-widget","main"]`, `permissions:["core:default","updater:default"]`. The single edit point for any window permission.
- `pulse-app/ui/src/components/{Titlebar,WindowControls}.tsx` + `hooks/use-window-controls.ts` — confirm the drag markup + the three window-op buttons and their permission-gated calls.
- `pulse-app/ui/src/components/Titlebar.test.tsx` — existing vitest asserts `data-tauri-drag-region` present + no tabindex + button ARIA. Extend here for any titlebar test.

## Graph impact (code-graph query → tree-query-2026-06-29-window-geometry-movable-shell.json)
- **`apply_widget_settings` / `show_compact_widget` / `on_window_event`** — each called from exactly ONE site: `pulse-app::main()` at main.rs:1012 / 1022 / 1024. Single caller, leaf wiring → hardening the snap fallback or centering is a low-blast-radius additive change. (DB regenerated: 6508 nodes / 32080 edges.)

## Patterns detected
- **Frameless custom titlebar** (`decorations:false` + `data-tauri-drag-region`) — frontend.md §Custom titlebar documents exactly this; the missing piece is the capability grant, not markup.
- **Boot-time window config via the setup closure** (window.rs `apply_widget_settings`, main.rs:1024) — the established place to apply geometry, not a new TauRPC procedure (arch §Cross-cutting; 2026-05-09 Settings-extension learning).
- **Corner-snap glance widget** (`compute_snap_position` + `WidgetPosition`, default TopRight) — the compact-widget is BY DESIGN a corner-snapped `alwaysOnTop` quarter-screen widget (chunk #30 + layout-templates §Compact widget), not a centered window.
- **Capability JSON is the single negative-default gate** — adding a `core:window:*` permission is a one-file edit to default.json; it is NOT a TauRPC procedure so `xtask capability-drift` is unaffected (2026-05-03), and not one of the 3 NEVER-widen capabilities so `capability-widening-check` is unaffected.

## Conventions to follow
- **Centering via static config** — `"center": true` on a window in tauri.conf.json centers natively at creation with NO JS permission required (vs computing a centered `set_position`, which would need `allow-set-position`). Prefer config-centering to keep the capability surface minimal.
- **Window permission rationale** — arch §Webview IPC capability policy: core APIs beyond enumerated TauRPC require "an explicit per-feature capability addition with a stated rationale." Adding `core:window:allow-start-dragging` (movability) is the stated F1 rationale; document it in default.json's `description` (the file already carries a per-chunk provenance log).
- **Boot smoke gate is mandatory** — this chunk touches `pulse-app/tauri.conf.json` + `pulse-app/capabilities/*.json` (+ maybe main.rs/window.rs), so per testing.md `boot-smoke-coverage` the plan MUST include `npx @tauri-apps/cli dev` (60s) — and the actual F1 fix (drag works) can only be confirmed by a runtime drag test, classified as the e2e/manual residual.
- **Settings integer-only if persisting geometry** — any "remembered" x/y must be integer (`i32`), never f32 (Settings `Eq` derive; 2026-05-19).

## New files to create
- (none expected) — the change is config + capability JSON + optionally a small window.rs hardening; tests extend existing files.

## Files to modify (pending P4 scope decision)
- `pulse-app/capabilities/default.json` — add `core:window:allow-start-dragging` (+ optionally `allow-minimize` / `allow-toggle-maximize` per the capability-breadth decision); extend the `description` provenance line.
- `pulse-app/tauri.conf.json` — add `"center": true` to the `main` window (and decide compact-widget per the position decision).
- `pulse-app/src/window.rs` — (optional, per position decision) harden `apply_widget_settings` so a monitor-unavailable snap falls back to a centered/sane position instead of leaving the OS top-left default; add a unit test.
- `pulse-app/src/window.rs` tests / `pulse-app/tests/` — unit coverage for any new fallback logic (pulse-app source `mod tests` do NOT run under `[lib] test=false` — put runnable tests in `pulse-app/tests/*.rs` per 2026-05-20).

## Open questions (→ resolve at P4 via AskUserQuestion)
1. **Position behavior** — center the `main` dashboard + keep/harden the compact-widget's TopRight corner-snap (rely on now-working drag to reposition) vs add "remembered" free-position persistence vs center both windows. The corner-snap is established design; "remembered" needs new Settings fields + `allow-set-position`.
2. **Capability breadth** — grant only `allow-start-dragging` (strict F1) vs also `allow-minimize` + `allow-toggle-maximize` (fix the orphaned, same-root-cause titlebar controls in-chunk; `close` left to P-063). The user's standing preference favors in-chunk fixes.
