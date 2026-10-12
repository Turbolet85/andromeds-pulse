group: architecture-2
read: .claude/docs/services/interpretation.md · 59 · «- `.claude/rules/observability.md` — allowlist-leaf discipli»
read: .claude/docs/services/mcp-server.md · 65 · «- `.andromeda/obs-plan.md` §1 P3 (must-trace `mcp.session` f»
read: .claude/docs/services/plugins.md · 73 · «- `.andromeda/obs-plan.md` §1 P4 (must-trace `plugin.lifecyc»
read: .claude/docs/services/security.md · 55 · «- **Tests:** 85 in the crate suite (`cargo nextest list -p s»
read: .claude/docs/services/snapshot.md · 67 · «- `.andromeda/design-system.md` §Motion Investigation Captur»
read: .claude/docs/services/triage.md · 48 · «- **Tests:** unit tests inline per sub-module; integration v»
read: .claude/docs/services/ui-bridge.md · 61 · «- `.andromeda/obs-plan.md` §3 Trace context propagation (IPC»

- leaf: .claude/docs/services/mcp-server.md:6   class: omission
  leaf-text: «The door's form: architecture §Standard Contracts → Engine door.»
  source: .andromeda/architecture.md:60 «a stdio agent reaches the nine through the door with no sidecar and no feature, by starting `andromeda-pulse-engine door` (`pulse_app::door_bridge`, on `std` Unix sockets»
  replace-with: «Since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent also reaches the nine through the door with no sidecar and no feature, by starting `andromeda-pulse-engine door` (`pulse_app::door_bridge`), which relays its standard-input lines to `run/door/sock` (one connection per request) and the door's answer lines, unchanged, to its standard output — so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine, and the ways in stay two (PROVISIONAL, with the door). The door's form: architecture §Standard Contracts → Engine door.»

- leaf: .claude/docs/services/mcp-server.md:45   class: stale
  leaf-text: «(`ALL_TOOL_NAMES` + `dispatch_tool`; no macro annotations)»
  source: .andromeda/architecture.md:60 «the name dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>` in `tools.rs`, which writes no log record and has two callers: `dispatch_tool` — the sidecar's path»
  replace-with: «(`ALL_TOOL_NAMES` + `call_tool`, the name dispatch, called by the sidecar's `dispatch_tool` and by the door's `door::EngineTools`; no macro annotations)»

- leaf: .claude/docs/services/ui-bridge.md:9   class: omission
  leaf-text: «its state is read from its log records.»
  source: .andromeda/architecture.md:295 «or, since chunk 2026-10-10-door-inside-the-engine-s-process, through the door (`pulse-app/tests/integration_door.rs` sends a log record over real OTLP and reads it back with a `query_logs` call»
  replace-with: «its state is read from its log records, from the `EngineHandles` that `engine_boot::start` returns, or through the door (`run/door/sock`; architecture §Standard Contracts → Engine door).»
