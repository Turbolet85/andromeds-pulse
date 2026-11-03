## 2026-11-03-stdio-sidecar-retired — §1 scope: eight surfaces, the door the one tool surface
**Section:** §1 Obs Scope Summary (the major-surfaces justification, the `mcp-server` instrumentation-scope row, the External-OTLP snapshot bullet, the P3 critical-path row, the logging-sensitive Vector 4 trigger row)
**Change:**
- Major surfaces read 8 (was 9): MCP stdio leaves; the console engine's door (JSON-RPC 2.0 over a Unix-domain socket, console program only) is also reached by a stdio agent through `andromeda-pulse-engine door`.
- The `mcp-server` row is the tools library the door serves (was the hand-rolled JSON-RPC stdio surface, feature-gated `--features mcp-server`, instrumented at stdin): `pulse-app` names it as a plain dependency, no cargo feature; its boundary is the door (`door.call` / `door.refused`); the library writes no `mcp.*` record; response bodies never logged (vector 4).
- The snapshot bullet: the four telemetry tools are four of the nine the door serves, over its socket or the stdio relay (was the sidecar's, over the hand-rolled stdio layer).
- The P3 row: spans read the per-tool read under `door.call` and `door.refused`, no `mcp.*` span and no gate (was `mcp.tools.call.request`, `mcp.feature.gate.check`, `mcp.response.emit`, parent `mcp.session`); fields read `door.call`'s eight (`tool` one of the nine or `none`), `door.refused`'s `reason`, `peer` and the per-tool reads' time range and service filter (was `method`, `result.count`, `feature_flag_enabled`, `env_var_mcp_enabled`); the tests excerpt drops its `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` spawn and stdin/stdout clause, the security-plan double-gate quote leaves, and the door is the one carrier (was the second).
- The Vector 4 row: the door is the one enforcement site (was the `mcp-server` boundary's `mcp.tools.call.response` span, the door a second site); its negative reading, the planted marker 0 times in the engine's log family, stands.
**Why:** the report's Retired bullet (the stdio sidecar `andromeda-pulse-mcp`, its double gate, the `mcp.*` record chain), its Symbols (`dispatch_tool` and `mcp.tools.call.response` removed; `call_tool`'s one product caller the door; no `mcp.*` record has an emitter), its Crates and Schema (`mcp-server` the tools library, a plain dependency; the cargo feature removed) and its Counts (engine ways in besides telemetry 2 → 1, the door).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the sidecar's stderr-JSON sink and its stdout ban
**Section:** §1 Obs Scope Summary (the Logging stack bullet); §6 Log Coverage (the sink bullet); §11 Obs Anti-Patterns → Universal (agent-driven specific); §3 → Logging stack (logging-stack.md: Sink)
**Change:**
- §1 Logging stack and §3 Sink: the app's one sink, the JSON file `~/.andromeda-pulse/logs/agent-latest.jsonl`, stands; the mcp-server sidecar clause leaves (was rmcp 3.x resolved 3.1.4, declared-but-unused, the JSON-RPC 2.0 layer hand-rolled, stderr forced JSON, stdout reserved for framing). The console program's `door` word still has no sink, its stdout the door's answer lines.
- §6: the bullet "MCP server (hand-rolled JSON-RPC stdio): stderr JSON-only" is retired.
- §11 Universal: the stdout ban reads for the `door` word alone — its stdout only the door's answer bytes unchanged, its stderr only the one fixed line on exit 1, no subscriber, no record (PROVISIONAL with the door's item); its stdout carries the door's JSON-RPC 2.0 answers and any accidental `println!` / `dbg!` / library stdout write corrupts them for the agent (was the ban for the `andromeda-pulse-mcp` sidecar, all `tracing` forced to stderr, the relay its later extension).
**Why:** the report's Retired bullet (the sidecar's stderr-JSON sink), its Symbols (`mcp_server::tracing_setup` removed) and its Dependencies (`rmcp` and `rmcp-macros` left `Cargo.lock`; `mcp-server` dropped `tracing-subscriber`, `tracing-appender`, `tracing-error`).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the workspace-key publish and its record retired
**Section:** §1 Obs Scope Summary (the P7 critical-path row); §4 Scenario P7 (Must-trace spans, Required log fields); §8 PII Scrubbing & Compliance (the `app.boot.workspace_key` leaf)
**Change:**
- P7's must-trace spans end at `workspace.identity.resolve`, in the §1 row and in §4 (was `→ app.boot.workspace_key`, the publish of the resolved key to `{data_dir}/run/workspace-key` for the MCP sidecar, INFO on success, WARN on failure).
- P7's fields drop that record's clause (was `workspace_root_basename` + `key_bytes` at INFO, `error_category` + `error_detail` at WARN, never the full path).
- §8's `app.boot.workspace_key` allowlist entry is retired.
**Why:** the report's Symbols (`engine_boot::publish_workspace_key_for_sidecar` and its call removed: neither program writes `{data dir}/run/workspace-key` or the `app.boot.workspace_key` record) and its Schema (that allowlist leaf removed); its Outcome reads it met on two window boots and an engine cycle.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — §4 Scenario P3: the door the one carrier, door.call its record
**Section:** §4 Span / Trace Coverage (the `span.kind = "server"` bullet; the instrumentation-per-surface MCP stdio row); §4 Scenario P3 (Surfaces involved, Must-trace spans, Required span attributes, Required log fields, Cleanup, the carrier paragraph)
**Change:**
- `span.kind = "server"` drops MCP tool invocations: the door records a call as the log event `door.call` and opens no span.
- The MCP stdio surface row is retired (its double gate, stderr-forced writer and `mcp.*` spans).
- Surfaces: the door (socket, console program only; the one carrier), reached directly or through `andromeda-pulse-engine door`, plus buffer and the incident corpus (was CLI stdio, IPC-internal, buffer, the door the second carrier).
- Must-trace: the per-tool read, then `door.call` (one per answered request, before the answer line) (was `mcp.tools.call.request` → `mcp.feature.gate.check` → read → `mcp.tools.call.response`).
- Attributes: `door.call` `method` (`initialize` | `tools/list` | `tools/call` | `ping` | `other` | `none`), `tool` (the nine `ALL_TOOL_NAMES` or `none`), `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`; `duckdb.query.*` unchanged (was the `mcp.*` sets, `params_count` and the two gate fields).
- Log fields: `door.call`'s eight, `door.refused`'s `reason`, `peer`, the reads' `time_range_start`, `time_range_end`, `service_filter` (was `trace_id`, `method`, `result.count`, `feature_flag_enabled`, `env_var_mcp_enabled`).
- Cleanup: the door's idle close and `-32002` over its bound, the relay one connection per line (was the `jsonrpc.rs` stdio loop).
- The paragraph reads **One carrier** (was **Two carriers**, the sidecar's `dispatch_tool` chain first): `call_tool` over `door::EngineTools`, its one product caller; no `mcp.*` record has an emitter and no gate exists.
**Why:** the report's Retired bullet (the sidecar, its double gate, the `mcp.*` chain), its Symbols (`dispatch_tool` and its three records removed; `mcp.session`, `mcp.tools.call.request`, `mcp.feature.gate.check` left with the binary) and its Schema (the `mcp-server` leaf removed, the door's three unchanged).
**Kept:** the door's bounds, the relay's records and the PROVISIONAL item (the pc overseer's answers, the founder's word owed) stand unchanged: the pass narrowed a surface, no widening.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the double gate and the sidecar's mcp.* records retired
**Section:** §6 Log Coverage (the `warn` level row; the per-module `mcp-server` row); §8 PII Scrubbing & Compliance (the `mcp-server` leaf); §11 Obs Anti-Patterns → Project-specific (andromeda-pulse) (the double-gate ban)
**Change:**
- §6's `warn` row drops `warn!(target: "mcp.gate.check", "MCP enabled but feature flag absent")`; its other examples stand.
- §6's `mcp-server` row is the tools library the door serves, with no level of its own: a call is logged by the door as `door.call` (`info` for `ok`, `warn` otherwise) (was the stdio sidecar, `info` on tool calls, `debug` on JSON-RPC detail).
- §8's `mcp-server` leaf (`method`, `result_type`, `result_count`, `feature_flag_enabled`, `duration_ms`) is retired.
- §11: NEVER read a feature or a variable as what guards the nine tools — no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED`, no `mcp.feature.gate.check` record; the tools are reached only through the door, its admission the owner check on every accepted connection, its state read from `app.boot.door` (`state`, `resolve_write`) and `door.refused` (`reason`, `peer`) (PROVISIONAL — §4 Scenario P3) (was NEVER feature-gate without the runtime double gate, the sidecar's ban, `rmcp` entering only under the feature).
**Why:** the report's Retired bullet (the double gate `--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`), its Schema (the cargo feature and the `mcp-server` allowlist leaf removed), its Symbols (`mcp.gate.check` left with `tracing_setup.rs`; the env var read nowhere) and its Dependencies (`rmcp` removed).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — §8 the reconciled-count writer
**Section:** §8 PII Scrubbing & Compliance (the `triage.incident.persist` entry)
**Change:**
- `reconciled_count` counts registry rows dropped because the persistent corpus already held them resolved — written by `mark_incident_resolved` through the console engine's door, the running engine dropping such a row from its active set within one persist cycle (was a writer OUTSIDE this process, the `andromeda-pulse-mcp` sidecar). The five fields, the once-per-cycle fold and the equality guard stand.
**Why:** the report's Retired bullet (the sidecar) and its Symbols (`call_tool`'s one product caller the door; `mark_incident_resolved`'s description now names the running engine dropping the incident within one persist cycle).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — §9 the test binaries that start the console program
**Section:** §9 CI Integration (the `xtask test` row)
**Change:**
- Six test binaries of the run start `andromeda-pulse-engine run` (was four): `integration_console_engine`, `agent_run_shell`, `engine_cycle_refusals`, `integration_door`, and the two door-reading tests `e2e_p3_mcp_subprocess_tools_call` and `security_mcp_response_body_redaction`; the cleared-environment clause reads for the first four.
**Why:** the report's New text and Outcome: the two tests, rewritten at this chunk, boot the console engine and read through the door.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — §3 the sidecar's service identity retired
**Section:** §3 → Service identity (service-identity.md: service.name)
**Change:**
- `service.name` reads the compile-time constant `"com.andromeda.pulse"` (fallback `"pulse-app"`) alone (was also `"andromeda-pulse-mcp"` for the mcp-server sidecar, a distinct process identity).
**Why:** the report's Symbols: the binary `andromeda-pulse-mcp` removed, the product binaries now `pulse-app` and `andromeda-pulse-engine`.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — §3 the MCP tools through the door
**Section:** §3 → Snapshot / paste-to-AI integration (snapshot-paste-to-ai-integration.md: MCP tools)
**Change:**
- The label reads "hand-rolled JSON-RPC 2.0 through the door" (was "over stdio; rmcp 3.x declared-but-unused").
- The door serves every tool through `mcp_server::tools::call_tool` (`ToolAnswer { value, result_type, result_count }`, no record), its one product caller `door::EngineTools`; each call is recorded as `door.call` with `result_type` + `result_count`, never the answer (vector 4); over stdio means the `andromeda-pulse-engine door` relay alone (was the sidecar's `mcp.tools.call.request` → `mcp.tools.call.response` at `dispatch_tool`, with the door a second carrier).
**Why:** the report's Symbols (`dispatch_tool` and its records removed; `call_tool`'s one product caller `pulse-app/src/door.rs`) and its Dependencies (`rmcp` removed).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
