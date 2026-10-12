group: architecture-4
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:35   class: stale
  leaf-text: «`version`; anything else exits 2;»
  source: .andromeda/architecture.md:217 «a closed three-word command line (usage `usage: andromeda-pulse-engine <run|version|door>`; the third word since chunk 2026-10-12-stdio-agent-reaches-the-door)»
  replace-with: «`version`; since chunk 2026-10-12-stdio-agent-reaches-the-door `door`, the stdio relay to a running engine's `run/door/sock` (`pulse_app::door_bridge::main`; no tokio runtime, no engine boot, no file, listener, log record or variable of its own; an unreachable door exits 1 with one fixed standard-error line; PROVISIONAL with the door); anything else exits 2;»

- leaf: CLAUDE.md:100   class: stale
  leaf-text: «`run` | `version`, no window»
  source: .andromeda/architecture.md:300 «driven by a closed three-word command line (`run` | `version` | `door`) with no window and no display — `door` (since chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL»
  replace-with: «`run` | `version` | `door`, no window»

- leaf: CLAUDE.md:100   class: stale
  leaf-text: «so the OS sees one process per program»
  source: .andromeda/architecture.md:4 «the OS sees one engine process per program; since chunk 2026-10-12-stdio-agent-reaches-the-door the console program's `door` word also runs as short-lived relay processes that host no engine part»
  replace-with: «so the OS sees one engine process per program (since chunk 2026-10-12-stdio-agent-reaches-the-door the console program's `door` word also runs as short-lived relay processes that host no engine part; PROVISIONAL with the door)»

- leaf: CLAUDE.md:47   class: omission
  leaf-text: «the door serves the same nine tools behind neither.»
  source: .andromeda/architecture.md:25 «a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door), also with no feature — so `--features mcp-server` gates the stdio sidecar, not every stdio way»
  replace-with: «the door serves the same nine tools behind neither, and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through the door's relay `andromeda-pulse-engine door`, also behind neither — the double gate guards the stdio sidecar, not every stdio way to the nine.»
