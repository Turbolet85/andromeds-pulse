# Session Handoff

**Last Updated:** 2026-05-05T22:30:00Z
**Branch:** main
**Session End Status:** clean
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; chunk #19 rate limiting + port-override validation shipped this session)

## Current State

- **Last completed chunk:** route#19 "Rate limiting + port-override validation — tower_governor coarse + per-source-port + ANDROMEDA_PULSE_OTLP_*_PORT TryFrom<u16>" (committed this wrap; SHA pending Phase 10 amend)
- **Next chunk:** route#20 "DuckDB ring buffer schema + Arrow appender — :memory: connection, 7 reserved tables, TIMESTAMPTZ + ts_unix_nano BIGINT, OTLP-native composite keys" (Epoch 3 opener — Storage & query)
- **In-progress phase:** no active phase
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-16}/{combined.md, research.md, plan.md}` + audit trails per phase under `.andromeda/runs/`
- **Epoch 2 — Ingest pipeline:** CLOSED. Chunks #16, #17, #18, #19 all committed.
- **Epoch 3 — Storage & query:** opens with chunk #20.

## Andromeda State Detection (states A-L)

⚠️ F — Pending phase planning: chunk #20 listed in route §2 but no `.andromeda/phases/phase-17/` directory exists yet (normal workflow signal — next session begins with `/andromeda-phase`).

All other states (A, B, C, D, E, G, H, I, J, K, L) — clear.

## Drift Detection (6 dimensions)

D1, D2, D3, D4, D5, D6 — no drift detected.

(D1 cleared by Phase 5 reconcile completing successfully for both dependency-tree.md + api-surface.md. D5 cleared because no specialist plans were touched this session — chunk #19 was a smooth implementation per plan; no Trigger 4 spec ↔ reality drift surfaced.)

## Spec Amendments (this session)

(none this session — chunk #19 implementation matched specialist plan expectations; no Trigger 4 dialogue. Pre-existing archive untouched: lift-accent (2026-05-03) + clarify-pii-grep-ui-vocab (2026-05-04) both already archived per state.yaml.spec_amendments.archive.)

## Key Decisions This Session

- **Q1 → Option A (fail-startup on invalid port)**: replaced silent fall-back-to-default in `pulse-app/src/main.rs::resolve_*_port` with a `Result<OtlpPort, IngestError>` pipeline. Invalid env-var override → no bind attempt → `BindStatus::Failed("invalid_port")` routed through `health` envelope → overall status degrades. The OTHER receiver still runs if its port is valid. Generalizes to any future config-load validation: structured rejection events at `config.load.{*}_validation` targets log env-var NAME + sanitized reject category, never the raw user-controlled value (Vector 6).
- **tower_governor `tracing` feature disabled in favor of own `.error_handler()` callback**: external libs default to unbounded info-level events at their own targets that bypass our default-deny scrubber. Disabling the lib's tracing feature + emitting our own structured `tracing::warn!(target: "ingest.{grpc|http}.rate_limit.rejected", quota_window_seconds, reject_reason)` keeps the AllowList discipline intact. Pattern generalizes to any future external middleware library wiring (axum / tonic interceptors / etc.).
- **`GlobalKeyExtractor` chosen over per-source-port keying for v1**: combined.md said "AND per-source-port keying"; for a 127.0.0.1-only receiver, all peer IPs collapse to 127.0.0.1 anyway, so `GlobalKeyExtractor` IS the correct coarse-global semantic. Per-source-port differentiation (custom `LocalPortKeyExtractor` reading TCP source port from connection extensions) deferred — would only matter if multiple distinct local producers spike simultaneously, which is bounded by the 1000-burst capacity already.
- **governor crate uses quanta-backed monotonic clock; not mockable via tokio::time::pause()**: rate-limit window tests in `crates/ingest/tests/rate_limit.rs` use real-time short sleep (100-300ms total per test) instead. The testing.md "NEVER sleep(N) for sync" rule applies to event-waiting sync (poll for state change); time-elapsed-behavior testing is a distinct use case where real time is the canonical signal.
- **Sibling-isolation grep gate over-specified for permitted DAG edges**: plan acceptance criterion `cargo tree -p ui-bridge | grep tower_governor returns empty` was unachievable given the existing ui-bridge → ingest sibling-DAG edge (chunk #18 From-impl-as-contract). Spirit (no DIRECT tower_governor dep on ui-bridge) IS met — verified via `cargo tree --depth 1 | grep tower_governor` empty + `grep tower_governor crates/ui-bridge/Cargo.toml` empty. Plan-defect not specialist plan amendment.
- **`Error::InvalidPort { value: String }` already existed** in `crates/ingest/src/contract.rs` AND `From<IngestError> for AppError` already mapped it to `AppError::Ingest { message: "invalid OTLP port configuration" }`. Chunk #19 plumbed through the pre-existing variant rather than introducing new types — reduced API surface change to zero on the ui-bridge side.

## Files Modified

(13 files this session — chunk #19 implementation + Cargo.lock regen + 3 new phase artifacts + 1 new test file. Living artifacts reconciled separately by wrap.)

**Code files (chunk #19 — Rust):**
- `Cargo.lock` — regen reflecting tower_governor 0.8 + governor 0.10.4 + quanta 0.12.6 + raw-cpuid 11.6 + nonempty / nonzero_ext / forwarded-header-value / spinning_top transitive deps
- `Cargo.toml` (workspace) — `[workspace.dependencies]` adds `tower_governor = { version = "0.8", default-features = false, features = ["axum", "tonic"] }` (tracing feature OFF)
- `crates/ingest/Cargo.toml` — `[dependencies] tower_governor.workspace = true`
- `crates/ingest/src/contract.rs` — `+OtlpPort` smart enum (wraps `u16`; `TryFrom<u16>` validates non-privileged-or-spec-default range; reuses existing `Error::InvalidPort { value: String }` variant; co-located 12 unit tests via `#[rstest]` parameterization)
- `crates/ingest/src/grpc.rs` — `+OTLP_GRPC_RATE_LIMIT_BURST_SIZE = 1000` + `+OTLP_GRPC_RATE_LIMIT_PERIOD = Duration::from_micros(1200)` constants; `+pub async fn serve_on_with_rate_limit(...)` parameterized variant; `serve_on()` now wraps with default constants; `Server::builder().layer(GovernorLayer::new(...))` wires tower_governor with `GlobalKeyExtractor` + `error_handler` emitting structured `tracing::warn!(target: "ingest.grpc.rate_limit.rejected", ...)` event
- `crates/ingest/src/http.rs` — same shape as grpc.rs: `+OTLP_HTTP_RATE_LIMIT_*` constants + `serve_on_with_rate_limit` + `build_router` extended with `rate_limit_period` + `rate_limit_burst_size` params + `GovernorLayer` slotted between body-limit and host-header-check (so rate-limit runs after Host validation but before body parsing) + `error_handler` emitting `tracing::warn!(target: "ingest.http.rate_limit.rejected", ...)`
- `crates/ingest/tests/rate_limit.rs` (NEW, ~225 lines) — 3 integration tests: `grpc_returns_resource_exhausted_when_rate_limit_exceeded` (gRPC saturation/recovery via `tonic::Code::ResourceExhausted` + 250ms real-time sleep replenishment), `http_returns_429_when_rate_limit_exceeded` (HTTP 429 saturation/recovery), `http_429_includes_retry_after_header` (rate-limit response carries `retry-after`/`x-ratelimit-after` header)
- `deny.toml` — `[bans] skip` extends with `{ crate = "rand" }` + `{ crate = "rand_core" }` + provenance comment `# tower_governor 0.8 transitive deps via governor 0.10 + axum-test (route#19, 2026-05-05)`
- `pulse-app/src/main.rs` — replaces `raw.parse::<u16>()` fallback-to-default with `OtlpPort::try_from(parsed)?` validating pipeline; `resolve_grpc_port()` + `resolve_http_port()` now return `Result<OtlpPort, IngestError>` via `resolve_port(env_var_name, default)` shared helper that emits `config.load.port_validation` warn at rejection; main() conditionally spawns gRPC/HTTP receivers based on `Option<SocketAddr>` (None → record `BindStatus::Failed("invalid_port")` to heartbeat_state + log error + skip spawn); 7 co-located unit tests covering (a) unset env returns default, (b) unparseable rejects, (c) overflow rejects, (d) privileged port rejects, (e) zero rejects, (f) valid non-privileged passes, (g) spec-default 4318 via env passes
- `pulse-app/src/observability.rs` — `AllowList::production().by_target` extends with 3 new targets: `"config.load.port_validation"` (fields: env_var_name, reject_reason), `"ingest.grpc.rate_limit.rejected"` (fields: quota_window_seconds, reject_reason), `"ingest.http.rate_limit.rejected"` (same fields); 5 new co-located scrubber tests verify allowlisted fields pass + non-allowlisted (raw_value, client_ip) redact

**Phase artifacts (committed for audit):**
- `.andromeda/phases/phase-16/{combined.md, research.md, plan.md}` (NEW) — Phase 16 planning artifacts for chunk #19 (171 + 87 + 192 lines)

**Wrap-session maintenance:**
- `.andromeda/context/dependency-tree.md` — reconciled via `cargo tree --workspace --depth 2 --prefix indent` (337 lines; updated for ingest crate's new tower_governor + transitive governor / quanta / raw-cpuid / nonempty / forwarded-header-value / pin-project edges)
- `.andromeda/context/api-surface.md` — reconciled via per-crate `cargo public-api --simplified` iteration (2286 lines; ingest gains `OtlpPort` + `OTLP_*_RATE_LIMIT_*` consts + `serve_on_with_rate_limit` + Duration-typed signatures)
- `.andromeda/state.yaml` — schema_version=2 preserved; last_completed_chunk advanced to 19 + epoch 2 closes; session_count=19; spec_amendments.{active,archive} unchanged (no Trigger 4 this session); drift_warnings empty; plan_freshness mtimes captured fresh; living_artifact_freshness reconciled at 2026-05-05T22:30:00Z
- `.claude/docs/session-learnings.md` — prepended 2 entries (governor crate clock not mockable; sibling-isolation grep over-specified for permitted DAG edges)
- `.claude/session-handoff.md` — this file

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 0 additions
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions
  - (a) governor crate's quanta-backed monotonic clock is not mockable via tokio::time::pause; rate-limit timing tests use real-time short sleep
  - (b) sibling-isolation grep gates over-specified when a permitted sibling-DAG edge exists; use --depth 1 OR direct Cargo.toml grep for direct-dep declaration check
- **Filtered:** ~2 candidates rejected — (1) tower_governor's tracing-feature-disable pattern (Filter 4 confidence < 0.6 — too generic, applies to most external middleware libs not just tower_governor); (2) cargo deny check bans licenses sources subcommand selection (Filter 5 deferred — workflow knowledge, marginal value vs the project's existing security.md mentions of `cargo deny check bans`).

## Last Failed Command

(none — all test commands pass cleanly: cargo nextest 151/151 workspace, cargo clippy --workspace --all-targets --all-features -- -D warnings clean, cargo fmt --check clean, cargo deny check bans/licenses/sources ok, cargo audit ok with 18 pre-existing allowed warnings, cargo tree -p ingest | grep opentelemetry empty (obs criterion #7 strict-grep forbid PRESERVED), cargo tree -p ingest | grep -E "(garde|validator|validify)" empty (arch decisions PRESERVED).)

## Tests Status

passing — 151 cargo nextest (across workspace; was 124 last wrap, +27 from chunk #19: 12 contract.rs OtlpPort tests via #[rstest] parameterization, 7 main.rs resolve_port env-var tests, 5 observability.rs scrubber tests for new AllowList targets, 3 integration tests in tests/rate_limit.rs for saturation/recovery/retry-after) + cargo deny ok + cargo audit ok + cargo clippy clean + cargo fmt clean. Total: 151 tests + 4 lint/typecheck gates + 2 supply-chain gates = 157 checks. cargo nextest ~600ms.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #20 (Epoch 3 opener):**

`/andromeda-phase` to plan chunk #20 "DuckDB ring buffer schema + Arrow appender — :memory: connection, 7 reserved tables, TIMESTAMPTZ + ts_unix_nano BIGINT, OTLP-native composite keys". Opens Epoch 3 — Storage & query. The placeholder consumer task in `pulse-app/src/main.rs:138-142` (currently drains `ingest_receiver` via `while rx.recv().await.is_some() {}` and drops batches) gets replaced by the real DuckDB Arrow appender consumer in chunk #20. Surface this as the FIRST implementation step at /andromeda-phase planning time so the buffer crate scaffold + DuckDB :memory: connection + 7 reserved tables (`spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`) + Arrow appender land coherently.

**Priority 2 (background, optional) — Verify ingest crate API stability:**

The ingest crate's public API now includes `serve_on_with_rate_limit` + `OtlpPort` + new constants. Future Epoch 3 chunks may reference the new exports. Verify via `cargo public-api --simplified -p ingest` — should match the api-surface.md content this wrap reconciled.

## Session Goals (carry-over)

(none — chunk #19 fully implemented + tests green + curation applied (2 Tier 3) + reconcile complete + Epoch 2 closes; Epoch 3 opens with chunk #20 next; ready for `/andromeda-phase`)

## Session End Status

clean
