# Codebase Research — 2026-09-30-perf-budget-gate-reads-real-samples

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 24 · **CI reads (gh):** 9 (run 36710506171 jobs/steps/logs, the `logs-boot-Linux` artifact, cache list + usage)
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full as a structural extraction: 151 lines in two offset-bounded reads covering 1–128 and 129–151, 21 Session Additions. Applied: the 5-command discipline and "no 6th verb", the `status` `ended` grammar, the PID-file lifecycle, "scenario legs are NOT gates", window ≥ threshold (2026-08-28), a harness's negative finding needs a second source (2026-08-29), one exported data dir per leg (2026-08-23), glob the `agent-latest.jsonl*` family (2026-06-29), and the child-cargo package-variable trap (2026-09-29).
- **Platform issues consulted:** none. No runner-only bullet was folded, because Setup 5a read `fb93fca` green. The CI facts below come from this repository's own recorded run, not from a platform tracker.

## Files inspected
- `xtask/src/main.rs` (454–545, 700–745, 800–845) — `run_ci_gates` (`:454`) prints `perf-budget PASS` when `perf-slo-check` exits 0. `invoke_perf_slo_check` (`:705`) hands the script ONE file, `log_files.last()` (`:724-726`). The no-log branch prints `perf-budget DEFERRED (no criterion bench yet)` (`:465`), a residue of the histogram/criterion framing that obs-plan's 2026-05-02 entry retired. `run_perf_load_profiles` (`:803`, `:839`) is the second caller of both `collect_log_files` and `invoke_perf_slo_check`.
- `xtask/ci/perf-slo-check.sh` (full, 80 lines) and `.ps1` (108 lines) — three arms: frame p99 ≤ 33, memory max ≤ 512 000 000, snapshot p99 ≤ 500. Each arm maps an empty stream to NEUTRAL, and the script exits 0 when all three are NEUTRAL. The sh arms need `jq`; with no `jq` the script reads NEUTRAL (`:25-28`).
- `.github/workflows/ci.yml` (1–22, 194–239, 388–462) — `ci-gates` runs only in `boot` (`:451-452`), after a three-verb xvfb smoke (`:437-447`). `release` restores `lint-test-${{ runner.os }}` with `save-if: false` (`:217-222`). The header comment (`:18-21`) states the cache budget.
- `pulse-app/src/heartbeat.rs` (120–225, 300–340) — `run_buffer` uses `tokio::time::interval(15 s)`. Its first tick fires immediately and emits `buffer.tick` plus `metric.buffer.memory_bytes {value, …}` (`:329`).
- `pulse-app/ui/src/canvas/CanvasContainer.tsx` (1–140), `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (180–300) and `pulse-app/ui/src/canvas/webgpu-adapter.ts` (40–55) — frame samples are recorded only inside a frame loop created AFTER `requestWebGPUAdapter()` returns `available`. `navigator.gpu` undefined or a null adapter renders `<Fallback />` with no loop and no sample.
- `crates/snapshot/src/markdown.rs` (39–240) — `format_markdown` emits `metric.snapshot.token_count_ms {value, duration_ms, …}` at `finish_ok` (`:179`) and at `finish_phase_d` (`:230`).
- `pulse-app/src/snapshot_runtime.rs` (1–220) — `snapshot.generate` (`:183`) and `load_curated_markdown` (`:138`, shared with `investigate.run_action`) are the in-app callers. Both are started by the webview.
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (`:53`), `crates/mcp-server/src/tools.rs` (284–307) and `crates/mcp-server/src/tracing_setup.rs` (40–120, 248–285) — the sidecar's `generate_snapshot` runs over the sidecar's OWN `Connection::open_in_memory()`, which is empty. Its allowlist has no `metric.snapshot.*` leaf, and no bare `metric` key either (`for_target` returns `None`).
- `pulse-app/src/observability.rs` (855–870, 965–980, 1140–1160, 2609) — the app allowlist has EXACT leaves carrying the graded fields: `metric.buffer.memory_bytes {value, …}`, `metric.webgpu.frame_duration_ms {duration_ms, …}` and `metric.snapshot.token_count_ms {value, duration_ms, …}`. `pub fn init(data_dir) -> WorkerGuard` (`:2609`) is callable from `pulse-app/tests/`.
- `pulse-app/tests/perf_slo_10k_spans.rs` (1–60) — this is the existing CI perf test: in-process loopback gRPC receiver, real DuckDB and `run_consumer`. It asserts throughput only, installs no JSON sink and emits no perf samples.
- `scripts/agent-run.sh` (60–135) — the waiting-subshell recorder writes `run/andromeda-pulse.{spawn,exit}` with `exit N` or `signal N (NAME)`.
- `scripts/agent-run.ps1` (100–200) — `boot` spawns the app with `Start-Process -PassThru -NoNewWindow` (`:150`) and writes `$proc.Id` to the pidfile. It writes no `.spawn` or `.exit`.
- `xtask/src/harness_status.rs` (190–225, 355–375) — `end_file(pidfile)` is `pidfile.with_file_name("andromeda-pulse.exit")` (`:200`). `read_ended` (`:205`) trims and accepts ≤ 48 chars of ASCII graphic or space, and is pinned at `:359-371`.
- `xtask/src/pre_push.rs` (45, 209, 249, 468) — the local `pre-push:linux` seeds ONE `app.boot.ready` record and runs `cargo xtask ci-gates`, which must stay green.
- `pulse-app/tests/quality_gate_workflow.rs` and `pulse-app/tests/a11y_perf_workflow.rs` — workflow self-lints. They pin `cargo xtask ci-gates` (`:211`), `cargo xtask perf:slo-load` (`:34`) and the no-`continue-on-error` rule on gate steps (`:147-181`), so any `ci.yml` edit rides them.
- `.config/nextest.toml` (1–28) — `[profile.default]` excludes `binary(perf_load_profiles)`, and `[profile.load-profiles]` selects it.

## CI measurements (re-derived at the chunk base, not copied from the CONTEXT)
- **The boot-smoke log.** From `gh run download 36710506171 -n logs-boot-Linux`, parsed by target:
  - The log holds 23 records spanning 14 ms (`11:55:49.542Z → .556Z`), with 0 `metric.webgpu.frame_duration_ms`, 0 `metric.buffer.memory_bytes`, 0 `metric.snapshot.token_count_ms` and 0 `buffer.tick`.
  - `app.boot.gpu.check {gpu_available: false, wgpu_backend: vulkan}` · `app.boot.webview.init {webview_backend: GTKWebKit}` · `corpus.open.error {error_kind: KeyringUnavailable}`.
  - The CONTEXT's figure was 52 records. It re-derives to 23 at this run, a different run of the same shape. The 0-sample mechanism holds.
- **The job log** (`gh api …/jobs/109870888182/logs`):
  - `boot: ready` at `11:55:49.61`, `cleanup: clean` at `11:55:51.13`, so the app lived about 1.5 s.
  - `perf-slo-check` printed three NEUTRAL lines, then `ci-gates: perf-budget PASS` (log `:1591-1594`).
  - The boot pre-build inside `agent-run.sh boot` took 3 m 51 s (`11:51:58 → 11:55:49`) after step 12's `--features mcp-server` build.
- **Local comparison** (`target/discovery/2026-09-30T10-34-38Z/agent-latest.jsonl.2026-09-30`, parsed):
  - `buffer.tick` and `metric.buffer.memory_bytes` land 1.30 s after `app.boot.tracing.init`, and the first frame sample 0.93 s after it (GPU host, `webview2`/`vulkan`).
  - A 1.5 s CI lifetime therefore sits at the edge of the first memory tick. In the CI run the log shows nothing after boot setup at all.
- **Round shape** (run 36710506171, `gh run view --json jobs`):
  - The critical path is coverage at 21 m 37 s.
  - lint-test Windows ran 17 m 42 s. release Windows 12 m 53 s, with its build step at 9 m 37 s. release macOS 9 m 19 s. boot 10 m 18 s, ending about 11 min before the round ends.
- **Release warmth** (`gh api …/jobs/109870887972/logs`):
  - `Restored from cache key "v0-rust-lint-test-Windows-Windows_NT-x64-15d035cd-769a9503" full match: true`.
  - `grep -c Compiling` = 16, exactly the 16 workspace members. Every dependency came from the cache, and the build still took 9 m 35 s.
- **Cache inventory** (`gh api …/actions/cache/usage` + `gh cache list`):
  - 8 576 022 877 B across 6 entries: lint-test-Windows 2 551 MB · lint-test-macOS 2 067 MB · lint-test-Linux 1 718 MB · boot-Linux 1 646 MB · coverage 188 MB · gitleaks 5 MB.
  - All sit on `refs/pull/39/merge`. The CARRY's 8.58 GB re-derives exactly.

## P4 frame-adapter probes (operator slots granted 2026-09-30; dev host, release binary)
- **Binary provenance:** `target/release/pulse-app.exe`, sha256 `9e51d1d92e80fdc0b998fe5e1c65fbbd9c5eef4dd9e6b7c4883c5ccad1bf9ab4`. That is the 87fe658 GREEN-leg binary the handoff records, and `git diff --stat 87fe658 fb93fca` over the product paths is empty. Each launch ran by path on a fresh scratchpad data dir, with OTLP ports `14317`/`14318` and teardown by the pidfile pid through `Stop-Process`. Every launch read `app gone` and `ports free` afterwards. Script: scratchpad `frame_probe.sh`, parsing the whole `agent-latest.jsonl*` family.
- **Control** (same binary, no flags; `target/discovery/2026-09-30T10-34-38Z`): 578 frame samples, labels `vulkan`/`webview2`.
- **Probe 1:** `--use-webgpu-adapter=swiftshader --enable-unsafe-swiftshader` → **0 frame samples** in 40 s (855 records, 3 memory samples, 0 ERROR).
- **Probe 2:** `--disable-gpu` → **0 frame samples** in 40 s (873 records, 3 memory samples, 0 ERROR).
- **Probe 3:** `--enable-unsafe-webgpu --enable-features=Vulkan --use-vulkan=swiftshader --use-webgpu-adapter=swiftshader` → **496 frame samples** in 2 s (the poll stops at 300). p50 0.3 ms · p99 4.1 ms · max 14.6 ms; labels `vulkan`/`webview2` (496/496); 0 ERROR; 568 records.
- **Limit of that reading:** this host HAS a GPU, and the frame label reads `vulkan` in the control and in probe 3 alike, so the log cannot prove the adapter was SwiftShader rather than the hardware GPU. The inference that it was rests on probes 1 and 3 together: requesting the SwiftShader adapter without the Vulkan-SwiftShader backend REMOVED the adapter (0 frames), and adding that backend restored one. The discriminating measurement is a GPU-less hosted runner, where any adapter is software by construction.
- **Finding: `app.boot.gpu.check` is a constant, not a probe.** `pulse-app/src/window.rs:91-96` emits `gpu_available = false` unconditionally, and `wgpu_backend` comes from `detect_wgpu_backend()` (`:76-84`), a `cfg!(target_os)` switch (Windows `dx12`, macOS `metal`, otherwise `vulkan`). The message calls it "boot-time pre-render; runtime adapter check at chunk #28". It read `false` on the 578-frame control and on the CI runner alike, so no reader may treat it as adapter evidence. The frame loop's own adapter branch (`widget/ConstellationCanvas.tsx`) is the only true source of adapter availability.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-09-30-perf-budget-gate-reads-real-samples.json`)
- **invoke_perf_slo_check** — 2 callers: `run_ci_gates` @ `xtask/src/main.rs:531` and `run_perf_load_profiles` @ `:839`. A signature change, for example passing every family member or a per-arm result, threads through both.
- **collect_log_files** — 3 callers: `run_ci_gates` (`:455`), `run_check_ingest_progress` (`:550`) and `run_perf_load_profiles` (`:803`).
- **run_ci_gates** — 1 caller, `main` (`:275`). The CLI dispatch is `Cmd::CiGates`.
- **read_ended / end_file** — callers are `harness_status::run` (`:45`) and the pin `read_ended_takes_one_bounded_line_beside_the_pidfile` (`:360-371`). The ps1 recorder changes no Rust signature.
- probe_hits: every name > 0, so these are genuine call sets. Lines are cited editor-form (graph line + 1).

## Patterns detected
- **Self-proving scenario leg** (`xtask/src/discovery.rs`, `xtask/src/hue_shift.rs`; verification-harness.md §Scenario legs) — exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE, a precondition that cannot be borrowed, window ≥ threshold, and an artifact under `target/<leg>/`. They are dev-host scenario legs, NOT gates.
- **Cannot-evaluate is its own arm** (`xtask/src/staged_gate.rs`; `check:npm-supply-chain`) — exit 2 is never a pass. The staged gate was folded into capability-drift's 0/1 failure exit (arch history, 2026-08-30-staged-bindings-assertion).
- **Pure classifier + per-arm pins** (`xtask/src/harness_status.rs::classify` + `read_ended` pins) — the form for a per-arm perf verdict.
- **In-process real-pipeline perf test** (`pulse-app/tests/perf_slo_10k_spans.rs`) — real loopback gRPC + real DuckDB + real consumer. It is the precedent for driving production code under load without a window.

## Conventions to follow
- **Gate steps carry no `continue-on-error`** (`pulse-app/tests/quality_gate_workflow.rs:147-181`), and every job keeps the `ANDROMEDA_PULSE_DATA_DIR` `$GITHUB_ENV` export right after harden-runner (`ci.yml:204-206`).
- **Log readers glob the family** (`collect_log_files`; verification-harness.md 2026-06-29). `invoke_perf_slo_check` currently grades ONE member (`:724`).
- **pulse-app tests live in `pulse-app/tests/`** (the flat-zero lib-src ratchet). Narrow runs use `--workspace -E`, never `-p`.
- **Every cargo child xtask spawns uses `cargo_command()`** (verification-harness.md 2026-09-29).

## Mechanism equalities re-derived (for P4's citations)
- **CI perf gate = PASS for a log with 0 samples.** When every arm stream is empty, `perf-slo-check.sh` exits 0 and `run_ci_gates` prints PASS (`main.rs:531-532`). Observed live at `ci#36710506171` job log `:1591-1594`. This is the vacuity.
- **Memory sample ⇐ app alive past the first buffer tick.** The first tick fires at interval start. Measured locally at +1.30 s after tracing init; the CI app lived about 1.5 s and its log carries none. A leg that keeps the app up for ≥ 2 ticks (> 15 s) yields ≥ 2 samples by construction.
- **Frame sample ⇐ a WebGPU adapter in the webview.** The recorder sits inside the adapter-`available` branch (`widget/ConstellationCanvas.tsx:255-285`). `[corrected at P4]` The CI runner's `gpu_available: false` is NOT evidence: that record is a hardcoded constant (§P4 frame-adapter probes). What the CI Linux run does show is `GTKWebKit` plus a 1.5 s lifetime, which confounds any absence. On the dev host, WebView2 exposed an adapter only under the Vulkan-SwiftShader flag set (probe 3). Whether a hosted Windows runner's WebView2 exposes one under that set is UNMEASURED until a CI run reads it.
- **Snapshot sample ⇐ `format_markdown` running inside a process whose sink carries the leaf.** In the shipped binary the only callers are webview-started. The sidecar's copy runs over an empty private DB with no leaf. So no existing CI run can produce a snapshot sample from the product. The obs-plan §10 snapshot row names "`xtask test` span aggregation", a test-layer producer, as the enforcement point.
- **ps1 `ended` ⇐ a `.exit` record beside the pidfile that `read_ended` accepts.** A Windows exit such as `exit 3221225477` (an NTSTATUS crash code, 15 chars) is inside the grammar. `signal N` has no Windows analogue.

## New files to create
- `pulse-app/tests/perf_budget_samples.rs` — the in-process sample producer, if P4 takes the test-layer producer: the real `observability::init` sink, real loopback ingest under load, the real `heartbeat::emit_buffer_tick`, and `snapshot::curate` + `format_markdown` at the 25k budget, over a data dir `ci-gates` then grades.

## Files to modify
- `xtask/src/main.rs` — `run_ci_gates` gains the vacuity guard and the all-family-members read; `invoke_perf_slo_check` threads through both callers; the `:465` DEFERRED line is retired.
- `xtask/ci/perf-slo-check.sh` — the per-arm verdict, if the arms stay in shell.
- `xtask/ci/perf-slo-check.ps1` — the same, in lockstep.
- `.github/workflows/ci.yml` — the sample-producing step and its gate, the release job's cache allocation, and the log artifact for the new data dir.
- `scripts/agent-run.ps1` — the boot end-status recorder mirror.
- `pulse-app/tests/quality_gate_workflow.rs` — self-lint pins for the new or changed ci.yml steps and the release cache key.
- `xtask/src/pre_push.rs` — only if the guard's opt-in reaches `pre-push:linux`'s `ci-gates` stage.
- `.config/nextest.toml` — only if the sample producer needs its own profile or filter, so the default suite does not run it twice.

## Open questions
- Frame arm in CI: no hosted runner has a GPU (Linux measured `gpu_available: false`; Windows WebView2 unmeasured). Options: gate frames on a software WebGPU adapter on a Windows runner, report the frame arm as a named cannot-evaluate in CI with the frame budget gated where a GPU exists, or something else → blocks: plan-decision.
- Producer for the memory and snapshot samples: an in-process test in an existing job (no window, no operator slot, spec-sanctioned for snapshot) vs a real-binary leg in `boot` (faithful for memory; snapshot unreachable without a driver) → blocks: plan-decision.
- Release cache allocation within 10 GB: its own `release-{os}` key (sizes unmeasured; could exceed the cap and cause LRU churn) vs `lint-test` building the release dep set (adds time near the second-longest job) vs dropping or narrowing the job. Measured: a release cache miss would NOT lengthen today's round, because coverage at 21.6 min dominates a cold release job of about 13 + 3.3 min. It costs runner minutes and headroom → blocks: plan-decision.
