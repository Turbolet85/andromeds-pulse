### Snapshot / paste-to-AI integration

Two cleanly separated surfaces:

**External-OTLP snapshot** (the product's CORE feature — the thing users invoke):
- **Path:** `~/.andromeda-pulse/snapshots/{timestamp}.md` (curated markdown, not raw OTLP JSON); timestamp format `YYYY-MM-DDTHH:MM:SSZ`
- **Schema:** Curated markdown with header (`## Snapshot from {time_range_start} to {time_range_end}`), metadata (`token_count`, `dedup_count`, `service_filter`, `anomaly_markers` like "⚠ latency spike" / "🔴 error cluster"), aggregated metrics (p50/p95/p99/max per service), critical-path top-slow-spans, trace IDs for drill-down. Deduped (identical span trees counted once with emission count). NOT raw OTLP JSON dump (per tests excerpt P2 + creator brief Section 6)
- **Trigger:** TauRPC `snapshot.generate({time_range, token_budget: 10000 | 25000 | 50000})` from webview; span `session.snapshot` wraps the pipeline; child spans emit token-budget telemetry (Section 5 perf-budget-instruments)
- **MCP tools (hand-rolled JSON-RPC 2.0 over stdio; rmcp 3.x declared-but-unused):** nine (`ALL_TOOL_NAMES`) — `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot` are the JSON-RPC 2.0 interface to the same external-OTLP query pipeline; `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved` (chunk #94) and `retrieve_incident_events` (chunk `2026-10-02-incident-events-readable-through-mcp`) read the incident corpus. Every tool emits `mcp.tools.call.request` (parent) → `mcp.tools.call.response` (result metadata only — `result_type` + `result_count`, NOT result_content per security plan vector 4) through the single `dispatch_tool` emission site; the `duckdb.query.{traces|metrics|logs}` child belongs to the telemetry tools only

**Self-observation paste-to-AI** (the product's debug surface — the thing developers/agents read):
- **Path:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (the same JSON log file from Logging stack above)
- **Format:** JSON-per-line, immediately greppable / `jq`-able / paste-to-LLM-able
- **No separate snapshot needed** — the JSON log file IS the self-observation snapshot

The two surfaces NEVER intersect: external client OTLP data lives in DuckDB and is curated into the markdown snapshot; the product's own runtime telemetry lives in the JSON log file. By separating them, no recursion is possible.
