group: security-plan-1
read: .claude/docs/security-summary.md · 67 · «Full plan: `.andromeda/security-plan.md`.»

- leaf: .claude/docs/security-summary.md:22   class: retired
  leaf-text: «**MCP stdio surface** — `andromeda-pulse-mcp` sidecar: hand-rolled serde JSON-RPC 2.0 over stdin/stdout (`jsonrpc.rs`; `rmcp` is a feature-gated anchor import, not the protocol layer). Feature double-gated (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true`) — the double gate is the sidecar's alone since chunk `2026-10-10-door-inside-the-engine-s-process`, and stdio is no longer the sidecar's alone since chunk `2026-10-12-stdio-agent-reaches-the-door`: a stdio agent reaches the same nine tools through `andromeda-pulse-engine door`, the relay to the console engine's door, behind neither gate, its caller checked by the door's account check (vector 4a; PROVISIONAL); the sidecar does not enforce the manifest's `additionalProperties: false` (its serde structs read past an undeclared member; no fix is built, the sidecar leaves at the route entry `Stdio sidecar retired`).»
  source: fct-kmsyqgta «It is a way through the door, not a second way in: the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay.»
  replace-with: «**MCP tools over stdio** — no surface of their own: a stdio agent reaches the nine tools only through `andromeda-pulse-engine door`, the relay to the console engine's door (vector 4a; PROVISIONAL), the engine's one way in besides telemetry; hand-rolled serde JSON-RPC 2.0 (`jsonrpc.rs`, no `rmcp` in any manifest or `Cargo.lock`); the manifest's `additionalProperties: false` is refused by the door's `declared_only` alone (`-32602` for all nine tools before any tool runs), the argument structs carrying no `deny_unknown_fields`.»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «serving the sidecar's nine tools over the engine's own stores, behind neither sidecar gate»
  source: fct-kmsyqgta «`tools/call` serves the nine tools in the engine's own process, over the engine's buffer read connection, its `CorpusWriter` and its `incident_workspace_key`.»
  replace-with: «serving the nine tools over the engine's own stores»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a way through the door, not a third way in, under the same PROVISIONAL item»
  source: fct-kmsyqgta «It is a way through the door, not a second way in: the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay.»
  replace-with: «a way through the door, not a second way in — the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay — under the same PROVISIONAL item»

- leaf: .claude/docs/security-summary.md:59   class: stale
  leaf-text: «and since chunk `2026-10-12-stdio-agent-reaches-the-door` `andromeda-pulse-engine door` is a third carrier of tool answers, writing the door's answers unchanged to the standard output of the process that started it.»
  source: fct-cvhwrox6 «Tool responses reach an agent through the console engine's door alone: the door returns the `query_*` / `generate_snapshot` answers, unsanitized OTLP-derived content included, to a caller of the engi»
  replace-with: «and tool answers reach an agent through the console engine's door alone — over its Unix socket to a caller of the engine's own account, and through `andromeda-pulse-engine door`, which writes them unchanged to the standard output of the process that started it.»
