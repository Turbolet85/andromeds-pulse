group: security-plan-2
read: .claude/rules/security.md · 202 · «- 2026-10-10: Harness code that runs INSIDE the product's pr»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «all three product binaries (`pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp`) read the system `XDG_RUNTIME_DIR`»
  source: fct-ixrmtmoe «both product binaries (`pulse-app`, `andromeda-pulse-engine`) read the system `XDG_RUNTIME_DIR` (Linux) through `Corpus::open` only to locate the content-free corpus-key lock file.»
  replace-with: «both product binaries (`pulse-app`, `andromeda-pulse-engine`) read the system `XDG_RUNTIME_DIR`»

- leaf: .claude/rules/security.md:39   class: retired
  leaf-text: «## MCP feature double-gate»
  source: fct-6geshnvr «MCP tool inputs (`mcp-server` crate, served through the console engine's door alone — all 9 tools dispatched by name»
  replace-with: «## MCP tools (served through the door alone)»

- leaf: .claude/rules/security.md:40   class: retired
  leaf-text: «MCP stdio surface MUST require BOTH compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`. Single-gate is a regression of the architecture-declared double-gate.»
  source: fct-6geshnvr «Hand-rolled serde JSON-RPC 2.0 framing (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`, name-dispatch via `mcp_server::tools::call_tool`, whose one product caller is `door::EngineTools`; no `rmcp`)»
  replace-with: «The nine MCP tools are served through the console engine's door alone — hand-rolled serde JSON-RPC 2.0 framing (`crates/mcp-server/src/jsonrpc.rs`), name-dispatch via `mcp_server::tools::call_tool`, whose one product caller is `door::EngineTools`.»

- leaf: .claude/rules/security.md:41   class: retired
  leaf-text: «The graceful-degrade `warn` (env var set against a binary built without the feature) MUST be preserved — do not regress to fail-startup.»
  source: fct-zvxze7ay «MCP feature flag mismatch warning (`ANDROMEDA_PULSE_MCP_ENABLED=true` against a binary built without `--features mcp-server`) — preserve the existing graceful-degrade `warn` per Occupied Resources»
  replace-with: «`call_tool` returns `ToolAnswer` and writes no log record; `tools/list` gives every tool a boolean `write`, true for `mark_incident_resolved` alone.»

- leaf: .claude/rules/security.md:42   class: retired
  leaf-text: «The double gate governs the stdio sidecar ONLY. Since chunk `2026-10-10-door-inside-the-engine-s-process` the console engine's door serves the same nine tools outside it (`pulse-app` links `mcp-server` in every build; `rmcp` still enters only under the feature); the door's own controls are the next section's.»
  source: fct-6geshnvr «No `rmcp`: it is in no manifest and not in `Cargo.lock`.»
  replace-with: «Since chunk `2026-10-10-door-inside-the-engine-s-process` the console engine's door serves the nine tools, and it alone (no `rmcp`: it is in no manifest and not in `Cargo.lock`); the door's own controls are the next section's.»

- leaf: .claude/rules/security.md:42   class: stale
  leaf-text: «behind neither gate; the door's own controls are what hold it — a way through the door, not a third way in. Until the route entry `Stdio sidecar retired` lands, the engine has two ways in besides telemetry.»
  source: fct-kmsyqgta «It is a way through the door, not a second way in: the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay.»
  replace-with: «the door's own controls are what hold it — a way through the door, not a second way in: the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay.»

- leaf: .claude/rules/security.md:53   class: retired
  leaf-text: «The stdio sidecar does NOT refuse an undeclared argument member (its serde structs read past one; the manifest's `additionalProperties: false` is unenforced there); only the door does. No fix is built — the sidecar leaves at the route entry `Stdio sidecar retired`, which owns the gap.»
  source: fct-6geshnvr «the door's `declared_only` is the one refusal of it — `-32602` for all nine tools before any tool runs, nothing of it answered»
  replace-with: «The tool manifest declares `additionalProperties: false` on every tool; the door's `declared_only` is the one refusal of an undeclared argument member — `-32602` for all nine tools before any tool runs, nothing of it answered (the argument structs carry no `deny_unknown_fields`).»
