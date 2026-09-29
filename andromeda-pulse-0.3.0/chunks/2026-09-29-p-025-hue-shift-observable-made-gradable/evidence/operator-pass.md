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
