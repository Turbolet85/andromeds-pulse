# Session Handoff

**Last Updated:** 2026-05-06T21:11:34Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #22 Query routers (traces/metrics/logs) shipped this session)

## Current State

- **Last completed chunk:** route#22 "Query routers (traces/metrics/logs) — viz crate prepared statements (no format-string SQL), query_id + param_count anonymized logging" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#23 "Broadcast fan-out + Tauri Channel API — tokio broadcast → pulse://stream/{spans,metrics,logs} binary Arrow IPC + _trace_context metadata + size cap" (Epoch 3 closes)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-19}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Epoch 3 — Storage & query:** chunks #20 + #21 + #22 committed; chunk #23 remains. Chunk #23 closes Epoch 3 and opens Epoch 4 (Webview shell + TauRPC bridge).

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #23 listed in route §2 but no `.andromeda/phases/phase-20/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear.

## Drift Detection (6 dimensions)

D1, D2, D3, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because no specialist plans were edited this session — plan_freshness mtimes unchanged from baseline. D6 advances cleanly: state.yaml.last_completed_chunk progresses 21 → 22 with this wrap commit.)

## Spec Amendments (this session)

(none this session — chunk #22 implementation matched specialist plan expectations; no Trigger 4 dialogue. Pre-existing archive untouched: lift-accent (2026-05-03) + clarify-pii-grep-ui-vocab (2026-05-04) both already archived per state.yaml.spec_amendments.archive.)

## Key Decisions This Session

- **Plan deviation: TauRPC trait+impl pairs relocated from viz crate to pulse-app/src/viz_routers.rs.** Plan step 5 proposed `crates/viz/src/runtime.rs` gated by `taurpc-runtime` feature with viz↔ui-bridge feature-flag cycle break. Implement-time analysis: cargo's feature unification forces uniform feature-active state workspace-wide → real cycle (viz-with-`taurpc-runtime` → ui-bridge → viz-with-`taurpc-runtime`). Resolution: relocate the 3 trait+impl triplets (TracesApi/MetricsApi/LogsApi) to pulse-app's binary-crate composition layer. viz crate stays cycle-free with no `taurpc-runtime` feature. Mirrors how pulse-app already orchestrates the existing HealthApi from ui-bridge. Plan note 2 explicitly anticipated "verify cargo resolves at impl time".
- **Minimal-viable column shape: chunk #22 returns 4 columns only (trace_id, span_id, ts_unix_nano + per-table identity).** TIMESTAMPTZ `ts` column dropped from query result rows because the workspace duckdb dep doesn't enable the `chrono` feature, so `DateTime<Utc>` does not implement `FromSql`. Clients can derive display time from `ts_unix_nano` (BIGINT). Documented in Phase 4 plan.md Implementation notes; richer columns (service_name, duration_ms, status_code) defer to a future buffer schema extension chunk.
- **specta::Type derive strategy: unconditional (not feature-gated) on viz public types.** ui-bridge's `HealthEnvelope` uses `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` to gate the derive. For viz, since the cycle-break design means viz NEVER has a `taurpc-runtime` feature (trait+impl in pulse-app), all 7 public types (TracesQueryArgs/MetricsQueryArgs/LogsQueryArgs/PaginatedResponse/TraceRow/MetricRow/LogRow) derive specta::Type unconditionally. Cost: specta becomes a hard workspace dep of viz; benefit: types directly usable from pulse-app's TauRPC routers without conditional-compilation gymnastics.
- **viz dep tree (cargo tree -p viz --depth 1):** `chrono, duckdb, serde, specta, thiserror, tokio, tracing, tracing-error`. ZERO sibling crates — STRONGER than the plan's "buffer is the only sibling allowed" criterion. The cycle-break design eliminated viz's need for any sibling.
- **Cursor encoding: simple stdlib `format!("c1:{ts_unix_nano}")` + `strip_prefix("c1:")`.** No base64 dep added. Opaque-string contract per arch §Standard Contracts is satisfied by versioned prefix + i64 string body. Tests verify decode rejects non-prefixed garbage AS InvalidArgument (covers SQL-injection negative test surface where cursor is the only user-controlled string parameter).
- **Pagination clamp behavior on out-of-window cursor:** `compute_window` returns `(start_ns, start_ns)` when cursor decodes to a value before the window's start (e.g., 2017-era cursor on a 60s window in 2026). Effectively returns zero rows rather than querying outside the active window. Documented via two split tests at chunk #22 implement-time.
- **build.rs for viz mirroring buffer chunk #20 workaround:** any new crate with direct `duckdb.workspace = true` dep on Windows MSVC needs `build.rs` emitting `cargo:rustc-link-lib=dylib=rstrtmgr` on `CARGO_CFG_TARGET_OS == "windows"`. libduckdb-sys 1.10502's build.rs doesn't emit this for downstream test-binary linking. (This pattern is already in session-learnings.md as a Tier 3 entry from chunk #20 — not re-curated this wrap; just applied.)
- **pulse-app needs `serde.workspace + specta.workspace` deps for taurpc::procedures macros.** taurpc 0.7's macro emits `taurpc::serde::Serialize` and `specta::*` paths that resolve at the call-site crate. Without these in pulse-app's `[dependencies]`, compile fails with E0463 (can't find crate `serde`) + E0433 (cannot find module `specta`). Curated as Tier 3 this wrap.

## Files Modified

(15 files this session — chunk #22 implementation + Cargo.lock + 4 new phase-19 artifacts. Living artifacts reconciled separately by wrap.)

**Code files (chunk #22 — Rust):**
- `crates/viz/Cargo.toml` — full dep set: `tokio.workspace`, `tracing.workspace`, `tracing-error.workspace`, `duckdb.workspace`, `serde.workspace`, `specta.workspace`, `chrono.workspace`; dev-deps `rstest.workspace`, `tempfile.workspace`, `serde_json.workspace`, `tokio = { workspace, features = ["test-util"] }`. NO `taurpc-runtime` feature (cycle break).
- `crates/viz/build.rs` (NEW) — mirrors `crates/buffer/build.rs` rstrtmgr workaround (chunk #20 precedent).
- `crates/viz/src/state.rs` (NEW) — `VizState { active_subscribers: AtomicU32, total_query_latency_ms_sum: AtomicU64, query_count: AtomicU64 }` + `VizStateSnapshot { subscribers_active, query_latency_ms_avg, query_count }` + setter methods (`record_query_latency_ms`, `inc_subscribers`, `dec_subscribers`); 5 unit tests (init zero, accumulate avg, inc/dec subscribers, snapshot zero count, concurrent multi-thread accumulation).
- `crates/viz/src/contract.rs` — replaced `Error::Placeholder` stub with 4 real variants (`QueryFailed{reason}`, `InvalidArgument{field, reason}`, `ConnectionLost`, `Decode{reason}`); replaced no-arg `heartbeat_payload()` with parameterized `heartbeat_payload(state: &VizState)` returning extended `VizHeartbeat { query_latency_ms, subscribers_active, query_count }`; +7 unit tests.
- `crates/viz/src/query.rs` (NEW) — 3 query helpers (`query_traces`, `query_metrics`, `query_logs`) + arg/result types (TracesQueryArgs/MetricsQueryArgs/LogsQueryArgs/PaginatedResponse<T>/TraceRow/MetricRow/LogRow with serde + specta::Type derives) + per-surface SQL constants (SELECT_TRACES/SELECT_METRICS/SELECT_LOGS + COUNT_TRACES/COUNT_METRICS/COUNT_LOGS — all static `&str`, never `format!`-built); `LIMIT_MAX = 1000` + `LIMIT_DEFAULT = 100` + `validate_args` + `compute_window` + `encode_cursor`/`decode_cursor` (`c1:{i64}` opaque format) + `hex_encode` (BLOB → hex string for trace_id / span_id / resource_hash) + `short_err` sanitizer; `tracing::info!(target = "viz.query.{traces|metrics|logs}", query_id, param_count = 3, param_types = "i64,i64,i64", time_window_seconds, row_count, latency_ms)` per-handler instrumentation (NEVER parameter values); 18 unit tests (validate_args bounds, cursor round-trip + rejection, query_traces/metrics/logs round-trip, SQL-injection negative test on cursor, pagination edge cases, ConnectionLost on poisoned mutex, hex_encode known bytes, compute_window cursor-inside + cursor-outside-window clamp).
- `crates/viz/src/lib.rs` — module declarations + public re-exports (Error / VizHeartbeat / heartbeat_payload / VizState / VizStateSnapshot / TracesQueryArgs / MetricsQueryArgs / LogsQueryArgs / PaginatedResponse / TraceRow / MetricRow / LogRow / query_traces / query_metrics / query_logs / LIMIT_MAX / LIMIT_DEFAULT).
- `crates/ui-bridge/Cargo.toml` — added `viz = { path = "../viz" }` sibling dep (extends chunk #18 ui-bridge → ingest pattern + chunk #20 ui-bridge → buffer to ui-bridge → viz; declared contract = the From impl per session-learnings 2026-05-05 entry).
- `crates/ui-bridge/src/contract.rs` — added `use viz::Error as VizError;` + `From<VizError> for AppError` impl (4 arms: QueryFailed → Storage{"viz query failed"}, InvalidArgument{field, ..} → Validation{field, "invalid query argument"}, ConnectionLost → Storage{"viz connection lost"}, Decode → Storage{"viz row decode failed"}); +5 sanitization tests asserting no DuckDB / version / file path / Rust struct name leakage.
- `crates/ui-bridge/src/health.rs` — `VizQueryStatus { Ok, QueryFailed(String) }` enum + `viz_query: Mutex<Option<VizQueryStatus>>` slot in HeartbeatState + `record_viz_query` setter + `viz_query()` reader + `current_health()` arms surfacing `subsystems.viz.status = "query_failed"` for the QueryFailed variant; +3 tests (slot starts empty, records ok/failed, current_health degrades on viz query failure).
- `pulse-app/Cargo.toml` — added `serde.workspace = true` + `specta.workspace = true` to `[dependencies]` (required by taurpc::procedures macro expansion in viz_routers.rs).
- `pulse-app/src/viz_routers.rs` (NEW) — 3 TauRPC trait+impl pairs (TracesApi/MetricsApi/LogsApi); each impl holds `Arc<Mutex<Connection>>` + `Arc<VizState>`; resolvers wrap `query_*` calls in `tokio::task::spawn_blocking` and convert `viz::Error → AppError` at the boundary via the From impl. Relocated from plan's `crates/viz/src/runtime.rs` per cycle-break deviation.
- `pulse-app/src/main.rs` — `use viz::VizState;` + `mod viz_routers;` + `let viz_state = Arc::new(VizState::new());` after buffer_state init + `taurpc::Router::new().merge(HealthApiImpl.into_handler()).merge(TracesApiImpl::new(...).into_handler()).merge(MetricsApiImpl::new(...).into_handler()).merge(LogsApiImpl::new(...).into_handler())` multi-router compose (with None-arm fallback to just HealthApi when buffer init failed) + `Arc::clone(&viz_state)` threaded into `heartbeat::spawn`.
- `pulse-app/src/heartbeat.rs` — `use viz::VizState;` + `viz_state: Arc<VizState>` arg added to `spawn(...)` + `run_viz` + `emit_viz_tick`; `emit_viz_tick(state, viz_state)` now calls parameterized `viz::contract::heartbeat_payload(viz_state)` instead of no-arg stub; +2 new tests (emit_viz_tick reflects real query_latency_ms avg, reflects real subscribers_active count) + 1 existing test updated to pass viz_state.
- `pulse-app/src/observability.rs` — extended `viz` AllowList entry (was 6 fields, now 11): added `param_types`, `row_count_returned`, `time_window_seconds`, `traceparent`, `filter_count`. New entries: `viz.query.error` (error_type, duration_ms, spantrace), `metric.trace.latency_percentiles` (value, service_name, p50/p95/p99/max_ms — service_name is documented obs-plan §5 cardinality exemption), `metric.trace.error_count` (value, service_name, error_type). +10 co-located pass+redact tests (5 pass + 5 redact mirroring chunks #20/#21 precedent).

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-19/{combined.md, research.md, plan.md}` (NEW) — Phase 19 planning artifacts for chunk #22 (187 + 85 + 307 lines).

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent`. New deps surfaced: viz now has 8 direct deps (chrono, duckdb, serde, specta, thiserror, tokio, tracing, tracing-error) vs prior single thiserror; ui-bridge gains viz sibling dep; pulse-app adds serde + specta direct deps.
- `.andromeda/context/api-surface.md` — reconciled via per-crate `cargo public-api --simplified` (style preserved; chunk #22 contributions hand-annotated). viz section grew from 4 pub items to ~30 (full module + types + functions). ui-bridge adds From<VizError> + VizQueryStatus + viz_query getter/setter on HeartbeatState.
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advanced to 22 + epoch 3 progresses; commit_sha advances from `"9eee494"` (stale chunk #21 placeholder) → `"pending"` → real SHA via Phase 10 amend; session_count=22; spec_amendments.{active,archive} unchanged from chunk #21 baseline; drift_warnings empty; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-06T21:11:34Z.
- `.claude/docs/session-learnings.md` — prepended 2 Tier 3 entries (TauRPC trait+impl placement / cargo feature-unification cycle break; taurpc::procedures macro deps + specta::Type derive requirements).
- `.claude/session-handoff.md` — this file.

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 0 additions
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions
  - (a) TauRPC trait+impl pairs belong in the binary crate, not in producer library crates (cargo feature-unification cycle)
  - (b) taurpc::procedures macro needs serde + specta crates at the call-site crate, plus specta::Type on every touched type
- **Filtered:** 1 dedup (rstrtmgr.lib build.rs lesson — already documented in session-learnings.md as 2026-05-06 chunk #20 entry; viz crate's build.rs is just applying the existing pattern) + 0 task-specific + 0 conflicts + 0 deferred (under max-3 cap)

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 288/288 workspace, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok, cargo audit ok with 18 pre-existing allowed warnings, cargo tree -p viz | grep opentelemetry empty (obs criterion #7 strict-grep PRESERVED), grep -rnE "format!.*(CREATE|SELECT|INSERT|UPDATE|DELETE|WHERE|FROM|JOIN)" crates/viz/src/ empty, grep -rnE "(param|value|service_name|span_name|attribute).*tracing::(info|debug|warn|error|trace)" crates/viz/src/ empty.)

## Tests Status

passing — 288 cargo nextest workspace (was 238 last wrap, +50 from chunk #22: 5 viz state tests, 7 viz contract tests, 18 viz query tests including SQL-injection negative + pagination edge cases + connection-lost mutex test + compute_window in/out-of-window split, 5 ui-bridge AppError From<VizError> sanitization tests, 3 ui-bridge health VizQueryStatus tests, 2 heartbeat emit_viz_tick tests, 10 observability AllowList tests for new viz/metric.trace.* entries) + cargo deny ok (skip list unchanged from chunk #21) + cargo audit ok + cargo clippy clean + cargo fmt clean. Total: 288 tests + 4 lint/typecheck gates + 2 supply-chain gates = 294 checks. cargo nextest --workspace ~1.02s parallel.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #23:**

`/andromeda-phase` to plan chunk #23 "Broadcast fan-out + Tauri Channel API — tokio broadcast → pulse://stream/{spans,metrics,logs} binary Arrow IPC + _trace_context metadata + size cap". Closes Epoch 3 (Storage & query). Builds on chunk #22's viz query routers but switches mode: pull-query (chunk #22) → push-stream (chunk #23). Anchors webview's real-time canvas updates per arch §Standard Contracts Real-time push contract.

**Priority 2 (background, optional) — chunk #23 prep verification:**

Before chunk #23 starts, validate that the existing buffer crate's broadcast subscriber pattern (if any was scaffolded in chunk #20/#21) is suitable for the Arrow IPC binary streaming. The plan should investigate `buffer::run_consumer`'s broadcast-fan-out path (or lack thereof) + whether a separate broadcast::Sender owned by main.rs is needed. Chunk #23 also introduces the size-cap discipline on Arrow IPC payloads (security plan §Plugin host bullet about "NEVER trust plugin-returned Arrow IPC bytes without an 8 MB size cap" — extends here for self-emitted streams).

## Session Goals (carry-over)

(none — chunk #22 fully implemented + tests green + curation applied (2 Tier 3) + reconcile complete + Epoch 3 chunk #22 closes; chunk #23 next; ready for `/andromeda-phase`)

## Session End Status

clean
