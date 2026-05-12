# Session Handoff

**Last Updated:** 2026-05-12T20:30:00Z
**Branch:** main
**Session End Status:** clean (chunk #49 implemented + all gates green + Epoch 7 closes)
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; closes session 59)

## Current State

- **Last completed chunk:** route#49 "MCP tool methods + IPC routers — query_traces/query_metrics/query_logs/generate_snapshot #[tool] sharing curation + mcp.status/start/stop, response.body never logged" (Epoch 7 — Plugin runtime + MCP server CLOSES)
- **Next chunk:** route#50 "End-to-end test pass — synthetic OTLP via :4317/:4318 + TauRPC roundtrip + Channels + MCP subprocess + plugin lifecycle + workspace detection (P1-P7)" (Epoch 8 — Polish & ship opens)
- **In-progress phase:** none — chunk #49 implementation landed this session; phase-46 artifacts archived (combined.md + research.md + plan.md)
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-46}/{combined.md, research.md, plan.md}` (phase-46 from this session)
- **Epoch 7 — Plugin runtime + MCP server: 5 of 5 chunks closed (#45 substrate + #46 sandbox + #47 loader + #48 sidecar + #49 tool methods + IPC).** Total route §2 chunk count: 56 (unchanged).

## Andromeda State Detection (states A-K)

⚠️ **E — Pending phase planning**: chunk #50 next; no `.andromeda/phases/phase-47/` directory yet. Remediation: `/andromeda-phase` to plan chunk #50 (Epoch 8 opens — End-to-end test pass).

(A, B, C, D, F, G, H, I, J, K all clean post-wrap.)

## Drift Detection (6 dimensions)

**No drift detected this wrap.**

- D1 (living artifact staleness): cleared by Phase 5 reconcile — dep-tree.md + api-surface.md both reconciled 2026-05-12T20:30:00Z with fresh tooling output reflecting chunk #49 delta (+11 lines on dep-tree.md `mcp-server v0.1.0` block from 4 new workspace deps `buffer`/`viz`/`snapshot`/`duckdb` + `serde_json` lifted to direct; +73 lines on api-surface.md `mcp-server` crate block from new `pub mod tools` + 4 per-tool argument structs + dispatch_tool entry + ALL_TOOL_NAMES + 2 new Error variants + tools_list_with_4_tools).
- D2 (wrong content): clean — Phase 5 tooling output captured cleanly; both artifacts updated atomically with stdout-only content.
- D3 (plan-to-code): clean — arch §Workspace crates LOCKED list (10 members) matches Cargo.toml workspace.members; chunk #49 adds 3 new TauRPC procedures `mcp.{status,start,stop}` which arch §Occupied Resources reserves (mcp-server crate, "only when --features mcp-server"); `cargo xtask capability-drift` exits 0 (quadruple binding all-aligned: router + capability JSON description + EXPECTED_PROCEDURES uncommented + emit_taurpc_bindings test merges McpApiImpl).
- D4 (plan-to-plan): clean — no spec amendments this session.
- D5 (plan-to-CLAUDE.md mtime): clean — no upstream (arch + 6 specialist plans + route + input) regenerated this session; all upstream mtimes ≤ CLAUDE.md mtime (CLAUDE.md updated 2026-05-11T19:17Z via session 55's full setup-project re-derive).
- D6 (route chunk progression): clean post-wrap — state.yaml.last_completed_chunk advances to chunk #49 with this wrap's commit SHA.

## Spec Amendments (this session)

(none this session — chunk #49 implementation surfaced no Trigger 4 spec ↔ reality drift; all plan-specified scope landed within the original Phase 4 plan.)

state.yaml.spec_amendments.active: 0 entries (unchanged from session 58 — clean lifecycle).
state.yaml.spec_amendments.archive: 17 entries (unchanged from session 58).

## Key Decisions This Session

- **Single-chunk phase 46 plan**: chunk #49 grouped alone per "single-substantial" heuristic — estimated ≥3h, touches mcp-server crate (5 modules + 2 new Error variants + 4 new tool dispatchers + tools_list_with_4_tools + AllowList extension + binary entry rewrite for tool dispatch + ephemeral DuckDB at boot) + pulse-app (new mcp_router.rs + main.rs router merge + emit_taurpc_bindings test extension + resolve_mcp_sidecar_binary_path helper) + ui-bridge contract.rs From impl extension + xtask EXPECTED_PROCEDURES + default.json description update + 7 new sidecar subprocess integration tests; cross-cuts security (response-body redaction + quadruple binding + double-gate preservation) + obs (boundary spans + AllowList extension + per-tool metric event) + tests (subprocess integration tests + SQL-injection canary + negative canary for response body) + arch (Occupied Resources `mcp.{status,start,stop}` activation + module dependency direction preserved DAG-rooted at pulse-app + 10 workspace crates unchanged). Closes Epoch 7.

- **Five-place cfg-gate pattern emergent (Tier 2 lesson)**: chunk #49's mcp_router.rs imports `mcp_server_crate::feature_gate::*` — the renamed-snake-case alias from pulse-app/Cargo.toml's `[dependencies.mcp-server-crate] package = "mcp-server" optional = true`. The symbol only exists when `--features mcp-server` is on, so the entire `mod mcp_router;` declaration + `use mcp_router::*;` import + helper functions + `let mcp_impl = ...;` construction + `.merge(mcp_impl...)` router chain ALL needed `#[cfg(feature = "mcp-server")]` gates. Default-features cargo check failed first attempt with E0433 "cannot find module or crate `mcp_server`"; the fix was 5 cfg-gate insertions across main.rs + the unrenamed `mcp_server` → `mcp_server_crate` rewrite in mcp_router.rs. Curated as Tier 2 lesson extending the 2026-05-12 quadruple-binding rule (security.md Session Additions).

- **Cross-process buffer sharing deferred to Epoch 8**: chunk #49's sidecar binary opens its OWN ephemeral `:memory:` DuckDB connection at boot + initializes the standard buffer schema via `buffer::schema::create_schema(&conn)`. The ephemeral buffer starts empty — `query_*` MCP tool calls return empty `PaginatedResponse` results. Live cross-process buffer sharing (sidecar ↔ main process DuckDB ring buffer) deferred to a future chunk; chunk #49 delivers the substrate (tool dispatch + IPC routing + response-body redaction harness + quadruple binding) without blocking on the complex cross-process IPC design. Documented as Open Question in plan.md §Implementation notes; resolved during Phase 4 plan synthesis with concrete commitment.

- **mcp.start spawns sidecar as child of main process**: chunk #49's `mcp.start` TauRPC resolver constructs `tokio::process::Command::new(sidecar_binary_path)` with `ANDROMEDA_PULSE_MCP_ENABLED=true` env injection + `Stdio::null()` for all 3 stdio channels (sidecar dangling-stdio is acceptable since MCP clients invoke the binary directly out-of-band, NOT via the spawned child). The `McpApiImpl` holds `Arc<Mutex<Option<Child>>>` for sidecar process tracking. `mcp.start` idempotent (returns existing pid if already running); `mcp.stop` takes the Child out of the lock + calls `child.kill().await` (std::sync::Mutex used to avoid holding async-aware lock across await — take-then-drop pattern from chunk #47 precedent). Re-validates double-gate at spawn time per security plan §Anti-Patterns Code Patterns row 3.

- **TokenBudget enum threshold mapping for MCP clients**: chunk #49's `generate_snapshot` MCP tool accepts a `token_budget: u32` argument (default 25000) but the snapshot crate's `TokenBudget` enum is closed-set (`Conservative` / `Balanced` / `Detailed` = 10k / 25k / 50k per arch §Established Decisions). The dispatcher uses `budget_for_count()` helper: `< 17_500 → Conservative; < 37_500 → Balanced; else Detailed`. MCP clients send approximate budgets; dispatcher snaps к nearest preset. Avoided adding a `TokenBudget::Custom(usize)` variant to the snapshot crate (would have widened the public surface unnecessarily; the 3-preset model is preserved).

## Files Modified

This wrap's commit:

**NEW files (2 + audit trail):**
- `crates/mcp-server/src/tools.rs` (~450 lines — 4 per-tool argument structs + 4 dispatch functions + top-level `dispatch_tool` entry + `ALL_TOOL_NAMES` slice + `budget_for_count` + `short_reason` + `load_recent_spans` + `compute_cutoff` helpers + 10 unit tests including SQL-injection canary via cursor arg)
- `pulse-app/src/mcp_router.rs` (~280 lines — `McpServerState` 3-variant enum + `McpStatusDto` / `McpStartResult` / `McpStopResult` DTOs + `McpApi` trait + `McpApiImpl` struct + 3 resolver impls + 6 co-located unit tests + `gate_state_to_mcp_state` + `state_label` helpers)
- `.andromeda/phases/phase-46/combined.md` (251 lines)
- `.andromeda/phases/phase-46/research.md` (83 lines)
- `.andromeda/phases/phase-46/plan.md` (261 lines)

**MODIFIED files (12):**
- `Cargo.lock` — duckdb/snapshot/viz/buffer transitive deps surface in mcp-server's workspace deps
- `crates/mcp-server/Cargo.toml` — added `duckdb.workspace = true` + `buffer = { path = "../buffer" }` + `viz = { path = "../viz" }` + `snapshot = { path = "../snapshot" }` workspace deps
- `crates/mcp-server/src/lib.rs` — added `pub mod tools;` declaration
- `crates/mcp-server/src/contract.rs` — 2 new `Error` variants (`ToolDispatchFailed { tool_name, reason }`, `ToolArgsInvalid { tool_name, reason }`) + 2 Display tests
- `crates/mcp-server/src/jsonrpc.rs` — `tools_list_with_4_tools()` helper + 3 schema-assertion tests
- `crates/mcp-server/src/tracing_setup.rs` — `AllowList::for_mcp_server()` `mcp` target extended with 6 new fields (`tool_name`, `query_id`, `param_count`, `traceparent`, `error_detail`, `tool_name_unknown`) + new `metric.mcp.tool_call_duration_ms` target entry with allowlisted `value`/`method`/`result_count` + 3 cascade tests
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` — full rewrite of dispatch path: replaced `empty_tools_list()` with `tools_list_with_4_tools()`; replaced `tools/call` -32601 placeholder with per-tool dispatch routing via new `dispatch_tools_call()` function; added `SidecarContext { conn, viz_state }` struct + `init_buffer_connection()` opening ephemeral DuckDB with `buffer::schema::create_schema` at boot
- `crates/mcp-server/tests/sidecar_subprocess.rs` — updated `sidecar_tools_list_returns_empty_array` → `sidecar_tools_list_returns_4_tools_with_names`; added 7 new integration tests (`sidecar_tools_call_query_{traces,metrics,logs}_returns_empty_paginated_response`, `sidecar_tools_call_generate_snapshot_returns_markdown_envelope`, `sidecar_response_body_never_in_stderr_for_query_traces`, `sidecar_sql_injection_canary_query_traces`, `sidecar_unknown_tool_returns_neg_32601`) + `send_tools_call_and_read` helper
- `crates/ui-bridge/src/contract.rs` — `From<McpServerError> for AppError` impl extended for 2 new variants (`ToolDispatchFailed` → "mcp-server: tool dispatch failed", `ToolArgsInvalid` → "mcp-server: tool arguments invalid") + 2 round-trip-no-leak tests verifying no Rust struct names / file paths / library versions leak in sanitized `message` field
- `pulse-app/capabilities/default.json` — description acknowledges `mcp.{status,start,stop}` router-level coverage (per CLAUDE.md Session Additions 2026-05-03 TauRPC capability rule; sidecar lifecycle gated by `--features mcp-server` build + `ANDROMEDA_PULSE_MCP_ENABLED` runtime env var)
- `pulse-app/src/main.rs` — `#[cfg(feature = "mcp-server")] mod mcp_router;` + `use mcp_router::{McpApi, McpApiImpl};` + `resolve_mcp_sidecar_binary_path()` helper (gated) + `let mcp_impl = ...` construction (gated, block-expression) + 2 router merge points (Some(conn) + None branches) with conditional `.merge(mcp_impl.clone().into_handler())` via block-expression idiom + `emit_taurpc_bindings` test extended to construct `McpApiImpl::new(...)` and `.merge()` into the test router (4th binding per .claude/rules/security.md Session Additions 2026-05-12)
- `xtask/src/main.rs` — uncommented `"mcp.status", "mcp.start", "mcp.stop"` in `EXPECTED_PROCEDURES` + added sibling test `expected_procedures_includes_mcp_namespace_at_chunk_49` mirroring chunk #47 precedent
- `.andromeda/context/dependency-tree.md` — fresh `cargo tree --workspace --depth 2 --prefix indent` captured (327 → 338 lines; +11 lines for new mcp-server deps); METADATA Maintenance prepended with session 59 note
- `.andromeda/context/api-surface.md` — fresh `cargo +nightly public-api --simplified` per-crate output captured (4861 → 4934 lines; +73 lines for mcp-server crate public surface); METADATA Maintenance prepended with session 59 note
- `.andromeda/state.yaml` — session_count 58 → 59; last_wrap/last_reconcile refreshed; last_completed_chunk advanced to chunk #49; drift_warnings empty; plan_freshness mtimes re-captured
- `.claude/rules/security.md` — 1 new Tier 2 entry (five-place cfg-gate pattern for feature-gated TauRPC namespaces)
- `.claude/session-handoff.md` — full overwrite (this file)

**Audit trail (gitignored — `.andromeda/runs/`):**
- `.andromeda/runs/2026-05-12T19-50-00-phase-46/{.raw-{specialty}.md × 7, {specialty}.md × 7}` — phase 46 sub-agent raw + stripped extracts

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition to security.md ("Five-place cfg-gate pattern for OPTIONAL feature-gated TauRPC namespaces — extends quadruple binding when crate is renamed via `[dependencies.alias] package = "..." optional = true`"; confidence 0.85)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 0 additions
- **Filtered:** 0 duplicates + 0 task-specific + 0 conflicts + 0 deferred

## Last Failed Command

(none — all session 59 operations succeeded.)

## Tests Status

passing — 762/762 Rust workspace tests with `--features mcp-server`. Default-features (no feature flag): 742/742 (chunk #49 new tests are all behind `cfg(feature = "mcp-server")` so they only appear under the feature flag; default-features count unchanged).

Coverage gates sustained: Standard tier ≥75% line / ≥70% branch / ≥85% function. Per-file: `crates/mcp-server/src/tools.rs` + `pulse-app/src/mcp_router.rs` above thresholds via co-located test modules; integration tests in `crates/mcp-server/tests/sidecar_subprocess.rs` (12 tests total — 5 original + 7 new) validate the binary's tool dispatch end-to-end against synthetic JSON-RPC frames.

Capability-drift gate: `cargo xtask capability-drift` exits 0 with `drift_state=clean, extra_count=0, missing_count=0` — verifies the quadruple binding for `mcp.{status,start,stop}` matches production TauRPC surface (router registration + capability JSON description + EXPECTED_PROCEDURES uncommented + emit_taurpc_bindings test merges McpApiImpl).

Supply-chain gates: `cargo deny check bans licenses sources` exits 0 (no new transitive duplicates introduced by chunk #49's workspace dep additions — buffer/viz/snapshot all already in the dep graph via ui-bridge); `cargo audit` exits 0 (18 pre-existing warnings; no new advisories).

Lint gates: `cargo fmt --check` clean after one cargo fmt apply during /implement Phase 1; `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits 0 with no new warnings.

Direct sidecar binary smoke: VERIFIED via the 12 sidecar_subprocess.rs integration tests (Phase 2b smoke `npx @tauri-apps/cli dev` skipped per env — Tauri CLI dev not feasible in this CLI bash session; direct binary smoke covered via subprocess Command spawn pattern from chunk #48 precedent extended with 7 new tools/call round-trip + negative-canary + SQL-injection-canary + unknown-tool tests).

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #50 (Epoch 8 opens):**

route#50 "End-to-end test pass — synthetic OTLP via :4317/:4318 + TauRPC roundtrip + Channels + MCP subprocess + plugin lifecycle + workspace detection (P1-P7)". The chunk introduces (a) the full E2E test pass covering P1-P7 critical paths from test-plan §6 — synthetic OTLP gRPC ingest via `tonic` 0.14.5 client to loopback :4317, OTLP HTTP ingest via `reqwest` 0.12.x + `axum-test` 18.7.0 to :4318, TauRPC roundtrip via `tauri::test::mock_builder() + get_ipc_response()`, Channel subscription via Tauri 2 IPC Channel API with binary Arrow IPC decode, MCP subprocess via `tokio::process::Command` + JSON-RPC 2.0 frames (extending chunk #49's 12 subprocess tests), plugin lifecycle via `plugins::loader::discover_plugins` against a real fixture plugin dir, workspace detection via `workspace_detector::detect`; (b) the chunk #49 substrate's cross-process buffer sharing question may resurface at E2E time — depending on how P3 (MCP `tools/call query_traces` returns array of trace objects) is constructed, the test may need to seed the sidecar's buffer cross-process OR re-architect the sidecar к run in-process with the main process's buffer. Open architectural call deferred from chunk #49 plan §Implementation notes — surfaces here as scope decision for the test author. Opens Epoch 8 — Polish & ship.

**No outstanding remediation items** — D1-D6 clean, state E expected (pending phase-47 planning is normal for chunk-completion wrap), no active spec amendments, no stale drifts. Clean session-start state for the next /andromeda-new-session invocation.

## Session Goals (carry-over)

(none — session 59 user goal achieved: chunk #49 MCP tool methods + IPC routers implemented + tool dispatch wired through ephemeral DuckDB + 4 #[tool] methods exposed via tools/list + 3 TauRPC procedures `mcp.{status,start,stop}` + response-body redaction harness + quadruple binding + Epoch 7 closes.)

## Deferred decisions (Trigger 4 to Path B carry-over)

(none — no Trigger 4 dialogues this session.)

## Deferred learnings (filtered out from Phase 4 curation)

(none — only 1 Tier 2 candidate this session; applied without dedup conflicts.)
