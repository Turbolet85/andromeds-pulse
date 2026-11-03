group: architecture+test-plan
read: .claude/docs/commands.md · 192 · «- Views: `symbol` / `refs` / `calls` / `contains` / `crate_e»

- leaf: .claude/docs/commands.md:72   class: retired
  leaf-text: «# With MCP feature»
  source: fct-hvcuzwfp «- **Feature flags**: Cargo features are `kebab-case` (`otap-ingest`); default features are minimal (no `otap-ingest`).»
  replace-with: «# MCP tools: no feature — the console engine serves the nine through its door (run/door/sock); a stdio MCP client starts `andromeda-pulse-engine door` (below)»

- leaf: .claude/docs/commands.md:73   class: retired
  leaf-text: «cargo run --bin pulse-app --features mcp-server»
  source: fct-bka6rc62 «`pulse-app` names `mcp-server` as a plain dependency linked in every build, with no feature gate and no stdio sidecar; the console engine serves the nine through the door»
  replace-with: «cargo run --bin andromeda-pulse-engine -- run»
