# Operator pass — plan entries 26, 27, 28

Fired by the implementing agent on the overseer's word (founder-delegated, 2026-10-07, in the session): "Go: the
operator pass as planned: hygiene (26), the pre-CI commit and clean-tree push (27), the ci.py CI read (28). If a
CI job goes red on a runner or network fault with no link to this diff, re-run the failed job yourself once and
record both readings. Report the verdict and stop before the wrap." Each entry was driven once, by hand, in its
listed form.

## Entry 26 — hygiene (before the pre-CI commit)
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- fired: 2026-10-06T22:36:50Z · exit 0
- atoms: `exit 0` held · `contains hygiene: clean` held
- output, verbatim:

```
gate v1.11 · 0aca1113
root . · case exact · planes rust, ts · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 38 (runs 32 · evidence 3 · inputs 3) · trails 12 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

- This file was written after that read, so the read did not cover it. A second read with this file in the tree
  (not entry 26's record; fired before `git add -A`) printed `hygiene: clean — read 39 (runs 32 · evidence 4 ·
  inputs 3)` at exit 0.

## The pre-CI commit
- `git add -A`, then `fc4ebdf` — `chore(2026-10-06-npm-supply-chain-gate-is-green-again): operator pre-CI commit`,
  on `chore/migrate-pulse-to-v3`, parent `b1fcba5` (the chunk base).
- 47 files: 42 added, 5 modified. The one file outside `.andromeda/`, the version workspace and
  `.claude/session-handoff.md` is `pulse-app/ui/package-lock.json`.
- Before it: `gate.py scope` read `scope: clean — changed 1 · listed 1 · recorded 0`; the staged bindings carry
  the `mcp` namespace and equal the chunk base.

## Entry 27 — the clean-tree push
- run: `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`
- fired: 2026-10-06T22:37:17Z · exit 0 (default `expect`: `exit 0` held)
- output: `b1fcba5..fc4ebdf  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`
- read back: `git ls-remote origin chore/migrate-pulse-to-v3` printed
  `fc4ebdfe6b32964a82529f8cd8071c7041d084e6`, equal to local `HEAD`. Not a force push.

## Entry 28 — the CI read
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`
  (under a 2580 s bound, the entry's `timeout`)
- fired: 2026-10-06T22:37:22Z · returned 2026-10-06T23:05:47Z · exit 0
- atoms: `exit 0` held · `contains verdict: green` held
- output, verbatim (the tool clipped its own fourth line):

```
ci v1.0 · 84a1732c
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 56× over 1705 s
fc4ebdfe6b32 verdict: green · checks 13/13 · wall 1678 s · runs secret-scan#37541745786 completed/success ci#3754174567…
runs: secret-scan#37541745786 pull_request completed/success · ci#37541745674 pull_request completed/success
```

- The run the acceptance names: **ci#37541745674** on `fc4ebdfe6b32964a82529f8cd8071c7041d084e6`,
  `completed/success`, attempt 1. No job was re-run; the overseer's one-re-run allowance was not used.
- Its twelve jobs, each `success` (read through `gh api …/actions/runs/37541745674/jobs`): `supply-chain (audit +
  deny + auditable)` · `a11y` on ubuntu-22.04, macos-latest, windows-latest · `lint / test` on ubuntu-22.04,
  macos-latest, windows-latest · `release build` on macos-latest, windows-latest · `mcp-server tests` ·
  `boot smoke` · `coverage gate`. The thirteenth check is secret-scan#37541745786.
- The `supply-chain` job (112536140445), every step RAN — none `skipped`: 9 `cargo audit` · 10 `cargo deny check
  bans licenses sources` · 11 the Cranelift assertion · 14 `cargo xtask check:npm-supply-chain` · 15 `npm ci
  (pulse-app/ui)` · 16 `npm run build` · 17 `Install cargo-auditable` · 18 `cargo auditable build smoke`, each
  `success`. On the red commits step 14 read `failure` and 15–18 `skipped`.
- So the plain `npm ci` (no `PUPPETEER_SKIP_DOWNLOAD`), on Node 24, installed the changed lockfile on the runners
  in all the jobs that install it, which this host could not witness.
- The step-14 log on the runner was not opened: its arm there is read from the step's `success`, not from its
  printed JSON.
