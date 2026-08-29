# `mcp-server` — MCP stdio Sidecar (hand-rolled JSON-RPC 2.0)

## Responsibility
Optional MCP server sidecar for AI agents to query telemetry directly. JSON-RPC 2.0 over stdin/stdout per MCP spec. Tool methods (8): `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot` (shares the snapshot crate's curation pipeline) + `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved` (corpus-backed incident/report, chunk #94). Hosts `mcp.{status,start,stop}` TauRPC routers. **Feature double-gated:** compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`.

## Key integrations

### Consumes from
- `buffer` crate via DuckDB prepared statements (read-side).
- `viz` crate query layer (shares prepared-statement code path).
- `snapshot` crate curation pipeline (`generate_snapshot` tool wraps `snapshot.generate`).

### Publishes to
- stdout: JSON-RPC 2.0 framing (initialize / tools/list / tools/call / notifications/*).
- stderr: forced JSON `tracing` output (TTY check disabled — stdout reserved for protocol).
- TauRPC routers (visible from app side): `mcp.status`, `mcp.start`, `mcp.stop`.
- `tracing` events: `mcp.session`, `mcp.tools.call.request`, `mcp.tools.call.response`, `mcp.feature.gate.check`.

### Dependencies
- `rmcp` — feature-gated ANCHOR dependency only (requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29; sole source contact `use rmcp as _;` in the bin — the protocol layer is hand-rolled, not rmcp-provided). The old 1.5.0-vs-0.3.x reconciliation is CLOSED by measurement.
- `serde_json` for JSON-RPC framing.
- `tokio` for async stdio.
- `duckdb` crate (read-side via `viz` shared code).

## Internal conventions
- **Feature gate (compile-time):** `--features mcp-server` adds `rmcp` to the build graph; flag-specific dependencies only enter when feature enabled per arch §Cross-cutting Patterns Feature-gate hygiene.
- **Runtime gate:** `ANDROMEDA_PULSE_MCP_ENABLED=true` — default off. `false` AND/OR feature absent = sidecar not spawned.
- **Graceful degrade:** env var set against binary built without feature → log `warn` naming the missing feature flag and proceed (do NOT regress to fail-startup).
- **Stdout discipline:** stdout reserved for JSON-RPC 2.0 framing. ANY accidental `println!` / `dbg!` / library stdout write corrupts protocol → silently disconnects MCP client.
- **Stderr forced JSON:** `tracing-subscriber::fmt::layer().json().with_writer(std::io::stderr)` with TTY check disabled in this binary.
- **Tool method spans:** `#[tracing::instrument(skip(req), fields(method, time_range_start, time_range_end, service_filter, result.count))]`.
- **Response body NEVER logged** (security plan vector 4 — indirect-prompt-injection surface). Emit `result_type: "traces"` + `result_count: N` metadata only.
- **DuckDB prepared statements** via shared code path with `viz` crate — never `format!` SQL.
- **Shared curation pipeline** with `snapshot` crate — `generate_snapshot` tool method invokes the same dedup / anomaly / critical-path / aggregation / token budget pipeline.

## Service-specific gotchas
- **Sidecar binary identity:** distinct from `pulse-app` — `andromeda-pulse-mcp` (per arch §Occupied Resources Process / service identity).
- **rmcp is declared-but-unused** — the sidecar links rmcp under the feature flag (anchor import) but no rmcp API executes; the MCP protocol is the hand-rolled `jsonrpc.rs` layer (MCP protocol `2024-11-05`). Measured at chunk 2026-08-29-advisory-backlog, which also closed the old 1.5.0-vs-0.3.x reconciliation (pinned `"3"`, resolved 3.1.4).
- **OTLP attribute leakage to LLM clients** — MCP `query_*` and `generate_snapshot` tool responses surface attribute values to external LLM clients. Documented as the indirect-prompt-injection surface that the calling LLM client must defend itself against; the receiver app cannot fully sanitize OTLP-derived attribute values. Surfaced in README under "Using MCP with andromeda-pulse" and in security plan §Decisions Log.

## Entry points for modification
- **Sidecar entrypoint:** `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (the binary; gated by `--features mcp-server`, `required-features` on the `[[bin]]`)
- **Tool method definitions:** ALL tools — the 4 telemetry tools and the chunk #94 incident/report tools (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`) — are name-dispatch fns in `crates/mcp-server/src/tools.rs` (`ALL_TOOL_NAMES` + `dispatch_tool`; no macro annotations), the incident tools corpus-backed (read/write `corpus/corpus.db` cross-process)
- **JSON-RPC framing:** `crates/mcp-server/src/jsonrpc.rs` (hand-rolled serde; MCP protocol `2024-11-05` — measured 2026-08-29, nothing rmcp-provided)
- **Feature gate check:** `crates/mcp-server/src/feature_gate.rs` (compile-time + runtime double-gate)
- **TauRPC router:** `crates/mcp-server/src/router.rs` (mcp.status / mcp.start / mcp.stop visible from main app)
- **Tests:** colocated + `tests/integration/mcp/` for E2E P3

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(mcp-server)'`
- **Integration (test-plan §6 P3):** spawn app with `cargo build --bin pulse-app --features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true` → spawn sidecar via `tokio::process::Command` with stdin/stdout piped → populate buffer with 100 synthetic spans via OTLP ingest → write JSON-RPC 2.0 frame: `{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"query_traces","arguments":{"time_range":[…],"service_filter":"test-app","limit":100}}}` → parse stdout JSON → assert `result.traces` is array with schema match.
- **Negative tests:** spawn without feature flag → build fails OR runtime detects missing binary; runtime env var unset → assert sidecar not spawned.

## Local development
- **Build with MCP:** `cargo build --bin pulse-app --features mcp-server`
- **Run with MCP enabled:** `ANDROMEDA_PULSE_MCP_ENABLED=true cargo run --bin pulse-app --features mcp-server`
- **Test sidecar interactively:** `echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/debug/andromeda-pulse-mcp` → expect tools array on stdout.

## References
- `.andromeda/architecture.md` §Stack (MCP server row) + §Established Decisions [MCP Server Surface] (hand-rolled JSON-RPC 2.0, rmcp anchor dep — amended 2026-08-29) + §Occupied Resources MCP stdio surface + §Workspace crates
- `.andromeda/security-plan.md` §API Security MCP feature double-gate + §Logging Vector 4 (response body never logged) + §Anti-Patterns (MCP double-gate omission ban)
- `.andromeda/test-plan.md` §6 P3
- `.andromeda/obs-plan.md` §1 P3 (must-trace `mcp.session` family) + §3 stderr forced JSON / stdout reservation
