group: obs-plan-2
read: .claude/rules/observability.md · 152 · «- 2026-08-26: THE ALLOWLIST FALLBACK HAS A THIRD SHAPE — it»

- leaf: .claude/rules/observability.md:23   class: retired
  leaf-text: «(Tauri bundle id); for the MCP sidecar = `"andromeda-pulse-mcp"`.»
  source: fct-mfvbrkak «- **service.name:** compile-time constant `"com.andromeda.pulse"` (Tauri bundle identifier; fallback `env!("CARGO_PKG_NAME")` = "pulse-app")»
  replace-with: «(Tauri bundle id).»

- leaf: .claude/rules/observability.md:39   class: retired
  leaf-text: «**MCP sidecar (`andromeda-pulse-mcp`; hand-rolled JSON-RPC 2.0):** stderr forced JSON (no TTY check) — stdout reserved for JSON-RPC 2.0 framing. ANY accidental `println!` / `dbg!` / library stdout write corrupts MCP protocol and silently disconnects the client. The console program's `door` word»
  source: fct-anuozml7 «its stdout carries the door's JSON-RPC 2.0 answers, and ANY accidental `println!` / `dbg!` / library stdout write corrupts them for the agent; it installs no `tracing` subscriber and writes no record»
  replace-with: «**Door relay (`andromeda-pulse-engine door`; no stdio sidecar exists):** its stdout carries the door's JSON-RPC 2.0 answers — ANY accidental `println!` / `dbg!` / library stdout write corrupts them for the agent. The console program's `door` word»

- leaf: .claude/rules/observability.md:58   class: retired
  leaf-text: «- NEVER log the full workspace root path or the published workspace-key value — the `app.boot.workspace_key` event carries `workspace_root_basename` + `key_bytes` only (WARN form: `error_category` + `error_detail`). It needs its own explicit allowlist leaf entry, since `for_target`'s prefix fallback would otherwise resolve this dotted `app.`-prefixed target to an unrelated field set and redact both fields — same trap as `app.boot.buffer.degraded`.»
  source: fct-oodqyure «- `app.boot.workspace_key`: `workspace_root_basename`, `key_bytes` on the INFO publish line; `error_category`, `error_detail` on the WARN failure line. Explicit leaf entry for the same `for_target` fallback»
  replace-with: «»

- leaf: .claude/rules/observability.md:59   class: retired
  leaf-text: «(same `for_target` trap as `app.boot.workspace_key`)»
  source: fct-2s3fzja2 «Each is an EXACT leaf for the same `for_target` prefix-fallback reason, and **no bare `corpus` key may exist** — a prefix key would silently widen every future `corpus.*` target to one field set»
  replace-with: «(the same `for_target` prefix-fallback trap)»

- leaf: .claude/rules/observability.md:60   class: stale
  leaf-text: «counting registry rows dropped because a writer outside this process had already resolved them»
  source: fct-avpvhn3k «counting registry rows dropped because the persistent corpus already held them resolved — written by `mark_incident_resolved` through the console engine's door»
  replace-with: «counting registry rows dropped because the persistent corpus already held them resolved (written by `mark_incident_resolved` through the console engine's door)»

- leaf: .claude/rules/observability.md:69   class: retired
  leaf-text: «— the `mcp.*` chain is the stdio sidecar's (obs-plan §4 Scenario P3, which holds the door's PROVISIONAL sentence).»
  source: fct-4vrfwv27 «`tools/call` runs `mcp_server::tools::call_tool` over `door::EngineTools`, its one product caller; no `mcp.*` record has an emitter, and no feature or runtime gate exists.»
  replace-with: «— no `mcp.*` record has an emitter, and no `mcp-server` feature, `ANDROMEDA_PULSE_MCP_ENABLED` or other gate guards the nine tools: the door's admission is the owner check on every accepted connection (obs-plan §4 Scenario P3, which holds the door's PROVISIONAL sentence).»
