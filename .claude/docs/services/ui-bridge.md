# `ui-bridge` — TauRPC Routers + AppError

## Responsibility
Owns the TauRPC bridge between Rust and webview. Hosts the cross-cutting envelope (`app_info` / `health` / `ready` / `get_settings` / `update_settings`). Defines and serializes the `AppError` enum at the IPC boundary. Houses TypeScript binding generation via `taurpc` derive macros.

## Key integrations

### Consumes from
- All other library crates expose router modules mounted by `pulse-app` via `ui-bridge`'s contract.
- `health` polls subsystem state from `ingest` / `buffer` / `viz` / `plugins` / `mcp-server`.
- `ready` polls duckdb_connection / ingest_mpsc_capacity_pct / broadcast_subscribers / plugins_loaded / mcp_server_enabled.

### Publishes to
- TauRPC procedure surface (per arch §Occupied Resources Tauri IPC routes):
  - Top-level: `app_info`, `health`, `ready`, `get_settings`, `update_settings`
  - Routers (mounted from sibling crates): `traces.*`, `metrics.*`, `logs.*`, `snapshot.*`, `plugins.*`, `mcp.*`, `workspace.*`, `telemetry.frontend.*`
- Auto-generated TypeScript bindings: `pulse-app/ui/src/bindings/*.d.ts` (per crate router).

### Dependencies
- `taurpc` derive macro (router-level type-safe IPC).
- `tauri` 2.x core.
- `serde` + `serde_json` for `AppError` serialization.
- `thiserror` 2.x for module-internal error enums.
- `anyhow` 1.x at the boundary (`main.rs` + non-IPC top-level).

## Internal conventions
- **`AppError` enum (binding):** `Validation { field, reason }`, `NotFound { resource }`, `Internal { message }`, `Plugin { plugin_id, message }`, `Storage { message }`, `Ingest { message }`. All variants `Serialize`-able.
- **Conversion at bridge:** `From<thiserror::Error> for AppError` impls strip stack traces, file paths, library versions, Rust struct names. NEVER serialize `anyhow::Error` directly.
- **`Internal { message }` carries a sanitized one-liner** suitable for surfacing in the UI; full error chain stays in internal log via `tracing-error` SpanTrace.
- **Procedure naming (binding):** top-level bare `snake_case` verbs OR `<router>.<verb>` dotted namespaces (per arch §Conventions Endpoint naming).
- **Procedure pin sync:** every TauRPC procedure REQUIRES its `EXPECTED_PROCEDURES` pin in `xtask/src/main.rs` (per-procedure `pulse-app/capabilities/` entries do NOT exist — one invoke handler, measured 2026-08-21). The `xtask capability-drift` check enforces — against the worktree AND, since 2026-08-30, the staged git-index bindings, plus the staged `capabilities/*.json` grants vs `staged_gate::EXPECTED_GRANTS`.
- **`#[tracing::instrument(skip_all, fields(traceparent = %tp))]`** on every router method.
- **Common response envelope (paginated lists):** `{ items: [], total: N, next_cursor: opaque-string-or-null }`.

## Service-specific gotchas
- **Tauri capability silent rejection** — a missing CORE API / `core:window:*` grant produces a hard-to-diagnose dead-affordance bug (procedures ride one invoke handler and need the `EXPECTED_PROCEDURES` pin instead). CI gates (`xtask capability-drift` + the folded-in staged-artifacts assertion) are the enforcement mechanism.
- **No `pub use` re-exports across crate boundaries** — only the explicit contract module of each crate exposes public surface.

## Entry points for modification
- **AppError enum:** `crates/ui-bridge/src/error.rs`
- **Cross-cutting envelope:** `crates/ui-bridge/src/{app_info,health,ready,settings}.rs`
- **TauRPC type generation:** `pulse-app/build.rs` invokes taurpc generation for each router crate
- **Capability JSON:** `pulse-app/capabilities/{pulse-default,pulse-tray,pulse-notification,pulse-updater,pulse-plugin-fs}.json`
- **Tests:** colocated per module

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(ui-bridge)'`
- **Integration:** `tauri::test::mock_builder()` + `get_ipc_response()` for in-process IPC tests; assert response JSON shape + AppError variant on error paths.
- **Capability drift CI gate:** `cargo xtask capability-drift` diffs procedures (worktree + staged git-index bindings) vs `EXPECTED_PROCEDURES` and asserts the staged capability grants vs `staged_gate::EXPECTED_GRANTS`; fails on mismatch.

## Local development
- **Regenerate TypeScript bindings:** `cargo build` triggers taurpc codegen as part of the build pipeline.
- **Inspect IPC traffic:** Tauri webview DevTools (Cmd+Opt+I macOS / F12 Windows / Ctrl+Shift+I Linux) shows all `invoke()` calls; backend log file `agent-latest.jsonl` shows handler spans.

## References
- `.andromeda/architecture.md` §Standard Contracts (Tauri IPC introspection — app_info, health, ready) + §Conventions Error response schema (Tauri IPC) + §Occupied Resources Tauri IPC routes
- `.andromeda/security-plan.md` §API Security (TauRPC capability authorization) + §Error Handling
- `.andromeda/test-plan.md` §5 (cross-module patterns covered)
- `.andromeda/obs-plan.md` §3 Trace context propagation (IPC envelope traceparent)
