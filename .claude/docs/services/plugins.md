# `plugins` — wasmtime Component Model Host

## Responsibility
Hosts the WASM Component Model plugin runtime via `wasmtime` 25+. Capability-scoped sandboxing per WIT interface declarations. Loads plugins from `~/.andromeda-pulse/plugins/` (canonicalized + confined). Hosts `plugins.{list,reload,invoke}` TauRPC routers. Three plugin categories: custom dashboard / data transform / snapshot template.

## Key integrations

### Consumes from
- Filesystem `~/.andromeda-pulse/plugins/` (path resolution via `strict-path` + Tauri capability `pulse:plugin-fs`).
- WIT interface definitions in `crates/plugins/wit/` (declares host imports plugins may receive).

### Publishes to
- TauRPC routers: `plugins.list`, `plugins.reload`, `plugins.invoke`.
- Tauri Channel: `pulse://stream/plugin-events` (plugin lifecycle events).
- `tracing` events: `plugin.load.request`, `wasmtime.instantiate`, `plugin.capability.check`, `plugin.invoke.request`, `plugin.invoke.error`, `plugins.tick`.

### Dependencies
- `wasmtime` 25+ with `component-model` feature (Cranelift backend on x86_64 — verified).
- `wit-bindgen` for guest binding generation (build-time).
- `strict-path` for plugin file path canonicalization.
- `tauri` capability `pulse:plugin-fs`.
- `arrow-rs` for plugin input/output (size-bounded + schema-validated at host boundary).

## Internal conventions
- **Capability-scoped imports** — guests receive ONLY host imports declared in their WIT. NO syscalls, file, or socket access unless explicitly granted.
- **`wasmtime::Config` settings (binding):**
  - `epoch_interruption(true)` — call timeout enforcement (2-3× faster than fuel per security-research §wasmtime ResourceLimiter)
  - `max_wasm_http_fields_size` — set per April 2026 CVE-2026-27572
  - Cranelift backend on x86_64 — DO NOT switch to Winch; April 2026 sandbox-escape advisories (CVE-2026-34941 / CVE-2026-35195) were Cranelift-unaffected
- **`wasmtime::ResourceLimiter` per Store:**
  - Memory cap 64 MB
  - Tables cap (configurable)
  - Instances cap (configurable)
- **Plugin file path** canonicalize + confine to `~/.andromeda-pulse/plugins/` (resolved per platform); reject any escape attempt (`../`, symlink chains).
- **Plugin path logging:** `plugin_path_basename: "my-plugin.wasm"` ONLY (security plan vector 3) — NEVER full canonicalized path.
- **Plugin-returned Arrow IPC** — size-bound to 8 MB at host boundary before re-emit on Tauri Channel API; schema validation on deserialize.
- **NEVER use `wasmtime::Linker` to expose host functions outside the WIT contract** — bypasses capability scoping (security plan §Anti-Patterns Code Patterns).
- **Errors** collapse to `AppError::Plugin { plugin_id, message }`.

## WIT interface categories
1. **custom-dashboard** — receives query results from DuckDB; emits rendered viz spec.
2. **data-transform** — receives ingest stream; emits transformed events (e.g., redact PII attributes; derive synthetic spans).
3. **snapshot-template** — receives curated snapshot; emits formatted markdown / JSON / custom format.

## Service-specific gotchas
- **Signature verification deferred post-v1** — third-party plugins run unverified in v1. Capability-scoped WIT + `ResourceLimiter` mitigate impact, NOT provenance. Document in user-facing plugin install README.
- **Cranelift x86_64 lock** — adding a non-Cranelift wasmtime feature flag re-introduces April 2026 sandbox-escape exposure. CI build-time check on `cargo tree -p wasmtime | grep -q "cranelift"`.

## Entry points for modification
- **Engine substrate (chunk #45):** `crates/plugins/src/engine.rs` (`wasmtime::Engine` + `Config::epoch_interruption(true)` + Cranelift default on x86_64 + `MAX_WASM_HTTP_FIELDS_SIZE_BYTES`)
- **Component loader substrate (chunk #45):** `crates/plugins/src/wit_loader.rs` (Component bytes loader + 8 MB size cap + empty Linker constructor)
- **Sandbox + ResourceLimiter (chunk #46):** `crates/plugins/src/sandbox.rs` (`ResourceLimiterState` + per-Store memory / tables / instances caps + `store_for_category`)
- **Capability dispatch (chunk #46):** `crates/plugins/src/capability.rs` (per-category `Linker<ResourceLimiterState>` constructor; today returns empty Linker per category since all 3 WIT files declare zero host imports)
- **Contract module:** `crates/plugins/src/contract.rs` (`Error` enum + `PluginCategory` enum + `PluginsHeartbeat` registry-driven payload)
- **Plugin loader (chunk #47):** `crates/plugins/src/loader.rs` (filesystem scan + canonicalize + `PluginRegistry` + `discover_plugins` + per-category subdirectory layout)
- **Plugin TauRPC router (chunk #47):** `pulse-app/src/plugins_router.rs` (`plugins.{list,reload,invoke}` resolvers + DTOs; capability-handshake-only invoke at chunk #47 — full export invocation deferred to a future chunk with `wasmtime::component::bindgen!`)
- **WIT definitions:** `crates/plugins/wit/{custom-dashboard.wit,data-transform.wit,snapshot-template.wit}`
- **Tests:** colocated `#[cfg(test)] mod tests { … }` per source file + `wat::parse_str(...)` test-time WASM Component fixture generation (no committed `.wasm` binaries)

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(plugins)'`
- **Integration (test-plan §6 P4):** stage fixture WASM at `~/.andromeda-pulse/plugins/test-plugin.wasm` → `plugins.reload()` → assert `plugins.list()` contains entry → `plugins.invoke({plugin: "test-plugin", capability: "transform_spans", input: [...]})` → assert response → **negative test:** invoke with disallowed capability → assert `AppError::Plugin` rejection.
- **Cranelift enforcement (test-plan §10):** build-time check `cargo tree -p wasmtime | grep -q "cranelift"` MUST pass.

## Local development
- **Author a fixture plugin:** see `plugins-examples/` directory for built-in templates (component model + WIT bindings).
- **Test load:** copy `.wasm` to `~/.andromeda-pulse/plugins/`; run `plugins.reload()` from app or test client.

## References
- `.andromeda/architecture.md` §Stack (wasmtime 25+ Component Model) + §Conventions (WIT files kebab-case)
- `.andromeda/security-plan.md` §Plugin host capability sandbox + §Data Protection Cranelift x86_64 + §Anti-Patterns (Linker bypass, plugin path canonicalize, basename-only logging)
- `.andromeda/test-plan.md` §6 P4 + §10 (Cranelift-only WASM enforcement)
- `.andromeda/obs-plan.md` §1 P4 (must-trace `plugin.lifecycle` family) + §3 trace context
