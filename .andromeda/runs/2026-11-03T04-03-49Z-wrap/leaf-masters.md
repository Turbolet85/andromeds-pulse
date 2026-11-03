group: masters
read: CLAUDE.md · 158 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:7   class: retired
  leaf-text: «Optional MCP stdio sidecar lets AI agents query telemetry directly.»
  source: fct-wuyupllp «MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)»
  replace-with: «The nine MCP tools, served by the console engine's door (over its socket or the stdio relay `andromeda-pulse-engine door`), let AI agents query telemetry directly.»

- leaf: CLAUDE.md:9   class: retired
  leaf-text: «hand-rolled JSON-RPC 2.0 MCP sidecar (`rmcp` 3.x rides feature-gated as a declared anchor dep)»
  source: fct-x56az6wm «reached over stdio through `andromeda-pulse-engine door` (PROVISIONAL — §Occupied Resources → Network ports, the door); no feature gate, no stdio sidecar and no `rmcp`»
  replace-with: «hand-rolled JSON-RPC 2.0 MCP layer served by the console engine's door over `run/door/sock` and reached over stdio through `andromeda-pulse-engine door` (no feature gate, no stdio sidecar, no `rmcp`)»

- leaf: CLAUDE.md:15   class: stale
  leaf-text: «six parallel jobs on Linux alone, `ubuntu-22.04`: lint-test · mcp-test · a11y · boot · supply-chain · coverage»
  source: fct-p7fl7gmb «Each of the five jobs of `ci.yml` (lint-test · a11y · boot · supply-chain · coverage) declares `runs-on: ubuntu-22.04`»
  replace-with: «five parallel jobs on Linux alone, `ubuntu-22.04`: lint-test · a11y · boot · supply-chain · coverage»

- leaf: CLAUDE.md:30   class: retired
  leaf-text: «MCP stdio sidecar (feature-gated `--features mcp-server`; hand-rolled serde JSON-RPC 2.0 at `jsonrpc.rs`, protocol `2024-11-05` — `rmcp` 3.x is a declared anchor dep, measured unused at 2026-08-29-advisory-backlog)»
  source: fct-ahat5ytg «# the nine tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by pulse-app in every build and served by the door; no binary, no feature»
  replace-with: «the tools library the console engine's door serves (no binary, no feature; hand-rolled serde JSON-RPC 2.0 at `jsonrpc.rs`, protocol `2024-11-05`; no `rmcp`)»

- leaf: CLAUDE.md:30   class: retired
  leaf-text: «`mcp.{status,start,stop}`»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router»
  replace-with: «no `mcp.*` TauRPC procedure (the crate mounts no router)»

- leaf: CLAUDE.md:30   class: retired
  leaf-text: «Since chunk 2026-10-10-door-inside-the-engine-s-process the crate is linked by `pulse-app` in every build (the feature gates the sidecar binary and `rmcp` only): `call_tool` is the dispatch with no log record, called by `dispatch_tool` (the sidecar) and by the console engine's door»
  source: fct-rsvfsams «The name dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>` in `tools.rs`, which writes no log record and has one product caller, `door::EngineTools`»
  replace-with: «The crate is `pulse-app`'s plain dependency, linked in every build with no feature: `call_tool` is the dispatch with no log record, its one product caller the console engine's door (`door::EngineTools`)»

- leaf: CLAUDE.md:31   class: stale
  leaf-text: «served by the MCP sidecar and, since chunk 2026-10-10-door-inside-the-engine-s-process, by the console engine's door»
  source: fct-cgszvbwl «Readers, one: the `retrieve_incident_events` tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`)»
  replace-with: «served by the console engine's door alone»

- leaf: CLAUDE.md:35   class: stale
  leaf-text: «with the sidecar's nine tools»
  source: fct-kmsyqgta «`tools/call` serves the nine tools in the engine's own process, over the engine's buffer read connection, its `CorpusWriter` and its `incident_workspace_key`»
  replace-with: «with the nine MCP tools»

- leaf: CLAUDE.md:47   class: retired
  leaf-text: «The stdio sidecar's double gate (`--features mcp-server` AND `ANDROMEDA_PULSE_MCP_ENABLED=true`) is the sidecar's alone: the door serves the same nine tools behind neither, and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through the door's relay `andromeda-pulse-engine door`, also behind neither — the double gate guards the stdio sidecar, not every stdio way to the nine.»
  source: fct-udg32czu «NEVER read a feature or a variable as what guards the nine tools: no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED` and no `mcp.feature.gate.check` record exist»
  replace-with: «No feature or variable guards the nine tools — no `mcp-server` feature and no `ANDROMEDA_PULSE_MCP_ENABLED` exist; the door, over its socket or through its stdio relay `andromeda-pulse-engine door`, is their one way in, and its owner check is what holds them.»
