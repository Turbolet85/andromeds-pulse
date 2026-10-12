group: obs-plan-1
read: .claude/docs/obs-summary.md · 200 · «Full plan: `.andromeda/obs-plan.md`.»

- leaf: .claude/docs/obs-summary.md:17   class: omission
  leaf-text: «the console program `andromeda-pulse-engine` has no event loop and ends by signal)»
  source: .andromeda/obs-plan.md:67 «`door` (the stdio relay, `pulse_app::door_bridge`; PROVISIONAL with the door's item, §4 Scenario P3) builds no tokio runtime, calls neither boot function, installs no sink»
  replace-with: «the console program's `andromeda-pulse-engine run` has no event loop and ends by signal; its `door` word, the stdio relay since chunk 2026-10-12-stdio-agent-reaches-the-door, builds no tokio runtime, calls neither boot function, installs no sink and writes no record, file or pid file)»

- leaf: .claude/docs/obs-summary.md:57   class: omission
  leaf-text: «stdout reserved for JSON-RPC 2.0 framing — ANY accidental `println!` corrupts MCP protocol.»
  source: .andromeda/obs-plan.md:76 «The console program's `door` word (the stdio relay, since chunk 2026-10-12-stdio-agent-reaches-the-door) writes no `tracing` output at all; its stdout is reserved for the door's answer lines.»
  replace-with: «stdout reserved for JSON-RPC 2.0 framing — ANY accidental `println!` corrupts MCP protocol. The console program's `door` word (the stdio relay, since chunk 2026-10-12-stdio-agent-reaches-the-door) writes no `tracing` output at all; its stdout is reserved for the door's answer lines, its stderr for one fixed line on exit 1.»

- leaf: .claude/docs/obs-summary.md:73   class: omission
  leaf-text: «The door is PROVISIONAL — obs-plan §4 Scenario P3 holds the sentence.»
  source: .andromeda/obs-plan.md:223 «Since chunk 2026-10-12-stdio-agent-reaches-the-door the door carrier is reached two ways: a client of `{data dir}/run/door/sock` directly, or a stdio agent through `andromeda-pulse-engine door`»
  replace-with: «Since chunk 2026-10-12-stdio-agent-reaches-the-door the door is reached two ways: over its socket directly, or by a stdio agent through `andromeda-pulse-engine door` (the relay: one connection per request line, the whole answer written to standard output unchanged), which writes no record itself — through it each answered request leaves one `door.call` in the engine's log, a notification none, and no `door.refused {reason: idle}`; another account's relay that reaches the door leaves one `door.refused {other_account, other}` and exits 1 with its fixed stderr line and 0 bytes on stdout. The door is PROVISIONAL — obs-plan §4 Scenario P3 holds the sentence.»

- leaf: .claude/docs/obs-summary.md:191   class: omission
  leaf-text: «NEVER write to stdout from any subsystem when running as the `andromeda-pulse-mcp` stdio sidecar.»
  source: .andromeda/obs-plan.md:658 «the ban reaches the console program's `door` word (the stdio relay): NEVER write to its stdout anything but the door's answer bytes, unchanged, or to its stderr anything but the one fixed line»
  replace-with: «NEVER write to stdout from any subsystem when running as the `andromeda-pulse-mcp` stdio sidecar; since chunk 2026-10-12-stdio-agent-reaches-the-door the ban reaches the console program's `door` word (the stdio relay): NEVER write to its stdout anything but the door's answer bytes, unchanged, or to its stderr anything but the one fixed line on exit 1 — it installs no `tracing` subscriber and writes no record (PROVISIONAL with the door's item).»

- leaf: .claude/docs/obs-summary.md:198   class: omission
  leaf-text: «the console engine's door serves the same tools behind neither gate and emits no gate-check span»
  source: .andromeda/obs-plan.md:662 «since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate; the door emits no gate-check span»
  replace-with: «the console engine's door serves the same tools behind neither gate (and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate) and emits no gate-check span»
