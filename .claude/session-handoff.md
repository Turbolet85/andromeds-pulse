# Session Handoff

**Last Updated:** 2026-05-12T19:55:00Z
**Branch:** main
**Session End Status:** clean (chunk #47 implemented + all gates green + Phase 2b boot verified user-screenshot-confirmed)
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; closes session 57)

## Current State

- **Last completed chunk:** route#47 "Plugin loader + IPC routers — strict-path canonicalize from ~/.andromeda-pulse/plugins/, plugins.list/reload/invoke + xtask drift check, built-in templates" (Epoch 7 — Plugin runtime + MCP server)
- **Next chunk:** route#48 "rmcp stdio sidecar + double-gate — andromeda-pulse-mcp binary --features mcp-server + ANDROMEDA_PULSE_MCP_ENABLED runtime gate, JSON-RPC 2.0, stderr-forced-JSON" (Epoch 7 continues)
- **In-progress phase:** none — chunk #47 loader + router + xtask drift + plugin-templates landed this session; phase-44 artifacts archived (combined.md + research.md + plan.md)
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-44}/{combined.md, research.md, plan.md}` (phase-44 from this session)
- **Epoch 7 — Plugin runtime + MCP server: 3 of 5 chunks closed (#45 substrate + #46 sandbox + #47 loader; #48-#49 pending).** Total route §2 chunk count: 56 (unchanged).

## Andromeda State Detection (states A-K)

⚠️ **E — Pending phase planning**: chunk #48 next; no `.andromeda/phases/phase-45/` directory yet. Remediation: `/andromeda-phase` to plan chunk #48.

(A, B, C, D, F, G, H, I, J, K all clean post-wrap.)

## Drift Detection (6 dimensions)

**No drift detected this wrap.**

- D1 (living artifact staleness): cleared by Phase 5 reconcile — dep-tree.md + api-surface.md both reconciled 2026-05-12T19:55:00Z with fresh tooling output reflecting chunk #47 delta (+2 lines on dep-tree.md pulse-app block; +30 lines on api-surface.md plugins crate block).
- D2 (wrong content): clean — Phase 5 tooling output captured cleanly; both artifacts updated atomically with stdout-only content.
- D3 (plan-to-code): clean — arch §Workspace crates LOCKED list (10 members) matches Cargo.toml workspace.members; `plugins.list`/`plugins.reload`/`plugins.invoke` present in bindings.ts; `cargo xtask capability-drift` exits 0.
- D4 (plan-to-plan): clean — no spec amendments this session.
- D5 (plan-to-CLAUDE.md mtime): clean — no upstream (arch + 6 specialist plans + route + input) regenerated this session; all upstream mtimes ≤ CLAUDE.md mtime (CLAUDE.md updated 2026-05-11T19:17Z via session 55's full setup-project re-derive).
- D6 (route chunk progression): clean post-wrap — state.yaml.last_completed_chunk advances to chunk #47 with this wrap's commit SHA.

## Spec Amendments (this session)

(none this session — chunk #47 implementation surfaced no Trigger 4 spec ↔ reality drift; the strict-path workspace-dep "declared but unused" observation is documented in session-learnings as a Tier 3 entry, not amendment-worthy because the security plan's strict-path reference is aspirational rather than mandate-form, and the workspace-detector precedent of manual canonicalize+traversal-check satisfies the same security intent.)

state.yaml.spec_amendments.active: 0 entries (unchanged from session 56 — clean lifecycle).
state.yaml.spec_amendments.archive: 17 entries (unchanged from session 56).

## Key Decisions This Session

- **Single-chunk phase 44 plan**: chunk #47 grouped alone per "single-substantial" heuristic — estimated ≥3h, touches plugins crate + ui-bridge + pulse-app/capabilities/ + xtask + plugins-examples/, cross-cuts security (strict-path canonicalize) + arch (TauRPC namespace, AppError::Plugin rebind) + obs (plugins.tick heartbeat activation). Grouping with chunk #48 (rmcp stdio sidecar) would breach cognitive review window.

- **Research-time correction at /implement (Open Q5 strict-path API)**: grep over project source confirmed `strict_path::` is not used in any *.rs file — only declared in `crates/workspace-detector/Cargo.toml:12` + Cargo.toml workspace.dependencies line 43 as dead deps. workspace-detector's `detect.rs:21-62` uses `std::fs::canonicalize` + manual `path_contains_traversal` (3-line ParentDir check) instead. Decision: chunk #47 loader follows the ACTUAL codebase precedent (manual canonicalize + traversal check), NOT the speculative `strict_path::PathBoundary` API in research.md. Same security intent (path canonicalization + CWE-22 confinement); no new dep needed.

- **WIT export-name correction**: research.md guessed `snapshot-template` export as `render-template`; actual WIT at `crates/plugins/wit/snapshot-template.wit` declares `render` (matching custom-dashboard's export name). `loader::declared_exports()` reflects the real WIT — `custom-dashboard`: `["render"]`, `data-transform`: `["transform"]`, `snapshot-template`: `["render"]`. Tests parametrize on these.

- **`plugins.invoke` capability-handshake-only semantics**: chunk #47 validates the requested capability name matches one of the plugin's category WIT exports and returns success/rejection — no actual `wasmtime::component::Instance::call` invocation. Full export invocation requires `wasmtime::component::bindgen!` per-category type generation; deferred to a future chunk. Documented in plan.md §Implementation notes + chunk #47 invoke fn doc-comment.

- **active_invocations counter placement**: `registry.record_invocation()` fires AFTER lock acquisition but BEFORE the find() check — so not-found invocations also count toward the cumulative counter. Test expectation set to 3 (all 3 invokes increment regardless of outcome). Matches the security-relevant signal of "plugin host activity".

- **emit_taurpc_bindings test = 4th binding**: fix-loop iteration #4 caught a confusing capability-drift "missing: 3" diagnosis. Root cause: the test in `pulse-app/src/main.rs::tests::emit_taurpc_bindings` triggers the bindings.ts emission via `Router::into_handler()` in dev mode — production router's contents don't matter for emission; only the test's router does. Without adding `plugins_impl.clone().into_handler()` to the test's `.merge(...)` chain, bindings.ts omits plugins.* even though EXPECTED_PROCEDURES is correct. Curated to `.claude/rules/security.md` Session Additions 2026-05-12 as a Tier 2 rule (extending the 2026-05-09 triple-binding cluster to a quadruple).

- **specta::Type unconditional in pulse-app DTOs**: fix-loop iteration #2. ui-bridge uses `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` for AppError because xtask consumes it without taurpc; pulse-app has no `taurpc-runtime` feature (binary always builds with Tauri), so DTOs in pulse-app should use unconditional `#[derive(specta::Type)]`. Documented in session-learnings Tier 3 entry.

## Files Modified

This wrap's commit:

**NEW files (9):**
- `crates/plugins/src/loader.rs` (~470 lines — `PluginRegistry` + `LoadedPlugin` + `resolve_plugin_dir` + `canonicalize_plugin_dir` + `discover_plugins` + `declared_exports` + 17 rstest cases)
- `pulse-app/src/plugins_router.rs` (~320 lines — `PluginsApi` trait + `PluginsApiImpl` + 3 DTOs + `#[taurpc::resolvers]` impl + 7 tokio tests)
- `plugins-examples/README.md` + `plugins-examples/{custom-dashboard,data-transform,snapshot-template}-example/README.md` (4 template scaffolds)
- `.andromeda/phases/phase-44/combined.md` (223 lines)
- `.andromeda/phases/phase-44/research.md` (120 lines)
- `.andromeda/phases/phase-44/plan.md` (341 lines)

**MODIFIED files (14):**
- `crates/plugins/src/contract.rs` — 3 new Error variants + Hash derive on PluginCategory + refactored heartbeat_payload signature + 3 new round-trip Display tests
- `crates/plugins/src/lib.rs` — `pub mod loader;` added
- `crates/ui-bridge/src/contract.rs` — `From<PluginsError> for AppError` refactored: plugin_id-bearing variants route through `AppError::Plugin`; PathCanonicalizationFailed routes through `AppError::Validation`; tracing target switched to `ui-bridge.error.plugin`; 5 new round-trip + 5 new tracing-target tests
- `pulse-app/Cargo.toml` — `wasmtime.workspace = true` + `wat.workspace = true` (dev-dep) added
- `pulse-app/src/main.rs` — `mod plugins_router;` + pre-router construction of plugin_engine + canonical_plugin_dir + plugins_registry + plugins_impl; merged into router; `heartbeat::spawn` call extended; `emit_taurpc_bindings` test extended with PluginsApiImpl merge
- `pulse-app/src/heartbeat.rs` — `run_plugins` + `emit_plugins_tick` accept `Arc<Mutex<PluginRegistry>>`; `spawn` signature extended + `#[allow(clippy::too_many_arguments)]`
- `pulse-app/capabilities/plugin-fs.json` + `default.json` — descriptions updated for chunk #47 actualization (no permissions change)
- `xtask/src/main.rs` — 3 plugins.* entries uncommented in EXPECTED_PROCEDURES + 1 new test
- `.claude/docs/services/plugins.md` — loader.rs + plugins_router.rs added to Entry points; chunk #47 status landed
- `.andromeda/context/dependency-tree.md` — fresh cargo tree output captured (310 → 312 lines; pulse-app gains wasmtime + wat direct deps)
- `.andromeda/context/api-surface.md` — fresh cargo +nightly public-api output captured (4715 → 4745 lines; +30 lines plugins crate public API delta from loader::* + 3 new Error variants + heartbeat_payload signature change)
- `.andromeda/state.yaml` — session_count 56 → 57; last_wrap/last_reconcile refreshed; last_completed_chunk advanced to chunk #47; drift_warnings empty; plan_freshness mtimes re-captured
- `.claude/rules/security.md` — 1 new Tier 2 entry (emit_taurpc_bindings 4th binding)
- `.claude/docs/session-learnings.md` — 2 new Tier 3 entries (strict-path dead-dep + specta::Type unconditional pulse-app)
- `.claude/session-handoff.md` — full overwrite (this file)
- `Cargo.lock` + `pulse-app/ui/src/bindings/index.ts` — auto-regenerated

**Audit trail (gitignored — `.andromeda/runs/`):**
- `.andromeda/runs/2026-05-12T08-00-00-phase-44/{.raw-{specialty}.md × 7, {specialty}.md × 7}` — phase 44 sub-agent raw + stripped extracts

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition to security.md ("emit_taurpc_bindings test = 4th binding"; confidence 0.85)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions ("strict-path dead-dep observation"; "pulse-app DTOs unconditional specta::Type"; both confidence 0.75)
- **Filtered:** 0 duplicates + 0 task-specific + 0 conflicts + 1 deferred (Tauri silent-boot pattern; already in 2026-05-12 session-learnings) + 1 below-threshold (chunk #47 capability-handshake-only deferral; already in plan.md)

## Last Failed Command

(none — all session 57 operations succeeded.)

## Tests Status

passing — 678/678 Rust workspace tests (+40 vs session 56's 638 baseline reflecting chunk #47 test additions: loader 17 + plugins_router 7 + contract 4 + ui-bridge 10 + xtask 1 + heartbeat 1 = 40 net).

Coverage gates sustained: workspace TOTAL 87.28% line / 87.86% function (above ≥75 line / ≥85 function thresholds per test-plan §10 Standard tier). Chunk #47 per-file: `crates/plugins/src/contract.rs` 100% / 100%; `crates/plugins/src/loader.rs` 91.67% / 92.50%; `pulse-app/src/plugins_router.rs` 83.20% / 80.65%; `pulse-app/src/heartbeat.rs` 96.24% / 96.83%.

Phase 2b runtime smoke: ✓ pulse-app booted, compact widget rendered with Halo State Pulse, no `panicked at` / `app.panic.fatal`; user-screenshot-confirmed at 2026-05-12T19:45Z.

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #48 (Epoch 7 continues):**

route#48 "rmcp stdio sidecar + double-gate — andromeda-pulse-mcp binary --features mcp-server + ANDROMEDA_PULSE_MCP_ENABLED runtime gate, JSON-RPC 2.0, stderr-forced-JSON". Introduces the rmcp sidecar as a separate binary crate (`andromeda-pulse-mcp`) gated by `--features mcp-server` at compile-time + `ANDROMEDA_PULSE_MCP_ENABLED=true` env var at runtime (double-gate per security plan §MCP feature double-gate). Stdout reserved for JSON-RPC 2.0 framing; stderr forced-JSON (per obs plan + universal anti-pattern row 7). `mcp.{status,start,stop}` namespace already pre-enumerated in arch §Occupied Resources — likely no `/andromeda-scope-arch` run required.

**No outstanding remediation items** — D1-D6 clean, state E expected (pending phase-45 planning is normal for chunk-completion wrap), no active spec amendments, no stale drifts. Clean session-start state for the next /andromeda-new-session invocation.

## Session Goals (carry-over)

(none — session 57 user goal achieved: chunk #47 plugin loader + IPC routers + xtask drift gate + plugin-examples scaffolds implemented + all gates green + Phase 2b smoke verified.)

## Deferred decisions (Trigger 4 to Path B carry-over)

(none — no Trigger 4 dialogues this session.)

## Deferred learnings (filtered out from Phase 4 curation)

(2 deferred: (a) Tauri silent boot pattern verification — duplicate of 2026-05-12 session-learnings from session 56; (b) chunk #47 capability-handshake-only deferral — already in plan.md §Implementation notes.)
