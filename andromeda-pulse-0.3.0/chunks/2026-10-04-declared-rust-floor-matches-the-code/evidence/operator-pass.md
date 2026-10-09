# Operator pass — plan entries 24–35

Overseer go (founder-delegated), 2026-10-04: "run the operator pass, entries 24-35, as planned". Every entry was
fired by hand with its exact plan `run` text, in block order, on the Linux dev host. Exit codes are read from the
bare command; printed verdicts quoted.

| entry | stage | fired (UTC) | exit | atoms / verdict |
|---|---|---|---|---|
| 24 | hygiene | 22:00:55 → re-fired | 0 | first read `hygiene: refused 1 files — P1 1` (`evidence/residue-removal.md:7`, the `tmp` predicate on the `ls` transcript's literal temp paths); the 3 literals replaced with a labelled `{tmp}` placeholder; re-fired: `hygiene: clean — read 28` ✓ |
| 25 | pre-push 1/6 script-modes | 22:01:22 | 0 | `100755 … scripts/agent-run.sh` ✓ |
| 26 | pre-push 2/6 source-lint | 22:01:23 | 0 | `"verdict": "clean"`, `files_scanned` 485, `hits` 0 ✓ |
| 27 | pre-push 3/6 npm | 22:01:31 → 22:02:38 | 0 | fresh `PUPPETEER_CACHE_DIR=…/target/pre-push/puppeteer-20261004T220131Z`; `npm ci` added 800 packages; `npm run build` `✓ built in 1.74s`. Standing `10 high severity vulnerabilities` line (handoff; CI `supply-chain` owns it) |
| 28 | pre-push 4/6 clippy | 22:03:02 → 22:03:03 | 0 | `Finished`, no warning ✓ |
| 29 | pre-push 5/6 test | → 22:03:34 | 0 | `2630 tests run: 2630 passed, 0 skipped` ✓ (`contains passed` ✓). Re-emitted the no-mcp bindings (−22 lines), as predicted |
| 30 | pre-push 6/6 ci-gates | 22:04:03 | 0 | zero-spans PASS · zero-panic PASS · heartbeat-gap PASS (max 15000 ms) · perf-budget NEUTRAL (frame cannot-evaluate: no adapter record; memory/snapshot NEUTRAL) |
| 31 | `{tmp}` residue census | 22:04:09 | 2 | report-only (`expect = []`): `No such file or directory` — no corpus-key lock of any name; reading in `residue-removal.md` |
| 32 | bindings regen (re-fired) | 22:04:47 | 0 | `1 test run: 1 passed`; `"mcp":` count 1 |
| 33 | base-identity close (re-fired) | 22:04:47 | 0 | `git diff --quiet 46b600a… -- pulse-app/ui/src/bindings/index.ts` ✓ |
| 34 | push | 22:05:34 (after commit `e9ec090`) | 0 | clean-tree guard held; `46b600a..e9ec090  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`; `HEAD` == `origin/chore/migrate-pulse-to-v3` == `e9ec0905…` |
| 35 | CI read (`ci.py conclusion --sha HEAD --wait 2400`) | 22:05:41 → 22:33:03 | 0 | `e9ec09058bee verdict: green · checks 13/13 · wall 1631 s` (ci#37238688476 + secret-scan#37238688539, `pull_request`) ✓ — `contains verdict: green` ✓. Jobs (`gh run view 37238688476`): lint / test on ubuntu-22.04, macos-latest, windows-latest · release build macos + windows · boot smoke · mcp-server tests · coverage gate · supply-chain · a11y ×3 — all `success` |

Entries 34–35 were written into this file at the wrap (2026-10-04T22:5xZ), from the operator-pass transcript of the same session; the commit `e9ec090` they measured predates this edit, which rides the chunk commit.

Entries 20–21 (the boot-smoke precondition and the Xvfb smoke) were fired at /implement P2: `boot-smoke.md`.
