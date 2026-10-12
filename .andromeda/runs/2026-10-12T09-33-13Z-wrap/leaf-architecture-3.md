group: architecture-3
read: .claude/docs/services/viz.md · 55 · «- `.andromeda/obs-plan.md` §1 P1 + §5 Metric Coverage (`metr»
read: .claude/docs/services/workspace-detector.md · 68 · «- `.andromeda/obs-plan.md` §1 P7 (must-trace `app.boot.works»
read: .claude/docs/stack.md · 98 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 101 · «- For Tauri webview debugging in dev: `cargo tauri dev` open»
read: .claude/rules/frontend.md · 139 · «- 2026-09-29: UNDER vitest 4, `vi.restoreAllMocks()` NO LONG»

- leaf: .claude/docs/stack.md:12   class: omission
  leaf-text: «No Cargo or npm dependency, `Cargo.lock` byte-identical (PROVISIONAL with the door).»
  source: .andromeda/architecture.md:34 «Since chunk 2026-10-12-stdio-agent-reaches-the-door one more test binary, `unit_door_bridge`, needs Unix-domain sockets»
  replace-with: «No Cargo or npm dependency, `Cargo.lock` byte-identical. Since chunk 2026-10-12-stdio-agent-reaches-the-door one more test binary, `unit_door_bridge`, needs Unix-domain sockets (a 107-byte path reached, a 108-byte path made by hard-linking a socket bound at a short path), and a second arm of `integration_door`, `another_account_through_the_bridge_receives_nothing`, runs a copy of the built `andromeda-pulse-engine` as `door` under `sudo -n -u nobody` and `sudo -n`, so it needs a passwordless `sudo` (not `python3`); it too prints its skip sentence and passes where `sudo -n true` fails, and the `lint-test` step reads both arms' sentences and fails on either skip. Still no Cargo or npm dependency, `Cargo.lock` byte-identical (PROVISIONAL with the door).»

- leaf: .claude/docs/stack.md:21   class: omission
  leaf-text: «through the door with no feature — PROVISIONAL (arch §Occupied Resources → Network ports).»
  source: .andromeda/architecture.md:25 «since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door), also with no feature»
  replace-with: «through the door with no feature; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine through the console program's word `andromeda-pulse-engine door` (a relay to the door), also with no feature — so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine — PROVISIONAL (arch §Occupied Resources → Network ports, the door).»

- leaf: .claude/docs/stack.md:50   class: stale
  leaf-text: «N/A — each product program is a single process (the desktop window app; the console engine `andromeda-pulse-engine`).»
  source: .andromeda/architecture.md:29 «N/A — one engine process per program (the desktop window app; the console engine `andromeda-pulse-engine`), its parts joined by in-process tokio channels»
  replace-with: «N/A — one engine process per program (the desktop window app; the console engine `andromeda-pulse-engine`), its parts joined by in-process tokio channels; since chunk 2026-10-12-stdio-agent-reaches-the-door the console program's `door` word also runs as short-lived relay processes that host no engine part and join nothing (PROVISIONAL — arch §Occupied Resources → Network ports, the door).»

- leaf: .claude/docs/stack.md:72   class: stale
  leaf-text: «(`run` | `version`, no window, no display; not yet a background service or a bundled release artifact)»
  source: .andromeda/architecture.md:300 «driven by a closed three-word command line (`run` | `version` | `door`) with no window and no display — `door` (since chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL»
  replace-with: «(`run` | `version` | `door`, no window, no display — `door`, since chunk 2026-10-12-stdio-agent-reaches-the-door and PROVISIONAL, a stdio relay to the running engine's door that hosts no engine part; not yet a background service, a bundled release artifact, or a place where an incident forms without the deterministic gate set)»

- leaf: .claude/docs/workflow.md:31   class: tree
  leaf-text: «bumps `Cargo.toml` workspace version + updates `CHANGELOG.md`.»
  source: (tree) «no file matches **/CHANGELOG* outside target/ and node_modules/, tracked or untracked»
  replace-with: «bumps `Cargo.toml` workspace version.»

- leaf: .claude/docs/workflow.md:89   class: tree
  leaf-text: «`/wrap-session` curates session learnings + reconciles living artifacts (dependency-tree.md + api-surface.md) at session end.»
  source: (tree) «no file named dependency-tree.md or api-surface.md exists outside target/ and node_modules/, tracked or untracked»
  replace-with: «`/wrap-session` curates session learnings at session end.»
