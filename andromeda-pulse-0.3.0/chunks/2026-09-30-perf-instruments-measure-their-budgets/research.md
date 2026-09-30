# Codebase Research — 2026-09-30-perf-instruments-measure-their-budgets

## Scope
- **Depth:** deep (mature codebase; three instruments across Rust, xtask, webview and CI) · **Reads:** 24 · **Globs/Greps:** 21 · **Graph queries:** 3 (rust ×2, ts ×1; trace `.andromeda/runs/2026-09-30T16-06-19Z-phase/tree-query-2026-09-30-perf-instruments-measure-their-budgets.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (body :1-112 + Session Additions :113-153, 21 entries); applied: the direct-binary / one-exported-data-dir rule (2026-08-23 `agent-run.sh run` entry), the family-glob rule (2026-06-29), the leg-precondition + second-source rules (2026-08-28, 2026-08-29), the `perf:frame-sample` body entry (:89, opens a window), the ci-gates fresh-dir recipe (2026-05-14).
- **Platform issues consulted:** none — no runner-only bullet is folded (the one CI verdict Setup read is `in progress`, not red), and this chunk reads no hosted-runner failure.

## Files inspected
- `crates/snapshot/src/markdown.rs` (:1-240) — `format_markdown` starts `Instant::now()` at :43; `finish_ok` (:138-193) computes `duration_ms = started.elapsed().as_millis() as u64` (whole ms, :158) and emits `metric.snapshot.token_count_ms` (:178-187); `finish_phase_d` (:197-) emits the same target (:230). The span load and `curate()` run before this function is entered, so the graded value excludes them.
- `crates/snapshot/src/contract.rs` (:1-120) — re-exports `format_markdown` (:19); `curate` (:112) has its own `Instant` (:113).
- `pulse-app/src/snapshot_runtime.rs` (:95-310) — the two production orchestrators: `load_curated_markdown` (:137-146: load :140 → curate :142 → format :143) and the `snapshot.generate` resolver (:180-310: `spawn_blocking` load :198-206 → curate :208 → format :209 → md/json file writes → clipboard → notification → `snapshot.generate.request` INFO :297).
- `crates/mcp-server/src/tools.rs` (:284-300) — the third orchestrator, `dispatch_generate_snapshot`: load :289 → curate :290 → format :295.
- `pulse-app/tests/perf_budget_samples.rs` (:1-80, :140-215) — the CI producer; binds `127.0.0.1:0` (ephemeral, :142-146), opens no window, and generates its 50 snapshots through `pulse_app::snapshot_runtime::load_curated_markdown` (:209), never the resolver.
- `xtask/src/perf_budget.rs` (:1-60, :120-275) — `grade_arm(lines, arm)` (:137) sees EVERY log line, so a frame cause can be derived inside it with no signature change; `arm_lines` prints the fixed string `perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run` for any Neutral frame arm (:256-258); the pin `frame_absent_is_neutral` asserts that string (:394-401).
- `xtask/src/perf_frame.rs` (:1-90, :225-252) — the dev-host frame gate prints its own fixed `frame: 0 samples — no WebGPU adapter in this run` (:244), another unevidenced cause.
- `xtask/src/main.rs` (:117, :480-560, :810-830, :1250-1262) — `run_ci_gates` (:547-557) and `run_perf_load_profiles` (:818-828) call `grade` + `arm_lines` with no arm required; `EXPECTED_PROCEDURES` lists five `telemetry.frontend.*` pins (:1256-1260).
- `.github/workflows/ci.yml` (:23-160, :213-262, :411-486, :563-590) — lint-test Linux runs the producer + `perf:budget --require memory,snapshot` (:147-153); the `release` rust-cache step (:238-241) has no `cache-on-failure`; `boot` (:411-484) boots the REAL app under xvfb (WebKitGTK webview) and runs `cargo xtask ci-gates` over that log (:469-471); the release job's frame-boot comment (:256-259) states no frame boot runs there.
- `crates/ui-bridge/src/telemetry.rs` (:1-360) — the `telemetry.frontend` trait (:229-241, five methods), bounded input structs, `coerce_window_label` (:207-215), `record_ipc_rejection` WARN emit (:320-334) — the shape precedent.
- `pulse-app/src/observability.rs` (:960-985, :1040-1050, :1140-1160) — exact leaves for `app.boot.gpu.check` (`wgpu_backend` only), `metric.webgpu.frame_duration_ms`, `ui.ipc.rejection`, `metric.snapshot.token_count_ms` (`value, duration_ms, token_budget, time_range_minutes, token_count_actual, dedup_count, budget_exceeded`).
- `pulse-app/src/window.rs` (:86-101) — `app.boot.gpu.check` carries the compile-target default only; its message says "the frame loop's adapter branch is the adapter evidence" — which reaches no log today.
- `pulse-app/ui/src/canvas/webgpu-adapter.ts` (full) — `requestWebGPUAdapter()` returns `available` or `unavailable` with three reasons; `await navigator.gpu.requestAdapter()` (:50) is NOT wrapped — a rejection escapes as an unhandled promise and the consumer's `setAdapter` never runs (no fallback, no record).
- `pulse-app/ui/src/canvas/CanvasContainer.tsx`, `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (:140-160, :290-315), `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (:90-100, :236-270), `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.tsx` (:40-50, :128-150) — each consumer calls `requestWebGPUAdapter()` once per mount and renders the shared `<Fallback />` on `unavailable`.
- `pulse-app/ui/src/report/ipc-rejection.ts` (:1-60) — webview-side classifier + `currentWindowLabel()` via `sanitizeWindowLabel(getCurrentWebviewWindow().label)` + fire-and-forget reporter through the TauRPC proxy.
- `pulse-app/ui/tests-a11y/helpers/mock-tauri.ts` (:20-30, :80-140) — the `telemetry.frontend.*` mocks (:25-27, `null`); an unmocked procedure resolves `null`.
- `.andromeda/obs-plan.md` (:370, :628-629) — §5 snapshot row and §10 snapshot/frame rows, both recording the formatting-only span and naming this entry as owner; §4 P2 must-trace chain (:299): `snapshot.generate` → `duckdb.query.aggregation` → `snapshot.render.markdown` → `snapshot.token.count.validate`.
- Actions cache (live `gh api repos/Turbolet85/andromeds-pulse/actions/cache/usage` + `/actions/caches`, read 2026-09-30T16:2xZ) — 10 605 172 169 B in 8 entries (7 rust keys, all on lock hash `769a9503`, + gitleaks); cap 10 737 418 240 B; headroom 132 246 071 B (1.23 %).

## Graph impact (code-graph; editor lines = SCIP line + 1)
- **format_markdown** — production callers: `snapshot_runtime::load_curated_markdown` @ `pulse-app/src/snapshot_runtime.rs:143`, `SnapshotApiImpl::generate` @ `pulse-app/src/snapshot_runtime.rs:214`, `dispatch_generate_snapshot` @ `crates/mcp-server/src/tools.rs:295`; 15 in-crate tests in `crates/snapshot/src/markdown.rs` (:741-1027). Moving the metric emission OUT of `format_markdown` changes no signature, so none of these callers must change for compilation — the three orchestrators change only to time their stages.
- **curate** — the same three production callers (`snapshot_runtime.rs:142`, `:213`, `tools.rs:290`) + `pulse-app/tests/e2e_p2_snapshot_generate.rs:70,:140` + 3 in-crate tests.
- **load_curated_markdown** — callers `InvestigateApiImpl::run_action` @ `pulse-app/src/investigate_router.rs:219` and the producer @ `pulse-app/tests/perf_budget_samples.rs:209`. Timing inside this function therefore reaches the CI producer with no producer edit.
- **load_recent_spans** — `snapshot_runtime.rs:141`, `:208`; `tools.rs:289`.
- **requestWebGPUAdapter** (ts) — production callers `CanvasContainer` (`CanvasContainer.tsx:52`, no production render site — grep `<CanvasContainer` finds only its definition), `MetricsChart` (`MetricsChart.tsx:44`), dashboard `ConstellationCanvas` (`dashboard/routes/traces/ConstellationCanvas.tsx:96`), `HaloCanvas` (`halo/HaloCanvas.tsx:94`, orphaned per design-history 2026-08-21), widget `ConstellationCanvas` (`widget/ConstellationCanvas.tsx:149`); tests `webgpu-adapter.test.ts` (5 cases), `MetricsChart.test.tsx:52` (mocked). Recording inside `requestWebGPUAdapter` covers every live mount with one edit.
- **grade / grade_arm / arm_lines / read_family** (rust, `xtask/src/perf_budget.rs`) — callers `run_ci_gates` (`xtask/src/main.rs:548-549`), `run_perf_load_profiles` (`:819-820`), `run_perf_budget` (`perf_budget.rs:330-331`), `run_perf_frame_sample` (`perf_frame.rs:237-239`) + 13 pins. `grade_arm` keeps `(lines, arm)`; the frame cause is derived from `lines`, so no caller changes.

## Patterns detected
- **Webview-only fact → log via `telemetry.frontend.*`** (`crates/ui-bridge/src/telemetry.rs:229-241`, `pulse-app/ui/src/report/ipc-rejection.ts`): the webview classifies into a closed enum, a fire-and-forget proxy call crosses, the backend emits one bounded record behind its own exact leaf (`pulse-app/src/observability.rs:1045`).
- **Re-anchor an instrument, keep its target and leaf** (obs-history: `2026-09-29-p-025-hue-shift-observable-made-gradable`, `2026-09-30-p-027-discovery-bound`): the correction changes WHICH interval `duration_ms` spans and states the new anchor at the registry site; field set unchanged.
- **Neutral-with-reason** (`xtask/src/perf_budget.rs:147-153`): an empty arm carries a `reason` string; the frame arm alone overrides it with a fixed line (:256).
- **Owning cache keys save on failure** (`ci.yml:50`, `:434`, `:588`).

## Conventions to follow
- **pulse-app tests live in `pulse-app/tests/`** (`[lib] test = false`; tests-history 2026-08-14 / 2026-08-30): a new allowlist-leaf pin goes to a new `pulse-app/tests/unit_observability_allowlist_webgpu_adapter.rs`, mirroring the `ui.ipc.rejection` pin file.
- **Exact leaf per target** (obs-history 2026-08-21 / 2026-08-14): a `ui.*` target with no leaf has every field redacted (no bare `ui` key).
- **Closed-enum input + coerced label** (`telemetry.rs:207-215`, `:320-334`).
- **Whole-procedure-set gates** — a new procedure = router method + `EXPECTED_PROCEDURES` pin (`xtask/src/main.rs:1256-1260`) + regenerated bindings staged + the a11y mock entry (`mock-tauri.ts:25-27`); no capability JSON.

## Mechanism equalities (verified at HEAD)
- **Snapshot:** for the producer's input, `duration_ms` = the time from `format_markdown` entry to `finish_ok` (markdown.rs:43 → :158), so load (`load_recent_spans`, snapshot_runtime.rs:140) and `curate` (:142) are outside it — the equality "graded value = formatting only" holds at HEAD (unchanged since the predecessor measured 50 × 0 ms vs 61–76 ms). After the change the equality the criterion needs is: the graded `duration_ms` = time from before `load_recent_spans` to after `format_markdown` returns, for each of the 50 producer snapshots. The frame producing it will be the orchestrator (`load_curated_markdown`), which the producer calls at `perf_budget_samples.rs:209`.
- **Frame cause:** `perf:budget` over the lint-test producer log has NO webview in the run (the producer is in-process, perf_budget_samples.rs:1-12), so today's "no WebGPU adapter in this run" is a cause the run cannot contain; `ci-gates` over the boot job's log DOES have a real WebKitGTK webview (ci.yml:458-467). The criterion's equality: for a log with 0 `metric.webgpu.frame_duration_ms` records, the printed cause is a function of the adapter records in the SAME log — none → "no adapter record"; any `obtained` → "adapter obtained, 0 frames"; otherwise the non-obtained outcomes seen.
- **Adapter record reach:** the webview's `requestWebGPUAdapter()` runs once per canvas mount; the widget canvas mounts at boot (the compact widget is shown at boot). The record therefore lands in the boot log whenever the webview executes the canvas module — which the CI boot job exercises with no operator slot.

## New files to create
- `pulse-app/ui/src/canvas/adapter-state.ts` — closed adapter-outcome classifier + fire-and-forget reporter through `telemetry.frontend.record_webgpu_adapter`.
- `pulse-app/ui/src/canvas/adapter-state.test.ts` — vitest pins: each outcome, the reporter's fire-and-forget + swallowed rejection, no raw error text crossing.
- `pulse-app/tests/unit_observability_allowlist_webgpu_adapter.rs` — exact-leaf pin for the new target (field set by equality, banned-field redaction).
- `andromeda-pulse-0.3.0/chunks/2026-09-30-perf-instruments-measure-their-budgets/evidence/` — RED/GREEN readings, the boot-log adapter witness, the cache re-read.

## Files to modify
- `crates/snapshot/src/markdown.rs` — stop emitting `metric.snapshot.token_count_ms` from `finish_ok` / `finish_phase_d`; the `snapshot.render.markdown` span keeps its own fields.
- `crates/snapshot/src/contract.rs` — a generation-timing helper the three orchestrators share (start before load; finish after format emits `metric.snapshot.token_count_ms` with the same field set, fractional ms).
- `pulse-app/src/snapshot_runtime.rs` — `load_curated_markdown` and the `generate` resolver time load → curate → format through the helper.
- `crates/mcp-server/src/tools.rs` — `dispatch_generate_snapshot` times through the same helper.
- `crates/ui-bridge/src/telemetry.rs` — `WebgpuAdapterOutcome` closed enum + `WebgpuAdapterInput` + `record_webgpu_adapter` method (INFO on `obtained`, WARN otherwise) + its in-crate tests.
- `pulse-app/src/observability.rs` — exact leaf for the new target.
- `xtask/src/main.rs` — `EXPECTED_PROCEDURES` gains `telemetry.frontend.record_webgpu_adapter`.
- `xtask/src/perf_budget.rs` — frame Neutral cause derived from adapter records; per-cause pins; the old fixed-string pin rewritten.
- `xtask/src/perf_frame.rs` — its 0-sample line names the cause from the same derivation.
- `pulse-app/ui/src/canvas/webgpu-adapter.ts` — catch a rejecting `requestAdapter()` as its own reason; report the outcome through `adapter-state.ts` without awaiting it.
- `pulse-app/ui/src/canvas/webgpu-adapter.test.ts` — the rejecting-request case + reporting assertions.
- `pulse-app/ui/tests-a11y/helpers/mock-tauri.ts` — `telemetry.frontend.record_webgpu_adapter: null`.
- `.github/workflows/ci.yml` — `cache-on-failure: true` on the release rust-cache step.
- derived `pulse-app/ui/src/bindings/index.ts` by `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E test(emit_taurpc_bindings)` — the new procedure + its input types.

## Sweep records
- `token_count_ms` (`grep -rln token_count_ms` over `*.rs *.ts *.tsx *.toml *.yml *.sh *.ps1`, `target/` excluded): 5 files · 1 changed (`crates/snapshot/src/markdown.rs` — its two emit sites move to the new `contract.rs` helper) · 4 no-change (`crates/triage/src/digest/mod.rs:78` is a different target `metric.pipeline.l3.digest_token_count_ms`; `pulse-app/src/observability.rs:1148` and `pulse-app/tests/observability_pins.rs:383,1628-1732` hold/pin the leaf's field set, which is unchanged; `xtask/src/perf_budget.rs:49` keeps the same target name).
- `no WebGPU adapter in this run` over code (`--include=*.rs --include=*.yml`): 2 files · 2 changed (`perf_budget.rs`, `perf_frame.rs`); spec/leaf/evidence hits are wrap amendments or history, not touchpoints.
- `requestWebGPUAdapter` test mocks (`MetricsChart.test.tsx:17-52`): no change — the function's signature and result type stay.
- `TelemetryApiImpl` wiring (`pulse-app/src/main.rs:1017`, `:1044`): no change — a new trait method rides the already-merged router and the already-merged `emit_taurpc_bindings` handler.

## Open questions
- Should the per-frame-cause derivation treat a mixed log (one window `obtained`, another `no_navigator_gpu`) as "adapter obtained, 0 frames"? → blocks: plan-decision — resolved in P4 by lean: `obtained` anywhere wins, because frames need only one adapter.
- The in-process producer run (`--profile perf-samples`) launches no `pulse-app` binary and opens no window (ephemeral loopback port) — is it outside the operator's slot rule? → blocks: implementation-scope — stated in the plan for the operator to confirm at P5.
