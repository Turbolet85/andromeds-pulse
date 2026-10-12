group: security-plan-3
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:35   class: stale
  leaf-text: «`version`; anything else exits 2»
  source: .andromeda/security-plan.md:145 «A closed three-word grammar: `run` boots the engine and stays up until signalled; `version` prints one line and exits 0; `door` is the stdio relay to a running engine's door (below);»
  replace-with: «`version`; `door`, since chunk 2026-10-12-stdio-agent-reaches-the-door, boots no engine and relays a stdio agent's lines to the door of an engine already running on the same data dir, behind neither gate of the sidecar's double gate — PROVISIONAL with the door's one item; anything else exits 2»

- leaf: CLAUDE.md:100   class: stale
  leaf-text: «(`andromeda-pulse-engine`, `run` | `version`, no window)»
  source: .andromeda/security-plan.md:104 «by a third word, `door`, which boots no engine and relays a stdio agent's lines to the door of an engine already running on the same data dir»
  replace-with: «(`andromeda-pulse-engine`, `run` | `version` | `door`, no window; `door` boots no engine and relays a stdio agent's lines to a running engine's door)»
