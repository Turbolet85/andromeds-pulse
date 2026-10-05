# Operator pass

Overseer go (founder-delegated), 2026-10-05: "run the operator pass as planned: re-check evidence/ before the commit,
hygiene, the six native pre-push stages with a fresh per-run PUPPETEER_CACHE_DIR under target/pre-push/ for stage 3,
the bindings regen and base check after stage 5, the pre-CI commit, push, and the ci.py CI read."

This chunk's plan lists only the hygiene entry (21) as an operator entry. The pre-push stages, the regen and the close
were fired by hand in the exact `run` text of the precedent operator pass (`2026-10-04-declared-rust-floor-matches-the-code`
plan entries 25–33), with this chunk's base `200312b` in the close. Fired on the Linux dev host, in order. Exit codes
were read from the bare command, and printed verdicts are quoted.

| step | stage | fired (UTC) | exit | atoms / verdict |
|---|---|---|---|---|
| 0 | evidence re-check | 05:34 | — | the three `*-runs.json` copies are byte-equal (`cmp`) to their `target/l4-decision-probe/{run}/runs.json` sources; the recount from the copies is unchanged (Slot 1 28/37/36/33 · Slot 2 34/40 · held-out 20/20); `series.md` has 0 prompt-text hits and 0 host-path hits; row keys are the bounded label set only |
| 1 | hygiene (plan entry 21) | 05:34:19 | 0 | `hygiene: clean — read 36 (runs 28 · evidence 4 · inputs 4)` ✓ |
| 2 | pre-push 1/6 script-modes | 05:34:25 | 0 | `100755 … scripts/agent-run.sh` ✓ |
| 3 | pre-push 2/6 source-lint | 05:34:25 | 0 | `"verdict": "clean"`, `files_scanned` 485, `hits` 0 ✓ |
| 4 | pre-push 3/6 npm | 05:34:32 → 05:35:38 | 0 | fresh `PUPPETEER_CACHE_DIR=…/target/pre-push/puppeteer-20261005T053432Z`; `npm ci` added 800 packages; `npm run build` `✓ built in 1.68s`. Standing `10 high severity vulnerabilities` line (CI `supply-chain` owns it) |
| 5 | pre-push 4/6 clippy | 05:35:44 → 05:35:45 | 0 | `Finished`, 0 warning or error lines ✓ |
| 6 | pre-push 5/6 test | 05:35:50 → 05:36:35 | 0 | `2639 tests run: 2639 passed, 0 skipped` ✓. Re-emitted the no-mcp bindings (1 insertion, 22 deletions), as predicted |
| 7 | pre-push 6/6 ci-gates | 05:36:42 | 0 | zero-spans PASS · zero-panic PASS · heartbeat-gap PASS (max 15000 ms) · perf-budget NEUTRAL (frame cannot-evaluate: no adapter record; memory and snapshot NEUTRAL) |
| 8 | bindings regen (after stages 5–6) | 05:36:47 | 0 | `1 test run: 1 passed, 14 skipped`; `"mcp":` count 1 |
| 9 | base-identity close | 05:36:53 | 0 | `git diff --quiet 200312b… -- pulse-app/ui/src/bindings/index.ts` ✓ |

Hygiene was re-fired after this file was written (an evidence edit), before the pre-CI commit.
