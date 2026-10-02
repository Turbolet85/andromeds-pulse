# Report — 2026-09-30-perf-instruments-measure-their-budgets

**Chunk:** Perf instruments measure what their budgets name — the snapshot sample spans the whole generation; a frame-less run names its cause; release cache saves on failure
**Date:** 2026-09-30T19:51Z
**Commits:** `5fbf762 chore(2026-09-30-perf-instruments-measure-their-budgets): operator pre-CI commit` (the one commit since `last_wrap` 2026-09-30T15:57:10Z; parent `ea50ca2`)

## Changes (structured — detectors read this)
- **Files:** `crates/snapshot/src/contract.rs` · `crates/snapshot/src/markdown.rs` · `pulse-app/src/snapshot_runtime.rs` · `crates/mcp-server/src/tools.rs` · `xtask/src/perf_budget.rs` · `xtask/src/perf_frame.rs` · `xtask/src/main.rs` · `crates/ui-bridge/src/telemetry.rs` · `pulse-app/src/observability.rs` · NEW `pulse-app/tests/unit_observability_allowlist_webgpu_adapter.rs` · NEW `pulse-app/ui/src/canvas/adapter-state.ts` + `.test.ts` · `pulse-app/ui/src/canvas/webgpu-adapter.ts` + `.test.ts` · `pulse-app/ui/tests-a11y/helpers/mock-tauri.ts` · `.github/workflows/ci.yml` · REGENERATED `pulse-app/ui/src/bindings/index.ts` · chunk `evidence/{red-at-base,green-after,adapter-wire,ci}.md`. Basis: `git diff --name-only ea50ca2` + `gate.py scope` (clean, 17 changed · 17 listed · 0 recorded).
- **Symbols / APIs:**
  - NEW `snapshot::contract::GenerationTimer` (`start()` / `finish(&Result<MarkdownReport, FormatError>, &CurationOutput, TokenBudget)`) and `GenerationSample` (`new(Duration, …) -> Option<Self>` / `emit()`). The sole emitter of `metric.snapshot.token_count_ms` now. Field set unchanged (`value, duration_ms, token_budget, time_range_minutes, token_count_actual, dedup_count, budget_exceeded`). `value` and `duration_ms` are now FRACTIONAL ms `f64` (were whole-ms `u64`).
  - The interval is now start before the span load → end right after `format_markdown` returns. It covers load + curate + format and EXCLUDES the resolver's md/json writes, clipboard and notification.
  - `budget_exceeded = true` on `FormatError::BudgetExceeded`. `AnchorEncodingFailed` and a load/curate failure emit nothing.
  - Its three callers: `pulse_app::snapshot_runtime::load_curated_markdown`, the `snapshot.generate` resolver, and `mcp-server` `dispatch_generate_snapshot`.
  - CHANGED `snapshot::markdown::format_markdown`: no longer emits `metric.snapshot.token_count_ms` (both sites removed, in `finish_ok` and `finish_phase_d`). Its `snapshot.render.markdown` span still records its own `duration_ms` (the render sub-stage). `snapshot.token.count.validate` ERROR unchanged. Signature unchanged; its 3 production callers + 15 in-crate tests keep calling it.
  - NEW TauRPC procedure `telemetry.frontend.record_webgpu_adapter(input: WebgpuAdapterInput) -> Result<(), AppError>`, the 6th `TelemetryApi` method (roster 5 → 6).
    - `WebgpuAdapterInput { outcome: WebgpuAdapterOutcome, window_label: String }`. `WebgpuAdapterOutcome` is a closed serde `snake_case` enum: `obtained` · `no_navigator_gpu` · `adapter_null` · `adapter_request_rejected` · `device_request_failed`; an unknown value is rejected at deserialization.
    - The label is coerced by `coerce_window_label` (4 + `unknown`).
    - Emits ONE record per call on NEW target `ui.webgpu.adapter` {`outcome`, `window_label`}: INFO on `obtained`, WARN otherwise. Once per canvas mount, never per frame.
    - Founder ratification at P4 (Boundary widening, playbook `verdict: escalate`), verbatim: «Да, делай».
    - `pulse-app/src/main.rs` unchanged: `TelemetryApiImpl` was already merged in the router and in `emit_taurpc_bindings`.
  - NEW `xtask::perf_budget::frame_cause(lines) -> String`. `grade_arm(lines, Arm::Frame)` with 0 samples returns `Neutral { reason: frame_cause(lines) }`, derived from `ui.webgpu.adapter` records in the same log by `fields.outcome`:
    - none → `no adapter record in this log`;
    - any `obtained` → `adapter obtained but no frame recorded`;
    - else → `no WebGPU adapter ({distinct outcomes, sorted, comma-joined})`.
    - `arm_lines` prints `perf-budget: frame: cannot-evaluate: 0 samples, {cause}` for an unrequired Neutral frame arm. A REQUIRED empty frame arm prints `perf-budget: frame NEUTRAL — {cause} (required) FAIL` and FAILs.
    - The hardcoded `no WebGPU adapter in this run` is gone from `perf_budget.rs` and from `perf_frame.rs`, whose 0-sample line is now `perf:frame-sample: frame: 0 samples — {cause}`.
    - `grade_arm` signature unchanged; callers `run_ci_gates`, `run_perf_load_profiles`, `run_perf_budget`, `run_perf_frame_sample` unedited. Exit codes and the nearest-rank rule unchanged.
  - Webview: NEW `canvas/adapter-state.ts`: the `WebgpuAdapterOutcome` union, `outcomeForReason`, `currentWindowLabel()` (via `sanitizeWindowLabel`), and a fire-and-forget `reportAdapterOutcome` through the TauRPC proxy that swallows its own rejection.
  - `requestWebGPUAdapter()` now catches a REJECTING `navigator.gpu.requestAdapter()` as NEW reason `"requestAdapter rejected"`. Before, the rejection escaped unhandled and no fallback rendered. It reports its outcome once on every return path, without awaiting. Signature and `AdapterResult` shape unchanged. The existing consumers render the shared `<Fallback />` on any `unavailable`.
- **Crates / modules:** changed `snapshot`, `ui-bridge`, `mcp-server`, `pulse-app`, `xtask`; none added or removed.
- **Dependencies:** none (`git diff --name-only ea50ca2 -- Cargo.lock pulse-app/ui/package-lock.json` → no output).
- **Schema / config:** new allowlist exact leaf `ui.webgpu.adapter` → {`outcome`, `window_label`} in `AllowList::production()` beside `ui.ipc.rejection`. Still no bare `ui` key. `metric.snapshot.token_count_ms` leaf unchanged. `ci.yml`: the `release` job's `Swatinem/rust-cache` step (`shared-key: release-${{ runner.os }}`) gains `cache-on-failure: true` — the only ci.yml delta (`git diff --stat ea50ca2 -- .github/workflows/ci.yml` → `1 insertion(+)`).
- **Spec-master edits:** none this chunk (the seven masters are untouched at `5fbf762`).
- **Counts / qualifiers moved:**
  - `EXPECTED_PROCEDURES` 43 → 44 (awk over the array at `ea50ca2` / HEAD).
  - `telemetry.frontend.*` roster 5 → 6 (arch §Occupied Resources lines 176-178 state 5).
  - Workspace nextest 2466 → 2485 (entry `cargo nextest run --workspace --profile ci`: `2485 tests run: 2485 passed, 0 skipped`).
  - `xtask/src/perf_budget.rs` pins 15 → 21 (`grep -c '#\[test\]'`); `perf_frame.rs` 2 unchanged; `crates/ui-bridge/src/telemetry.rs` tests 35 → 40; `crates/snapshot/src/contract.rs` +5 `generation_timer_*`.
  - The snapshot p99 value: 0.0 ms → 94.0 ms (below).
  - The frame NEUTRAL text: fixed → cause-derived.
- **Dev-tool versions:** none.
- **Harness / gate surface:** the `perf:budget`, `ci-gates`, `perf:load-profiles` and `perf:frame-sample` frame lines now print a cause derived from the log (above). `ci.yml` release cache `cache-on-failure: true`. No new xtask verb, no new CI step or job, no agent-run change.
- **Cross-project / external claims:**
  - CI `ci#36765040464` measured sha `5fbf762`: `verdict: green · checks 13/13 · wall 1640 s` (`ci.py conclusion`).
  - Its two frame lines, verbatim: `boot smoke (ubuntu-22.04) … ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log` and `lint / test (ubuntu-22.04) … perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log`. The old fixed string appears 0× in the run log.
  - Actions cache (`gh api …/actions/cache/usage`): 10 605 172 169 B / 8 entries, identical to the P3 pre-change read; 132 246 071 B (1.23 %) under the 10 737 418 240 B cap; no key evicted. It stays a WATCH (relay).
  - `secret-scan#36765040450` success.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **"The CI boot job witnesses the adapter record."**
     - The claim: research.md §Mechanism equalities "Adapter record reach" ("the record therefore lands in the boot log whenever the webview executes the canvas module — which the CI boot job exercises"); plan.md §Test Commands "CI half" + entry `gh run view <id> --log | grep 'frame: cannot-evaluate'` note (predicted `no WebGPU adapter (no_navigator_gpu)`); scope.md §B "Where the frame line prints" ("`ci-gates` over the CI boot job's log (a real WebKitGTK webview under xvfb)").
     - MEASURED FALSE: the boot job's uploaded log (`logs-boot-Linux` of `ci#36765040464`) holds 23 records over 14 ms (`19:31:42.778 → .792`), all backend boot records, 0 webview-originated. The step runs `agent-run.sh boot` → `status` → `cleanup` back to back, so the app stops before the webview issues any IPC. The predecessor's boot log (`ci#36741143328`) has the same shape: 40 records / 0.7 s, 0 webview.
     - The printed cause (`no adapter record in this log`) is TRUE for that log. The ONLY live witness of the adapter record is the dev-host `perf:frame-sample` leg (2 `obtained` records).
     - Disposition: chunk-artifact claims → recorded here, no amendment owed (research/plan/scope are closed). Masters: none states it (grep `boot job` / `WebKitGTK` over the seven: the only hit, test-plan.md:647, describes the boot job's steps, not an adapter witness). Where obs-plan now describes the adapter record, the live witness must be named as the dev-host frame leg (relay).
  2. **"Suite health: `npx playwright test --list`."**
     - The claim: a11y-plan.md §3 line 381 ("Verify suite health cheaply with `npx playwright test --list`"); plan entry `cd pulse-app/ui && npx playwright test --list` (citing a11y-plan §3); leaf `.claude/rules/a11y.md` §Harness shape "Suite health" bullet.
     - MEASURED: the bare form exits 1 with `Error: No tests found … Total: 0 tests in 0 files` (it reads the deliberately inert default `playwright.config.ts`). It fails identically on the base, so it can never pass. The config-named form `npx playwright test --config=playwright-a11y.config.ts --list` → exit 0, `Total: 41 tests in 18 files`.
     - test-plan.md:56 already states the config-named form. Disposition: a11y-plan §3 amendment (master claim) + leaf re-derive; the plan entry is a chunk artifact → recorded here.
- **Expected amendments (from plan):**
  - obs-plan §5 snapshot row + §10 snapshot row (`duration_ms` spans load + curate + format; the timer's start/end; the measured p99) → carried (Symbols `GenerationTimer`; Outcome A). Sites: grep `formatting only|format_markdown only|times formatting` obs-plan 1 · test-plan 1; `metric.snapshot.token_count_ms` rows obs-plan.md:125 (§5 instruments) + :628 (§10 table).
  - obs-plan §10 frame row + CI gates + §1 multi-platform/perf-budget rows (frame line names its cause; "the app records no adapter-state event" retired) → carried (Symbols `frame_cause`). Sites: `no WebGPU adapter in this run` obs-plan 3 · architecture 1 · test-plan 2; `adapter-state|records no adapter` obs-plan 2.
  - obs-plan §6 warn row + §8 (`ui.webgpu.adapter` dual-site) → carried (Schema / config leaf; Symbols procedure). Sites: `ui.ipc.rejection` obs-plan 2 (the sibling registration pattern).
  - security-plan §Input Validation TauRPC row (`WebgpuAdapterOutcome` closed enum + coerced label) + §Security Anti-Patterns → Logging (new bounded webview-originated record), quoting the founder's «Да, делай» → carried (Symbols procedure). Sites: `record_ipc_rejection` security-plan 1 · `IpcRejectionCategory` security-plan 1 · `ui.ipc.rejection` security-plan 2.
  - architecture §Occupied Resources → Tauri IPC routes (`telemetry.frontend.*` roster 5 → 6, ratification cited) + xtask CLI surfaces (the frame lines name a cause) + §Infrastructure Patterns → CI/CD (`release-{os}` saves on failure; the post-change cache re-read) → carried (Counts; Harness; Schema). Sites: `telemetry.frontend` architecture 3 · `release-${{ runner.os }}` architecture 1 · `no WebGPU adapter in this run` architecture 1.
  - test-plan §1 `perf-slo-check-arm-coverage` row ("snapshot times formatting only" clause retired) + §3 (the procedure-changing bindings close, specified by the plan's Order note) + §9 release row (`cache-on-failure`) → carried (Symbols; Harness; Decisions). Sites: `perf-slo-check-arm-coverage` test-plan 1 · `cache-on-failure` test-plan 3 · `release-${{ runner.os }}` test-plan 1.
- **Coverage of new surfaces:**
  - `telemetry.frontend.record_webgpu_adapter` → validation serde closed enum + `coerce_window_label`✓ · instrumentation `ui.webgpu.adapter` log✓ · PII redacted✓ (2 bounded fields; exact leaf; raw error text never crosses; out-of-set label lands `unknown`) · tests unit (ui-bridge `webgpu_adapter` ×5) + integ (`unit_observability_allowlist_webgpu_adapter` ×3) + e2e live wire (dev-host frame leg, 2 records) · a11y n/a · tokens n/a
  - `snapshot::contract::GenerationTimer` / `GenerationSample` → validation n/a · instrumentation `metric.snapshot.token_count_ms`✓ · PII n/a (aggregate numerics + bounded budget label) · tests unit (`generation_timer_*` ×5, incl. a one-emitter-per-generation pin with a capturing subscriber) + integ (`perf_budget_samples` producer → `perf:budget`) · a11y n/a · tokens n/a
  - `xtask::perf_budget::frame_cause` → validation n/a · instrumentation n/a (reads logs) · PII n/a · tests unit (`frame_cause_*` ×6 + the rewritten `frame_absent_is_neutral`) + the `ci-gates` seeded probe · a11y n/a · tokens n/a
  - webview `adapter-state.ts` reporter + `requestAdapter rejected` reason → validation n/a (closed union) · instrumentation via the procedure above✓ · PII n/a (only the closed outcome + sanitized label; pinned) · tests unit vitest (`adapter-state.test.ts` 9; `webgpu-adapter.test.ts` +6) · a11y n/a (no new element; the existing `<Fallback />` now also renders on a rejected request; `cargo xtask test:a11y` 0 new tuples) · tokens n/a

## Deviations from intent
1. **Plan entry `cd pulse-app/ui && npx playwright test --list` is red by construction.** It exits 1, `Total: 0 tests in 0 files`, because it reads the inert default config, and it fails the same on the base. The config-named form (`--config=playwright-a11y.config.ts --list`) was run beside it: exit 0, 41 tests in 18 files. Recorded, not "fixed" in the plan (plan.md is immutable). See Spec claims disproved 2.
2. **Plan gate order left the live leg's binary without the new procedure.** Entry `npm run build --prefix pulse-app/ui` precedes the bindings regen entry. The taurpc proxy resolves methods against the `ARGS_MAP` BUNDLED into `ui/dist`, so the first release build (`cargo build -p pulse-app --release`) embedded a dist without `record_webgpu_adapter`: 1 occurrence in `ui/dist/assets/*.js`, the caller's own property access. The fire-and-forget reporter would have returned silently. At /implement the two entries were re-run AFTER the regen, before any window opened. dist and binary then carried 2 occurrences each (map + caller), and the live leg recorded 2 `obtained` records.
3. **One case the plan left open:** `FormatError::AnchorEncodingFailed` (never produced by `format_markdown` today) yields no sample, matching today's no-emission on it. The plan named only the Ok and BudgetExceeded arms.
4. **The operator pass made one pre-CI commit** (`5fbf762`, the whole tree, as the predecessor's did) so the guarded push entry's clean-tree guard could hold. That was on the overseer's word ("Run the OPERATOR PASS on my word, entries 37 to 42").

scope record: none — `gate.py scope` clean (changed 17 · listed 17 · recorded 0), at /implement P4 and at this wrap's P1.

## Decisions & corrections
- **Founder ratification (P4, before this session):** «Да, делай» for the new `telemetry.frontend.record_webgpu_adapter` procedure in the plan's exact shape.
- **Operator rulings this session:**
  - Window-opening runs (`self-verify`, `perf:frame-sample`) are the overseer's slots; granted once ("4317/4318 free, no nvda/pulse-app/conductor running; conductor-builder is held"). The in-process producer runs freely.
  - The 6 `msedgewebview2` processes left at /implement's census are SearchHost-owned (overseer, measured since 2026-09-26).
  - No live leg re-fires in this wrap without the overseer's slot.
- **Relay directives for this wrap** (`pc-overseer/relays/pulse-wrap-perfinst-2026-09-30.md`):
  - State the false prediction as false.
  - Name the dev-host frame leg as the only live adapter witness in obs-plan.
  - Keep the cache as a watch.
  - Quote «Да, делай» wherever the procedure is recorded (arch, security, obs).
  - Carry the entry-18 lesson where the bare form is taught.
  - Carry the regen-before-UI-build-and-release-build ordering into the playbook's bindings-order rule, not a new rule elsewhere.
  - No new route entry; the tail stays Span-level redaction → Real-model incident surfacing → Conductor return.
- **The playbook bindings rule's open scope is now specified in practice.** The rule said "a chunk that changes the procedure set needs a close reading the new shape, not yet specified — escalate that case until one is". This chunk ran one: workspace nextest → mcp-feature regen → content probes (`grep -c record_webgpu_adapter` ≥ 1, `grep -c '"mcp":'` ≥ 1) → `git add` bindings + `xtask/src/main.rs` → `check:staged-artifacts` → `capability-drift` LAST, all exit 0. Measured addition: the regen must ALSO precede the `ui/dist` build and every release build a live leg uses (Deviation 2).
- **Measured, reusable:**
  - The CI boot job never runs the webview long enough to issue IPC (23 records / 14 ms), so no CI job can witness a webview-originated record today.
  - A python process writing to a file block-buffers: the gate tool's background output file stayed EMPTY for 30 min while entries completed. The per-entry logs under `%TEMP%/andromeda-gate/{marker}/{run}/` were the live progress record. This is the output-appearance family, testing.md 2026-08-25.
- **Sweep hazards:**
  - `no WebGPU adapter` as a grep ALSO matches the new cause text `no WebGPU adapter ({outcomes})`. Sweep the retired string by its full form `no WebGPU adapter in this run`.
  - `record_webgpu_adapter` over a minified bundle: 1 hit = the caller's property access only, 2 = the ARGS_MAP entry present. Presence is not the proof; the count is (testing.md 2026-08-30 ARGS_MAP entry).

## Outcome
- **(obs) A — MET.** On the producer log the snapshot p99 is `94.0 ms <= 500 ms (n=50) PASS`, no longer `0.0 ms` (base). Samples: 50 fractional-ms values, min 62.97 · median 67.99 · max 94.02 ms (debug build), matching the predecessor's hand-measured 61–76 ms full generation (`evidence/green-after.md`).
- **(obs) One emitter — MET.** `generation_timer_is_the_only_metric_emitter_across_a_whole_generation` (1 record per curate + format + finish; a still-emitting formatter would read 2). The three orchestrators are timed. Field set + leaf unchanged (`binary(observability_pins)` green).
- **(obs) Frame cause — MET.** The 6 `frame_cause_*` pins. `ci-gates` over a seeded sample-less log prints `… 0 samples, no adapter record in this log` and stays NEUTRAL. A required empty frame arm still FAILs (pin).
- **(obs + security) One record per adapter request — MET, live on the dev host.** 2 `obtained` INFO records (`compact-widget`, `main`), fields unredacted and nothing else. The WARN path is unit-pinned per outcome.
- **(security) Validated input + pin — MET.** An unknown outcome is rejected (pin). `EXPECTED_PROCEDURES` 44. `check:staged-artifacts` and `capability-drift` exit 0 with the staged bindings. Capability JSONs byte-identical (the diff lists none).
- **(design + layouts + a11y) — MET.** No `.tsx` changed (entry `git diff --name-only ea50ca2 -- pulse-app/ui/src | grep -c '\.tsx$'` → 0). `cargo xtask test:a11y` 0 new violation tuples (run twice: before and after the dist rebuild). The suite collects 41 tests under the config-named `--list`. The listed bare form is red by construction (Deviation 1).
- **(tests) Release cache — MET.** `cache-on-failure: true` on the release step; the ci.yml delta vs `ea50ca2` is that one line. Post-change cache usage recorded in `evidence/ci.md`.
- **(tests) Standard gates + CI — MET.** CI `ci#36765040464` green on `5fbf762`; its lint-test `perf:budget` frame line names the no-record cause.
- **Matrix:** no capability claimed (`matrix.py show --chunk` → `claimed … 0`; pool P-075 unclaimed, outside this chunk).

**Gates (from /implement run `2026-09-30T18-06-28Z`, by `run`):**
- green: `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- green, 19 passed / 2466 skipped: the new-pin selector `test(/frame_cause/) | test(/generation_timer/) | test(/webgpu_adapter/)`.
- green: the grader/frame/observability_pins selector.
- green, `1 test run: 1 passed`: `cargo nextest run --workspace --profile perf-samples`.
- green, all 5 atoms: `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot`.
- green: the seeded `ci-gates` probe · the `cache-on-failure` probe (last line 1) · the ci.yml unpinned-`uses:` probe (exit 1, no output) · the ci.yml `--stat` probe (1 insertion, no deletion) · the `.tsx` probe (0) · the `WEBVIEW2` probe (exit 1, no output).
- green: `npm run lint` · `npm run typecheck` · `npm run test` · `npx vitest run src/canvas/adapter-state.test.ts` · `npm run build`.
- **red · exit 0 expected, exit 1 read:** `cd pulse-app/ui && npx playwright test --list`. Red by construction on base and head; the config-named form is green (Deviation 1). This red is the PLAN FORM's, not a regression. Owner: the a11y-plan §3 amendment + the leaf fix this wrap.
- green: `cargo xtask test:a11y` · `cargo deny check bans licenses sources` · the lockfile probe (no output) · `cargo xtask capability-widening-check` · `cargo xtask check:ingest-progress`.
- green, 2485/2485: `cargo nextest run --workspace --profile ci`.
- green: `cargo build -p pulse-app --release` (re-run after the regen, Deviation 2) · `cargo build --release -p ingest --example inject_demo`.
- green, operator slot: `cargo xtask self-verify` (PASS; 264 log lines, 0 panics, clean quit).
- green: the port precondition probe.
- leg live, driven by hand in the operator slot: `cargo xtask perf:frame-sample` → exit 0, `perf:frame-sample: PASS`, frame p99 2.7 ms (n=8045).
- green, count 2: the adapter-record grep over `target/perf-frame/`.
- green: the mcp-feature bindings regen · `grep -c record_webgpu_adapter` (2) · `grep -c '"mcp":'` (1) · `git add` bindings + `xtask/src/main.rs` · `cargo xtask check:staged-artifacts` (staged-clean) · `cargo xtask capability-drift` (green).

**Operator pass, `leg = 'operator'`, fired on the overseer's word (`evidence/ci.md`):**
- `gate.py hygiene` → `hygiene: clean`.
- `cargo xtask pre-push:linux` → `"verdict": "green"`.
- the guarded push → `ea50ca2..5fbf762`.
- `ci.py conclusion --sha HEAD --wait 2400` → `verdict: green` (`ci#36765040464`).
- `gh run view <id> --log | grep 'frame: cannot-evaluate'` → both lines `… no adapter record in this log`; the old string absent.
- `gh api …/actions/cache/usage` → recorded, 10 605 172 169 B / 8 entries.

**Smoke:** the boot-path change (ui-bridge + observability.rs) is covered by `self-verify` on the re-embedded release binary.

**Watches:** the Actions cache headroom (carried from the predecessor) · 1 green round read [`ci#36765040464`]: 1.23 % headroom, byte-identical to the pre-change read, no eviction. It stays a watch.

**Outcome basis:** /implement's P4 report (this conversation) + the operator pass's final state (Setup 4: one pre-CI commit `5fbf762`; the final HEAD's CI run `ci#36765040464` recorded in `evidence/ci.md`) + post-implement artifacts `evidence/ci.md` and the downloaded `logs-boot-Linux` artifacts of `ci#36765040464` / `ci#36741143328` (session scratchpad, not committed).

**Process hygiene:**
- /implement's census: gate-launched cargo/nextest/npm/playwright ended; `pulse-app` pid 38788 (self-verify) clean quit; the frame leg's `pulse-app` + `inject_demo` stopped by pid; 0 repo-owned processes after.
- The 6 `msedgewebview2` are SearchHost-owned (overseer).
- The operator pass started `pre-push:linux` (WSL, no `pulse-app`; ended) and `ci.py` / `gh` (ended).
- Re-measured at this wrap's P1: `:4317` / `:4318` refuse (10061, 10061). 2 live cargo processes are this wrap's own background code-graph refresh.
