# Scope — 2026-09-29-ci-wall-time-and-round-trips

**Chunk:** CI wall time and round-trips
**Working-route intent (verbatim outcome):** a CI round stops being the bottleneck: parallel jobs, a kept cache, no
duplicate rebuilds, a local Linux pre-push check.

**Provenance:** founder 2026-09-29 verbatim «Надо ускорять эти ci а то это реально бутылочное горлышко во всей нашей
работе» ("these CIs need speeding up — they are genuinely the bottleneck in all our work"), restated by the operator
at this take-up. Order and scope were relayed by the overseer at the 2026-09-29 wrap (run `2026-09-29T15-21-19Z-wrap`):
split the ubuntu job into parallel jobs, keep the dependency cache on failure, drop the duplicate full rebuilds, add a
local WSL Ubuntu pre-push Linux check. The entry was placed first after P-025 because every later chunk pays a full CI
read. Each coordinate the freight names was re-verified at P1 against HEAD `e98d838`; results are inline. Mechanism
claims stay `[inferred]` until P3 closes them.

**Capability claim:** none expected. This is CI/dev-loop infrastructure; no 0.3.0 matrix id names CI wall time. P4
re-reads the unclaimed pool (`P-075` is the only unclaimed id and belongs to the Conductor-return entry).

## What it builds

### A. Parallel jobs (the ubuntu leg stops being one ~2 h serial job)
- Today `.github/workflows/ci.yml` (431 lines) holds three jobs: `lint-test-build` (`:19`, a matrix over
  `ubuntu-22.04` · `macos-latest` · `windows-latest`, `fail-fast: false`), `supply-chain` (`:255`, ubuntu) and
  `coverage` (`:329`, ubuntu). Verified at P1.
- The ubuntu leg of `lint-test-build` runs its gates in series. Measured 2026-09-29 (entry CONTEXT): ~2 h per round —
  clippy 13 · typecheck 11 · test 14 · a11y 10 · perf 10 · release 26 · mcp build 5 · nextest 12 · boot 12 min.
  `[premise-corrected: those are the COLD-cache round ci#36574279289 (ubuntu log "No cache found"), 119.7 min. The first
  warm round, ci#36593334009 on e98d838, took 44.8 min: ubuntu 44.8 · windows 37.1 · macOS 33.4 · coverage 16.0 ·
  supply-chain 7.8. Warm ubuntu: test 7.1 · a11y 5.9 · perf 6.1 · release 3.2 · mcp release 3.2 · mcp nextest 7.2 ·
  boot 7.7 min. Every heavy step is compile-dominated; the 2398-test suite runs in ~30 s. research.md §CI runs read]`
- Build: split that serial chain into independent jobs that run at the same time, so the round's wall time is the
  longest job (plus shared setup), not the sum of all of them.
- `[premise-corrected: macOS/Windows measured at 33.0 / 42.4 min cold and 33.4 / 37.1 min warm. After an ubuntu-only
  split they set the critical path (33–37 min), so the split or a trim has to reach them too, or the round stays ≥ ~33
  min. research.md §CI runs read]`

### B. A kept cache
- Keep the dependency cache when a job fails (the entry: "keep the dependency cache on failure"). Today every job
  uses `Swatinem/rust-cache@23869a5…` (v2.9.1) with `shared-key: ${{ runner.os }}-cargo` (`ci.yml:42-45`, verified).
- The mechanism, verified: rust-cache saves only on job success by default (`cache-on-failure` "Defaults to false",
  action.yml at the pinned SHA). A red round's compiled dependencies are lost, and the next round rebuilds them cold:
  all 8 red P-025 rounds were cold, and the Linux key's first save came at 15:18 on the first green ubuntu leg.
- `[premise-corrected: the measured defect is FREEZING, not racing. A full-match restore never re-saves ("Cache
  up-to-date"), so a key saved once is frozen until its lockfile/toolchain hash changes. coverage shares
  "ubuntu-22.04-cargo" with supply-chain, restores a release-shaped cache and is never warm (16–17 min every round); the
  macOS/Windows keys have been frozen since ~10:00, and warm macOS clippy still compiles libduckdb-sys from source.
  The repo cache is 9.27 GB of the 10 GB cap across 5 keys, so a naive per-job key set evicts and brings cold rounds
  back. Workspace crates are never cached (cache-workspace-crates default false). research.md §Files inspected]`

### C. No duplicate full rebuilds
- Drop the duplicate full rebuilds. The measured list names `release 26`, `mcp build 5`, `test 14` and `nextest 12`
  as separate steps.
- `[premise-corrected: the true duplicates are measured, and they are not the pairs guessed at P1.
  (i) perf:slo-load runs nextest -p pulse-app --test perf_slo_10k_spans, a separate compile (9m52s cold / 6.1 min warm)
  for ONE test the workspace ci run already executes: default-filter excludes only perf_load_profiles.
  (ii) Linux cargo build --workspace --release duplicates supply-chain's cargo auditable build --workspace --release
  (same profile, scope and features).
  Not duplicates: test (default features, 2398 tests) vs nextest --features mcp-server (2433) have different feature
  sets, and default features are the shipping configuration. mcp build vs a "bindings regen": no CI step regenerates
  bindings, and the mcp release build is the only CI smoke of the feature-gated binary.
  Unmeasured: boot's --bin pulse-app --release recompile after --workspace --release (7.7 min warm); feature unification
  is the hypothesis. research.md §Scope premise closure]`
- Boundary: a rebuild goes only when it duplicates another gate's coverage. No gate is dropped for being slow; a gate
  that must survive moves into its own parallel job (A) instead.

### D. A local Linux pre-push check (WSL Ubuntu)
- Add a local check that runs the Linux-reachable gates in WSL Ubuntu before a push, so Linux-only failures surface
  locally instead of as a red CI round.
- Precedent (read-only, another repo): `D:/dev/projects/viola/scripts/wsl-exec.sh` (2 672 B, verified present) and
  `D:/dev/projects/viola/crates/viola-e2e/src/harness/pre_push/linux.rs` (27 876 B, verified present).
- (P3: kept as the target list, not re-derived — it is a recount of past rounds, not a claim about HEAD.) The entry's
  claim, verbatim: «of P-025's 8 red CI rounds, a local Linux (WSL) run would have caught 4
  (apt/libdbus deps and their order, npm 10 vs 11, the perf-slo-check abort, the boot-script mode); gitleaks, cargo
  deny and the coverage threshold are caught by any local run of the same gates; 3 were macOS-only (icons,
  quarantine, salt race) and no local Linux check catches them (the overseer's recount)». This is a recount, not a
  measurement at HEAD. P3 does not re-litigate it but uses it as the target list: the check has to be able to catch
  those four classes.
- `[premise-corrected: the distro exists but is only partly provisioned. Ubuntu 26.04.1 LTS on WSL2 (32 cores, 31 GiB,
  disk image on C: with 151 GB free); rustup with 1.95.0 (auto-installed by a P3 probe) + cargo-nextest +
  cargo-llvm-cov; Node v24.21.0 (Viola's, ~/.local/viola-node); gcc; xvfb-run. None of ci.yml's 9 Tauri/dbus apt
  packages are installed, and sudo needs a password, so installing them is a one-time operator step. CI runs
  ubuntu-22.04, so the distro is a newer Ubuntu than the runner. research.md §Files inspected]`
- Its surface belongs with the existing harness verbs, not in a new ad-hoc script. The 5-command `agent-run`
  discipline forbids a 6th verb, so it is an xtask verb (arch §Established Decisions [CI Task Runner]). Its name is a
  P4 decision.

### E. CARRY — register the two unregistered scenario smokes in arch
- Register `smoke:gap-resume` and `smoke:external-resolve` in arch §Occupied Resources → xtask CLI surfaces, beside
  `smoke:hue-shift`.
- Verified at P1: `.andromeda/architecture.md:241` (the xtask CLI surfaces bullet) names only `smoke:hue-shift`;
  test-plan registers all three (`test-plan.md:311` gap-resume · `:313` external-resolve · `:315` hue-shift). This
  gap predates P-025.
- It is pinned here on the overseer's word, because this chunk rewrites the xtask and CI surfaces. The arch edit
  itself is wrap's amendment (phase never amends spec sources). /implement's part is to make sure the xtask
  registrations exist, so the wrap amendment describes something real.

### Observation (not an obligation)
- watch: Linux boot silent death: on `546d3f0` the app died silently ~0.5 s after its webview began polling. Cause
  not identified; no recurrence on `464f2a3`. Hypothesis: an intermittent Linux crash. The instrument is
  `agent-run.sh boot`'s failure diagnosis, which names the signal or exit status. Green runs so far: ci#36574279289.
  Retires on a recurrence or 3 green runs (1/3; since 2026-09-29-p-025-hue-shift-observable-made-gradable).

## CI verdict read at Setup (5a)
- `e98d838` (the last wrap's commit, the only sha from the flip through HEAD): **verdict not yet available**. ci
  run ci#36593334009 was in progress at 15:5xZ (5 checks running, oldest `lint / test / build (ubuntu-22.04)` at
  225 s); secret-scan#36593334085 completed/success. No wall-clock yet. Nothing is folded. /implement re-reads it,
  and the finished run is also a before-measurement for this chunk.
- P3 re-read: ci#36593334009 completed **success**, 6/6, wall 15:51:09 → 16:35:57 = **44.8 min**. It is this chunk's
  warm-cache BEFORE baseline.

## Host constraint (operator directive at take-up, 2026-09-29; stands until the operator releases it)
- Conductor is driving real-model legs against `target/release/pulse-app.exe` on `127.0.0.1:4317`. Until released:
  **no release-profile rebuild, no starting `pulse-app`, no binding `4317`/`4318`**. Debug-profile cargo and WSL work
  are fine.
- Disk: 103 GB free, `target/` 79 GB. Check disk before any heavy step (a cold WSL build populates its own target dir
  inside the distro's filesystem).
- The WSL pre-push check (D) builds in the distro's own clone and target dir, never in the Windows `target/release/`.
  Verified: there is no `~/.wslconfig`, so networking is NAT by default and WSL's loopback is not Windows' loopback.
  `agent-run.sh` honours `ANDROMEDA_PULSE_OTLP_{GRPC,HTTP}_PORT` (`scripts/agent-run.sh:32-33`), so a WSL boot need
  not use 4317/4318. Whether a WSL boot counts as "starting pulse-app" under the directive is the operator's call
  (research.md open question 2).

## Boundaries
- No gate is weakened or removed as a cost cut; every gate that runs today still runs after the change (A/C), or its
  removal is argued as a true duplicate.
- The CI perf-budget gate's vacuousness (it reads the boot log with 0 frame/memory/snapshot samples) belongs to the
  later entry "Perf-budget gate reads real samples". It stays out of scope here, even if the perf step moves.
- The CI security posture stays: third-party actions SHA-pinned, `step-security/harden-runner` as every job's first
  step, workflow-level `permissions: contents: read`. Every new job inherits these (security.md §Supply chain + CI).
- macOS-only failure classes (icons, quarantine, salt race) are out of reach of a Linux pre-push check by
  construction. Nothing here claims them.
- The draft PR #39 is never merged or closed by the builder (handoff note).
