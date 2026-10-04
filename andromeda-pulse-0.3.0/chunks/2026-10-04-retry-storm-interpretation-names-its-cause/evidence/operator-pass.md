# Operator pass — 2026-10-04-retry-storm-interpretation-names-its-cause

The plan's `leg = 'operator'` entries (17-27) were fired by hand on 2026-10-04 at the overseer's direction. Each
`run` was fired as listed. Exits were read from the bare command (output redirected to a file, never piped).
The chunk base is `e71dba539a7d0379d960b49b37b2cf3538de6ec2`, and HEAD equalled it until the pre-CI commit.

| # | entry | exit | atoms | verdict |
|---|---|---|---|---|
| 17 | `gate.py hygiene` | 0 | `contains hygiene: clean` — first run: NO (`refused 2 files`); re-run: yes | green after remedy |
| 18 | stage 1/6 script-modes: `git ls-files -s scripts/agent-run.sh` | 0 | `contains 100755`: yes | green |
| 19 | stage 2/6 source-lint (`env -i`): `cargo xtask check:english-sources` | 0 | `"verdict": "clean"` (484 files, 0 hits): yes | green |
| 20 | stage 3/6 npm (`env -i`, fresh `PUPPETEER_CACHE_DIR=target/pre-push/puppeteer-20261004T142505Z`): `npm ci && npm run build` | 0 | — (added 800 packages; `built in 1.93s`) | green |
| 21 | stage 4/6 clippy (`env -i`) | 0 | — (0 warnings, 0 errors) | green |
| 22 | stage 5/6 test (`env -i`): `cargo xtask test` | 0 | `contains passed`: yes — `2601 tests run: 2601 passed, 0 skipped` | green |
| 23 | stage 6/6 ci-gates (`env -i`, fresh data dir `target/pre-push/data-20261004T142658Z`, seeded with SEED_LOG) | 0 | — (zero-spans / zero-panic / heartbeat-gap PASS; perf-budget NEUTRAL, frame cannot-evaluate: 0 samples, no adapter record) | green |
| 24 | bindings regen (`--features mcp-server`, re-fired after stage 5) | 0 | — (`1 test run: 1 passed`) | green |
| 25 | base-identity close: `git diff --quiet e71dba5… -- pulse-app/ui/src/bindings/index.ts` | 0 | — | green |

## Entry 17 — the hygiene remedy

The first hygiene run refused two of the phase run's captured dry-run outputs:
`.andromeda/runs/2026-10-04T12-47-24Z-phase/p4-dryrun.txt` and `p5-dryrun.txt`, each `P1 ×3 · tmp,home`. Both are
`gate.py run` stdout captures. Their header lines named the gate log dir under `/tmp` and the repo root under
`/home`, and two entry rows named the skills dir under `/home`. The host paths were replaced with labelled
placeholders (`<gate-log-dir>/`, `<skills-dir>/`, `<repo-root>`), and every other byte was left as it was. The
re-run read `hygiene: clean — read 39`.

## Observations, not this chunk's

- Stage 3's `npm ci` printed `10 high severity vulnerabilities`. No npm manifest or lockfile changed in this
  chunk. The npm channel's graded gate is `cargo xtask check:npm-supply-chain` in the CI `supply-chain` job, which
  the CI read below covers.
- Stage 5 rewrote `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape, as the plan predicted. Entries 24-25
  restored it, byte-identical to the base.
- Stage 5 ran under `env -i`, which drops the session bus, so it exercised no credential-store leg. This is the
  known handoff fact.

## Entries 26-27 (push, CI read)

Recorded below after the pre-CI commit.
