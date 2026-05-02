# `snapshot` — Curated Markdown Generator

## Responsibility
Generates token-efficient curated markdown snapshots from buffer data. Pipeline: time-window selection → filter → curate (dedup identical spans / anomaly highlighting / critical-path extraction / p50/p95/p99 aggregation / drop verbose attributes) → format hierarchical markdown → enforce token budget (10k / 25k / 50k preset). Hosts `snapshot.{generate,list_recent,copy_to_clipboard}` TauRPC routers.

## Key integrations

### Consumes from
- `buffer` crate via DuckDB prepared statements.
- `viz` crate aggregation utilities for percentile computation.
- `workspace-detector` for snapshot path resolution (`.andromeda/` marker → `.andromeda/pulse/{timestamp}.md`; else → `~/.cache/andromeda-pulse/snapshots/{timestamp}.md`).

### Publishes to
- TauRPC routers: `snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard`.
- Tauri Channel: `pulse://stream/snapshot-progress` (progress events during generation; final completion event).
- Filesystem: `~/.andromeda-pulse/snapshots/{timestamp}.md` (canonical) + dual `.json` raw OTLP if user enabled.
- OS notification via `tauri-plugin-notification`: "Snapshot ready ({N} tokens). Paste in {AI tool} to investigate."
- `tracing` events: `snapshot.generate.request`, `snapshot.render.markdown`, `snapshot.token.count.validate`, `metric.snapshot.token_count_ms`.

### Dependencies
- `duckdb` crate (read-side aggregation queries).
- `arrow-rs` for batch processing.
- `tauri` 2.x for Channel API + clipboard.
- `tauri-plugin-notification` 2.x for OS toasts.
- `tiktoken-rs` (or equivalent) for token counting.

## Internal conventions
- **Pipeline stages (creator brief Section 6):**
  1. Time window selection (default 5 min, configurable 30s..30min OR user-selected from heatmap)
  2. Filter (service / trace ID / error-only / latency-outlier; preset + custom SQL filter)
  3. Curate: dedup (collapse `50× → "50× duplicate {span name} median {latency}"`), anomaly highlighting (latency outliers / error correlation / cardinality spikes), critical-path extraction (longest span path through service mesh per trace), aggregate metrics (p50/p95/p99/max), drop verbose attributes (keep `service.name`, `request_id`, `error`; drop `runtime.go.gc.heap` unless anomalous)
  4. Format hierarchical markdown with citation anchors
  5. Token budget enforcement (≤10k / ≤25k / ≤50k preset; smart truncation prioritizing anomalies + critical path)
- **Output schema:** header `## Snapshot from {time_range_start} to {time_range_end}` + metadata (`token_count`, `dedup_count`, `service_filter`, `anomaly_markers`) + aggregated metrics per service + critical-path top-slow-spans + trace IDs for drill-down.
- **NOT raw OTLP JSON dump** — snapshot tests assert dedup count + anomaly markers + p50/p95/p99 aggregates per test-plan §10.
- **Clipboard write side effects:** MUST emit `pulse://stream/snapshot-progress` event at clipboard-write time so the UI can show non-suppressible "X bytes copied" toast (visible signal that data left the app's address space).
- **NEVER log snapshot file contents** — security plan vector 4.

## Service-specific gotchas
- **OTLP attribute leakage** — snapshots can incidentally contain secrets / IDs / URLs / SQL fragments from instrumented host applications. Documented behavior; mitigated by user-facing warnings (README + first-run UI hint) and visible clipboard-write event.
- **MCP server shares the curation pipeline** — `generate_snapshot` MCP tool method invokes the same pipeline as the TauRPC `snapshot.generate` command. Sequencing: snapshot epoch precedes MCP sub-block (route Phase 6 + 7) so the curation pipeline is testable standalone before MCP wraps it as a JSON-RPC tool method.

## Entry points for modification
- **Pipeline orchestrator:** `crates/snapshot/src/pipeline.rs` (5-stage pipeline)
- **Curation primitives:** `crates/snapshot/src/curate/{dedup,anomaly,critical_path,aggregate}.rs`
- **Markdown formatter:** `crates/snapshot/src/format.rs`
- **Token counter + budget enforcer:** `crates/snapshot/src/budget.rs`
- **Snapshot path resolution:** `crates/snapshot/src/path.rs` (uses `workspace-detector` API)
- **Clipboard + notification:** `crates/snapshot/src/io.rs`
- **TauRPC router:** `crates/snapshot/src/router.rs`
- **Tests:** colocated per module + `tests/integration/snapshot/` for E2E P2

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(snapshot)'`
- **Integration (test-plan §6 P2):** populate buffer with 500 synthetic spans (varying service / latency / error) via OTLP gRPC → invoke `snapshot.generate({time_range, token_budget: 25000})` → assert response.markdown contains anomaly markers (regex `⚠|🔴|critical.*path`) + `token_count <= 25000` + `dedup_count > 0` + p50/p95/p99 aggregates + final `pulse://stream/snapshot-progress` completion event.
- **Performance budget (test-plan §10):** snapshot generation p99 <500ms for 25k token budget; chaos test 10k span backlog with 10k token budget asserts aggressive culling still produces valid snapshot.

## Local development
- **Manual snapshot:** Click "Investigate" button in compact widget or full dashboard → curated markdown saved to `.andromeda/pulse/{timestamp}.md` (if cwd has `.andromeda/`) or `~/.andromeda-pulse/snapshots/{timestamp}.md`.
- **Clipboard:** Snapshot generator copies path to clipboard with one of 4 preset prompts (Claude Code default / Cursor / ChatGPT / Custom).

## References
- `.andromeda/input.md` §Investigate workflow (creator brief curation pipeline spec)
- `.andromeda/architecture.md` §Workspace crates
- `.andromeda/security-plan.md` §Logging snapshot/clipboard hygiene + Vector 4 (response body never logged)
- `.andromeda/test-plan.md` §6 P2 + §10 (perf budget)
- `.andromeda/obs-plan.md` §1 P2 (must-trace `snapshot.generate.request`)
- `.andromeda/design-system.md` §Motion Investigation Capture Collapse supporting moment
