# Session Handoff

**Last Updated:** 2026-05-05T17:01:39Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #17 OTLP HTTP receiver — axum 0.8 :4318 + 3 POST handlers + DefaultBodyLimit 8MB + Host-header allowlist + CORS deny shipped this session)

## Current State

- **Last completed chunk:** route#17 "OTLP HTTP receiver — axum 0.8 :4318 bound 127.0.0.1, POST /v1/{traces,metrics,logs}, DefaultBodyLimit + Host-header allowlist + CORS deny" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#18 "Ingest channel + post-decode invariants — tokio mpsc backpressure, span_id 8 / trace_id 16 / attribute bounds, AppError::Ingest variant" (Epoch 2 — Ingest pipeline continues)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-14}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Epoch 2 — Ingest pipeline:** chunks #16 + #17 implemented; chunks #18 + #19 remain.

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #18 listed in route §2 but no `.andromeda/phases/phase-15/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear.

## Drift Detection (6 dimensions)

D1, D2, D3, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because no specialist plans were touched this session — chunk #17 was a smooth implementation per plan; no Trigger 4 spec ↔ reality drift surfaced.)

## Spec Amendments (this session)

(none this session — chunk #17 implementation matched specialist plan expectations exactly; no Trigger 4 dialogue.)

## Key Decisions This Session

- **HTTP/JSON content-type scoping deferred to follow-on chunk**: chunk #17 ships `application/x-protobuf` only; `application/json` returns HTTP 415 (Unsupported Media Type). Plan documented both paths at Phase 6 review; /implement resolved the open question by matching chunk title literal (preserves chunk scope discipline; defers `pbjson` codegen scope creep). Future HTTP/JSON chunk would add `pbjson-build` to `crates/ingest/build.rs` + content-type dispatch in handlers.
- **`tower::ServiceBuilder` order via `.layer()` chain**: each `.layer()` call wraps the previous, so the LAST `.layer()` is the OUTERMOST middleware. For chunk #17, the security ordering (tracing outermost → host check → body limit → CORS innermost) maps to `.layer(CorsLayer::new()) → .layer(DefaultBodyLimit::max(8 MB)) → .layer(middleware::from_fn(host_check)) → .layer(TraceLayer::new_for_http())` in code-order. Inverse of natural reading; documented in Tier 3 session-learnings.md as future-self gotcha.
- **`reqwest 0.12` AND `reqwest 0.13` coexist in dep tree without `cargo deny check bans` firing**: 0.12 added directly as ingest dev-dep for HTTP integration tests; 0.13 pulled transitively by `tauri-plugin-updater 2.10`. `cargo deny check bans` returned `bans ok` despite `multiple-versions = "deny"`. Either deny tolerates dev-dep duplicates or skip-list catches it implicitly. No action required; documented in Tier 3 entry.

## Files Modified

(11 files this session — chunk #17 implementation + reconcile + curation, includes Cargo.lock regen)

**Code files (chunk #17 — Rust + axum 0.8 stack):**
- `Cargo.toml` (workspace) — added `axum`, `tower`, `tower-http`, `bytes`, `http`, `axum-test`, `reqwest` to `[workspace.dependencies]` OTLP HTTP receiver section (with provenance comment)
- `Cargo.lock` — regen reflecting new resolved versions (axum 0.8.9, tower 0.5.3, tower-http 0.6.8, hyper 1.9.0, axum-test 18.7.0, reqwest 0.12.28)
- `crates/ingest/Cargo.toml` — opted into workspace deps (`axum`, `tower`, `tower-http`, `bytes`, `http`) + dev-deps (`axum-test`, `reqwest`)
- `crates/ingest/src/lib.rs` — added `pub mod http;` declaration
- `crates/ingest/src/http.rs` (NEW, ~430 lines) — full HTTP receiver module: `try_bind` + `serve_on` split mirroring grpc.rs; `build_router(state, port)` with 4-layer middleware stack (TraceLayer → host-header allowlist → DefaultBodyLimit 8MB → CorsLayer default-deny); 3 POST handlers (`/v1/traces`, `/v1/metrics`, `/v1/logs`) with `#[tracing::instrument]` boundary spans (skip + bounded fields including `http.method`, `http.route`, `http.status_code`, `body_size_bytes`, `traceparent`, `span_count`); `parse.protobuf` inner debug span; content-type dispatch (`application/x-protobuf` accepted, all else returns HTTP 415); `extract_traceparent_http(headers)` reading W3C `traceparent` header; `host_header_check(allowed, req, next)` middleware rejecting non-`{127.0.0.1, localhost, [::1]}:port` hosts with HTTP 403; reuse vendored proto types from `crate::grpc::proto::*`; reuse `IngestState` shared atomic counters; 12 co-located unit tests
- `crates/ingest/tests/http_loopback.rs` (NEW, ~225 lines) — integration suite: `start_test_http_server()` ephemeral-port helper; synthetic OTLP builders (`make_span`, `make_traces_request`, `make_metrics_request`, `make_logs_request`); 8 tests (P1 traces/metrics/logs roundtrips, body-over-8MB returns 413, host-header evil returns 403, CORS preflight no allow-origin, application/json returns 415, bind collision)
- `crates/ui-bridge/src/health.rs` — extended `HeartbeatState` with `otlp_http_bind: Mutex<Option<BindStatus>>` slot + `record_otlp_http_bind` setter + `otlp_http_bind` reader; `current_health()` adds parallel `match state.otlp_http_bind() {...}` block populating `subsystems.otlp_http_receiver.{status, error_msg, last_tick_at}` and degrading envelope status on `BindStatus::Failed`; 3 new co-located tests (slot starts empty / records Ok and Failed / current_health degrades on http bind failure)
- `pulse-app/src/main.rs` — added `ENV_OTLP_HTTP_PORT` constant + `resolve_http_port()` helper (mirror of `resolve_grpc_port`); renamed `port`/`addr` to `grpc_port`/`grpc_addr` for symmetry; added second `tauri::async_runtime::spawn` block for HTTP server boot with `bind_announcer = http_announcer` + `http_state` clones, `app.boot.otlp.http.bind` info/error events, and BindStatus reporting through `record_otlp_http_bind`
- `pulse-app/src/observability.rs` — extended `AllowList::production()` `ingest` allowlist with 11 new HTTP-boundary fields (`http.method`, `http.route`, `http.status_code`, `content_type`, `route`, `body_size_bytes`, `limit_bytes`, `http_response_code`, `status_code`, `host_header_rejected`, `expected_host_class`); added 4 new `by_target` registry entries (`app.boot.otlp.http.bind`, `app.boot.otlp.http.port`, `ingest.http.parse.error`, `ingest.http.body_size.exceeded`)

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-14/{combined.md, research.md, plan.md}` (NEW) — Phase 14 planning artifacts for chunk #17 (172 + 73 + 235 lines)

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent` (substantial new content from ingest's axum + tower + tower-http + http + bytes + dev-deps axum-test + reqwest with full transitive listings)
- `.andromeda/context/api-surface.md` — reconciled via per-crate `cargo public-api --simplified` iteration (substantial growth in ingest section due to axum 0.8 transitive public surface; ui-bridge gained `record_otlp_http_bind` + `otlp_http_bind` accessors)
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advanced to 17 + epoch 2; session_count=17; spec_amendments.{active,archive} unchanged (no Trigger 4 this session); drift_warnings empty; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-05T17:01:39Z
- `.claude/rules/observability.md` — Session Additions appended 1 entry (tracing dotted field names + AllowList.for_target prefix-strip resolver)
- `.claude/docs/session-learnings.md` — prepended 2 entries (axum/tonic stack harmonized; axum::Router.layer chain order)
- `.claude/session-handoff.md` — this file

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition — `.claude/rules/observability.md`: dotted-name tracing fields + AllowList.for_target `::`-prefix-strip resolver (confidence ~0.8; useful for future HTTP/RPC/messaging boundary instrumentation)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions — (a) axum 0.8 + tonic 0.14 share tower 0.5 + hyper 1 cleanly (no transitive deny duplicate); (b) axum::Router.layer chain order: each `.layer()` wraps the previous, so LAST chained = OUTERMOST middleware (confidence ~0.8 each; both are technical facts useful for future Epoch 2-4 HTTP work)
- **Filtered:** ~5 candidates rejected — (1) reqwest Host header override (Filter 4 confidence < 0.6); (2) HTTP/JSON deferral process tip (Filter 2 task-specific); (3) `http::header::HOST` canonical idiom (Filter 4); (4) CorsLayer::new() default-deny + axum's 405 for OPTIONS combine for free CORS deny — DEFERRED to handoff per max-3 cap (confidence ~0.6, was 4th by confidence rank); (5) reqwest 0.12 vs 0.13 coexistence detail surfaced inline in Tier 3 entry #1 instead of separate entry.

## Deferred learnings

1 learning analyzed but not applied due to max-3 cap:

- **`tower_http::cors::CorsLayer::new()` default-deny combined with axum's default 405 for OPTIONS-on-POST-only routes gives free CORS denial without preflight handler (Tier 3 candidate, confidence ~0.6):** the layer doesn't add `Access-Control-Allow-Origin` to ANY response when no `.allow_origin()` configured; OPTIONS preflight reaches axum's 405 (Method Not Allowed) which carries no CORS headers. Combined posture is spec-correct default-deny without explicit preflight-rejection handler. Documented inline in chunk #17 plan §Implementation notes; promote to Tier 3 if any future chunk needs to make explicit-deny CORS decisions. Review manually with `/wrap-session --review` if this should be applied next session.

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 78/78, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok, cargo audit ok with 18 pre-existing allowed warnings, cargo tree -p ingest | grep opentelemetry empty (obs criterion #7 strict-grep forbid PRESERVED).)

## Tests Status

passing — 78 cargo nextest (across workspace; was 55 last wrap, +23 from chunk #17: 12 http unit tests, 8 http_loopback integration tests, 3 ui-bridge tests for otlp_http_bind slot + degraded envelope) + cargo deny ok + cargo audit ok + cargo clippy clean + cargo fmt clean. Total: 78 tests + 4 lint/typecheck gates + 2 supply-chain gates = 84 checks. cargo nextest ~190ms.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #18:**

`/andromeda-phase` to plan chunk #18 "Ingest channel + post-decode invariants — tokio mpsc backpressure, span_id 8 / trace_id 16 / attribute bounds, AppError::Ingest variant". Continues Epoch 2 — Ingest pipeline. Substantial chunk: tokio mpsc channel with backpressure semantics + post-prost invariant validation (span_id 8 bytes / trace_id 16 bytes / attribute bounds) shared across both gRPC + HTTP receivers + `AppError::Ingest` variant + `ingest.{grpc,http}.parse.error` event population (allowlists already registered). Likely 1-chunk plan (single substantial; cross-cutting both receivers).

**Priority 2 (background, optional) — Verify HTTP/JSON deferral decision over a follow-on chunk:**

Chunk #17 ships `application/x-protobuf` only with HTTP 415 on `application/json`. If full OTLP HTTP spec conformance becomes a near-term priority (e.g., specific SDK clients require JSON encoding), schedule a follow-on chunk that adds `pbjson-build` to `crates/ingest/build.rs` + content-type dispatch in handlers + JSON roundtrip tests. Current: deferred.

## Session Goals (carry-over)

(none — chunk #17 fully implemented + tests green + curation applied (1 Tier 2 + 2 Tier 3) + reconcile complete + Epoch 2 continues with chunk #18 next; ready for `/andromeda-phase`)

## Session End Status

clean
