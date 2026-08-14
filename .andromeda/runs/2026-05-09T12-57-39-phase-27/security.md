# security extract — phase-27

## Chunk relevance

- **route#30 "Compact widget shell"** — Partially in-domain. This chunk introduces a new Tauri window (native window configuration, always-on-top policy, snap-to-edge persistence) and a custom titlebar. Security concerns apply around: (1) Tauri capability gating for the new window surface, (2) window-level IPC authorization since the compact widget is a distinct webview context, (3) close-to-tray process lifecycle policy (no silent termination that could orphan sockets). No network surfaces introduced; no plugin host; no storage writes; no OTLP path touched. Security footprint is moderate-low but non-trivial due to the new Tauri window + capability scoping questions.

## Constraints

- The compact widget shell is a new Tauri webview window. If it shares the `pulse:default` capability, it inherits the full TauRPC procedure surface — confirm this is intentional and documented. If it runs under a separate capability, that capability must be explicitly defined in `pulse-app/capabilities/` with stated rationale (per security plan §Tauri Capability Gating).
- Any new TauRPC procedures registered for the widget window (e.g., geometry persistence, always-on-top toggle) require BOTH router registration AND a capability JSON entry. The `xtask capability-drift` check (chunk #27) enforces router ↔ capability sync and must exit 0 after this chunk lands (per security plan §Tauri Capability Gating + §Session Additions 2026-05-03).
- NEVER grant `fs`, `shell`, `dialog`, or `http` Tauri core APIs to the widget window's capability without explicit per-feature rationale (per security plan §Tauri Capability Gating).
- If snap-to-edge position is persisted to disk (`config.toml` or per-display sidecar file), the write path MUST be resolved via `strict-path` and confirmed to live under the resolved data dir (`ANDROMEDA_PULSE_DATA_DIR` or platform default). CWE-22 applies even to geometry files (per security plan §Input Validation §Path env vars).
- Close-to-minimize-to-tray policy means the Tokio runtime, OTLP receivers (`:4317`/`:4318`), and broadcast sockets remain live after widget close — must not inadvertently widen the loopback-only bind constraint at runtime (per security plan §Loopback-only network surfaces + §Tauri Capability Gating).
- `tracing` instrumentation added for window geometry events, always-on-top toggles, or snap-to-edge state changes MUST NOT log raw coordinate or display-bounds values that could leak multi-monitor topology (per security plan §Self-observation + §Logging & Redaction).

## Patterns к follow

- **Capability-first window design**: decide capability assignment for the widget window before writing any IPC-invoking frontend code; xtask drift check catches router mismatches but cannot catch a window receiving the wrong capability bundle (per security plan §Tauri Capability Gating).
- **`strict-path` for geometry persistence**: follow the same canonicalize-then-assert-under-data-dir pattern used for plugin paths and snapshot paths (per security plan §Input Validation §Path env vars).
- **`tracing` event shape**: window lifecycle events use structured `tracing::info!(event = "widget.window.opened", ...)` — no raw coordinate values in payload (per security plan §Self-observation).
- **Existing capability JSON pattern**: `pulse-app/capabilities/*.json` `permissions` array carries `core:default` + plugin-level perms — NOT per-procedure strings; the widget window capability follows this same router-level shape (per security rules.md §Session Additions 2026-05-03).

## Anti-patterns к avoid

- Do NOT bind a new tray-interop IPC surface to `pulse:default` — `pulse:notification` and `pulse:tray` are outbound-emit-only; input-event handlers (tray menu actions, notification callbacks) require a separately-named capability (per security plan §Tauri Capability Gating).
- Do NOT widen `pulse:tray` to include write or execute permissions — scope must remain outbound-emit only; static-analysis enforcement is documented as a gap (no automated gate yet) so author discipline is the only enforcement (per security plan §Tauri Capability Gating §static analysis gap note).
- Do NOT log display geometry values — monitor count, display bounds, window coordinates are environment topology data; log only event names and boolean state changes (per security plan §Logging & Redaction).
- Do NOT use `format!()` SQL for any geometry storage if DuckDB is the backing store (per security plan §Input Validation DuckDB anchor).

## Contract bindings

- **security ↔ tests/CI**: any new TauRPC procedures for the widget shell must appear in both router registration and capability JSON before `cargo xtask capability-drift` exits 0 — hard CI gate binding security to tests/CI domain (per security plan §Tauri Capability Gating + xtask chunk #27).
- **security ↔ arch**: new TauRPC namespaces must be registered in arch §Occupied Resources via `/andromeda-scope-arch` BEFORE the chunk merges — xtask catches router/capability sync but does NOT enforce arch registry sync (per security rules.md §Session Additions 2026-05-09 + arch.md §Architecture Registry Updates).
- **security ↔ buffer**: any geometry persistence write path inherits the `buffer` crate's path-canonicalization pattern as the reference implementation (per security plan §Input Validation §Path env vars).

## Acceptance criteria contributions

- (security) `cargo xtask capability-drift` exits 0 after chunk #30 merges — all TauRPC procedures introduced for the compact widget shell appear in `pulse-app/capabilities/*.json` with no extras or missing entries (per security plan §Tauri Capability Gating).
- (security) Widget window capability assignment documented in `pulse-app/capabilities/*.json` — explicitly lists only the authorized procedure surface; any deviation from `pulse:default` sharing carries a rationale comment (per security plan §Tauri Capability Gating).
- (security) Geometry persistence path safety — if implemented, write path canonicalized via `strict-path` and asserted under resolved data dir; `grep` for `std::fs::write` / `tokio::fs::write` confirms no raw path is used (per security plan §Input Validation §Path env vars).
- (security) `tracing` events emitted by window open/close/move/resize handlers contain only event names + boolean state (e.g., `always_on_top = true`); no raw coordinate / display-bounds values in structured log output (per security plan §Self-observation + §Logging & Redaction).
- (security) No new `TcpListener` or socket bind introduced; OTLP receivers remain `127.0.0.1`-only after this chunk lands (per security plan §Loopback-only network surfaces).
- (security) Close-to-tray lifecycle preserved — Tokio runtime + OTLP receivers remain live after widget close; no `AppHandle::exit()` / `process::exit()` triggered by window close handler (per security plan §Tauri Capability Gating + arch §Tray icon policy).
