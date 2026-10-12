group: security-plan-1
read: .claude/docs/security-summary.md · 67 · «Full plan: `.andromeda/security-plan.md`.»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «the double gate is the sidecar's alone since chunk `2026-10-10-door-inside-the-engine-s-process`;»
  source: .andromeda/security-plan.md:68 «**Stdio is no longer the sidecar's alone:** since chunk `2026-10-12-stdio-agent-reaches-the-door` a stdio agent reaches the same nine tools through `andromeda-pulse-engine door`, the relay to»
  replace-with: «the double gate is the sidecar's alone since chunk `2026-10-10-door-inside-the-engine-s-process`, and stdio is no longer the sidecar's alone since chunk `2026-10-12-stdio-agent-reaches-the-door`: a stdio agent reaches the same nine tools through `andromeda-pulse-engine door`, the relay to the console engine's door, behind neither gate, its caller checked by the door's account check (vector 4a; PROVISIONAL);»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «Own host only, no TCP port added.»
  source: .andromeda/security-plan.md:72 «a stdio agent reaches the door through `andromeda-pulse-engine door`, a short-lived relay it starts (one per agent session) — a client of `run/door/sock`, one connection per request line»
  replace-with: «Own host only, no TCP port added. Since chunk `2026-10-12-stdio-agent-reaches-the-door` a stdio agent reaches the door through `andromeda-pulse-engine door`, a short-lived relay it starts (one per agent session) — a client of `run/door/sock`, one connection per request line, with no listener, port, file, pid file, variable or log record of its own, admitted or refused by the same owner check (run by another account it receives no byte; measured on `ci#38227488889`); a way through the door, not a third way in, under the same PROVISIONAL item.»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «A CI step runs a client as `nobody` and as root against a test-spawned engine's socket (`Door refuses another account`).»
  source: .andromeda/security-plan.md:412 «runs two arms of `pulse-app/tests/integration_door.rs` — `another_account_is_refused_at_each_of_three_layers` and, since chunk `2026-10-12-stdio-agent-reaches-the-door`,»
  replace-with: «A CI step (`Door refuses another account`) runs two arms, each as `nobody` and as root against a test-spawned engine's socket: a `python3` client (`another_account_is_refused_at_each_of_three_layers`) and, since chunk `2026-10-12-stdio-agent-reaches-the-door`, a copy of the built `andromeda-pulse-engine` run as `door` (`another_account_through_the_bridge_receives_nothing`); it passes only on both arms' sentences, and either arm's skip sentence fails it.»

- leaf: .claude/docs/security-summary.md:27   class: stale
  leaf-text: «a closed two-word grammar (`run` | `version`; any other command line prints usage on stderr and exits 2, creating nothing; no word takes a value, a path or an address)»
  source: .andromeda/security-plan.md:89 «the console program's command line is a closed three-word grammar (the third word, `door`, since chunk `2026-10-12-stdio-agent-reaches-the-door`, PROVISIONAL with the door's item)»
  replace-with: «a closed three-word grammar (`run` | `version` | `door`; any other command line prints `usage: andromeda-pulse-engine <run|version|door>` on stderr and exits 2, creating nothing; no word takes a value, a path or an address; `run` and `version` a bounded input on the operator's reading, while `door` — since chunk `2026-10-12-stdio-agent-reaches-the-door`, the stdio relay to a running engine's door, booting no engine — is a boundary widening, PROVISIONAL with the door's item, vector 4a)»

- leaf: .claude/docs/security-summary.md:59   class: stale
  leaf-text: «addressed via user-facing warnings + visible clipboard-write event + README documentation rather than attempted sanitization.»
  source: .andromeda/security-plan.md:368 «the guidance a reader needs ("Using MCP with andromeda-pulse") is not written — no such document is in the tree»
  replace-with: «addressed via user-facing warnings + visible clipboard-write event rather than attempted sanitization; the "Using MCP with andromeda-pulse" guidance is not written (no such document is in the tree; owned by the route entry `Records say what the product is`), and since chunk `2026-10-12-stdio-agent-reaches-the-door` `andromeda-pulse-engine door` is a third carrier of tool answers, writing the door's answers unchanged to the standard output of the process that started it.»
