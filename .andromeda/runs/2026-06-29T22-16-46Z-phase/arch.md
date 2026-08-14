# arch extract

## Relevance — relevant
Window size constraints are Tauri-level shell behavior affecting the visualization surface, within the desktop-application scope that architecture.md explicitly commits to.

## Constraints
- **Stack & Technologies** (§Stack and Technologies) — Tauri 2.x window configuration and (if available in the pinned version) native aspect-ratio constraint API; fallback is resize-event clamping via the same Tauri event loop P-061 established.
- **Design Philosophy** (§Design Philosophy) — the "WebGPU canvas + tray icon + OS-notification surfaces are the visual touchpoints arch commits to"; window sizing constrains the canvas layout, so constraints must preserve the "dense, chart-first, low-chrome dashboard" density intent.
- **Established Decisions** (§Established Decisions) — Tauri 2.x is the locked desktop shell; no native constraint APIs → fallback to resize-event clamp in the event loop (mirrors P-061's `on_window_event` Moved handler pattern).
- **Infrastructure Patterns** (§Infrastructure Patterns) — single-process on user's machine; no Docker/K8s; window lifecycle is Rust-owned (`pulse-app` binary crate only).
- **Inherited Defaults** (§Inherited Defaults) — Tauri 2.x IPC bridge and webview integration; TauRPC procedures and Tauri capabilities are reserved surfaces (no new namespace expected per scope boundaries).

## Patterns to follow
- **Configuration hierarchy** (§Cross-cutting Patterns) — if constraints become user-tunable, follow precedence: env vars > `~/.andromeda-pulse/config.toml` > built-in defaults. Default is a fixed sane constant (no tuning expected).
- **Window-setup substrate from P-061** — P-061 established `pulse-app/src/window_geometry.rs` helper and boot-time window-event handlers in `main.rs`; reuse this pattern for constraint enforcement at startup (static `tauri.conf.json` `minWidth`/`minHeight`) and/or at runtime (resize-event clamp if native API unavailable).
- **Rust-owned, capability-clean** — window operations have no IPC surface and require no new Tauri capability (per scope), so no capability JSON amendment needed.

## Anti-patterns to avoid
- **Webview JS-driven constraint enforcement** — constraints are Rust/config-side only; webview is the consumer, not the enforcer.
- **Don't create new TauRPC namespace or procedure** — window constraints are static configuration or event-loop clamps, not a new query/command surface.
- **Don't create new capability JSON** — per scope boundaries, no new `core:window:*` capability grant is required.

## Contract bindings
(none) — window sizing is a self-contained shell concern. The visualization (WebGPU canvas in the webview) is the beneficiary of constrained layout, but no cross-module IPC or new contract boundary is introduced.

## Acceptance criteria contributions
- "(arch) Minimum-size and aspect-ratio constraints specified via Tauri 2.x static config (`tauri.conf.json` `minWidth` / `minHeight`) or native runtime API (per §Established Decisions Tauri 2.x pinned version availability); fallback is resize-event clamp in `pulse-app/src/main.rs` event loop."
- "(arch) No new TauRPC procedure, Tauri capability, or environment variable (§Occupied Resources unchanged; constraint values are fixed constants or sourced from existing config precedence chain)."
- "(arch) Constraint enforcement reuses window-setup infrastructure from P-061 (window_geometry.rs helper, main.rs event handler registration)."

## Relevant amendment history
- **2026-06-29 (P-061, window-geometry-movable-shell)**: Added `window-geometry.json` filesystem location for remembered window positions; established `pulse-app/src/window_geometry.rs` helper and boot-time/event-loop window-setup pattern in `main.rs`. **P-062 (this chunk) reuses this pattern** for size-constraint enforcement at boot and on resize. See §Occupied Resources §Filesystem locations + §Infrastructure Patterns window-lifecycle entry.
