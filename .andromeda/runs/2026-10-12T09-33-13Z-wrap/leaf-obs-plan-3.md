group: obs-plan-3
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:35   class: stale
  leaf-text: «`version`; anything else exits 2;»
  source: .andromeda/obs-plan.md:67 «the console program `andromeda-pulse-engine` (grammar closed at `run`, `version` or, since chunk 2026-10-12-stdio-agent-reaches-the-door, `door`)»
  replace-with: «`version`; since chunk 2026-10-12-stdio-agent-reaches-the-door `door`, the stdio relay to the door (`pulse_app::door_bridge`; no tokio runtime, neither boot function, no sink and no record of its own — its calls are recorded by the engine it reaches as `door.call` / `door.refused`; stdout carries only the door's answer bytes, stderr only the fixed line `andromeda-pulse-engine door: the door cannot be reached` on exit 1; PROVISIONAL with the door's item); anything else exits 2;»

- leaf: CLAUDE.md:100   class: stale
  leaf-text: «(`andromeda-pulse-engine`, `run` | `version`, no window)»
  source: .andromeda/obs-plan.md:67 «the console program `andromeda-pulse-engine` (grammar closed at `run`, `version` or, since chunk 2026-10-12-stdio-agent-reaches-the-door, `door`)»
  replace-with: «(`andromeda-pulse-engine`, `run` | `version` | `door`, no window; `door`, the stdio relay to the running engine's door, calls neither boot function)»

- leaf: CLAUDE.md:47   class: omission
  leaf-text: «is the sidecar's alone: the door serves the same nine tools behind neither.»
  source: .andromeda/obs-plan.md:662 «and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate»
  replace-with: «is the sidecar's alone: the door serves the same nine tools behind neither, and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate.»
