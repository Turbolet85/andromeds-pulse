# security extract — phase-35

## Chunk relevance

Chunk #38 (Settings modal form) is within security domain scope. Key security-relevant surfaces:
- Settings form persists user-controlled values via `update_settings` TauRPC procedure — input validation at bridge boundary
- MCP-toggle field directly controls the double-gate runtime env var behavior
- Retention-seconds field is a port-range-adjacent numeric value requiring `TryFrom`-style bounds validation
- No new TauRPC namespaces introduced (extending existing `Settings` struct) — avoids capability-drift triple binding per `.claude/rules/security.md` Session Additions 2026-05-09

## Constraints

**Input validation at TauRPC bridge boundary** (per security plan §Input Validation + §TauRPC Bridge):
- `update_settings` procedure argument must pass through `serde` deserialization on every field; no raw string passthrough
- `retention_seconds` field MUST enforce bounded range via `TryFrom`-equivalent validation (not just `u32` deserialization) — same pattern as port validation (`TryFrom<u16>`)
- Theme picker and widget-position fields MUST use `serde`-friendly enum types, not raw strings — prevents invalid-variant injection

**AppError sanitization** (per security plan §Error Sanitization at IPC Boundaries):
- `update_settings` returns `Result<T, AppError>` — validation failures MUST surface as `AppError::Validation { field, reason }`, not raw Rust errors or panic
- No stack traces, file paths, library versions, or Rust struct names in `AppError::Internal { message }` if settings persistence fails

**MCP-toggle field security constraint** (per security plan §MCP Feature Double-Gate):
- The MCP-toggle in the Settings form controls the runtime gate (`ANDROMEDA_PULSE_MCP_ENABLED` equivalent stored in Settings); the compile-time `--features mcp-server` gate is independent and MUST NOT be bypassed by any form persistence path
- Toggling MCP on via Settings MUST NOT start the MCP stdio surface if binary was built without `--features mcp-server` — the graceful-degrade `warn` path must be preserved

**No new capability JSON entries required** (per `.claude/rules/security.md` Session Additions 2026-05-09, second entry):
- Chunk #38 extends `Settings` struct and flows through `get_settings`/`update_settings` — zero new `pulse-app/capabilities/` JSON entries needed; no `/andromeda-scope-arch` run required for this chunk's IPC surface

## Patterns to follow

**Serde-enum validation pattern** (per security plan §Input Validation):
- All enum-typed Settings fields (theme, widget-position, snapshot-preset, snapshot-format) follow existing `serde`-friendly enum pattern already established in `crates/ui-bridge/src/contract.rs`; new fields added in chunk #38 MUST match that pattern

**AppError Validation variant usage** (per security plan §Error Sanitization):
- Out-of-range `retention_seconds` or invalid enum deserialization MUST map to `AppError::Validation { field: "retention_seconds", reason: "..." }` — follow the existing `From` impl pattern in `ui-bridge`

**Capability-drift coupling discipline** (per `.claude/rules/security.md` Session Additions 2026-05-10):
- If `pulse-app/src/main.rs` is touched to wire tray "Open Settings" → modal launch, verify `cargo xtask capability-drift` exits 0 — no new procedures are introduced but the check validates the existing surface is intact

## Anti-patterns to avoid

**Raw string settings fields** (per security plan §Input Validation):
- NEVER add a `theme: String` or `widget_position: String` field to `Settings` — MUST be typed enums; raw strings bypass serde-enum validation and allow invalid variant injection

**Format-string validation bypass** (per security plan §DuckDB SQL / general input validation):
- NEVER validate `retention_seconds` bounds via a `format!`-interpolated check or string comparison; use integer comparison after typed deserialization

**Leaking AppError internals** (per security plan §Error Sanitization):
- NEVER return `anyhow::Error` or `thiserror`-derived module errors directly from `update_settings` — convert at bridge via `From` impls before crossing IPC boundary

**MCP single-gate regression** (per security plan §MCP Feature Double-Gate):
- NEVER wire the MCP-toggle to bypass the compile-time feature gate — the Settings toggle is the runtime gate only; double-gate must be preserved

**Logging settings values verbatim** (per security plan §Logging & Redaction):
- NEVER log raw settings field values at `debug`/`trace` level — plugin-manager paths and snapshot-preset values could contain user path data; log field names + types only

## Contract bindings

**AppError ↔ TauRPC bridge** (security plan §Error Sanitization + arch §Standard Contracts):
- `update_settings` returns `Result<Settings, AppError>` — `AppError` must be `Serialize`; this is a compile-time binding enforced by TauRPC derive macros; chunk #38 inherits this contract and must not weaken it

**MCP double-gate ↔ mcp-server crate** (security plan §MCP Feature Double-Gate):
- The MCP-toggle persisted via `Settings` feeds into the runtime half of the double-gate owned by the `mcp-server` crate; the Settings form MUST NOT own or duplicate the compile-time gate logic — that logic stays in `mcp-server`

**Capability-drift xtask gate** (security plan §Tauri Capability Gating + `.claude/rules/security.md` Session Additions 2026-05-09):
- `cargo xtask capability-drift` is a required gate for any commit touching `pulse-app/src/main.rs` or `crates/ui-bridge/src/`; chunk #38 likely touches both — gate must exit 0 before merge

## Acceptance criteria contributions

1. **Input validation — enum fields**: All new enum-typed `Settings` fields (theme, widget-position, snapshot-preset, snapshot-format) fail `serde` deserialization on invalid variant strings; `update_settings` returns `AppError::Validation { field, reason }` rather than panicking or accepting the invalid value.

2. **Input validation — retention_seconds bounds**: `update_settings` called with `retention_seconds` outside the valid range (per arch `ANDROMEDA_PULSE_RETENTION_SECONDS` — 300–600 default range, any configured min/max) returns `AppError::Validation { field: "retention_seconds", reason: "..." }`, not `AppError::Internal` or `Ok`.

3. **MCP double-gate preserved**: When Settings form MCP-toggle is set to `true` and persisted, but binary was built without `--features mcp-server`, the app logs a `warn`-level message naming the missing feature flag and MCP stdio surface does NOT start — graceful-degrade path verified (per security plan §MCP Feature Double-Gate).

4. **No new capability JSON entries**: `cargo xtask capability-drift` exits 0 after chunk #38 merges, with no new entries added to `pulse-app/capabilities/` JSON files — confirms chunk correctly used the `Settings`-extension path rather than introducing new TauRPC namespaces.

5. **AppError sanitization**: `update_settings` error responses observed at the webview IPC boundary contain no stack traces, Rust struct names, file paths, or library versions — verified by inspecting `AppError` JSON serialization in tests.
