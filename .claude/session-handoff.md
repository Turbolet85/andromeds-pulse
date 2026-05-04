# Session Handoff

**Last Updated:** 2026-05-04T23:55:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #16 OTLP gRPC receiver — tonic 0.14 :4317 bound 127.0.0.1, TraceService/MetricsService/LogsService, .max_decoding_message_size 8MB shipped this session via Trigger 4 → Path A' vendored-proto resolution)

## Current State

- **Last completed chunk:** route#16 "OTLP gRPC receiver — tonic 0.14 :4317 bound 127.0.0.1, TraceService/MetricsService/LogsService, .max_decoding_message_size 8MB" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#17 "OTLP HTTP receiver — axum 0.8 :4318 bound 127.0.0.1, POST /v1/{traces,metrics,logs}, DefaultBodyLimit + Host-header allowlist + CORS deny" (Epoch 2 — Ingest pipeline continues)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-13}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Foundation epoch CLOSED:** chunks #1-#15 all implemented; **Epoch 2 — Ingest pipeline OPENED** by chunk #16 this session.

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #17 listed in route §2 but no `.andromeda/phases/phase-14/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear.

## Drift Detection (6 dimensions)

D1, D2, D3, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because no specialist plans were touched this session — Trigger 4 → Path A' implementation resolution did NOT amend obs-plan §12 (per spec-drift-protocol.md Path A' discipline: NO marker file / NO state.yaml entry / NO Decisions Log).)

## Spec Amendments (this session)

(none this session — Trigger 4 was resolved via Path A' impl-adjusted-per-spec; no specialist plans amended.)

## Key Decisions This Session

- **Trigger 4 → Path A' (vendored .proto + tonic-build) over Path A (amend obs-plan §12)**: detected during Phase 1 dep-add when `opentelemetry-proto 0.31` (planned upstream) pulled `opentelemetry` + `opentelemetry_sdk` as direct (non-feature-gated) dependencies, contradicting obs-plan §12 user-pivot ("NO OTel SDK in self-observation runtime") + CLAUDE.md universal invariant + obs criterion #7 strict-grep forbid. Plus `prost 0.13 ↔ 0.14` duplicate fired `cargo deny check bans` canary. User chose Path A' because (a) foundational [arch §Self-Observation + obs §12 user-pivot] intentionally chose "no OTel SDK" as defense-in-depth rather than "no SDK initialization"; (b) Foundation epoch is the right time to invest in strict dep hygiene; (c) vendored .proto matches arch §Cross-cutting Patterns "minimal dep surface" + agent-driven CI ethos. Result: 8 OTLP `.proto` files vendored into `crates/ingest/proto/` (Apache 2.0; ~700 lines); `crates/ingest/build.rs` invokes `tonic-prost-build::configure().compile_protos(...)` with `protoc-bin-vendored = "3"` for zero-system-dep cross-platform builds. `cargo tree -p ingest` now contains ONLY `tonic`/`tonic-prost`/`prost` (zero opentelemetry crates).
- **`tonic-prost` separate crate, no `prost` feature on `tonic` 0.14**: tonic 0.14 dropped the inline `prost` feature; the available features are `_tls-any, channel, codegen, default, deflate, gzip, router, server, tls-aws-lc, tls-native-roots, tls-ring, tls-webpki-roots, transport, zstd`. The prost message support moved to the dedicated `tonic-prost` runtime crate + `tonic-prost-build` build crate. Both are added as workspace deps + opted into by ingest crate.
- **`HeartbeatState` extended with `otlp_grpc_bind: Mutex<Option<BindStatus>>` slot** (vs introducing a separate ReceiverStatus holder): preferred for symmetry with existing `last_ingest`/`last_buffer`/etc slots; `current_health()` populates `subsystems.otlp_grpc_receiver.{status,error_msg}` from the slot. New `BindStatus { Ok | Failed(String) }` enum exported from `ui_bridge::health`.
- **Bind/serve split via `try_bind() -> TcpListener` + `serve_on(listener) -> Future`** (vs single `serve(addr)` that's all-in-one): allows `pulse-app/src/main.rs` to record `BindStatus::Ok` immediately after listener accepts (before the long-running serve future), so `xtask harness:status` polling sees `subsystems.otlp_grpc_receiver.status == "ok"` within boot timeout. Implementation uses `tokio_stream::wrappers::TcpListenerStream` to bridge from `TcpListener` to tonic's `serve_with_incoming`.
- **`foldhash` added to deny.toml skip-list** (with provenance): tonic-prost-build 0.14 build-dep stack pulls foldhash 0.1 via hashbrown / petgraph; tauri-utils / dom_query pull foldhash 0.2. Both transitive, both build-time only, benign. The `tonic <0.14` deny canary remains intact.

## Files Modified

(15 files this session — chunk #16 implementation + reconcile, includes Cargo.lock regen)

**Code files (chunk #16 — Rust + .proto):**
- `Cargo.toml` (workspace) — added `tonic`, `tonic-prost`, `tonic-build`, `tonic-prost-build`, `protoc-bin-vendored`, `prost`, `tokio-stream`, `tempfile`, `rstest`, `proptest` to `[workspace.dependencies]` Ingest pipeline section
- `Cargo.lock` — regen reflecting new resolved versions (tonic 0.14.5, tonic-prost 0.14.5, prost 0.14.3, tokio-stream 0.1.18, etc.)
- `crates/ingest/Cargo.toml` — opted into workspace deps + `[build-dependencies]: tonic-build, tonic-prost-build, protoc-bin-vendored` + `[dev-dependencies]: tempfile, rstest, proptest`
- `crates/ingest/build.rs` (NEW) — `tonic_prost_build::configure().compile_protos(...)` invocation; vendored protoc via `protoc_bin_vendored::protoc_bin_path()` + `unsafe { std::env::set_var("PROTOC", ...) }` (Edition 2024 set_var unsafe contract)
- `crates/ingest/proto/opentelemetry/proto/{common,resource,trace,metrics,logs,collector/{trace,metrics,logs}}/v1/*.proto` (8 NEW files) — vendored OTLP wire-format definitions per Apache 2.0 source github.com/open-telemetry/opentelemetry-proto v1.7.0 (~700 lines total)
- `crates/ingest/src/lib.rs` — added `pub mod grpc;` and `pub mod state;` alongside existing `pub mod contract;`
- `crates/ingest/src/contract.rs` — extended `Error` enum with `BindFailed { reason }`, `InvalidPort { value }`, `ServeFailed { reason }` variants; `heartbeat_payload` signature changed to take `&IngestState` and read real `span_count`
- `crates/ingest/src/state.rs` (NEW) — `IngestState` with atomic counters (span_count / log_record_count / metric_data_point_count / broadcast_subscribers) + `IngestStateSnapshot` + `record_*` methods + 4 unit tests including concurrent multi-threaded fan-in
- `crates/ingest/src/grpc.rs` (NEW) — full gRPC server module: 3 service implementations (`TraceServiceImpl`, `MetricsServiceImpl`, `LogsServiceImpl`) with `#[tracing::instrument(skip(req), fields(rpc.system, rpc.service, rpc.method, span_count, traceparent))]`; `parse.protobuf` inner debug span; `extract_traceparent(metadata)` reading `grpc-trace-bin`; `try_bind(addr)` + `serve_on(listener, state)` split for bind-status reporting; `MAX_DECODING_MESSAGE_SIZE = 8 * 1024 * 1024` applied per `*ServiceServer::new(impl).max_decoding_message_size(...)` chain; vendored proto stubs included via nested module `pub mod proto::opentelemetry::proto::*`
- `crates/ingest/tests/grpc_loopback.rs` (NEW) — integration test suite: P1 step 3 (live tonic client → 100 spans → response OK + state.span_count >= 100); 9 MB payload rejection (asserts `tonic::Code::OutOfRange` or matching size-rejection variant); bind collision returns `Error::BindFailed`; security grep gate (`no_non_loopback_bind_literals_in_ingest_src` walks `crates/ingest/src/` recursively asserting zero `0.0.0.0` / `Ipv4Addr::UNSPECIFIED` / `[::]` literals)
- `crates/ui-bridge/src/health.rs` — added `BindStatus { Ok | Failed(String) }` enum; extended `HeartbeatState` with `otlp_grpc_bind: Mutex<Option<BindStatus>>` slot + `record_otlp_grpc_bind` setter + `otlp_grpc_bind` reader; `current_health()` populates `subsystems.otlp_grpc_receiver.{status,error_msg}` from the slot + sets envelope `status` to `Degraded` on bind-failed; 2 new co-located tests (slot starts empty / records Ok and Failed)
- `pulse-app/src/heartbeat.rs` — extended `spawn(state, ingest_state)` signature with `Arc<IngestState>`; `emit_ingest_tick(state, ingest_state)` reads real `span_count` via `ingest::contract::heartbeat_payload(ingest_state)`; updated 5 co-located tests
- `pulse-app/src/main.rs` — added `resolve_grpc_port()` helper honoring `ANDROMEDA_PULSE_OTLP_GRPC_PORT` env var with `u16::parse` validation + warn-on-invalid fallback to default 4317; instantiates `Arc<IngestState>` + `let addr = SocketAddr::from(([127, 0, 0, 1], port))`; spawns gRPC server inside Tauri `setup()` via `tauri::async_runtime::spawn(...)`; records `BindStatus::Ok` immediately after `try_bind` succeeds (before `serve_on` future); emits `app.boot.otlp.grpc.bind` info / error events
- `pulse-app/src/observability.rs` — extended `ingest` allowlist with rpc.system / rpc.service / rpc.method / traceparent / latency_ms / method / status / attributes_count / service_name_tag / trace_ids fields (covers boundary span field allowlist for both `ingest::grpc` module-path target and `ingest.tick` heartbeat target via existing `for_target` strip-suffix lookup); registered 3 new target keys: `app.boot.otlp.grpc.bind` (bind_address, reason), `app.boot.otlp.grpc.port` (raw_len), `ingest.grpc.parse.error` (span_field_invalid, expected_length, actual_length, rejection_reason — chunk #18 invariant violation scaffold)
- `deny.toml` — added `{ crate = "foldhash" }` to `[bans] skip` list with provenance comment (tonic-prost-build pulls foldhash 0.1; tauri-utils / dom_query pull foldhash 0.2; both transitive, both build-time only, benign per route#16 2026-05-04). The `tonic <0.14` deny canary remains outside the skip list (intent preserved).

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-13/{combined.md, research.md, plan.md}` (NEW) — Phase 13 planning artifacts for chunk #16 (194 + 72 + 248 lines)

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent` (substantial new content from ingest crate's tonic + prost + tokio-stream + tonic-prost runtime deps + tonic-build + tonic-prost-build + protoc-bin-vendored build-deps + proptest + rstest dev-deps)
- `.andromeda/context/api-surface.md` — reconciled via `cargo public-api --simplified` per-crate iteration; updated ingest section (new Error variants + state module + grpc module top-level surface + grpc::proto::* nested OTLP types summary) + ui-bridge section (BindStatus enum + HeartbeatState slot accessors)
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advanced to 16 + epoch 2; session_count=16; spec_amendments.{active,archive} unchanged (Path A' did not write amendment); drift_warnings empty; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-04T23:55Z
- `.claude/rules/observability.md` — Session Additions appended 1 entry (opentelemetry-proto 0.31 SDK-pull gotcha + vendored .proto resolution path)
- `.claude/docs/session-learnings.md` — prepended 1 entry (tonic 0.14 prost-feature split into separate `tonic-prost` runtime + `tonic-prost-build` build crates)
- `.claude/session-handoff.md` — this file

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition — `.claude/rules/observability.md`: opentelemetry-proto 0.31 has opentelemetry+opentelemetry_sdk as direct deps; vendor .proto + tonic-prost-build for strict spec compliance (confidence ~0.8; explicit user-validated discovery via Trigger 4 cycle, persistent invariant)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 1 addition — tonic 0.14 split prost integration into separate tonic-prost crate (confidence ~0.7; technical migration detail; future tonic work in any crate will need this)
- **Filtered:** ~3 candidates rejected — (1) Edition 2024 `unsafe { std::env::set_var(...) }` requirement: rejected as task-specific (general Rust 2024 knowledge, not project-specific gotcha); (2) Vendored .proto + tonic-build as Path A' resolution pattern: rejected per Filter 1 dedup (token overlap >0.7 with the Tier 2 entry); (3) `protoc-bin-vendored = "3"` as cross-platform protoc solution: deferred per max-3 cap.

## Deferred learnings

2 learnings analyzed but not applied due to max-3 cap:

- **try_bind / serve_on bind-status pattern (Tier 3 candidate, confidence ~0.6):** `tonic::transport::Server::serve(addr)` doesn't separate bind from serve — its returned future resolves only when the server stops. To record `BindStatus::Ok` immediately after listener accepts (before long-running serve future), split into `try_bind(addr) -> TcpListener` + `serve_on(listener) -> Future` using `tokio_stream::wrappers::TcpListenerStream` + `Server::serve_with_incoming(stream)`. Future bind-status reporting in buffer/viz/snapshot servers should follow this pattern.
- **`protoc-bin-vendored = "3"` as zero-system-dep cross-platform protoc solution (Tier 3 candidate, confidence ~0.6):** when `tonic-prost-build` (or any prost-build downstream) requires `protoc` at build time, the cleanest cross-platform path is `protoc-bin-vendored = "3"` build-dep + `unsafe { std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?) }` in `build.rs`. Adds ~10MB of vendored binaries (one per OS/arch) but completely removes the system-dep on `protoc` in PATH; matches arch §Cross-cutting Patterns "deterministic harness" + agent-driven CI hygiene. Review manually with `/wrap-session --review` if any should be applied.

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 55/55, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok (after foldhash skip-list addition), cargo audit ok with 18 pre-existing allowed warnings, cargo xtask test 55/55. Security gate `no_non_loopback_bind_literals_in_ingest_src` test passes asserting zero forbidden literals. Obs criterion #7 verified via `cargo tree -p ingest | grep opentelemetry` → no matches.)

## Tests Status

passing — 55 cargo nextest (across workspace; was 36 last wrap, +19 from chunk #16: 4 state tests, 6 grpc unit tests, 4 integration tests in tests/grpc_loopback.rs, 2 contract tests, 2 ui-bridge bind tests, 1 heartbeat additional test) + cargo deny ok + cargo audit ok + 0 ESLint errors (webview unchanged) + cargo clippy clean + cargo fmt clean + cargo xtask test green. Total: 55 tests + 4 lint/typecheck gates + 2 supply-chain gates = 61 checks. cargo nextest ~180ms; cargo deny ~300ms; cargo audit ~200ms.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #17:**

`/andromeda-phase` to plan chunk #17 "OTLP HTTP receiver — axum 0.8 :4318 bound 127.0.0.1, POST /v1/{traces,metrics,logs}, DefaultBodyLimit + Host-header allowlist + CORS deny". Continues Epoch 2 — Ingest pipeline. Substantial chunk: axum 0.8 server + 3 HTTP POST handlers + 127.0.0.1 bind + DefaultBodyLimit + Host-header allowlist (DNS rebinding defense per security plan §Loopback-only) + CORS deny + post-prost invariants per security-plan + tracing instrumentation per obs-plan. Likely 1-chunk plan (single substantial; cross-domain coordination security/obs/tests).

**Priority 2 (background, optional) — Verify vendored protoc + .proto compilation on next agent boot:**

The `crates/ingest/build.rs` invokes `tonic_prost_build::compile_protos(...)` against 8 vendored `.proto` files using vendored protoc binary (~10MB across 8 OS/arch variants in `protoc-bin-vendored-*` crates). Run `cargo clean -p ingest && cargo check -p ingest` on a fresh clone or different OS to verify cross-platform reproducibility. Should be a no-op verification but worth confirming on Linux/macOS runners during CI matrix bring-up.

## Session Goals (carry-over)

(none — chunk #16 fully implemented + tests green + Trigger 4 → Path A' cleanly resolved (no marker file / no state.yaml entry per spec-drift-protocol Path A' discipline) + curation applied (1 Tier 2 + 1 Tier 3) + reconcile complete + Epoch 2 OPENED; ready for `/andromeda-phase` to plan chunk #17)

## Session End Status

clean
