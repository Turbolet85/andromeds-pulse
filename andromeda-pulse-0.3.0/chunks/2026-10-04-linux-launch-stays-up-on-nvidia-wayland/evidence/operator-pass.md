# Operator pass — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

Fired 2026-10-04 by the implementing session on the overseer's word ("Run the operator pass now, entries 21-31:
hygiene, the six native pre-push stages with a fresh per-run PUPPETEER_CACHE_DIR, the bindings regen and base check
after stage 5, the pre-CI commit, push, CI read. Then stop and report."). The six native pre-push stages are the
founder deviation of 2026-10-04 (plan §Test Commands; `pre-push:linux` drives `wsl.exe` and cannot evaluate on the
Linux dev host). The rust-analyzer flycheck was checked before each heavy cargo stage; none was running, so nothing
was stopped. Every entry was fired by hand from the plan's exact `run` text; exits read from the unpiped command.

| plan entry | what | result |
|---|---|---|
| 21 | `gate.py hygiene` | first read `hygiene: refused 2 files — P1 0 · P2 0 · P3 2` (see §Hygiene) → after the rewrite `hygiene: clean — read 33 (runs 31 · evidence 2) · trails 12 not read · binary 0 not read by P1` |
| 22 | stage 1/6 script-modes | green — exit 0, `100755` |
| 23 | stage 2/6 source-lint | green — exit 0, `"verdict": "clean"`, 484 files, 0 hits |
| 24 | stage 3/6 npm, fresh `PUPPETEER_CACHE_DIR` = `target/pre-push/puppeteer-20261004T115850Z` (absolute, gitignored, `test ! -e` before use) | green — exit 0: `npm ci` added 800 packages; `npm run build` built; `pulse-app/ui` tree unchanged |
| 25 | stage 4/6 clippy | green — exit 0, 0 warning/error lines |
| 26 | stage 5/6 test | green — exit 0, `2592 tests run: 2592 passed, 0 skipped` (2580 + the 12 new fns) |
| 27 | stage 6/6 ci-gates over fresh `target/pre-push/data-20261004T120053Z` (SEED_LOG verbatim) | green — exit 0: zero-spans PASS (3 records) · zero-panic PASS · heartbeat-gap PASS (max 15000 ms vs 45000 ms) · perf-budget NEUTRAL (frame cannot-evaluate, no adapter record; memory and snapshot NEUTRAL: the seed carries no samples) |
| 28 | `--features mcp-server` bindings regen, after stage 5 | green — exit 0, `1 test run: 1 passed`. Before it, `git diff --quiet ffb62f0 -- bindings/index.ts` read 1: stage 5 had re-emitted the no-mcp shape, as predicted |
| 29 | base-identity close | green — exit 0, bindings byte-identical to `ffb62f0` |
| 30 | pre-CI commit + push | see §CI |
| 31 | `ci.py conclusion --sha HEAD --wait 2400` | see §CI |

## Hygiene
The first read refused two P3 rows, both in the phase run dir: `.andromeda/runs/2026-10-04T09-36-17Z-phase/ctl-main.rs`
(119 B, a minted `main()` whose first statement after a comment and a blank line is the apply call) and `ctl-posture.rs`
(92 B, two lines formatting `var_os(LEVER_ENV)`) — the known-positive controls P5 minted for the placement probe
(entry 5) and the log-content census (entry 7), described in those entries' `baseline` notes but cited by no path.
Per gate-contract §Hygiene (the letter removes or rewrites each listed file), they were RENAMED to `ctl-main.rs.txt` /
`ctl-posture.rs.txt` — content byte-identical, readable as controls, no longer a plane source file. Re-run: clean.

## CI
(entries 30-31; written after the push)

## Deviations
- Stage 3 ran with a fresh per-invocation `PUPPETEER_CACHE_DIR` under `target/pre-push/` (founder ruling 2026-10-04);
  `~/.cache/puppeteer` was not touched.
- Stage 6 ran over a fresh per-invocation data dir under `target/pre-push/` (no `rm`), per the plan's form.
- The two phase-run control files were renamed, not deleted (§Hygiene).
