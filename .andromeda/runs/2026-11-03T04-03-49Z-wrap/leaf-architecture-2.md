group: architecture-2
read: .claude/docs/services/mcp-server.md · 65 · «- `.andromeda/obs-plan.md` §1 P3 (must-trace `mcp.session` f»
read: .claude/docs/services/plugins.md · 73 · «- `.andromeda/obs-plan.md` §1 P4 (must-trace `plugin.lifecyc»
read: .claude/docs/services/snapshot.md · 67 · «- `.andromeda/design-system.md` §Motion Investigation Captur»
read: .claude/docs/services/triage.md · 48 · «- **Tests:** unit tests inline per sub-module; integration v»
read: .claude/docs/services/ui-bridge.md · 61 · «- `.andromeda/obs-plan.md` §3 Trace context propagation (IPC»
read: .claude/docs/services/viz.md · 55 · «- `.andromeda/obs-plan.md` §1 P1 + §5 Metric Coverage (`metr»
read: .claude/docs/services/workspace-detector.md · 68 · «- `.andromeda/obs-plan.md` §1 P7 (must-trace `app.boot.works»

- leaf: .claude/docs/services/mcp-server.md:1   class: stale
  leaf-text: «MCP stdio Sidecar (hand-rolled JSON-RPC 2.0)»
  source: fct-ahat5ytg «the nine tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by pulse-app in every build and served by the door; no binary, no feature»
  replace-with: «the nine MCP tools, served by the console engine's door (hand-rolled JSON-RPC 2.0)»

- leaf: .claude/docs/services/mcp-server.md:4   class: retired
  leaf-text: «Optional MCP server sidecar for AI agents to query telemetry directly. JSON-RPC 2.0 over stdin/stdout per MCP spec.»
  source: fct-bka6rc62 «`pulse-app` names `mcp-server` as a plain dependency linked in every build, with no feature gate and no stdio sidecar; the console engine serves the nine through the door»
  replace-with: «The nine MCP tools for AI agents to query telemetry directly, served inside the console engine's process by its door: line-delimited JSON-RPC 2.0 per MCP spec over the Unix-domain socket `run/door/sock`, reached over stdio through the relay `andromeda-pulse-engine door`.»

- leaf: .claude/docs/services/mcp-server.md:4   class: retired
  leaf-text: «The `mcp.{status,start,stop}` TauRPC router is hosted by `pulse-app/src/mcp_router.rs` (`McpApiImpl`), not by this crate, which supplies the `feature_gate` it reads. **Feature double-gated:** compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`.»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)»
  replace-with: «There is no `mcp.*` TauRPC router and no gate: no `mcp-server` feature, and `ANDROMEDA_PULSE_MCP_ENABLED` is read by no program.»

- leaf: .claude/docs/services/mcp-server.md:6   class: stale
  leaf-text: «**A second carrier since chunk 2026-10-10-door-inside-the-engine-s-process — the console engine's door (PROVISIONAL, the founder's own word owed).** `pulse-app` links this crate in every build (it is no longer an optional dependency; the feature `mcp-server` now gates the sidecar binary and `rmcp` only).»
  source: fct-rsvfsams «`mcp-server` is the plain `[dependencies.mcp-server-crate]` of `pulse-app`, linked in every build, with no feature.»
  replace-with: «**The one carrier — the console engine's door (PROVISIONAL, the founder's own word owed).** `pulse-app` names this crate as the plain `[dependencies.mcp-server-crate]`, linked in every build, with no feature (no sidecar binary, no `rmcp`).»

- leaf: .claude/docs/services/mcp-server.md:6   class: stale
  leaf-text: «It has two callers: `dispatch_tool` — the sidecar's path, signature and its three `mcp.*` records unchanged — and `door::EngineTools` (`pulse-app/src/door.rs`), which serves the nine tools inside `andromeda-pulse-engine run` over an owner-only Unix-domain socket, behind neither of the sidecar's gates, and records each call as `door.call`.»
  source: fct-rsvfsams «has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), the console engine's door, which writes `door.call`»
  replace-with: «It has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), which serves the nine tools inside `andromeda-pulse-engine run` over an owner-only Unix-domain socket, behind no feature gate, and records each call as `door.call`; no `mcp.*` record has an emitter in product code.»

- leaf: .claude/docs/services/mcp-server.md:6   class: stale
  leaf-text: «`tools_list_manifest()` gives every tool a boolean `write`, true for `mark_incident_resolved` alone — on both carriers, the one change the sidecar's own output took. The argument structs carry no `deny_unknown_fields`: the sidecar reads past an undeclared member (the manifest's `additionalProperties: false` is unenforced there), the door refuses one itself; no fix is built, the sidecar leaves at the route entry `Stdio sidecar retired`.»
  source: fct-exoqgaup «The manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`); the tools' argument structs read past one»
  replace-with: «`tools_list_manifest()` gives every tool a boolean `write`, true for `mark_incident_resolved` alone. The argument structs carry no `deny_unknown_fields` and read past an undeclared member; the manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`).»

- leaf: .claude/docs/services/mcp-server.md:6   class: stale
  leaf-text: «a stdio agent also reaches the nine through the door with no sidecar and no feature»
  source: fct-rsvfsams «A stdio agent reaches the nine through the door by starting `andromeda-pulse-engine door`»
  replace-with: «a stdio agent reaches the nine through the door»

- leaf: .claude/docs/services/mcp-server.md:6   class: stale
  leaf-text: «— so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine, and the ways in stay two (PROVISIONAL, with the door).»
  source: fct-rsvfsams «the engine has one way in besides telemetry, the door (socket and stdio relay)»
  replace-with: «— the one stdio way to the nine: the engine has one way in besides telemetry, the door (socket and stdio relay) (PROVISIONAL, with the door).»

- leaf: .claude/docs/services/mcp-server.md:16   class: retired
  leaf-text: «- stdout: JSON-RPC 2.0 framing (initialize / tools/list / tools/call / notifications/*).»
  source: fct-4nu3wh7b «the door's methods (`initialize` · `tools/list` · `tools/call` · `ping`, a request with no `id` answered with nothing) over the stdin/stdout of `andromeda-pulse-engine door`, the relay to `run/door/sock`»
  replace-with: «- The console engine's door: line-delimited JSON-RPC 2.0 (`initialize` · `tools/list` · `tools/call` · `ping`; a request with no `id` answered with nothing) over `run/door/sock`, and over the stdin/stdout of the relay `andromeda-pulse-engine door`.»

- leaf: .claude/docs/services/mcp-server.md:17   class: retired
  leaf-text: «- stderr: forced JSON `tracing` output (TTY check disabled — stdout reserved for protocol).»
  source: fct-qsvopkhj «The relay parses nothing, writes no answer of its own, retries nothing and has no time bound of its own; it writes no record and no file.»
  replace-with: «- stderr: nothing of this crate's (it has no binary); the relay's one standard-error line is the fixed `andromeda-pulse-engine door: the door cannot be reached`.»

- leaf: .claude/docs/services/mcp-server.md:18   class: retired
  leaf-text: «- TauRPC routers (visible from the window app only — the console program `andromeda-pulse-engine` mounts no TauRPC router): `mcp.status`, `mcp.start`, `mcp.stop`. The sidecar's `run/workspace-key` is published at boot by either engine program through the shared `engine_boot::start`.»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)»
  replace-with: «- TauRPC routers: none — the crate mounts no TauRPC router (no `mcp.*` procedures); the door reads no `run/workspace-key` and runs the tools over the engine's own `incident_workspace_key`.»

- leaf: .claude/docs/services/mcp-server.md:19   class: retired
  leaf-text: «- `tracing` events: `mcp.session`, `mcp.tools.call.request`, `mcp.tools.call.response`, `mcp.feature.gate.check`.»
  source: fct-rsvfsams «the console engine's door, which writes `door.call`; no `mcp.*` record has an emitter in product code»
  replace-with: «- `tracing` events: none of this crate's own — `call_tool` writes no log record; the door records `app.boot.door`, `door.call`, `door.refused`; no `mcp.*` record has an emitter in product code.»

- leaf: .claude/docs/services/mcp-server.md:22   class: retired
  leaf-text: «- `rmcp` — feature-gated ANCHOR dependency only (requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29; sole source contact `use rmcp as _;` in the bin — the protocol layer is hand-rolled, not rmcp-provided). The old 1.5.0-vs-0.3.x reconciliation is CLOSED by measurement.»
  source: fct-rsvfsams «the wire contract is MCP-spec-conformant, and no manifest and no `Cargo.lock` entry names `rmcp`»
  replace-with: «- No `rmcp`: no manifest and no `Cargo.lock` entry names it — the protocol layer is hand-rolled. The old 1.5.0-vs-0.3.x reconciliation is CLOSED by measurement.»

- leaf: .claude/docs/services/mcp-server.md:28   class: retired
  leaf-text: «- **Feature gate (compile-time):** `--features mcp-server` adds `rmcp` to the build graph and builds the sidecar binary; `rmcp` only enters when the feature is enabled per arch §Cross-cutting Patterns Feature-gate hygiene. The crate itself is not flag-specific since chunk 2026-10-10-door-inside-the-engine-s-process (`pulse-app` links it in every build). Both gates below are the sidecar's alone.»
  source: fct-dm325geu «The workspace crate `mcp-server` is not flag-specific — `pulse-app` names it as a plain dependency and links it in every build (the console engine's door calls its dispatch).»
  replace-with: «- **No feature gate:** the crate is not flag-specific — `pulse-app` names it as a plain dependency and links it in every build (the console engine's door calls its dispatch), per arch §Cross-cutting Patterns Feature-gate hygiene; there is no `mcp-server` feature, no sidecar binary and no `rmcp`.»

- leaf: .claude/docs/services/mcp-server.md:29   class: retired
  leaf-text: «- **Runtime gate:** `ANDROMEDA_PULSE_MCP_ENABLED=true` — default off. `false` AND/OR feature absent = sidecar not spawned.»
  source: fct-g7bqq44n «`ANDROMEDA_PULSE_MCP_ENABLED` — N/A: read by no program; the MCP surface has no feature gate and no switch of its own (the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE`, below)»
  replace-with: «- **No runtime gate:** `ANDROMEDA_PULSE_MCP_ENABLED` is read by no program; the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE` — `mark_incident_resolved` answers `-32001` unless it reads on.»

- leaf: .claude/docs/services/mcp-server.md:30   class: retired
  leaf-text: «- **Graceful degrade:** env var set against binary built without feature → log `warn` naming the missing feature flag and proceed (do NOT regress to fail-startup).»
  source: fct-g7bqq44n «`ANDROMEDA_PULSE_MCP_ENABLED` — N/A: read by no program; the MCP surface has no feature gate and no switch of its own (the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE`, below)»
  replace-with: «- **Graceful degrade:** N/A — no feature and no MCP switch exist to mismatch; the door answers a fault with a fixed-message JSON-RPC error (`-32603` tool failed), never the tool's own text.»

- leaf: .claude/docs/services/mcp-server.md:31   class: retired
  leaf-text: «- **Stdout discipline:** stdout reserved for JSON-RPC 2.0 framing. ANY accidental `println!` / `dbg!` / library stdout write corrupts protocol → silently disconnects MCP client.»
  source: fct-qsvopkhj «the whole answer read until the door closes, and only then written to standard output unchanged»
  replace-with: «- **Stdout discipline:** the crate has no binary and owns no stdout; on the stdio path the relay `andromeda-pulse-engine door` writes the door's answer lines to standard output unchanged and nothing of its own.»

- leaf: .claude/docs/services/mcp-server.md:32   class: retired
  leaf-text: «- **Stderr forced JSON:** `tracing-subscriber::fmt::layer().json().with_writer(std::io::stderr)` with TTY check disabled in this binary.»
  source: fct-ahat5ytg «the nine tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by pulse-app in every build and served by the door; no binary, no feature»
  replace-with: «- **Records:** `call_tool` writes no log record; the door records each answered call as `door.call`, closed labels only.»

- leaf: .claude/docs/services/mcp-server.md:39   class: retired
  leaf-text: «- **Sidecar binary identity:** distinct from `pulse-app` — `andromeda-pulse-mcp` (per arch §Occupied Resources Process / service identity).»
  source: fct-wuyupllp «MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)»
  replace-with: «- **No sidecar binary:** `andromeda-pulse-mcp` is built by no workspace target; a stdio MCP client starts `andromeda-pulse-engine door` (per arch §Occupied Resources Process / service identity). The door's `initialize` still reads `serverInfo.name` `andromeda-pulse-mcp`.»

- leaf: .claude/docs/services/mcp-server.md:40   class: retired
  leaf-text: «- **rmcp is declared-but-unused** — the sidecar links rmcp under the feature flag (anchor import) but no rmcp API executes; the MCP protocol is the hand-rolled `jsonrpc.rs` layer (MCP protocol `2024-11-05`). Measured at chunk 2026-08-29-advisory-backlog, which also closed the old 1.5.0-vs-0.3.x reconciliation (pinned `"3"`, resolved 3.1.4).»
  source: fct-x56az6wm «no feature gate, no stdio sidecar and no `rmcp` (the old 1.5.0-vs-0.3.x open question is CLOSED by measurement — chunk `2026-08-29-advisory-backlog`)»
  replace-with: «- **No `rmcp`** — no manifest and no `Cargo.lock` entry names it; the MCP protocol is the hand-rolled `jsonrpc.rs` layer (MCP protocol `2024-11-05`). The old 1.5.0-vs-0.3.x question is CLOSED by measurement (chunk 2026-08-29-advisory-backlog).»

- leaf: .claude/docs/services/mcp-server.md:44   class: retired
  leaf-text: «- **Sidecar entrypoint:** `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (the binary; gated by `--features mcp-server`, `required-features` on the `[[bin]]`)»
  source: fct-ahat5ytg «the nine tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by pulse-app in every build and served by the door; no binary, no feature»
  replace-with: «- **Carrier entrypoints (not this crate):** the door `pulse-app/src/door.rs` (`door::EngineTools`, `door::bind_endpoint`) and the stdio relay `pulse_app::door_bridge` (`andromeda-pulse-engine door`); this crate has no binary.»

- leaf: .claude/docs/services/mcp-server.md:45   class: stale
  leaf-text: «called by the sidecar's `dispatch_tool` and by the door's `door::EngineTools`»
  source: fct-rsvfsams «has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), the console engine's door, which writes `door.call`»
  replace-with: «called by the door's `door::EngineTools`, its one product caller»

- leaf: .claude/docs/services/mcp-server.md:45   class: stale
  leaf-text: «(read/write `corpus/corpus.db` cross-process)»
  source: fct-3k7t2g4l «Its one carrier is the console engine's door, in the engine's own process, over the engine's own `Arc<dyn CorpusWriter>`»
  replace-with: «(read/write the corpus in the console engine's own process, over the engine's own `Arc<dyn CorpusWriter>`)»

- leaf: .claude/docs/services/mcp-server.md:47   class: retired
  leaf-text: «- **Feature gate check:** `crates/mcp-server/src/feature_gate.rs` (compile-time + runtime double-gate)»
  source: fct-g7bqq44n «`ANDROMEDA_PULSE_MCP_ENABLED` — N/A: read by no program; the MCP surface has no feature gate and no switch of its own (the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE`, below)»
  replace-with: «- **Feature gate check:** none — the MCP surface has no feature gate and no switch of its own.»

- leaf: .claude/docs/services/mcp-server.md:48   class: retired
  leaf-text: «- **TauRPC router:** `pulse-app/src/mcp_router.rs` (`McpApiImpl`: mcp.status / mcp.start / mcp.stop; compiled only with `--features mcp-server`, mounted by the window app's `main.rs`)»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)»
  replace-with: «- **TauRPC router:** none — the `mcp-server` crate mounts no TauRPC router (no `mcp.*` procedures).»

- leaf: .claude/docs/services/mcp-server.md:53   class: retired
  leaf-text: «spawn app with `cargo build --bin pulse-app --features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true` → spawn sidecar via `tokio::process::Command` with stdin/stdout piped»
  source: fct-wuyupllp «MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)»
  replace-with: «boot `andromeda-pulse-engine run` → reach its door through `run/door/sock` or by spawning `andromeda-pulse-engine door` with stdin/stdout piped»

- leaf: .claude/docs/services/mcp-server.md:54   class: retired
  leaf-text: «- **Negative tests:** spawn without feature flag → build fails OR runtime detects missing binary; runtime env var unset → assert sidecar not spawned.»
  source: fct-qsvopkhj «`mark_incident_resolved` answers `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on»
  replace-with: «- **Negative tests:** another account is refused at the door; `mark_incident_resolved` answers `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on; an undeclared argument member answers `-32602`.»

- leaf: .claude/docs/services/mcp-server.md:57   class: retired
  leaf-text: «- **Build with MCP:** `cargo build --bin pulse-app --features mcp-server`»
  source: fct-z32zdd7c «no step names `--features mcp-server` and no job is `mcp-test`»
  replace-with: «- **Build:** `cargo build --bin andromeda-pulse-engine` — the nine tools are linked in every build, no feature.»

- leaf: .claude/docs/services/mcp-server.md:58   class: retired
  leaf-text: «- **Run with MCP enabled:** `ANDROMEDA_PULSE_MCP_ENABLED=true cargo run --bin pulse-app --features mcp-server`»
  source: fct-hgj34est «console program only, Unix only, in every build (no feature gate)»
  replace-with: «- **Run with the door:** `cargo run --bin andromeda-pulse-engine -- run` — the door opens at `run/door/sock` under the data dir (console program only, Unix only).»

- leaf: .claude/docs/services/mcp-server.md:59   class: retired
  leaf-text: «- **Test sidecar interactively:** `echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/debug/andromeda-pulse-mcp` → expect tools array on stdout.»
  source: fct-wuyupllp «MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)»
  replace-with: «- **Test interactively:** with a console engine running on the same data dir, `echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/debug/andromeda-pulse-engine door` → expect tools array on stdout.»

- leaf: .claude/docs/services/mcp-server.md:62   class: stale
  leaf-text: «rmcp anchor dep — amended 2026-08-29»
  source: fct-rsvfsams «Hand-rolled serde JSON-RPC 2.0, served by the console engine's door; no `rmcp`»
  replace-with: «served by the door, no `rmcp`»

- leaf: .claude/docs/services/ui-bridge.md:11   class: retired
  leaf-text: «- `ready` reports eight checks: duckdb_connection / ingest_mpsc_capacity_pct / broadcast_subscribers / plugins_loaded / mcp_server_enabled / rows_ingested / buffer_used_seconds / retention_seconds.»
  source: fct-y2ufsarj «When not ready, `ready` is `false` and any failing check is the offending reason string (e.g., `"duckdb_connection": "init_in_progress"`).»
  replace-with: «- `ready` reports whether the app is ready to ingest and serve queries; when not, `ready` is `false` and any failing check is the offending reason string (e.g., `"duckdb_connection": "init_in_progress"`).»

- leaf: .claude/docs/services/ui-bridge.md:17   class: retired
  leaf-text: «`plugins.*`, `mcp.*` (feature-gated), `connection.*`»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)»
  replace-with: «`plugins.*`, `connection.*`»

- leaf: .claude/docs/services/ui-bridge.md:54   class: retired
  leaf-text: «(run it with `--features mcp-server` last, so the committed mcp shape stands)»
  source: fct-hvcuzwfp «Cargo features are `kebab-case` (`otap-ingest`); default features are minimal (no `otap-ingest`).»
  replace-with: «(one shape in every build — there is no `mcp-server` feature)»

- leaf: .claude/docs/services/workspace-detector.md:4   class: retired
  leaf-text: «and publishes `run/workspace-key` for both programs) and read cross-process by the `andromeda-pulse-mcp` sidecar.»
  source: fct-qsvopkhj «the door opens no database and no corpus, reads no `run/workspace-key` and prepares no SQL»
  replace-with: «for both programs); the door reads no `run/workspace-key` and runs the tools over the engine's own `incident_workspace_key`.»
