# `mcp-server` — the nine MCP tools, served through the console engine's door (hand-rolled JSON-RPC 2.0)

## Responsibility
The nine MCP tools for AI agents to query telemetry directly, served inside the console engine's process by its door: line-delimited JSON-RPC 2.0 per MCP spec over the Unix-domain socket `run/door/sock`, reached over stdio through the relay `andromeda-pulse-engine door`. Tool methods (9): `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot` (shares the snapshot crate's curation pipeline) + `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved` (corpus-backed incident/report, chunk #94) + `retrieve_incident_events` (input `{incident_id}`; response `{incident_id, events: [{event_kind, occurred_unix_nano}], total, truncated}` oldest first, bounded at 256; `event_kind` coerced to `triage::contract::incident_event_kinds()`, else `unknown`; unknown id → -32603 `incident not found`; chunk 2026-10-02-incident-events-readable-through-mcp). There is no `mcp.*` TauRPC router and no gate: no `mcp-server` feature, and `ANDROMEDA_PULSE_MCP_ENABLED` is read by no program.

**The one carrier — the console engine's door (ratified).** `pulse-app` names this crate as the plain `[dependencies.mcp-server-crate]`, linked in every build, with no feature (no sidecar binary, no `rmcp`). The name dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>` (`tools.rs`): it writes NO log record and returns `ToolAnswer { value, result_type, result_count }`. It has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), which serves the nine tools inside `andromeda-pulse-engine run` over an owner-only Unix-domain socket, behind no feature gate, and records each call as `door.call`; no `mcp.*` record has an emitter in product code. `tools_list_manifest()` gives every tool a boolean `write`, true for `mark_incident_resolved` alone. The argument structs carry no `deny_unknown_fields` and read past an undeclared member; the manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`). Since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine through the door, by starting `andromeda-pulse-engine door` (`pulse_app::door_bridge`), which relays its standard-input lines to `run/door/sock` (one connection per request) and the door's answer lines, unchanged, to its standard output — the one stdio way to the nine: the engine has one way in besides telemetry, the door (socket and stdio relay). The door's form: architecture §Standard Contracts → Engine door.

## Key integrations

### Consumes from
- `buffer` crate via DuckDB prepared statements (read-side).
- `viz` crate query layer (shares prepared-statement code path).
- `snapshot` crate curation pipeline (`generate_snapshot` tool wraps `snapshot.generate`).

### Publishes to
- The console engine's door: line-delimited JSON-RPC 2.0 (`initialize` · `tools/list` · `tools/call` · `ping`; a request with no `id` answered with nothing) over `run/door/sock`, and over the stdin/stdout of the relay `andromeda-pulse-engine door`.
- stderr: nothing of this crate's (it has no binary); the relay's one standard-error line is the fixed `andromeda-pulse-engine door: the door cannot be reached`.
- TauRPC routers: none — the crate mounts no TauRPC router (no `mcp.*` procedures); the door reads no `run/workspace-key` and runs the tools over the engine's own `incident_workspace_key`.
- `tracing` events: none of this crate's own — `call_tool` writes no log record; the door records `app.boot.door`, `door.call`, `door.refused`; no `mcp.*` record has an emitter in product code.

### Dependencies
- No `rmcp`: no manifest and no `Cargo.lock` entry names it — the protocol layer is hand-rolled. The old 1.5.0-vs-0.3.x reconciliation is CLOSED by measurement.
- `serde_json` for JSON-RPC framing.
- `tokio` for async stdio.
- `duckdb` crate (read-side via `viz` shared code).

## Internal conventions
- **No feature gate:** the crate is not flag-specific — `pulse-app` names it as a plain dependency and links it in every build (the console engine's door calls its dispatch), per arch §Cross-cutting Patterns Feature-gate hygiene; there is no `mcp-server` feature, no sidecar binary and no `rmcp`.
- **No runtime gate:** `ANDROMEDA_PULSE_MCP_ENABLED` is read by no program; the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE` — `mark_incident_resolved` answers `-32001` unless it reads on.
- **Graceful degrade:** N/A — no feature and no MCP switch exist to mismatch; the door answers a fault with a fixed-message JSON-RPC error (`-32603` tool failed), never the tool's own text.
- **Stdout discipline:** the crate has no binary and owns no stdout; on the stdio path the relay `andromeda-pulse-engine door` writes the door's answer lines to standard output unchanged and nothing of its own.
- **Records:** `call_tool` writes no log record; the door records each answered call as `door.call`, closed labels only.
- **Tool method spans:** `#[tracing::instrument(skip(req), fields(method, time_range_start, time_range_end, service_filter, result.count))]`.
- **Response body NEVER logged** (security plan vector 4 — indirect-prompt-injection surface). Emit `result_type: "traces"` + `result_count: N` metadata only.
- **DuckDB prepared statements** via shared code path with `viz` crate — never `format!` SQL.
- **Shared curation pipeline** with `snapshot` crate — `generate_snapshot` tool method invokes the same dedup / anomaly / critical-path / aggregation / token budget pipeline.

## Service-specific gotchas
- **No sidecar binary:** `andromeda-pulse-mcp` is built by no workspace target; a stdio MCP client starts `andromeda-pulse-engine door` (per arch §Occupied Resources Process / service identity). The door's `initialize` still reads `serverInfo.name` `andromeda-pulse-mcp`.
- **No `rmcp`** — no manifest and no `Cargo.lock` entry names it; the MCP protocol is the hand-rolled `jsonrpc.rs` layer (MCP protocol `2024-11-05`). The old 1.5.0-vs-0.3.x question is CLOSED by measurement (chunk 2026-08-29-advisory-backlog).
- **OTLP attribute leakage to LLM clients** — MCP `query_*` and `generate_snapshot` tool responses surface attribute values to external LLM clients. Documented as the indirect-prompt-injection surface that the calling LLM client must defend itself against; the receiver app cannot fully sanitize OTLP-derived attribute values. Surfaced in README under "Using MCP with andromeda-pulse" and in security plan §Decisions Log.

## Entry points for modification
- **Carrier entrypoints (not this crate):** the door `pulse-app/src/door.rs` (`door::EngineTools`, `door::bind_endpoint`) and the stdio relay `pulse_app::door_bridge` (`andromeda-pulse-engine door`); this crate has no binary.
- **Tool method definitions:** ALL tools — the 4 telemetry tools and the incident/report tools (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` from chunk #94, `retrieve_incident_events` from chunk 2026-10-02-incident-events-readable-through-mcp) — are name-dispatch fns in `crates/mcp-server/src/tools.rs` (`ALL_TOOL_NAMES` + `call_tool`, the name dispatch, called by the door's `door::EngineTools`, its one product caller; no macro annotations), the incident tools corpus-backed (read/write the corpus in the console engine's own process, over the engine's own `Arc<dyn CorpusWriter>`)
- **JSON-RPC framing:** `crates/mcp-server/src/jsonrpc.rs` (hand-rolled serde; MCP protocol `2024-11-05` — measured 2026-08-29, nothing rmcp-provided)
- **Feature gate check:** none — the MCP surface has no feature gate and no switch of its own.
- **TauRPC router:** none — the `mcp-server` crate mounts no TauRPC router (no `mcp.*` procedures).
- **Tests:** colocated, and two subprocess files under `crates/mcp-server/tests/` (`sidecar_subprocess.rs`, `incident_events_subprocess.rs`). The E2E P3 legs sit at the binary boundary: `pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`, `e2e_p3_mcp_incident_tools.rs`, `e2e_p3_mcp_incident_events_content.rs`, `e2e_p3_mcp_resolve_content.rs`, beside `security_mcp_response_body_redaction.rs` and `unit_mcp_router.rs`. There is no `tests/integration/mcp/`.

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(mcp-server)'`
- **Integration (test-plan §6 P3):** boot `andromeda-pulse-engine run` → reach its door through `run/door/sock` or by spawning `andromeda-pulse-engine door` with stdin/stdout piped → populate buffer with 100 synthetic spans via OTLP ingest → write JSON-RPC 2.0 frame: `{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"query_traces","arguments":{"time_range":[…],"service_filter":"test-app","limit":100}}}` → parse stdout JSON → assert `result.traces` is array with schema match.
- **Negative tests:** another account is refused at the door; `mark_incident_resolved` answers `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on; an undeclared argument member answers `-32602`.

## Local development
- **Build:** `cargo build --bin andromeda-pulse-engine` — the nine tools are linked in every build, no feature.
- **Run with the door:** `cargo run --bin andromeda-pulse-engine -- run` — the door opens at `run/door/sock` under the data dir (console program only, Unix only).
- **Test interactively:** with a console engine running on the same data dir, `echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/debug/andromeda-pulse-engine door` → expect tools array on stdout.

## References
- `.andromeda/architecture.md` §Stack (MCP server row) + §Established Decisions [MCP Server Surface] (hand-rolled JSON-RPC 2.0, served by the door, no `rmcp`) + §Occupied Resources MCP stdio surface + §Workspace crates
- `.andromeda/security-plan.md` §API Security MCP feature double-gate + §Logging Vector 4 (response body never logged) + §Anti-Patterns (MCP double-gate omission ban)
- `.andromeda/test-plan.md` §6 P3
- `.andromeda/obs-plan.md` §1 P3 (must-trace `mcp.session` family) + §3 stderr forced JSON / stdout reservation
