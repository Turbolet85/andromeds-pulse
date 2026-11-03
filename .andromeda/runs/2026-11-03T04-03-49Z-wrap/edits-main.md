# Main's edits — 2026-11-03-stdio-sidecar-retired

The two hand-removal rows that keep text (design-system 1 — the Actions list re-written as a new fact after fct-dzpd3ztt, which is then retired; obs-plan 28). Trigger 2's removal leaves the source digits 1, 3, 4 (a list marker cannot change in place, `reshaped`); an ordered list renders in sequence.

## Edits

@fact after=fct-dzpd3ztt as=n1
- Actions:
  - "Generate Snapshot" (invokes `snapshot.generate` TauRPC, shows "Snapshot ready" notification on completion).
  - "Open Settings" (focuses the Settings panel in the app).
  - Divider.
  - "Quit" (terminates the process; tray icon disappears).

@fact id=fct-udg32czu expect_rev=1
old: - NEVER feature-gate MCP server without runtime `ANDROMEDA_PULSE_MCP_ENABLED=true` double-gate (security plan anti-pattern: "MCP double-gate omission") — compile-time flag (`--features mcp-server`) + runtime env var both required; the gate state is a `tracing` field on `mcp.feature.gate.check` span. The ban is the stdio sidecar's (`andromeda-pulse-mcp`) and stands unchanged for it. Since chunk 2026-10-10-door-inside-the-engine-s-process the console engine's door serves the same nine tools behind neither gate (`pulse-app` links `mcp-server` in every build; `rmcp` still enters only under the feature), and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate; the door emits no gate-check span: its admission
new: - NEVER read a feature or a variable as what guards the nine tools: no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED` and no `mcp.feature.gate.check` record exist since chunk 2026-11-03-stdio-sidecar-retired, and the tools are reached only through the console engine's door, over its socket or through `andromeda-pulse-engine door`; the door emits no gate-check span: its admission

## Patterns
(none)
