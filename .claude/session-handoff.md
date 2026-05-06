# Session Handoff

**Last Updated:** 2026-05-06T18:30:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #21 Retention task + buffer.tick heartbeat shipped this session)

## Current State

- **Last completed chunk:** route#21 "Retention task + buffer.tick heartbeat — periodic DELETE WHERE ts < cutoff via ANDROMEDA_PULSE_RETENTION_SECONDS, eviction_count + memory_bytes ticks" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#22 "Query routers (traces/metrics/logs) — viz crate prepared statements (no format-string SQL), query_id + param_count anonymized logging" (Epoch 3 continues)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-18}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Epoch 3 — Storage & query:** OPEN. Chunks #20 + #21 committed; chunks #22, #23 remain.

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #22 listed in route §2 but no `.andromeda/phases/phase-19/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear. State I from new-session (state.yaml.commit_sha "dad9d02" stale) self-heals via this wrap's Phase 10 SHA-fixup amend.

## Drift Detection (6 dimensions)

D1, D2, D3, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because CLAUDE.md mtime is newer than all 9 upstream mtimes. D6 advances cleanly: state.yaml.last_completed_chunk progresses 20 → 21 with this wrap commit.)

## Spec Amendments (this session)

(none this session — chunk #21 implementation matched specialist plan expectations; no Trigger 4 dialogue. Pre-existing archive untouched: lift-accent (2026-05-03) + clarify-pii-grep-ui-vocab (2026-05-04) both already archived per state.yaml.spec_amendments.archive.)

## Key Decisions This Session

- **Retention sweep cadence:** `tokio::time::interval(retention_seconds.max(60) / 6)` — yields 100s default at 600s retention, 10s minimum at 60s retention. Independent of the 15s `buffer.tick` heartbeat cadence; sweep runs on its own interval. First tick is skipped (immediate-fire on `tokio::time::interval` would sweep before any rows ingested — misleading first-tick eviction-of-zero log).
- **`BufferConnectionStatus` widened (chunk #20 → chunk #21 evolution):** renamed `Failed(String)` → `InitFailed(String)` and added `RetentionFailed(String)` variant. Cleaner than reason-string pattern matching at `current_health()`. Touched 5 existing tests + `pulse-app/src/main.rs::init_buffer` call site.
- **`heartbeat_payload` signature change:** old `heartbeat_payload(state)` → new `heartbeat_payload(state, retention_window_seconds, eviction_count_since_last_tick)`. Per-tick parameterization keeps BufferState time-window-agnostic; the heartbeat-output-specific delta lives at the call site (`emit_buffer_tick` in heartbeat.rs).
- **`eviction_count_since_last_tick` delta tracking:** `Arc<AtomicU64>` owned by `run_buffer` task in heartbeat.rs (not BufferState/HeartbeatState). Compute delta via swap-and-compare per tick. Heartbeat-output-specific concern; doesn't belong in BufferState which is module-internal.
- **`memory_bytes` heuristic over `pragma_database_size()` parsing:** `rows_active * 256` (where rows_active = `rows_ingested - eviction_count`). Cheaper than parsing DuckDB's string-typed pragma columns; accurate enough for SLO tracking ≤ 512 MB. Documented in session-learnings.md for future decision-revisit.
- **Retention bounds enforced at env-var resolution:** `[60, 86400]` per security plan; default fallback 600 (arch upper-default of 300-600s range). `resolve_retention_seconds()` returns `u64` directly (not Result) — out-of-range falls back to default rather than blocking app startup.
- **Refactor: extract `run_one_sweep` async helper from `run_retention`:** initial test design hit a tokio paused-clock × `spawn_blocking` × `handle.abort()` race that left `state.eviction_count = 0` despite rows physically evicted. Refactor extracts loop body into a separately-callable async helper; tests call `run_one_sweep(...).await` directly without paused-clock orchestration. The wrapper-loop spawn/abort lifecycle is covered by a single smoke test.
- **`#[tokio::test(start_paused = true)]` requires `flavor = "current_thread"`:** Initial test design used `flavor = "multi_thread", worker_threads = 4, start_paused = true` which fails compile with explicit error. Fix: drop `multi_thread` for paused-clock tests OR refactor away from paused-clock entirely (chosen for retention tests via the `run_one_sweep` extraction).
- **`format!`-grep gate held:** chunk #20's `grep -rnE "format!.*(CREATE|SELECT|INSERT|UPDATE|DELETE)" crates/buffer/src/` returns empty after chunk #21. Per-table DELETE constants (`DELETE_BY_CUTOFF: [&str; 7]`) parallel `RESERVED_TABLES`; cutoff parameter binds via prepared-statement `?` placeholder.

## Files Modified

(13 files this session — chunk #21 implementation + Cargo.lock untouched + 3 new phase-18 artifacts. Living artifacts reconciled separately by wrap.)

**Code files (chunk #21 — Rust):**
- `crates/buffer/Cargo.toml` — adds `tokio = { workspace = true, features = ["test-util"] }` to `[dev-dependencies]` (production tokio.workspace unchanged).
- `crates/buffer/src/state.rs` — `BufferState` gains `retention_window_active: AtomicBool` field + `mark_retention_active()` setter; `BufferStateSnapshot` gains `retention_window_active: bool` field; +3 unit tests.
- `crates/buffer/src/contract.rs` — `Error::Retention { reason: String }` 6th variant; `BufferHeartbeat` extended with `memory_bytes` + `retention_window_seconds` + `eviction_count_since_last_tick` u64 fields; `heartbeat_payload(state, retention_window_seconds, eviction_count_since_last_tick)` signature change; +3 unit tests.
- `crates/buffer/src/consumer.rs` — `describe_error` match adds `Error::Retention => "retention_failed"` arm; +1 test extension.
- `crates/buffer/src/lib.rs` — adds `pub mod retention;` + `pub use retention::run_retention;`.
- `crates/buffer/src/retention.rs` (NEW) — `pub async fn run_retention` (long-running periodic task) + `pub(crate) async fn run_one_sweep` (testable single-sweep helper) + `pub(crate) fn retention_sweep_inner` (synchronous DELETE iteration over 7 tables) + `DELETE_BY_CUTOFF: [&str; 7]` constants + `compute_cutoff_ns` (SystemTime + saturating sub) + `describe_error` (incl. Retention arm) + `BYTES_PER_ROW_ESTIMATE: u64 = 256`; 11 co-located tests including chaos-evicts-partial-subset, poisoned-mutex returns ConnectionLost, all 7 tables iterated, cutoff before/after window correctness, 3 async run_one_sweep tests, 1 spawn-and-abort smoke test for run_retention.
- `crates/ui-bridge/src/contract.rs` — `From<BufferError> for AppError` extends with `BufferError::Retention { .. } => "buffer retention sweep failed"` arm; +1 sanitization test (no DuckDB error / version / path leakage).
- `crates/ui-bridge/src/health.rs` — `BufferConnectionStatus` enum widened: `Failed(String)` renamed to `InitFailed(String)` + new `RetentionFailed(String)` variant; `current_health()` mapping updated to surface `subsystems.buffer.status = "retention_failed"` for the new variant; existing 5 tests updated for rename + 2 new tests.
- `pulse-app/src/main.rs` — `ENV_RETENTION_SECONDS` const + `RETENTION_SECONDS_MIN/MAX/DEFAULT` constants + `resolve_retention_seconds() -> u64` helper (mirrors `resolve_port` pattern with bounds rejection + sanitized warn at `config.load.retention_seconds` target); `tauri::async_runtime::spawn(run_retention(...))` added in `Some(conn)` arm parallel to `run_consumer` spawn; `BufferConnectionStatus::Failed(...)` call sites updated to `InitFailed(...)`; +7 tests for resolve_retention_seconds.
- `pulse-app/src/heartbeat.rs` — `run_buffer` + `emit_buffer_tick` signatures extended with `retention_seconds: u64` arg + `last_eviction: &AtomicU64` for delta tracking; `emit_buffer_tick` emits 6 fields on `buffer.tick` (rows_ingested + retention_window_active + eviction_count + memory_bytes + retention_window_seconds + eviction_count_since_last_tick) + 2 metric events per tick (`metric.buffer.memory_bytes` always; `metric.buffer.evicted_span_count` only when delta > 0); `spawn` signature gains `retention_seconds`; +6 new tests.
- `pulse-app/src/observability.rs` — `AllowList::production().by_target` `buffer` entry extended with `eviction_count_since_last_tick`; 5 new registry entries (`buffer.retention.sweep` / `buffer.retention.sweep.error` / `metric.buffer.memory_bytes` / `metric.buffer.evicted_span_count` / `config.load.retention_seconds`); 10 new co-located tests (5 pass + 5 redact mirroring chunk #20 precedent).

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-18/{combined.md, research.md, plan.md}` (NEW) — Phase 18 planning artifacts for chunk #21 (204 + 88 + 194 lines).

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent` (refreshed; buffer crate's direct dep tree unchanged from chunk #20 baseline — arrow + duckdb + ingest + thiserror + tokio + tracing + tracing-error in `[dependencies]`, rstest + tempfile + tokio in `[dev-dependencies]`).
- `.andromeda/context/api-surface.md` — reconciled via per-crate `cargo public-api --simplified`. Buffer surface gains `pub mod buffer::retention` + `pub async fn buffer::run_retention(...)` + `Error::Retention` variant + 3 new BufferHeartbeat fields + `BufferState::mark_retention_active` + `BufferStateSnapshot::retention_window_active`. Ui-bridge surface: `BufferConnectionStatus::Failed → InitFailed` rename + new `RetentionFailed(String)` variant. Format simplified to focus on actual public API (cargo build noise prefix dropped — keeps file scannable for integrity-check purposes).
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advanced to 21 + epoch 3 progresses; commit_sha advances from `"dad9d02"` (stale chunk #20 placeholder) → `"pending"` → real SHA via Phase 10 amend; session_count=21; spec_amendments.{active,archive} unchanged; drift_warnings empty; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-06T18:30:00Z.
- `.claude/rules/testing.md` — Session Additions extended with 2026-05-06 entry on `start_paused × multi_thread` incompatibility (Tier 2).
- `.claude/docs/session-learnings.md` — prepended 2 entries (extract async helper pattern; memory_bytes heuristic decision) (Tier 3).
- `.claude/session-handoff.md` — this file.

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition
  - testing.md: `#[tokio::test(start_paused = true)]` requires `flavor = "current_thread"` (incompatible with multi_thread)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions
  - (a) Extract async helper from periodic-task loop body for unit-testability — sidesteps tokio paused-clock × spawn_blocking × abort race
  - (b) `memory_bytes` heuristic: `rows_active * 256` chosen over `pragma_database_size()` parsing for SLO tracking simplicity
- **Filtered:** 1 candidate rejected via Filter 1 dedup — chunk #19's existing testing.md 2026-05-03 entry on `start_paused` requiring `test-util` feature already covers part of the same surface; merged into 2026-05-06 entry rather than duplicating.

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 238/238 workspace, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok, cargo audit ok with 18 pre-existing allowed warnings, cargo tree -p buffer | grep opentelemetry empty (obs criterion #7 strict-grep PRESERVED), cargo tree -p buffer --depth 1 unchanged from chunk #20 baseline, grep -rnE "format!.*(CREATE|SELECT|INSERT|UPDATE|DELETE)" crates/buffer/src/ empty, grep -rnE "(cutoff|retention).*(format!|to_rfc3339|to_string).*tracing" empty.)

## Tests Status

passing — 238 cargo nextest workspace (was 195 last wrap, +43 from chunk #21: 3 BufferState retention_window_active tests, 3 contract.rs heartbeat extension tests, 11 retention.rs tests, 1 ui-bridge AppError From<BufferError::Retention> sanitization test, 2 ui-bridge health BufferConnectionStatus widening tests + 5 existing renamed Failed→InitFailed, 6 heartbeat.rs emit_buffer_tick extension tests, 10 observability.rs AllowList tests for new targets, 7 main.rs resolve_retention_seconds tests, 1 consumer describe_error extension test, 1 retention.rs describe_error_returns_constant_strings_including_retention) + cargo deny ok (skip list unchanged from chunk #20) + cargo audit ok + cargo clippy clean + cargo fmt clean. Total: 238 tests + 4 lint/typecheck gates + 2 supply-chain gates = 244 checks. cargo nextest --workspace ~0.85s parallel.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #22:**

`/andromeda-phase` to plan chunk #22 "Query routers (traces/metrics/logs) — viz crate prepared statements (no format-string SQL), query_id + param_count anonymized logging". Builds on chunk #21's `BufferState` heartbeat surface (existing `subsystems.buffer.retention_seconds` field will be consumed by `traces.query` etc. routers) and the prepared-statement-only invariant established for the buffer crate (the same discipline applies to viz crate query routers).

**Priority 2 (background, optional) — chunk #22 prep verification:**

Before chunk #22 starts, re-validate that parameterized BLOB equality queries work against Arrow-appended data. The chunk #20 round-trip test had to switch from `WHERE col = X'…'` to `LIMIT 1` because Arrow-appended BLOBs don't match hex literals. Chunk #22 query routers (e.g., `traces.query_by_trace_id`) will need parameterized BLOB queries — confirm `stmt.query_row(params![&[u8]_slice], …)` works against Arrow-appended BLOBs OR find the right CAST/conversion. The chunk #21 retention test seeded rows via SQL `INSERT` (not Arrow appender), so this test surface remains untouched.

## Session Goals (carry-over)

(none — chunk #21 fully implemented + tests green + curation applied (1 Tier 2 + 2 Tier 3) + reconcile complete + Epoch 3 chunk #21 closes; chunk #22 next; ready for `/andromeda-phase`)

## Session End Status

clean
