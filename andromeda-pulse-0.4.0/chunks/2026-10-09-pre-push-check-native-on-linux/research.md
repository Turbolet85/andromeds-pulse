# Codebase Research — 2026-10-09-pre-push-check-native-on-linux

## Scope
- **Depth:** moderate · **Reads:** 15 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included
  (it auto-loaded on `xtask/src/pre_push.rs`). Applied: the `pre-push:linux` row under "Scenario legs"; 2026-09-29
  (a child cargo spawned by a `cargo run`-launched tool inherits that tool's package variables, and every spawn goes
  through `cargo_command()`); 2026-10-04 (the `held back` install-script line is not proof the browser download did
  not run; isolate the cache, never delete the shared one); 2026-05-14 (`ci-gates` reads every `agent-latest.jsonl*`
  under the resolved data dir, so the stage is given a fresh one). `.claude/rules/testing.md` auto-loaded with it:
  2026-10-04 (a clean skip is invisible to nextest's counts; `env -i` drops the session bus); 2026-06-05 (a verdict is
  never read through `| tail`).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's directive for this chunk (the pc overseer, 2026-10-09): how the
  six stages have been run by hand, what "the surviving stages" means, the two boot-smoke readings, the scope edge,
  the host load, the stops.

## Files inspected
- `xtask/src/pre_push.rs` (full, 736 lines) — the whole verb: the host guard (`:146-148`), the hop (`:23-24`,
  `:274-292`, `:315-328`), the clone (`:25-26`, `:394-469`), the pins it reads (`:504-566`), the provisioning probe
  (`:332-389`), the six stages (`:185-257`), the seed (`:44-51`, `:472-483`), the verdict document (`:78-136`), ten
  tests (`:600-736`).
- `xtask/src/main.rs` (`:256-266`, `:340-345`, `:360-649`) — the verb's registration and its `about` text, which
  describes the WSL form; `cargo_command()` (`:373-381`); `run_cargo_nextest` (`:444-467`, what the `test` stage
  runs: `cargo nextest run --workspace --profile ci --no-tests=pass --message-format libtest-json`);
  `run_ci_gates` (`:498-586`) with its four printed lines (`ci-gates: zero-spans PASS (…)` `:545`,
  `ci-gates: zero-panic PASS` `:554`, `ci-gates: heartbeat-gap PASS` `:558`, `ci-gates: perf-budget {word}` `:582`);
  the comment at `:570-572` names the seeded record and stays true.
- `xtask/src/rust_floor.rs` (`:55-80`) — `declared_floor_equals_the_pinned_channel` calls
  `crate::pre_push::rust_channel` at `:69`.
- `xtask/src/perf_budget.rs` (`:104-110`) — the verdict words `PASS` · `FAIL` · `NEUTRAL`.
- `xtask/src/source_lint.rs` (`:24-30`) — the five scanned roots.
- `xtask/Cargo.toml` — `tempfile` is already a normal dependency; the native path needs no new crate.
- `xtask/ci/heartbeat-gap-check.sh` (`:45`, `:63-64`) — uses `date`, `python3` as a fallback and `awk`; no `jq`.
- `.github/workflows/ci.yml` (`:1-200`, `:368-392`) — the `lint-test` steps the stages mirror (`npm ci` `:67-69`,
  `npm run build` `:71-73`, the source lint `:84-85`, clippy `:87-88`, `cargo xtask test` `:111-112`); the boot job's
  by-path runs of `scripts/agent-run.sh` (`:385-388`); the two pins the check reads (`node-version: '24'` at `:65`
  and five more; the first `apt-get install` list at `:58-60`).
- `pulse-app/ui/package.json` — `build` is `node scripts/build.mjs`; `pa11y`, `pa11y-ci` and `lighthouse` are the
  devDependencies behind the two puppeteer copies; no `engines` field.
- `pulse-app/tauri.conf.json` (`:7-8`) — `frontendDist: ui/dist`.
- `rust-toolchain.toml` — channel `1.95.0`.
- `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md` (full) — the standard gate set and its
  order; the paragraph that registers `pre-push:linux`.
- `.andromeda/residuals.md` (`:21`) — the carried residual with its four measured facts.
- `andromeda-pulse-0.3.0/chunks/2026-10-04-supply-chain-advisories-on-wasmtime-resolved/evidence/operator-pass.md`
  (the stage rows and deviations) — the hand-run of the six stages, with the puppeteer measurement.
- `andromeda-pulse-0.3.0/chunks/2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement/plan.md`
  (`:382-415`) — the native stage commands as last written: each under
  `env -i HOME="$HOME" PATH="$HOME/.cargo/bin:$(dirname "$(command -v node)"):/usr/local/bin:/usr/bin:/bin"
  ANDROMEDA_PULSE_DATA_DIR=…`, stage 3 with a per-run `PUPPETEER_CACHE_DIR`, stage 6 over a per-run seeded data dir.
- `andromeda-pulse-0.4.0/chunks/2026-10-09-boot-smoke-s-early-exit-found-and-closed/plan.md` (`:118-313`) and
  `report.md` (`:301`, `:318`) — the standard entries' spelling in this version's plans; clippy and the workspace
  suite green on this host today (2826 passed).

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **`pre_push::run`** — 1 caller: `main` @ `xtask/src/main.rs:342`. The verb's entry point; its signature does not
  change.
- **`pre_push::rust_channel`** — 1 caller outside the module: `declared_floor_equals_the_pinned_channel` @
  `xtask/src/rust_floor.rs:69`. It stays, with its signature.
- **Every other function of the module** (`node_major`, `apt_packages`, `script_mode`, `installed_packages`,
  `node_major_of`, `remediation`, `wsl_path`) — 0 callers and 0 references outside `xtask/src/pre_push.rs`
  (`tree-query` trace, rust plane: the calls query returned 2 rows, the refs query 3 groups — the module, `run`,
  `rust_channel`). The module is a leaf of `xtask`; removing or reshaping the rest has no caller to thread.

## Patterns detected
- **A verdict verb with three exits and a report twin** (`xtask/src/pre_push.rs:109-136`): the document is built in
  one place, printed to stdout and written to `target/pre-push/report.json`; stage output is sent to stderr
  (`:290`). The native path keeps this frame.
- **Stage commands as first-party constants** (`pre_push.rs:224-256`): every argv is a literal or a pin read from
  the repository; nothing comes from telemetry.
- **A constructed child environment** (`pre_push.rs:280-288`): `env -i` plus `HOME`, `PATH` and the data dir. The
  hand-run form kept it natively and added `PUPPETEER_CACHE_DIR` for the `npm` stage.
- **A pure decision pinned per arm, probes pinned apart** (`xtask/src/harness_status.rs` `classify`,
  `xtask/src/harness_ready.rs` `decide_ready`; named by the tests extract): the form for the host guard and the
  provisioning verdict.
- **A platform-guard pin and a child-environment pin for a stage-spawning verb** (`xtask/src/perf_frame.rs`,
  `platform_guard` and `child_env`; named by the tests extract).

## Conventions to follow
- **Tests co-located in the crate's own source** (test-plan §2): `#[cfg(test)] mod tests` at the foot of
  `xtask/src/pre_push.rs`, as today (`:600`).
- **Host-independent pins**: a probe takes what it inspects as a parameter; a filesystem fixture is a real
  `tempfile::TempDir` (test-plan §8). No pin reads this host's PATH, package manager or `/usr`.
- **No `std::env::set_var` in a test** (test-plan §11): the stage environment is built by a function that takes its
  inputs as arguments and returns the variable set.
- **Module doc comments avoid a wrapped line that starts with `+`, `-` or `*`** (`rules/testing.md` 2026-06-02),
  and source files stay ASCII where the English-only lint reads them.

## Measured on this host (2026-10-09, `ID=omarchy`, kernel 7.2.5; nothing below was built or run as a stage)
- **The verb today**: `drive` returns `cannot-evaluate` / `not-windows` (`pre_push.rs:146-148`).
- **Tools**: `git`, `cc`, `python3`, `pkg-config`, `jq`, `xvfb-run` all under `/usr/bin`; `rustup` and
  `cargo-nextest` under `~/.cargo/bin`; `node` and `npm` only under the `mise` install (`/usr/bin/node` does not
  exist).
- **Rust**: `1.95.0-x86_64-unknown-linux-gnu (active)`, matching `rust-toolchain.toml`.
- **Node**: `v26.8.2` (the only version `mise ls node` lists), npm `11.19.1`. ci.yml pins `24`. The project rule
  behind the pin is about the npm MAJOR (`rules/security.md` 2026-09-29: npm 10 and npm 11 disagree on a lockfile's
  optional peers); Node 24 and Node 26 both ship npm 11.
- **System libraries**: `pkg-config --exists` answers for `openssl`, `dbus-1`, `gtk+-3.0`, `webkit2gtk-4.1`,
  `libsoup-3.0`, `javascriptcoregtk-4.1`, `ayatana-appindicator3-0.1`, `librsvg-2.0`; `libxdo` is absent and
  nothing links it (`Cargo.lock`: no `libxdo` crate; `ldd target/release/pulse-app | grep -c xdo`: 0).
- **Package manager**: `pacman`; no `dpkg-query`, no `apt-get`.
- **Ignored paths**: `git check-ignore -q` exits 0 for `pulse-app/ui/dist`, `pulse-app/ui/node_modules` and
  `target/pre-push`; exit 1 for `pulse-app/ui/src/bindings/index.ts` (tracked).
- **What the hand-runs left**: `target/pre-push/` holds ten per-run puppeteer caches of 963 MB each, ten data dirs
  and seven small logs, 9.4 GB in all (`du -sh target/pre-push`). Each cache holds a chrome and a
  chrome-headless-shell folder for 148 and for 152, and the 148 zip files.
- **The developer's shell**: carries `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY`, `WAYLAND_DISPLAY`;
  carries no `RUST*`, `CARGO*`, `CC`, `CXX` or `PUPPETEER*` variable.
- **Build-script environment tracking in the workspace's own crates**: `crates/triage/build.rs:52-53` (two
  variables of its own) and Tauri's `TAURI_CONFIG` / `REMOVE_UNUSED_COMMANDS` (the last `pulse-app` build output).
  None of them is set in the developer's shell or in the constructed stage environment. Whether a stage run on the
  shared `target/` rebuilds anything a developer build had current is not measured; the hand-runs recorded no
  stage timings.

## Mechanisms re-derived
- **The `test` stage rewrites the tracked bindings file.** `cargo xtask test` is the default-features workspace
  nextest (`main.rs:444-467`); under default features the `emit_taurpc_bindings` router omits the `mcp` namespace
  and `taurpc` writes `pulse-app/ui/src/bindings/index.ts` on every `Router::into_handler()` (test-plan §3
  Per-chunk gate discipline; `rules/testing.md` 2026-05-13). The frame that produces the write is the test binary,
  whatever environment it runs in, so a constructed environment does not prevent it.
- **The credential-store legs skip in a stage whose environment has no session bus.** The legs print
  `[skip] no OS credential store …` and return (`crates/corpus/src/keychain.rs:535`, `:610`, `:625`); with
  `DBUS_SESSION_BUS_ADDRESS` absent no Secret Service answers. A constructed environment holding `HOME`, `PATH` and
  the stage's own variables is that case, as `env -i` was (test-plan amendment
  `2026-10-04-corpus-key-creation-is-race-free`).
- **`ci-gates` over the seed reads four lines, none of them a fail.** The seed is one `app.boot.ready` record and
  two `ingest.tick` records 15 s apart (`pre_push.rs:44-51`): three records, no panic record, one tick gap of
  15 000 ms against 45 000 ms, no perf sample. The recorded reading of that stage, run natively: "zero-spans PASS (3
  records) · zero-panic PASS · heartbeat-gap PASS (max 15000 ms vs 45000 ms) · perf-budget NEUTRAL" (the operator
  pass above, row 31). The seed's doc comment still names "the perf-budget script's empty-metric-stream path"; that
  script was deleted at chunk 2026-09-30-perf-budget-gate-reads-real-samples and the grader is in-process.
- **`npm ci` does not write the lockfile** (npm's own contract for the command), so the `npm` stage's writes are
  `node_modules` and, through `npm run build`, `ui/dist`.

## New files to create
- none — the native path lives in the module that holds the verb today

## Files to modify
- `xtask/src/pre_push.rs` — the hop and the clone leave; the host guard, the provisioning check, the stage runner, the stage environment and the verdict document become native; the tests follow
- `xtask/src/main.rs` — the verb's `about` text describes the native check

## Open questions
- Which Node major must the host hold for the check to evaluate: ci.yml's pin (24), which this host does not
  have, or the host's own (26)? → blocks: plan-decision — it decides whether the check can read green on this host
  without the operator installing a Node 24, and so whether P-103's verdict is reachable at /implement.
- How is the `test` stage's rewrite of the tracked bindings file undone now that no clone absorbs it: by putting
  back what the stage found, or by a regen stage after it? → blocks: plan-decision — it decides whether the check
  gains a stage and a feature-graph build on every run.
