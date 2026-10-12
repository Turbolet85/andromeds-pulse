group: obs-plan-2
read: .claude/rules/observability.md · 152 · «- 2026-08-26: THE ALLOWLIST FALLBACK HAS A THIRD SHAPE — it»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «ANY accidental `println!` / `dbg!` / library stdout write corrupts MCP protocol and silently disconnects the client.»
  source: .andromeda/obs-plan.md:658 «Since chunk 2026-10-12-stdio-agent-reaches-the-door the ban reaches the console program's `door` word (the stdio relay): NEVER write to its stdout anything but the door's answer bytes, unchanged»
  replace-with: «ANY accidental `println!` / `dbg!` / library stdout write corrupts MCP protocol and silently disconnects the client. The console program's `door` word (the stdio relay `pulse_app::door_bridge`, since chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL with the door's item) has no sink at all — no tokio runtime, neither boot function, no subscriber, no record, file or pid file: NEVER write to its stdout anything but the door's answer bytes, unchanged, or to its stderr anything but the one fixed line `andromeda-pulse-engine door: the door cannot be reached` on exit 1; its calls are recorded only by the engine it reaches (`door.call` / `door.refused`).»

- leaf: .claude/rules/observability.md:69   class: omission
  leaf-text: «Through the door a tool call leaves `door.call` and NO `mcp.*` span and passes no gate check»
  source: .andromeda/obs-plan.md:109 «also reached by a stdio agent through the relay `andromeda-pulse-engine door`, which adds no span, no gate and no record of its own»
  replace-with: «Through the door — over its socket directly or, since chunk 2026-10-12-stdio-agent-reaches-the-door, from a stdio agent through the relay `andromeda-pulse-engine door`, which adds no span, no gate and no record of its own (each answered request one `door.call`, a notification none, no `door.refused {reason: idle}`) — a tool call leaves `door.call` and NO `mcp.*` span and passes no gate check»
