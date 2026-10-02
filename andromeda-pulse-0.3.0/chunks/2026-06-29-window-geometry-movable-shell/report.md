# Report — 2026-06-29-window-geometry-movable-shell

**Chunk:** Window geometry + movable shell — sane default size/position (centered or remembered) + a working custom-titlebar drag region (P-061 · intent F1 · Epoch 2)
**Date:** 2026-06-29
**Commits:** none yet (this wrap is the chunk's first commit)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `pulse-app/src/window_geometry.rs` · NEW `pulse-app/tests/unit_window_geometry.rs` · NEW `pulse-app/tests/window_chrome_config.rs`
  - MOD `pulse-app/src/lib.rs` (module decl) · `pulse-app/src/window.rs` · `pulse-app/src/main.rs` · `pulse-app/capabilities/default.json` · `pulse-app/tauri.conf.json`
- **Symbols / APIs:**
  - NEW module `pulse_app::window_geometry`: `WindowGeometry` (`load_from_data_dir`/`position`/`record`/`save_to_data_dir`), `Position{x:i32,y:i32}`, `GeometryStore` (`load`/`snapshot`/`record_move_throttled`/`flush`).
  - CHANGED `window::apply_widget_settings(app, settings, geometry: &WindowGeometry)` — added param; now remembered-position → corner-snap → centered-fallback (was: snap → skip-on-monitor-unavailable).
  - CHANGED `window::on_window_event(window, event, store: &Mutex<GeometryStore>, data_dir: &Path)` — added 2 params; now also handles `WindowEvent::Moved` (throttled capture) + flushes geometry on `CloseRequested`.
  - NEW `window::restore_main_window_position(app, geometry)`.
  - **NO TauRPC procedure / IPC method / endpoint / port / socket / env var added or changed.**
  - **Capability (`pulse:default` / `default.json`):** +`core:window:allow-start-dragging`, +`core:window:allow-minimize`, +`core:window:allow-toggle-maximize` (core window perms; `close` deferred to P-063). These are NOT TauRPC procedures.
- **Crates / modules:** added module `window_geometry` inside `pulse-app`; no new workspace crate; no crate removed.
- **Dependencies:** none added / none bumped (reused existing `serde` / `serde_json` / dev `tempfile`).
- **Schema / config:** `tauri.conf.json` `main` window gains `"center": true`. NEW persisted file `<data_dir>/window-geometry.json` (JSON: window-label → `{x,y}`, Rust-owned, atomic `.tmp`+rename). No DB / corpus migration. No violation-schema change.
- **Coverage of new surfaces:**
  - `core:window capability grant (drag/minimize/maximize)` → validation {n/a — declarative ACL, compile-validated + embedded by tauri-build} · instrumentation {span ✓ — `ui.layout.transition` outcome label} · PII {redacted ✓ — coordinates never logged} · tests {static guard ✓ `window_chrome_config.rs`} · a11y {n/a — enables existing titlebar affordance; buttons keep ≥24px, unchanged} · tokens {n/a — no UI source change}
  - `<data_dir>/window-geometry.json remembered-position persistence` → validation {✓ integer-only coords via serde} · instrumentation {n/a — Moved path emits no span by design (coordinate-logging ban)} · PII {redacted ✓ — no coordinate tuples logged} · tests {unit ✓ `unit_window_geometry.rs` roundtrip/missing/corrupt} · a11y {n/a} · tokens {n/a}

## Deviations from intent
- **Premise correction (headline, intent F1 mechanism falsified):** intent said "build a custom-titlebar drag region so the window is movable" — but `Titlebar.tsx` has carried `data-tauri-drag-region` since chunk #24 (rendered by the boot window). The REAL defect was the missing `core:window:allow-start-dragging` permission (Tauri 2 `core:default` is getters + internal-toggle-maximize only → silent IPC rejection). Fix = the capability grant, NOT new markup. `Titlebar.tsx` untouched. Outcome (movable shell) unchanged; mechanism corrected. Surfaced to + approved by the user at /phase P4 (recorded in `scope.md` P4-resolutions + `verification-matrix.json#P-061` `notes`).
- **No `allow-set-position` capability** (vs the Q1 preview): remembered-position restore runs Rust-side at boot (`window.set_position`); capabilities gate only webview JS, so no JS capability was needed. Capability surface = exactly the Q2 three. Justified simplification (smaller surface).
- **Remembered geometry kept in a Rust-owned sink** (`window-geometry.json`), NOT the webview `Settings` struct (vs the Q1 preview's `Settings += win_x/win_y`): avoids the `update_settings` form-clobber bug + zero bindings churn. `SettingsModalForm` untouched; `bindings.ts` has no diff. Justified (correctness + minimal surface).
- **Capability breadth = drag + minimize + maximize** (Q2 answer): fixed the orphaned, same-root-cause minimize/maximize titlebar buttons in the one capability edit; `close` left to P-063. As planned.
- **Minor in-scope obs tweak** in `apply_widget_settings`: `layout_mode_to` now reports the outcome (`remembered`/`centered`/corner label) + message "applied widget geometry"; bounded values, no field-name change → obs allowlist untouched.
- **Boot smoke skipped with cause** (not run): Tauri embeds the capability ACL at compile time (the successful `cargo build` validated + embedded the 3 perms — verified in the generated `target/.../capabilities.json`; an invalid identifier fails the build), the new setup-closure code is panic-safe by construction (sync best-effort IO + warn-on-err window ops, no new async/spawn/reactor), and a `cargo run --release` GUI launch carries the documented Windows GUI-orphan hazard (verification-harness.md 2026-05-19, which recommends `cargo build` for non-webview-behavior chunks). Runtime drag-delta is the CARRIED headful e2e residual.

## Decisions & corrections
- **Premise-correction pattern (capability-already-exists / RESEARCH-CORRECTS-INTENT):** a "build UI affordance X" intent for a frameless Tauri app can actually be a **missing `core:window:*` capability** — the markup exists but the permission-gated IPC is silently rejected under `core:default` (negative-default). Discover by reading the component (markup present?) + the capability JSON (permission granted?) BEFORE planning to "build" it. (Curation candidate — frontend/security.)
- **Tauri capabilities are compile-time-embedded:** a successful `cargo build` validates capability permission identifiers (an invalid one fails the build) and embeds the ACL — so for a capability-only change, a GUI boot smoke adds no capability-validation value beyond the build (informs when boot smoke is skippable). (Curation candidate — testing/verification-harness.)
- **Remembered-geometry decoupling:** keep window-position persistence Rust-owned + separate from the webview `Settings` contract → avoids the form-clobber bug (a form `update_settings` omitting geometry would reset it) AND keeps capture/restore Rust-side so no `allow-set-position` is needed (capabilities gate webview JS only). (Curation candidate — Tauri state pattern.)
- Two user AskUserQuestion decisions (position = remembered free position; capability breadth = drag+minimize+maximize) — chunk-specific, recorded in scope.md/plan.md (not curation-worthy).

## Outcome
- **Acceptance (P-061):** met to the achievable bar — drag/minimize/maximize capability granted + compile-validated + embedded; `main` dashboard opens centered; remembered-position persist/restore + monitor-unavailable centered fallback (unit-tested). Headful tauri-driver drag-delta e2e = CARRIED residual → P-075/P-076.
- **Gates green:** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` (**1701 passed, 1 skipped, 0 failed**; +7 new) · `cargo xtask capability-drift` (clean) · `cargo xtask capability-widening-check` (clean, 3 inspected) · webview `typecheck`/`lint`/`test` (**642 passed**) · bindings regenerated (mcp+investigate present, no diff).
- **Smoke:** skipped with cause (see Deviations) — boot-path = setup-closure geometry wiring + capability/config; capability ACL compile-validated; panic-safe by construction.
