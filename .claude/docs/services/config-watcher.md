# `config-watcher` — Configuration Hot-Reload Watcher

## Responsibility
Watches `<data_dir>/config.toml` and fans a validated `Settings` delta out to the running app (chunk #96 — Epoch 9 Foundation v0.2.0). Owns the filesystem watcher, the debounce, and the **classification** of each changed key into hot-applied / restart-required / silent. It does **NOT** own the re-application itself — the boot boundary in `pulse-app` subscribes and applies.

## Key integrations

### Consumes from
- `notify` 8.x filesystem events on `<data_dir>/config.toml`.
- `ui_bridge::contract::Settings` — the shape whose deltas are classified.

### Publishes to
- `tokio::sync::watch` fan-out at the `pulse-app` boot boundary.
- `pulse://stream/config-events` — **aggregate-only** (counts + bounded category labels; never a changed VALUE).
- Consumed by `pulse-app::config_router` → `config.reload` (force an immediate re-read + hot-apply) and `config.status` (last-reload timestamp + last error category).

### Dependencies
- Workspace-inherited: `notify` 8.x, `tokio` (sync + time), `serde`, `thiserror`, `tracing`. **No `tauri` dep** — see the runtime-agnostic rule below.

## Internal conventions
- **Runtime-agnostic background task (load-bearing).** `start_config_watcher(...)` returns `(ConfigWatchHandle, ConfigWatchTask)` — it does **NOT** call `tokio::spawn` internally. The caller spawns with ITS spawner: `tauri::async_runtime::spawn(task.run())` in production boot, `tokio::spawn(task.run())` in `#[tokio::test]`. A Tauri `.setup(...)` closure has no entered tokio runtime, so an internal `tokio::spawn` panics "there is no reactor running" (2026-06-04 pattern). The task owns the `RecommendedWatcher` + the mpsc receiver so the watcher outlives the call.
- **`partition_changed_keys`** classifies each delta into `hot_applied` (cadence / lifecycle knobs) · `restart_required` (ports, data dir, feature-shaped settings) · `silent` (no runtime effect). Adding a `Settings` field means classifying it here — an unclassified field defaults conservatively rather than silently hot-applying.
- **Prospective-only by design:** a hot-applied change affects work from that moment forward; it does NOT retroactively recompute the window already processed. Full retrospective re-classification is **opt-in** via `diagnostics.reevaluate_recent_window`.
- **`contract` module** is the ONLY `pub` surface.

## Service-specific gotchas
- **A malformed `config.toml` must not take the app down.** Parse failure ⇒ keep the last-good `Settings`, record an error *category* on `config.status`, and continue — never panic, never adopt a partial parse.
- **Editors fire multiple events per save** (write + rename + attribute touch). The debounce is what makes one save produce one reload; removing it produces reload storms.
- **Aggregate-only telemetry:** `pulse://stream/config-events` and the tracing events carry counts and bounded category labels only. A config VALUE may be a path or a port — never emit it (§Logging default-deny).
- **Settings shape constrains what is watchable:** `Settings` derives `PartialEq + Eq`, so a float field cannot be added — use the scaled-integer form (`*_x100: u32`) per the 2026-05-19 precedent, or the delta comparison will not compile.

## Entry points for modification
- **Watcher + task:** `crates/config-watcher/src/watcher.rs` (`start_config_watcher` / `ConfigWatchTask::run`)
- **Classification:** `crates/config-watcher/src/` (`partition_changed_keys`)
- **Public contract:** `crates/config-watcher/src/contract.rs`
- **Boot wiring:** `pulse-app/src/main.rs` (spawns via `tauri::async_runtime::spawn`)
- **Resolver:** `pulse-app/src/config_router.rs` (`config.reload` / `config.status`)
- **Retrospective path:** `pulse-app/src/reevaluation.rs` (`RecentWindowReevaluator` / `LiveReevaluator`)
- **Tests:** co-located unit tests + `pulse-app/tests/integration_config_hot_reload.rs`

## Capabilities
P-055 (hot reload) · P-056 (prospective + opt-in reevaluate).

## References
- `.andromeda/architecture.md` §Occupied Resources — `config.reload` / `config.status` · `pulse://stream/config-events`
- `.claude/docs/conventions.md` — configuration units (`*_seconds` naming, `TryFrom<u16>` ports)
- `.claude/rules/security.md` — the Settings-extension pattern and its derive constraints
