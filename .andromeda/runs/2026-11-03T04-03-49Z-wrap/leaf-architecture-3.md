group: architecture-3
read: .claude/docs/stack.md · 98 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 101 · «- For Tauri webview debugging in dev: `cargo tauri dev` open»

- leaf: .claude/docs/stack.md:21   class: retired
  leaf-text: «hand-rolled serde JSON-RPC 2.0 over stdio (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`); `rmcp` rides as a feature-gated anchor dep (req `"3"`, resolved 3.1.4)»
  source: fct-rsvfsams «Hand-rolled serde JSON-RPC 2.0, served by the console engine's door; no `rmcp`»
  replace-with: «hand-rolled serde JSON-RPC 2.0 (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`) served inside the console engine's process over the Unix-domain socket `run/door/sock` and reached over stdio only through the relay `andromeda-pulse-engine door`; no `rmcp`»

- leaf: .claude/docs/stack.md:21   class: retired
  leaf-text: «(gated by `--features mcp-server` as the stdio sidecar and the `rmcp` anchor only). Since chunk 2026-10-10-door-inside-the-engine-s-process the same layer also runs over a Unix-domain socket inside the console engine's process (the door, `pulse-app/src/door.rs`, Unix only): `pulse-app` links the `mcp-server` crate in every build and the console engine serves the nine through the door with no feature; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine through the console program's word `andromeda-pulse-engine door` (a relay to the door), also with no feature — so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine —»
  source: fct-bka6rc62 «`pulse-app` names `mcp-server` as a plain dependency linked in every build, with no feature gate and no stdio sidecar; the console engine serves the nine through the door»
  replace-with: «— `pulse-app` names `mcp-server` as a plain dependency linked in every build, with no feature gate and no stdio sidecar; the console engine serves the nine through the door (`pulse-app/src/door.rs`, Unix only), and a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door) —»

- leaf: .claude/docs/stack.md:88   class: retired
  leaf-text: «the published line reached 3.x; pinned `"3"`, resolved 3.1.4.»
  source: fct-rsvfsams «The old "rmcp 1.5.0 vs published 0.3.x" reconciliation caveat is CLOSED by measurement: the published line reached 3.x.»
  replace-with: «the published line reached 3.x; no manifest and no `Cargo.lock` entry names `rmcp` today.»
