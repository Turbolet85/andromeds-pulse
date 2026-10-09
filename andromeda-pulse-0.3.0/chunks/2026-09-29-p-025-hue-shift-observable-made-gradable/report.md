# Report — 2026-09-29-p-025-hue-shift-observable-made-gradable

**Chunk:** P-025 hue-shift observable made gradable. `hue_update_ms` now measures the interval from the tier-effective
instant to the paint, per changed service (the Conductor contract). The CI workflows parse again.
**Date:** 2026-09-29
**Commits** (`git log --format='%h %s' f15536b..HEAD`; the basis is the parent of the oldest pre-CI commit):
- `f2a131a` chore: operator pre-CI commit. The chunk's own work: hue fix, workflow parse fix, leg, rustls/quinn-proto
  bumps, dev debuginfo profile.
- `200e5ed` fix: first real CI run — build prerequisites, wasmtime advisory, fixture allowlist.
- `edfd8b3` fix: CI Node matches the lockfile's npm; dbus before the xtask gate.
- `28c3238` fix: PNG icons are RGBA so `generate_context!` accepts them on macOS/Linux.
- `e904cb1` fix: CI a11y browsers, npm advisories, dead ignored test.
- `33abd9b` fix: distinct injector salts on coarse clocks; Linux boot smoke under xvfb.
- `46c3aac` fix: `harness:status` reports a dead app as not-running.
- `eac30d9` fix: coverage measure excludes xtask (TEMPORARY, founder ruling).
- `546d3f0` fix: perf-slo-check reads an empty metric stream as NEUTRAL instead of dying.
- `464f2a3` fix: agent-run boot names how a failed app ended; cleanup-on-failure runs.

Scope in one line: the chunk's **product** scope is P-025. Its **CI** scope became a rehabilitation — the first real CI
run in two months, worked to green over 8 fix pushes under the overseer's CI directive and the founder's rulings.

## Changes (structured — detectors read this)

### Files

*Product and tests*
- `crates/triage/src/incident/{registry.rs, tier_effective.rs (new), mod.rs}`, `crates/triage/src/contract.rs`
- `crates/triage/src/lifecycle/{registry.rs, persistence.rs}`
- `pulse-app/src/services_router.rs`
- `pulse-app/tests/{unit_services_router.rs, quality_gate_workflow.rs, unit_llamacli_inference.rs}`
- `pulse-app/ui/src/widget/{constellation-types.ts, constellation-types.test.ts, ConstellationCanvas.tsx,
  ConstellationCanvas.test.tsx}`
- `pulse-app/ui/src/bindings/index.ts` (regenerated)
- `pulse-app/ui/src/hooks/use-findings.test.tsx`, `pulse-app/ui/src/{csp.test.ts, contrast/parse-tokens.test.ts}`
- `pulse-app/icons/{32x32,128x128,128x128@2x}.png` (palette → RGBA, pixel-identical)
- `crates/ingest/examples/inject_demo.rs`

*Harness and xtask*
- `xtask/src/{hue_shift.rs (new), main.rs, external_resolve.rs, harness_status.rs}`
- `xtask/ci/perf-slo-check.sh`
- `scripts/agent-run.sh` (git mode 100644 → 100755)

*CI and supply chain*
- `.github/workflows/{ci.yml, release.yml, update-channels.yml}`
- `.gitleaksignore` (new)
- `Cargo.toml`, `Cargo.lock`
- `pulse-app/ui/{package.json, package-lock.json, npm-policy.json}`

*Audit trail*
- The phase run dir `control-e14.py`: a host path was replaced by `$SHELL`.

### Symbols / APIs

*Triage and the services resolver*
- **new** `IncidentRegistry::list_for_workspace(&str) -> Vec<Incident>`, returning all statuses including Resolved.
  It is a trait method; its sole implementor is `InMemoryIncidentRegistry` (grep `impl IncidentRegistry for` → 1).
- **new** `triage::contract::tier_effective_at(&[Incident], service) -> Option<i64>`, a pure replay:
  - rise: the opening incident's `opened_at_unix_nano`;
  - fall: the last max-tier holder's `resolved_at_unix_nano`;
  - Acknowledged keeps its tier.
- **new field** `ServiceListItem.tier_effective_at_unix_nano: Option<i64>`, `#[serde(default)]`. TS renders it as
  `tier_effective_at_unix_nano?: number | null`. `list_all` emits `None`; the resolver enriches it.
- **changed** `services_router::list_with_states` reads ONE `list_for_workspace` snapshot. Both `priority_tier` (from its
  non-Resolved subset) and `tier_effective_at_unix_nano` derive from that snapshot. The TauRPC procedure set is
  unchanged: `EXPECTED_PROCEDURES` Δ0.

*Webview*
- **new** `hueShiftSamples(previous, dots, items, mountedAtMs, paintNowMs)` in `constellation-types.ts`: one sample
  per changed tier, emitted only when the change was witnessed (`effective ≥ mount`).
- `ConstellationCanvas` emits once per sample, and re-calls the frame loop's `start()` on a data change under reduced
  motion. The P-027 effect is byte-untouched.
- The emitted leaf `metric.constellation.hue_update_ms {duration_ms, severity_tier}` is unchanged in NAME and FIELDS.
  Only its value's meaning changed: paint instant − `tier_effective_at`.

*xtask*
- **new** `smoke:hue-shift` (`xtask/src/hue_shift.rs`): exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE; the verdict keys on
  anchor error ≤ 1000 ms. `external_resolve` helpers were widened to `pub(crate)` with no behaviour change.
- **changed** `harness:status::classify(pid, alive, newest_log)`: a dead pid is `not-running` whatever the log says.
  The probe is `ps -o stat=` on Unix (a zombie counts as dead) and `tasklist` on Windows. The verdict JSON contract is
  unchanged.
- **new const** `COVERAGE_IGNORE_FILENAME_REGEX` (xtask `test:coverage`, TEMPORARY).
- **changed** `inject_demo::fresh_run_salt` now goes through a pure `mint_salt(pid, nanos, nth)` with a process
  counter.

*Harness scripts*
- **changed** `agent-run.sh boot`: the failure path reports how the app ended (the signal name, its own exit status,
  or still running) and runs `bash "$0" cleanup`.
- **changed** `perf-slo-check.sh`: an empty metric stream reads NEUTRAL; the budgets are unchanged.

*Tests*
- Deleted: `manual_subprocess_timeout_smoke`, an `#[ignore]`d test with an empty body.

### Crates / modules
- triage gains the module `incident::tier_effective`. No new crate. No new dependency edge.

### Dependencies
- **Rust** (`Cargo.lock`):
  - rustls 0.23.40 → 0.23.45 (RUSTSEC-2026-0285)
  - rustls-webpki 0.103.13 → 0.103.15
  - quinn-proto 0.11.14 → 0.11.15 (RUSTSEC-2026-0185; 0.11.16+ would add rand 0.10)
  - wasmtime 46.0.3 → **48.0.3** (RUSTSEC-2026-0316; the `Cargo.toml` requirement is now `"48.0.3"`; 49.0.1 needs
    Rust 1.96, and the toolchain is pinned at 1.95)
- **npm, dev-only**:
  - vitest 3.2.7 → 4.1.11 (@vitest/mocker 4.1.11, GHSA-82fw)
  - qs → 6.16.0 (GHSA-4mjr, GHSA-x5fp)
  - undici → 6.29.0 (GHSA-3wwx)
  - webdriverio → 9.32.0 (deepmerge-ts 8.0.2)
  - @types/node ^24 added

### Schema / config
- `[profile.dev] debug = "line-tables-only"` and `[profile.dev.package."*"] debug = false`. Measured: one workspace
  test build went from ~95 GB to 19.0 GB of `target/debug`.
- npm policy:
  - **added** GHSA-7pqw-9j4j-h8q3 (extract-zip). Range `*`; 2.0.1 is the latest release; npm's remedy is a pa11y-ci
    downgrade; the chain is dev-only.
  - **pruned** GHSA-ggr8-5vv4-36mx, because its closing condition fired (deepmerge-ts 8.0.2).
- `.gitleaksignore`: two fingerprint entries (commit:file:rule:line) for the PII-canary placeholder
  `REDACTME-SECRET-1234` in the planning run `2026-05-11T00-39-00-phase-41` (`tests.md:26`, `.raw-tests.md:28`).

### Spec-master edits
- none. The seven masters are untouched; `git diff --name-only f15536b -- .andromeda/*.md` shows only
  `master-route.md`.

### Counts / qualifiers moved
- Workspace Rust tests: 2392 run, all passed (nextest `Summary`, `4.log`). This chunk added 28 Rust tests; HEAD's 2364
  is inferred, not measured.
- Webview tests: 848 passed across 81 files (vitest, post-vitest-4).
- xtask tests: 176.
- The only `#[ignore]` in the codebase was removed: `grep -rn '#\[ignore' --include=*.rs crates pulse-app xtask` →
  0 attribute hits.
- Coverage (CI lcov): line 84.3 % / function 83.95 % → with the xtask exclusion 88.6 % / 87.80 %
  (`coverage-linux` artifact of run e904cb1).

### Dev-tool versions
- Playwright chromium-headless-shell build **1217** installed on the dev host (for @playwright/test 1.59.1; the host
  had 1243). The npm package @playwright/test is NOT this line's subject.
- CI `setup-node` 22 → **24** (npm 10 → 11, matching the dev host that writes the lockfile: Node 24.13.1 / npm 11.8.0).
- `xvfb` added to the Linux apt set.

### Harness / gate surface

*`ci.yml`*
- Workflow- and job-level `env:` no longer references `runner.temp`. Every job exports `ANDROMEDA_PULSE_DATA_DIR` from a
  step right after harden-runner (ci ×3, release ×1, update-channels ×2).
- Linux system libraries (Tauri/WebKitGTK/dbus/xvfb) install and `ui/dist` builds before the first compile, in
  lint-test-build, supply-chain and coverage.
- The four xtask drift checks run BEFORE `cargo xtask test`: the default-features test run rewrites the bindings
  without `mcp`.
- The deny action runs `command: check bans licenses sources` (it had been running every check).
- Playwright chromium is installed before `test:a11y`.
- The boot smoke runs inside one `xvfb-run`.

*`release.yml`*
- Node 24.

*Guards and legs*
- New workflow self-lint guards: `workflow_env_references_no_step_only_context` and
  `data_dir_export_precedes_every_consumer`.
- New scenario leg: `smoke:hue-shift`.

*Verdict shapes*
- The `harness:status` liveness rule, the `agent-run.sh` boot failure diagnosis, and the `perf-slo-check` NEUTRAL fix
  are all listed under Symbols / APIs above.

### Cross-project / external claims
- **Conductor contract** `D:/dev/projects/conductor/contracts/pulse-p025-measurement-contract.md` (read at /phase).
  Its fall-source and acknowledgement clauses were premise-corrected in `scope.md`.
- **CI, PR #39** (draft; never merged or closed by the builder):
  - `f2a131a` red `checks 6/6` in 5 s: every job was "not started", blocked by account billing (run
    `ci#36535320175`).
  - After the repo went public, successive runs went red on real failures (listed in Deviations).
  - **FINAL `464f2a3`: `verdict: green · checks 6/6`**. Runs `ci#36574279289` (success) and
    `secret-scan#36574279070` (success).
- **GitHub context table** (fetched at /phase): `runner.*` is unavailable in workflow- and job-level `env:`.

### Reverted / negative API facts
- `--features mcp-server` on the coverage run was measured and NOT adopted: local 83.75 %, no gain.
- The `vitest 3` path was abandoned because its mocker is vulnerable (GHSA-82fw).

### Insufficient fixes (written, kept, not the remedy)
- The xvfb boot smoke: it removed the GTK-init panic, but on `546d3f0` the app still died silently ~0.5 s after its
  webview began polling. The cause was not identified and did not recur on `464f2a3`. Owner: the WATCH below.
- The new boot diagnosis names the signal or exit status if it recurs.

### Spec claims disproved by measurement

*Contract and relay*
- The Conductor contract's fall source (the broadcast's `transitioned_at`) and "acknowledgement ends the tier" — both
  false at HEAD. Premise-corrected in `scope.md` at /phase.
- The directive's option "job-level `env:`": the context table says no. Premise-corrected in `scope.md`.
- The plan's CONTEXT "the U05 reflow is the first step": `cargo fmt --check` was already green at HEAD. Premise-corrected
  in `scope.md`.

*Plan gates*
- **Plan gate `cargo audit`** expects `exit 1` + `duplicate advisory ID: RUSTSEC-2026-0244`. Measured: `cargo audit`
  LOADS the DB (1273 advisories), exit 0 after the bumps. **Pin #22 is DISCHARGED**: re-pin the gate, and retire the pin
  by route-resolve's rule.
- **Plan gate `git -C "$HOME/.cargo/advisory-db/advisory-db-3157b0e258782691" fetch --quiet`** reads a copy 359 commits
  behind origin (HEAD 2026-05-01). `cargo audit` reads `D:\dev\rust\cargo\advisory-db` (CARGO_HOME). Its green proves
  nothing, so the gate's path must be fixed.

*`agent-run.sh` and harness*
- **`agent-run.sh`'s boot failure path** ran `"$0" cleanup` with git mode 100644. That branch never ran on a checkout
  (CI exit 126).
- **`harness:status`**: liveness from pidfile + log freshness called a crashed app healthy for 60 s. Measured on CI
  run e904cb1's boot smoke (a 12 ms panic, printed `running-healthy`).

*Plan, research, security-plan and CI*
- **Plan acceptance "no `Cargo.toml` changes"**: broken — the dev profile, wasmtime 48.0.3. There is still no new
  dependency edge.
- **Research "the recorded runs' `checks 0/0` are the workflow-file signature"**: true for the parse error. After that
  fix, the next red (`checks 6/6`, 5 s) was account billing, not the code.
- **security-plan's `cargo audit` standing-deferral text** ("cannot LOAD the RustSec DB") — now false.
- **The CI perf-budget gate is vacuous**: `ci-gates` reads only the boot-smoke log (52 records, 0 frame / memory /
  snapshot samples). It reads NEUTRAL and cannot fail (relay part 2 item 3).

### Expected amendments (from plan)
Master hit counts come from `grep -c` per master; the patterns are the quoted tokens.
- **obs-plan §8, `metric.constellation.hue_update_ms` interval text** (`hue_update_ms`: obs-plan 2 · architecture 1 ·
  layout-templates 1) — carried (Symbols / APIs, leaf meaning). The new text: paint instant −
  `ServiceListItem.tier_effective_at_unix_nano`, one record per changed service, witnessed changes only. Keep the
  metric-fallback clause.
- **architecture §Occupied Resources, `telemetry.frontend.record_constellation_hue_latency` interval text**
  (`record_constellation_hue_latency`: architecture 1) — carried (same fact).
- **architecture §Occupied Resources: xtask CLI surfaces gain `smoke:hue-shift`; the agent-run entry's "workflow-level
  at `ci.yml:12`" becomes the per-job `$GITHUB_ENV` export** (`ci.yml:12`: architecture 1 · test-plan 2) — carried
  (Harness / gate surface).
- **test-plan §9 Boot smoke row: the `ci.yml:12` mention becomes the per-job export** (same grep) — carried. It also now
  runs under xvfb.
- **test-plan §3 Scenario legs registers `smoke:hue-shift`** (dev-host only, not CI-wired, anchor-error verdict,
  INCONCLUSIVE preconditions). The token `Scenario legs` has 0 hits in test-plan, while `smoke:external-resolve` has
  test-plan 1, so the site is that registration's section — carried.
- **test-plan §9 "runs in CI today" rows become measured true** (`runs in CI today`: test-plan 1) — carried. Run
  `ci#36574279289` GREEN 6/6 on `464f2a3`.
- design-system / layout-templates: not carried. The plan says their status notes already name the dot hue.

Beyond the plan's list, facts for P2 from rulings and measurements:
- **test-plan §10 coverage**: the xtask exclusion is TEMPORARY. Founder, 2026-09-29, verbatim: «Давай исключим но
  временно, после конца эпохи и код анализа посмотрим как качественно покрытие увеличить а то 85 процентов маловато так
  то». The review point is the next epoch-boundary code audit: coverage quality, thresholds above 85 %, xtask
  inclusion. Grep `85% function|≥ 85%` finds test-plan 4.
- **security-plan §Dependency Security**: pin #22 discharged (`cargo audit` hits: security-plan 6; `RUSTSEC-2026-0244`:
  security-plan 1). Also: wasmtime 48.0.3 (`wasmtime 46|"46"`: architecture 3 · security-plan 1); RUSTSEC-2026-0285 /
  -0185 / -0316 closed; npm GHSA-7pqw accepted as the one residual (no fixed release anywhere; dev-only via pa11y-ci);
  GHSA-ggr8 exception pruned.
- **verification-harness and test-plan §3 status arms**: `harness:status` now includes pid liveness (`running-healthy`:
  architecture 1 · test-plan 5). The agent-run boot failure path names the exit reason.

### Coverage of new surfaces
- `IncidentRegistry::list_for_workspace` → validation n/a · instrumentation n/a · PII n/a · tests unit ·
  a11y n/a · tokens n/a
- `ServiceListItem.tier_effective_at_unix_nano` (IPC payload field) → validation n/a (backend-produced) ·
  instrumentation via `services.list_with_states.request` · PII n/a (an instant) · tests unit + integ · a11y n/a ·
  tokens n/a
- `hueShiftSamples` + the canvas fire site + the reduced-motion repaint → instrumentation `hue_update_ms` leaf
  (unchanged) · PII no service id on the leaf · tests unit (mutation-checked ×4) · a11y reduced-motion repaint
  unit-proven; the live leg ran default motion only · tokens n/a (no literal added)
- `smoke:hue-shift` → tests unit (10) + live RED/GREEN · the rest n/a
- `harness:status` liveness → tests unit (dead-pid, exited-child, parsers; mutation-checked ×2) + live kill check ·
  the rest n/a

## Deviations from intent
- **The operator pass became a CI rehabilitation** (overseer CI directive + founder "no deferral"). 8 fix pushes, each
  a failure class. The founder made the repo public for free Actions.
- **Founder rulings recorded:**
  - in-chunk advisory upgrades (rustls, quinn-proto; wasmtime later);
  - Playwright browser install;
  - harness liveness fixed in-chunk;
  - coverage exclusion TEMPORARY (verbatim above).
- **Overseer URGENT** (D: at 1.8 GB free): `cargo clean --profile dev` freed 190.6 GiB. Cause found: full dev
  debuginfo PDBs (114 GB) plus a second generation after a lockfile bump. Fixed with the dev profile.
- **Leg design**: the finite storm is stopped once the rise paints (the plan left await-vs-stop open). The rise is
  matched to the nearest preceding creation record when several exist. The first-draft verdict skipped the fall on a
  failing rise; fixed to grade both.
- **Order**: steps 4–9 were written while the HEAD release build and the RED leg ran. The measured binary was proven
  HEAD by content (the new field 0×, the embedded dist hash matched).
- **Scope record** — `gate.py scope`: clean · changed 37 · listed 20 · recorded 17. All 17 are **widenings**:
  - `Cargo.lock`, `Cargo.toml` — founder (advisory upgrades) + overseer (debug profile, wasmtime).
  - `.gitleaksignore`, `pulse-app/icons/*.png` ×3, `unit_llamacli_inference.rs`, `pulse-app/ui/{package.json,
    package-lock.json, npm-policy.json}`, `use-findings.test.tsx`, `parse-tokens.test.ts`, `csp.test.ts`,
    `inject_demo.rs` — overseer CI directive.
  - `xtask/src/harness_status.rs` — founder ruling.
  - `xtask/ci/perf-slo-check.sh`, `scripts/agent-run.sh` — overseer.

## Decisions & corrections
- **Founder (coverage), verbatim** above: exclude xtask TEMPORARILY; revisit at the epoch-boundary code audit.
- **Founder (licensing, relay part 2), verbatim**: «надо будет добавить лицензии апачи мит» → `MIT OR Apache-2.0`, the
  next route entry.
- **Relay route order**: dual license → P-027 discovery (first rise 9986 ms from the 15 s registry tick) → perf-budget
  vacuity → CI wall time (only on the founder's yes) → Conductor return.
- **Sweep hazards**:
  - llvm-cov function% counts each compiled COPY of a function, so a library tested in its own crate reads uncovered in
    the copies linked elsewhere.
  - `grep … | sort` under `set -o pipefail` dies on an EMPTY stream (perf-slo-check).
  - `ps`/`kill -0` read a zombie as alive; check the `Z` stat.
  - macOS `SystemTime` ticks in whole µs, so the clock is not a uniqueness source.
  - npm 10 vs 11 disagree on unmet OPTIONAL peers, so `npm ci` refuses a lock written by npm 11.
  - Windows builds embed `.ico`, so palette PNG icons fail only on macOS/Linux `generate_context!`.
  - vitest 4's `restoreAllMocks` no longer clears `vi.fn()` history.
  - `cargo audit` reads CARGO_HOME's advisory-db, not `$HOME/.cargo`.

## Outcome
**Acceptance, re-asserted against the diff**
- **Observable leaf unchanged in name and fields, one record per changed service, no service id**: MET (canvas +
  `hueShiftSamples` pins; the allowlist guard is unchanged and green).
- **`tier_effective_at` rise/fall/ack rule + consistency pin**: MET (11 `tier_effective` tests).
- **Live leg**: MET.
  - RED at HEAD: FAIL, rise anchor error 9229 ms.
  - GREEN: PASS, rise 44 ms / fall 25 ms.
  - Re-run after wasmtime 48: PASS, 36 / 28 ms.
  - Measured first rise: 9986 ms. That is context only (Conductor grades it) and routes to the P-027 entry.
- **Reduced-motion repaint**: MET (unit only; the live leg ran default motion).
- **a11y suite lists**: MET (41 tests / 18 files).
- **Design** (no halo claim, no new literal): MET.
- **Capabilities, bindings, staged**: MET (`EXPECTED_PROCEDURES` Δ0, capability JSON byte-identical, bindings carry
  the field).
- **"No new dependency edge"**: MET. **"No `Cargo.toml` changes"**: **UNMET** (profile + wasmtime) → P2 escalation
  for acknowledgement.
- **Workflow env guard + export position**: MET (guard mutation-checked).
- **CI read green with real checks**: MET — `ci#36574279289` GREEN 6/6 on `464f2a3`.
- **Rust-gate PREREQ discharged**: MET (clippy + workspace nextest green).
- **Pin #22 between-point**: superseded — the pin is DISCHARGED (above).

**Gates** — the final `/implement` state, then the operator pass
- `cargo fmt --all -- --check` — green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green.
- `cargo build --workspace --tests --jobs 4` — green.
- `cargo nextest run --workspace --profile ci` — green (2392).
- nextest collection witness — green (26).
- `npm run lint` · `typecheck` · `test` — green.
- The bindings-regen nextest — green.
- `grep -c tier_effective_at_unix_nano …` — green.
- `npm run build` — green.
- `cargo build -p pulse-app --release` — green.
- `cargo xtask self-verify` — green (boot + a11y halves, after the browser install).
- Precondition probe — green.
- `cargo xtask smoke:hue-shift` — leg live, driven by hand: PASS.
- Playwright `--list` — green.
- The advisory-db `fetch` probe — green, but VACUOUS (reads the wrong copy; above).
- `cargo audit` — **red · `exit 1` expected, got 0** → the pin #22 discharge. Owner: this wrap re-pins the gate (P5).
- `cargo deny check advisories` — green.
- `cargo deny check bans licenses sources` — green.
- `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` — green.
- Operator `gate.py hygiene` — green after the `control-e14.py` path repair.
- Operator push — done.
- Operator draft PR — #39.
- Operator `ci.py conclusion` — **green on `464f2a3`**, recorded in `evidence/operator-pass.md`.

**Watches**
- **WATCH (Linux boot)**: on `546d3f0` the app died silently ~0.5 s after its webview polled (hypothesis:
  intermittent). 1 green run since: `ci#36574279289`. The instrument is the agent-run boot failure diagnosis (signal or
  exit status).

**Outcome basis**
- The operator pass commit list above; the final HEAD's CI run `ci#36574279289` in `evidence/operator-pass.md`.
- Evidence files: `evidence/{red-leg-at-head.md, green-leg.md}`.

**Process hygiene** (re-measured with `Get-Process pulse-app,inject_demo,cargo,rustc`): none running. The implement
census recorded that every leg, self-verify app and gate tree was terminated, one of them by the overseer.
