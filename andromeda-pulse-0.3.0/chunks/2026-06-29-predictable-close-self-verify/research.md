# Codebase Research — 2026-06-29-predictable-close-self-verify

## Scope
- **Depth:** moderate · **Reads:** 9 (tray.rs, window.rs, 3× capability JSON, main.rs §builder, window_chrome_config.rs test, package.json, agent-run.sh) · **Globs/Greps:** 4 · **Graph:** 1 query (close handlers)

## Files inspected
- `pulse-app/src/window.rs` (full) — **the close behavior already exists.** `on_window_event` (l.100-124) matches `WindowEvent::CloseRequested { api, .. }` → `api.prevent_close()` + geometry flush + `handle_close_to_tray(window)` (l.88-98) → `window.hide()`. Applies to BOTH windows (no label filter). This is chunk #24's hide-to-tray. No signpost emitted. Also holds `restore_main_window_position` (main starts hidden+centered).
- `pulse-app/src/tray.rs` (full) — tray icon + flat OS-native menu (Open / summary(disabled) / Snapshot / [MCP toggle] / Open Settings / Quit). `MENU_ID_QUIT` → `app.exit(0)` (l.258-263) is the canonical terminate path. Summary line is a STATIC placeholder ("Ingest: 0 sp/s…", l.167-172) — live refresh is an explicit follow-up, NOT in F3.
- `pulse-app/src/main.rs` (l.985-1095) — builder wiring: `.plugin(tauri_plugin_notification::init())` (l.1020) IS present; `.on_window_event(|w,e| window::on_window_event(...))` (l.1021-1023) registers the close handler globally; tray set up in `.setup` (l.1041-1042). No close signpost anywhere.
- `pulse-app/capabilities/default.json` — P-061 granted `core:window:allow-start-dragging|allow-minimize|allow-toggle-maximize` + `core:default` + `updater:default`. The description note explicitly says "close stays tray-intercepted, owned by P-063." `allow-close` is part of `core:window:default` (read note) — close already fires; behavior is ours.
- `pulse-app/capabilities/notification.json` — `pulse:notification` = outbound-emit-only (`allow-notify`/`allow-show`/`allow-is-permission-granted`/`allow-request-permission`); NEVER input handlers. Ready for a close signpost; must NOT widen (capability-widening-check).
- `pulse-app/capabilities/tray.json` — `pulse:tray` permissions EMPTY (outbound-only by construction).
- `crates/ui-bridge/src/contract.rs` (Settings, grep) — `Settings` has `notifications_enabled: bool` (default true, l.119-120) + `always_on_top` + `widget_position` + `mcp_server_enabled`. **No `close_to_tray` field** — a toggle would be a new field (cheap Settings-extension pattern, but Eq-compatible bool is fine).
- `pulse-app/tests/window_chrome_config.rs` (full) — P-061's test PATTERN to mirror: static JSON-manifest assertions (`capabilities/default.json` perms present; `tauri.conf.json` main window `center:true`). P-063 reuses this shape for any capability/config assertions.
- `scripts/agent-run.sh` (full) — the existing 5-command harness: `boot` = `cargo run --bin pulse-app --release` in BACKGROUND with isolated `ANDROMEDA_PULSE_DATA_DIR`, polls `cargo xtask harness:status` (which uses `tauri::test::mock_builder` — NOT the live app's IPC) up to 10s; `cleanup` = SIGTERM the real PID + verify ports released + rm tempdir. So the harness ALREADY boots+kills the real release binary.
- `pulse-app/ui/package.json` — `test:a11y` = `verify:contrast` (colorjs.io) + Playwright axe + Lighthouse + pa11y + aggregator + regression-detector. This IS the existing a11y/contrast harness the self-verify composes.

## Graph impact (from the code-graph query)
- **`handle_close_to_tray`** — 1 caller: `window/on_window_event` @ `window.rs:111`. Single, self-contained.
- **`on_window_event`** — 1 caller: `main().` @ `main.rs:1021`. The close path is one function with one registration site → minimal blast radius for a behavior change.

## Patterns detected
- **Hide-to-tray already implemented** (`window.rs:88-124`): `prevent_close()` + `hide()`. The chunk ADDS the signpost; it does not build close from scratch.
- **Bounded-label obs discipline** (`window.rs:78-84`, `tray.rs:305-315`): `sanitize_window_label` / `sanitize_menu_id` collapse to bounded enums; the close/signpost spans must follow (aggregate-only, no per-service ids per obs 2026-05-17).
- **Static JSON-manifest tests** (`window_chrome_config.rs`): capability/config assertions read the manifest + assert keys — the P-063 test pattern for any capability/config delta.
- **xtask subcommand home** (verification-harness.md): `capability-drift` / `capability-widening-check` / `test:a11y` / `harness:status` / `verify:capability-matrix` all live in `xtask` — a new `self-verify` verb fits there WITHOUT touching the byte-bound 5-command `agent-run` contract.

## Conventions to follow
- **Close signpost = OS notification, first-close only, respects `notifications_enabled`** (arch §OS notification policy "no other subsystem emits without an explicit decision"; design §desktop-native OS-native-only; `contract.rs:119` opt-out).
- **No new TauRPC procedure** (arch extract): close is a window event, not a bridged RPC → `xtask capability-drift` unaffected. `emit_taurpc_bindings`/bindings.ts regen NOT triggered by this chunk's Rust (no router change).
- **pulse-app tests live in `pulse-app/tests/*.rs`**, never source `mod tests` (testing.md 2026-05-20 `[lib] test=false`).
- **Boot-smoke** is capability/config + panic-safe-by-construction here → `cargo build -p pulse-app` validates the embedded ACL; GUI boot-smoke is skippable-with-cause (verification-harness.md 2026-06-29) — and the self-verify harness IS the durable replacement.

## New files to create (plan-confirmed)
- `xtask/src/…` self-verify subcommand (mechanism per P4 Q2) — orchestrates boot→assert→a11y→clean-quit.
- `pulse-app/tests/unit_*_close*.rs` (+ a capability/config guard test mirroring `window_chrome_config.rs`) for the signpost + close logic.

## Files to modify (plan-confirmed)
- `pulse-app/src/window.rs` — emit the first-close signpost from `handle_close_to_tray` (or a sibling), idempotent per session, gated on `notifications_enabled`.
- possibly `pulse-app/src/main.rs` — thread a "first-close" flag / notification handle if the signpost needs app state.
- possibly `pulse-app/capabilities/default.json` — add `pulse:notification`-scoped grant to the webview windows IF the notification is emitted from a context needing it (it is emitted Rust-side via the plugin, so likely no default.json change — confirm at implement).

## Open questions (→ P4 AskUserQuestion)
1. **Close behavior** — keep hide-to-tray (arch default) + add a first-close signpost (premise-correction: close already hides; the gap is the signpost) · vs a `Settings.close_to_tray` toggle · vs quit-on-close (needs arch amendment).
2. **Self-verify harness mechanism/extent** — `xtask self-verify` orchestrator (boot real binary + log/geometry shell-health asserts + `npm test:a11y` + clean-quit no-orphan) · vs tauri-driver headful WebDriver spec · vs extend the byte-bound 5-command harness.
3. (folded into Q1) signpost form — OS notification (recommended) vs persistent tray tooltip vs both.
