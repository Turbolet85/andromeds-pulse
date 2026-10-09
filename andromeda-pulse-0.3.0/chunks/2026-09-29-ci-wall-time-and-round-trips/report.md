# Report — 2026-09-29-ci-wall-time-and-round-trips

**Chunk:** CI wall time and round-trips — parallel jobs, a kept cache, no duplicate rebuilds, a local WSL Linux pre-push check
**Date:** 2026-09-29
**Commits:** `226554a` · `4502d5d` · `dd5c700` — each `chore(2026-09-29-ci-wall-time-and-round-trips): operator pre-CI commit` (base `e98d838`, the oldest pre-CI commit's parent; `git log --format='%h %s' e98d838..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `.github/workflows/ci.yml` · `xtask/src/main.rs` · `xtask/src/pre_push.rs` (new) · `xtask/src/harness_status.rs` ·
  `xtask/src/gap_resume.rs` · `xtask/src/external_resolve.rs` · `xtask/src/webview_drive.rs` · `scripts/agent-run.sh` ·
  `pulse-app/tests/quality_gate_workflow.rs` · `pulse-app/tests/a11y_perf_workflow.rs` (basis: `gate.py scope`, base
  `e98d8384` — changed 10 · listed 5 · recorded 5)
- **Symbols / APIs:**
  - NEW xtask verb `cargo xtask pre-push:linux` (`Cmd::PrePushLinux`, `xtask/src/pre_push.rs::run`). Contract: exit 0 green ·
    1 red · 2 cannot-evaluate; one pretty-JSON verdict on stdout `{verdict, reason, head, tree, stages[{name, ok, ms}],
    missing[], remediation, cache{bytes, cap, cleaned}}` + report twin `target/pre-push/report.json`. Windows host only;
    every distro call is `wsl.exe -d Ubuntu --exec /usr/bin/env -i HOME=… PATH=… [ANDROMEDA_PULSE_DATA_DIR=…]`. Pins read
    from the repo: `rust-toolchain.toml` channel, ci.yml `node-version` major (all occurrences must agree), ci.yml's first
    `apt-get install` package list; also checks `clippy` + `cargo-nextest` for the channel and `git` / `jq` / `xvfb-run` /
    `cc` on PATH. Any missing → `cannot-evaluate` `provisioning-missing`, `missing[]` named, `remediation` = one
    `sudo apt-get install -y --no-install-recommends …` line for the apt-installable ones (the verb never escalates).
    Sync: a distro-side clone `~/andromeda-pulse-pre-push` brought to HEAD + the working tree as one binary patch through a
    temporary `GIT_INDEX_FILE` (`target/pre-push/index`; the real index untouched), verified by tree-id equality
    (`sync-mismatch` / `sync-failed:{step}` → red). Clone `target/` capped at 40 GiB. Stages in order, first failure stops:
    `script-modes` (`scripts/agent-run.sh` git mode 100755) · `npm` (`npm ci` + `npm run build` in `pulse-app/ui`) ·
    `clippy` (`--workspace --all-targets --all-features -- -D warnings`) · `test` (`cargo xtask test`) · `ci-gates`
    (`cargo xtask ci-gates` over a fresh data dir seeded with 3 non-metric records: one `app.boot.ready`, two `ingest.tick`
    15 s apart). Binds no port, starts no `pulse-app`. Reads NO new env var (distro name, clone dir, cap are constants).
    **Cross-project dependency (fragility):** the verb's distro PATH puts `~/.local/viola-node/bin` first for Node, because
    Ubuntu 26.04's apt ships `nodejs` 22.22 / `npm` 9.2 against ci.yml's Node 24 (npm 10 vs 11 disagree on lockfiles —
    measured `apt-cache policy nodejs npm`); that Node 24.21.0 is provisioned by the Viola repo's `scripts/wsl-provision.sh`.
    If it goes, the verb reads `cannot-evaluate` (`node:24 (found …)`), never green.
  - NEW `xtask/src/main.rs::cargo_command()` (+ `is_cargo_run_injected`): every cargo child xtask spawns drops the variables
    `cargo run` injects into xtask — `CARGO_PKG_*`, `CARGO_MANIFEST_DIR`, `CARGO_MANIFEST_PATH`, `CARGO_MANIFEST_LINKS`,
    `CARGO_CRATE_NAME`, `CARGO_BIN_NAME`, `CARGO_PRIMARY_PACKAGE` (keeps `CARGO`, `CARGO_HOME`, `CARGO_TARGET_DIR`, …).
    All 8 spawn sites use it (`grep -n 'cargo_command()\|Command::new("cargo")' xtask/src/*.rs`: 5 in main.rs —
    `run_cargo`, `run_cargo_nextest`, `run_cargo_llvm_cov`, `run_perf_slo_load`, `run_perf_load_profiles` — plus
    `build_injector` in gap_resume.rs / external_resolve.rs / webview_drive.rs; the only remaining `Command::new("cargo")` is
    inside the helper).
  - CHANGED `run_perf_slo_load`: `nextest run -p pulse-app --test perf_slo_10k_spans` → `nextest run --workspace -E
    'binary(perf_slo_10k_spans)'` (same `--profile ci --no-tests=pass --message-format libtest-json` + env). Sole caller:
    `main()` dispatch.
  - CHANGED `cargo xtask harness:status` verdict JSON: adds `ended` (string | null) → `{verdict, pid, ended,
    log_file_basename, last_write_age_seconds, stale_after_seconds}`. `ended` is the bounded one-line record (≤ 48 printable
    ASCII chars, `exit N` or `signal N (NAME)`) read from `andromeda-pulse.exit` beside the pidfile, reported only when the
    verdict is not `running-healthy`; arms and exit codes unchanged. Callers unchanged (the two `agent-run` legs).
  - CHANGED `scripts/agent-run.sh boot`: the app now runs under a waiting subshell (stdin/stdout/stderr `/dev/null`) that
    writes the app pid to `run/andromeda-pulse.spawn` and, on reaping it, writes `run/andromeda-pulse.exit`; boot reads the
    spawn record for `DAEMON_PID` (no record within 5 s → `boot: the app did not start (no spawn record)`, exit 1). Boot's
    failure path now prints `app ended: {record}` (was a `wait` on a direct child). The 5 verbs, their exit codes and
    stdout tokens are unchanged. `scripts/agent-run.ps1` is NOT mirrored (its `ended` stays null).
- **Crates / modules:** new module `xtask::pre_push`; new `mod cargo_command_tests` in `xtask/src/main.rs`.
- **Dependencies:** none (no Cargo.toml / lockfile change; no new GitHub Action — every `uses:` keeps its 40-char SHA).
- **Schema / config:** `.github/workflows/ci.yml` — 3 jobs → 7:
  - `lint-test` (matrix ubuntu-22.04/macos/windows): fmt · No-Cyrillic · clippy · typecheck · quarantine-tracking ·
    capability-drift · check:staged-artifacts · capability-widening-check · verify:capability-matrix · `cargo xtask test` ·
    (Linux) `perf:slo-load` · uploads `logs-${{ runner.os }}`, `nextest-*`, `criterion-*`, criterion download +
    `criterion-regression`, `capability-drift-*`. Tool install is `cargo-nextest` only (no llvm-cov). Cache
    `shared-key: lint-test-${{ runner.os }}`, `cache-on-failure: true`.
  - `release` (matrix macos-latest/windows-latest): `cargo build --workspace --release`. Cache restore-only
    `lint-test-${{ runner.os }}` (`save-if: false`).
  - `mcp-test` (ubuntu-22.04): `cargo nextest run --workspace --features mcp-server --profile ci --no-tests=pass`. Cache
    restore-only `lint-test-Linux`.
  - `a11y` (matrix, 3 OS): `npm run build --prefix pulse-app/ui` → Playwright chromium → PR-only `a11y-violations-base`
    download → `cargo xtask test:a11y` → uploads `a11y-violations-*`, `playwright-a11y-report-*` (`if: always()`). Cache
    restore-only `lint-test-${{ runner.os }}`.
  - `boot` (ubuntu-22.04): `cargo build --workspace --release --features mcp-server` → `Boot pulse-app smoke` (one xvfb-run:
    boot → status → cleanup) → `cargo xtask ci-gates` → upload `logs-boot-${{ runner.os }}` (`if: always()`). Cache
    `shared-key: boot-Linux`, `cache-on-failure: true`.
  - `supply-chain`: steps unchanged; cache restore-only `boot-Linux`. Its `cargo auditable build --workspace --release` is now
    the only Linux workspace release build (Linux `cargo build --workspace --release` removed as its duplicate).
  - `coverage`: steps unchanged; cache `shared-key: coverage`, `cache-targets: false` (registry only), `cache-on-failure: true`.
  - Removed from the roster (basis: `yaml` roster diff, 68 → 104 job|step rows): the Linux instance of `cargo build
    --workspace --release`; the macOS/Windows instances of `cargo xtask ci-gates` (both read `zero-spans NEUTRAL … no
    agent-latest.jsonl*` in ci#36593334009 — no boot on those OSes). Every other gate step keeps a blocking step.
  - New data-dir files written by the harness (never by the product binary): `run/andromeda-pulse.exit`,
    `run/andromeda-pulse.spawn`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - ci.yml job count 3 → 7 (pinned `("ci.yml", 7)` in `quality_gate_workflow.rs::data_dir_export_precedes_every_consumer`);
    GitHub check count per round 10 → 13 (ci#36625507595 / ci#36632205717: 12 ci jobs + secret-scan).
  - Workspace test count 2408 → 2411 on Linux (`pre-push:linux` Summary lines: 2410 after `cargo_command_tests` +2, 2411 after the `read_ended` test +1) and xtask unit
    tests +13 (10 `pre_push::tests`, 2 `cargo_command_tests`, 1 `read_ended_takes_one_bounded_line_beside_the_pidfile`).
  - CI round wall-clock: cold 80.9 min (ci#36607192309 attempt 1, `226554a`, 5-job layout, every key renamed → `No cache
    found`); warm 47.6 min first-start→last-end (19:07:55 → 19:55:30; the overseer's 47.7 min is the run duration) on
    attempt 2 of the same run; 27.6 min on `4502d5d` (ci#36625507595, 7 jobs, warm); 25.5 min on `dd5c700`
    (ci#36632205717). The old 3-job layout warm baseline: 44.8 min (ci#36593334009).
  - Actions cache: 9.27 GB / 5 keys at P3 → 8.58 GB / 6 keys (`gh api …/actions/cache/usage` after each round).
- **Dev-tool versions:** none on the dev host. The WSL `Ubuntu` 26.04.1 distro was provisioned by the founder
  (2026-09-29, `wsl -d Ubuntu -u root -- sh -c "apt-get update && apt-get install -y --no-install-recommends jq pkg-config
  libssl-dev libdbus-1-dev libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
  libayatana-appindicator3-dev librsvg2-dev libxdo-dev"`): `jq` + the 10 ci.yml apt packages, none installed before. Rust
  1.95.0 + cargo-nextest + Node 24.21.0 (Viola's) pre-existed there. No project lockfile touched.
- **Harness / gate surface:** new xtask verb `pre-push:linux`; `harness:status` gains `ended`; `agent-run.sh boot` waiting
  wrapper + the two run-dir records; `perf:slo-load` narrowing form; ci.yml job split and cache allocation (above); xtask's
  child-cargo env discipline (`cargo_command`).
- **Cross-project / external claims:**
  - CI runs (repo Turbolet85/andromeds-pulse, draft PR #39): ci#36607192309 attempt 1 on `226554a` — success 10/10, cold,
    80.9 min; attempt 2 (overseer re-run) — success, warm, 47.6 min; ci#36625507595 on `4502d5d` — **failure**, 12/13
    (boot smoke red, below), 27.6 min; ci#36632205717 on `dd5c700` — **success 13/13**, 25.5 min (`ci.py conclusion`:
    `dd5c700d0b3f verdict: green · checks 13/13 · wall 1533 s`). This wrap's own commit adds to `dd5c700`'s tree.
  - The Viola repo (`D:/dev/projects/viola`, read-only): `scripts/wsl-exec.sh` + `crates/viola-e2e/src/harness/pre_push/linux.rs`
    as the sync/verdict precedent; its `~/.local/viola-node` Node 24 as the pre-push verb's Node (fragility above).
  - `Swatinem/rust-cache` at `23869a5…`: `action.yml` inputs `shared-key` / `cache-targets` / `cache-on-failure` / `save-if`
    exist; `src/cleanup.ts` keeps a nested target dir (CACHEDIR.TAG) — read via `gh api …/contents?ref=`.
- **Reverted / negative API facts:** the first draft of `pre_push.rs` built the stage table as nested slice literals
  (`Stage::Run(&[(&str, &[&str])])`) — replaced by a plain `Stage` enum before any compile; nothing shipped.
- **Insufficient fixes (written, kept, not the remedy):**
  - The first operator pass's layout (5 jobs, `perf:slo-load` narrowed to `--workspace -E`) did not deliver the wall-time
    goal: warm attempt 2 took 47.6 min, over the 44.8 min 3-job baseline and the ~27 min forecast. The `-E` narrowing is
    correct and kept, but on its own it did not stop the rebuild. The cause, measured: `cargo run -p xtask` injects xtask's
    own `CARGO_PKG_*` / `CARGO_MANIFEST_DIR` into the xtask process; `ring`'s build script declares
    `rerun-if-env-changed` for `CARGO_MANIFEST_DIR`, `CARGO_PKG_NAME`, `CARGO_PKG_VERSION_{MAJOR,MINOR,PATCH,PRE}` (read
    from `target/debug/build/ring-*/output` in the WSL clone), so every alternation between a shell-launched cargo and an
    xtask-launched cargo re-ran ring's build script and made ~22 crates stale (rustls, reqwest, ureq, `libduckdb-sys` whose
    C++ build is ~7 min, every workspace crate). Fingerprint log (`CARGO_LOG=cargo::core::compiler::fingerprint=info`,
    WSL clone): `dependency on ring is newer than we are` / `UnitDependencyInfoChanged`. Attempt 2's ubuntu `perf:slo-load`
    step spent 7.5 min silent (the `--quiet` alias hiding xtask's own rebuild) then 9.4 min recompiling. Remedy
    `cargo_command()`; measured in the WSL clone after it: `perf:slo-load` 14 s / `criterion-regression` 1 s / a second
    `cargo xtask test` 15 s, each `compiled=0`; `pre-push:linux` test stage 340 s → 24.9 s, ci-gates stage 110 s → 0.7 s.
  - The end-status recorder does not fix the Linux boot death (below); it only makes the next one name its signal. Owner:
    the WATCH.
- **Spec claims disproved by measurement:**
  - This chunk's own plan forecast (plan.md Acceptance, "predicted ≈ 27 min warm, set by Windows lint-test"): false for the
    5-job layout (47.6 min warm); met only after the env fix + the 7-job split (25.5 min). Disposed: recorded here, no
    amendment owed (a chunk-artifact claim).
  - This chunk's research.md §Graph impact/§Patterns: "a `-p pulse-app` selection compiles a different unit graph … perf
    pays exactly this compile" as the whole cause of the perf step's cost — incomplete: the dominant cost was the env-driven
    ring rerun, which also hit `criterion-regression` (5.9–7.6 min warm per OS) and the warm clippy/test steps. Disposed:
    recorded here, no amendment owed.
  - arch §Infrastructure Patterns → CI/CD approach (`architecture.md:319`): "matrix over Linux/macOS/Windows; steps `cargo
    fmt --check` → `cargo clippy … -D warnings` → `cargo xtask test` → `cargo build --workspace` (release profile smoke)"
    — now false (7 jobs; Linux release smoke is supply-chain's auditable build). Owner: the arch expected amendment below.
- **Expected amendments (from plan):**
  - `architecture.md §Infrastructure Patterns → CI/CD approach` → carried: Schema/config (7 jobs). Site: `grep -n 'CI/CD
    approach' architecture.md` 1 hit (:317; body :319).
  - `architecture.md §Occupied Resources → xtask CLI surfaces` — register `pre-push:linux`; register `smoke:gap-resume` +
    `smoke:external-resolve` beside `smoke:hue-shift` (CARRY E) → carried: Symbols/APIs (pre-push:linux). CARRY E's
    by-construction evidence: `grep -n 'name = "smoke:' xtask/src/main.rs` → `:65` gap-resume, `:81` external-resolve, `:91`
    hue-shift, dispatch arms in `main()`; the verbs are dev-host only, not CI-wired; gap-resume's three arms (default gap ·
    `--sustained` · `--reconnect-only`); external-resolve's `reconciled_count` verdict. Sites: `grep -c 'smoke:hue-shift'
    architecture.md` 1 (:241) · `smoke:gap-resume` 1 · `smoke:external-resolve` 0 · `pre-push` 0. The same `:241` bullet
    also states the `harness:status` JSON shape, now with `ended` (Symbols/APIs).
  - `test-plan.md §9 Pipeline structure` — rows follow the jobs → carried: Schema/config. Sites: `grep -n 'Pipeline
    structure' test-plan.md` 1 (:627), `lint-test-build` 1 hit, `shared-key|rust-cache` 1 hit.
  - `test-plan.md §3` — the pre-push verb beside the harness verbs → carried: Symbols/APIs. Site: `harness:status` 6 hits in
    test-plan (`:183`, `:195`, `:268` region = §3); the `status` verb shape there gains `ended`.
  - `obs-plan.md §9 Telemetry artifact handling` — `logs-boot-*` beside `logs-*` → carried: Schema/config. Site: `grep -n
    'Telemetry artifact handling' obs-plan.md` 1 (:579).
  - `a11y-plan.md §9 Pipeline integration` — the a11y gate as its own matrix job → carried: Schema/config. Sites: `grep -n
    'Per-pipeline-stage\|CI integration' a11y-plan.md` 2 (:358 §3 CI integration, :580 §9); `test:a11y` 6 hits.
  - Not in the plan's list, surfaced by this chunk: arch §Occupied Resources filesystem subpaths (`architecture.md:214`) —
    `run/andromeda-pulse.{exit,spawn}` are harness-written files under the data dir (Schema/config).
- **Coverage of new surfaces:**
  - `cargo xtask pre-push:linux` → validation pins read from repo + tree-id equality✓ · instrumentation JSON verdict + report
    twin✓ · PII n/a (no telemetry content; host paths stay in the distro, the verdict carries head/tree ids) · tests unit
    (10 pure-part tests, each known-positive + known-negative) + integration (the gate entry, green) · a11y n/a · tokens n/a
  - `harness:status.ended` → validation bounded printable one-liner✓ · instrumentation n/a · PII n/a · tests unit (5
    arms) + live (30 WSL trials each recorded `signal 15 (TERM)` from cleanup; CI dd5c700 boot green) · a11y n/a · tokens n/a
  - `xtask cargo_command()` → validation n/a · tests unit (2; the drop test mutation-checked: neutralizing the removal
    turned it red, restored) + live (WSL tail replay compiled=0) · others n/a
  - ci.yml jobs → tests `quality_gate_workflow.rs` (job count 7, `logs-boot-` present, no `continue-on-error` on gates) +
    `a11y_perf_workflow.rs` (build-before-a11y scoped to the `a11y` job) · SHA pins✓ · workflow `contents: read`✓

## Deviations from intent
- **Seven jobs, not five** — overseer, after attempt 2: "move the serial tail (perf, criterion, the per-OS release build, mcp
  nextest) into parallel jobs so the longest job lands near 25 min. Keep cache keys stable or shared". `release`
  (macOS/Windows) and `mcp-test` (Linux) were split out. `perf:slo-load` and `criterion-regression` stayed in `lint-test`:
  with the env fix they measured 14 s and 1 s (WSL replay), so a separate job would only add a ~3 min prelude.
- **Plan gate entry 3 re-pinned** (the YAML job-list probe): its `expect` now reads `last line a11y boot coverage lint-test
  mcp-test release supply-chain`, on the operator's word at this wrap ("gate entry 3 (five jobs) is re-pinned to the
  seven-job roster"). Before the re-pin it read red at /implement's second gate run on the seven-job layout.
- **Release-job cache trade-off** (a cost of "keep cache keys stable"): `release` restores `lint-test-${{ runner.os }}`
  read-only instead of owning a key, because the 10 GB cap cannot hold two more target caches (8.58 GB used). That key is
  full-match frozen with release deps from the cold round, so the release job is warm now; when `Cargo.lock` next changes,
  `lint-test` re-saves without release deps and the release job builds them cold every round (~13+ min on Windows) until the
  budget is revisited. Operator: owner by route-resolve in this version.
- **`agent-run.ps1` not mirrored** for the end-status recorder: the Windows harness writes no `andromeda-pulse.exit`, so
  `ended` stays null there; CI boots only on Linux. Operator: owner by route-resolve in this version.
- **ci-gates no longer runs on macOS/Windows** (plan Step 1 layout): those instances read NEUTRAL with no boot log in
  ci#36593334009, so no measured gate was lost.
- **Node 24 from Viola's distro install** (above) — the plan said "Node major = ci.yml's node-version" without naming a
  source; apt cannot supply it.
- Scope record (`gate.py scope`: clean — changed 10 · listed 5 · recorded 5):
  - in-intent · serves `xtask/src/main.rs` · self — `xtask/src/gap_resume.rs`, `xtask/src/external_resolve.rs`,
    `xtask/src/webview_drive.rs` (their `build_injector` spawns switched to `cargo_command()`).
  - widening · serves `.github/workflows/ci.yml` · word: "Make status (or the harness) report how the app ended (signal or
    exit code) on this path too" — the overseer — `scripts/agent-run.sh`, `xtask/src/harness_status.rs`.

## Decisions & corrections
- Measured, reusable: a `cargo run`-launched tool that spawns cargo leaks `CARGO_PKG_*` / `CARGO_MANIFEST_DIR` into the
  child; build scripts that `rerun-if-env-changed` on them (ring does) flip on every alternation with shell-launched cargo.
  Symptom: the same crates recompile in consecutive steps on a full-match cache. Probe: the crate's
  `target/*/build/{crate}-*/output` `rerun-if-env-changed` lines + `CARGO_LOG=cargo::core::compiler::fingerprint=info`
  (`dependency on X is newer than we are`).
- The `cargo xtask` alias is `run --quiet`, which hides xtask's OWN compile — a long silent stretch at the start of an xtask
  CI step is a rebuild, not a hang.
- A `run_in_background`-spawned app is an orphan once its launching script returns: nothing can later read its exit status.
  The recorder is a waiting wrapper that writes the status when it reaps the app.
- Overseer rulings this chunk: seven-job split + keep cache keys stable; `pre-push:linux` exists; the host constraint
  (no release build / no pulse-app / no 4317-4318) held until released; a Conductor slot HOLD mid-chunk (no pulse-app launch
  in WSL either, since WSL localhost forwarding would take 4317).
- Sweep hazard: `grep -c 'Command::new("cargo")'` also matches the helper's own body — dispositioned by line.

## Outcome
- (arch/tests) ci.yml parses and carries exactly the planned jobs — MET for the seven-job roster (operator-widened); every
  job starts Harden runner → Export DATA_DIR, pinned at 7.
- (tests/security) gate roster preserved — MET: every gate step still blocking; removals argued above (Linux release build
  duplicate; mac/win ci-gates vacuous).
- (tests) `cargo nextest run --workspace --profile ci` 0 — MET (green, Windows; 2411 on Linux via pre-push).
- (tests) `perf:slo-load` selects exactly one test via `--workspace -E` — MET (`Summary … 1 test run: 1 passed`).
- (security) SHA pins, no new action, `contents: read`, no signing secret — MET (`a11y_perf_workflow.rs` pins green).
- (a11y) a11y job order + per-OS uploads — MET; ≤10 min budget: cold round MISSED (12.9–18.4 min), attempt 2 MET (4.3–8.9),
  `4502d5d` MISSED on Windows (12.1 min), `dd5c700` MET (4.2–9.1 min) — Windows a11y sits at the budget's edge.
- (obs) ci-gates in `boot` after the boot smoke; `logs-*` + `logs-boot-*` uploads — MET.
- (arch/tests) `pre-push:linux` 0/1/2 + one JSON verdict + cannot-evaluate on missing provisioning — MET (unit-tested
  mapping; live `cannot-evaluate` before provisioning, `green` after, 5/5 stages).
- (ci) final pushed HEAD green over all jobs — MET: `dd5c700` ci#36632205717 `verdict: green · checks 13/13`. Forecast
  ≈27 min warm — MISSED on the 5-job layout (47.6 min, attempt 2: ubuntu lint-test 47.6 · windows 41.9 · macOS 34.6 ·
  coverage 19.9 · boot 10.2 · a11y 4.3–8.9 · supply-chain 7.5), MET after the env fix + split: 27.6 min (`4502d5d`),
  25.5 min (`dd5c700`, longest job coverage 25.5 — its instrumented build is uncached by design).
- (ci) cache usage after the green round + no `No cache found` in round 2+ — MET: 8.58 GB / 10 GB; `4502d5d` and `dd5c700`
  logged `full match: true` for every rust-cache step.
- (arch) CARRY E — MET by construction (`xtask/src/main.rs:65,81,91`), evidence for the arch amendment.
- Gates (plan `## Test Commands`, /implement second run + re-run): `cargo fmt --check` green · `cargo clippy --workspace
  --all-targets --all-features -- -D warnings` green · the YAML job-list probe red · `last line …` (five-job atom; re-pinned
  at this wrap to seven) · the YAML roster probe recorded (104 rows) · `cargo nextest run --workspace --profile ci` green ·
  `cargo xtask perf:slo-load` green (17.0 s Windows) · `cargo xtask pre-push:linux` green · `cargo xtask
  check:ingest-progress` green · `cargo xtask capability-drift` red on the default-features nextest's `bindings/index.ts`
  rewrite (worktree only, staged copy clean) → green after restoring the file from HEAD (this chunk changes no TauRPC).
  Operator legs (evidence `evidence/operator-pass.md`, three passes): hygiene clean (pass 2 first refused one host path in
  the evidence file, rewritten `{tools_dir}/`) · guarded pushes `226554a`, `4502d5d`, `dd5c700` · `ci.py conclusion`:
  green / red (boot) / green · per-job timings and cache usage recorded per pass.
- Smoke: skipped at /implement P3 (no product boot-path change; `pulse-app` start forbidden then); CI's boot job ran it each
  round.
- Watches:
  - Linux boot silent death · RECURRED ci#36625507595 on `4502d5d`: `boot: ready (PID=6847)` then `harness:status`
    `not-running` 0.7 s later; `logs-boot-Linux` 40 records ending at the boot binds + first ticks, no `app.panic.fatal`,
    only ERROR `KeyringUnavailable`; `boot.log` one AT-SPI warning; the pre-recorder harness could not name the end.
    Not reproduced locally: 30/30 WSL trials healthy (10 unconstrained + 20 `taskset -c 0-3`, status at 0 s and +3 s; the
    app reached its webview IPC phase). Green after: ci#36632205717 on `dd5c700` (the recorder's first CI round) → 1 of 3.
    Instrument: `harness:status.ended`. Caveat: both CI deaths fell within a second of `boot`'s shell exiting (the app then
    an orphan); the recorder keeps a waiting parent, which may itself change the rate — a green round is not a fix.
- Outcome basis: the operator pass ran — Setup 4's commits `226554a` / `4502d5d` / `dd5c700` and the final HEAD's CI run
  (ci#36632205717) recorded in `evidence/operator-pass.md`; overseer relays between passes (attempt-2 numbers, the split
  directive, the recorder directive, the Conductor HOLD).
- Process hygiene: none left running — WSL `pgrep` no `pulse-app` / `Xvfb`, `ss` 4317/4318 free; Windows `netstat` no
  4317/4318 listener; background monitors stopped (measured after the last trial loop; nothing launched since).
