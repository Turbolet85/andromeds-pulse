# security extract

## Relevance
Partial — touches Settings persistence and static window configuration, both covered by existing validation controls, with no new threat boundaries or IPC surfaces.

## Constraints
1. **Settings field validation (security plan §Input Validation, Configuration values row):** If "remembered geometry" is chosen, any position/size fields added to the Settings struct MUST use `serde` derive for shape validation and bounded integer types (e.g., `u16`/`i32`, not `String` or unbounded `i64`) to prevent pathological window dimensions per the four-boundary input-validation discipline.
2. **No new TauRPC procedures (security plan §API Security, TauRPC capability authorization paragraph):** Geometry application occurs at boot via the existing `apply_widget_settings()` path (Settings get/set), NOT via a new `set_window_geometry` IPC procedure. Adding a procedure without matching `pulse-app/capabilities/` JSON entry is a silent-rejection runtime bug and a trust-boundary gap.
3. **Environment-variable registration (security plan amendments 2026-06-28, §Input Validation CLI / env var inputs row):** If a future scope adds environment overrides (e.g., `ANDROMEDA_PULSE_WINDOW_WIDTH`), they MUST be registered in §Input Validation CLI / env var inputs row with bounded-parse discipline (`TryFrom<u16>` for port ranges; explicit bounds for dimensions) — do not allow unbounded string coordinates. Register at code-validation time; do not defer to security-plan amendments.
4. **Path canonicalization for config resolution (security plan §Input Validation, CLI / env var inputs row + §Security Anti-Patterns § Code Patterns):** If window configuration reads from `ANDROMEDA_PULSE_CONFIG_PATH`, that path MUST be canonicalized via `std::path::Path::canonicalize()` and asserted to reside under the per-platform data dir per CWE-22 defense (zip/RustFS 2025 CVEs document the same symlink-traversal class). Use `strict-path` crate per bootstrap discipline.
5. **Error collapse at TauRPC boundary (security plan §Error Handling, TauRPC bridge paragraph):** Geometry-validation errors on `get_settings`/`update_settings` MUST convert to `AppError::{Validation, Internal}` enum variants; no raw `thiserror` enum variants leak to the webview per Conventions Error response schema (Tauri IPC).
6. **No logging of coordinate values as sensitive (security plan §Logging & Monitoring, NEVER-log list + anti-patterns §Logging):** While window position is not a secret, avoid logging full coordinate tuples in user-visible error messages or tracing output to prevent privacy-information leakage. Log geometry validation failures only as `"invalid dimension bounds"`, not `"x=99999, y=out_of_range"`.

## Patterns to follow
1. **Existing Settings validation pattern (§Input Validation, Configuration values row):** Extend the Settings struct with serde derive on new geometry fields; leverage existing `serde` + `TryFrom<u16>` discipline rather than inventing new validation layers.
2. **Boot-time Settings application (chunk scope §Boot-time geometry application + §Bootstrap phases `logging-redaction-wire`):** Apply geometry at app startup via the established `apply_widget_settings()` pattern, consistent with other Epoch-1 Settings flows; pair with existing Settings-extension learning (2026-05-09 referenced in scope).

## Anti-patterns to avoid
1. **No ad-hoc TauRPC procedure for geometry (security plan §API Security, TauRPC capability authorization):** Do not add `set_window_geometry` or similar procedure; capability-less procedures are silently rejected at runtime, creating hard-to-diagnose UX bugs and trust-boundary gaps.
2. **No unbounded or string-typed coordinates (security plan §Input Validation bounded-integer discipline + §Anti-Patterns § Input):** Never use `String`/`f64`/unbounded `i64` for window dimensions; enforce `u16` or bounded `i32` ranges via type system.
3. **No plaintext coordinate logging in user-facing paths (security plan §Anti-Patterns § Logging, NEVER-log list + internal §Error Handling logging paragraph):** Keep full coordinate tuples out of error messages, UI tooltips, and tracing output visible to users; log geometry failures as semantic errors only.

## Contract bindings
- **Settings ↔ TauRPC bridge (§API Security TauRPC capability authorization):** Geometry persisted to Settings flows through existing `get_settings`/`update_settings` IPC procedures; no new capability JSON entry needed (confirmed by chunk scope "no new namespace expected").

## Acceptance criteria contributions
1. **(security) Window geometry fields use bounded integer types** — if added to Settings, grep verifies no `String` / `f64` / unbounded `i64` coordinate fields; only `u16` / `i32` or bounded newtype wrappers per §Input Validation Configuration values row.
2. **(security) No new TauRPC procedure for geometry** — capability JSON (`pulse-app/capabilities/`) unchanged; existing `get_settings`/`update_settings` capability remains the sole Settings-access surface.
3. **(security) Geometry applied via existing Settings pattern** — window configuration happens via `apply_widget_settings()` / existing Settings apply flow at boot, verified by code inspection of `pulse-app/src/main.rs` setup closure.

## Relevant amendment history
**2026-06-28 — Env-var registration discipline (§Input Validation, CLI / env var inputs row):** Added `ANDROMEDA_PULSE_L4_DETERMINISTIC` to the env-var boundary registry with bounded truthy-parse validation; concluded that code-validated env vars must be registered in §Input Validation enumeration to close D-security-input registry-completeness gaps. Implication for this chunk: if window-geometry env-var overrides are added in a future scope, register them in the same row with explicit bounds (not unbounded string parse). This chunk's scope does not mention env-var overrides (only `tauri.conf.json` static config and optional Settings persistence), so this is forward-looking.