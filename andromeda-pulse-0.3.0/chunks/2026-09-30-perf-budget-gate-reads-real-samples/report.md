# Report — 2026-09-30-perf-budget-gate-reads-real-samples

**Chunk:** Perf-budget gate reads real samples — CI perf gate fed a sample-bearing log so it can fail; release-job cache; Windows boot end-status recorder
**Date:** 2026-09-30T15:40Z
**Commits:** since `last_wrap` (2026-09-30T11:41:40Z, `fb93fca`): `d708ad7` · `c6eb395` — both `chore(2026-09-30-perf-budget-gate-reads-real-samples): operator pre-CI commit`

## Changes (structured — detectors read this)

- **Files** (basis: `git diff --name-status fb93fca -- .`, product/CI/harness paths, 18):
  - new: `xtask/src/perf_budget.rs` · `xtask/src/perf_frame.rs` · `pulse-app/tests/perf_budget_samples.rs`
  - deleted: `xtask/ci/perf-slo-check.sh` · `xtask/ci/perf-slo-check.ps1`
  - modified: `xtask/src/main.rs` · `xtask/src/smoke.rs` · `pulse-app/src/snapshot_runtime.rs` ·
    `pulse-app/src/window.rs` · `pulse-app/src/observability.rs` · `pulse-app/tests/observability_pins.rs` ·
    `pulse-app/tests/quality_gate_workflow.rs` · `pulse-app/tests/perf_slo_10k_spans.rs` (doc comment) ·
    `crates/ingest/examples/load_profiles.rs` (doc comment) · `.config/nextest.toml` · `.github/workflows/ci.yml` ·
    `scripts/agent-run.ps1`
- **Symbols / APIs:**
  - `xtask::perf_budget` (new): `Arm {Frame, Memory, Snapshot}` · `ArmState {Pass, Fail, Neutral, Unreadable}` ·
    `grade_arm` / `grade` / `evaluate(results, required) -> Verdict {Pass, Fail, Neutral}` · `arm_lines` ·
    `nearest_rank_p99_index(n) = ⌈0.99·n⌉ − 1` (0-based; `(n*99).div_ceil(100).max(1) - 1`) · `read_family` (every
    `agent-latest.jsonl*` member, sorted, concatenated) · `family_members` · `parse_required` · `run_perf_budget`.
    Budgets as constants: frame p99 ≤ 33.0 ms (`fields.duration_ms`), memory max ≤ 512 000 000 (`fields.value`),
    snapshot p99 ≤ 500.0 ms (`fields.duration_ms`). A graded field that is not a JSON number → `Unreadable` → gate
    FAIL whether or not the arm is required. Memory samples of 0 are counted but not informative (`Neutral —
    populated 0 of n`). Gate: FAIL on any Fail/Unreadable or a required arm Neutral; else PASS on any Pass; else
    NEUTRAL (never PASS over empty input).
  - **Frame arm rendering on CI (operator decision, see Deviations):** an empty frame arm that is NOT required prints
    `perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run` — never PASS, never silent. A
    REQUIRED empty frame arm (the dev-host leg) prints `… NEUTRAL — … (required) FAIL`.
  - `xtask::perf_frame` (new): `platform_guard(os)`, `child_env(data_dir)` (`ANDROMEDA_PULSE_DATA_DIR` +
    `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--enable-unsafe-webgpu --enable-features=Vulkan --use-vulkan=swiftshader
    --use-webgpu-adapter=swiftshader`, child env only), `run_perf_frame_sample`.
  - `xtask::smoke::read_jsonl_lines` → `pub(crate)` (callers now: `smoke::assert_log_invariants`,
    `perf_budget::read_family`, `run_perf_load_profiles`).
  - `xtask::invoke_perf_slo_check` REMOVED; its two callers (`run_ci_gates`, `run_perf_load_profiles`) call the grader
    in-process with no required arm.
  - `pulse_app::snapshot_runtime::load_curated_markdown` → `pub` (callers: `snapshot.generate`,
    `investigate.run_action`, and the new producer test).
  - `app.boot.gpu.check` (`pulse-app/src/window.rs`): field `gpu_available` REMOVED (it was a hardcoded `false`);
    `wgpu_backend` kept (`detect_wgpu_backend()`, a `cfg!(target_os)` switch: dx12 / metal / vulkan); message now
    "compile-target default wgpu backend; no adapter probe runs here (the frame loop's adapter branch is the adapter
    evidence)". Consumers of `wgpu_backend` unchanged: `xtask/src/smoke.rs:440,474`, `xtask/src/self_verify.rs:115,141`.
  - Env var `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`: SET by `perf:frame-sample` on its app child only; read by no
    product file (`grep -rn WEBVIEW2 pulse-app --include=*.rs --include=*.json --include=*.ts --include=*.tsx` →
    exit 1, 0 hits).
  - No TauRPC procedure, IPC method, port or bridge-crossing field added (bindings byte-identical to `fb93fca`).
- **Crates / modules:** xtask gained modules `perf_budget`, `perf_frame`. No crate added or removed.
- **Dependencies:** none (`git diff --name-only fb93fca -- Cargo.lock Cargo.toml pulse-app/ui` → empty).
- **Schema / config:**
  - `.config/nextest.toml`: `[profile.default] default-filter = "not binary(perf_load_profiles) and not
    binary(perf_budget_samples)"`; new `[profile.perf-samples]` (`default-filter = "binary(perf_budget_samples)"`,
    `test-threads = 1`, `retries = 0`, `slow-timeout = { period = "60s", terminate-after = 5 }`).
  - Allowlist leaf `app.boot.gpu.check` = `["wgpu_backend"]` (was `["gpu_available", "wgpu_backend"]`).
  - `.github/workflows/ci.yml`:
    - lint-test, Linux only, after `cargo xtask perf:slo-load`: `cargo nextest run --workspace --profile perf-samples`
      → `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot` → `if:
      always()` upload `logs-perf-samples-${{ runner.os }}` of `target/tmp/perf-budget-samples/logs/`.
    - release (macOS + Windows): cache step now `shared-key: release-${{ runner.os }}`, saving (the `save-if: false`
      restore of `lint-test-${{ runner.os }}` is gone). **No frame boot step** in the final tree (see Reverted).
    - header comment: cache keys are one per lint-test OS, one per release OS (macOS, Windows), one for boot.
- **Spec-master edits:** none this chunk before P2.
- **Counts / qualifiers moved:**
  - workspace nextest (`--profile ci`): 2447 → **2466** (basis: gate log `Summary … 2466 tests run: 2466 passed`;
    +17 xtask unit tests = 15 `perf_budget` + 2 `perf_frame`; +2 `quality_gate_workflow` pins; the producer test is
    excluded by the default filter). Measured in implement's re-run and in `pre-push:linux` (`(2466/2466)`).
  - ci.yml cache keys: 6 target caches → **8 entries** (`gh cache list`: + `release-Windows`, `release-macOS`).
  - "release restores lint-test read-only" → release owns `release-${{ runner.os }}`.
- **Dev-tool versions:** none — no host tool installed or changed.
- **Harness / gate surface:**
  - New xtask verbs: `perf:budget --data-dir <DIR> --require <arm,arm>` (exit 0 PASS · 1 FAIL · 2 cannot-evaluate:
    unknown arm, no log family under `<DIR>/logs`, or all arms empty with none required) and `perf:frame-sample`
    (Windows only; exit 0 PASS · 1 FAIL — 0 frame samples once the app is healthy, or p99 > 33 ms · 2 INCONCLUSIVE —
    not Windows, a release binary missing, :4317/:4318 in use, or the app never healthy; artifact
    `target/perf-frame/<UTC stamp>/`). **`perf:frame-sample` is a dev-host gate, NOT CI-wired.**
  - `ci-gates`: perf arm graded in-process over EVERY family member (was: a script over `log_files.last()`); prints
    one line per arm then `ci-gates: perf-budget PASS|FAIL|NEUTRAL`; all-empty → `NEUTRAL`, never PASS; the no-log
    line is `ci-gates: perf-budget NEUTRAL (no log file to grade)` (was `perf-budget DEFERRED (no criterion bench
    yet)`). `perf:load-profiles` prints `perf:load-profiles: perf-budget {verdict}` from the same grader.
  - `xtask/ci/perf-slo-check.{sh,ps1}` DELETED (the no-`jq` NEUTRAL branch is gone with them).
  - `scripts/agent-run.ps1 boot` now mirrors `agent-run.sh`'s recorder: a hidden `powershell -EncodedCommand` wrapper
    launches the app, writes `run/andromeda-pulse.spawn` (the app pid) and on exit `run/andromeda-pulse.exit`
    (`exit N`, ASCII, no BOM); boot polls the spawn record ≤ 5 s (50 × 100 ms), exits 1 with `boot: the app did not
    start (no spawn record)`, writes the pid to the pidfile; on a failed readiness poll it prints `app ended: {record}`
    or `app still running (pid N) but never reported healthy` before cleanup. Verb set, exit semantics and
    status/cleanup fields unchanged. `harness:status`'s `ended` is therefore real under ps1 (measured `"ended":
    "exit -1"` after `Stop-Process -Force`).
  - New integration producer `pulse-app/tests/perf_budget_samples.rs` (nextest `--profile perf-samples` only):
    `observability::init` real sink into `<CARGO_TARGET_TMPDIR>/perf-budget-samples/` (removed and recreated each
    run), loopback gRPC + in-memory DuckDB + `run_consumer` + `run_retention(…, 60)` (10 s sweep), 60 batches × 500
    spans at 500 ms spacing (30 000 spans / 30 s), `heartbeat::emit_buffer_tick` every 20 batches (3 ticks), then 50
    × `load_curated_markdown` (Balanced budget). Self-checks only: rows landed, ≥ 2 non-zero ticks, 50 Ok.
- **Cross-project / external claims:**
  - `ci#36723465727` on `d708ad7` (the first pre-CI commit): **red**, 13/13 completed, sole failure `release build
    (windows-latest)` at `perf:frame-sample` — `frame: 0 samples — no WebGPU adapter in this run`, exit 1; app
    healthy (4 windows navigated, 185 `services.list_with_states.request`, 1261 records, 0 ERROR). The deciding
    measurement for the frame fallback.
  - `ci#36729367693` on `c6eb395` (final HEAD): attempt 1 **green** 13/13 (wall 2149 s); attempt 2 (`gh run rerun`)
    **green** 13/13 (attempt wall 28 m 18 s). Overseer re-read through `gh` (relay 2026-09-30).
  - `gh api …/actions/cache/usage` after attempt 2: `active_caches_size_in_bytes` **10 605 172 169** of the
    10 737 418 240 B cap (headroom 132 246 071 B ≈ 1.2 %), 8 entries.
  - Dev-host frame reading (not external, recorded here for the gate's home): `perf:frame-sample` exit 0, `frame
    p99 2.6 ms <= 33 ms (n=7971) PASS` (`evidence/slot-legs.md`).
- **Reverted / negative API facts:**
  - The CI Windows frame leg — release job steps `cargo build --release -p ingest --example inject_demo`,
    `./target/release/xtask.exe perf:frame-sample` (bash) and the `logs-perf-frame-${{ runner.os }}` upload — shipped
    in `d708ad7` and REMOVED in `c6eb395`: the hosted runner produced 0 frame samples under the SwiftShader flag set
    with the app healthy (`ci#36723465727`), and the operator took the phase fallback.
- **Insufficient fixes (written, kept, not the remedy):**
  - `app.boot.gpu.check` no longer asserts `gpu_available = false`, but nothing replaced it: the app records no
    truthful adapter-state event, so a 0-frame run cannot name its cause from the app's own log ("flags applied, no
    adapter" vs "flags not applied" read identically at `ci#36723465727`). Owner: the route entry minted at this wrap
    (relay §3 item 2).
- **Spec claims disproved by measurement:**
  1. **obs-plan §10 snapshot row / §5 perf-budget-instruments** state the 500 ms p99 bounds snapshot **generation**
     latency and that `metric.snapshot.token_count_ms` events enforce it. Measured: the emitter's timer starts at
     `crates/snapshot/src/markdown.rs:43` (`Instant::now()` at the top of `format_markdown`) and ends at
     `finish_ok` / `finish_phase_d` (`as_millis()`, whole ms) — formatting only, excluding `load_recent_spans` and
     `curate()`. The producer log: 50 × `duration_ms: 0` while back-to-back snapshots were 61–76 ms apart (median
     62, debug build, 5000 spans each, `dedup_count 4995`). The snapshot arm passes but bounds formatting only.
     Disposition: obs-plan amended to what the metric times; the fix is the route entry minted at this wrap.
  2. **obs-plan §10 p99 rule:** the §10 row's jq example indexes `.[(length * 0.99 | floor)]` and the CI-gates bullet
     says "assert max ≤ 500"; the shipped grader is nearest-rank (`⌈0.99·n⌉`-th smallest), which differs for n ≥ 100
     (n=100: floor-index picks the max, nearest-rank the 99th). Disposition: obs-plan amended to the grader's one rule
     (relay §2).
  3. **Plan premise (chunk artifact) — "a hosted Windows runner's WebView2 exposes a software WebGPU adapter under the
     probe-3 flag set":** measured false at `ci#36723465727` (0 samples, app healthy). Stated in `plan.md` Goal and
     the (obs, CI) acceptance. Chunk-artifact claim → recorded here, no amendment owed; the operator's decision
     (`evidence/frame-gate-decision.md`) supersedes that acceptance.
  4. **obs-plan §8 `app.boot.gpu.check` (`gpu_available`)** implies an adapter check; measured a hardcoded `false`
     that read `false` on a 578-frame GPU control (research §P4). Disposition: field removed in code (this chunk);
     obs-plan §8 amended.
- **Expected amendments (from plan):** (basis: `grep -cE` per master, run 2026-09-30 at P1)
  - obs-plan §1 perf-budget-instruments frame row + §10 budgets + §10 CI gates → **carried** (Harness / gate surface;
    Spec claims 1–2). Sites: `VACUOUS` obs-plan 2 (:126, :641), `perf-slo-check` obs-plan 1 (:641), `Perf-budget
    gate reads real samples` obs-plan 4 (:126, :629, :630, :641), p99 form obs-plan 3 (:628, :640, :642). The frame
    row's home is now the DEV-HOST `perf:frame-sample` (not the release Windows job the plan named) — operator
    decision. Memory gauge = rows × 256 B (`crates/buffer/src/retention.rs:153`), no RSS gate.
  - obs-plan §8 `app.boot.gpu.check` field set → **carried** (Symbols). Sites: `gpu_available` obs-plan 1 (:132).
  - obs-plan §9 artifacts `logs-perf-samples-*` / `logs-perf-frame-*` → **carried for `logs-perf-samples-*` only**;
    `logs-perf-frame-*` superseded (the CI frame upload was removed). Sites: `logs-boot-|logs-perf` obs-plan 1 (:583).
  - test-plan §1 `perf-slo-check-arm-coverage` discharged → **carried** (15 grader pins). Site: test-plan :135 (1).
    `harness-cleanup-verdict-and-boot-spawn-shell-coverage` ps1 boot half discharged → **carried** (the ps1 ended leg).
    Sites: test-plan :134, :183, :213 (3).
  - test-plan §3 `status` `ended` under ps1 + `boot` ps1 mirrors the recorder → **carried** (Harness). Sites: `does
    not mirror` test-plan 1 (:183); `andromeda-pulse.(spawn|exit)` test-plan 4 (:134, :183, :208, :269).
  - test-plan §9 lint-test row (perf-samples + `perf:budget`) + release row (own key; frame leg) → **carried for the
    lint-test row and the release key; the release frame leg superseded** (removed). Sites: cache keys test-plan 3
    (:639, :640, :642), `save-if` test-plan 1 (:640), `logs-boot-` test-plan 1 (:645). §10 load-profile constraint
    scripts → carried (`perf-slo-check` test-plan 1 :135; §10 grep 0 further hits).
  - architecture §Infrastructure Patterns → CI/CD approach cache sentence → **carried** (Schema / config). Site:
    architecture :320 (1).
  - architecture §Occupied Resources → xtask CLI surfaces (`perf:budget`, `perf:frame-sample` dev-host,
    `ci-gates` NEUTRAL wording, `agent-run.ps1` "does not mirror" retired) → **carried**. Sites: `does not mirror`
    architecture 1 (:242); `andromeda-pulse.(spawn|exit)` architecture 2 (:215, :242); `perf:budget` /
    `perf:frame-sample` 0 hits (new).
  - architecture §Occupied Resources → Environment variables: `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` harness-SET,
    frame leg child only, never read by the product → **carried** (Symbols). Section at architecture :218; `WEBVIEW2`
    0 hits in any master (new).
  - security-plan §Security Anti-Patterns → Input carve-out: ps1 `.spawn`/`.exit` records join the harness-only
    class → **carried** (Harness). Sites: `andromeda-pulse.(spawn|exit)` security-plan 0 hits; carve-out prose at
    security-plan :392–:394 names `_PIDFILE` / `_LOGFILE` / `MSEDGEDRIVER_PATH` (the class the records join).
- **Coverage of new surfaces:**
  - `cargo xtask perf:budget` → validation arm names parsed, unknown → exit 2 ✓ · instrumentation n/a (CLI verdict
    lines) · PII n/a (reads bounded numeric fields) · tests unit (15) + gate run over the producer log · a11y n/a ·
    tokens n/a
  - `cargo xtask perf:frame-sample` → validation OS / binaries / free ports preconditions ✓ · instrumentation n/a ·
    PII n/a · tests unit (2) + live leg (dev host) · a11y n/a · tokens n/a
  - `pulse-app/tests/perf_budget_samples.rs` → validation n/a · instrumentation real obs sink ✓ · PII n/a (synthetic
    spans) · tests integration (itself) · a11y n/a · tokens n/a
  - `agent-run.ps1` boot recorder → validation spawn-record poll ✓ · instrumentation n/a · PII n/a (pid + exit code)
    · tests e2e (ps1 ended leg) · a11y n/a · tokens n/a
  - `app.boot.gpu.check` (changed field set) → PII bounded static field ✓ · tests unit (2 pins) + self-verify ✓

## Deviations from intent

- **Frame arm on CI — the phase fallback (operator, founder-delegated, 2026-09-30).** The plan's (obs, CI)
  acceptance "the release Windows job's `perf:frame-sample` prints a frame line with n ≥ 1 and p99 ≤ 33 ms" is
  **SUPERSEDED, not met**: `ci#36723465727` read 0 samples with the app healthy. Decision: on CI the frame arm prints
  the named `frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run` line (never PASS, never silent);
  memory + snapshot stay hard-required; the Windows release job keeps no frame boot step; the frame budget gate is
  `perf:frame-sample` on the GPU dev host (p99 2.6 ms, n = 7971). Record: `evidence/frame-gate-decision.md`.
- `perf:budget` exits 2 when every arm is empty and none is required — the plan listed exits only for named cases,
  and exit 0 is PASS only.
- `perf:frame-sample` does not start the injector when no frame appears within 60 s after the app is healthy; it
  grades 0 frames as FAIL. It stops the app by its child handle (spawned by path, so the child pid is the pidfile
  pid), falling back to `Stop-Process` on the pidfile pid only if they differ.
- nextest `[profile.perf-samples]` carries `test-threads = 1`, `retries = 0`, a 60 s × 5 slow-timeout — unspecified
  in the plan.
- The redact pin additionally asserts `gpu_available` now redacts (step 10 named it the non-allowlisted field).
- Hygiene blocker (operator's instruction): the phase run's 49-byte known-positive fixture for plan entry #8 moved
  from `.andromeda/runs/2026-09-30T12-15-03Z-phase/ctl/pulse-app/x.rs` to
  `.andromeda/cache/phase-ctl-2026-09-30/ctl/pulse-app/x.rs` (gitignored, sha256 `08c19304…d208`). Entry #8's
  `baseline` still cites the old path.
- Not re-run after the fallback edit: `cargo xtask self-verify` (green at 13:28Z; no pulse-app source changed after
  it) and the dev-host `perf:frame-sample` (green; the edit changed only the empty, non-required frame arm's line,
  not the PASS path; the desktop was held by conductor-builder's NVDA legs).
- **Scope record** (`gate.py scope`: `clean — changed 17 · listed 8 · recorded 9 (companion 4 · mechanical 0 ·
  in-intent 5 · widening 0)`). research.md's lists were written at P3, before P4's rulings; the plan's touchpoints
  name all nine:
  - in-intent (authority: self): `xtask/src/perf_budget.rs` (step 2) · `xtask/src/perf_frame.rs` (step 6) ·
    `pulse-app/src/snapshot_runtime.rs` (step 5) · `pulse-app/src/window.rs` (step 10) ·
    `pulse-app/src/observability.rs` (step 10)
  - companion (authority: self): `xtask/src/smoke.rs` (serves `xtask/src/main.rs`) ·
    `pulse-app/tests/observability_pins.rs` (serves `pulse-app/src/observability.rs`) ·
    `crates/ingest/examples/load_profiles.rs` and `pulse-app/tests/perf_slo_10k_spans.rs` (serve the deleted
    `xtask/ci/perf-slo-check.sh`: doc comments pointing at it)

## Decisions & corrections

- **Operator: a hosted-runner 0-sample or over-budget frame reading is a decision, never a retune** — honoured; it
  came back and the operator chose the phase fallback (named cannot-evaluate on CI; dev-host frame gate).
- **Operator: every window-opening run is a slot** (conductor-builder shares the desktop and ports 4317/4318): asked
  before #12–#15; ran back to back; desktop released with a measured census.
- **Operator: the snapshot 0 ms** — record precisely which span `duration_ms` times and what obs-plan §10 bounds
  (Spec claims 1); the fix is a route entry, not this chunk.
- **Operator relay 2026-09-30:** mint ONE entry at the head of the tail — "the perf instruments measure what their
  budgets name" (snapshot timer + adapter-state record); amend obs-plan to the grader's ONE p99 rule; name the cache
  headroom as a watch.
- **Sweep hazard:** `grep WEBVIEW2` over `pulse-app` is the product-isolation probe; the var name lives in xtask
  (`perf_frame.rs`) by design, so a repo-wide grep hits and is not a violation.
- **Measurement habit:** a 0 ms percentile is not proof of a fast path — the record SPACING of back-to-back work is
  the independent reading that exposed the timer's scope.
- **Host-shell:** a PowerShell `-Command` passed inside Bash single quotes lost the inner quotes of `-like` patterns;
  a scratchpad `.ps1` run by path was the working form.
- `Stop-Process -Force` leaves exit code `-1` (`TerminateProcess(-1)`), which `read_ended`'s grammar accepts.

## Outcome

Acceptance criteria, against the diff:
- (tests) 15 grader pins pass in the 17-test selector — **met** (`17 tests run: 17 passed`).
- (tests/obs) a non-numeric graded field is a pinned FAIL whether or not required — **met**
  (`unreadable_field_fails_even_when_not_required`).
- (obs) `perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot` exits 0; memory n ≥ 2
  populated, max ≤ 512 000 000; snapshot n = 50, p99 ≤ 500 ms — **met** (local: `memory max 7680000 B … (n=3,
  populated 3) PASS`, `snapshot p99 0.0 ms <= 500 ms (n=50) PASS`; same on CI). Caveat: the snapshot value times
  formatting only (Spec claims 1).
- (obs) `perf:frame-sample` exits 0 on the dev host, frame n ≥ 1, p99 ≤ 33 ms — **met** (`frame p99 2.6 ms <= 33 ms
  (n=7971) PASS`, exit 0, 13:28:25Z → 13:28:58Z).
- (obs, CI) round-1 green; lint-test Linux prints memory + snapshot lines; release Windows prints a frame line n ≥ 1
  — **SUPERSEDED by the operator decision, not met as written**: the Windows frame line read 0 samples at
  `ci#36723465727`; at `c6eb395` round 1 is green, Linux prints the memory + snapshot PASS lines and the frame arm the
  named cannot-evaluate line.
- (obs) `ci-gates` over a sample-less log prints `perf-budget NEUTRAL`, never PASS; zero-panic + heartbeat-gap and
  `pre-push:linux` green — **met** (probe; `pre-push:linux` `"verdict": "green"`, tree `6ff3145`).
- (arch, CI) after round 2: usage ≤ 10 737 418 240 B, every rust-cache step `full match: true`, each release job
  compiles only the 16 workspace crates — **met** (10 605 172 169 B; 12/12 `full match: true`; release Windows 16,
  macOS 16; `evidence/ci-cache.md`).
- (arch/tests) under `agent-run.ps1`, after boot and the app's end, `harness:status` prints a non-null `"ended":
  "exit …"` and exits 1 — **met** (`"ended": "exit -1"`, `not-running`, exit 1).
- (obs/security) `app.boot.gpu.check` without `gpu_available`, leaf `["wgpu_backend"]`, pins updated, self-verify
  reads the record — **met** (live record in the frame leg's log; self-verify PASS).
- (security) every added `uses:` SHA-pinned, `permissions:` unchanged, no `secrets.*`; WEBVIEW2 product grep 0 —
  **met** (probe exit 1 / no output; grep exit 1).
- (a11y) the a11y matrix job unchanged and green on round 1 — **met** (3/3 a11y jobs success at `ci#36729367693`
  attempt 1).
- (tests) the standard gate set in order, closing on the bindings `git diff --quiet fb93fca` exit 0 — **met**.
- No verification-matrix capability claimed — **met** (`matrix.py show`: claimed 0).

Gates (implement's re-run after the fallback edit — run dir `2026-09-30T12-54-35Z-implement`):
- `cargo fmt --check` green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` green ·
  `cargo nextest run --workspace --profile ci -E 'test(/^perf_budget::/) | test(/^perf_frame::/)'` green (exit 0,
  `17 tests run: 17 passed`) · `cargo nextest run --workspace --profile ci -E 'binary(observability_pins) |
  binary(quality_gate_workflow)'` green (148 passed) · `cargo nextest run --workspace --profile perf-samples` green
  (`1 test run: 1 passed`, artifact fresh) · `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples
  --require memory,snapshot` green (exit 0) · the seeded `ci-gates` probe green (exit 0, `perf-budget NEUTRAL`, no
  `perf-budget PASS`) · `grep -rn WEBVIEW2 pulse-app …` green (exit 1, no output) · the ci.yml `uses:/secrets./
  permissions:` diff probe green (exit 1, no output) · `cargo build -p pulse-app --release` green · `cargo build
  --release -p ingest --example inject_demo` green · `cargo xtask self-verify` green (exit 0, `self-verify: PASS`,
  first pass; not re-run after the fallback — no pulse-app change) · the frame-leg precondition probe green (first
  pass) · `cargo xtask perf:frame-sample` — leg live, fired by hand in the operator slot, exit 0 · the ps1 `ended`
  leg — leg live, fired by hand, exit 1 with `"ended": "exit` (its expect) · `git diff --name-only fb93fca --
  Cargo.lock Cargo.toml pulse-app/ui` green (no output) · `git diff --name-only fb93fca -- . …` recorded (the roster
  above) · `cargo deny check bans licenses sources` green · `cargo xtask capability-widening-check` green · `cargo
  xtask check:ingest-progress` green · `cargo xtask check:staged-artifacts` green · `cargo xtask capability-drift`
  green · `cargo nextest run --workspace --profile ci` green (2466/2466) · the `--features mcp-server`
  `emit_taurpc_bindings` regen green · `git diff --quiet fb93fca -- pulse-app/ui/src/bindings/index.ts` green.
- Operator entries (`evidence/operator-pass.md`, `evidence/ci-cache.md`): `gate.py hygiene` → `hygiene: clean` ·
  `cargo xtask pre-push:linux` → `"verdict": "green"` · the guarded push → `d708ad7..c6eb395` · `ci.py conclusion`
  round 1 → `verdict: green` (`ci#36729367693`) · `gh run rerun` → exit 0 · round 2 → `verdict: green` · `gh api
  …/cache/usage` → recorded, 10 605 172 169 B.
- Smoke: `self-verify` PASS on the release binary (boot trio, window shown, heartbeat, 0 panics, a11y/contrast
  PASS, zero orphan).

Watches: none folded. **Watch (relay):** cache headroom 1.2 % (10 605 172 169 / 10 737 418 240 B). The next
`Cargo.lock`-driven lint-test re-save is expected to shrink the lint-test Windows/macOS keys (they still carry release
dependencies); a round that reads over the cap is a regression of this chunk's acceptance.

Outcome basis: the operator pass ran — commits `d708ad7` (round 1 red, the frame reading) and `c6eb395` (after the
fallback); the final HEAD's CI `ci#36729367693` both attempts green, recorded in `evidence/ci-cache.md` and
`evidence/operator-pass.md`; implement's gate re-run after the operator's decision; the operator relay
`pc-overseer/relays/pulse-wrap-perf-2026-09-30.md`.

Process hygiene (implement's census, measured through PowerShell `Get-Process` / `Get-NetTCPConnection`): gate-block
cargo builds and tests — terminated; `pulse-app` from `self-verify` — terminated (zero orphan); `pulse-app` +
`inject_demo` from `perf:frame-sample` — terminated, :4317/:4318 released; `pulse-app` + the hidden wrapper
`powershell` from `agent-run.ps1 boot` — terminated (0 wrappers, 0 listeners on 14317/14318). No process started
since the operator pass.
