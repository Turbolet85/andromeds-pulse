# Codebase Research — 2026-10-09-ci-on-linux-alone

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 17
- **Harness rules consulted:** none — no live leg in this chunk. The chunk boots no app and drives no external
  process; its only run against something outside the tree is the CI run the operator pass reads.
- **Platform issues consulted:** `GitHub Actions ubuntu-22.04 xvfb-run Tauri WebKitGTK app intermittently exits code 1
  shortly after start` (web search, 2026-10-09) → the results are Tauri's WebDriver CI guide and two unrelated pull
  requests; none describes an app that reaches ready and then ends `exit 1` with no record. No issue or release note
  was fetched, so nothing is cited as a cause. The boot-smoke red is a folded `watch:`, an observation: this chunk
  plans no instrumentation for it.
- **External inputs:** `inputs#I1` — the operator's relay: the owner of each of the two reds since the last flip, the
  scope edge against P-120 and the release workflows, the per-job wall-clock record, the chunk-base rule for
  diff-shaped probes. `inputs#I2` — the operator's invocation directive: the founder gave the word to start code; the
  two reds are owned as the relay says.

## Files inspected
- `.github/workflows/ci.yml` (full, 649 lines at `0b61bfb`) — seven jobs. `lint-test` (`:23-189`) and `a11y`
  (`:301-390`) run a three-system matrix; `release` (`:193-242`) runs macOS and Windows only; `mcp-test`, `boot`,
  `supply-chain`, `coverage` run `ubuntu-22.04`. No job has a `needs:` edge. `continue-on-error` stands on the three
  baseline downloads only (`:174`, `:363`, `:634`). The one `secrets.` reference is the audit step's token (`:496`).
  The `release` job reads no secret and names no environment. Every job opens with `Harden runner`, then
  `Export ANDROMEDA_PULSE_DATA_DIR`.
- `pulse-app/tests/quality_gate_workflow.rs` (full, 456 lines) — the workflow self-lint. Two tests read what leaves:
  `ci_workflow_release_job_owns_and_saves_its_cache_key` (`:101-112`) calls `workflow_job_block(…, "release")`, which
  panics when the job is absent (`:85`); `data_dir_export_precedes_every_consumer` (`:411-456`) expects
  `("ci.yml", 7)` jobs (`:413`) and asserts each job's first two step names. Four assertions pin the
  `${{ runner.os }}` template's own text in artifact names (`:290`, `:308`, `:319`, `:327`). The job-header rule the
  count uses (`:424-429`) takes every line after `jobs:` that starts with exactly two spaces and ends in `:`, a
  comment line included.
- `pulse-app/tests/a11y_perf_workflow.rs` (full, 161 lines) — reads step commands, the `a11y` job block
  (`job_block`, `:45-58`), the two a11y artifact name prefixes, the SHA pin of every `uses:` line (`:120-149`) and
  the workflow-level `permissions:` (`:152-160`). Nothing it reads leaves.
- `xtask/src/pre_push.rs` (`:148-162`, `:520-566`, and a grep of the file) — `node_major` (`:527`) reads every
  `node-version:` line and requires them equal; `apt_packages` (`:541`) reads the FIRST `apt-get install` command.
  Neither reads a job name, a matrix value or an `if:`. The test fixture at `:605` carries an
  `if: runner.os == 'Linux'` line inside a string; the parser skips it.
- `andromeda-pulse-0.3.0/chunks/2026-10-07-l4-probe-reproduces-the-canary-history-miss/plan.md` (`:323-575`) — the
  standard gate set in its fixed order and the operator entries' form.
- `andromeda-pulse-0.3.0/chunks/2026-09-29-p-025-hue-shift-observable-made-gradable/plan.md` (`:40-51`) — the step
  that added the data-dir export with `shell: bash`; it states no reason for the key.
- `andromeda-pulse-0.4.0/working-route.md` (`:11`, `:13`, `:15`, `:31`, `:38`, `:40`, `:42`, `:52`) — this entry and
  the entries that own what this chunk leaves alone.
- `andromeda-pulse-0.4.0/intent.md` (`:176-205`) — R7, the source of P-113.
- `scripts/code-graph-cookbook.md` (full) — before the two graph queries.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
Trace: `tree-query-2026-10-09-ci-on-linux-alone.json` in the phase run dir, rust plane, 2 queries.
- **`workflow_job_block`** @ `pulse-app/tests/quality_gate_workflow.rs:79` — 1 caller,
  `ci_workflow_release_job_owns_and_saves_its_cache_key` @ `:102` (query 2, 13 rows). Deleting that test leaves the
  helper with no caller, and `clippy -D warnings` reads an unused function as an error; the helper leaves with it or
  gains a caller.
- **`apt_packages`** @ `xtask/src/pre_push.rs:541` and **`node_major`** @ `:527` — called from `drive` @ `:155` and
  from three tests of the same file (`apt_packages_join_continuations_and_drop_flags`,
  `apt_packages_read_the_real_workflow`, `node_major_reads_one_agreed_major`). No signature changes; the second test
  reads the real workflow and stays the witness that the pins survive the edit.
- **`ci_workflow_runs_on_linux_only`** — 0 rows in the existence query (query 1, 5 rows for the five other names),
  so the name is free.

## Patterns detected
- **A single-system job states its runner literally** (`ci.yml:245-246`, `:393-394`): `name: mcp-server tests
  (ubuntu-22.04)` with `runs-on: ubuntu-22.04`, no `strategy` block. The two matrix jobs take this form, and their
  check names stay `lint / test (ubuntu-22.04)` and `a11y (ubuntu-22.04)`, which is what the matrix resolves to
  today.
- **Names templated on the runner's system** (`ci.yml:49`, `:139`, `:148`, `:157`, `:166`, `:176`, `:186`, `:268`,
  `:328`, `:375`, `:387`, `:462`): cache keys and artifact names carry `${{ runner.os }}`. On Linux each resolves to
  the string it resolves to today.
- **A workflow self-lint is a pure file read with line-anchored matching** (`quality_gate_workflow.rs:79-98`,
  `a11y_perf_workflow.rs:45-58`): `str::lines()` and equality, no YAML parser, no network.
- **The export step is uniform across the three workflow files** (`ci.yml:36-38` and six more; `release.yml`,
  `update-channels.yml`): `Harden runner`, then `Export ANDROMEDA_PULSE_DATA_DIR` with `shell: bash`.

## Conventions to follow
- **Step and job names are what records cite**: a step that loses its "(Linux only)" suffix keeps the rest of its
  name (`ci.yml:121`, `:127`, `:131`).
- **Every third-party action stays pinned by its 40-character sha with its version comment**; the edit removes
  `uses:` lines with the legs and retypes none (`a11y_perf_workflow.rs:120-149` reads every one).
- **A comment line at two-space indent inside `jobs:` must not end in a colon**: the job count at
  `quality_gate_workflow.rs:424-429` would read it as a job.
- **Gate order** (test-plan §3, as the 2026-10-07 plan lists it): `capability-drift` before the default-features
  workspace nextest; the `--features mcp-server` bindings regen is the last cargo-adjacent step; the bindings close
  names the chunk base.

## Measured, for the plan
- **Other-system tokens in `ci.yml`:** `grep -c -i -E 'macos|windows|matrix\.os|runner\.os ==' .github/workflows/ci.yml`
  reads 20 at `0b61bfb`. `grep -c -E '^    runs-on: ubuntu-22\.04$'` reads 4; `grep -c -E '^    runs-on:'` reads 7.
- **Per-job seconds of the eight runs Setup read** (`gh run view {id} --json jobs`, each job's `startedAt` to
  `completedAt`; the before-reading's table is in `scope.md`):

  | sha | run | event | span | longest job | non-Linux legs | all jobs | boot smoke |
  |---|---|---|---|---|---|---|---|
  | `0b61bfb` | `ci#37934330231` | pull_request | 1463 | coverage 1463 | 3120 | 7221 | success, 1106 |
  | `b3ac58a` | `ci#37924991598` | pull_request | 1754 | coverage 1753 | 3166 | 6961 | failure, 568 |
  | `7f99c38` | `ci#37916373451` | pull_request | 1123 | coverage 1123 | 3119 | 6350 | success, 552 |
  | `39edd11` | `ci#37914412856` | pull_request | 1091 | coverage 1091 | 3283 | 6574 | success, 654 |
  | `60ef43c` | `ci#37907730264` | push | 1923 | lint / test (ubuntu-22.04) 1923 | 8159 | 15140 | success, 1595 |
  | `0e45d58` | `ci#37904682919` | pull_request | 1293 | coverage 1293 | 3215 | 6609 | success, 562 |
  | `18a872d` | `ci#37731002463` | pull_request | 1763 | coverage 1763 | 3005 | 6888 | success, 490 |
  | `f18c631` | `ci#37686027609` | pull_request | 1855 | coverage 1820 | 3368 | 8113 | success, 663 |

  The longest job is a Linux job in 8 of 8. The longest non-Linux job reads 626 to 812 s on the pull-request runs.
- **The red boot-smoke job** (`113801670494`, in `ci#37924991598`): `boot: ready (PID=7498 …)`, then `status` printed
  `"ended": "exit 1"`, `"last_write_age_seconds": 0`, `"verdict": "not-running"` and the step ended with exit code 1
  (the job log, `gh api …/actions/jobs/113801670494/logs`). That matches the `watch:` as folded.
- **The repository cache** (`gh api …/actions/caches`, `…/actions/cache/usage`, read 2026-10-09): 10 entries,
  12,208,662,121 B. On `refs/heads/main`: `lint-test-Linux` 1,746,797,483 B, `boot-Linux` 1,727,003,285 B, `coverage`
  198,115,796 B, the gitleaks cache 5,717,472 B, and four leaving entries (`release-Windows`, `lint-test-Windows`,
  `lint-test-macOS`, `release-macOS`, 5,240,525,619 B together). On `refs/pull/39/merge`: two leaving Windows entries,
  3,290,502,466 B.
- **`main` has no required status check**: `gh api …/branches/main/protection` answers 404 `Branch not protected` and
  `…/rulesets` answers `[]`, so no rule names a check that leaves.
- **No test binds a fixed OTLP port**: `grep -rn -E '(127\.0\.0\.1|localhost):431[78]'` over `crates`, `pulse-app`
  and `xtask` Rust files finds one non-comment line, a message string (`xtask/src/perf_frame.rs:150`). The grep sees
  a literal address only; a port built from a constant is outside it.
- **Sweeps** (code, scripts and the other workflows; the planning folders, `.claude` and `docs` left out):
  - `(Linux only)`: 3 hits in `ci.yml` · 3 changed · 0 elsewhere.
  - `matrix.os`: 6 hits in `ci.yml` · 6 changed · 2 no-change (`release.yml:43-44`, the release workflow, not this
    chunk's).
  - `runner.os == 'Linux'`: 7 hits in `ci.yml` · 7 changed · 1 no-change (`xtask/src/pre_push.rs:605`, a parser
    fixture string; the file is `Pre-push check native on Linux`'s).
  - `release-${{`: 1 hit in `ci.yml` (`:221`, leaves with the job) · 2 changed
    (`quality_gate_workflow.rs:104-105`, the test that leaves).
  - `("ci.yml", 7)`: 1 hit · 1 changed (`quality_gate_workflow.rs:413`).
  - `"release"` as a job name: 1 changed (`quality_gate_workflow.rs:102`) · 1 no-change
    (`update-channels.yml:46`, which names the `release` WORKFLOW in a `workflow_run` trigger).
  - `windows-latest` / `macos-latest` outside `ci.yml`: 1 no-change (`release.yml:58`).
- **What the extracts asked research to read:**
  - The `a11y-violations-base` download has no producer: `grep -rn -e '-base' .github/workflows/` finds three
    downloads and two consumers and no upload under a `-base` name. The same holds for `criterion-Linux-base` and
    `coverage-linux-base`. Each download is `continue-on-error`.
  - `ci.yml` runs no ESLint step (`grep -n -i -e eslint -e 'run lint' .github/workflows/ci.yml`: 0 lines). The
    lint lives behind `cargo xtask lint` (`xtask/src/main.rs:144`, `:304`), which no job calls. No leg that leaves
    was its host.
  - `ci.yml` has no `check:ingest-progress` step (0 lines); the log-reading gates in CI are `cargo xtask ci-gates`
    in `boot` (`:455-456`) and `perf:budget` in `lint-test` (`:131-133`).
  - The condition at `ci.yml:514` sits on the system-library install step of `supply-chain`, not on the audit step
    (`:493-496`).
  - No harness file under `pulse-app/ui` or `xtask` reads a `ci.yml` matrix value: the only readers of the file are
    the three listed above.
  - Conditional-compilation lines naming Windows or macOS: 61 in 24 Rust files
    (`grep -rn -E 'cfg(_attr)?!?\(.*(windows|macos)'` over `crates`, `pulse-app/src`, `pulse-app/tests`,
    `pulse-app/examples`, `pulse-app/build.rs`, `xtask/src`). In test files they are runtime `cfg!` branches
    (8 lines in 5 files), not tests gated off Linux.
- **Local YAML tooling:** PyYAML 6.0.3 imports on the dev host; `actionlint` and `yamllint` are not on PATH. A
  job-list probe through PyYAML is already in the corpus
  (`andromeda-pulse-0.3.0/chunks/2026-09-29-ci-wall-time-and-round-trips/plan.md:181`).
- **Steps of the before-run** (`gh api …/actions/runs/37934330231/jobs`, each step's `conclusion`): every step of
  the six Linux jobs reads `success`, the three baseline downloads included. The only steps that read otherwise are
  10 `skipped` steps on the Windows and macOS legs (the system-library install and the three perf steps). Step
  counts on the Linux jobs: `lint / test` 37, `a11y` 21, `mcp-server tests` 18, `boot smoke` 20, `supply-chain` 23,
  `coverage gate` 22.
- **Artifacts of the before-run** (`gh api …/actions/runs/37934330231/artifacts`, 12 names): on Linux
  `a11y-violations-Linux`, `capability-drift-Linux`, `coverage-linux`, `logs-boot-Linux`, `logs-perf-samples-Linux`,
  `playwright-a11y-report-Linux`; the other six are the `-macOS` and `-Windows` twins of three of them. `logs-Linux`,
  `nextest-Linux` and `criterion-Linux` are NOT among them: their upload steps ran and found no file
  (`if-no-files-found: ignore`, `ci.yml:144-169`). That stands before this chunk and is not changed by it; it is
  why no criterion of this chunk asserts `logs-Linux`.

## New files to create
- none

## Files to modify
- `.github/workflows/ci.yml` — the two matrix jobs become single-runner Linux jobs, the `release` job leaves, the seven always-true system conditions and the other-system comments leave
- `pulse-app/tests/quality_gate_workflow.rs` — the release-job test leaves, the job count reads 6, one pin that the workflow names no other system is added

## Open questions
- none
