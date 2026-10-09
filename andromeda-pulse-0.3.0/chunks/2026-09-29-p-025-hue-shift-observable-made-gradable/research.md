# Codebase Research — 2026-09-29-p-025-hue-shift-observable-made-gradable

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (§5-command … §Anti-patterns + all 17 Session Additions); applied: the 2026-08-23 invoke-the-producer-by-path + one-exported-data-dir entry, the 2026-08-30b one-exported-`ANDROMEDA_PULSE_DATA_DIR`-for-cross-verb-state entry (binds §B), the `smoke:external-resolve` spawn form (deterministic L4 + bootstrap override + backgrounded finite storm, `xtask/src/external_resolve.rs:456-470`), the 2026-06-29 glob-the-`agent-latest.jsonl*`-family entry, and the release-first binary resolver (testing.md 2026-07-05 ext. 2026-08-27/28). Also `.claude/rules/testing.md`, `observability.md`, `a11y.md`, `frontend.md`, `design-tokens.md` (auto-loaded on the touched paths).
- **Platform issues consulted:** GitHub docs "Contexts reference — Context availability" (https://docs.github.com/en/actions/reference/workflows-and-actions/contexts), fetched 2026-09-29: workflow-level `env` → `github, secrets, inputs, vars`; `jobs.<job_id>.env` → `github, needs, strategy, matrix, vars, secrets, inputs` (NO `runner`); `jobs.<job_id>.steps.env` / `steps.run` / `steps.with` → include `runner` and `env`; `runner.temp` = "a temporary directory on the runner … emptied at the beginning and end of each job". The recorded runs (Setup 5a) are the runner-only witness: every red run at `f15536b` / `8b86529` / `83d4060` is path-named, 0 s wall-clock, `checks 0/0`.

## Files inspected
- `pulse-app/src/services_router.rs` (full) — `list_with_states` (`:93-123`) joins `list_active(&workspace_root)` on `scope == Service && scope_id == service`, reduces by `tier_rank` max (`:97-104`); holds only ACTIVE incidents at the decision point.
- `crates/triage/src/lifecycle/registry.rs` (`:40-110`, `:185-196`) — `ServiceListItem` (`:54`) `{service, state, last_seen_unix_nano, manual_override, #[serde(default)] priority_tier}`; `list_all` literal at `:189`.
- `crates/triage/src/incident/registry.rs` (`:60-367`) — `IncidentRegistry` trait; `InMemoryIncidentRegistry` (DashMap). `list_active` excludes ONLY `Resolved` (`:213-221`) — Acknowledged stays active. `mark_resolved` sets `resolved_at_unix_nano` (`:252-269`) and keeps the row in the map; no `remove`/`retain` anywhere in `crates/triage/src/incident/*.rs` or `pulse-app/src/*.rs` (grep `incidents.remove\|\.retain(` → 0).
- `crates/triage/src/contract.rs` (`:395-450`) — `Incident` fields: `opened_at_unix_nano` (`:418`), `updated_at_unix_nano`, `acknowledged_at_unix_nano`, `resolved_at_unix_nano`, `priority_tier`.
- `crates/triage/src/incident/broadcast.rs` (`:29-41`) — `transitioned_at_unix_nano` lives on the broadcast event, not on `Incident`.
- `pulse-app/src/inference_runtime.rs` (`:770-860`) — the sole production incident constructor; dedupe on `(kind, scope, scope_id)` calls `observe_reemission` and never rewrites `priority_tier`.
- `crates/corpus/src/contract.rs` (`:652`, `:682`, `:698`) — the two incident UPDATEs set `status, updated_unix_nano, resolved_unix_nano, payload` and `read_unix_nano`; hydration loads `status != 'resolved'` only.
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (full) — P-027 effect `:88-110`, P-025 effect `:116-156` (staleness anchor `:142`, slowest-wins `:143-146`, one emission per pass `:152`), WebGPU loop `:170-300` (reads `dotsRef.current`; effect deps `[adapter, reducedMotion]`).
- `pulse-app/ui/src/canvas/frame-loop.ts` (full) — reduced motion: `onReducedMotionFrame` fires ONCE on `start()`, no rAF; the header comment says consumers MAY re-call `start()` to repaint hue updates.
- `pulse-app/ui/src/canvas/frame-metrics.ts` (`:27-40`, `:140-200`) — `recordConstellationHueLatency` clamps `duration_ms` to [0, 60 000] and swallows errors.
- `pulse-app/ui/src/hooks/use-service-constellation.ts` (full) — `POLL_INTERVAL_MS = 1000` + focus re-poll.
- `crates/ui-bridge/src/telemetry.rs` (`:74-135`, `:215-300`) — `ConstellationHueLatencyInput {duration_ms: f64, severity_tier: HueSeverityTier}`, `validate_duration_ms` [0, 60 000], emit at target `metric.constellation.hue_update_ms` (`:279`).
- `pulse-app/src/observability.rs` (`:980-1000`) — exact leaf `metric.constellation.hue_update_ms = {duration_ms, severity_tier}`.
- `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` (full) — 3 tests: required-fields SUBSET, own-leaf-not-bare-`metric`, banned per-service identifiers.
- `pulse-app/ui/src/widget/ConstellationCanvas.test.tsx` (`:1-60`, `:120-175`) — the adapter is mocked `unavailable`; the hue test asserts an emission with `severity_tier == "autonomous"` on first sighting of a non-null tier.
- `.github/workflows/{ci,release,update-channels}.yml` — structure, `env:` blocks, first steps; `pulse-app/tests/quality_gate_workflow.rs` (`:185-210`) requires ci.yml's workflow-level `env:` to keep `CI_RUN_ID` / `GIT_COMMIT_SHA` / `DEPLOYMENT_ENVIRONMENT`.

## Graph impact (from the code-graph query; rust plane regenerated this run)
- **`ServiceListItem`** — 11 reference sites (trace `tree-query-{marker}.json`, 19 rows total; editor lines = SCIP+1): struct LITERALS only at `crates/triage/src/lifecycle/registry.rs:189` and `crates/triage/src/lifecycle/persistence.rs:312`, `:319` (both test module); the rest are `use` / type positions (`contract.rs:99` re-export, `lifecycle/mod.rs:45`, `services_router.rs:33,43`). A new `#[serde(default)] Option` field breaks exactly those 3 literals; the grep sweep `ServiceListItem {` over crates/pulse-app/xtask agrees (3 literals + the definition).
- **`list_with_states`** — callers are `pulse-app/tests/unit_services_router.rs:72,84,123` (+ the webview via bindings — the name-bridge: `hooks/use-service-constellation.ts:34`).
- **`tier_rank`** — `services_router.rs:104` (pub, its own), plus a DIFFERENT private `tier_rank` in `crates/triage/src/cue/emitter.rs:66` and `priority_tier_rank` in `digest/assembler.rs:692` — three private rank copies already exist.
- **`IncidentRegistry`** — exactly ONE implementor (`InMemoryIncidentRegistry`, grep `impl IncidentRegistry for` → 1); a new trait method threads nowhere else.

## Patterns detected
- **Binary-boundary join over a lower-crate registry** (`services_router.rs:93-105`): Tauri-free `triage` holds state; the pulse-app resolver joins. A pure derivation belongs in `triage` where co-located tests RUN (pulse-app `[lib] test = false`).
- **Injected-instant pure functions** (triage `now_unix_nano` convention, e.g. `evaluate_auto_resolution(now, window)` `incident/registry.rs:125`).
- **Webview delegated-timing emission** is fire-and-forget through `invokeTelemetry` (`frame-metrics.ts:157-172`); client clamps, server validates.
- **Reduced-motion repaint contract** (`frame-loop.ts:3-9`): re-calling `start()` repaints once under reduced motion — the widget canvas never does.
- **`$GITHUB_ENV` step pattern** is absent from the repo today (grep `GITHUB_ENV` over `.github/` → 0); every job's first step is `step-security/harden-runner` (SHA-pinned), which must stay first.

## Conventions to follow
- **pulse-app tests live in `pulse-app/tests/*.rs`** (ratchet `pulse_app_src_carries_no_new_dead_test_attributes`; testing.md).
- **`#[serde(default)] Option<T>` on a specta type renders `field?: T | null`** — normalize `?? null` at the consumer (frontend.md 2026-05-30).
- **Bindings regen → `npm run build` → release re-embed before any live leg**, and regen LAST before staging (testing.md 2026-08-30 ARGS_MAP entry; security.md 2026-06-12).
- **Live legs launch the binary by path with cwd = the throwaway data dir** — dev-mode `export_types()` writes bindings relative to cwd (`xtask/src/external_resolve.rs:457-458`).
- **Workflow edits keep SHA pins, `contents: read`, harden-runner first** (security.md §Supply chain).

## New files to create
- `crates/triage/src/incident/tier_effective.rs` — pure `tier_effective_at` derivation (replay of opened/resolved instants per service) + co-located tests
- `xtask/src/hue_shift.rs` — the `smoke:hue-shift` scenario leg (added at P4: scenario legs live in xtask by precedent — `external_resolve.rs`, `gap_resume.rs`) + its pure verdict and co-located tests

## Files to modify
- `crates/triage/src/lifecycle/registry.rs` — `ServiceListItem` gains `#[serde(default)] tier_effective_at_unix_nano: Option<i64>`; `list_all` literal emits `None`
- `crates/triage/src/lifecycle/persistence.rs` — two test literals gain the field
- `crates/triage/src/incident/registry.rs` — `IncidentRegistry` gains an all-statuses workspace listing; `InMemoryIncidentRegistry` impl + tests
- `crates/triage/src/incident/mod.rs` — declare the new module
- `crates/triage/src/contract.rs` — re-export the derivation
- `pulse-app/src/services_router.rs` — enrich `tier_effective_at_unix_nano` per item
- `pulse-app/tests/unit_services_router.rs` — rise / fall / acknowledged / cross-service cases
- `pulse-app/ui/src/bindings/index.ts` — regenerated (`ServiceListItem.tier_effective_at_unix_nano?`)
- `pulse-app/ui/src/widget/constellation-types.ts` — pure per-service hue-sample derivation
- `pulse-app/ui/src/widget/constellation-types.test.ts` — its pins
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` — fire site: per changed service, anchored on `tier_effective_at_unix_nano`; reduced-motion repaint on data change
- `pulse-app/ui/src/widget/ConstellationCanvas.test.tsx` — per-service emission + no-staleness + reduced-motion repaint pins
- `.github/workflows/ci.yml` — drop the workflow-level `runner.temp` line; per-job `$GITHUB_ENV` step after harden-runner
- `.github/workflows/release.yml` — same
- `.github/workflows/update-channels.yml` — same
- `pulse-app/tests/quality_gate_workflow.rs` — guard: no workflow- or job-level `env:` value references a step-only context, across all four workflow files
- `xtask/src/main.rs` — register the `smoke:hue-shift` command (added at P4)
- `xtask/src/external_resolve.rs` — widen the reused leg helpers (`spawn_app`, `wait_for_receiver`, `shutdown`, `wait_ports_released`, `binary_age`, `tempdir`, `log_family`, `locate_app_binary`, `build_injector`) to `pub(crate)`; no behaviour change (added at P4)

## Open questions
- Reduced-motion repaint: fix the widget canvas's once-only reduced-motion paint in this chunk (so a tier change repaints and the paint instant exists), or record it as a route entry and exclude reduced-motion samples → blocks: plan-decision
- First-sighting / restored-tier samples: emit only when the tier changed while this canvas instance was watching (`tier_effective_at >= mount instant`), or on every first sighting of a non-null tier as today → blocks: plan-decision
