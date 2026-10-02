# Codebase Research — 2026-09-29-ci-wall-time-and-round-trips

## Scope
- **Depth:** deep on the CI surface (the workflow, the xtask verbs it calls, the workflow pinning tests, two completed CI
  runs read step by step), moderate on the WSL leg (host probes + the Viola precedent read-only) · **Reads:** 14 ·
  **Globs/Greps:** 22 · **CI runs read:** 2 (per-job and per-step timings, cache lines, compile/run split)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (51.5 KB, 20 Session Additions). Four
  apply here:
  - The 5-command discipline: no 6th `agent-run` verb, so the pre-push surface is an xtask verb.
  - `agent-run.sh` honours `ANDROMEDA_PULSE_OTLP_{GRPC,HTTP}_PORT` (`scripts/agent-run.sh:32-33`).
  - Cross-verb state needs ONE exported `ANDROMEDA_PULSE_DATA_DIR` (never `$$`).
  - `boot`'s pre-build shape (`cargo build --bin pulse-app --release` then `cargo build -p xtask`,
    `scripts/agent-run.sh:48,55`).
- **Platform issues consulted:** none. Scope carries no runner-only bullet: the only CI verdict Setup read (`e98d838`) was
  in progress then and completed green, per this research.

## CI runs read (the measurements this plan rests on)
Both are runs of this PR's ci.yml at HEAD shape. Timings are from `gh run view {id} --json jobs` (step
`startedAt`/`completedAt`), and cache lines are from `gh run view {id} --job {job} --log`.

| run | sha | cache state (ubuntu leg) | round wall | ubuntu | windows | macOS | coverage | supply-chain |
|---|---|---|---|---|---|---|---|---|
| ci#36574279289 | `464f2a3` | `No cache found` (log 13:20:14) → saved 15:18 | **119.7 min** | 119.7 | 42.4 | 33.0 | 17.0 | 6.6 |
| ci#36593334009 | `e98d838` | `Restored … full match: true` (15:52:43) | **44.8 min** | 44.8 | 37.1 | 33.4 | 16.0 | 7.8 |

Ubuntu leg per step, cold → warm (steps ≥ 30 s). Cold compile/run split read from the
`Finished … in` and `Summary [` lines:

| step | cold | warm | what it compiles |
|---|---|---|---|
| clippy `--workspace --all-targets --all-features` | 13.0 | 0.4 | check profile, all features |
| `cargo xtask typecheck` | 11.4 | 0.2 | the xtask binary (dev); tsc itself is seconds |
| `cargo xtask test` (nextest `--workspace --profile ci`, default features) | 14.4 (compile 13m53s, run ~30 s, 2398 tests) | 7.1 | test profile, default features |
| `cargo xtask test:a11y` | 10.0 | 5.9 | the xtask binary only (npm / Playwright / Lighthouse run) |
| `cargo xtask perf:slo-load` (`nextest -p pulse-app --test perf_slo_10k_spans`) | 10.1 (compile 9m52s, run 12.9 s, 1 test) | 6.1 | test profile, `-p pulse-app` feature set |
| `cargo build --workspace --release` | 25.6 | 3.2 | release, default features |
| `cargo build --workspace --release --features mcp-server` | 5.2 | 3.2 | release, mcp feature |
| `cargo nextest run --workspace --features mcp-server --profile ci` | 12.4 (compile 11m54s, run ~30 s, 2433 tests) | 7.2 | test profile, mcp feature |
| Boot smoke (`boot` → `status` → `cleanup` in one xvfb-run) | 12.8 | 7.7 | `--bin pulse-app --release` + xtask, then the ~2 s boot |

- **Premise corrected: the entry's "~2 h per round" is a cold-cache round.** That number came from ci#36574279289, and its
  ubuntu leg logged `No cache found`. The first warm round, ci#36593334009, took 44.8 min. Each of P-025's 8 red rounds
  paid the cold price, because a failed job saves no cache (see B below). So the steady-state bottleneck is ~45 min, and
  it sits on the ubuntu leg, which is **compile-dominated**: every heavy step is a separate compile of a separate
  profile/feature set, and the whole suite's execution is ~30 s.
- The un-split macOS/Windows legs (33.4 / 37.1 min warm) sit just under ubuntu. Splitting ubuntu alone can bring the round
  down no further than those legs.

## Files inspected
- `.github/workflows/ci.yml` (full, 431 lines) — the three jobs are at `:19`, `:255` and `:329`:
  - `lint-test-build`: an OS matrix with `fail-fast: false`, holding the serial chain above.
  - `supply-chain`: audit-check, deny-action, the Cranelift assertion, `check:npm-supply-chain`, then
    `cargo auditable build --workspace --release`.
  - `coverage`: `cargo xtask test:coverage`, a threshold `run:` block, then coverage-regression.

  Every job has harden-runner first and the `Export ANDROMEDA_PULSE_DATA_DIR` step second. Three rust-cache keys are in
  use: `${{ runner.os }}-cargo` (`:42-45`), and `ubuntu-22.04-cargo`, which `supply-chain` (`:273-276`) and `coverage`
  (`:347-350`) share.
- `xtask/src/main.rs:303-332` `run_cargo_nextest` — `nextest run --workspace --profile ci --no-tests=pass --message-format
  libtest-json` (the `cargo xtask test` body).
- `xtask/src/main.rs:647-670` `run_perf_slo_load` — `nextest run -p pulse-app --test perf_slo_10k_spans --profile ci`.
- `xtask/src/main.rs:334-355` `run_cargo_llvm_cov` — `llvm-cov nextest --workspace --lcov … --ignore-filename-regex
  COVERAGE_IGNORE_FILENAME_REGEX` (the TEMPORARY xtask exclusion, `:332`).
- `xtask/src/main.rs:357-…` `run_ci_gates` — reads the resolved log dir's `agent-latest.jsonl*`, so it must run in the job
  whose boot wrote those logs.
- `xtask/src/main.rs:65,81,91` — `smoke:gap-resume`, `smoke:external-resolve` and `smoke:hue-shift` are all registered
  `Cmd` names (dispatch `:223`, `:244`, `:256`). **CARRY E's premise holds:** the verbs exist, and only the arch
  registry lags (`architecture.md:241` names `smoke:hue-shift` alone).
- `.config/nextest.toml` — `[profile.default] default-filter = "not binary(perf_load_profiles)"`, so
  `perf_slo_10k_spans` IS collected by the workspace `ci` run. The cold log confirms it: `2398 tests across 115 binaries
  (1 binary skipped via profile.default.default-filter)`.
- `pulse-app/tests/quality_gate_workflow.rs` (393 lines) — pins these parts of the workflow shape:
  - `data_dir_export_precedes_every_consumer` (`:348`) asserts `("ci.yml", 3)` jobs, with Harden runner first and the
    DATA_DIR export second in each.
  - `ci_workflow_test_gates_no_continue_on_error` (`:147`) scans every step block against 12 gate substrings.
  - `ci_workflow_uploads_logs_artifact_unchanged` (`:260`) pins `name: logs-${{ runner.os }}`.
  - Coverage-baseline and criterion steps are pinned by name (`:108-135`, `:221-259`).
- `pulse-app/tests/a11y_perf_workflow.rs` (outline) — pins `cargo xtask test:a11y`, `cargo xtask perf:slo-load`,
  `verify:capability-matrix` and `capability-widening-check` by `contains`. It pins `npm run build --prefix pulse-app/ui`
  appearing BEFORE `cargo xtask test:a11y` by string position (`:44-58`), the `a11y-violations-` and
  `playwright-a11y-report-` artifact prefixes, SHA-pin discipline, and workflow-level `contents: read`.
- `scripts/agent-run.sh:32-56` — port defaults `${ANDROMEDA_PULSE_OTLP_GRPC_PORT:-4317}` / `_HTTP_PORT:-4318`; boot
  exports `ANDROMEDA_PULSE_DATA_DIR` / `_LOG_LEVEL` / `RUST_LOG`, then pre-builds. `git ls-files -s` → mode `100755`.
- `crates/triage/build.rs:51-53` — the workspace's only `rerun-if-env-changed` pair (`ANDROMEDA_LLAMA3_TOKENIZER_SHA256` /
  `_PATH`). Boot exports neither, so boot's release relink is not an env re-fingerprint of a build script. The feature
  unification of `--bin pulse-app` vs `--workspace` is the unmeasured hypothesis (open question 3).
- `Cargo.toml:252-255` `[profile.release] lto = "thin"`, `codegen-units = 1`, `strip = true` — every release link is slow.
  `pulse-app/Cargo.toml:71-74` `[features] default = []`, `mcp-server`, `otap-ingest`.
- `Swatinem/rust-cache` `action.yml` + `README.md` at the pinned SHA `23869a5…` (`gh api …/contents?ref=`) — caches
  `~/.cargo` and `./target` **dependency** artifacts. Workspace crates are not cached (`cache-workspace-crates` default
  false). `cache-on-failure`: "Cache even if the build fails. Defaults to false." `save-if` defaults true.
  `add-job-id-key` defaults true (overridden by `shared-key`).
- Repo cache state (`gh cache list` + `gh api …/actions/cache/usage`) — **5 caches, 9 267 495 585 B active, against the
  10 GB repository cap.** Windows 2 552 MB (created 10:06), macOS 2 067 MB (10:21), `ubuntu-22.04` 1 580 MB (09:00),
  Linux 2 632 MB (15:19), gitleaks 5 MB. All sit on `refs/pull/39/merge`.
- Warm-run cache lines (ci#36593334009): all four rust-cache jobs log `Restored from cache key … full match: true`, then at
  post `Cache up-to-date`. **A full-match restore never re-saves**, so a key saved once stays frozen until its lockfile
  or toolchain hash changes. Two consequences:
  - `coverage` restores `supply-chain`'s release-shaped cache (saved 09:00), so its instrumented build is never cached:
    16–17 min every round.
  - The macOS/Windows caches date from ~10:00. Warm macOS clippy still logs `Compiling libduckdb-sys v1.10505.0` (the
    bundled C++ build), where ubuntu logs `Checking libduckdb-sys`.
- Viola precedent (read-only, another repo):
  - `D:/dev/projects/viola/scripts/wsl-exec.sh` — `wsl.exe -d Ubuntu --exec /usr/bin/env -i HOME=… PATH=…`, with
    `MSYS2_ARG_CONV_EXCL='*'` and a `--probe` mode proving that only HOME and PATH cross.
  - `crates/viola-e2e/src/harness/pre_push/linux.rs` — named Stop reasons (`tool-missing`, `tool-pin-mismatch`,
    `sync-failed`, `sync-mismatch`, `linux-document-unreadable`). It keeps a distro-side clone synced from the working
    tree as ONE binary patch through a temporary index, at the same HEAD, verified by tree-id equality. Tool pins are
    read from ci.yml, the clone's `target/` is capped at 40 GiB (`pre_push.rs:28`), and the verdict is one JSON document.
  - `scripts/wsl-provision.sh` — installs ci.yml's pinned Node under the distro home.
- WSL host probes (this session):
  - Distros: `Ubuntu` (WSL2, `Ubuntu 26.04.1 LTS`, kernel 6.6.87.2) and `docker-desktop`. No `~/.wslconfig`, so NAT
    networking is the default.
  - The distro has 32 cores and 31 GiB RAM, and its disk image lives on `C:` (151 GB free, so it does not draw on D:'s
    103 GB).
  - Tools present: rustup with `1.98.1` default plus `1.95.0` (auto-installed by this research's `rustup toolchain list`
    probe, run from a cwd carrying `rust-toolchain.toml`), cargo-nextest, cargo-llvm-cov, gcc, xvfb-run, Node `v24.21.0`
    under `~/.local/viola-node/bin`.
  - **Absent:** every Tauri/dbus apt package ci.yml installs (`dpkg -l` finds none of the 9). `libwebkit2gtk-4.1-dev` is
    available (candidate `2.52.6-0ubuntu0.26.04.1`). **`sudo -n true` → interactive authentication required**, so
    installing the libraries is a one-time operator step.
  - CI's image is `ubuntu-22.04`; the distro is 26.04.

## Graph impact (from the code-graph query)
Trace: `.andromeda/runs/2026-09-29T15-54-46Z-phase/tree-query-2026-09-29-ci-wall-time-and-round-trips.json`, rust plane,
`db_state: fresh`.
- **`run_perf_slo_load` / `run_cargo_nextest` / `run_cargo_llvm_cov` / `run_ci_gates` / `run_npm_script`** — 7 caller rows,
  every one `xtask main()` (the `Cmd` dispatch at `xtask/src/main.rs:223-282`). Changing any of these bodies changes one
  CLI verb, with no other call site.
- **Name existence** `pre_push%` / `PrePush%` / `wsl%` / `Wsl%` — 0 rows, so a new `pre_push` module or `Cmd::PrePush…` is
  free.
- **`crate_edges` for `xtask`** — `xtask → ingest` only (1 row). Every xtask compile compiles `ingest` and its tonic/prost
  graph, which is why a cold `cargo xtask typecheck` cost 11.4 min.

## Patterns detected
- **Compile-dominated gates** (the tables above): the suite runs in ~30 s while its compile takes 7–14 min. The lever is
  the NUMBER of distinct (profile × feature-set × package-scope) compiles on the critical path, not test time.
- **`-p` vs `--workspace` feature unification** (test-plan §3, measured at 2026-08-15-tier-1-incident-path-investigation):
  a `-p pulse-app` selection compiles a different unit graph from a `--workspace` build. `perf:slo-load` pays exactly this
  compile, 9m52s cold / 6.1 min warm, for one test that `cargo xtask test` already ran under the same `ci` profile.
- **A step as its own named gate**: every gate is a plain named `run:` step. `a11y_perf_workflow.rs` /
  `quality_gate_workflow.rs` pin verbs by substring, so a verb can move jobs while those pins still pass. The job COUNT,
  the `logs-${{ runner.os }}` artifact name and the build-before-a11y ORDER are pinned structurally.
- **Per-job prelude** (`ci.yml:27-60`): harden-runner → DATA_DIR export → checkout → toolchain → rust-cache → tool
  install → (Linux) apt libraries → Node → `npm ci` → `npm run build`. Any new job repeats this prelude, which costs
  ~2–3 min on a warm cache (apt 0.4–0.8 min, cache restore 1.4–2.0 min).
- **Companion sweep** (`grep -rln -E "ci\.yml|perf:slo-load|perf_slo_10k_spans" pulse-app/ xtask/ crates/ --include=*.rs`):
  5 hits · 3 changed · 2 no-change. The two workflow test files carry the pins, and `xtask/src/main.rs` carries
  `run_perf_slo_load`. The two no-change files:
  - `pulse-app/tests/perf_slo_10k_spans.rs` defines the test, which is re-selected, not edited.
  - `pulse-app/tests/perf_load_profiles.rs` is the release-gate load suite; it only mentions the name, and nothing in it
    changes.

## Conventions to follow
- **The xtask verb contract** (arch §Occupied Resources → xtask CLI surfaces; `check:npm-supply-chain`,
  `harness:status`, `check:staged-artifacts`, `smoke:hue-shift`): exits 0 green · 1 red · 2 cannot-evaluate, named
  verdict arms, one pretty-JSON verdict on stdout, and an optional report twin under `target/<name>/`.
- **Nextest narrowing** is `-E`/`--filter-expr` under `--workspace`, never `-p` (test-plan §3, `run` verb note).
- **Workflow self-lint**: every job starts `Harden runner` → `Export ANDROMEDA_PULSE_DATA_DIR`
  (`quality_gate_workflow.rs:348-392`), and no gate step carries `continue-on-error` (`:147-186`).
- **Action pins**: 40-char SHA plus a version comment (`a11y_perf_workflow.rs:103`); workflow-level `permissions:
  contents: read` (`:135`).

## New files to create
- `xtask/src/pre_push.rs`
  - the WSL Linux pre-push verb module: distro probe, tool pins read from ci.yml, a clone synced by binary patch at HEAD
    with tree-id equality, the Linux gate run inside the clone, one JSON verdict, exits 0/1/2

## Files to modify
- `.github/workflows/ci.yml`
  - the job split, per-job cache keys and `save-if`, `cache-on-failure`, the duplicate-rebuild removals, and per-job
    artifact names
- `xtask/src/main.rs`
  - the `Cmd` variant and dispatch for the pre-push verb
  - `run_perf_slo_load` re-pointed from `-p pulse-app` to `--workspace -E` so it reuses the workspace test build
- `pulse-app/tests/quality_gate_workflow.rs`
  - the `("ci.yml", 3)` job-count pin and the `logs-${{ runner.os }}` name pin follow the new layout
- `pulse-app/tests/a11y_perf_workflow.rs`
  - the build-before-a11y order pin follows the a11y step into its job

## Scope premise closure (P3 → scope.md amended)
- A `[inferred]` macOS/Windows timing → **corrected**: measured at 33.0 / 42.4 min cold and 33.4 / 37.1 min warm. After
  an ubuntu-only split they are the critical path (33–37 min) unless they are split or trimmed too.
- CONTEXT "~2 h per round" (measured-marked; spot-checked) → **corrected**: true of the cold round only. The warm round is
  44.8 min, and the 8 red P-025 rounds were all cold because of B.
- B `[inferred]` save-on-success-only → **verified**: `cache-on-failure` defaults to false (action.yml at the pinned
  SHA); the cold run logged `No cache found`, and the Linux key's first save came at 15:18 on the first green ubuntu leg.
- B `[inferred]` parallel jobs racing one key → **corrected**: the measured defect is FREEZING, not racing. A full-match
  restore never re-saves, so `coverage` rides `supply-chain`'s release-shaped cache and is never warm, and the
  macOS/Windows keys have been frozen since ~10:00. The 10 GB repo cap is 93 % used, so a naive per-job key set evicts.
- C `[inferred]` duplicate pairs → **partly verified, partly corrected**:
  - `perf:slo-load` is a true duplicate compile of a test the workspace run already executes (verified).
  - Linux `cargo build --workspace --release` duplicates `supply-chain`'s `cargo auditable build --workspace --release`:
    same profile, scope and features, and the auditable build also embeds the dependency tree (verified by reading both
    commands; not a measured artifact identity).
  - `test` vs `nextest --features mcp-server` is NOT a byte duplicate: the feature set differs, 2398 vs 2433 tests, and
    default features are the shipping configuration (open question 1).
  - `release` vs the boot binary: boot compiles `--bin pulse-app --release` again after the workspace release build (7.7
    min warm). The cause is unmeasured (open question 3).
  - `mcp build` (release) vs "the bindings regen": no CI step regenerates bindings under the feature, so that pair does
    not exist (corrected). The mcp release build is the only CI build smoke of the feature-gated binary.
- D `[inferred]` the overseer's recount → kept verbatim as the target list; not re-derived (a recount of past rounds, not
  a HEAD claim).
- D `[inferred]` the WSL distro and toolchain → **partly verified**: the distro, Rust, nextest, Node 24 and xvfb are
  present; the Tauri/dbus apt libraries are absent and sudo needs a password (operator provisioning); the distro is
  Ubuntu 26.04, while CI runs 22.04.
- Host-constraint `[inferred]` networking → **verified**: there is no `.wslconfig`, so NAT is the default and WSL's
  loopback is not Windows' loopback. `agent-run.sh` honours the port overrides, so a WSL boot need not use 4317/4318.
  Whether a WSL boot counts as "starting pulse-app" under the operator's directive is the operator's call (open
  question 2).

## Open questions
- Should ubuntu keep BOTH test runs, the default-features `cargo xtask test` and `nextest --features mcp-server`? Each
  costs ~7 min warm, and in separate parallel jobs neither lengthens the round. Or is the Linux default-features run
  dropped because macOS/Windows run the default-features suite? → blocks: plan-decision
- The WSL pre-push boot arm under the standing host constraint: build the arm and exercise it only after the operator
  releases the constraint (on overridden ports), or ship the verb without a boot arm this chunk. → blocks: plan-decision
- Boot's `--bin pulse-app --release` recompile after `--workspace --release`: is it feature unification, and would
  building `--workspace --release` in the boot job first let boot's pre-build no-op? Measured by one CI round's
  `cargo build -v` fingerprint lines, or by the WSL clone once provisioned. → blocks: implementation-scope
