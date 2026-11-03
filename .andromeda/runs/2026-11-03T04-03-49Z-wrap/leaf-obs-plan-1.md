group: obs-plan-1
read: .claude/docs/obs-summary.md · 200 · «Full plan: `.andromeda/obs-plan.md`.»

- leaf: .claude/docs/obs-summary.md:32   class: retired
  leaf-text: «`service.name` = compile-time `"com.andromeda.pulse"` (Tauri bundle id); MCP sidecar = `"andromeda-pulse-mcp"`»
  source: fct-mfvbrkak «**service.name:** compile-time constant `"com.andromeda.pulse"` (Tauri bundle identifier; fallback `env!("CARGO_PKG_NAME")` = "pulse-app")»
  replace-with: «`service.name` = compile-time `"com.andromeda.pulse"` (Tauri bundle id)»

- leaf: .claude/docs/obs-summary.md:57   class: retired
  leaf-text: «**MCP sidecar (`andromeda-pulse-mcp`; hand-rolled JSON-RPC 2.0):** stderr forced JSON (no TTY check); stdout reserved for JSON-RPC 2.0 framing — ANY accidental `println!` corrupts MCP protocol. The console program's `door` word (the stdio relay, since chunk 2026-10-12-stdio-agent-reaches-the-door) writes no `tracing` output at all»
  source: fct-65zmgzbk «The console program's `door` word (the stdio relay, chunk `2026-10-12-stdio-agent-reaches-the-door`; PROVISIONAL with the door's item) has no sink at all — no subscriber, no record»
  replace-with: «**Door relay (the console program's `door` word, the stdio relay; PROVISIONAL with the door's item):** no sink at all — no subscriber, no record, no `tracing` output»

- leaf: .claude/docs/obs-summary.md:71   class: stale
  leaf-text: «Every P1–P7 must emit parent + child spans with the `{module}.{operation}` naming convention.»
  source: fct-sjvcbexl «`duckdb.query.traces` (or metrics/logs per telemetry tool; the five incident tools read the corpus instead), under the door's must-log records `door.call` and `door.refused` — no `mcp.*` span and no gate»
  replace-with: «Every P1–P7 but P3 must emit parent + child spans with the `{module}.{operation}` naming convention; P3's per-tool read sits under the door's must-log records `door.call` / `door.refused`, the door opening no span.»

- leaf: .claude/docs/obs-summary.md:73   class: retired
  leaf-text: «**P3 has two carriers since chunk 2026-10-10-door-inside-the-engine-s-process.** The `mcp.tools.call.request` → `mcp.feature.gate.check` → per-tool read → `mcp.tools.call.response` chain is the stdio sidecar's, unchanged. The console engine's door (JSON-RPC 2.0 over the Unix-domain socket `{data dir}/run/door/sock`) runs the same nine tools through `call_tool`, which writes no `mcp.*` record and passes no gate:»
  source: fct-4vrfwv27 «**One carrier.** A tool call is carried by the door inside the console engine's process (`andromeda-pulse-engine run`; JSON-RPC 2.0, one request per line, over the Unix-domain socket `{data dir}/run/door/sock`;»
  replace-with: «**P3 has one carrier, the console engine's door.** The door (JSON-RPC 2.0 over the Unix-domain socket `{data dir}/run/door/sock`; Unix only, the window program seats none) runs the nine tools through `call_tool` over `door::EngineTools`, its one product caller; no `mcp.*` record has an emitter and no feature or runtime gate exists:»

- leaf: .claude/docs/obs-summary.md:73   class: stale
  leaf-text: «The obs scope counts 9 major surfaces since that chunk (8 before).»
  source: fct-o4vhpvgv «8 major surfaces (desktop-webview, desktop-native, IPC-internal, API service gRPC, API service HTTP, CLI, library-only plugin host and the console engine's door — JSON-RPC 2.0 over a Unix-domain socket»
  replace-with: «The obs scope counts 8 major surfaces, the door among them (no MCP stdio surface).»

- leaf: .claude/docs/obs-summary.md:167   class: stale
  leaf-text: «counting registry rows dropped because a writer outside this process had already resolved them»
  source: fct-avpvhn3k «counting registry rows dropped because the persistent corpus already held them resolved — written by `mark_incident_resolved` through the console engine's door, the running engine dropping such a row»
  replace-with: «counting registry rows dropped because the persistent corpus already held them resolved (written by `mark_incident_resolved` through the console engine's door, dropped from the active set within one persist cycle)»

- leaf: .claude/docs/obs-summary.md:191   class: retired
  leaf-text: «NEVER write to stdout from any subsystem when running as the `andromeda-pulse-mcp` stdio sidecar; since chunk 2026-10-12-stdio-agent-reaches-the-door the ban reaches the console program's `door` word (the stdio relay): NEVER write to its stdout»
  source: fct-anuozml7 «NEVER write to the stdout of the console program's `door` word (the stdio relay) anything but the door's answer bytes, unchanged, or to its stderr anything but the one fixed line on exit 1»
  replace-with: «NEVER write to the stdout of the console program's `door` word (the stdio relay) — it carries the door's JSON-RPC 2.0 answers, and ANY accidental `println!` / `dbg!` / library stdout write corrupts them — anything but the door's answer bytes»

- leaf: .claude/docs/obs-summary.md:198   class: retired
  leaf-text: «The MCP double-gate ban (compile-time feature + runtime variable, its state on `mcp.feature.gate.check`) is the stdio sidecar's; the console engine's door serves the same tools behind neither gate (and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate) and emits no gate-check span»
  source: fct-udg32czu «NEVER read a feature or a variable as what guards the nine tools: no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED` and no `mcp.feature.gate.check` record exist, and the tools are reached only through»
  replace-with: «NEVER read a feature or a variable as what guards the nine tools: no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED` and no `mcp.feature.gate.check` record exist; the tools are reached only through the console engine's door, over its socket or through `andromeda-pulse-engine door`, and the door emits no gate-check span»
