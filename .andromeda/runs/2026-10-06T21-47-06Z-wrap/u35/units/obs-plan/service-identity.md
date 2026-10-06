### Service identity

- **service.name:** compile-time constant `"com.andromeda.pulse"` (Tauri bundle identifier; fallback `env!("CARGO_PKG_NAME")` = "pulse-app"); for mcp-server sidecar: `"andromeda-pulse-mcp"` (distinct process identity per arch convention)
- **service.version:** compile-time `env!("CARGO_PKG_VERSION")` (or runtime read from `tauri.conf.json` for the bundled desktop build)
- **deployment.environment:** hardcoded `"production"` (desktop app, no staging/dev distinction at runtime)
- **Default subscriber fields:** registered once at subscriber init via `tracing_subscriber::fmt::Layer::with_default_fields([service_name, service_version, deployment_environment])` — every emitted JSON line carries identity in the `fields` map without per-call boilerplate
