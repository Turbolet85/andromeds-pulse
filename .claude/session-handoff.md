# Session Handoff

**Last Updated:** 2026-05-06T22:49:18Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #23 Broadcast fan-out + Tauri Channel API shipped this session)

## Current State

- **Last completed chunk:** route#23 "Broadcast fan-out + Tauri Channel API — tokio broadcast → pulse://stream/{spans,metrics,logs} binary Arrow IPC + _trace_context metadata + size cap" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#24 "Tauri webview shell — WebView2/WKWebView, frameless + custom titlebar with drag region, close→minimize-to-tray policy" (Epoch 4 — Webview shell + TauRPC bridge opens)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-20}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Epoch 3 — Storage & query: COMPLETE.** All 4 chunks (#20 + #21 + #22 + #23) committed. Epoch 4 opens with chunk #24.

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #24 listed in route §2 but no `.andromeda/phases/phase-21/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear.

## Drift Detection (6 dimensions)

⚠️ D3 — Plan-to-code drift: chunk #23 introduced `streams.{subscribe_spans, subscribe_metrics, subscribe_logs}` TauRPC procedures via `pulse-app/src/streams.rs`, but arch §Occupied Resources Tauri IPC routes list does NOT include `streams.*` namespace. The procedures were necessary for Tauri Channel<Vec<u8>> handle exchange (webview registers a Channel by invoking the procedure; backend forwards broadcast bytes via Channel::send). Remediation options for next session: (a) re-run `/andromeda-arch` to incorporate the new namespace into §Occupied Resources, (b) capture as a Decisions Log entry against route.md noting the namespace addition without re-deriving arch, (c) consider whether to relocate to existing `traces.subscribe` / `metrics.subscribe` / `logs.subscribe` namespaces (would require refactoring the trait+impl in `pulse-app/src/streams.rs` + maintenance of the ai11y/security ↔ arch downstream-flagged contract bindings from phase-20 combined.md). Severity: warning. (D3 first_observed_session_count: 23.)

D1, D2, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because no specialist plans were edited this session — plan_freshness mtimes unchanged from baseline. D6 advances cleanly: state.yaml.last_completed_chunk progresses 22 → 23 with this wrap commit.)

## Spec Amendments (this session)

(none this session — chunk #23 implementation matched specialist plan expectations; no Trigger 4 dialogue. Pre-existing archive untouched: lift-accent (2026-05-03) + clarify-pii-grep-ui-vocab (2026-05-04) both already archived per state.yaml.spec_amendments.archive.)

## Key Decisions This Session

- **Refactor extracted `build_*_record_batch` from `append_*_batch` in `crates/buffer/src/appender.rs`** to allow consumer.rs to reuse the same Arrow `RecordBatch` for both DuckDB persistence AND broadcast emission (encode for broadcast → append to DuckDB → emit broadcast). Old `append_*_batch` wrappers stay as thin compose layers around `build_*_record_batch` + `append_record_batch_to_table` but became `#[cfg(test)]`-gated since production now goes through the helpers directly. Plan §Implementation notes anticipated this: "encode BEFORE append (so encoder sees the same batch DuckDB will store), but only emit to broadcast AFTER append succeeds (so subscribers see only durably-stored data)."
- **`bytes::Bytes` chosen for broadcast Sender payload type, `Vec<u8>` at Tauri Channel boundary.** Bytes is cheap-clonable across N broadcast subscribers (Arc bump per receiver, not deep clone). Tauri 2 `tauri::ipc::Channel<T>` requires `T: Serialize + Send + Sync + 'static`; bytes::Bytes does NOT implement Serialize, so streams.rs converts via `bytes.to_vec()` once at the Channel send site. Trade-off accepted: one Vec allocation per emission per subscriber (typical 3 subscribers × ~10k events/sec ≈ 30k allocs/sec; well within tokio scheduling overhead headroom).
- **Bytes already at workspace level (chunk #17 ingest http body limit).** Plan §Implementation Steps 1 said "add `bytes = "1"` to `[workspace.dependencies]` if not present" — verified during research and confirmed at impl time it was already pinned. Per-crate `bytes.workspace = true` added to buffer/Cargo.toml + pulse-app/Cargo.toml only.
- **api-surface reconcile: viz crate requires `--features specta/derive` for per-crate `cargo +nightly public-api` invocation.** The workspace specta dep is `features = ["chrono"]` only; `specta::Type` derive macro is gated behind `derive` feature. Per-crate `cargo public-api` doesn't pick up the workspace-wide unification that `cargo nextest --workspace --all-features` provides. Updated METADATA Tooling field with this caveat. Same root cause as chunk #22 wrap reconcile (per session-handoff #22 "viz section grew from 4 pub items to ~30") — likely the same `--features specta/derive` workaround was used, but wasn't documented in METADATA. This wrap fixes the documentation.
- **D3 plan-to-code drift on `streams.*` namespace surfaced (not auto-resolved).** The TauRPC procedures `streams.subscribe_spans/metrics/logs` are necessary for the Tauri Channel API binary path (per arch §Standard Contracts Real-time push contract); arch's §Occupied Resources Tauri IPC routes list anticipated event names (`pulse://stream/*`) but not the procedure namespace required for Channel handle exchange. Surfaced via D3 drift warning rather than silently auto-amending arch (per spec-drift-protocol §Architecture.md amendment exception — arch amendments are forbidden as deltas).

## Files Modified

(15 files this session — chunk #23 implementation + 3 NEW phase-20 artifacts + 2 reconciled living artifacts.)

**Code files (chunk #23 — Rust):**
- `crates/buffer/Cargo.toml` — added `bytes.workspace = true`.
- `crates/buffer/src/broadcast.rs` (NEW) — public `BroadcastSenders` struct (3 `tokio::sync::broadcast::Sender<bytes::Bytes>` for spans/metrics/logs) + `create()` factory at capacity 128 + `MAX_PAYLOAD_BYTES = 8 * 1024 * 1024` + `STREAM_NAME_{SPANS,METRICS,LOGS}` literal constants + `encode_{spans,metrics,logs}(batch: &RecordBatch) -> Result<Bytes, Error>` Arrow IPC StreamWriter encoders with size-cap check; 13 unit tests (constants match arch literals, encode round-trip via StreamReader, size-cap rejection, send-with-zero-subscribers SendError, three-subscriber fan-out, lag detection on capacity overrun).
- `crates/buffer/src/contract.rs` — added 2 Error variants `BroadcastEncode { reason: String }` + `BroadcastSizeCapExceeded { payload_bytes: usize }`.
- `crates/buffer/src/lib.rs` — `pub mod broadcast;` + 9 new top-level re-exports.
- `crates/buffer/src/appender.rs` — extracted `build_{spans,metrics,logs}_record_batch(batch) -> Result<Option<RecordBatch>, Error>` builders + `append_record_batch_to_table(conn, table, batch) -> Result<u64, Error>` writer; old `append_{spans,metrics,logs}_batch` wrappers gated `#[cfg(test)]` (production path goes through builders + writer directly via consumer.rs); +3 new tests for build helpers; +1 `#[cfg(test)] use std::time::Instant;` import gate.
- `crates/buffer/src/consumer.rs` — `run_consumer` signature gains `broadcast_senders: Arc<BroadcastSenders>` 4th arg; `dispatch_batch` now: build RecordBatch → encode for broadcast → append to DuckDB → emit broadcast bytes if encode-Ok and append-Ok; `encode_or_log` helper logs `tauri.channel.emit.size_exceeded` / `tauri.channel.emit.error` events on encode failure (best-effort emit); +3 new integration tests (broadcast encoded Arrow to subscriber, three subscribers receive same payload, no-subscribers still appends to DuckDB); `describe_error` extended with 2 new arms.
- `crates/buffer/src/retention.rs` — `describe_error` extended with `BroadcastEncode` + `BroadcastSizeCapExceeded` arms (forced by non-exhaustive match after Error variant additions).
- `crates/ui-bridge/src/contract.rs` — `From<BufferError> for AppError` extended with 2 new arms (BroadcastEncode → "buffer broadcast encode failed", BroadcastSizeCapExceeded → "buffer broadcast payload exceeded size cap"); +2 sanitization tests asserting no Rust struct name / numeric byte count / library name leakage.
- `pulse-app/Cargo.toml` — added `bytes.workspace = true`.
- `pulse-app/src/streams.rs` (NEW) — single TauRPC trait `StreamsApi` (path = "streams") with 3 methods `subscribe_{spans,metrics,logs}(channel: tauri::ipc::Channel<Vec<u8>>) -> Result<(), AppError>`; `StreamsApiImpl::new(senders: Arc<BroadcastSenders>)`; resolver clones broadcast::Sender, calls `.subscribe()` → spawns `forward_loop(stream_name, receiver, channel)` task that loops: recv → size cap re-check → `bytes.to_vec()` → `channel.send(payload)` → log `tauri.channel.emit` info event; on `RecvError::Lagged(n)` log `tauri.channel.lag` warn; on `RecvError::Closed` or Channel send err exit task. +2 unit tests (constructor smoke + clone shares senders).
- `pulse-app/src/main.rs` — `let broadcast_senders = Arc::new(buffer::broadcast::create());` after viz_state init; threaded `Arc::clone(&broadcast_senders)` into `run_consumer` (4th arg), `heartbeat::spawn(...)` (final arg), and `StreamsApiImpl::new(...)` × merged into both Some-arm + None-arm `taurpc::Router`s.
- `pulse-app/src/heartbeat.rs` — `spawn(...)` signature gains `broadcast_senders: Arc<BroadcastSenders>` 7th arg; `emit_ingest_tick(...)` adds 4th arg + populates `IngestState.set_broadcast_subscribers(total_subs as u32)` from sum of receiver_counts before emitting `ingest.tick`; new `emit_broadcast_gauge(channel_name, value)` helper emits per-stream `metric.ingest.channel.broadcast_subscribers` gauge events with enumerated `channel_name`; +2 new tests (subscriber sum across streams, per-stream gauge emission shape).
- `pulse-app/src/observability.rs` — extended `AllowList::production()` with 5 new targets (`tauri.channel.emit` / `tauri.channel.emit.size_exceeded` / `tauri.channel.emit.error` / `tauri.channel.lag` / `metric.ingest.channel.broadcast_subscribers`); +10 co-located pass+redact tests (5 pass + 5 redact mirroring chunks #20/#21/#22 precedent) covering all 5 targets.

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-20/{combined.md, research.md, plan.md}` (NEW) — Phase 20 planning artifacts for chunk #23 (228 + 84 + 190 lines).

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent`. Diff: bytes added as direct dep to buffer + pulse-app (one new line each).
- `.andromeda/context/api-surface.md` — buffer section + ui-bridge `From<BufferError>` arm extended for chunk #23 contributions; viz/ingest/snapshot/workspace-detector/plugins/mcp-server sections preserved from chunk #22 reconcile (no chunk #23 surface changes); METADATA Tooling field updated to document `--features specta/derive` requirement for per-crate viz invocation.
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advances to 23 + epoch 3 closes; commit_sha advances from `"26f0859"` (stale chunk #22 placeholder per known wrap-amend pattern) → real SHA via Phase 10 amend; session_count=23; spec_amendments.{active,archive} unchanged from chunk #22 baseline; drift_warnings populated with single D3 entry on `streams.*` namespace; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-06T22:49:18Z.
- `.claude/docs/session-learnings.md` — prepended 3 Tier 3 entries (RecordBatch reuse refactor pattern; cfg(test) gating of test-only API wrappers; TauRPC + broadcast + Channel forwarding pattern).
- `.claude/session-handoff.md` — this file.

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 0 additions
- **Tier 3 (`.claude/docs/session-learnings.md`):** 3 additions
  - (a) RecordBatch reuse refactor: extract `build_*_record_batch` from `append_*_batch` so producer can fan out same Arrow data to multiple sinks (DuckDB persist + broadcast emit) without doubled construction cost
  - (b) `#[cfg(test)]` gating of test-only API wrappers after refactor — old wrappers becoming dead-code after extraction trigger `-D warnings` clippy gate; gate them + any newly-test-only imports (`std::time::Instant` precedent)
  - (c) TauRPC `Channel<Vec<u8>>` + tokio broadcast + Tauri Channel API forwarding pattern: TauRPC procedure receives Channel handle, clones broadcast Sender, subscribes to get Receiver, spawns forwarding task; bytes::Bytes does NOT implement Serialize so convert via `to_vec()` at Channel send site
- **Filtered:** 1 dedup-deferred (Arrow IPC StreamWriter borrow-scope pattern — close to a generic Rust borrow-scope idiom rather than a project-specific convention; left out below 0.6 confidence) + 0 task-specific + 0 conflicts + 0 deferred-due-to-cap (3 candidates passed; under max-3 cap)

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 324/324 workspace --all-features, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok, cargo audit ok with 18 pre-existing allowed warnings, cargo tree -p viz | grep opentelemetry empty (obs criterion #7 strict-grep PRESERVED), cargo tree -p buffer | grep opentelemetry empty, grep -rnE "format!.*(CREATE|SELECT|INSERT|UPDATE|DELETE|WHERE|FROM|JOIN)" crates/buffer/src/ empty, grep -rnE "(payload|service_name|span_name|attribute).*tracing::(info|debug|warn|error|trace)" crates/buffer/src/ pulse-app/src/streams.rs empty, grep -rn 'pulse://stream/' returns exactly the 3 canonical names plus their definition + test sites.)

## Tests Status

passing — 324 cargo nextest workspace --all-features (was 288 last wrap, +36 from chunk #23: 13 broadcast.rs unit tests, 4 consumer.rs integration tests for broadcast emit, 2 streams.rs constructor tests, 4 heartbeat.rs tests for broadcast_subscribers + per-stream gauges, 10 observability.rs AllowList tests, 2 ui-bridge contract.rs sanitization tests, 1 describe_error test for new variants) + cargo deny ok (skip list unchanged from chunk #22) + cargo audit ok + cargo clippy clean + cargo fmt clean. Total: 324 tests + 4 lint/typecheck gates + 2 supply-chain gates = 330 checks. cargo nextest --workspace --all-features ~1.19s parallel.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #24:**

`/andromeda-phase` to plan chunk #24 "Tauri webview shell — WebView2/WKWebView, frameless + custom titlebar with drag region, close→minimize-to-tray policy". Opens Epoch 4 (Webview shell + TauRPC bridge). Substrate for the webview surface that consumes chunk #23's broadcast streams.

**Priority 2 (background, optional) — D3 drift remediation decision:**

Before chunk #24 phase planning, consider how to remediate the D3 drift on `streams.*` namespace. Three paths:
(a) Defer indefinitely — chunk #27 (`xtask capability-drift` check) will surface the inconsistency more loudly when it tries to introspect TauRPC procedures and finds `streams.*` not in arch. That chunk's plan can carry the arch update as a deliverable. **Lowest effort.**
(b) Re-run `/andromeda-arch` to incorporate `streams.*` into §Occupied Resources Tauri IPC routes. Heavy — the `/andromeda-arch` skill regenerates the entire arch.md, and downstream specialist plans + route + CLAUDE.md ecosystem may need refresh too. Probably overkill for one missing namespace.
(c) Add a `/andromeda-route` Decisions Log entry capturing the namespace addition without re-deriving arch. Lightest formal capture; matches the precedent at `2026-05-03` "Chunk #3 scope split" entry in route.md §3 Route Decisions Log.

**Recommendation:** option (a) — let chunk #27 carry the arch update naturally. The D3 warning persists in state.yaml until then; new-session Phase 7 will surface it under "stale drift" treatment after 3 wraps if it lingers (per session-state-contract.md v2.1 first_observed_session_count escalation).

## Session Goals (carry-over)

(none — chunk #23 fully implemented + tests green + curation applied (3 Tier 3) + reconcile complete + Epoch 3 closes; chunk #24 opens Epoch 4; ready for `/andromeda-phase`)

## Session End Status

clean
