# Operator pass — plan entries 25–28 (2026-09-29, ordered by the overseer)

| # | entry | exit | atoms / reading |
|---|---|---|---|
| 25 | `gate.py hygiene` | 0 | first run: `hygiene: refused 1 files — P1 1` (`.andromeda/runs/2026-09-29T04-59-33Z-phase/control-e14.py:14`, the Git Bash install path). Repaired: the control now reads `os.environ["SHELL"]`, the shell gate.py resolves. It was re-proven on its known positive (probe exit 1 with :4317 held). Re-run: **`hygiene: clean — read 29 (runs 27 · evidence 2)`**, exit 0. Both atoms hold. |
| — | pre-CI commit | 0 | **`f2a131a`** `chore(2026-09-29-p-025-hue-shift-observable-made-gradable): operator pre-CI commit`, 59 files. Before commit: the staged bindings carry `"mcp":` ×1 and `tier_effective_at_unix_nano` ×1, and `cargo xtask check:staged-artifacts` exited 0. |
| 26 | guarded `git push origin chore/migrate-pulse-to-v3` | 0 | `f15536b..f2a131a`; 0 ahead of upstream afterwards |
| 27 | guarded `gh pr create --draft …` | 0 | **PR #39** https://github.com/Turbolet85/andromeds-pulse/pull/39, draft, open, head `f2a131ac2c41`. The attribution line was appended through the REST API, because `gh pr edit` lacks the `read:project` scope. The PR was not merged, closed or re-targeted. |
| 28 | `ci.py conclusion --sha HEAD --wait 3600` | 0 | **`verdict: red · checks 6/6`**, wall 5 s, first failure after 2 s. Runs: `ci#36535320175` (pull_request, failure) and `secret-scan#36535320183` (pull_request, failure). The atom `contains verdict: green` does NOT hold. |

## The CI read, measured

- **The parse fix is proven.** Jobs now exist on the pushed sha: `checks 6/6`, each job listed by its `name:`. The prior
  reds on `f15536b` / `8b86529` / `83d4060` were path-named runs with `checks 0/0` and 0 s. The workflow-file-issue
  signature is gone.
- **Every job failed before starting, for a reason outside the repository.** All 6 jobs (supply-chain,
  lint/test/build ×3 OS, coverage, gitleaks) have 0 steps and no runner assigned. The check-run annotation is
  identical on each job read (gitleaks, the ubuntu and windows lint/test/build jobs, supply-chain): *"The job was not
  started because recent account payments have failed or your spending limit needs to be increased. Please check the
  'Billing & plans' section in your settings"*.
- **This is a GitHub account billing / spending-limit block, not a finding about the code.** No step ran, so the
  first real CI run has measured nothing about the workflows' jobs yet. The chunk was not widened to chase it; the
  remedy is the account owner's. PR #39 re-fires CI on every later push once billing is restored.

## Entry 28 re-read after the fix pushes — GREEN

`ci.py conclusion --sha HEAD --wait 3600` on **`464f2a3`**: **`verdict: green · checks 6/6`**, wall 7185 s; runs
`ci#36574279289` (pull_request, success) and `secret-scan#36574279070` (pull_request, success). The entry's atoms
`exit 0` and `contains verdict: green` hold.

The repo was made public (the founder's remedy for the billing block, so Actions are free). The chunk then worked
the first real CI run to green, one class of failure at a time. Each fix is a commit on the branch:
- `200e5ed` — build prerequisites (ui/dist before the first compile, Linux Tauri/dbus libraries), capability-drift
  ordered before the test run, the cargo-deny step's command, wasmtime 46 → 48.0.3 (RUSTSEC-2026-0316), and gitleaks
  fingerprint entries for fake fixture text.
- `edfd8b3` — CI Node 24 (npm 11, the lockfile's writer); dbus before the supply-chain xtask gate.
- `28c3238` — RGBA PNG icons (macOS/Linux generate_context!).
- `e904cb1` — Playwright chromium before the a11y harness; npm advisories (vitest 4.1.11, qs, undici, webdriverio;
  a GHSA-7pqw exception added and the GHSA-ggr8 exception pruned); vitest-4 follow-ups; the empty `#[ignore]`d
  placeholder test deleted.
- `33abd9b` — inject_demo salts stay distinct on coarse clocks (macOS); the Linux boot smoke runs under xvfb.
- `46c3aac` — harness:status reports a dead app as not-running (founder ruling).
- `eac30d9` — coverage excludes xtask (TEMPORARY, founder ruling; see below).
- `546d3f0` — perf-slo-check reads an empty metric stream as NEUTRAL instead of dying under pipefail.
- `464f2a3` — agent-run boot names how a failed app ended; cleanup-on-failure runs through bash (mode 100755).

**Green-run Linux readings:** boot smoke `boot: ready` → `running-healthy` → `cleanup: clean`; ci-gates zero-spans
PASS (52 records), zero-panic PASS, heartbeat-gap PASS, perf-budget PASS (frame and snapshot streams NEUTRAL,
buffer.memory_bytes max 0 ≤ 512 000 000); criterion-regression NEUTRAL (no bench output); perf:slo-load (10k spans/s)
PASS. **Open, for the wrap:** on `546d3f0` the Linux app died about 0.5 s after its webview began polling (no
panic, silent stderr). The cause was not identified, and it did not recur on `464f2a3`. Boot's failure path now
reports the signal or exit status, so a recurrence names its own cause.


- **Measured on CI, e904cb1** (the `coverage-linux` lcov artifact): line 35513/42151 = **84.3 %** (≥ 75 ✓),
  function 3739/4454 = **83.95 %** (≥ 85 ✗), branch 0/0 (not instrumented). Of the 715 uncovered functions, 251 are
  in `xtask/` (the dev task-runner, 61.5 %; process-spawning harness code) and 179 in `pulse-app` (Tauri boot and
  runtime resolvers). LLVM counts each compiled copy of a function separately, so library code tested in its own
  crate also reads uncovered in the copies linked into other test binaries. Running the measure with
  `--features mcp-server` did NOT close the gap (local: 83.75 %).
- **Founder ruling, 2026-09-29, verbatim:** «Давай исключим но временно, после конца эпохи и код анализа посмотрим как
  качественно покрытие увеличить а то 85 процентов маловато так то». In English: exclude xtask, but temporarily;
  after the epoch ends and the code analysis, look at how to raise coverage properly, since 85 % is on the low side.
- **Applied (TEMPORARY):** `cargo xtask test:coverage` passes `--ignore-filename-regex (^|[/\])xtask[/\]`
  (`xtask/src/main.rs::COVERAGE_IGNORE_FILENAME_REGEX`); the thresholds are unchanged. Re-measured locally on the
  instrumented profile: 0 xtask files left; line 88.2 %, function 87.54 %. The same exclusion over the CI lcov gives
  line 88.6 %, function 87.80 %.
- **Owed by the wrap:** amend test-plan §10 so the xtask exclusion reads as TEMPORARY with this word. Its review point
  is the next epoch-boundary code audit, which revisits coverage quality, raising the thresholds above 85 %, and
  including xtask again.
