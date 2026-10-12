group: test-plan-4
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:35   class: stale
  leaf-text: «`run` boots the engine with no window, no model runner and three ticks, and ends by SIGTERM / SIGINT; `version`; anything else exits 2»
  source: .andromeda/test-plan.md:62 «`door` builds no runtime and boots no engine, exits 0 when standard input ends and 1 with one fixed standard-error line when the door cannot be reached»
  replace-with: «`run` boots the engine with no window, no model runner and three ticks, and ends by SIGTERM / SIGINT; `version`; since chunk 2026-10-12-stdio-agent-reaches-the-door `door`, the stdio relay to `run/door/sock` (one connection per request line, no record of its own; it builds no runtime and boots no engine, exits 0 when standard input ends and 1 with one fixed standard-error line when the door cannot be reached; PROVISIONAL with the door); anything else exits 2»

- leaf: CLAUDE.md:100   class: stale
  leaf-text: «(`andromeda-pulse-engine`, `run` | `version`, no window)»
  source: .andromeda/test-plan.md:62 «the usage line `usage: andromeda-pulse-engine <run\|version\|door>`»
  replace-with: «(`andromeda-pulse-engine`, `run` | `version` | `door`, no window)»
