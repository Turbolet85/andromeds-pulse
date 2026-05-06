# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-05-06 — RecordBatch reuse refactor: extract `build_*_record_batch` from `append_*_batch` to enable fan-out

When a producer crate needs to emit the same Arrow `RecordBatch` data to multiple sinks (e.g., chunk #23: DuckDB persist via `Connection::appender(...).append_record_batch(...)` AND tokio broadcast emit via Arrow IPC StreamWriter byte stream), refactor any existing single-sink `append_*_batch(conn, proto_input) -> Result<u64, Error>` function into two pieces:

1. **Builder:** `build_*_record_batch(proto_input) -> Result<Option<RecordBatch>, Error>` — does the proto → Vec<column-wise> → RecordBatch::try_new construction; returns `None` for zero-row inputs (matches existing semantic of "0 rows = no-op").
2. **Persister:** `append_record_batch_to_table(conn, table_name: &'static str, batch: RecordBatch) -> Result<u64, Error>` — does the DuckDB `appender(table_name).append_record_batch(batch).flush()` work; returns row count from `batch.num_rows()`.

The original `append_*_batch` becomes a thin compose layer (`build → append → tracing log`). Caller for chunk #23-style fan-out flows (`crates/buffer/src/consumer.rs::dispatch_batch`) bypasses the wrapper entirely: build once, encode for broadcast (via `crate::broadcast::encode_*(&record_batch)`), append the (cloned) RecordBatch to DuckDB, emit broadcast bytes if encode-Ok and append-Ok.

Coordination invariant: encode happens BEFORE append (so encode failure aborts the whole flow), but emit happens AFTER append (so subscribers only see durably-stored data). RecordBatch::clone is cheap (Arc bump on the underlying buffers), so the build → clone → append + clone → encode pattern is roughly O(1) extra overhead.

Side effect: with the production path going through builders + writer directly, the old wrappers `append_*_batch` may become unused in production code (only the co-located tests still call them). See the cfg(test) gating learning below for the workflow follow-up.

See: `crates/buffer/src/appender.rs::build_spans_record_batch / build_metrics_record_batch / build_logs_record_batch / append_record_batch_to_table`; `crates/buffer/src/consumer.rs::dispatch_batch` chunk #23 fan-out path.

---

## 2026-05-06 — `#[cfg(test)]` gating of test-only API wrappers after refactor (dead-code under -D warnings)

When extracting a public-API function into helpers + a thin wrapper, the wrapper may end up unused by production code (only co-located tests call it). Rust's `dead_code` lint will fire, and clippy's `-D warnings` gate will reject the build. Solution: gate the wrapper with `#[cfg(test)]`. The wrapper preserves existing test ergonomics + signature; production path bypasses it via the helpers.

Same gating applies to imports newly needed only in test paths. The chunk #23 buffer/appender refactor moved `Instant::now()` calls from the production wrappers into cfg(test)-only territory; the `use std::time::Instant` import then needed `#[cfg(test)]` too:

```rust
use std::sync::Arc;
#[cfg(test)]
use std::time::Instant;
```

Diagnostic shape: `warning: function 'append_spans_batch' is never used` + `warning: unused import: 'std::time::Instant'`. Without gating, both fire as warnings under default rustc, which clippy promotes to errors via `-D warnings`.

Pattern generalizes to refactor-time discipline: when extracting helpers from existing API, audit whether the OLD entry-point (and its imports) is still called from production. If only tests call it, gate with `#[cfg(test)]`. If genuinely unused (no callers anywhere), delete it outright (per CLAUDE.md "no half-finished implementations / TODO panics" guidance) — keeping it cfg(test)-gated is the right move only if tests legitimately need the compose layer.

See: `crates/buffer/src/appender.rs` chunk #23 — `append_{spans,metrics,logs}_batch` wrappers cfg(test)-gated after extraction; `Instant` import gated; chunk #23 fix-loop iteration #2.

---

## 2026-05-06 — TauRPC + tokio broadcast + Tauri Channel API binary-payload forwarding pattern

The chunk #23 push-stream surface (`pulse://stream/{spans,metrics,logs}`) wires three components:

1. **`tokio::sync::broadcast::Sender<bytes::Bytes>`** in the producer crate (buffer): one Sender per stream, capacity 128. After successful DuckDB append, encode the RecordBatch via `arrow::ipc::writer::StreamWriter` to a `bytes::Bytes` payload (with 8 MB cap check), then call `senders.{spans|metrics|logs}.send(bytes)`. SendError when no subscribers — silently drop via `let _ = sender.send(...)`.
2. **TauRPC `#[taurpc::procedures(path = "streams")]`** in the binary crate (`pulse-app/src/streams.rs`) with 3 procedures `subscribe_{spans,metrics,logs}(channel: tauri::ipc::Channel<Vec<u8>>) -> Result<(), AppError>`. Tauri 2.11 + taurpc 0.7 accepts `Channel<Vec<u8>>` as a procedure parameter without special handling; webview creates a Channel via `new Channel<Uint8Array>()`, passes it as the procedure arg, and the procedure stores the handle.
3. **Forwarding task** spawned at procedure entry: clone the relevant `broadcast::Sender`, call `.subscribe()` to get a `Receiver`, then `tokio::spawn(forward_loop(stream_name, receiver, channel))`. The loop: `match receiver.recv().await { Ok(bytes) => { /* size cap re-check, payload = bytes.to_vec(), channel.send(payload), tracing::info! tauri.channel.emit */ }, Err(Lagged(n)) => tracing::warn! tauri.channel.lag, Err(Closed) => break }`. Channel send error (webview disconnect) → break loop, exit task, drop Receiver, decrement subscriber count via `Sender::receiver_count()` natural decay.

Two notable trip-ups during impl:

- **`bytes::Bytes` does NOT implement Serialize**, so `Channel<bytes::Bytes>` doesn't compile. Use `Channel<Vec<u8>>` and convert via `bytes.to_vec()` at the send site. Trade-off: one Vec allocation per emission per subscriber. For 3 subscribers × 10k events/sec ≈ 30k allocs/sec — acceptable within tokio scheduling overhead headroom; revisit only if profiling shows hot-path cost.
- **Subscriber count tracking** lives in `IngestState.broadcast_subscribers` (chunk #18 precedent — single AtomicU32 representing total across streams). Per-tick heartbeat polls `broadcast_senders.{spans|metrics|logs}.receiver_count()` and sums into `IngestState.set_broadcast_subscribers(total_subs as u32)` before emitting `ingest.tick`. Per-stream visibility achieved via separate `metric.ingest.channel.broadcast_subscribers` events with enumerated `channel_name` field — does NOT use unbounded labels per obs cardinality discipline.

Pattern is reusable for any future scope-arch chunk that needs binary push from backend to webview without JSON-stringify tax. Avoid `tauri::Manager::emit(event_name, payload)` for bulk binary data — emit serializes to JSON regardless of T (Vec<u8> becomes a JSON array of u8s).

See: `pulse-app/src/streams.rs` (TauRPC trait + StreamsApiImpl + forward_loop); `crates/buffer/src/broadcast.rs` (Sender trio + encoders + cap); `pulse-app/src/heartbeat.rs::emit_ingest_tick` (subscriber-count poll); chunk #23 plan.md §Implementation Steps 8 + research.md "Open questions".

---

## 2026-05-06 — TauRPC trait+impl pairs belong in the binary crate, not in producer library crates (cargo feature-unification cycle)

When a TauRPC API is exposed by a library crate's content (viz query types + Error, future scope-crate types, etc.), the natural Rust instinct is to colocate the `#[taurpc::procedures] pub trait Api` + `#[taurpc::resolvers] impl Api for ApiImpl` with the data types in the same library crate. This works for `ui-bridge` because ui-bridge OWNS the `AppError` type that procedures return — the trait can be feature-gated and reference AppError directly (see `crates/ui-bridge/src/health.rs::runtime` mod under `#[cfg(feature = "taurpc-runtime")]`).

For peer library crates (viz, future scope crates), procedures still must return `Result<T, AppError>` per arch §Conventions. AppError lives in ui-bridge. So the producer's runtime module needs ui-bridge as a dep. Meanwhile ui-bridge already depends on the producer for `From<ProducerError> for AppError` (per the chunk #18 sibling-dep precedent already documented in this file). This APPEARS solvable via cargo features:

- viz declares feature `taurpc-runtime` → activates optional dep on `ui-bridge`
- ui-bridge → declares dep on `viz` with `default-features = false` (no `taurpc-runtime` active)

Cargo's feature unification breaks this: when pulse-app activates viz's `taurpc-runtime` feature, the unification rule requires EVERY copy of viz across the workspace to share the same feature set. ui-bridge's viz copy thus also gets `taurpc-runtime` active → that viz copy depends on ui-bridge → ui-bridge depends on viz-with-`taurpc-runtime` → CYCLE. Cargo rejects.

Resolution (chunk #22 implement-time deviation from plan): place the TauRPC trait+impl pairs in the binary crate at `pulse-app/src/{name}_routers.rs`. The binary crate already depends on every library crate; the routers module imports types from the producer crate (`use viz::{TracesQueryArgs, ...}`) and constructs the impl with the orchestration handles (`Arc<Mutex<Connection>>` + `Arc<ProducerState>`) the binary already holds. Producer crate stays cycle-free with no `taurpc-runtime` feature. ui-bridge keeps the unconditional `From<ProducerError> for AppError` impl. Mirrors how `pulse-app/src/main.rs` already orchestrates the existing HealthApi (which colocates with its data types in ui-bridge — colocation works for ui-bridge specifically because ui-bridge owns AppError).

Future scope-arch additions of new TauRPC routers in producer crates should default to placing trait+impl in pulse-app from the start, NOT in the producer crate behind a `taurpc-runtime` feature. Plan templates that propose feature-gated cycles need an implement-time verification step (cargo check the workspace under both feature configurations) before assuming cargo will resolve.

See: `pulse-app/src/viz_routers.rs` (TracesApi/MetricsApi/LogsApi triplet); chunk #22 plan.md step 5 (planned `crates/viz/src/runtime.rs`) deviated to actual `pulse-app/src/viz_routers.rs`; sibling pattern at `crates/ui-bridge/src/health.rs::runtime` works ONLY because ui-bridge owns AppError.

---

## 2026-05-06 — taurpc::procedures macro needs serde + specta crates at the call-site crate, plus specta::Type on every touched type

The `#[taurpc::procedures(path = "...")]` attribute macro (taurpc 0.7) emits code that references `taurpc::serde::Serialize`, `specta::Type`, and `specta::function::specta_fn::SpectaFn` directly by path. At macro expansion, these paths resolve via the call-site crate's `[dependencies]` — having `taurpc` in `[dependencies]` is NOT enough. The compiler errors are misleading because they point at the attribute macro line, not the missing dep:

- `error[E0463]: can't find crate for `serde`` (note: `this error originates in the derive macro `taurpc::serde::Serialize``) → add `serde.workspace = true` to the call-site crate
- `error[E0433]: cannot find module or crate `specta``  → add `specta.workspace = true` to the call-site crate
- `error[E0277]: the trait bound `MyType: specta::Type` is not satisfied` (note: `required for `MyType` to implement `FunctionArg``) → derive `specta::Type` on every argument and result type of every procedure

For pulse-app (the binary crate that hosts TauRPC trait+impl pairs per the cycle-break pattern), this meant adding `serde.workspace = true` + `specta.workspace = true` to `[dependencies]` even though pulse-app doesn't directly use the `serde` or `specta` types — they're invoked entirely via taurpc's emitted macro paths.

For producer library crates (viz, future scope crates), every public type crossing a TauRPC procedure surface must derive `specta::Type` — typically alongside `serde::{Serialize, Deserialize}`. Generic wrapper types like `PaginatedResponse<T>` need `T: specta::Type` bound on the type parameter (Rust derives this automatically via `#[derive(specta::Type)]` on the generic struct, but the bound becomes part of the public API — every concrete instantiation must satisfy it).

Two derive strategies, both seen in this workspace:

- **Conditional (ui-bridge precedent):** `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` — gates the derive to feature-active builds. Used by `HealthEnvelope`/`HealthStatus`/`SubsystemStatus`. Useful when the type is occasionally used outside taurpc contexts and the specta dep cost is unwanted in those builds.
- **Unconditional (chunk #22 viz precedent):** `#[derive(serde::Serialize, serde::Deserialize, specta::Type)]` always. Simpler when the producer crate has no `taurpc-runtime` feature (because trait+impl lives in pulse-app per the cycle-break pattern). Cost: specta becomes a hard dep of viz and any crate that depends on viz.

Choose conditional when the producer crate may be reused in non-taurpc contexts (mcp-server hypothetically, or stdlib-only consumers). Choose unconditional when the producer is in this workspace's pure TauRPC-IPC pipeline only.

See: `crates/viz/src/query.rs` derives (TracesQueryArgs/MetricsQueryArgs/LogsQueryArgs/PaginatedResponse/TraceRow/MetricRow/LogRow); `pulse-app/Cargo.toml` `[dependencies] serde.workspace = true; specta.workspace = true`; chunk #22 fix-loop iterations 2 + 3.

---

## 2026-05-06 — Extract async helper from periodic-task loop body for unit-testability

When an async task wraps a periodic loop with `tokio::time::interval(...).tick().await` + `tokio::task::spawn_blocking(...)` calls inside, unit tests using `tokio::time::pause()` + `tokio::time::advance()` reliably race with the spawn_blocking thread + the test's `handle.abort()`. The chunk #21 retention task hit this: `run_retention(conn, state, retention_seconds)` spawned blocking DuckDB DELETE work whose completion didn't reliably reach the `state.record_eviction(rows)` call before the abort fired, leaving `state.eviction_count = 0` in tests despite rows being physically evicted.

Resolution: extract the loop body (one tick worth of work) into a separately-callable async helper. For chunk #21:

```rust
pub async fn run_retention(conn, state, retention_seconds) {
    let mut interval = tokio::time::interval(...);
    interval.tick().await;  // skip immediate first tick
    loop {
        interval.tick().await;
        run_one_sweep(&conn, &state, retention_seconds).await;
    }
}

pub(crate) async fn run_one_sweep(conn, state, retention_seconds) {
    // spawn_blocking + state updates + tracing — full sweep deterministic on `.await`
}
```

Tests then call `run_one_sweep(&conn, &state, 60).await` directly — no paused clock, no interval orchestration, no abort race. The behavior is exactly one sweep + state record + tracing event, which is what the test wants to verify. The smoke test `run_retention_can_be_spawned_and_aborted_cleanly` covers the wrapper-loop's spawn/abort lifecycle as a separate concern.

Pattern generalizes to any async task that wraps a periodic body. The `pub(crate)` visibility on `run_one_sweep` keeps the abstraction from leaking into the public surface while still being testable from co-located `mod tests`.

See: `crates/buffer/src/retention.rs::run_one_sweep`; chunk #21 fix-loop iteration #1.

---

## 2026-05-06 — `memory_bytes` heuristic: `rows_active * 256` over `pragma_database_size()` parsing

DuckDB's `pragma_database_size()` returns multiple columns (`database_name`, `database_size`, `block_size`, `total_blocks`, `used_blocks`, `free_blocks`, `wal_size`, `memory_usage`, `memory_limit`) where the size-shaped columns (`database_size`, `wal_size`, `memory_usage`, `memory_limit`) are STRINGS like `"0 bytes"`, `"1.2 KiB"`, `"1.0 GiB"`. Parsing them requires unit-string matching (KiB / MiB / GiB / TiB) and float-to-bytes conversion. For a `memory_bytes` heartbeat gauge tracked at 15s cadence, the parse cost + the inherent imprecision of the human-readable formatting argues for a simpler heuristic.

Chunk #21 chose: `memory_bytes = (rows_ingested - eviction_count) * 256` where 256 is an empirical bytes-per-row estimate (composite BLOB PK + 2 timestamp columns + a few attribute columns averages around this range across the 7 reserved tables). This is monotonic with row count, requires no DuckDB pragma parse, and tracks well-enough with actual buffer memory for SLO purposes (the `metric.buffer.memory_bytes` ≤ 512 MB SLO is a coarse upper bound, not a precise accounting).

If a future need surfaces precise byte accounting (e.g., chunk-level memory profiling for performance regression CI), revisit by parsing `pragma_database_size().memory_usage` — but expect to invest in a unit-string parser that handles all DuckDB-emitted size formats.

See: `crates/buffer/src/retention.rs::run_one_sweep` (`set_memory_bytes(rows_active.saturating_mul(BYTES_PER_ROW_ESTIMATE))`); chunk #21 plan §Implementation notes "memory_bytes measurement source".

---

## 2026-05-06 — DuckDB 1.10502 hangs `INSERT` on duplicate composite-BLOB primary key

The `duckdb` crate 1.10502.0 (DuckDB 1.5 bundled C++) on Windows MSVC enters an unbounded loop when an `INSERT` would violate a `PRIMARY KEY (col1 BLOB, col2 BLOB)` composite. The first insert succeeds; the duplicate insert never returns from `Connection::execute()` / `execute_batch()` — observed via cargo nextest's `SLOW [>2400.000s]` reports on the chunk #20 buffer crate `spans` table (composite PK over `trace_id BLOB(16)` + `span_id BLOB(8)`). Single-INSERT into the same table works fine, both via SQL `INSERT … VALUES (X'…')` and via the Arrow appender (`Connection::appender("spans")?.append_record_batch(...)`). Only the PK-violation path on multi-column BLOB PK hangs. Workaround: assert composite-PK structure via schema introspection (`information_schema.key_column_usage` filtered to `table_schema = 'main' AND table_name = '<table>'`, asserting `column_name` set + `ordinal_position` count) instead of behavioral runtime PK-violation tests.

The chunk #20 schema test `spans_primary_key_is_composite_trace_id_span_id` originally inserted-then-duplicated; rewritten to query `key_column_usage` and assert (a) exactly 2 columns in PK, (b) names contain `trace_id` AND `span_id`. Pattern generalizes to any future schema test that needs to assert composite PK on BLOB-typed columns: prefer information_schema introspection over behavioral PK-violation paths until DuckDB upstream confirms / fixes the issue. Rust `cargo nextest` reports SLOW indefinitely without timeout; use cargo nextest's per-test slow-timeout config or kill the test binary manually. Direct `target/debug/deps/buffer-{hash}.exe` invocation reproduces the hang outside nextest, ruling out test-runner parallelism as cause.

See: `crates/buffer/src/schema.rs::tests::spans_primary_key_is_composite_trace_id_span_id`; `information_schema.key_column_usage` filter discipline; chunk #20 fix-loop iteration #6.

---

## 2026-05-06 — libduckdb-sys 1.10502 needs `rstrtmgr.lib` link hint on Windows MSVC

The `libduckdb-sys` crate 1.10502.0 (DuckDB C++ build) on the `x86_64-pc-windows-msvc` target references Restart Manager APIs (`RmStartSession` / `RmEndSession` / `RmRegisterResources` / `RmGetList` from `Rstrtmgr.dll`) inside `duckdb::AdditionalLockInfo` but does NOT emit the corresponding `rstrtmgr.lib` link directive from its own `build.rs` for downstream test-binary linkage. Linking the consumer crate's lib succeeds (the symbols stay unresolved-but-tolerated until binary link), but the test binary link step fails with `LNK2019 unresolved external symbol` for all four symbols. Workaround: add a `build.rs` to the consuming crate that emits `cargo:rustc-link-lib=dylib=rstrtmgr` when `CARGO_CFG_TARGET_OS == "windows"`. The chunk #20 buffer crate ships `crates/buffer/build.rs` with exactly this guard.

Pattern generalizes to any future workspace crate that takes `duckdb` (or any libduckdb-sys-bundled dep) as a direct or transitive dep with bundled C++ on Windows MSVC. The Linux + macOS targets do not need this — Restart Manager is Windows-specific. Diagnostic shape: `error: linking with link.exe failed: exit code: 1120` followed by `LNK2019 unresolved external symbol Rm{Start|End|RegisterResources|GetList}Session`. If a future libduckdb-sys version fixes its own `build.rs` to emit the link directive (would manifest as `print-cargo:rustc-link-lib=dylib=rstrtmgr` in `cargo build -vv` for libduckdb-sys), the workaround can be removed.

See: `crates/buffer/build.rs`; chunk #20 fix-loop iteration #5.

---

## 2026-05-06 — DuckDB Arrow-appended BLOB does not match `WHERE col = X'…'` hex literal

When the `duckdb` crate Arrow appender (`Connection::appender("table")?.append_record_batch(record_batch)?`) inserts a `BLOB` column from an `arrow::array::BinaryArray`, the resulting stored bytes do NOT match a `WHERE col = X'…'` hex BLOB literal in subsequent SELECT queries — the SELECT returns `QueryReturnedNoRows` even though `SELECT COUNT(*) FROM table` reports the row IS present. SQL-INSERT'd BLOB literals (`INSERT … VALUES (X'…', …)`) and SELECT WHERE hex-literal pairings DO match each other; Arrow-appended BLOB and hex-literal SELECT do NOT. Root cause unverified but consistent with the Arrow → DuckDB BLOB conversion using a different internal storage encoding (e.g., length-prefixed inline vs out-of-line variable-length representation) that the hex-literal-based equality check doesn't normalize across.

Workaround for round-trip tests: don't use `WHERE col = X'…'` on Arrow-appended BLOBs. Read back via `SELECT col, … FROM table ORDER BY ts_unix_nano LIMIT 1` (or LIMIT N + collect rows) and assert on the OTHER columns (timestamps, integer IDs, varchar names). Equality via parameter binding (`stmt.query_row(params![&[u8]_slice], …)`) was NOT tested as workaround — separately known to hang per the chunk #20 PK-on-BLOB issue, so it can't isolate the encoding question. The chunk #20 `append_spans_batch_round_trips_nanosecond_precision` test uses LIMIT 1 + `row.get::<_, i64>(0)` for `ts_unix_nano` exactly because of this constraint.

Implication: any future query router (chunk #22+) that needs to filter spans by `trace_id` BLOB (e.g., `traces.query_by_trace_id`) must validate Arrow-appended BLOBs match the parameter-binding path before assuming `WHERE col = ?` works. Likely the proper path is `WHERE col = CAST(? AS BLOB)` or DuckDB's specific BLOB binding in the duckdb crate's prepared-statement API. Plan chunk #22 acceptance criteria should explicitly probe this before relying on parameterized BLOB queries.

See: `crates/buffer/src/appender.rs::tests::append_spans_batch_round_trips_nanosecond_precision`; chunk #20 fix-loop iteration #7.

---

## 2026-05-05 — `governor` crate uses real-time clock; not mockable via `tokio::time::pause()`

The `governor` crate (used transitively by `tower_governor` 0.8 for OTLP receiver rate limiting per route#19) uses a `quanta`-backed monotonic clock (`governor::clock::DefaultClock` → `QuantaInstant`) for token-bucket replenishment. This clock is independent of tokio's runtime clock; calling `tokio::time::pause()` + `tokio::time::advance(Duration)` does NOT freeze or fast-forward governor's view of time. Rate-limit window assertions therefore cannot use the testing.md "use `tokio::time::pause()` for time-sensitive tests" pattern — saturation/recovery tests must use real-time short sleep with bounded windows.

The chunk #19 `crates/ingest/tests/rate_limit.rs` integration tests use `TIGHT_PERIOD = Duration::from_millis(100)` + `TIGHT_BURST_SIZE = 2` + `RECOVERY_WAIT = Duration::from_millis(250)` — total real-time wall cost ~250-400ms per test, comfortably bounded. The testing.md "NEVER `sleep(N)` for sync" rule applies to event-waiting synchronization (poll for state change); time-elapsed-behavior testing on a real-clock-backed library is a distinct use case where real time IS the canonical signal. Document the deviation in the test file's module docstring; do NOT add the testing.md rule's `tokio = { features = ["test-util"] }` dev-dep just for governor tests — the feature flag wouldn't help.

If a future external middleware library exposes a `Clock` trait or `governor::clock::FakeRelativeClock` becomes accessible through `tower_governor`'s public API, prefer that path; until then, real-time bounded windows are the working pattern. Pattern generalizes to any future timing test against a non-tokio-clock library.

---

## 2026-05-05 — Sibling-isolation grep gates over-specified when permitted DAG edge exists

Plan acceptance criteria of the form `cargo tree -p {sibling_crate} | grep {dep} returns empty` are too coarse when the workspace has a permitted sibling-DAG edge. Concrete case (chunk #19): the criterion `cargo tree -p ui-bridge | grep tower_governor returns empty` was unachievable given the existing `ui-bridge → ingest` sibling dep edge (chunk #18's `From<IngestError> for AppError` impl in `crates/ui-bridge/src/contract.rs` per arch §Conventions Error response schema (Tauri IPC) From-impl-as-contract). Since `ingest` carries `tower_governor` as a direct dep, the whole-tree grep MUST match transitively through ui-bridge → ingest → tower_governor.

The intent (no DIRECT tower_governor dep on ui-bridge) is captured better by:
- `cargo tree -p ui-bridge --depth 1 | grep tower_governor` returns empty (only direct deps), OR
- `grep tower_governor crates/ui-bridge/Cargo.toml` returns empty (declaration check).

Both succeed for chunk #19's actual implementation (tower_governor declared only in `crates/ingest/Cargo.toml`). When future plans assert sibling-isolation, prefer one of these forms. The whole-tree grep is appropriate ONLY when the sibling pair has NO permitted dep edge between them. Document the edge in plan.md "Files к leave untouched" or research.md "Conventions to follow" section to make the constraint visible at planning time.

---

## 2026-05-05 — `tokio::sync::mpsc::Sender::capacity()` returns FREE slots, not used

The Tokio mpsc bounded-channel `Sender::capacity()` method returns the number of currently-available slots (free count), NOT the number of queued messages (used count). This is opposite of what most "capacity" mental models suggest — a bounded channel built with `mpsc::channel(1024)` reports `capacity() == 1024` when empty and `capacity() == 0` when full. To compute "% used" for instrumentation (the obs-plan §3 `buffer_capacity_pct` field on the `ingest.tick` heartbeat carries this), the formula is `(total - sender.capacity()) / total * 100.0`, where `total` is the original constructor argument (NOT exposed by the Sender directly — must be tracked by the caller). The chunk #18 `IngestSender` wrapper at `crates/ingest/src/channel.rs` stores the constructor capacity alongside the inner sender exactly because the Tokio API doesn't surface it; without that snapshot, capacity_pct calculation is impossible.

Implication for future channel-introspection code: any wrapper around `tokio::sync::mpsc::Sender` that wants to report "fullness" must capture the constructor capacity at build time. `Sender::max_capacity()` does NOT exist on stable as of tokio 1.x; only `capacity()` (free) and `len()`-style methods on the receiver side exist. The wrapper-with-snapshot pattern from `ingest::channel::IngestSender` generalizes to any future bounded mpsc that needs introspection.

See: `crates/ingest/src/channel.rs::IngestSender::capacity_pct`; tokio docs `tokio::sync::mpsc::Sender::capacity` (returns free, not used).

---

## 2026-05-05 — ui-bridge → ingest sibling crate dep is permitted because the From impl IS the declared contract

Arch §Cross-cutting Patterns "Module dependency direction" states the workspace dep graph is a DAG with `pulse-app` as the only root, AND "no library crate depends on a sibling unless its declared contract requires it". Chunk #18 introduced `ingest = { path = "../ingest" }` to `crates/ui-bridge/Cargo.toml` — the only sibling-crate edge in the workspace as of session 18. The justification: `From<ingest::contract::Error> for AppError` impl lives in `crates/ui-bridge/src/contract.rs` because arch §Conventions "Error response schema (Tauri IPC)" mandates that `From` impls collapsing module-internal `thiserror` enums to `serde`-friendly `AppError` variants live in the bridge crate (where `AppError` is owned). The From impl IS the declared contract that the dependency edge serves; without it, ui-bridge cannot perform the boundary conversion `pulse-app/src/main.rs` (and future TauRPC procedure call-sites) need.

Future-self gotcha when reading `crates/ui-bridge/Cargo.toml` and wondering "wait, why does ui-bridge depend on ingest?" — the answer is the From impl. The same pattern would apply if/when `From<buffer::Error> for AppError` or `From<viz::Error> for AppError` becomes necessary (Epoch 3 buffer chunk lands a similar impl). Each new module-error-to-AppError conversion adds a sibling-dep edge from ui-bridge to that module's crate; the DAG-discipline language permits this as "declared contract" exception.

See: `crates/ui-bridge/Cargo.toml` `[dependencies] ingest = { path = "../ingest" }`; `crates/ui-bridge/src/contract.rs::From<IngestError> for AppError`; arch.md §Cross-cutting Patterns + §Conventions "Error response schema (Tauri IPC)".

---

## 2026-05-05 — axum 0.8 + tonic 0.14 share tower 0.5 + hyper 1 cleanly (no transitive deny duplicate)

When chunk #17 introduced `axum = "0.8"` + `tower = "0.5"` + `tower-http = "0.6"` alongside the existing `tonic = "0.14"` + `tokio-stream` + `tonic-prost` ingest stack, the expected risk was that `cargo deny check bans` (`multiple-versions = "deny"`) would fire on a transitive `tower 0.4 vs 0.5` or `hyper 0.14 vs 1` duplicate. It did not — the resolved dep graph contains exactly one `tower 0.5` + one `hyper 1` + one `http 1` shared across both receivers. axum 0.8 and tonic 0.14 are version-aligned by design (both target hyper 1 + tower 0.5 + http 1 simultaneously). The pre-existing `deny.toml [bans] skip` list (with the chunk #16 `foldhash` provenance entry) did not need extension for chunk #17.

Implication for future Epoch 2-4 chunks: when adding HTTP/web infrastructure crates that need to coexist with the OTLP/gRPC stack, prefer versions that target hyper 1 + tower 0.5 + http 1 to maintain this clean unification. The `tonic <0.14` deny canary at `deny.toml [bans] deny` continues to enforce the original OTLP-receiver invariant — that line is the canonical anchor for "we use the tonic 0.14 + hyper 1 + tower 0.5 stack only".

Note: `cargo check` output during chunk #17 showed `Checking reqwest v0.13.3` AND `Checking reqwest v0.12.28` (12.x added directly as dev-dep for HTTP integration tests; 13.x pulled transitively by tauri-plugin-updater 2.10's HTTP client). `cargo deny check bans` did NOT fire — the resolver appears to have a tolerance carve-out for dev-dep duplicates that don't enter the production binary's link graph (or the duplicate is benign for this skip-list configuration). No action required.

See: `Cargo.toml` `[workspace.dependencies]` Ingest pipeline + OTLP HTTP receiver sections (route#16 + route#17 dep blocks); `deny.toml` `[bans] deny tonic <0.14` canary (security plan §Dependency Security Pinning).

---

## 2026-05-05 — `axum::Router::layer` chains apply outermost-LAST (each .layer() call wraps the previous)

`Router::new().route(...).layer(L1).layer(L2).layer(L3)` produces a service stack where on the request side, L3 runs first (outermost), then L2, then L1, then the handler; on the response side, the reverse. Each `.layer()` call WRAPS the previous layer, so the LAST `.layer()` chained becomes the OUTERMOST middleware. Without understanding this, middleware ordering goes wrong — e.g., placing `DefaultBodyLimit` BEFORE the Host-header allowlist in code-order means the body-limit check runs INSIDE (closer to handler) and the host check runs OUTSIDE (rejects first). The intuitive reading is reversed.

For the OTLP HTTP receiver at `crates/ingest/src/http.rs::build_router`, the desired security ordering is: tracing instrumentation outermost (so all rejected requests still emit boundary spans for observability), then Host-header allowlist (reject DNS-rebinding attempts before body read), then DefaultBodyLimit (reject oversize bodies before parsing — the JFrog axum-core advisory anchor), then CORS default-deny innermost. The matching code-order in build_router is:

```
.layer(CorsLayer::new())                  // innermost — applied first when entering
.layer(DefaultBodyLimit::max(8 * 1024 * 1024))  // wraps CORS
.layer(middleware::from_fn(host_header_check))  // wraps body-limit
.layer(TraceLayer::new_for_http())        // outermost — wraps everything
```

This is the inverse of how readers naturally scan the code, so worth documenting as a future-self gotcha. The pattern matches `tower::ServiceBuilder` (which chains layers in semantic outer-to-inner order via `.layer()` calls; same trap, different syntax).

Implication for future axum middleware additions: when adding a new layer, check the ordering by tracing one request through: which layer should run first → put it LAST in the `.layer()` chain. Add a brief code comment if ordering matters semantically (e.g., "// security: host check before body parse to short-circuit DNS rebinding").

See: `crates/ingest/src/http.rs::build_router` (chunk #17 layer stack); axum 0.8 docs `Router::layer` semantics; `tower::ServiceBuilder` ordering (same convention).

---

## 2026-05-04 — tonic 0.14 split `prost` integration into separate `tonic-prost` crate

The `tonic = "0.13"` legacy pattern bundled prost message support into the main `tonic` crate via the `prost` feature. `tonic = "0.14"` removed that feature — the available features are `_tls-any, channel, codegen, default, deflate, gzip, router, server, tls-aws-lc, tls-native-roots, tls-ring, tls-webpki-roots, transport, zstd` (no `prost`). Adding `tonic = { version = "0.14", features = ["prost"] }` errors with `package 'ingest' depends on 'tonic' with feature 'prost' but 'tonic' does not have that feature.` The migration: depend on `tonic-prost = "0.14"` separately for the `ProstCodec` runtime + change feature set to `["transport", "router", "server", "codegen"]` (or whatever subset needed). Same story for build dependencies: `tonic-build = "0.14"` is the general gRPC service codegen crate; `tonic-prost-build = "0.14"` is the prost-message codegen crate — both are required when invoking `tonic_prost_build::configure().compile_protos(...)` from `build.rs`. The `tonic-prost-build` crate also pulls in `prost-build` 0.14 transitively, which requires `protoc` on PATH (or a vendored binary via `protoc-bin-vendored = "3"`).

This split is part of the broader tonic 0.14 modularization (see also `tonic-types`, `tonic-reflection`, `tonic-health` as separate crates). Future Rust crates in this project that consume tonic should reference the workspace dep set committed at chunk #16: `tonic.workspace = true` + `tonic-prost.workspace = true` for runtime; `tonic-build.workspace = true` + `tonic-prost-build.workspace = true` + `protoc-bin-vendored.workspace = true` for build-deps.

See: `crates/ingest/Cargo.toml` `[dependencies]` + `[build-dependencies]`; `crates/ingest/build.rs` (codegen invocation + vendored-protoc setup); workspace `Cargo.toml` `[workspace.dependencies]` Ingest pipeline section.

---

## 2026-05-04 — ESLint 9 flat config layered structure for pulse-app/ui

`pulse-app/ui/eslint.config.mjs` (created chunk #13) layers in this order: `ignores` block → `@eslint/js` `js.configs.recommended` → `typescript-eslint` `tseslint.configs.recommended` SPREAD with `...` (it's an ARRAY of configs, not a single object — common footgun) → files-scoped block extending `eslint-plugin-react` `flat.recommended.rules` + `eslint-plugin-react-hooks` (rules-of-hooks: error, exhaustive-deps: warn) + `eslint-plugin-jsx-a11y` `flatConfigs.recommended.rules` → final files-scoped block adding Node globals for `scripts/` + config files. `react/react-in-jsx-scope` is OFF (React 19 + JSX runtime `react-jsx` makes the rule obsolete).

Custom `<Icon glyph="..."/>` components in `pulse-app/ui/src/components/icons/` (chunk #11 deliverable, design-system §Iconography) MUST be scoped out of `jsx-a11y/alt-text` via `{ elements: ['img'], img: ['NextImage'] }` — the rule defaults check Image-named components and false-positive on the project's token-registered Icon registry; without scoping, `npm run lint` errors on every Icon usage. The Icon registry is a design-system convention (icons clarify, not decorate), not raster images.

Companion stack installed at chunk #13: `eslint@^9.x` + `typescript-eslint@^8.x` (metapackage with parser+plugin+configs) + `eslint-plugin-react@^7.37.0` + `eslint-plugin-react-hooks@^5.0.0` + `eslint-plugin-jsx-a11y@^6.10.0` + `globals@^15.0.0`. The chunk title's "7 a11y packages" abbreviation hides this 5-package ESLint companion expansion required because installing `eslint-plugin-jsx-a11y` without ESLint base + recommended-config extension is functionally inert (a11y-plan §11 anti-pattern). Pattern: when chunk titles abbreviate by ecosystem name, expect implicit-peer expansion in the implement scope; surface in plan.md scope-expansion disclosure at Phase 6 user review rather than discovering during /implement.

See: `pulse-app/ui/eslint.config.mjs` (canonical structure); `pulse-app/ui/package.json` devDependencies (companion stack); a11y-plan.md §11 anti-pattern banning lint-only-without-runtime; phase-10/plan.md "Implementation notes" §Scope-expansion disclosure.

---

## 2026-05-04 — Honest provenance principle for Andromeda state schemas

When adding a new field to a shared contract that tracks "which skill performed action X and when", the field's TYPE should match what the writing skill actually produces, not what the schema author imagined. The Iteration 1 spec-amendment-protocol designed `state.yaml.spec_amendments.active[].noted_by_run` and `archived_by_run` as path-strings on the assumption that every lifecycle stage maps to a run-dir. Iteration 2 first-cycle live test exposed the lie: `/andromeda-wrap-session` does NOT create run-dirs (unlike `/andromeda-phase` and `/andromeda-setup-project --delta` which DO). The synthetic path `/andromeda-runs/2026-05-03T23-22-08-wrap-session-11/` was fabricated to fit the schema; no such directory existed on disk. Renamed to `noted_at` and `archived_at` (ISO timestamps) in v2.1; `propagated_by_run` STAYS as path because setup-project --delta creates a real run-dir with materialization-plan-delta.md as audit trail.

Generalizable principle: before locking a schema field type, identify which skill writes it and ask "does that skill actually produce this artifact?" Path = real run-dir audit trail; timestamp = action happened but no separate forensic dir exists. Mismatch = schema dishonesty that papers over with synthetic identifiers — eventually forces ugly migration when the lie surfaces. Applies to drift_warnings (timestamps not paths because wrap-session writes them), curation summaries (counts not paths because curation runs in-place), commit metadata (sha not path because git creates the commit). The honest-provenance test: would the field value resolve to a real disk artifact? If no → use timestamp / count / enum string instead of path.

See: `~/.claude/skills/andromeda-{setup-project,wrap-session,new-session}/references/spec-amendment-protocol.md` Part B Validation §"Field types (NEW v2.1)" for the explicit enumeration.

---

## 2026-05-04 — PYTHONIOENCODING=utf-8 for Python stdout with Unicode on Windows

When running Python one-liners via `python -c '...print("✓ ok")...'` on Windows (Git Bash, cmd.exe, PowerShell), default stdout codec is cp1252 which cannot encode `✓` (U+2713), `✗` (U+2717), `→` (U+2192), `⚠` (U+26A0), `ℹ` (U+2139), or any non-Latin-1 character. The script silently runs the logic but throws `UnicodeEncodeError: 'charmap' codec can't encode character '✓'` at the print statement, masking the actual computation result. Verification scripts that print pass/fail badges with checkmarks die mid-output.

Discipline: prefix verification commands with `PYTHONIOENCODING=utf-8 python -c '...'` (Git Bash) or `$env:PYTHONIOENCODING="utf-8"; python -c '...'` (PowerShell). Alternative: use `sys.stdout.reconfigure(encoding='utf-8')` inside the script (Python 3.7+) but env-var prefix is less invasive for one-liners. The Bash tool inherits the env var per command. The same issue does NOT appear with module-imports or file-output (those default to UTF-8); only stdout to a Windows console.

Discovered while running final verification of Iteration 2 spec-amendment protocol (`yaml.safe_load` + `print` of state.yaml v2.1 fields with `✓`/`✗` badges); first run silently failed at print, masked the YAML-parse-success result behind a UnicodeEncodeError trace. Re-run with `PYTHONIOENCODING=utf-8` rendered cleanly.

See: any verification one-liner emitting Unicode badges (e.g., the 6-contract md5 + state.yaml YAML parse + cyrillic-grep verification triplet from session 12).

---

## 2026-05-03 — Spec-drift workflow formalized as 4-skill cross-cutting protocol

The Variant 3 ad-hoc workflow (manual upstream edit + setup-project rerun pragmatic delta) used in session 10 has been formalized as the **spec-amendment-protocol** spanning all 4 Andromeda skills. When a chunk's harness/test correctly detects a gap between a specialist plan declaration and implementation reality (NOT a code bug, NOT environmental, NOT pure out-of-scope), `/andromeda-implement` Phase 2 fix loop fires Trigger 4 — a soft-exit-to-propose dialogue presenting Path A (amend specialist plan), Path A' (fix implementation to match existing spec), or Path B (defer to handoff Deferred decisions). Path A' MUST be presented prominently to prevent default-amendment bias; sometimes the impl is wrong, not the spec. Path A discipline: orphan-grep verification (`git grep -- "<old-value>"` should return matches only in the Decisions Log entry); coupled-ref updates via Edit `replace_all`; Decisions Log entry with 6 required fields (Trigger / Change / Brand-or-domain impact / Usage scope refinement / Cross-references / Authority statement); marker file at `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md`; state.yaml.spec_amendments.active append. Architecture.md amendment is forbidden as a delta — force re-plan via `/andromeda-arch` (greenfield path). The lifecycle implement (applies) → wrap-session (notes + acks via Phase 6 self-heal + D5 amendment-aware classification) → setup-project --delta (propagates to Tier 2/3 distillations only; bypasses full re-derive) → wrap-session (auto-archives propagated entries) closes the loop. Schema bumped to state.yaml schema_version=2 with `spec_amendments: {active, archive}` field; v1 files migrate automatically on first wrap-session run. The 6th shared contract `spec-amendment-protocol.md` is byte-identical-distributed across the triangle (setup-project / wrap-session / new-session) and cross-referenced from `andromeda-implement/references/spec-drift-protocol.md`.

**Backfill caveat:** Chunk #12's amendment was applied ad-hoc during session 11 BEFORE the protocol existed. The retroactive backfill (`.andromeda/runs/2026-05-03T21-30-00-spec-amendment-lift-accent/amendment.md`) was an exceptional recovery path to make chunk #12 the first use case of the new protocol AND to ensure state.yaml accurately reflects history. **Future amendments authored via `/andromeda-implement` Trigger 4 (Path A) write the marker file + state.yaml entry automatically as part of `spec-drift-protocol.md` §A1-A8 discipline — no backfill needed.** Backfill remains a recognized recovery pattern for amendments applied via tools / processes outside Andromeda's Trigger 4 flow (e.g., direct user edits to specialist plans without invoking `/andromeda-implement`); when needed, replicate the chunk #12 backfill procedure: write the marker file at `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md` per Part A schema, append the entry to `state.yaml.spec_amendments.active` per Part B schema, then proceed through normal lifecycle (wrap-session notes → setup-project --delta propagates → wrap-session archives).

**v2.1 schema refinement (2026-05-04):** Field renames in state.yaml.spec_amendments.active reflect what each skill actually produces — wrap-session does NOT create run-dirs, so `noted_by_run` was renamed to `noted_at` (ISO timestamp); same for `archived_by_run` → `archived_at`. `propagated_by_run` STAYS as a path because setup-project --delta DOES create a real audit-trail run-dir with materialization-plan-delta.md. Honest provenance: each field's type now matches its source skill's actual output. Existing v2 entries migrate via wrap-session Phase 8 best-effort step (extract timestamp from `noted_by_run` path basename if present; else current timestamp).

**v2.1 grep-expansion (2026-05-04):** setup-project --delta no longer trusts marker `expected_propagation` blindly — Detection step 8 runs `LC_ALL=en_US.UTF-8 grep -rn -E '<old-value>' .claude/ CLAUDE.md` against each amendment's primary value(s) extracted from marker `Before → After`. Hits NOT in the marker's `expected_propagation` list are auto-added to delta scope as defense-in-depth. Chunk #12's first-cycle delta exposed this gap: marker listed 1 file, Setup grep found 2 additional files (design-tokens.md + a11y.md). Future amendments authored via Trigger 4 SHOULD grep all Tier 2/3 + CLAUDE.md when populating `expected_propagation`, but the grep-expansion safety net catches authoring oversight.

**v2.1 stale-drift escalation (2026-05-04):** drift_warnings entries gain `first_observed_session_count` + `last_observed_session_count` int fields tracking persistence across wraps. new-session Phase 7 escalates entries with `(current_session_count - first_observed) > 3` to ⚠⚠ rendering with imperative remediation language. Generic D5 carryovers (e.g., test-plan.md from session 10's pragmatic delta) no longer silently re-fire as identical noise; user gets a forced choice after 3 wraps: resolve or accept.

**v2.1 cyrillic check (2026-05-04):** setup-project Phase 8 Check 16 + wrap-session Phase 8 step 6 grep staged files for cyrillic homoglyphs OUTSIDE allowed sections (USER:* / Decisions Log / `## Key Decisions This Session` / code fences). Warning-not-fatal posture; surfaces in commit message body for user review. Built-in complement to the manual sed-based cleanup discipline that emerged ad-hoc in this same session.

**v2.1 SHA-fixup amend (2026-05-04):** wrap-session Phase 10 step 4 captures the new commit SHA post-`git commit` and amends state.yaml.last_completed_chunk.commit_sha from `"pending"` to the real short SHA. One-commit-per-wrap invariant preserved; closes the cosmetic chicken-and-egg lie that surfaced in chunk #12's first-cycle wrap (state.yaml read `commit_sha: pending` for a full session cycle until self-heal next wrap).

---

## 2026-05-03 — Cyrillic-mixing discipline when editing Andromeda skill files

The original Andromeda skill author writes English text with Russian-cyrillic prepositions interleaved (e.g., " к " replacing "to", " с " replacing "with", " в " replacing "in", " не " replacing "not", " без " replacing "without", " против " replacing "against", "Не " at sentence start replacing "Not"). When Claude edits or creates files in `~/.claude/skills/andromeda-*/`, it tends to propagate this style — agents reading the existing files mirror the pattern, leading to ever-more-mixed output. This makes the contracts harder to read for non-Russian speakers and creates orthographic noise. Discipline: post-edit, run `LC_ALL=en_US.UTF-8 grep -E '[а-яА-ЯёЁ]' <files>` to detect remaining cyrillic, then batch-replace via sed with a script handling both word-boundary cases (` к ` → ` to `) and edge cases (`-к-`, ` к$`, ` к.`, `(к `, etc.). The `LC_ALL=en_US.UTF-8` prefix is necessary on Git Bash on Windows where default locale doesn't handle UTF-8 properly (grep counts wrong otherwise). Single-letter Russian prepositions (к, с, в, а, и) are the most common offenders. The shared-contract distribution (`cp` to triangle dirs + `md5sum` verify) must happen AFTER the cleanup, not before, to ensure all 3 byte-identical copies share the cleaned content.

---

## 2026-05-03 — Manual upstream edit + /andromeda-setup-project rerun for minor specialist-plan additions (vs greenfield /andromeda-{specialist} rerun)

`/andromeda-tests`, `/andromeda-security`, etc. are greenfield-only — they regenerate the entire specialist plan from scratch via 7 parallel sub-agents. Using them for а one-line addition (e.g., "Vitest landed at chunk #11" к `test-plan.md`) is overkill: rewrites the plan content, risks losing manual Decisions Log entries, и may diverge from cross-plan binding contracts (obs-plan §3 ↔ tests-plan §3 5-command discipline; a11y-plan §3.5 ↔ tests-plan §9 CI gate; a11y-plan structured violation JSON byte-identical к obs-plan §6 schema).

The pragmatic alternative: **manually edit the specialist plan** + **run `/andromeda-setup-project`** to propagate downstream. Preserves всё manual content и keeps cross-plan bindings intact. The setup-project re-run then:

1. Backs up `CLAUDE.md` к `.claude/backup/CLAUDE.md.pre-setup-{ISO}.md`
2. Regenerates only the `GENERATED:setup:*` sections of CLAUDE.md (`USER:*` preserved; almost always byte-identical если только anti-patterns / pointer table sources changed, which а minor framework addition typically doesn't trigger)
3. Regenerates rule + doc files (preserves `## Session Additions`; updates content above where the changed upstream propagates)
4. Refreshes `state.yaml.plan_freshness.{name}_mtime` к match actual upstream mtime
5. Closes drift D5 (plan-to-CLAUDE.md mtime) + State J (specialist plan freshness mismatch) for the affected upstream

The skill mandates regenerating всё materialized artifacts in Phases 1-6, но for re-runs where most upstream content is unchanged, the regenerated content will be byte-identical к existing files (atomic writes are idempotent on content; git sees no diff). **Pragmatic delta-rerun discipline:** write only the files whose content semantically changed; capture full synthesis intent в `materialization-plan.md` (run dir audit trail) so the rerun is fully auditable even when its file-write delta is minimal.

**Applies к:** minor framework addition (Vitest landing at chunk #11 was the worked example — modified `test-plan.md` §1 surface table + §4 framework section + downstream `tests-summary.md` Test pyramid + `testing.md` Framework + `state.yaml.plan_freshness.tests_mtime`); single Decisions Log append; minor Stack version bump; single anti-pattern revision; decision-rationale clarification.

**Does NOT apply к:** fundamental tier change (Standard → Comprehensive); test framework swap (cargo test → criterion); major architectural decision (Tauri → Electron); auth library swap (no auth → OAuth); logging library swap. Those warrant the greenfield specialist rerun (`/andromeda-tests`, `/andromeda-arch`, etc.) с full sub-agent regeneration so cross-plan bindings re-derive correctly.

See: `.andromeda/runs/2026-05-03T20-26-49-setup-project/materialization-plan.md` (worked example for Vitest propagation, including rejected universal-warning candidates as audit trail); `.claude/rules/testing.md` Framework section (final propagated state); `.andromeda/test-plan.md` §1 surface table + §4 Framework (the upstream edits that triggered the rerun); chunk #11 wrap's drift D3 reading (initial flag + how it closed across two consecutive wrap-session passes).

---

## 2026-05-03 — Vite "asset doesn't exist at build time, will remain unchanged" warning is benign for chained-pipeline outputs

When `index.html` references а static asset by absolute URL (e.g., `<link rel="stylesheet" href="/tokens.css">`) AND the asset is generated by а separate build step that runs BEFORE Vite (in andromeda-pulse: `scripts/build.mjs` orchestrating Tailwind → Vite), Vite's HTML transform during `vite build` emits the warning:

> `/tokens.css doesn't exist at build time, it will remain unchanged to be resolved at runtime`

This is benign и expected: Vite scans the HTML for assets it needs к bundle (modules referenced by `<script type="module">` и `<link rel="modulepreload">`); for everything else (absolute-URL CSS / font / image links), Vite preserves the literal href string in the transformed `dist/index.html` и trusts that the asset will exist at runtime. In the chunk #11 build pipeline, Tailwind has already written `dist/tokens.css` BEFORE Vite reads `index.html` — but Vite's project-root scan (looking at `pulse-app/ui/tokens.css` и `pulse-app/ui/public/tokens.css`) doesn't find it там. The dist-side file IS the intended target; the warning fires because Vite checks the wrong locations.

Triage cost: easy к misinterpret as а build error during CI log inspection. Add к runbook / commit message context when the warning first appears so future maintainers don't chase phantom failures.

NOT applicable to: assets imported by ES modules (`import "./tokens.css"` in main.tsx — Vite would bundle them); assets in `public/` (Vite's publicDir copy pattern; warning doesn't fire because Vite knows к copy them); relative-path links (e.g., `<link href="./tokens.css">` — Vite tries к resolve relative paths through the module graph).

See: `pulse-app/ui/scripts/build.mjs` chunk #11 Vite invocation; `pulse-app/ui/index.html` `<link rel="stylesheet" href="/tokens.css">`; Vite's HTML asset handling docs.

---

## 2026-05-03 — Acceptance-criterion grep patterns over а directory tree match documentation as well as source

Plan acceptance criteria of the form `grep -rE '<animate' src/components/icons/` (intended к enforce "no SVG animation tags in component sources") will match BOTH `.tsx` source files AND `.md` documentation that mentions the banned pattern as а quoted reference (e.g., README explaining the ban). Encountered in chunk #11 implementation: `README.md` documenting "icon components MUST NOT include `<animate>`" caused the criterion grep к return matches even though no actual SVG animation tag was emitted by the components.

Two fixes:
1. **Scope the grep к source files only** — append filename glob filtering: `grep -rE '<animate' --include='*.tsx' --include='*.ts' src/components/icons/`. Cleanest; the criterion's intent is "no animation tags in component output". Use this when authoring future criterion grep patterns over directories that mix source + docs.
2. **Rephrase documentation к avoid the literal substring** — change README from "icon components MUST NOT include `<animate>`" к "icon components MUST NOT include the SVG animation elements `animate`, `animateTransform`, `animateMotion`, or `set`". Same meaning к а human reader; doesn't trigger the literal grep. Use when (a) the criterion is already executed in CI / wrap-session AND (b) documentation lives in the same directory tree as the source it's documenting.

General principle for future plan acceptance criteria authors: when writing `grep -rE PATTERN DIR/` over а directory that may contain README / API docs that quote the pattern itself, either scope the grep к source extensions OR document explicitly that the directory has both source + docs и the pattern must avoid the literal substring in docs. Otherwise the criterion has а silent false-positive surface.

See: `.andromeda/phases/phase-8/plan.md` Test Commands section grep array; `pulse-app/ui/src/components/icons/README.md` ("Motion deferral" section, post-rephrase form).

---

## 2026-05-03 — Node 24 `execFileSync` rejects npm `.cmd` shims on Windows (CVE-2024-27980 hardening)

Node.js since the CVE-2024-27980 batch (Node 18.18.1, 20.5.1, 21.0.0+, all 22.x / 23.x / 24.x) refuses to spawn `.cmd` / `.bat` files via `child_process.spawnSync` / `execFileSync` without `shell: true` — this prevents argument-injection via crafted .cmd path arguments. The error surface is opaque: `EINVAL` with `status: null`, `signal: null`, `stdout: undefined`, `stderr: undefined` — NOT a "file not found" or "permission denied" message that would point at the .cmd file directly. Easy to misdiagnose as a Tailwind / build-tool config error instead of a Node platform behavior.

Symptom in this project: `pulse-app/ui/scripts/build.mjs` initially used `execFileSync(node_modules/.bin/tailwindcss.cmd, [args], { stdio: 'inherit' })` — silent EINVAL crash. The `.cmd` wrapper is just `node ../../@tailwindcss/cli/dist/index.mjs %*` so the workaround is: bypass the wrapper and invoke node directly with the `.mjs` entry path. Snippet from build.mjs:

```js
const tailwindEntry = join(ROOT, "node_modules", "@tailwindcss", "cli", "dist", "index.mjs");
execFileSync(process.execPath, [tailwindEntry, "-i", SRC, "-o", OUT, "--minify"], {
  stdio: "inherit",
  cwd: ROOT,
});
```

Three viable fixes for any future build/test/util script that invokes npm-installed CLI tools from Node on Windows:
1. **Direct .mjs invocation** (used here) — read the `.cmd` wrapper to find the actual entry point under `node_modules/<pkg>/dist/<entry>.mjs`, call `node` on it. Cleanest; no shell semantics; deterministic argument quoting.
2. **`shell: true`** — `execFileSync(cmd, args, { shell: true })` lets cmd.exe interpret the argument list. Works but reintroduces shell-quoting concerns the CVE hardening was meant to prevent.
3. **`process.platform === 'win32'` switch** — branch to `.cmd` on Windows, dotless name on POSIX. Conceptually correct but requires `shell: true` on Windows anyway.

Does NOT apply to: `npm run <script>` from a terminal (that path goes through cmd.exe directly, not Node spawn), Bash invocation of `.cmd` from Git Bash (uses MSYS exec), or POSIX hosts (no `.cmd` wrapper exists; `node_modules/.bin/<name>` is a symlink to the `.mjs`/`.js` entry). Applies specifically to: Node-script-spawning-npm-CLI-tool on Windows.

See: `pulse-app/ui/scripts/build.mjs` (Tailwind v4 invocation pattern); CVE-2024-27980 advisory; `node_modules/.bin/tailwindcss.cmd` (the wrapper that reveals the actual entry path).

---

## 2026-05-03 — Workspace feature unification reactivates Tauri across all test binaries; Windows requires MSVC toolchain for `cargo nextest run --workspace`

The chunk #4 ui-bridge feature-gating fix (`default = ["taurpc-runtime"]` + xtask consumes with `default-features = false`) **only resolves single-package builds** (`cargo run -p xtask`, `cargo build -p xtask`). For workspace-wide test discovery (`cargo nextest run --workspace` — which is what the test-plan §3 5-command harness mandates), Cargo's **feature unification** reactivates `taurpc-runtime` across the entire build:

1. `pulse-app/Cargo.toml` declares `ui-bridge = { path = "../crates/ui-bridge" }` — without `default-features = false`, so pulse-app activates ui-bridge's `taurpc-runtime` feature.
2. Cargo unifies features across all workspace members during a workspace build → ui-bridge is built **once** with `taurpc-runtime` active.
3. That single ui-bridge rlib (linked against tauri / wry / webview2-com / tao) is consumed by every workspace member that depends on ui-bridge — including xtask, despite xtask's `default-features = false` declaration.
4. Result: xtask's **test binary** (built by `cargo nextest run --workspace`) links Tauri DLLs.

On Windows GNU rustup-toolchain hosts, the resulting test binaries fail at startup with `STATUS_ENTRYPOINT_NOT_FOUND` (0xC0000139). The root cause is **NOT a WebView2 DLL search path issue** — it's a GNU vs MSVC ABI mismatch in WinRT API-set linkage. Tauri's wry / tao / webview2-com crates expect MSVC calling conventions for some Windows API-set imports (`api-ms-win-core-winrt-error-l1-1-0.dll`, etc.). MingW GCC linker resolves these symbols, but the resulting binary's import table doesn't match the actual procs available in the system DLLs at runtime.

**Fix — local dev parity with CI**: switch rustup `default-host` to MSVC. Per-user setting in `~/.rustup/settings.toml`, **NOT in the repo** (`rust-toolchain.toml` continues to pin `channel = "1.95.0"` which now resolves to the MSVC variant on this host).

Prerequisites:

1. **Visual Studio 2022 Build Tools** with the **Desktop development with C++** workload (~5-7GB). Download installer: `https://aka.ms/vs/17/release/vs_BuildTools.exe`. Run as admin; check the workload checkbox; install. (`winget install Microsoft.VisualStudio.2022.BuildTools` works on hosts with winget; not all Windows installs ship it.)
2. **WebView2 Runtime** — typically pre-installed on Windows 10/11 via Edge browser. Verify presence via registry `HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\ClientState\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}` (the Evergreen Runtime GUID).
3. `rustup toolchain install stable-x86_64-pc-windows-msvc` (~100MB).
4. `rustup set default-host x86_64-pc-windows-msvc`.
5. `cargo clean` to drop GNU build artifacts (~10GB freed after switch in this project's case).

After the switch, `cargo xtask test` (workspace nextest) succeeds locally — 10 binaries / 0 tests in Foundation epoch state. CI matrix runners (`windows-latest` = MSVC + WebView2 Runtime preinstalled, `macos-latest`, `ubuntu-22.04`) already have this configuration; the toolchain switch is purely about local dev parity.

Adjacent learnings (still valid for their original scopes — these don't replace, they complement):

- chunk #4 entry "Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers" — feature-gating fixes `cargo run -p xtask` (single-package, no workspace unification). Does NOT fix workspace test discovery; that requires the MSVC switch above.
- "Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov" — the same MSVC switch resolves both blocks (profiler_builtins available in MSVC std + workspace-wide nextest succeeds).

See: `.andromeda/phases/phase-4/plan.md` Implementation notes (re: NEXTEST_EXPERIMENTAL_LIBTEST_JSON env requirement); `xtask/src/main.rs` `run_cargo_nextest()`; chunk #4 wrap entries on ui-bridge feature-gating + cargo-llvm-cov profiler_builtins.

---

## 2026-05-03 — Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers

Any binary that transitively depends on the `tauri` crate links against WebView2 / DirectX / etc. Windows DLLs at link time. On Windows GNU rustup-toolchain hosts without WebView2 installed (or any DLL load-path issue), the resulting binary fails at startup with `STATUS_ENTRYPOINT_NOT_FOUND` (exit code `0xc0000139`) — **even if the binary never actually invokes any Tauri runtime code**. This blocks shared-crate designs where the data types live alongside the procedure implementation: an `xtask` binary that imports `ui-bridge` for `HealthEnvelope` (a pure data type) inherits the tauri DLL deps and crashes.

**Solution**: split runtime vs. types via Cargo features.

```toml
# crates/ui-bridge/Cargo.toml
[features]
default = ["taurpc-runtime"]
taurpc-runtime = ["dep:taurpc", "dep:tauri", "dep:specta", "dep:tokio"]

[dependencies]
thiserror.workspace = true
serde.workspace = true
chrono.workspace = true
taurpc = { workspace = true, optional = true }
tauri = { workspace = true, optional = true }
specta = { workspace = true, optional = true }
tokio = { workspace = true, optional = true }
```

Source code uses `#[cfg(feature = "taurpc-runtime")]` to gate the `#[taurpc::procedures]` trait and resolver impl, leaving the data types (`HealthEnvelope`, `AppError`, `SubsystemStatus`, etc.) compiled unconditionally.

```toml
# xtask/Cargo.toml — non-Tauri binary consumes types only
[dependencies]
ui-bridge = { path = "../crates/ui-bridge", default-features = false }
```

The pulse-app binary keeps default features (taurpc-runtime enabled) so the procedure trait + resolver are available for `taurpc::create_ipc_handler(...)` registration in the Tauri Builder chain.

The `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` pattern lets data types acquire the `specta::Type` derive only when the runtime feature is active — required for taurpc procedure parameter/return types but useless for type-only consumers.

This is also the cleanest architectural split — types belong in the contract module, runtime belongs in the runtime module.

See: `.andromeda/phases/phase-3/plan.md` Implementation note 1; `crates/ui-bridge/Cargo.toml`; `crates/ui-bridge/src/health.rs` `#[cfg(feature = "taurpc-runtime")] mod runtime`.

---

## 2026-05-03 — Cargo alias for `cargo xtask <subcommand>` shortcut

Without an alias, `cargo xtask harness:status` fails with "no such command: xtask" because cargo doesn't know `xtask` is a workspace member shortcut. The fix is `.cargo/config.toml` (project-root):

```toml
[alias]
xtask = "run --quiet --package xtask --"
```

After this, `cargo xtask <subcommand>` works equivalently to `cargo run --package xtask -- <subcommand>` from any directory inside the project. The `--quiet` flag suppresses Cargo's "Compiling … / Finished …" output so the subcommand's stdout (e.g. JSON envelope from `harness:status`) is the only thing on the pipe — important for `jq` / shell-script chains.

The agent-run scripts (`scripts/agent-run.{sh,ps1}`) invoke `cargo xtask harness:status` and depend on this alias being present.

See: `.cargo/config.toml`; `xtask/src/main.rs` clap dispatcher; `scripts/agent-run.sh` `status` case body.

---

## 2026-05-03 — Andromeda chunk scope-split for paid-prereq operator steps

When a route chunk's full scope requires paid external accounts (e.g., chunk #3 code-signing wants Azure Key Vault Premium ~$5/month + Windows EV cert from DigiCert/GlobalSign $300-500/year + Apple Developer ID $99/year + 1-2 weeks of legal-entity verification) but the project is in dogfooding/iteration phase, **split chunk scope** rather than skip the chunk or pay prematurely.

The pattern: `plan.md` divides Implementation Steps + Acceptance Criteria into **ACTIVE** (free + local + reversible work that `/andromeda-implement` runs now — e.g., generate Minisign keypair locally, add deps, edit `tauri.conf.json`, write rotation runbook) and **DEFERRED** (paid + external + bureaucracy items that become pre-v0.1.0 release blockers — Azure Key Vault provisioning, EV cert enrollment, Apple Developer ID, GitHub Environment with secrets). DEFERRED items are tracked in `plan.md §Acceptance Criteria → Deferred` + the runbook describing operator procedure + a `route.md` Decisions Log entry recording the scope-split rationale.

Pipeline integrity is preserved: `state.yaml.last_completed_chunk.route_index` advances when ACTIVE scope lands; DEFERRED items are explicit pre-release blockers tracked across artifacts (not lost). This is better than (a) skipping the chunk entirely (breaks route progression heuristics + state.yaml continuity) or (b) running paid prereqs before the project demonstrates value (premature commitment).

Apply when: chunk has clear paid-vs-free dependency split AND project is in pre-public-release dogfooding phase AND user explicitly states preference to defer paid commitments. Don't apply when: chunk's value depends entirely on paid prereqs (rare for solo OSS projects).

See: `.andromeda/route.md` Decisions Log 2026-05-03 entry "Chunk #3 scope split"; `.andromeda/phases/phase-2/plan.md` §Acceptance Criteria → Active vs Deferred; `docs/runbooks/updater-key-rotation.md` as DEFERRED procedure document.

---

## 2026-05-03 — Standalone minisign 0.12 as Tauri-cli fallback when Windows GNU mingw blocks compile

The rustup `x86_64-pc-windows-gnu` toolchain bundles a minimal mingw-w64 set in `<sysroot>\lib\rustlib\x86_64-pc-windows-gnu\lib\self-contained\` that does NOT include `libktmw32.a` (Windows Kernel Transaction Manager API import library). Modern Tauri 2.x ecosystem crates link transitively against `ktmw32` so `cargo install tauri-cli --version "^2.0" --locked` fails with `ld: cannot find -lktmw32`.

**Refreshing rust-mingw component does NOT fix it** — `rustup component remove rust-mingw && rustup component add rust-mingw` re-downloads the same minimal libset; `libktmw32.a` is not bundled by design.

Two viable paths:
- **MSVC switch (permanent fix)**: install Visual Studio 2022 Build Tools (~5 GB) + `rustup toolchain install stable-x86_64-pc-windows-msvc` + update `rust-toolchain.toml` channel to MSVC variant. Also resolves the `profiler_builtins` issue for `cargo llvm-cov` (separate Tier-3 entry from previous session). ~30 minutes including download.
- **Standalone minisign 0.12** (jedisct1, Frank Denis): download `minisign-0.12-win64.zip` from `https://github.com/jedisct1/minisign/releases` (~500 KB), unpack `x86_64/minisign.exe` into `~/.cargo/bin/` (already in PATH), use `minisign -G -W -f -s ~/.tauri/{name}.key -p ~/.tauri/{name}.key.pub` for no-password keypair (the `-W` flag = "do not encrypt secret key with a password" — acceptable for local dogfooding scope where private key stays in `~/.tauri/` gitignored; production HSM custody re-generates with password before public release).

Both `tauri signer generate` and standalone `minisign -G` produce **interoperable Minisign Ed25519 keypairs** — the tools both follow the public Minisign spec (`https://jedisct1.github.io/minisign/`). The verbatim base64 line from the `.pub` file (line 2, after `untrusted comment:` header) goes into `tauri.conf.json plugins.updater.pubkey` regardless of which tool generated it; `tauri-plugin-updater 2.x` accepts and verifies signatures from either.

Implication: when blocked on `cargo install tauri-cli` due to Windows GNU mingw limitations, the standalone-minisign fallback unblocks keypair generation without committing to the heavyweight MSVC switch. Document in the chunk's runbook that production-ready key custody re-generates the keypair WITH a password and uploads private + password to the production secrets manager (Azure Key Vault Premium SKU per security plan §Code-signing key custody).

See: `.andromeda/security-plan.md` §Code-signing key custody; `docs/runbooks/updater-key-rotation.md` Phase 1 (operator-side keypair generation); `pulse-app/tauri.conf.json` `plugins.updater.pubkey` field; previous Tier-3 entry "Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov" (related rustup-toolchain limitation pattern).

---

## 2026-05-03 — Tauri 2.x transitively requires rustc ≥ 1.88

The architecture's security plan pins minimum rustc to 1.85 for Edition 2024 security-positive defaults (`unsafe_op_in_unsafe_fn`, tightened `if let` temporary scopes, `static mut` reference denial). Tauri 2.11.0's transitive dependency tree (`darling 0.23` requires 1.88, `plist 1.9` requires 1.88, `serde_with 3.19` requires 1.88, `time 0.3.47` requires 1.88, `icu_* 2.2` requires 1.86, `icu_normalizer_data 2.2` requires 1.86) pushes the effective floor to rustc 1.88+ for any project that compiles Tauri 2.

Phase 1 implementation chose `channel = "1.95.0"` in `rust-toolchain.toml` to match the host installation while satisfying the security plan's `1.85+` minimum (the AC's grep regex `^channel = "1\.(8[5-9]|9[0-9])'` matches 1.95). Future Tauri version bumps may push the floor higher — bumping `rust-toolchain.toml` is not a security-plan violation as long as the channel stays ≥1.85.

Implication for future Tauri-related chunks: when adding/upgrading Tauri 2.x or its plugins, check if transitive deps require a rustc bump. Coordinate the bump with the security-plan minimum (≥1.85) and the CI matrix runners.

See: `.andromeda/security-plan.md` §Anti-Patterns Universal + §Decisions Log open question on 1.84 → 1.85 bump; `rust-toolchain.toml`.

---

## 2026-05-03 — Tauri 2 capability JSON `identifier` field uses simple kebab-case names

Architecture §Occupied Resources references Tauri capability identifiers as `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs` — these are **conceptual fully-qualified namespaced** names. The actual Tauri 2 capability JSON `identifier` field uses **simple kebab-case local** names (`default`, `tray`, `notification`, `updater`, `plugin-fs`); Tauri 2 does not accept colons in identifiers, and the bundle id `com.andromeda.pulse` provides implicit namespacing at runtime.

Filenames in `pulse-app/capabilities/` map 1:1 to the local identifiers (`default.json` → identifier `default`). The `pulse:` prefix is preserved in the architecture and security plans as the conceptual reference (e.g., when discussing "do not expose `pulse:updater` to webview JavaScript"), but the JSON file's `identifier` field uses just `updater`.

Implication: any future capability JSON edit (new TauRPC procedure → matching capability entry per security plan §API Security) should use the simple form. The `xtask capability-drift` check (route#22) will diff TauRPC routers against the file inventory by filename / local identifier — not against the namespaced form.

See: `.andromeda/architecture.md` §Occupied Resources Tauri capability identifiers; `.andromeda/security-plan.md` §API Security TauRPC capability authorization; `pulse-app/capabilities/*.json`.

---

## 2026-05-03 — Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov

`cargo-llvm-cov` requires the `profiler_builtins` crate (provided by the Rust standard library precompiled with profiler runtime support). The Rust standard library precompiled binaries for `x86_64-pc-windows-gnu` do NOT include `profiler_builtins`, even with the `llvm-tools-preview` rustup component installed. Running `cargo llvm-cov nextest --workspace ...` fails with `error[E0463]: can't find crate for 'profiler_builtins'` during build-script compilation of common deps (e.g., `serde`, `typeid`, `zmij`).

Phase 1 acceptance criterion T8 (`cargo llvm-cov nextest --workspace --lcov --output-path lcov.info --summary-only`) fails on the local Windows GNU host for this reason. The other 11 acceptance test commands pass. Workarounds: (a) install MSVC toolchain — `rustup toolchain install stable-x86_64-pc-windows-msvc` (requires Visual Studio 2022 Build Tools install) and update `rust-toolchain.toml` channel to `1.95.0-x86_64-pc-windows-msvc`; (b) defer coverage to CI Linux/macOS runners where profiler runtime is bundled — route#5 base CI workflow will primarily exercise the coverage gate on those targets; (c) switch to `cargo-tarpaulin` as alternative (different coverage tool, not in plan AC).

Implication: the route#5 CI matrix workflow should run the coverage gate primarily on Linux + macOS. A Windows MSVC runner can also pass; a Windows GNU runner cannot without rebuilding std with profiler support (nightly-only via `-Z build-std`).

See: `.andromeda/test-plan.md` §3 Bootstrap phase 7 "coverage-tooling-install" + §10 Coverage thresholds; route#5 Base CI workflow chunk.

---

## Entry format

Each entry follows this structure:

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}

See: `.claude/docs/services/{service}.md` (or similar cross-reference)
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.

Other tiers:
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` section (critical, short)
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` sections (loaded when matching files touched)
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read)

See the classification gallery in the refactor plan `§3.5` for which tier a given learning belongs to. wrap-session applies this classification automatically during curation.

## Promotion

When this file grows beyond ~200 lines, `/wrap-session` suggests promoting some entries to topic-specific files (e.g., `.claude/docs/services/{service}.md` if the learning is about a specific service). Promotion is a user action, not automatic — wrap-session never moves entries without approval.

## Demotion from CLAUDE.md

If `CLAUDE.md` `USER:session-learnings` section gets too large (≥ 180 lines total CLAUDE.md), wrap-session suggests promoting old Tier 1 entries down to this file (Tier 3) to keep CLAUDE.md within size budget. This is also a user action.
