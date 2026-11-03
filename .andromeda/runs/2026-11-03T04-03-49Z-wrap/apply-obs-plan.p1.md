# Apply — obs-plan · part 1 of 2

## Verdicts
- fanout-obs-plan.md 1 · applied · E1
- fanout-obs-plan.md 2 · applied · E2
- fanout-obs-plan.md 3 · applied · E3
- fanout-obs-plan.md 4 · applied · E4
- fanout-obs-plan.md 5 · applied · E5
- fanout-obs-plan.md 6 · applied · E6
- fanout-obs-plan.md 7 · applied · E7
- fanout-obs-plan.md 8 · applied · E8
- fanout-obs-plan.md 9 · applied · E9
- fanout-obs-plan.md 10 · applied · E10
- fanout-obs-plan.md 11 · applied · E11
- fanout-obs-plan.md 12 · applied · E12
- fanout-obs-plan.md 13 · refused · a hand removal · fct-vxhgwrdg
- fanout-obs-plan.md 14 · applied · E13
- fanout-obs-plan.md 15 · applied · E14
- fanout-obs-plan.md 16 · applied · E15
- fanout-obs-plan.md 17 · refused · a hand removal · fct-oodqyure
- fanout-obs-plan.md 18 · refused · a hand removal · fct-z6icbp7d
- fanout-obs-plan.md 19 · applied · E16
- fanout-obs-plan.md 20 · applied · E17
- fanout-obs-plan.md 21 · applied · E18
- fanout-obs-plan.md 22 · refused · a hand removal · fct-jektn265
- fanout-obs-plan.md 24 · applied · E19
- fanout-obs-plan.md 27 · applied · E20
- fanout-obs-plan.md 28 · refused · a hand removal · fct-udg32czu
- fanout-obs-plan.md 29 · applied · E21

## Edits
@fact id=fct-4vrfwv27 expect_rev=1
old: **Two carriers since chunk 2026-10-10-door-inside-the-engine-s-process.** Everything above is the first, the stdio sidecar, unchanged: the `mcp.tools.call.request` → `mcp.feature.gate.check` → per-tool read → `mcp.tools.call.response` chain through `dispatch_tool`. The second is the door inside the console engine's process (`andromeda-pulse-engine run`;
new: **One carrier.** A tool call is carried by the door inside the console engine's process (`andromeda-pulse-engine run`;
old: over `door::EngineTools`, which writes no `mcp.*` record and passes no feature or runtime gate. That carrier's one record per call
new: over `door::EngineTools`, its one product caller; no `mcp.*` record has an emitter, and no feature or runtime gate exists. The door's one record per call

@fact id=fct-5gsdjdy6 expect_rev=1
old: CLI (hand-rolled JSON-RPC stdio), IPC-internal, buffer (DuckDB); since chunk 2026-10-10-door-inside-the-engine-s-process also the console engine's door (the second carrier, below), reached over its socket directly or, since chunk 2026-10-12-stdio-agent-reaches-the-door, by a stdio agent through `andromeda-pulse-engine door`
new: the console engine's door (a Unix-domain socket, console program only; the one carrier, below), reached over its socket directly or by a stdio agent through `andromeda-pulse-engine door`, plus buffer (DuckDB) and the incident corpus

@fact id=fct-4pvtx7no expect_rev=1
old: **Must-trace spans:** `mcp.tools.call.request` (JSON-RPC 2.0 stdin method parse) → `mcp.feature.gate.check` (compile-time + runtime gate validation) → the per-tool read
new: **Must-trace spans:** the per-tool read
old: emits no span of its own, its error surfacing as JSON-RPC -32603) → `mcp.tools.call.response` (emit JSON result to stdout)
new: emits no span of its own, its error surfacing as JSON-RPC -32603), then the door's must-log record `door.call` (one per answered request, written before the answer line); the door opens no span of its own

@fact id=fct-nunu6scu expect_rev=1
old: `mcp.tools.call.request`: `method` (one of the nine `ALL_TOOL_NAMES`:
new: `door.call`: `method` (`initialize` | `tools/list` | `tools/call` | `ping` | `other` | `none`), `tool` (one of the nine `ALL_TOOL_NAMES`:
old: retrieve_incident_events — bounded), `params_count`; `mcp.feature.gate.check`: `feature_flag_enabled`, `env_var_mcp_enabled`; `duckdb.query.*`
new: retrieve_incident_events — bounded — or `none`), `outcome`, `write`, `result_type` (not the answer; `incident_events` for `retrieve_incident_events`), `result_count`, `answer_bytes`, `duration_ms`; `duckdb.query.*`
old: `service_filter`; `mcp.tools.call.response`: `result_type` (not `result_content`; `incident_events` for `retrieve_incident_events`), `result_count` (its `events` length)
new: `service_filter`

@fact id=fct-43wzevug expect_rev=1
old: `trace_id`, `method`, `time_range_start`, `time_range_end`, `service_filter`, `result.count`, `feature_flag_enabled`, `env_var_mcp_enabled`
new: `door.call`'s `method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`; on a refusal `door.refused`'s `reason`, `peer`; the per-tool reads' `time_range_start`, `time_range_end`, `service_filter`

@fact id=fct-rtszlvuo expect_rev=1
old: the hand-rolled `jsonrpc.rs` stdio loop closes the JSON-RPC stream (measured chunk `2026-08-29-advisory-backlog` — nothing rmcp-provided runs); spans end after stdout result sent
new: the door closes an idle connection and answers a call over its bound with `-32002`, and the relay opens one connection per request line — the bounds in the carrier paragraph below

@fact id=fct-o4vhpvgv expect_rev=1
old: 9 major surfaces (desktop-webview, desktop-native, IPC-internal, API service gRPC, API service HTTP, CLI, library-only plugin host, MCP stdio and, since chunk 2026-10-10-door-inside-the-engine-s-process, the console engine's door — JSON-RPC 2.0 over a Unix-domain socket, console program only; 8 before it)
new: 8 major surfaces (desktop-webview, desktop-native, IPC-internal, API service gRPC, API service HTTP, CLI, library-only plugin host and the console engine's door — JSON-RPC 2.0 over a Unix-domain socket, console program only, also reached by a stdio agent through `andromeda-pulse-engine door`)

@fact id=fct-7woyixer expect_rev=1
old: | **mcp-server (hand-rolled JSON-RPC stdio)** | Arch: Stack + Workspace | boundary-only | MCP stdio surface feature-gated (`--features mcp-server`); instrumentation at boundary (stdin JSON-RPC 2.0 request/response);
new: | **mcp-server (the tools library the door serves)** | Arch: Stack + Workspace | boundary-only | `pulse-app` names it as a plain dependency, no cargo feature; instrumentation at its boundary, the console engine's door (`door.call` / `door.refused`), the library writing no `mcp.*` record of its own;

@fact id=fct-sjvcbexl expect_rev=1
old: `mcp.tools.call.request` (JSON-RPC method), `mcp.feature.gate.check` (compile-time + runtime gate), `duckdb.query.traces`
new: `duckdb.query.traces`
old: read the corpus instead), `mcp.response.emit` (JSON result); parent span: `mcp.session` | `method` (one of the nine `ALL_TOOL_NAMES`:
new: read the corpus instead), under the door's must-log records `door.call` and `door.refused` — no `mcp.*` span and no gate | `door.call`'s `method`, `tool` (one of the nine `ALL_TOOL_NAMES`:
old: retrieve_incident_events), `time_range_start`, `time_range_end`, `service_filter`, `result.count`, `feature_flag_enabled`, `env_var_mcp_enabled` |
new: retrieve_incident_events, or `none`), `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`; `door.refused`'s `reason`, `peer`; the per-tool reads' `time_range_start`, `time_range_end`, `service_filter` |
old: Tests excerpt P3 ("Spawns app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server`... pipes JSON-RPC 2.0 call to stdin... reads stdout for JSON-RPC result... asserts
new: Tests excerpt P3 ("... asserts
old: ; security plan Section 2 anti-pattern: "MCP double-gate omission — MCP stdio surface must require both `--features mcp-server` compile-time gate AND `ANDROMEDA_PULSE_MCP_ENABLED=true` runtime gate". The spans and the two gate fields of this row are the stdio sidecar's. Since chunk 2026-10-10-door-inside-the-engine-s-process a tool call has a second carrier, the console engine's door (a Unix-domain socket, console program only; since chunk 2026-10-12-stdio-agent-reaches-the-door also reached by a stdio agent through the relay `andromeda-pulse-engine door`, which adds no span, no gate and no record of its own): it emits no `mcp.*` span and passes no double gate, and its must-log record is `door.call` (`method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`) — §4
new: ; a tool call's one carrier is the console engine's door (a Unix-domain socket, console program only, also reached by a stdio agent through the relay `andromeda-pulse-engine door`, which adds no span, no gate and no record of its own) — §4

@fact id=fct-ohds4uw3 expect_rev=1
old: Four of the sidecar's nine MCP tools (`query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`) over the hand-rolled JSON-RPC stdio layer expose
new: Four of the nine MCP tools the console engine's door serves (`query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`), over its socket or the `andromeda-pulse-engine door` stdio relay, expose

@fact id=fct-dfxkc45d expect_rev=1
old: Implement response redaction at MCP boundary (`mcp-server`): `mcp.tools.call.response` span must emit `result_type: "traces"`, `result_count: 42` but NOT `result_content: "..."`. Negative test: call `query_traces` → verify response.traces array absent from logs; only metadata (count, schema) present. A second enforcement site since chunk 2026-10-10-door-inside-the-engine-s-process: the console engine's door records
new: Implement response redaction at the one enforcement site, the console engine's door, which records

@fact id=fct-qk3ssznn expect_rev=1
old: TauRPC command handlers, MCP tool invocations
new: TauRPC command handlers (the door records a tool call as the log event `door.call` and opens no span)

@fact id=fct-gcfverax expect_rev=1
old: (project name resolution) → `app.boot.workspace_key` (publish of the resolved key to `{data_dir}/run/workspace-key` for the MCP sidecar; INFO on success, WARN on failure — chunk 2026-08-14-workspace-key-alignment)
new: (project name resolution)

@fact id=fct-gejrrwfx expect_rev=1
old: `detection_latency_ms`; on `app.boot.workspace_key` ONLY `workspace_root_basename` + `key_bytes` (INFO) or `error_category` + `error_detail` (WARN) — the full workspace path never appears on this event (§8)
new: `detection_latency_ms`

@fact id=fct-6mzzyb6t expect_rev=1
old: (project name resolution), `app.boot.workspace_key` (publish of the resolved key for the sidecar — INFO on success, WARN on failure); parent span
new: (project name resolution); parent span
old: `detection_latency_ms`; on `app.boot.workspace_key` ONLY `workspace_root_basename` + `key_bytes` (never the full path — §8) |
new: `detection_latency_ms` |

@fact id=fct-avpvhn3k expect_rev=1
old: counting registry rows dropped because a writer OUTSIDE this process (the `andromeda-pulse-mcp` sidecar) had already resolved them
new: counting registry rows dropped because the persistent corpus already held them resolved — written by `mark_incident_resolved` through the console engine's door, the running engine dropping such a row from its active set within one persist cycle

@fact id=fct-fzliht4o expect_rev=1
old: violation; `warn!(target: "mcp.gate.check", "MCP enabled but feature flag absent")`;
new: violation;

@fact id=fct-hbfmcroa expect_rev=1
old: | `mcp-server` (stdio sidecar) | `info` (tool calls) / `debug` (JSON-RPC detail) | Baseline info on tool method invocation; debug on request parse |
new: | `mcp-server` (the tools library the door serves) | none of its own — a tool call is logged by the door as `door.call` (`info` for `ok`, `warn` otherwise) | The library writes no `mcp.*` record; the console engine's door logs each answered call |

@fact id=fct-d2dydmme expect_rev=1
old: For the mcp-server sidecar (rmcp 3.x — resolved 3.1.4 — a declared-but-unused anchor dependency; the JSON-RPC 2.0 layer is hand-rolled serde at `crates/mcp-server/src/jsonrpc.rs`), stderr is forced JSON (TTY check disabled) — stdout is reserved for JSON-RPC 2.0 framing and ANY accidental stdout write would corrupt the protocol. The console program's `door` word (the stdio relay, since chunk 2026-10-12-stdio-agent-reaches-the-door) writes
new: The console program's `door` word (the stdio relay) writes

@fact id=fct-anuozml7 expect_rev=1
old: NEVER write to stdout from any subsystem when running as the `andromeda-pulse-mcp` stdio sidecar — stdout is reserved for JSON-RPC 2.0 framing; ALL `tracing` output forced to stderr (JSON formatter); ANY accidental `println!` / `dbg!` / library stdout write corrupts the protocol and silently disconnects the MCP client. Since chunk 2026-10-12-stdio-agent-reaches-the-door the ban reaches the console program's `door` word (the stdio relay): NEVER write to its stdout anything
new: NEVER write to the stdout of the console program's `door` word (the stdio relay) anything
old: on exit 1; it installs
new: on exit 1 — its stdout carries the door's JSON-RPC 2.0 answers, and ANY accidental `println!` / `dbg!` / library stdout write corrupts them for the agent; it installs

@fact id=fct-7uh57iiw expect_rev=1
old: four test binaries of the run start
new: six test binaries of the run start
old: and since chunk 2026-10-10-door-inside-the-engine-s-process `integration_door` (three before it) — each under
new: `integration_door`, and the two door-reading tests `e2e_p3_mcp_subprocess_tools_call` and `security_mcp_response_body_redaction` — the first four each under

## Patterns
[[pattern]]
id = "obs1-01"
fixed = 'Everything above is the first, the stdio sidecar'

[[pattern]]
id = "obs1-02"
fixed = 'CLI (hand-rolled JSON-RPC stdio)'

[[pattern]]
id = "obs1-03"
fixed = 'emit JSON result to stdout'

[[pattern]]
id = "obs1-04"
fixed = '`feature_flag_enabled`, `env_var_mcp_enabled`'

[[pattern]]
id = "obs1-05"
fixed = 'stdio loop closes the JSON-RPC stream'

[[pattern]]
id = "obs1-06"
fixed = 'MCP stdio and, since chunk'

[[pattern]]
id = "obs1-07"
fixed = 'MCP stdio surface feature-gated'

[[pattern]]
id = "obs1-08"
fixed = 'gate fields of this row are the stdio sidecar'

[[pattern]]
id = "obs1-09"
fixed = 'over the hand-rolled JSON-RPC stdio layer expose'

[[pattern]]
id = "obs1-10"
fixed = '`mcp.tools.call.response` span must emit'

[[pattern]]
id = "obs1-11"
fixed = 'TauRPC command handlers, MCP tool invocations'

[[pattern]]
id = "obs1-12"
fixed = 'publish of the resolved key'

[[pattern]]
id = "obs1-13"
fixed = 'on `app.boot.workspace_key` ONLY'

[[pattern]]
id = "obs1-14"
fixed = 'a writer OUTSIDE this process (the `andromeda-pulse-mcp` sidecar)'

[[pattern]]
id = "obs1-15"
fixed = 'MCP enabled but feature flag absent'

[[pattern]]
id = "obs1-16"
fixed = '`mcp-server` (stdio sidecar)'

[[pattern]]
id = "obs1-17"
fixed = 'stderr is forced JSON'

[[pattern]]
id = "obs1-18"
fixed = 'when running as the `andromeda-pulse-mcp` stdio sidecar'

[[pattern]]
id = "obs1-19"
fixed = 'four test binaries of the run start the console program'
