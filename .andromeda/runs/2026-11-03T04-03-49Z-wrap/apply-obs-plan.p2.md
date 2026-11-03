# Apply — obs-plan · part 2 of 2

## Verdicts
- fanout-obs-plan.md 23 · applied · E1
- fanout-obs-plan.md 25 · applied · E2
- fanout-obs-plan.md 26 · applied · E3

## Edits
@fact id=fct-65zmgzbk expect_rev=1
old: `2026-10-02-incident-events-readable-through-mcp`). For the mcp-server sidecar (rmcp 3.x resolved 3.1.4, declared-but-unused; hand-rolled JSON-RPC 2.0 over stdio), stderr is forced JSON (no TTY check) — stdout is reserved for JSON-RPC 2.0 framing and ANY accidental stdout write would corrupt the protocol. The console program's
new: `2026-10-02-incident-events-readable-through-mcp`). The console program's

@fact id=fct-mfvbrkak expect_rev=1
old: = "pulse-app"); for mcp-server sidecar: `"andromeda-pulse-mcp"` (distinct process identity per arch convention)
new: = "pulse-app")

@fact id=fct-qnzkp4df expect_rev=1
old: **MCP tools (hand-rolled JSON-RPC 2.0 over stdio; rmcp 3.x declared-but-unused):**
new: **MCP tools (hand-rolled JSON-RPC 2.0 through the door):**
old: Through the stdio sidecar every tool emits `mcp.tools.call.request` (parent) → `mcp.tools.call.response` (result metadata only — `result_type` + `result_count`, NOT result_content per security plan vector 4) at `dispatch_tool`, which since chunk 2026-10-10-door-inside-the-engine-s-process calls `mcp_server::tools::call_tool` (it returns `ToolAnswer { value, result_type, result_count }` and writes no record); `call_tool` has a second caller, `door::EngineTools` in the console engine, whose calls are recorded as `door.call` carrying `result_type` + `result_count` and never the answer, so vector 4 holds on both carriers. "Over stdio" in this label is the sidecar's transport and, since chunk `2026-10-12-stdio-agent-reaches-the-door`, also that of the `andromeda-pulse-engine door` relay, which carries each line to the door's Unix-domain socket; calls through the relay are recorded as `door.call` only (obs-plan §4 Scenario P3).
new: The console engine's door serves every tool through `mcp_server::tools::call_tool` (it returns `ToolAnswer { value, result_type, result_count }` and writes no record), its one product caller `door::EngineTools`; each call is recorded as `door.call` carrying `result_type` + `result_count` and never the answer (result metadata only, NOT result_content per security plan vector 4). Over stdio means the `andromeda-pulse-engine door` relay alone, which carries each line to the door's Unix-domain socket; calls through it are recorded as `door.call` only (obs-plan §4 Scenario P3).

## Patterns
[[pattern]]
id = "obs2-01"
fixed = 'stderr is forced JSON'
ignore_case = true

[[pattern]]
id = "obs2-02"
fixed = 'distinct process identity'
ignore_case = true

[[pattern]]
id = "obs2-03"
fixed = 'Through the stdio sidecar every tool emits'
ignore_case = true
