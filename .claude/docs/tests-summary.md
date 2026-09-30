# Tests Summary

_Distilled from `.andromeda/test-plan.md` by `/setup-project`. Read on demand. **Tier: Standard (1).**_

## Why Standard tier
Cross-platform desktop app (Windows/macOS/Linux via Tauri 2) with 19 testable entities, 7 critical paths, 28+ security anti-patterns requiring negative tests, and "portfolio-worthy GPU-accelerated visualization" risk tolerance. Agent-driven discipline requires every test layer to be machine-parseable end-to-end (no human-in-the-loop, no Percy/Chromatic).

## Test harness contract (5-command discipline)

| Command | Body |
|---|---|
| `boot` | pre-build under the verb's own env (`cargo build --bin pulse-app --release` + `cargo build -p xtask`), then spawn `target/release/pulse-app[.exe]` BY PATH (no `cargo run` wrapper; the sh verb runs it under a waiting subshell that records `run/andromeda-pulse.spawn` (the app pid boot uses) and, on reaping it, `run/andromeda-pulse.exit`; ps1 mirrors it since 2026-09-30 through a hidden `powershell -EncodedCommand` wrapper writing `exit N`, its boot/`ended` leg driven live once by hand, a committed leg still owed) + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$` + `RUST_LOG=debug`; poll the `cargo xtask harness:status` verdict until `running-healthy` (10s default, honest — builds absorbed before the window; `HARNESS_STATUS_TIMEOUT` overrides) |
| `run` | `cargo nextest run --workspace --profile ci --message-format libtest-json` |
| `status` | `cargo xtask harness:status` — real-process verdict JSON `{verdict, pid, ended, log_file_basename, last_write_age_seconds, stale_after_seconds}` (`ended` = the `run/andromeda-pulse.exit` record, `exit N` / `signal N (NAME)`, when not `running-healthy` — real under both scripts since 2026-09-30, `exit -1` after `Stop-Process -Force`), exits 0/1/1/2 from PID file + pid liveness (a dead pid is `not-running`, since 2026-09-29) + log-family mtime (since 2026-08-30; replaces the exit-0-unconditional in-process `health` form) |
| `cleanup` | terminate the pidfile's pid (app-written value canonical; msys `kill` + PowerShell fallback), bounded wait, KILL escalation; verdict from INDEPENDENT probes (pid liveness + resolved OTLP ports at loopback): one bounded token `clean`/`app-survived`/`ports-lingering`/`no-pid-ports-accepting`, exit 0 only on `clean` (since 2026-08-30-agent-harness-teardown-truth) |
| `logs` | `tail -F` the newest match of the ROTATED family `agent-latest.jsonl*`, resolved across three precedence-ordered bases (`ANDROMEDA_PULSE_LOGFILE` / `$ANDROMEDA_PULSE_DATA_DIR/logs/`, then `~/.andromeda-pulse/logs/`, then the data-dir `logs/`) — never a bare name, never `*.log` (`rolling::daily` date-suffixes the sink) |

## Status endpoint shape (binding)
```json
{
  "status": "ok" | "degraded" | "unhealthy",
  "subsystems": {
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "buffer": { "status": "ready" | "error", "rows_ingested": N, "retention_seconds": N },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": N }
  },
  "uptime_ms": N,
  "pid": N
}
```

## PID file
`<data_dir>/run/andromeda-pulse.pid` on every platform (`<data_dir>` = `resolve_data_dir()`, `ANDROMEDA_PULSE_DATA_DIR` override; harness-only `ANDROMEDA_PULSE_PIDFILE` overrides the whole path). The app's own `write_pid_file` value is canonical — it overwrites the provisional pid boot writes (taken from the waiting wrapper's `run/andromeda-pulse.spawn` record). Single decimal PID per line. (The per-OS `$XDG_RUNTIME_DIR`/`$TMPDIR`/`%LOCALAPPDATA%` set was stale v1 text, corrected at 2026-08-30-agent-harness-teardown-truth.)

## Log format (binding contract)
JSON-per-line; fields: `timestamp` (ISO-8601), `level`, `target`, `message`, `fields`. Optional: `trace_id`, `span_id`, `duration_ms`, `span_count`, `service`. Source: `tracing-subscriber::fmt::Layer::json()` per obs §3.

## Critical paths (P1–P7 E2E)

| Path | Surfaces | Verification |
|---|---|---|
| **P1** | OTLP/gRPC + ingest + buffer + viz + TauRPC + webview | Send 100 gRPC spans → `health.subsystems.buffer.rows_ingested >= 100` → `traces.query` returns matching rows |
| **P2** | buffer + snapshot + TauRPC + Channel `pulse://stream/snapshot-progress` | Populate 500 spans → `snapshot.generate({token_budget: 25000})` → assert anomaly markers + `token_count <= 25000` + `dedup_count > 0` + p50/p95/p99 aggregates |
| **P3** | MCP stdio sidecar + buffer | Spawn with `--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true` → JSON-RPC `tools/call query_traces` → assert array of trace objects |
| **P4** | plugins + WASM Component Model + TauRPC | Stage fixture WASM → `plugins.reload` → `plugins.list` → `plugins.invoke({capability})` → negative test on disallowed capability |
| **P5** | webview + tray + IPC | 17-stage headful path (`cargo xtask webview-drive`) asserting obs+DOM halves per stage; `widget-close` is no longer terminal (`signpost-repeat` follows it, proving the P-063 signpost fires on every close) and requires the `ui.layout.transition {compact-widget → hidden}` record AND the `tray.signpost.shown` record with `window_label == "compact-widget"`; resize / tray-repaint / notification stay IPC-surrogate |
| **P6** | buffer + Tauri Channel + Arrow IPC | Subscribe to `pulse://stream/spans` → ingest gRPC → decode Arrow IPC → schema match |
| **P7** | workspace-detector + TauRPC | `workspace.detect` → assert `{project_name, root_path, vcs_type: "git", vcs_root}` |

## Test pyramid (Standard tier)
| Level | Coverage | Tools |
|---|---|---|
| Unit (Rust) | ≥75% line / ≥70% branch / ≥85% function | `cargo test` + `cargo-nextest` 0.9 |
| Unit (webview, since chunk #11) | Presentational components (e.g., icons/) EXCLUDED at Foundation pre-shell stage; integration coverage via tauri-driver from chunk #25 | `vitest` 4 + `jsdom` 26 + `@testing-library/react` 16 |
| Integration | All Standard Contracts + boundary types | `tauri::test::mock_builder()` + `tonic` 0.14.5 + `axum-test` 18.7 + `duckdb-rs` 1.5 |
| E2E | All 7 critical paths | `@crabnebula/tauri-driver` 2.x + `webdriverio` 9.x via `cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]` (no `mocha`, no `@wdio/*` runner — measured 2026-08-23); DOM-driven, so selectors are measured per surface: accessible name where one ships, `data-testid` where none does. Native Windows live, Linux `xvfb-run` not yet exercised; NOT CI-wired |
| Property | Selective per trigger | `proptest` 1.10 |
| Performance / Load | Four dist-arch v3 profiles (release/tag gate) + per-PR 10k profile | `pulse-app/tests/perf_load_profiles.rs` (nextest `load-profiles` profile via `cargo xtask perf:load-profiles`); `perf_slo_10k_spans.rs` per-PR; live injector `crates/ingest/examples/load_profiles.rs`; `criterion` 0.5 in `xtask benches/` |
| Chaos / Fault | Trigger-driven | `tokio::time::pause()` + manual broadcast disconnect |

## Self-bootstrapping fixtures
- NO pre-baked DuckDB snapshots, NO `.sql` scripts.
- OTLP ingest via `tonic` client (fixture seeding through the live receiver path).
- Builder factories: `MockTraceSpan::builder()` (`ingest`), `MockArrowBatch::builder()` (`buffer`), `MockMetricPoint::builder()` (`viz`) — **none of the three exists yet** (0 workspace hits each, measured 2026-08-23; tracked as test-plan §1 `test-data-bootstrap-factories-unimplemented`). Self-bootstrapping is met meanwhile by OTLP ingest + per-test in-memory DuckDB seeding + `rstest` fixtures.
- Fixture composition: `rstest` 0.26.1 with `#[fixture]`.
- Per-test isolation: `tempfile::TempDir` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$`.
- WASM plugin fixtures: pre-compiled minimal Component Model `.wasm` binaries in `tests/fixtures/plugins/` (committed read-only).
- **Webview unit-test fixtures (since chunk #11):** Vitest's default test pool + `@testing-library/react` `render()` per test; `afterEach(cleanup)` registered via `pulse-app/ui/src/test-setup.ts`. Parameterized cases via `describe.each(...)` / `it.each(...)` (mirrors `rstest` parameterization at the JS layer). DOM-shape assertions only (no Percy/Chromatic; no visual diff per agent-driven discipline).

## Quality gates (CI)
- Coverage ≥75% line / ≥70% branch / ≥85% function via `cargo-llvm-cov` 0.8.5 — measured with `xtask/` excluded TEMPORARILY (founder ruling 2026-09-29; revisited at the next epoch-boundary code audit).
- **Zero-flakiness budget** — flake = real bug; quarantine + fix or delete (NO retry-once policy).
- Performance budgets (p99): OTLP gRPC <100ms, OTLP HTTP <120ms, TauRPC `traces.query` <150ms, snapshot 25k budget <500ms, DuckDB Arrow appender <50ms, WebGPU 10k spans/sec ≥30 fps sustained (descriptive — the ASSERTED frame budget is obs §10's `metric.webgpu.frame_duration_ms` p99 ≤33ms, per test-plan §12 2026-06-10).
- **Capability verification matrix (chunk #99):** `docs/v0_2_0/capability-verification-matrix.json` (60 P-entries → named scenarios) validated by `cargo xtask verify:capability-matrix` in CI; P-061+ extend the matrix in the landing chunk.
- **Four-profile load suite (chunk #99, release/tag gate):** `pulse-app/tests/perf_load_profiles.rs` behind nextest `[profile.load-profiles]` via `cargo xtask perf:load-profiles` (baseline/high/burst/sustained-extreme per dist-arch v3; default/ci profiles exclude it); load-shaped tests join the profile, never `#[ignore]`. A second opt-in profile `[profile.perf-samples]` selects `pulse-app/tests/perf_budget_samples.rs`, the in-process producer of the memory + snapshot samples CI's `cargo xtask perf:budget --require memory,snapshot` grades (lint-test Linux); the default profile excludes both binaries.
- `cargo deny check bans` blocks `multiple-versions = "deny"` regression (catches `tonic` 0.14/0.13 duplicate).
- Cranelift-only WASM enforcement: build-time check on `wasmtime` Cargo.lock.

## CI integration
GitHub Actions matrix (Linux/macOS/Windows × Rust stable):
- Lint → Unit tests → Integration → E2E (matrix per platform) → Coverage → Quality gates.
- Capability gates: `cargo xtask capability-drift` (worktree + staged-index bindings; runs the staged assertion since 2026-08-30) → `cargo xtask check:staged-artifacts` (the direct staged-artifacts verb) → `cargo xtask capability-widening-check` → `cargo xtask verify:capability-matrix` (chunk #99).
- JUnit XML output via `cargo nextest --message-format junit`; `dorny/test-reporter` for inline PR annotations.
- Build fails on: any test failure, coverage below threshold, flaky test, perf regression, lint/typecheck/`cargo deny check` failure.

## Scenario legs vs gates (test-plan §3)
`cargo xtask pre-push:linux` (2026-09-29) is the local Linux pre-push check — dev-host only (Windows + the WSL `Ubuntu` distro), not a CI step: it syncs a distro clone to HEAD + the working tree (tree-id verified) and runs `script-modes` · `npm` · `clippy` · `test` · `ci-gates` in order, first failure stops; exit 0 green · 1 red · 2 cannot-evaluate (a missing pinned tool or apt package — never green), one JSON verdict + `target/pre-push/report.json`. Its Node 24 is the Viola repo's `~/.local/viola-node` install (the distro's apt ships Node 22).

`cargo xtask perf:frame-sample` (2026-09-30) is the dev-host frame-budget gate — not in the standard gate set, not CI-wired (the hosted Windows runner exposed no WebGPU adapter): Windows only, it boots the release binary with a SwiftShader WebGPU flag set on the app child only, feeds `inject_demo --sustained` for 30 s, and grades frame p99 ≤ 33 ms with the frame arm required; exit 0 PASS · 1 FAIL (0 samples once healthy, or over budget) · 2 INCONCLUSIVE; opens a window. `cargo xtask perf:budget --data-dir <DIR> --require <arm,arm>` is its CI sibling — exit 0 PASS · 1 FAIL · 2 cannot-evaluate.

Not every harness command is a gate. There are now FOUR scenario legs, all deliberately outside the unconditional standard gate set and all dev-host-only (the third, `cargo xtask smoke:hue-shift`, grades the P-025 hue-shift samples by anchor; the fourth, `cargo xtask smoke:discovery` (2026-09-30), grades the P-027 discovery bound on a fresh boot — the first `metric.constellation.discovery_ms` within 5000 ms of the first `duckdb.append {table_name: spans}`, its own anchor within 1000 ms of that append, 0 panic and 0 ERROR; INCONCLUSIVE when no spans were appended, the webview was not polling first, or the data dir was not fresh; a window below 21 s is refused). `cargo xtask smoke:external-resolve` (2026-08-29) drives the app-registry-vs-durable-store divergence: boot → finite storm → resolve an incident through a REAL `andromeda-pulse-mcp` subprocess → observe two 60 s persist cycles. Its verdict keys on `reconciled_count` (≥ 1) and `declined_count` (0) and deliberately **NOT** on `item_count` — measured non-discriminating, since a mutation removing reconciliation still showed 2 → 0 in both arms (the finite storm auto-resolves inside the window). `--sustained` cannot drive it (constant error rate never trips the cue gate) and the injector must be backgrounded, or the 120 s auto-resolve window closes before the external write lands. Like its sibling it proves its own preconditions (INCONCLUSIVE, never PASS) and refuses an observe window below two persist cycles. `cargo xtask smoke:gap-resume` (2026-08-28) has THREE arms: `--sustained` (arm B) is gate-grade; **arm A became gate-grade on 2026-08-28** when the defect it reproduced was closed by the `duckdb`/`libduckdb-sys` 1.10502 → 1.10505 bump — it now exits 0 under a TWO-HALF precondition (the silence storm formed AND ≥1 append REJECTED on replayed identities, via `StormEvidence.append_rejections`), because arm A drives the injector with `--replay` and a clean drain without rejections would prove the collision was absent rather than survived; and `--reconnect-only` (`conflicts_with = "sustained"`) reconnects with no idle gap, which established that the idle window is not the trigger. **A red on arm A is now a real regression, and the standing instruction survives the promotion: never weaken, remove, or `--expect-absent` it** — it is the only on-demand reproduction of this defect class. Each arm proves its OWN precondition and may not borrow another's (the gap arm needs a silence storm — absent ⇒ INCONCLUSIVE, not PASS; the control arm needs its absence; the stormless reconnect arm needs FEED evidence instead, ≥1 `duckdb.append`). The former known gap is CLOSED: `observe_window_supports_verdict()` refuses an `observe_minutes` window below the 450 s stall threshold the verdict reads, rather than reporting a structurally-impossible PASS.

**RED-leg evidence depends on the defect's SHAPE.** A REJECT-shaped defect (write refused, batch failed) emits the ERROR it is measuring, so the clean-log assertion must not be applied to it. A **BLOCK-shaped** defect emits nothing — nothing returns, so nothing reports — and a RED leg over it carries 0 ERROR and 0 panics, which makes that same assertion pass VACUOUSLY; there the evidence of record is absence-of-progress (frozen `rows_ingested`, consecutive zero-delta `buffer.tick`s, `buffer.consumer.stalled`). Decide the shape before choosing the evidence.

## Top anti-patterns (test-plan §11)
- NEVER `sleep(N)` for sync — wait for `health` polling or Channel events.
- NEVER mock OTLP receiver under test — use live `tonic`/`reqwest` to loopback.
- NEVER mock DuckDB at integration — use ephemeral `duckdb::open_in_memory()`.
- NEVER pre-populate test DB via `.sql` scripts — self-bootstrapping fixtures only.
- NEVER use `std::time::Instant::now()` / `chrono::Utc::now()` without injection — `tokio::time::pause()` for determinism.
- NEVER tolerate flake — quarantine and fix immediately.
- NEVER ship release with known test failures.
- NEVER include "human verifies canvas" steps — pixel inspection out of agent-driven scope.
- ~~NEVER allow OTLP self-dialing — negative test must assert prevention.~~ **DEPRECATED 2026-05-08** — by-construction-satisfied per obs-plan.md `2026-05-02 — Phase 3.5 pivot to tracing-only self-observation` (no OTel SDK in self-observation runtime → no exporter to misconfigure → no `ANDROMEDA_OBSERVER_URL`-shaped variable exists → trigger is unimplementable). The architectural "must-never-exist" guard for `ANDROMEDA_OBSERVER_URL` remains in `gotchas.md` §Self-observation. Body sites in test-plan.md §1 Coverage triggers (lines 48 + 255) + §Anti-Patterns Project-specific bullet (line 866) are annotated with `**[DEPRECATED 2026-05-08]**` first-column prefixes / `> **DEPRECATED (2026-05-08)**` indented blockquote (content preserved verbatim for audit trail). Per amendments `2026-05-08T17-28-26Z-deprecate-self-otlp-loop-test` (Decisions Log only) + `2026-05-08T21-00-00-obs-pivot-test-bodies` (body annotations).
- NEVER dump raw OTLP JSON as snapshot — assertion requires dedup count + anomaly markers + p50/p95/p99 aggregates.
- NEVER skip post-`prost` invariant negative tests.
- NEVER assert receiver bound to `0.0.0.0` succeeds.

## Pending coverage triggers (deferred to next `/andromeda-tests` re-run)

The following test triggers are documented but not yet implemented; they are deferred to the next `/andromeda-tests` re-run rather than processed via delta-rerun, because each requires dedicated harness design (xtask test command + fixture pattern) beyond cross-reference scope.

**PII vector test gaps** (security plan §Logging vectors 2/3/4/6 currently lack triggers; test plan covers vector 5 only):
- `security-vector-coverage: AppError sanitization` — assert IPC error response excludes stack traces / file paths / Rust struct names / library versions; grep on `agent-latest.jsonl` after IPC error
- `security-vector-coverage: Plugin path basename only` — load plugin with symlink chain → assert resolved path NOT in logs, only basename
- `security-vector-coverage: MCP response body redaction` — call `query_traces` via MCP → assert `result_content` NOT in `agent-latest.jsonl`, only `result_type` + `result_count` metadata
- `security-vector-coverage: Path env var canonicalization log redaction` — set `ANDROMEDA_PULSE_PLUGIN_DIR=../../etc/passwd` → assert canonicalization rejects + logs only basename
- Per amendment `2026-05-08T17-28-27Z-document-pii-vector-test-gaps`.

**Capability widening static analysis — LANDED, gap closed** (security plan §API Anti-Patterns 3 NEVER-widen bans; `capability-drift` alone only checks TauRPC↔capability sync, which is why a second gate was needed):
- `security-vector-coverage: Capability widening static analysis` — shipped as `cargo xtask capability-widening-check` (chunk #77; CI-wired chunk #99; part of the §3 standard gate set since 2026-08-14). Parses each `pulse-app/capabilities/*.json` and asserts: (a) `pulse:notification` contains only outbound emit permissions (no input handlers); (b) `pulse:tray` contains only outbound menu/icon permissions (no incoming-event handlers); (c) `pulse:plugin-fs` permissions limited to read of resolved plugin dir, no write/delete/execute, never exposed to webview JavaScript. Test fails with named permission and capability on widening detection.
- Per amendment `2026-05-08T17-28-29Z-document-capability-widening-test-gap`.

**Boot smoke check gap** (chunks touching boot/setup paths lack runtime smoke gating; chunk #27/#30 latent panic at `crates/ui-bridge/src/health.rs:291` went undetected through 4 wrap cycles before chunk #31's Phase 2b smoke gate caught it):
- `boot-smoke-coverage: per-chunk Tauri dev smoke gate` — chunks touching `pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `pulse-app/src/observability.rs` (boot-time obs init), `pulse-app/src-tauri/tauri.conf.json`, OR `pulse-app/capabilities/*.json` MUST include `npx @tauri-apps/cli dev` (60s timeout; boot-completion signal detection: `Local:` / `ready in` / `Compiled successfully`) in their Test Commands. The direct-binary smoke variant (fresh `ANDROMEDA_PULSE_DATA_DIR` + env-gated mode + sustained `inject_demo`) is the accepted alternative where that form cannot express the needed runtime state — test-plan §3. On Phase 2b smoke failure, chunk surfaces "green per scope; runtime blocked" variant per /andromeda-implement Phase 3 — chunk implementation commits as-is; runtime blocker addressed via suggested redirect skill OR follow-up chunk.
- Per amendment `2026-05-09T16-00-54Z-smoke-check-boot-discipline`.

**Chunk-plan standard-gate baseline gap** (chunks #35-#36 committed with pre-existing fmt + tsc failures because their plan gate lists were incomplete; chunk #37's plan included the full set and exposed the inherited failures):
- `chunk-gate-baseline-coverage: standard gate set unconditional per chunk` — every `/andromeda-phase`-authored chunk plan's `## Test Commands` MUST list the FULL standard gate set: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo xtask capability-widening-check` + `cargo xtask check:ingest-progress` + `cargo xtask check:staged-artifacts` (staged git-index bindings + capability grants vs their pins; exit 0/1/2) + `cargo xtask capability-drift` (BEFORE the default-features workspace nextest, which rewrites `bindings/index.ts` to the no-mcp shape — overseer directive 2026-09-30; since 2026-08-30-staged-bindings-assertion the staged assertion is CODE folded into this gate itself) + `cargo nextest run --workspace --profile ci` + the `--features mcp-server` `emit_taurpc_bindings` regen (the last cargo-adjacent step) + `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` (the close, for a chunk adding no procedure; a chunk that CHANGES the procedure set closes instead with two content probes on the regenerated worktree bindings → `git add` of the bindings WITH the `EXPECTED_PROCEDURES` pin → `check:staged-artifacts`, then `capability-drift` LAST, and runs the regen BEFORE the `ui/dist` build and every release build a live leg drives — test-plan §3, measured 2026-09-30) + (when chunk's `Files-to-modify`/`New-files-to-create` lists touch any `pulse-app/ui/**` paths) `npm run lint --prefix pulse-app/ui` + `npm run typecheck --prefix pulse-app/ui` + `npm run test --prefix pulse-app/ui`. Webview gates excluded only when chunk touches zero `pulse-app/ui/**` paths. Boot-smoke conditional per existing `boot-smoke-coverage` trigger (cumulative, not exclusive). Pre-flight gate parity at chunk-plan authoring time prevents the carry-over accumulation that delays detection by N chunks.
- Per amendment `2026-05-10T12-41-03Z-mandate-standard-chunk-gates`.

**Living-artifact tooling re-run discipline gap** (wrap-session Phase 5 currently allows skipping tooling rerun on webview-only chunks; verified at chunk #37 wrap finding api-surface.md LIVING block 12× smaller in `pub` declarations than fresh `cargo public-api` output):
- `living-artifact-tooling-rerun-coverage: unconditional reconcile at every wrap` — `/andromeda-wrap-session` Phase 5 MUST execute the Tooling command from each `.andromeda/context/{artifact}.md` METADATA at every wrap regardless of chunk scope. Skip-on-webview-only is forbidden because (a) Cargo.lock advances independently across webview-only chunks via Tauri feature toggles, (b) cargo public-api re-derives from full dep graph + source AST so divergence is observable only by running the tooling. Format-mismatch reconciliation: when fresh tooling output diverges from LIVING block by >2× line count or >5× field-count metric, Phase 5 MUST overwrite LIVING block with fresh stdout. Skill-level fix lives in `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 5 spec; this trigger anchors the project-side requirement.
- Per amendment `2026-05-10T12-41-03Z-mandate-living-artifact-tooling-rerun`.

Full plan: `.andromeda/test-plan.md`.
