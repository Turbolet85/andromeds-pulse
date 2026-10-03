# Operator pass — 2026-10-02-incident-events-readable-through-mcp

Run by the session on the overseer's word (2026-10-02: "Run the operator pass in the approved plan: entries 17-22
(hygiene, pre-push:linux, the pre-CI commit, S = rev-parse HEAD, push, CI read). Keep the disk guard. Then stop and report
S"), after /implement closed green + smoke. The disk guard (stop below 15 GB free on D:) held throughout. No pulse-app
process was launched; 4317/4318 stayed closed.

## Gate 17 — hygiene
`python -X utf8 …/gate.py hygiene`: exit 0, `hygiene: clean — read 29 (runs 27 · evidence 2)`; re-fired after this file
was written (see below).

## Gate 18 — `cargo xtask pre-push:linux`
Exit 0, verdict document `"verdict": "green"`, six stages (script-modes · source-lint · npm · clippy · test ·
ci-gates). The WSL `test` stage ran the workspace suite at 2573/2573 passed — the Windows 2571 plus the two Unix-only
exit-cause arms — including this chunk's 11 feature-free pins (5 corpus `load_incident_events_*`, 6 dispatcher
`retrieve_incident_events_*`).

## Gate 19 — S
After the pre-CI commit (`git add -A` over the chunk's work: source, tests, chunk folder, the phase and implement run
dirs, the /phase route/handoff/ledger edits): `git rev-parse HEAD` exit 0, **S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`**,
parent `a69030a`. Recorded in `round-binary.md`.

## Gate 20 — push
Clean-tree guard held (`git status --short` empty after the commit); `git push origin chore/migrate-pulse-to-v3` exit 0,
`a69030a..4a26ad8`; origin head = S.

## Gate 21 — CI on S — RED, external advisory decay
`ci.py conclusion --sha HEAD --wait 2400`: exit 0, `4a26ad8f1d9c verdict: red` — first fail +87 s, `supply-chain (audit +
deny + auditable)`. The entry's `contains verdict: green` atom does NOT hold. The run was then read to completion
(`gh run view 37069724167`, 21:56:14Z → 22:22:36Z): `ci#37069724167` conclusion failure — 11 of 12 jobs success (lint /
test ×3, boot smoke, mcp-server tests, coverage gate, a11y ×3, release build ×2), 1 failure (supply-chain);
`secret-scan#37069723897` success.

**The failure.** `cargo audit` (advisory DB `f8dee89e1b2f`, updated 2026-10-02T22:27:46+02:00, 1288 advisories) found 3
vulnerabilities, all `wasmtime 48.0.3`, all dated **2026-10-02**, all with a stated safe upgrade (`>=48.0.4, <49.0.0` or
`>=49.0.2`): RUSTSEC-2026-0325 (mis-typed tag imports → GC heap corruption), RUSTSEC-2026-0326 (GC rooting across
`try_call`), RUSTSEC-2026-0327 (component async-lifted callback result count → native stack buffer overflow). Distinct
ids, not error blocks. The informational set (8 unmaintained, 2 unsound) is the standing one.

**Not this chunk's — the basis.** `Cargo.lock` is byte-identical between `a69030a` and S (plan gate 6 printed nothing for
`git diff --name-only a69030a -- Cargo.lock …`), and `cargo audit`'s verdict is a function of the lockfile and the
advisory DB alone; the base's own CI (`ci#37012645910`, green) read a DB that predates these three advisories. So the
same three findings land on the base against today's DB, by construction. Disposition is not this pass's: per
security-plan §Dependency Security a finding with a stated safe upgrade is never ignore-listed — it takes a named owner
and stays red until upgraded (a lockfile bump, `wasmtime` 48.0.3 → 48.0.4, outside this chunk's scope guard).

## Gate 22 — Conductor evidence for S
Not run here: it reads Conductor's committed round, which follows the overseer's relay of S and `round-request.md`.
**Superseded:** S was never relayed for a round — assertion 7 failed on S (the `created` event read `unknown`), Conductor
held its drive, and the round runs once against S2 (below; `premise-correction.md`).

---

# Operator pass to S2 — 2026-10-03, Linux host

Run by the session on the overseer's word (2026-10-03, founder word: "Run the operator pass now: entry 17 hygiene and
entries 21/22 with the path remaps, entry 18 stages natively recorded as a deviation (founder ruling), the pre-CI commit,
S2 = rev-parse HEAD, push, CI read"), after the fix-A tree re-verified green on this host (`mutation-checks.md` §Re-verification
on the Linux host, run `implement-2026-10-03T21-54-37Z`: 15 green, entry 6 red by the founder's widening). Host: Omarchy
Linux, repo `~/dev/projects/andromeda-pulse`; the disk guard (stop below 15 GB free on `/home`) held throughout (834 GB
free); no pulse-app process was launched; 4317/4318 stayed closed.

**Path remaps (operator's word; the plan's text is unchanged).** Entries 17 and 21 name the tools under the Windows
host's user profile; on this host they are at `~/.claude/skills/andromeda-tools/scripts/` — a path remap only. Entry 22
names Conductor on the Windows host's dev drive; here it is `~/dev/projects/conductor`.

## Gate 17 — hygiene
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`: exit 0, `hygiene: clean — read 9 (runs 4 ·
evidence 5)`; re-fired after this section was written (see the commit's own hygiene read).

## Gate 18 — DEVIATION (founder ruling 2026-10-03): the six `pre-push:linux` stages run natively
`cargo xtask pre-push:linux` cannot run on this host as written: it drives every stage through `wsl.exe` into an `Ubuntu`
distro clone (`xtask/src/pre_push.rs`) and is cannot-evaluate off Windows. Per the founder's ruling (2026-10-03, relayed by
the overseer), its six stages ran NATIVELY in this working tree, xtask untouched; porting the verb to native Linux is its
own route entry after P1. The mirror (a scratchpad script) reproduces `Stage::ALL` in order, stopping at the first failure,
and gives each stage child the same isolation the WSL form does — `env -i` with only `HOME`, a Linux `PATH` and one
`ANDROMEDA_PULSE_DATA_DIR` — with the `ci-gates` data dir seeded from `SEED_LOG` verbatim. Two deliberate differences
from the WSL form: the stages run in the working tree rather than a clone synced to HEAD + the working tree (same content,
no patch transport), and they share this host's `target/` and `~/.cache`.

**The `env -i` postinstall difference (named per the overseer).** In this session's ordinary environment, `npm ci` in
`pulse-app/ui` printed `npm warn install-scripts` listing esbuild / puppeteer / geckodriver postinstalls as held back —
yet it had in fact started puppeteer 24.43.1's browser download and exited 0 leaving
`~/.cache/puppeteer/{chrome,chrome-headless-shell}/linux-148.0.7778.97` PARTIAL (21 MB / 1.6 MB, no executable; the
v152 pair for pa11y's puppeteer 25.9.0 completed). Under the stage's `env -i`, the same `npm ci` ran puppeteer's
postinstall to completion-or-error, found the partial folder ("exists but the executable … is missing") and exited 1.
So the first native run went red at `npm` (`script-modes` and `source-lint` green, the rest not run). The founder deleted
exactly the two partial v148 folders; the stages were re-run from the start with nothing else changed and no
`PUPPETEER_SKIP_DOWNLOAD`. A fresh host's puppeteer cache is therefore a precondition the WSL clone never exercised.

**Reading (the re-run):** `verdict green`.

| stage | form | exit | s |
|---|---|---|---|
| script-modes | `git ls-files -s scripts/agent-run.sh` | mode `100755` | — |
| source-lint | `cargo xtask check:english-sources` | 0 (`"verdict": "clean"`) | 0 |
| npm | `npm ci` && `npm run build` (pulse-app/ui) | 0 && 0 | 120 + 3 |
| clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 0 | 1 |
| test | `cargo xtask test` | 0 — `2575 tests run: 2575 passed, 0 skipped` | 17 |
| ci-gates | `cargo xtask ci-gates` over the seeded data dir | 0 — zero-spans PASS · zero-panic PASS · heartbeat-gap PASS (max 15000 ms) · perf-budget NEUTRAL (no arm required) | 0 |

**Its one side effect, repaired by the plan's own entries.** The `test` stage is a default-features workspace nextest, so
it rewrote `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape (the known regen trap). Gate 14's regen
(`cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'`, 1 passed) and
gate 15's probe (`git diff --quiet a69030a -- pulse-app/ui/src/bindings/index.ts`, exit 0) restored it byte-identical
to the base before the commit.

## Gate 19 — S2
Printed after the pre-CI commit; recorded in `round-binary.md`.

## Gate 20 — push
Recorded below with the CI read.
