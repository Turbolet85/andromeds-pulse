group: architecture-1
read: .claude/docs/conventions.md · 96 · «- Default to writing no comments — only when WHY is non-obvi»
read: .claude/docs/gotchas.md · 83 · «_Wrap-session appends new entries below as they emerge from »
read: .claude/docs/services/buffer.md · 61 · «- `.andromeda/obs-plan.md` §1 P1 (must-trace `duckdb.append`»
read: .claude/docs/services/config-watcher.md · 48 · «- `.claude/rules/security.md` — the Settings-extension patte»
read: .claude/docs/services/corpus.md · 45 · «- **Tests:** 88 corpus tests (as measured at chunk 2026-10-0»
read: .claude/docs/services/curation.md · 32 · «- **Tests:** unit tests inline with each primitive; integrat»
read: .claude/docs/services/ingest.md · 60 · «- `.andromeda/obs-plan.md` §3 (tracing instrumentation patte»
read: .claude/docs/services/interpretation.md · 59 · «- `.claude/rules/observability.md` — allowlist-leaf discipli»

- leaf: .claude/docs/conventions.md:21   class: retired
  leaf-text: «`plugins.invoke`, `mcp.status`, `mcp.start`, `mcp.stop`, `workspace.detect`»
  source: fct-qiuibtut «no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)»
  replace-with: «`plugins.invoke`, `workspace.detect`»

- leaf: .claude/docs/conventions.md:23   class: stale
  leaf-text: «spec-fixed JSON-RPC 2.0 method names — `initialize`, `tools/list`, `tools/call`, `notifications/*`.»
  source: fct-4nu3wh7b «the door's methods (`initialize` · `tools/list` · `tools/call` · `ping`, a request with no `id` answered with nothing) over the stdin/stdout of `andromeda-pulse-engine door`»
  replace-with: «spec-fixed JSON-RPC 2.0 method names — `initialize`, `tools/list`, `tools/call`, `ping` (a request with no `id` answered with nothing).»

- leaf: .claude/docs/conventions.md:23   class: retired
  leaf-text: «Over stdio per MCP spec is the sidecar's wire; since chunk 2026-10-10-door-inside-the-engine-s-process the same nine tool methods have a second wire, the console engine's door — line-delimited JSON-RPC 2.0 over the Unix-domain socket `run/door/sock`, methods `initialize` · `tools/list` · `tools/call` · `ping` (arch §Standard Contracts → Engine door; PROVISIONAL). Since chunk 2026-10-12-stdio-agent-reaches-the-door that wire is also reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request; "over stdio per MCP spec" stays the sidecar's wire.»
  source: fct-ob4ts4xm «The wire is the console engine's door — line-delimited JSON-RPC 2.0 over the Unix-domain socket `run/door/sock`, methods `initialize` · `tools/list` · `tools/call` · `ping`»
  replace-with: «The wire is the console engine's door — line-delimited JSON-RPC 2.0 over the Unix-domain socket `run/door/sock`, methods `initialize` · `tools/list` · `tools/call` · `ping` (arch §Standard Contracts → Engine door; PROVISIONAL). That wire is reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request.»

- leaf: .claude/docs/conventions.md:51   class: retired
  leaf-text: «`kebab-case` (`mcp-server`, `otap-ingest`)»
  source: fct-hvcuzwfp «Cargo features are `kebab-case` (`otap-ingest`); default features are minimal (no `otap-ingest`).»
  replace-with: «`kebab-case` (`otap-ingest`)»

- leaf: .claude/docs/conventions.md:52   class: retired
  leaf-text: «minimal (no `mcp-server`, no `otap-ingest`)»
  source: fct-hvcuzwfp «Cargo features are `kebab-case` (`otap-ingest`); default features are minimal (no `otap-ingest`).»
  replace-with: «minimal (no `otap-ingest`)»

- leaf: .claude/docs/gotchas.md:11   class: retired
  leaf-text: «the published line reached 3.x and the workspace pins requirement `"3"` (lockfile-resolved 3.1.4, chunk 2026-08-29-advisory-backlog). Note the measured mechanism while touching rmcp: the crate is a feature-gated ANCHOR dependency (`use rmcp as _;` is the sole source contact)»
  source: fct-rsvfsams «the protocol is the hand-rolled layer at `crates/mcp-server/src/jsonrpc.rs` (MCP protocol `2024-11-05`) with name dispatch in `tools.rs`; the wire contract is MCP-spec-conformant, and no manifest and no `Cargo.lock` entry names `rmcp`»
  replace-with: «the published line reached 3.x (chunk 2026-08-29-advisory-backlog). `rmcp` is no longer linked: no manifest and no `Cargo.lock` entry names it»

- leaf: .claude/docs/gotchas.md:46   class: stale
  leaf-text: «## The door is a second way to the tools (PROVISIONAL)»
  source: fct-rsvfsams «behind no feature gate; the engine has one way in besides telemetry, the door (socket and stdio relay).»
  replace-with: «## The door is the one way to the tools (PROVISIONAL)»

- leaf: .claude/docs/gotchas.md:47   class: retired
  leaf-text: «the console engine serves the sidecar's nine tools over an owner-only Unix-domain socket (`{data dir}/run/door/sock`), behind neither of the sidecar's gates: `pulse-app` links `mcp-server` in every build, and only `rmcp` and the sidecar binary stay behind `--features mcp-server`; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent also reaches the nine with no feature by starting `andromeda-pulse-engine door`, a relay to the door — so the feature gates the stdio sidecar, not every stdio way to the nine.»
  source: fct-x56az6wm «reached over stdio through `andromeda-pulse-engine door` (PROVISIONAL — §Occupied Resources → Network ports, the door); no feature gate, no stdio sidecar and no `rmcp`»
  replace-with: «the console engine serves the nine tools over an owner-only Unix-domain socket (`{data dir}/run/door/sock`), behind no feature gate: `pulse-app` links `mcp-server` in every build, and there is no stdio sidecar and no `rmcp`; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine by starting `andromeda-pulse-engine door`, a relay to the door — the one stdio way to the nine.»

- leaf: .claude/docs/gotchas.md:51   class: retired
  leaf-text: «## rmcp stdio reservation»
  source: fct-wuyupllp «MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)»
  replace-with: «## Stdio relay output»

- leaf: .claude/docs/gotchas.md:52   class: retired
  leaf-text: «When running as the `andromeda-pulse-mcp` stdio sidecar, stdout is reserved for JSON-RPC 2.0 framing.»
  source: fct-qsvopkhj «The relay parses nothing, writes no answer of its own, retries nothing and has no time bound of its own; it writes no record and no file.»
  replace-with: «There is no `andromeda-pulse-mcp` stdio sidecar: a stdio MCP client starts `andromeda-pulse-engine door`, whose stdout carries only the door's answer lines, unchanged — the relay parses nothing and writes no answer of its own.»

- leaf: .claude/docs/gotchas.md:54   class: retired
  leaf-text: «ALL `tracing` output forced to stderr (JSON formatter); TTY check disabled for the sidecar.»
  source: fct-qsvopkhj «exit 1 with the one fixed standard-error line `andromeda-pulse-engine door: the door cannot be reached` and nothing on standard output from the failed connection»
  replace-with: «The relay writes no log record and no file; a failure is exit 1 with the one fixed standard-error line `andromeda-pulse-engine door: the door cannot be reached` and nothing on stdout from the failed connection.»

- leaf: .claude/docs/services/corpus.md:14   class: retired
  leaf-text: «the `mark_incident_resolved` tool calling `CorpusWriter` directly: cross-process in the `andromeda-pulse-mcp` sidecar and, since chunk 2026-10-10-door-inside-the-engine-s-process, in the engine's own process through the console engine's door,»
  source: fct-3k7t2g4l «Its one carrier is the console engine's door, in the engine's own process, over the engine's own `Arc<dyn CorpusWriter>`»
  replace-with: «the `mark_incident_resolved` tool calling `CorpusWriter` directly, whose one carrier is the console engine's door, in the engine's own process,»

- leaf: .claude/docs/services/corpus.md:26   class: retired
  leaf-text: «consumed by the `retrieve_incident_events` tool — served by the MCP sidecar (chunk 2026-10-02-incident-events-readable-through-mcp) and, since chunk 2026-10-10-door-inside-the-engine-s-process, by the console engine's door.»
  source: fct-cgszvbwl «Readers, one: the `retrieve_incident_events` tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`)»
  replace-with: «consumed by the `retrieve_incident_events` tool (chunk 2026-10-02-incident-events-readable-through-mcp), called through the console engine's door.»

- leaf: .claude/docs/services/corpus.md:35   class: retired
  leaf-text: «across processes the window app, the console program and the `andromeda-pulse-mcp` sidecar each open the same file through `Corpus::open`, the sidecar as a writer.»
  source: fct-ulzkjofc «Written by the window app and the console program `andromeda-pulse-engine`, both through `Corpus::open` from the shared `engine_boot::start`»
  replace-with: «across processes the window app and the console program `andromeda-pulse-engine` each open the same file through `Corpus::open`.»
