group: architecture-1
read: .claude/docs/commands.md · 190 · «- Views: `symbol` / `refs` / `calls` / `contains` / `crate_e»
read: .claude/docs/conventions.md · 96 · «- Default to writing no comments — only when WHY is non-obvi»
read: .claude/docs/gotchas.md · 83 · «_Wrap-session appends new entries below as they emerge from »
read: .claude/docs/services/buffer.md · 61 · «- `.andromeda/obs-plan.md` §1 P1 (must-trace `duckdb.append`»
read: .claude/docs/services/config-watcher.md · 48 · «- `.claude/rules/security.md` — the Settings-extension patte»
read: .claude/docs/services/corpus.md · 45 · «- **Tests:** 88 corpus tests (as measured at chunk 2026-10-0»
read: .claude/docs/services/curation.md · 32 · «- **Tests:** unit tests inline with each primitive; integrat»
read: .claude/docs/services/ingest.md · 60 · «- `.andromeda/obs-plan.md` §3 (tracing instrumentation patte»

- leaf: .claude/docs/commands.md:78   class: stale
  leaf-text: «# one line, exit 0; any other command line is usage, exit 2»
  source: .andromeda/architecture.md:217 «`door` is the stdio relay (`pulse_app::door_bridge::main`) — a client of a running console engine's `run/door/sock`, started by a stdio agent»
  replace-with: «# one line, exit 0; `door` (Unix; PROVISIONAL) is the stdio relay to a running engine's run/door/sock — no engine boot, exit 0 when standard input ends, 1 with one fixed stderr line when the door cannot be reached; any other command line is usage, exit 2»

- leaf: .claude/docs/conventions.md:23   class: omission
  leaf-text: «methods `initialize` · `tools/list` · `tools/call` · `ping` (arch §Standard Contracts → Engine door; PROVISIONAL).»
  source: .andromeda/architecture.md:85 «Since chunk 2026-10-12-stdio-agent-reaches-the-door that wire is also reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request»
  replace-with: «methods `initialize` · `tools/list` · `tools/call` · `ping` (arch §Standard Contracts → Engine door; PROVISIONAL). Since chunk 2026-10-12-stdio-agent-reaches-the-door that wire is also reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request; "over stdio per MCP spec" stays the sidecar's wire.»

- leaf: .claude/docs/gotchas.md:47   class: omission
  leaf-text: «and only `rmcp` and the sidecar binary stay behind `--features mcp-server`.»
  source: .andromeda/architecture.md:25 «so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine»
  replace-with: «and only `rmcp` and the sidecar binary stay behind `--features mcp-server`; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent also reaches the nine with no feature by starting `andromeda-pulse-engine door`, a relay to the door — so the feature gates the stdio sidecar, not every stdio way to the nine.»

- leaf: .claude/docs/services/corpus.md:10   class: stale
  leaf-text: «(macOS Keychain / Linux Secret Service / Windows DPAPI)»
  source: .andromeda/architecture.md:226 «(macOS Keychain / Linux Secret Service / Windows Credential Manager via the `keyring` crate)»
  replace-with: «(macOS Keychain / Linux Secret Service / Windows Credential Manager)»
