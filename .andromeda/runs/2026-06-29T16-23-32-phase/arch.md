# arch extract

## Relevance
Relevant — window geometry and movability are foundational desktop-shell concerns directly governed by architecture.

## Constraints
1. **Tauri 2.x window API required** — per §Established Decisions [Platform]; window size, position, decorations, and drag-region affordances are Tauri-native (no fallback to custom JS handling).
2. **Configuration precedence** — if remembered geometry is added, it follows env vars > `~/.andromeda-pulse/config.toml` > built-in defaults per §Cross-cutting Patterns.
3. **Settings extension via existing IPC** — if "remembered geometry" is chosen, it extends the existing `Settings` struct and flows through `get_settings` / `update_settings` procedures per §Conventions (no new TauRPC namespace, per chunk scope).
4. **No new capability JSON** — the webview runs under `pulse:default` (existing), which permits enumerated TauRPC procedures but not new filesystem/dialog/shell APIs; window chrome does not require capability additions per §Occupied Resources.
5. **Boot-time application path** — geometry defaults are applied at `pulse-app/src/main.rs` setup closure via the established `apply_widget_settings()`-style pattern, not a new IPC procedure.
6. **Developer-tool visual density commitment** — window size and position expectations are subordinate to §Design Philosophy "dense, chart-first, low-chrome dashboard" and dark-mode default (inform but do not override the designer's sizing decision).
7. **Interactive region exclusion** — drag region must not swallow clicks on titlebar controls (buttons, menus, etc.) per chunk acceptance criteria.

## Patterns to follow
1. **Settings extension (chunk #96 established pattern)** — `crates/ui-bridge/src/contract.rs::Settings` struct + `get_settings`/`update_settings` TauRPC procedures handle layered config if remembered geometry is chosen; no new namespace.
2. **Boot-time application** — follow the synchronous window-config application at `pulse-app/src/main.rs` setup closure (not async IPC), matching the pattern used for other startup config (e.g., window theme, ingest port overrides from env).
3. **Tauri webview drag affordance** — `data-tauri-drag-region` attribute on DOM elements declares window-move handles; Tauri 2 automatically interprets this without requiring custom JavaScript.

## Anti-patterns to avoid
1. **New TauRPC procedure for geometry** — scope explicitly excludes a new IPC namespace; centered-only requires no Settings change, and remembered-geometry stays within the existing Settings contract.
2. **New environment variable** — window geometry is not an `ANDROMEDA_PULSE_*` env var (scope §Boundaries "expected no-op" for resource drift); deferred geometry sources (Wayland protocol negotiation, etc.) are out of scope.
3. **Overly broad drag region** — do not blanket-apply `data-tauri-drag-region` to the entire titlebar if it disables interactive controls; use surgical placement or JavaScript event delegation to preserve button/menu interactivity.

## Contract bindings
**Settings interface (ui-bridge ↔ pulse-app)** — if "remembered geometry" path is chosen, it becomes the second Settings consumer (after ui-bridge's own theme/telemetry settings); extends the locked Settings struct contract via §Conventions and chunk #96 pattern (get/set routers, no new procedure).

## Acceptance criteria contributions
- "(arch) Window opens at `tauri.conf.json`-configured default `width`/`height` (not tiny, not top-left-pinned); Tauri 2 native window config per §Stack and Technologies."
- "(arch) Window position is centered or remembered; if remembered, applied at boot via existing Settings contract (no new TauRPC procedure per scope §Boundaries)."
- "(arch) Custom titlebar declares Tauri drag region via `data-tauri-drag-region` attribute; interactive controls remain clickable (no region overflow)."
- "(arch) No new IPC procedure, environment variable, capability identifier, or workspace crate (scope §Boundaries — 'expected no-op' drift confirmation)."

## Relevant amendment history
(none) — No prior amendments touch window geometry or drag-region affordances. The Settings extension pattern established by 2026-06-04 amendment (chunk #96) is applicable if "remembered geometry" is chosen; no conflict or prior override.