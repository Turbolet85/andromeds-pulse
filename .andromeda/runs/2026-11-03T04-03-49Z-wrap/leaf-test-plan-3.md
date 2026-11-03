group: test-plan-3
read: .claude/rules/verification-harness.md · 167 · «- 2026-10-10: Read a count or a verdict out of a captured ga»

- leaf: .claude/rules/verification-harness.md:78   class: retired
  leaf-text: «MCP via `tokio::process::Command` + JSON-RPC 2.0 frames on stdin/stdout.»
  source: fct-o6sm757u «**Module ↔ engine door:** a `tokio::net::UnixStream` client writing JSON-RPC 2.0 lines (`serde_json`) to `run/door/sock`»
  replace-with: «MCP tools via the console engine door: a `tokio::net::UnixStream` client writing JSON-RPC 2.0 lines (`serde_json`) to `run/door/sock`, on an engine booted in process by `engine_boot::start` or a spawned `andromeda-pulse-engine run` (or the stdio relay `andromeda-pulse-engine door`); no feature flag, no env gate.»

- leaf: .claude/rules/verification-harness.md:101   class: omission
  leaf-text: «Dev-host only, not CI-wired, `xtask/src/external_resolve.rs` (`--bootstrap-seconds` / `--observe-seconds`).»
  source: fct-m53lur54 «It refuses at its precondition: the `andromeda-pulse-mcp` binary it resolves through is built by no target»
  replace-with: «Dev-host only, not CI-wired, `xtask/src/external_resolve.rs` (`--bootstrap-seconds` / `--observe-seconds`). It refuses at its precondition: the `andromeda-pulse-mcp` binary it resolves through is built by no target, its leg against the console engine owned by the route entry «Scenario legs driven against the console engine» (CARRY 2).»
