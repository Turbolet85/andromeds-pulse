# Codebase Research — 2026-06-29-window-size-constraints

## Scope
- **Depth:** moderate · **Reads:** 5 (`tauri.conf.json`, `window.rs` full, `window_geometry.rs` full, `main.rs` setup section, tao-0.35 + tauri-2.11 registry source probe) · **Globs/Greps:** 6

## Files inspected
- `pulse-app/tauri.conf.json` (full) — the two declared windows. **`compact-widget`**: 480×270 (= 16:9 exactly, 1.7778), `resizable: true`, `decorations: false`, **no `minWidth`/`minHeight`, no aspect**. **`main`**: 1280×800 (= 16:10, 1.60), `center: true`, `resizable: true`, **no min-size**. This file is the static-config touchpoint for min-size.
- `pulse-app/src/window.rs` (full) — the window-lifecycle module (binary boundary). `on_window_event` (`window.rs:140`) matches `WindowEvent::CloseRequested` (→ hide-to-tray + signpost) and `WindowEvent::Moved` (→ geometry record), with a `_ => {}` catch-all arm — **the clean insertion point for a `WindowEvent::Resized` aspect-clamp**. `compute_snap_position` (`window.rs:212`) is a **pure `(monitor, window, position) → PhysicalPosition` fn with 10 colocated unit tests** — the exact template for a pure `clamp_to_aspect_bounds()` helper.
- `pulse-app/src/window_geometry.rs` (full) — P-061's Rust-owned `GeometryStore` (throttled `Moved` persistence). Confirms the established "Rust-owned window state, decoupled from the webview `Settings` contract, never logs coordinates" pattern that a size-constraint helper should follow.
- `pulse-app/src/main.rs` (setup closure ~1010-1050) — `on_window_event` is registered once via `.on_window_event(move |w,e| window::on_window_event(...))` with `Arc<Mutex<GeometryStore>>` + `data_dir` + a `signpost_shown` AtomicBool captured into the closure. A re-entrancy guard for the aspect-clamp would be a sibling captured `Arc<AtomicBool>` (or new param), mirroring the `signpost_shown` precedent. `apply_widget_settings` + `restore_main_window_position` run after window show.
- tao 0.35.0 + tauri 2.11.0 registry source — the **API-availability probe** (the scope's central open question).

## Graph impact (code-graph query)
- **Not queried (proportional-depth judgment).** The touched symbols (`window::on_window_event`, `window::apply_widget_settings`) are binary-boundary functions in `pulse-app` with a **single known call site** — the `main.rs` setup closure — already read directly. No cross-crate caller fan-out exists to map (the window module is `pulse-app`-internal; no library crate depends on it per the arch DAG). Targeted reads from the P2 extract signals fully cover the footprint; a `tree.db` `refs`/`calls` query would re-surface the same single caller. Recorded as a deliberate skip per `codebase-research.md` proportional-depth philosophy, not a cold-start.

## Patterns detected
- **Pure-helper + colocated unit tests** (`window.rs:212` `compute_snap_position` + 10 tests at `window.rs:437-495`): geometry math is extracted into a pure fn taking primitive tuples, exhaustively unit-tested without a live window. An aspect-clamp must follow this shape — `fn clamp_to_aspect_bounds(size, bounds) -> Size` unit-tested over the band edges.
- **Event-handler dispatch with threaded state** (`window.rs:140`): `on_window_event` already fans `WindowEvent` arms with `&Mutex<GeometryStore>` + `&AtomicBool` state; a `Resized` arm + a re-entrancy-guard atomic slot in beside `signpost_shown`.
- **Runtime window-API application at boot** (`window.rs:237` `apply_widget_settings`): the established place to call `window.set_min_size(Some(..))` at boot if the runtime path (vs static `tauri.conf.json`) is chosen; best-effort with a `warn` on failure, never aborting boot.
- **Coordinate/​dimension no-log discipline** (`window_geometry.rs:1-6` + `security` extract): geometry values are never logged; a size-clamp emitting an obs span must stay aggregate/label-only (no raw w/h tuples), or reuse the existing `ui.layout.transition` span (obs extract).

## Conventions to follow
- **No new TauRPC / capability / broadcast** — window ops are Rust/config-side; per P-061's finding, Rust-side window mutation needs no `core:window:*` grant (capabilities gate webview JS only). `cargo xtask capability-drift` + `capability-widening-check` are expected no-ops.
- **Bounded window-label enumeration** (`window.rs:80` `sanitize_window_label`): any new per-window logic keys off the two `MAIN_WINDOW_LABEL` / `COMPACT_WIDGET_LABEL` constants.
- **Boot-smoke gate applies** (tests extract): the chunk touches `tauri.conf.json` + `main.rs`/`window.rs` (boot-path) → `npx @tauri-apps/cli dev` 60s smoke is required in `## Test Commands` alongside the standard gate baseline.

## API-availability findings (the scope's open question — RESOLVED)
- **Min-size: SUPPORTED, OS-enforced.** `tauri::WebviewWindow::set_min_size(Option<Size>)` exists (`tauri-2.11.0/src/webview/webview_window.rs:2225`); tao `set_min_inner_size` backs it (`tao-0.35.0/src/window.rs:790`). Static `minWidth`/`minHeight` in `tauri.conf.json` is the declarative equivalent. **Min-size is enforced by the window manager** — the user cannot drag below it — so it needs **no event-loop clamp** and is robustly verifiable (config assertion + the OS refuses smaller).
- **Aspect-ratio: NO native API.** tao 0.35 has **no** window aspect-ratio lock (the only `aspect` hits are `DVASPECT_CONTENT` OLE constants in the Windows drag-drop handler — unrelated); winit/tao have never shipped one. → aspect must be enforced by a **`WindowEvent::Resized(size)` clamp** in `on_window_event`: recompute the size to keep `w/h` within a configured aspect band and call `window.set_size(..)` (`webview_window.rs:2220`), guarded by a re-entrancy `AtomicBool` (the `set_size` itself emits a `Resized`). The **live** "aspect stays within bound during a real drag-resize" assertion needs headful tauri-driver → **CARRY to P-076** (exactly the P-061 headful-drag-delta precedent: pure-helper + config proof land in-chunk; the live delta carries to the Epoch-4 e2e suite).

## New files to create
- (none expected) — all logic lands in the existing `pulse-app/src/window.rs` + `pulse-app/tauri.conf.json`; tests colocate in `window.rs` `mod tests` IF pure-helper-only, OR move to `pulse-app/tests/unit_window_constraints.rs` per the 2026-05-20 `[lib] test = false` discipline if they need the lib surface (the pure helper is testable in `mod tests` since it takes primitives, like `compute_snap_position`).

## Files to modify
- `pulse-app/tauri.conf.json` — add `minWidth`/`minHeight` to `compact-widget` (and to `main` if Q2 → both-windows).
- `pulse-app/src/window.rs` — add a pure `clamp_to_aspect_bounds()` helper + unit tests; add a `WindowEvent::Resized` arm to `on_window_event` invoking it with a re-entrancy guard (IF Q1 → aspect-in-chunk).
- `pulse-app/src/main.rs` — capture the re-entrancy-guard `Arc<AtomicBool>` into the `on_window_event` closure (mirrors `signpost_shown`); optionally call `set_min_size` at boot if the runtime path is chosen over static config.

## Open questions (→ resolve at P4 via AskUserQuestion)
1. **Aspect-ratio mechanism & in-chunk depth** — no native API exists; deliver aspect via a `Resized`-clamp now (pure helper unit-tested, live headful CARRIED to P-076) vs min-size-only-now/defer-aspect vs hard 16:9 lock. (Leaning: clamp-with-bound now — delivers both P-062 constraints, matches P-061 precedent.)
2. **Constraint scope** — widget-only (literal intent F2) vs widget + a sane `main`-window min-size (cheap adjacent fix; aligns with the user's standing in-chunk-fix preference). (Leaning: fold in the main min-size, aspect on the widget only.)
