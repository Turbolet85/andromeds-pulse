# tests extract

## Relevance
Relevant — a backend/data-flow correctness chunk whose proof IS a tests deliverable (deterministic-L4 storm integration test); squarely in-domain via {test-plan} §5 (Integration), §7 (fixtures), §3 (harness gates), §10 (quality gates).

## Constraints
- **Standard gate set is unconditional** — Test Commands MUST list `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` (per {test-plan} §3 "Per-chunk gate discipline"). Webview gates (`npm run lint`/`typecheck`/`test --prefix pulse-app/ui`) are excluded here only because scope's OUT boundary keeps `pulse-app/ui/**` untouched.
- **Boot-smoke gate is REQUIRED** — the chunk edits `pulse-app/src/main.rs` (the `incident_workspace_key` assignment), a listed boot path, so Test Commands MUST add `cd pulse-app && npx @tauri-apps/cli dev` (60s; watch `Local:`/`ready in`/`Compiled successfully` before SIGTERM) (per {test-plan} §3 "Boot-smoke gate (conditional)").
- **Integration tier, in-process, deterministic runner** — the module↔module boundary is proven by direct Rust calls with `rstest`/builders under the P-073 deterministic-L4 runner (no GPU/model), not E2E (per {test-plan} §5 Drivers + §2 pyramid Integration row; scope method=`integration`).
- **Self-bootstrapping fixtures, no pre-baked store** — the storm seeds its own incidents at runtime via digests/builders; no `.sql` or pre-populated registry (per {test-plan} §7 "Self-bootstrapping requirement" + §2 agent-runnable invariants).
- **Deterministic, no sleep-sync** — assert on the explicit signal (the workspace-filtered `list_active()` return), never `sleep(N)` / real wall-clock (per {test-plan} §11 E2E "NEVER use `sleep(N)`" + §2 "no real setTimeout").
- **Coverage + zero-flakiness** — changed-code coverage ≥ 75% line / 70% branch / 85% function; the storm→incident assertion must be reproducible, no retry-once (per {test-plan} §10).

## Patterns to follow
- The existing storm integration test `pulse-app/tests/integration_tier1_storm_one_incident.rs` is the canonical shape to extend: `DeterministicInferenceRunner::new(ModelTier::Primary)` + `InMemoryIncidentRegistry` + a recording `IncidentPersistence` double, driving N identical-fingerprint `storm_digest(seq)` through `handle_digest_outcome` → `create_incident_from_l4_output`, then asserting `registry.list_active(WORKSPACE).len()`. Note its `WORKSPACE = "/home/dev/payments"` is already a detected-root-shaped key (not a data dir) — the reconciliation target (per {test-plan} §5).
- Resolver-side assertion sites are already wired: `pulse-app/src/services_router.rs` (`ServicesApiImpl`) calls `self.incident_registry.list_active(&self.workspace_root)` and `pulse-app/src/incidents_router.rs` (`IncidentsApiImpl::list_active`) calls `self.registry.list_active(&self.workspace_root)` — both filter by the same `workspace_root`; the proof asserts non-empty return + non-healthy `priority_tier` through them.
- TauRPC contract tests run in-process via `tauri::test::mock_builder()` + `get_ipc_response()` for `services.list_with_states` / `incidents.list_active` (per {test-plan} §5 Module↔TauRPC IPC).
- workspace-detector canonicalization test pattern: `tempfile::TempDir` + `.andromeda/` marker + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR` → `workspace::detect()` → assert canonical path (per {test-plan} §5 "workspace-detector → filesystem" row; `assert_fs`/`assert_cmd`). A dedicated key-equality assertion (producer `digest.workspace` == resolver key, including the Windows `\\?\` form) closes the mismatch.
- pulse-app tests belong in `pulse-app/tests/*.rs` integration crates, NOT source-level `#[cfg(test)] mod tests` (`[lib] test = false` makes those compile-but-never-run) — testing.md/session-learnings 2026-05-20.

## Anti-patterns to avoid
- NEVER `sleep(N)` to wait for the storm to land — await the explicit query-result signal (per {test-plan} §11 E2E).
- NEVER pre-populate the incident store via `.sql`/snapshot — self-bootstrap the storm at runtime (per {test-plan} §11 "Test Data" + §7).
- NEVER add the proof test to `pulse-app/src/*.rs` `mod tests` (dead under `[lib] test = false`); and if it captures tracing across a `spawn_blocking` hop, assert the SPECIFIC target event (not `!captured.is_empty()` — vacuous-pass trap) and install via `set_global_default` (testing.md session-learnings 2026-05-20 + 2026-06-28).

## Contract bindings
- **tests deterministic-L4 storm harness ↔ triage incident producer (arch/obs):** the harness drives cadence→digest→L4→incident so the producer stamps `digest.workspace`; that stamp vs the resolver's `list_active(workspace)` filter is the exact data contract this chunk reconciles.
- **tests boot-smoke gate ↔ arch boot path:** `npx @tauri-apps/cli dev` exercises `main.rs` where the reconciled `incident_workspace_key` is assigned; the warm-boot obs-log smoke (session-learnings 2026-07-05) is where storm/incident + `services`/`incidents` tick evidence surfaces.
- **tests canonicalization parity ↔ obs/workspace-detector:** Windows `canonicalize()` returns `\\?\C:\...` while other sources don't — a known string-equality trap (observability.md session-learnings 2026-06-04); the equality assertion must hold across the `\\?\` form.
- **tests status/health ↔ obs §3:** unchanged here — this chunk does not alter the §3 status-endpoint shape; binding is read-only (boot readiness only).

## Acceptance criteria contributions
- (tests) A deterministic-L4 (P-073) sustained identical-fingerprint storm test in `pulse-app/tests/*.rs` asserts the workspace-filtered `list_active()` returns ≥1 incident (not zero) — the key mismatch is closed (per {test-plan} §5; scope Acceptance).
- (tests) The affected service's `priority_tier` from `services.list_with_states` renders non-healthy AND `incidents.list_active` returns the storm incident, asserted via resolver call or `tauri::test::mock_builder()` IPC (per {test-plan} §5 cross-module patterns).
- (tests) No-regression: a genuinely-zero-active-incident state still yields empty / all-healthy (P-067 live-only truth preserved) (per {test-plan} §5; scope Acceptance).
- (tests) `cargo nextest run --workspace --profile ci` + `cargo fmt --check` + `cargo clippy … -D warnings` + `cargo xtask capability-drift` pass, plus the `npx @tauri-apps/cli dev` 60s boot-smoke (main.rs touched); changed-code coverage ≥ 75% line (per {test-plan} §3 + §10).

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate for boot-path chunks** (test-plan-amendments §2026-05-09): directly triggered — this chunk edits `pulse-app/src/main.rs`, a named boot path. Why it exists: the in-process 5-command harness missed a latent boot-time panic in a non-Tokio context (chunk #27/#30 `health.rs` "no reactor running") across 4 wraps; a main.rs edit must gate on the Tauri-dev smoke.
- **2026-05-10 — Standard gate baseline unconditional** (§2026-05-10): the chunk's Test Commands must carry the full gate set regardless of the narrow backend scope. Why: omitting gates let pre-existing `fmt`/`tsc` failures surface N chunks late (chunk #36→#37).
- **2026-06-10 — Chunk #99 capability-matrix / load-profiles** (§2026-06-10): NOT triggered — scope lands no P-001–P-060 capability and no load profile, and reuses existing `services.list_with_states` + `incidents.list_active` (only `cargo xtask capability-drift` applies, no matrix extension). Noted because prospective P-079 is operator-surfaced/unminted (confirm at P5); if minted it would extend the matrix JSON in the same chunk, but scope expects "no new namespace."
