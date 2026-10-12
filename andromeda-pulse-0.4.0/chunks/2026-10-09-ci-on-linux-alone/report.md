# Report — 2026-10-09-ci-on-linux-alone

**Chunk:** CI on Linux alone: Windows and macOS legs leave lint-test and a11y, the release job whole; per-job
wall-clock recorded
**Date:** 2026-10-09T15:13:47Z
**Commits:** `569604b chore(2026-10-09-ci-on-linux-alone): operator pre-CI commit` (the one commit from the chunk
base `0b61bfb`; `git log --format='%h %s' 0b61bfb..HEAD`)

## Changes (structured — detectors read this)
- **Files:** two source files (`git diff --name-only 0b61bfb`, outside the planning folders):
  - `.github/workflows/ci.yml` — 11 lines added, 80 removed. Added ranges as the listing prints them: `19-21` ·
    `24-25` · `82` · `115` · `120` · `123` · `240-241`.
  - `pulse-app/tests/quality_gate_workflow.rs` — 42 lines added, 8 removed. Added ranges: `101-122` · `124-125` ·
    `127-134` · `136-144` · `447`.
  - No other source, manifest, lockfile, script or workflow file changed. `release.yml`, `update-channels.yml` and
    `secret-scan.yml` are byte-identical to the base (the scope-guard gate prints nothing).
- **Symbols / APIs:** no product symbol, IPC method, endpoint, port, env var or export changed. In the test file:
  - removed: test `ci_workflow_release_job_owns_and_saves_its_cache_key` (`quality_gate_workflow.rs:100-112` at
    `0b61bfb`), whose subject was the `release` job;
  - added: tests `ci_workflow_runs_on_linux_only` and `ci_workflow_keeps_the_linux_release_build_witnesses` (inside
    the added ranges above);
  - changed: `data_dir_export_precedes_every_consumer` expects 6 jobs in `ci.yml` (`:447`; 7 at the base);
  - the helper `workflow_job_block` keeps callers: the witness test calls it twice (`supply-chain`, `boot`). Its one
    caller at the base was the removed test.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none added or bumped.
- **Schema / config:** none (no migration, config key, violation schema or redaction shape).
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved:**
  - Jobs of the `ci` workflow: 7 → 6 (`a11y boot coverage lint-test mcp-test supply-chain`; the PyYAML job-list gate).
    `release` is gone. Stated as seven at: `registries/contracts/architecture/ci-cd-approach.md:3` ("seven
    independent jobs"); `CLAUDE.md` Key directories ("seven parallel jobs over Linux/macOS/Windows"), a leaf.
  - Runner systems of the workflow: 3 → 1. Every `runs-on:` line reads `ubuntu-22.04` (6 lines,
    `grep -n 'runs-on:' .github/workflows/ci.yml`); no job has a `strategy` block or a `needs:` edge (the `6 of 6`
    gate; `4 of 7` at the base). `grep -c -i -E 'macos|windows|matrix\.os|runner\.os ==' .github/workflows/ci.yml`:
    20 → 0.
  - Checks one commit registers: 13 → 7 (`ci.py conclusion`: `checks 7/7` on `569604b`; 13 on `0b61bfb`): six `ci`
    jobs and one `secret-scan`.
  - Artifacts one pull-request run uploads: 12 → 6 (`gh api …/runs/37945548047/artifacts`): `logs-boot-Linux`,
    `coverage-linux`, `playwright-a11y-report-Linux`, `a11y-violations-Linux`, `logs-perf-samples-Linux`,
    `capability-drift-Linux`.
  - Tests in the `quality_gate_workflow` binary: 18 → 19 (one removed, two added; nextest `19 tests run`). Workspace
    nextest at /implement: `2800 tests run: 2800 passed, 0 skipped`.
  - The repository's Actions cache, re-read at this wrap (2026-10-09T15:13:15Z, `gh api …/actions/cache/usage` and
    `…/actions/caches`): 10 entries, 12 208 662 121 B, which is 1 471 243 881 B over the 10 737 418 240 B cap. Six
    entries belong to keys no job writes any more (`release-Windows`, `lint-test-Windows`, `lint-test-macOS`,
    `release-macOS` on `refs/heads/main`, 5 240 525 619 B; `lint-test-Windows` and `release-Windows` on
    `refs/pull/39/merge`, 3 290 502 466 B): 8 531 028 085 B together. The four that stay (`boot-Linux`,
    `lint-test-Linux`, `coverage`, the gitleaks cache) sum 3 677 634 036 B. The architecture key file states
    "8 entries, 10 605 172 169 of the 10 737 418 240 B cap (≈ 1.2 % headroom)" and a re-read "byte-identical, 8
    entries, 132 246 071 B (1.23 %) headroom", both dated measurements of 2026-09-29/30.
- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed. PyYAML 6.0.3 (research) was
  used by two gates and not re-read.
- **Harness / gate surface:** the `ci` workflow only; no agent-run script, xtask verb or status shape changed.
  - `lint-test`: `name: lint / test (ubuntu-22.04)`, `runs-on: ubuntu-22.04`, the `strategy` / `matrix` block removed.
    The resolved check name `lint / test (ubuntu-22.04)` is what the matrix resolved to before.
  - `a11y`: `name: a11y (ubuntu-22.04)`, `runs-on: ubuntu-22.04`, the `strategy` / `matrix` block removed.
  - `release` (the macOS + Windows `cargo build --workspace --release` job, its own `release-${{ runner.os }}` cache
    key, and the two comment lines above it): removed whole. It read no secret and named no environment.
  - Seven `if: runner.os == 'Linux'` step conditions removed: the system-library install step of `lint-test`, `a11y`,
    `supply-chain` and `coverage`, and the three perf steps of `lint-test`. Each step stays and is unconditional.
  - Three step names lost the suffix "(Linux only)" and keep the rest: `cargo xtask perf:slo-load (10k spans/sec
    sustained-load)` · `cargo nextest run --workspace --profile perf-samples` · `cargo xtask perf:budget (memory +
    snapshot required)`.
  - Two comments rewritten: the cache-budget header above `jobs:` (one target cache for `lint-test` and one for
    `boot`; `a11y`, `mcp-test` and `supply-chain` restore read-only; `coverage` keeps the registry only) and the
    comment above the "No Cyrillic in sources" step, which lost its clause about the Windows runner's code page.
  - Unchanged: every `uses:` line and its sha pin, the workflow-level `permissions: contents: read`, every step's
    order, every `shell: bash`, every name and key templated on `${{ runner.os }}`, `node-version: '24'`, the
    `apt-get install` lists, the three `continue-on-error` baseline downloads, every `if: always()` and every
    `if: github.event_name == 'pull_request'`, the `cargo audit` step and its token. The gate
    `git diff 0b61bfb… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|permissions:|secrets\.|…)'` reads 0.
  - The Linux release build keeps two witnesses in the workflow: `supply-chain`'s
    `cargo auditable build --workspace --release` and `boot`'s
    `cargo build --workspace --release --features mcp-server`, now pinned by a test.
  - Cache keys no job writes any more: `release-{os}`, and `lint-test-{os}` for Windows and macOS. The keys that
    remain: `lint-test-Linux` (owned by `lint-test`, restored read-only by `mcp-test` and `a11y`), `boot-Linux`
    (owned by `boot`, restored read-only by `supply-chain`), `coverage` (registry only).
  - After the chunk no CI job compiles or tests on Windows or macOS. The other-system arms of 61
    conditional-compilation lines in 24 Rust files (research.md) and the two `.ps1` harness halves have no CI
    witness; they are owned by later route entries (`Other operating systems retired from the code`, `Window
    retired`) and were not changed here.
- **Cross-project / external claims:**
  - `ci#37945548047` on `569604b` (pull_request, attempt 1): success, 6 jobs, every step `success`; with
    `secret-scan#37945547982` success, `verdict: green · checks 7/7 · wall 1714 s`. The sha is the record: this
    wrap's own commit adds to that tree.
  - `ci#37934330231` on `0b61bfb` (pull_request): success, 12 jobs, wall 1463 s — the before-reading, read at
    take-up (scope.md).
  - GitHub repository `Turbolet85/andromeds-pulse`: the cache listing above; `main` has no required status check
    (research.md: `…/branches/main/protection` answers 404, `…/rulesets` answers `[]`), not re-read at this wrap.
  - The inputs channel, from `inputs.py verify` (run before this report existed):
    - `I1 · ../additional/pc-overseer/relays/pulse-phase-ci-on-linux-alone-2026-10-09.md · copy no-repo · unchanged`
    - `I2 · message: the operator (pc overseer), in the /andromeda-phase invocation, 2026-10-09 · copy message · n/a —
      a message has no live source`
    - `I3 · ../additional/pc-overseer/relays/pulse-wrap-ci-on-linux-alone-2026-10-09.md · copy no-repo · unchanged` —
      snapped at this wrap and cited here as inputs#I3: the operator's wrap directive (the a11y reading is a reading;
      four found-standing things get an owner at route-resolve; the watch's pin and tally; three readings for the
      closing report; the stops).
    - `I4 · message: the operator (pc overseer), in the /andromeda-wrap-session invocation, 2026-10-09 · copy message
      · n/a` — snapped at this wrap and cited here as inputs#I4: the CI read is green, the operator pass stands, stop
      before the commit.
    - No entry reads `drifted`, `vanished` or `broken`; no `UNPARSED:` row. `verify` printed I3 and I4 `UNCITED`
      because it ran before this report existed.
- **Reverted / negative API facts:** none. The Step 5 mutation (`--release` dropped from the `supply-chain` build
  step) was applied and reverted inside /implement; the file's sha256 read equal before and after
  (`evidence/mutation-checks.md`).
- **Insufficient fixes (written, kept, not the remedy):** none. The chunk makes the CI clause of P-113 true and
  claims no capability; the rest of P-113 is three later entries'.
- **Spec claims disproved by measurement:**
  1. obs-plan §9 Telemetry artifact handling (`obs-plan.md:499`) states the log file is uploaded by "the `lint-test`
     job per OS (`logs-${{ runner.os }}`)". Two things: (a) "per OS" is made false by this chunk — one system
     remains; (b) found standing before the chunk: no `logs-Linux` artifact exists on the before-run (12 names) or
     the after-run (6 names, `evidence/operator-pass.md` entry 22). The upload step runs and finds no file
     (`if-no-files-found: ignore`). The same holds for `nextest-Linux` and `criterion-Linux`, whose upload steps
     stand in `ci.yml` (`:139`, `:148`, `:157` after the edit, `grep -rn -A4 'upload-artifact' .github/workflows/`).
     (b) needs an owner, by the operator's direction (inputs#I3 item 3).
  2. a11y-plan §3 CI integration (`registries/contracts/a11y-plan/ci-integration.md:12`) states a per-PR regression
     detection that downloads the base branch's summary as `a11y-violations-base` and compares. Found standing: no
     workflow uploads an artifact under a `-base` name (`grep -rn -e '-base' .github/workflows/`: three downloads
     and two consumers in `ci.yml`, no upload), so the download never finds a baseline. The same holds for
     `criterion-${{ runner.os }}-base` and `coverage-linux-base`. Each download is `continue-on-error`. Needs an
     owner (inputs#I3 item 3).
  3. a11y-plan §9 CI Integration (`a11y-plan.md:464`) lists a `Lint | eslint-plugin-jsx-a11y` stage with "GitHub
     Actions annotations (PR check); CI fail if critical violations". Found standing: no workflow runs an ESLint
     step (`grep -n -i -e eslint -e 'run lint' -e 'xtask lint' .github/workflows/*.yml`: 0 lines, exit 1). The lint
     lives behind `cargo xtask lint`, which no job calls (research.md). Needs an owner (inputs#I3 item 3).
  4. The architecture CI/CD key file's cache figures (above, Counts) describe 8 entries inside the cap; the cache
     reads 10 entries and 1.47 GB over the cap today. Dated measurements, so stale and not false as dated; the
     leaving legs' six entries need an owner (inputs#I3 item 3). Nothing outside the tree is deleted by this wrap.
  5. Plan forecast (`plan.md`, Implementation notes): "the sum of job-seconds falls by about the six legs' share,
     3120 s of 7221 s". Read: 7221 → 4702 s, down 2519 s. The six legs' 3120 s left; the six jobs that stay summed
     601 s more than on the before-run, 851 s of it `a11y (ubuntu-22.04)` (237 → 1088 s). A chunk-artifact claim: a
     report entry, no edit.
  6. Plan's expected-amendments list names, in architecture §Occupied Resources → xtask CLI surfaces, "the boot
     smoke's Linux-only wording". `grep -n -o -i 'Linux.only' .andromeda/architecture.md`: 1 hit, at `:248`, about
     the NVIDIA launch-posture default, not the boot smoke; 0 hits in `:249`. No such site exists in that entry. A
     chunk-artifact claim: a report entry, no edit.
  7. research.md `## Files to modify` says of the test file "one pin that the workflow names no other system is
     added"; two pins were added (the second by the P5 amendment recorded in scope.md). A chunk-artifact claim: a
     report entry, no edit.
- **Expected amendments (from plan):** the site search for all seven entries is one sweep over the seven master
  bodies and every file under `.andromeda/registries/`:
  `(three|3|all|every|each|both)[ -](OS|OSes|systems|runners|platforms)|lint-test (on )?(Linux|macOS)|Linux/macOS|per-OS|per OS|CI matrix|matrix (job|runners?|Linux)|(macos|windows)-latest|seven (parallel|independent) jobs|release job|matrix`
  (case-insensitive; the bare `matrix` hits about the capability matrix, a rule matrix or a colour matrix left out).
  Hits that state the CI's runner systems or job set, per master (hits about the product's three systems, the
  `perf:budget` wiring "lint-test Linux" and the dated Windows-runner frame measurement are no-change, below):
  - architecture §Infrastructure Patterns → CI/CD approach — **carried**: Counts (jobs 7 → 6, the cache re-read) and
    Harness (both matrices, the `release` job, its cache key). Site: 1 hit, the key file
    `registries/contracts/architecture/ci-cd-approach.md:3` (architecture owns it); 0 hits in `architecture.md`'s
    body for the job set.
  - architecture §Occupied Resources → xtask CLI surfaces — **carried** for the `check:english-sources` clause:
    Harness (`lint-test` runs on one system). Site: 1 hit, `architecture.md:249` at char 3856, "on all three OSes".
    **Not carried** for "the boot smoke's Linux-only wording": no such site (Spec claims disproved, 6).
  - test-plan §9 Pipeline structure — **carried**: Harness and Counts. Sites in `test-plan.md`: `:490` (Lint + tests
    row: "matrix Linux/macOS/Windows", "one job per OS", `release-${{ runner.os }}` in its cache cell; also two dated
    measurements, "measured `clean` on windows-latest, `ci#36893004900`" and "green on ubuntu-22.04 / macos-latest /
    windows-latest, `ci#37327846820`", and "a Python 3 interpreter on all three runners") · `:491` (Release build
    row, the `release` job) · `:493` (A11y suite row: "its own `a11y` matrix job on all three OSes") · `:496` (Boot
    smoke row: "ci-gates no longer runs on macOS/Windows") · `:499` (E2E row: "matrix per surface (tauri-driver × 3
    platforms)") · `:503-516` (§9 Matrix builds, a fenced `matrix:` block over three systems).
  - test-plan §4 and §3 — **carried**: Harness. Sites: `test-plan.md:192` ("cross-platform on all 3 CI matrix
    runners"); `registries/contracts/test-plan/per-chunk-gate-discipline.md:42` (Process-end witness form, "CI
    lint-test Linux/macOS"; test-plan owns it).
  - security-plan — **carried**: Harness. Sites in `security-plan.md`: `:103` (§Threat Model Summary →
    Infrastructure → CI/CD, "matrix Linux/macOS/Windows") · `:216` (§Dependency Security → CI integration, "a step
    in `ci.yml` matrix") · `:259` (§Bootstrap phases `dep-security-ci-gate`, "into `ci.yml` matrix") · `:224` at char
    1031 (a dated measurement, "green on all three `lint-test` runners, `ci#37327846820`"; named by the security
    extract and read at that line).
  - obs-plan §9 Telemetry artifact handling — **carried**: Harness and Spec claims disproved, 1. Site: 1 hit,
    `obs-plan.md:499`.
  - a11y-plan §3 CI integration and §9 CI Integration → Pipeline integration — **carried**: Harness. Sites:
    `registries/contracts/a11y-plan/ci-integration.md:3` ("their own `a11y` matrix job (Linux/macOS/Windows …)",
    "under per-OS names"; a11y-plan owns it) and `a11y-plan.md:471` ("its own `a11y` matrix job").
  - design-system and layout-templates: 0 hits that state the CI.
  - No-change hits of the sweep, by class: the product's three systems (architecture `:54`, `:66`, `:202`; test-plan
    `:105`; a later P-113 entry's) · "lint-test Linux" as the perf-budget gate's host (architecture `:249` at char
    13967; test-plan `:135`, `:576`; obs-plan `:499` at char 366, `:544`, `:546`, `:559`, `:560` — true before and
    after, though "Linux" no longer distinguishes a leg) · the dated measurement "the hosted Windows runner exposed
    no WebGPU adapter (`ci#36723465727`)" (architecture `:249` at char 15272; test-plan `:100`; obs-plan `:126`,
    `:545` — a dated fact about a past run).
- **Coverage of new surfaces:** none — the chunk adds no external surface, hot-path operation or UI element. Its two
  added tests are unit-tier workflow self-lints (pure file reads, no runtime, no network).

## Deviations from intent
- Plan Step 5 asks to confirm `git diff --quiet` on `ci.yml` "against the Step 3 result". The Step 3 result was
  uncommitted, so git had no such base. Used `git diff --no-index --quiet` against a scratchpad copy taken before
  the mutation, plus sha256 before and after. Both read equal.
- The Step 5 mutation covered the `supply-chain` arm of the witness pin, as planned. The `boot` arm was not mutated;
  `evidence/mutation-checks.md` states the limit.
- The before-readings in `evidence/ci-wall-clock.md` are copied from scope.md and research.md, as the plan directs,
  and were not re-read from GitHub by /implement.
- The two new tests' assertion messages cite test-plan §9 rather than the chunk marker (a durable source).
- Operator pass, beyond the plan's six entries: one hand respell before entry 17 (below, Decisions), and one extra
  read, the a11y job's per-step instants on both runs, made because the job read 1088 s against 237 s.
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 2 · listed 2`, base `0b61bfb`).

## Decisions & corrections
- **The operator's word on hygiene (2026-10-09, in session, quoted in `evidence/operator-pass.md`):** in the phase
  run's `relay-1.md`, respell the home prefix of the one path to a tilde, one anchored edit, and say so in the
  operator-pass record. Done by hand before entry 17: the row's form was `home` (a path outside the repository),
  which `gate.py respell` does not rewrite.
- **The operator's word on the a11y reading (inputs#I3 item 2):** it is a reading, not a defect of this chunk; record
  it as measured, state no cause, re-run nothing. The job leaves at `Window's gates retired`.
- **The operator's word on the found-standing things (inputs#I3 item 3):** each gets an owner at route-resolve,
  presented for disposition, none pre-placed; "carried to the report as found" is not an owner.
- **The operator's word on the stops (inputs#I3 item 6, inputs#I4):** the first citation sweep stops for the
  operator's word; stop before the commit and print `git status --short`.
- The operator pass was run by the agent on the operator's explicit word, the pre-CI commit and the push included.
- Sweep hazards found:
  - A phase run dir's `relay-{n}.md` holds the operator's message verbatim, and a message that names a file by an
    absolute home path makes the run dir's copy a hygiene P1 row at the pre-CI commit, while the manifest-listed
    copy in `inputs/` is exempt. This wrap's own invocation message spelled the path with `~`.
  - `git diff --quiet` cannot compare against an uncommitted intermediate; a plan step that needs a restore check
    after a working-tree mutation needs a copy or a hash taken first.
  - The masters say "lint-test Linux" for the perf-budget gate's host. A grep for other-system wording that
    includes `Linux` over-matches these true sites; the sweep above lists them as no-change.

## Outcome
Acceptance criteria, each re-asserted against the diff:
- (arch) six jobs, each on `ubuntu-22.04`, no `strategy`, no `needs:` — **met**: the two PyYAML gates read `a11y boot
  coverage lint-test mcp-test supply-chain` and `6 of 6`.
- (tests) no line names another system, a matrix value or a system condition — **met**: the grep gate reads 0;
  `ci_workflow_runs_on_linux_only` passes, having read red at the base (`evidence/red-before.md`).
- (arch) nothing outside the two listed files changes — **met**: the scope-guard gate prints nothing; `gate.py scope`
  clean.
- (security) the edit adds no action reference, permission, secret reference, soft gate, job edge, condition, retry,
  Node pin or package-install line — **met**: the diff-grep gate reads 0;
  `ci_workflow_uses_sha_pin_discipline_unchanged` and `ci_workflow_preserves_workflow_level_contents_read_permission`
  pass.
- (security, tests) each of the six jobs opens with `Harden runner`, then `Export ANDROMEDA_PULSE_DATA_DIR` — **met**:
  `data_dir_export_precedes_every_consumer` passes with 6; `workflow_env_references_no_step_only_context` passes.
- (arch, tests) the Linux release build keeps both witnesses — **met**:
  `ci_workflow_keeps_the_linux_release_build_witnesses` passes and read red under the mutation
  (`evidence/mutation-checks.md`; one arm mutated).
- (arch) the pins `pre-push:linux` reads still read — **met**: `apt_packages_read_the_real_workflow` passes.
- (a11y) the a11y job's step sequence is intact — **met**: `ci_workflow_builds_ui_before_a11y_run`,
  `ci_workflow_invokes_xtask_test_a11y` and the two a11y artifact tests pass.
- (tests) the standard gate set passes in order and closes with the bindings probe at exit 0 — **met** at /implement
  (16 of 16 fired entries green); re-run at this wrap's P7.
- (tests, security, obs) the run on the pre-CI commit reads `verdict: green`, and no step reads other than `success`
  — **met**: `ci#37945548047`, entries 19 and 21 of `evidence/operator-pass.md`.
- (obs, a11y) the after-run uploads the six Linux artifacts and none for macOS or Windows — **met**: entry 22.
- (entry) `evidence/ci-wall-clock.md` holds the before- and after-reading per job, each with its run id, and the
  boot-smoke verdict of every run read — **met**: nine runs listed.

Gates, by `run`, in block order — /implement's run (`.andromeda/runs/2026-10-09T14-29-03Z-implement/`), on the tree
the pre-CI commit then carried:
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 0b61bfb… -- crates pulse-app xtask scripts docs … ':!.github/workflows/ci.yml' ':!pulse-app/tests/quality_gate_workflow.rs'` — green · exit 0 · no output
- `python -X utf8 -c "import yaml; … print(' '.join(sorted(d['jobs'])))"` — green · last line `a11y boot coverage lint-test mcp-test supply-chain`
- `python -X utf8 -c "import yaml; … 'of', len(d['jobs']))"` — green · last line `6 of 6`
- `grep -c -i -E 'macos|windows|matrix\.os|runner\.os ==' .github/workflows/ci.yml` — green · exit 1 · last line `0`
- `git diff 0b61bfb… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|permissions:|…)'` — green · exit 1 · last line `0`
- `cargo xtask check:english-sources` — green · contains `"verdict": "clean"`
- `cargo xtask capability-widening-check` — green
- `cargo xtask check:ingest-progress` — green
- `cargo xtask check:staged-artifacts` — green
- `cargo xtask capability-drift` — green
- `cargo xtask verify:capability-matrix` — green
- `cargo nextest run --workspace --profile ci` — green · 2800 passed, 0 skipped
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 0b61bfb… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- The six `leg = 'operator'` entries, driven once by hand in the operator pass (`evidence/operator-pass.md`):
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — green · `hygiene: clean`
  - `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` — green ·
    `0b61bfb..569604b`, a fast-forward
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` — green ·
    `verdict: green · checks 7/7 · wall 1714 s`
  - `gh api "…/actions/runs/<id>/jobs?per_page=100" --jq '.jobs[] | "\(.name) · …"'` — recorded. The outcome it read:
    `ci#37945548047`, six jobs, each `success`; the chunk's own run, green. Its rows are the After table.
  - `gh api "…/actions/runs/<id>/jobs?per_page=100" --jq '… select(.conclusion != "success") …'` — green · no output
  - `gh api "…/actions/runs/<id>/artifacts?per_page=100" --jq '.artifacts[].name'` — green · the six names, no macOS
    or Windows name
- No `defer` entry. No smoke: no boot-path or UI-surface change.

Readings recorded as measured, no cause stated, nothing re-run (inputs#I3 item 2):
- The round: 1714 s after against 1463 s before; `coverage gate` the longest job in both. Inside the 1091 to 1855 s
  spread of the seven earlier pull-request runs.
- Job-seconds: 4702 s over 6 jobs after, 7221 s over 12 before (the before-run's six Linux jobs alone: 4101 s).
- `a11y (ubuntu-22.04)`: 1088 s after, 237 s before. Per-step instants place the difference in
  `Install Linux system libraries (Tauri + dbus)` (666 s against 28 s) and `Install Playwright chromium (a11y
  harness)` (266 s against 76 s), two download steps whose commands the chunk did not change. One run is one
  reading.

Watches:
- boot smoke on ubuntu-22.04 ended exit 1 after ready with no process-end record (run 37924991598 on `b3ac58a`) ·
  2 green runs since that red: `ci#37934330231` on `0b61bfb` (job `113832488437`, 1106 s; equal source to the red's
  commit) and `ci#37945548047` on `569604b` (job `113870729265`, 658 s; a tree that differs from `0b61bfb` in
  `ci.yml` and one test file, the `boot` job's own block unchanged). No recurrence. How the tally counts the second
  reading, and where the pin goes now that its entry is frozen, are route-resolve's (inputs#I3 item 4).

Outcome basis: the operator pass ran, so the verdicts rest on its final state: the commit list above (one commit,
`569604b`) and that commit's CI run recorded in `evidence/operator-pass.md`. /implement's P4 report, given in this
session's conversation, is the basis for the sixteen local gates, the red-before reading and the mutation check.
Between /implement and this report: the operator's word that started the pass (the respell, then entries 17 to 22),
and the wrap directive (inputs#I3, inputs#I4). Post-implement artifacts: `evidence/operator-pass.md`, the After
section of `evidence/ci-wall-clock.md`, and the respelled line 1 of the phase run's `relay-1.md`.

Process hygiene:

| process | started by | final state |
|---|---|---|
| `gate.py run` (16 entries) | /implement | terminated |
| `cargo nextest … -E 'binary(quality_gate_workflow)'` ×4 | /implement | terminated |
| `ci.py conclusion --wait 2700` | the operator pass | terminated (returned 2026-10-09T15:06:31Z) |
| `scripts/code-graph.py refresh` | this wrap | terminated (exit 0) |

Re-measured at this wrap (2026-10-09T15:13:47Z, `ps -eo pid,etimes,comm,args`): no process of this project's runs is
alive. /implement's own census had read the same, with another project's `cargo-mutants` run as the only cargo
activity on the host.
## New text, by line
Generated by `cites.py added` (cites v1.3); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 0b61bfbe (the parent of the oldest pre-CI commit 569604be) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 11 line(s) in 7 range(s)
added: 19-21 · 24-25 · 82 · 115 · 120 · 123 · 240-241
### pulse-app/tests/quality_gate_workflow.rs — added 42 line(s) in 5 range(s)
added: 101-122 · 124-125 · 127-134 · 136-144 · 447
  - 104-122 «for (idx, line) in content.lines().enumerate() {»
