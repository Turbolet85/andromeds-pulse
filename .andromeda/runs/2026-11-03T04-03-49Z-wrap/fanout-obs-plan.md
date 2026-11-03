proposals:
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Replace the Two carriers paragraph with ONE carrier since chunk 2026-11-03-stdio-sidecar-retired: the door inside the console engine (`andromeda-pulse-engine run`; JSON-RPC 2.0 over `{data dir}/run/door/sock`, reached directly or by a stdio agent through `andromeda-pulse-engine door`); `tools/call` runs `mcp_server::tools::call_tool` over `door::EngineTools` (its one product caller); its record per answered call is `door.call` {method, tool, outcome, write, result_type, result_count, answer_bytes, duration_ms}, refusals `door.refused` {reason, peer}; keep the relay, idle/timeout and PROVISIONAL sentences; drop the stdio sidecar, its `mcp.tools.call.request` → `mcp.feature.gate.check` → `mcp.tools.call.response` chain and `dispatch_tool` as standing (retired with `andromeda-pulse-mcp`, no `mcp.*` record has an emitter)"
    sidecar: "§4 P3: the stdio sidecar carrier retired; the door is the one carrier of a tool call, `door.call` its record"
    rationale: "Report Retired names the stdio sidecar, its double gate and the `mcp.*` record chain; Symbols states `dispatch_tool` and its three records removed, `call_tool` has one product caller `pulse-app/src/door.rs`, and no `mcp.*` record has an emitter in product code"
    basis: "obs-plan fct-4vrfwv27; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Surfaces involved: the console engine's door (Unix-domain socket, console program only), reached over its socket or by a stdio agent through `andromeda-pulse-engine door`, plus buffer (DuckDB) and the incident corpus; drop CLI (hand-rolled JSON-RPC stdio) and the door as the second carrier"
    sidecar: "§4 P3 surfaces: the stdio sidecar surface retired; the door alone"
    rationale: "Report Retired: the stdio sidecar `andromeda-pulse-mcp`; Symbols: the product binaries are now `pulse-app` and `andromeda-pulse-engine`"
    basis: "obs-plan fct-5gsdjdy6; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Must-trace: the door's `door.call` record (one per answered request, written before the answer line) over the per-tool read — `viz.query.traces` / `.metrics` / `.logs` for the telemetry tools, a corpus read for the incident tools (`retrieve_incident_events` emits no span of its own, its error surfacing as JSON-RPC -32603); the door emits no span; drop `mcp.tools.call.request`, `mcp.feature.gate.check`, `mcp.tools.call.response` and stdout emission"
    sidecar: "§4 P3 must-trace: the `mcp.*` span chain retired with the sidecar; `door.call` is the must-log record"
    rationale: "Report Symbols: `dispatch_tool` and `mcp.tools.call.response` removed; the sidecar's `mcp.session`, `mcp.tools.call.request`, `mcp.feature.gate.check` left with the binary; Retired names the `mcp.*` record chain"
    basis: "obs-plan fct-4pvtx7no; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Required attributes: `door.call` {method (`initialize` | `tools/list` | `tools/call` | `ping` | `other` | `none`), tool (one of the nine `ALL_TOOL_NAMES` | `none`), outcome, write, result_type (not the answer; `incident_events` for `retrieve_incident_events`), result_count, answer_bytes, duration_ms}; the per-tool query records keep `query_id`, `param_count`, `time_range_start`, `time_range_end`, `service_filter`; drop the `mcp.tools.call.request`, `mcp.feature.gate.check` (`feature_flag_enabled`, `env_var_mcp_enabled`) and `mcp.tools.call.response` attribute sets"
    sidecar: "§4 P3 attributes: the `mcp.*` attribute sets and the two gate fields retired; `door.call` fields stand"
    rationale: "Report Retired: the double gate and the `mcp.*` record chain; Schema: the `mcp-server` allowlist leaf `{method, result_type, result_count, feature_flag_enabled, duration_ms}` removed, the door's three leaves unchanged"
    basis: "obs-plan fct-nunu6scu; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Required log fields: the `door.call` field set (method, tool, outcome, write, result_type, result_count, answer_bytes, duration_ms) and, on refusal, `door.refused` (reason, peer); drop `feature_flag_enabled` and `env_var_mcp_enabled`"
    sidecar: "§4 P3 log fields: the gate fields retired with the double gate"
    rationale: "Report Retired: the double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`); Symbols: the env var removed from every reader"
    basis: "obs-plan fct-43wzevug; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3
    change: "Cleanup: the door closes an idle connection at 30 s and answers a call over 5 s with -32002; the relay shuts its write side after each request line and exits after writing the answer; drop the hand-rolled `jsonrpc.rs` stdio loop closing the stream and spans ending after the stdout result"
    sidecar: "§4 P3 cleanup: the sidecar's stdio loop retired; the door's connection bounds stand"
    rationale: "Report Retired: the stdio sidecar `andromeda-pulse-mcp` (its binary deleted, `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs`); the door untouched"
    basis: "obs-plan fct-rtszlvuo; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Justification: 8 major surfaces (desktop-webview, desktop-native, IPC-internal, API service gRPC, API service HTTP, CLI, library-only plugin host, and the console engine's door — JSON-RPC 2.0 over a Unix-domain socket, console program only, reached by a stdio agent through `andromeda-pulse-engine door`); the MCP stdio sidecar surface retired at chunk 2026-11-03-stdio-sidecar-retired; `mcp-server` stays an instrumentable module as the tools library the door serves"
    sidecar: "§1 surface count 9 → 8: MCP stdio retired, the door the tool surface"
    rationale: "Report Retired: the stdio sidecar; Crates: `mcp-server` stays a workspace member and the tools library the door serves; Counts: engine ways in besides telemetry 2 → 1, the door"
    basis: "obs-plan fct-o4vhpvgv; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yn7uwxjo, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Instrumentation-scope `mcp-server` row: the tools library the door serves (`pulse-app` names it as a plain dependency, no cargo feature); boundary-only, instrumented at the door (`door.call` / `door.refused`), the library writing no record of its own; tool response bodies never logged (security plan vector 4); drop the feature-gated stdin JSON-RPC stdio boundary"
    sidecar: "§1 mcp-server entity: no longer a feature-gated stdio surface; the door is its boundary"
    rationale: "Report Schema: cargo feature `mcp-server` removed from `crates/mcp-server` and `pulse-app`; Crates: `pulse-app` names it as the plain `[dependencies.mcp-server-crate]`; Symbols: no `mcp.*` record has an emitter"
    basis: "obs-plan fct-7woyixer; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx, fct-yn7uwxjo, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Critical path P3 row: carried by the door alone — must-log `door.call` (method, tool, outcome, write, result_type, result_count, answer_bytes, duration_ms) and `door.refused` (reason, peer) over the per-tool reads; no `mcp.*` span and no gate; drop the `mcp.*` spans, parent `mcp.session`, the `feature_flag_enabled` / `env_var_mcp_enabled` fields and the sentence that they are the stdio sidecar's; the test excerpt spawning the app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` and the double-gate anti-pattern stand only as history"
    sidecar: "§1 P3 critical path: the sidecar's spans and gate fields retired; the door's records stand"
    rationale: "Report Retired: the sidecar, its double gate and the `mcp.*` record chain; Symbols: `mcp.session`, `mcp.tools.call.request`, `mcp.feature.gate.check` left with the binary"
    basis: "obs-plan fct-sjvcbexl; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Snapshot / paste-to-AI bullet: four of the nine MCP tools served by the console engine's door (`query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`), over its socket or the `andromeda-pulse-engine door` stdio relay, expose the same pipeline to AI clients; drop the sidecar's tools over the hand-rolled JSON-RPC stdio layer"
    sidecar: "§1 snapshot bullet: the nine tools are the door's, not the sidecar's"
    rationale: "Report Symbols: `call_tool` has one product caller left, `pulse-app/src/door.rs` (`EngineTools::call`); Retired: the stdio sidecar"
    basis: "obs-plan fct-ohds4uw3; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Telemetry trigger Vector 4 row: the console engine's door is the one enforcement site — `door.call` carries `result_type`, `result_count` and `answer_bytes`, no argument, no answer and no error text, every refusal a fixed message; its negative reading the marker returned by `query_logs` 0 times in the engine's log family; drop the `mcp-server` boundary `mcp.tools.call.response` span as standing (retired with the sidecar)"
    sidecar: "§1 Vector 4 trigger: `mcp.tools.call.response` retired; the door the one site"
    rationale: "Report Symbols: `mcp.tools.call.response` removed with `dispatch_tool`; Coverage: the four rewritten door tests, PII the marker 0× in the log"
    basis: "obs-plan fct-dfxkc45d; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-vdoco5gy"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage
    change: "Span kind `server`: inbound handlers `ingest.grpc.export.request`, `ingest.http.export.request` and TauRPC command handlers; drop MCP tool invocations (the door records a call as the log event `door.call` and opens no span)"
    sidecar: "§4 span kind: MCP tool-invocation spans retired with the sidecar"
    rationale: "Report Symbols: no `mcp.*` record has an emitter in product code; the door untouched (three log events, no span)"
    basis: "obs-plan fct-qk3ssznn, fct-rq75gb6d; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage
    change: "Instrumentation-per-surface table: remove the `MCP stdio (hand-rolled JSON-RPC 2.0, rmcp 3.x declared-but-unused, feature-gated)` row — its double gate, its stderr-forced writer and its `mcp.tools.call.request` / `mcp.feature.gate.check` / `mcp.response.emit` spans retired at chunk 2026-11-03-stdio-sidecar-retired; the door row is the tool-call surface"
    sidecar: "§4 surfaces: MCP stdio row removed (sidecar, gate, stderr sink, `mcp.*` spans retired)"
    rationale: "Report Retired: the sidecar, its double gate, its stderr-JSON sink and the `mcp.*` chain; Dependencies: `rmcp` removed from the workspace and `Cargo.lock`"
    basis: "obs-plan fct-vxhgwrdg; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yabevdww"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P7
    change: "Must-trace spans end at `workspace.identity.resolve`; drop `app.boot.workspace_key` (the publish of the resolved key to `{data_dir}/run/workspace-key` for the MCP sidecar) — neither program writes the file or the record since chunk 2026-11-03-stdio-sidecar-retired; `incident_workspace_key` still stamps incidents"
    sidecar: "§4 P7: the `run/workspace-key` publish and `app.boot.workspace_key` record retired"
    rationale: "Report Symbols: `publish_workspace_key_for_sidecar` and its call removed, neither program writes `{data dir}/run/workspace-key` or `app.boot.workspace_key`; Outcome (obs) met on two window boots and an engine cycle"
    basis: "obs-plan fct-gcfverax; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-dv3watly"
  - detector: D-retired
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P7
    change: "Required log fields: `trace_id`, `workspace.root_path`, `workspace.project_name`, `vcs_type`, `vcs_root`, `marker_present`, `detection_latency_ms`; drop the `app.boot.workspace_key` clause"
    sidecar: "§4 P7 log fields: the workspace-key record's fields retired"
    rationale: "Report Retired: the `run/workspace-key` publish; Schema: the `app.boot.workspace_key` allowlist leaf removed"
    basis: "obs-plan fct-gejrrwfx; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Critical path P7 row: drop `app.boot.workspace_key` (publish for the sidecar) from the must-trace spans and its `workspace_root_basename` + `key_bytes` clause from the required fields"
    sidecar: "§1 P7 critical path: the workspace-key publish retired"
    rationale: "Report Symbols: neither program writes `{data dir}/run/workspace-key` or the `app.boot.workspace_key` record"
    basis: "obs-plan fct-6mzzyb6t; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §8 PII Scrubbing & Compliance
    change: "Remove the `app.boot.workspace_key` allowlist entry: the leaf is removed and no program emits the record since chunk 2026-11-03-stdio-sidecar-retired"
    sidecar: "§8: `app.boot.workspace_key` leaf removed with its emitter"
    rationale: "Report Schema: allowlist leaves removed — `app.boot.workspace_key`; Symbols: the publish and its record removed"
    basis: "obs-plan fct-oodqyure; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §8 PII Scrubbing & Compliance
    change: "Remove the `mcp-server` whitelist entry (`method`, `result_type`, `result_count`, `feature_flag_enabled`, `duration_ms`): the leaf is removed with the sidecar's records; a tool call's fields are the `door.call` leaf's"
    sidecar: "§8: `mcp-server` allowlist leaf removed; the door's leaves stand"
    rationale: "Report Schema: allowlist leaves removed — `mcp-server` `{method, result_type, result_count, feature_flag_enabled, duration_ms}`; the door's three leaves unchanged"
    basis: "obs-plan fct-z6icbp7d; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §8 PII Scrubbing & Compliance
    change: "`triage.incident.persist` entry: state the `reconciled_count` writer as history — at chunk 2026-08-29-app-registry-reconciliation the outside writer was the `andromeda-pulse-mcp` sidecar, retired at chunk 2026-11-03-stdio-sidecar-retired; `mark_incident_resolved` now reaches the corpus through the console engine's door"
    sidecar: "§8 persist leaf: the sidecar named as the reconciled writer only as history"
    rationale: "Report Retired: the stdio sidecar; Symbols: `mark_incident_resolved` reached through `call_tool`, whose one product caller is the door, its description now the running engine dropping it within one persist cycle"
    basis: "obs-plan fct-avpvhn3k; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §6 Log Coverage
    change: "Log-levels warn row: drop the `mcp.gate.check` warn example (MCP enabled but feature flag absent) (the gate and its record retired with the sidecar); the other warn examples stand"
    sidecar: "§6 warn row: `mcp.gate.check` example retired"
    rationale: "Report Symbols: `mcp.feature.gate.check` / `mcp.gate.check` left with the binary and `tracing_setup.rs`"
    basis: "obs-plan fct-fzliht4o; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §6 Log Coverage
    change: "Per-module level `mcp-server` row: the tools library the door serves writes no record of its own; a tool call is logged by the door as `door.call` (INFO for `ok`, WARN otherwise); drop (stdio sidecar) and the JSON-RPC request-parse debug level"
    sidecar: "§6 per-module: mcp-server no longer a sidecar process with its own levels"
    rationale: "Report Crates: `mcp-server` stays the tools library the door serves; Symbols: no `mcp.*` record has an emitter in product code"
    basis: "obs-plan fct-hbfmcroa; doc:2026-11-03-stdio-sidecar-retired/report fct-yn7uwxjo, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §6 Log Coverage
    change: "Sink configuration: drop the MCP server stderr-JSON-only sink; both programs have the one JSON file sink and the console `door` word installs none"
    sidecar: "§6 sinks: the sidecar's stderr-JSON sink retired"
    rationale: "Report Retired: the sidecar's stderr-JSON sink; Symbols: `mcp_server::tracing_setup` removed"
    basis: "obs-plan fct-jektn265; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §3 → Logging stack
    change: "Sink: single file sink for both programs (`~/.andromeda-pulse/logs/agent-latest.jsonl`, JSON-only, no stderr layer); the console `door` word has no sink; drop the mcp-server sidecar sentence (rmcp 3.x declared-but-unused, stderr forced JSON) — the sidecar, its stderr-JSON sink and `rmcp` left at chunk 2026-11-03-stdio-sidecar-retired"
    sidecar: "§3 Logging stack: the sidecar's stderr-JSON sink and the rmcp anchor retired"
    rationale: "Report Retired: the sidecar's stderr-JSON sink; Dependencies: `rmcp` removed, `mcp-server` dropped `tracing-subscriber`, `tracing-appender`, `tracing-error`"
    basis: "obs-plan fct-65zmgzbk; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yabevdww"
  - detector: D-retired
    severity: warning
    section: §1 Obs Scope Summary
    change: "Harness-spec Logging stack bullet: the app has a single sink, the JSON file; drop the mcp-server sidecar clause (rmcp 3.x resolved 3.1.4 declared-but-unused anchor, stderr forced JSON); keep the `door` word writing no `tracing` output"
    sidecar: "§1 harness spec: the sidecar sink and rmcp anchor retired"
    rationale: "Report Retired: the stderr-JSON sink; Dependencies: `rmcp` and `rmcp-macros` left `Cargo.lock`"
    basis: "obs-plan fct-d2dydmme; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yabevdww"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §3 → Service identity
    change: "service.name: `com.andromeda.pulse` for every program; drop the mcp-server sidecar identity `andromeda-pulse-mcp` (the process is retired; `initialize`'s `serverInfo.name` still reads `andromeda-pulse-mcp`, a protocol value, not a service identity)"
    sidecar: "§3 Service identity: the sidecar's distinct identity retired"
    rationale: "Report Symbols: binary `andromeda-pulse-mcp` removed, product binaries are `pulse-app` and `andromeda-pulse-engine`; `serverInfo.name` unchanged"
    basis: "obs-plan fct-mfvbrkak; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §3 → Snapshot / paste-to-AI integration
    change: "MCP tools: the nine tools are served by the console engine's door through `mcp_server::tools::call_tool` (its one product caller `door::EngineTools`), each call recorded as `door.call` with `result_type` + `result_count` and never the answer (vector 4); over stdio means only the `andromeda-pulse-engine door` relay; drop the stdio sidecar's `mcp.tools.call.request` → `mcp.tools.call.response` at `dispatch_tool` and both carriers; the key's label (`obs-plan-contracts.toml`) reads MCP tools (hand-rolled JSON-RPC 2.0 through the door), without rmcp 3.x declared-but-unused"
    sidecar: "§3 Snapshot integration: the sidecar carrier and its `mcp.*` records retired; rmcp gone from the label"
    rationale: "Report Symbols: `dispatch_tool` and its records removed, `call_tool` one product caller `pulse-app/src/door.rs`; Dependencies: `rmcp` removed"
    basis: "obs-plan fct-qnzkp4df; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-yabevdww"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §11 Obs Anti-Patterns → Universal (agent-driven specific)
    change: "Keep the stdout ban for the console program's `door` word alone (stdout only the door's answer bytes, stderr only the fixed line on exit 1, no subscriber, no record); drop the `andromeda-pulse-mcp` stdio sidecar half and its forced-stderr tracing"
    sidecar: "§11 stdout ban: the sidecar half retired; the relay half stands"
    rationale: "Report Retired: the sidecar and its stderr-JSON sink; the door untouched"
    basis: "obs-plan fct-anuozml7; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §11 Obs Anti-Patterns → Project-specific (andromeda-pulse)
    change: "Retire the double-gate ban with the gate (no `--features mcp-server`, no `ANDROMEDA_PULSE_MCP_ENABLED`, no `mcp.feature.gate.check` since chunk 2026-11-03-stdio-sidecar-retired); the nine tools are reached only through the door, whose admission is the owner check on every accepted connection and whose state is read from `app.boot.door` and `door.refused` (PROVISIONAL — §4 Scenario P3); drop rmcp still enters only under the feature"
    sidecar: "§11 double-gate ban retired with the gate; the door's owner check stands"
    rationale: "Report Retired: the double gate; Schema: the `mcp-server` cargo feature removed; Dependencies: `rmcp` removed"
    basis: "obs-plan fct-udg32czu; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx, fct-yabevdww"
    dependent-of: D-retired
note: "D-obs-instrumentation, D-obs-stack, D-obs-pii and D-obs-defect-narrative found no drift (no new hot-path operation, nothing added or bumped, no logging of user data, the disproved claims are plan claims touching no §10 defect); rmcp corrections ride the D-retired proposals of the facts holding them; §10 snapshot row's mcp-server `dispatch_generate_snapshot` (fct-5gdcd34s) left as is, the report not naming it removed"
