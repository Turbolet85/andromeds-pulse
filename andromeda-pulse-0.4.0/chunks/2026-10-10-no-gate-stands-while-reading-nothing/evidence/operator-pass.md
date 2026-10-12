# Operator pass — entries 24 to 34, 2026-10-10

Run by the session on the operator's word (the pc overseer, 2026-10-10, given in the session: "Run the operator
pass, entries 25 to 34 in block order … then write the P-128 ref"). Every entry but the seeded pair was fired
through the gate tool with no run dir; the seeded pair (25, 28) was fired as written. Each line below is the tool's
own, without the header. Nothing was fixed on top, and no run was re-run.

**The run this chunk's verdict reads: `ci#38042949735`** — pull-request event, attempt 1, on the pushed tip
`d99d2dcb5b33692558fe19729c32d8c2ed0568f8` (the pre-CI commit; the run built its merge with `main`, merge commit
`45ea78a7eb66`), completed/success, beside `secret-scan#38042949743`, completed/success.

## 25 — hygiene, before the pre-CI commit (as written)

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`

```
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 45 (runs 38 · evidence 3 · inputs 4) · trails 14 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`git add -A` after the clean read, then
`chore(2026-10-10-no-gate-stands-while-reading-nothing): operator pre-CI commit` →
`d99d2dcb5b33692558fe19729c32d8c2ed0568f8` on `build/andromeda-pulse-0.4.0`, parent `7419496b` (the chunk base).
59 paths. The tree read clean after it (`git status --short`: 0 lines).

## 24 — the local pre-push check again, on the committed tree

```
 24 integration green · exit 0 · 86.62s · 1539179 B → 24.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```

Its verdict object, from the log: `"head": "d99d2dcb5b33692558fe19729c32d8c2ed0568f8"`, `"reason":
"all-stages-ok"`, `"verdict": "green"`. The tree read clean after it.

## 26 — the merge-base probe, directly before the push

```
operator entry 26 · history tripwire: none of 7 forms matched — not a clearance
 26 probe       green · exit 0 · 0.63s · 0 B → 26.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```

Exit 0: the build branch contained the remote's `main` as it stood then.

## 27 — the push

```
operator entry 27 · history tripwire: git
 27 probe       green · exit 0 · 2.31s · 135 B → 27.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 7419496b→d99d2dcb · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```

Its output, from the log (the entry has no `expect` key, so its line asserts the exit alone):

```
To https://github.com/Turbolet85/andromeds-pulse.git
   7419496b..d99d2dcb  build/andromeda-pulse-0.4.0 -> build/andromeda-pulse-0.4.0
```

The one ref moved is the intended one, fast-forward. Read back after it:
`git ls-remote origin refs/heads/build/andromeda-pulse-0.4.0` → `d99d2dcb5b33692558fe19729c32d8c2ed0568f8`.

## 28 — the CI read (as written, in the background)

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`

```
ci v1.2 · 72093aa2
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 59× over 1798 s
d99d2dcb5b33 verdict: green · checks 7/7 · wall 1785 s · runs secret-scan#38042949743 completed/success ci#38042949735 …
runs: secret-scan#38042949743 pull_request completed/success · ci#38042949735 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

Exit 0, `verdict: green`. One wait; it was not re-fired.

## 29 to 34 — the reads by run id (`--id 38042949735`)

Each call's tripwire line read `none of 7 forms matched — not a clearance`, and each entry's line `history: unmoved`.

```
 29 probe       green · exit 0 · 0.36s · 129 B → 29.log · history: unmoved · gh api "repos/Turbolet85/andromeds-pulse/actions/runs/<id>… (129 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
Output: `a11y-violations-Linux capability-drift-Linux coverage-linux logs-boot-Linux logs-perf-samples-Linux playwright-a11y-report-Linux`

```
 30 probe       green · exit 0 · 9.0s · 4 B → 30.log · history: unmoved · d="$(mktemp -d)" && for j in $(gh api "repos/Turbolet85/an… (380 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
Output: `6 0` — six job logs fetched, each non-empty, no line reading NEUTRAL, "trivially passes", a `Branch:`
percentage or the pass-on-empty flag. Its known-positive control, measured at P4 on `ci#38038281709`: `6 9`.

```
 31 probe       green · exit 0 · 1.91s · 4 B → 31.log · history: unmoved · f="$(mktemp)" && gh api --allow-escape-sequences "repos/Tu… (391 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
Output: `1 1` — the coverage step printed one line percentage and one function percentage, each over a non-zero total.

```
 32 probe       green · exit 0 · 1.77s · 6 B → 32.log · history: unmoved · f="$(mktemp)" && gh api --allow-escape-sequences "repos/Tu… (437 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
Output: `1 1 0` — in the boot job the verb printed its record-count PASS line over at least one record, its panic
PASS line, and no heartbeat or perf-budget line. Control at P4 on `ci#38038281709`: `1 1 6`.

```
 33 probe       green · exit 0 · 2.25s · 6 B → 33.log · history: unmoved · f="$(mktemp)" && gh api --allow-escape-sequences "repos/Tu… (482 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
Output: `1 1 3` — in `lint-test` the quarantine check printed PASS over at least one file, the perf gate graded
samples, and each of the three nextest steps ran at least one test. Control at P4 on `ci#38038281709`: `0 1 3`.

```
 34 probe       recorded · exit 0 · 0.58s · 545 B → 34.log · history: unmoved · gh api "repos/Turbolet85/andromeds-pulse/actions/runs/<id>… (159 chars)
entries 34 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 33
```
Output (report-only, `expect = []`):

```
lint / test (ubuntu-22.04) · success · 2026-10-10T09:52:57Z · 2026-10-10T10:00:29Z
coverage gate (line ≥75% / function ≥85%) · success · 2026-10-10T09:52:58Z · 2026-10-10T10:22:42Z
mcp-server tests (ubuntu-22.04) · success · 2026-10-10T09:52:57Z · 2026-10-10T09:59:03Z
a11y (ubuntu-22.04) · success · 2026-10-10T09:52:58Z · 2026-10-10T09:57:25Z
supply-chain (audit + deny + auditable) · success · 2026-10-10T09:52:58Z · 2026-10-10T10:00:42Z
boot smoke (ubuntu-22.04) · success · 2026-10-10T09:52:58Z · 2026-10-10T10:07:39Z
```

## The boot job

Green on its first and only attempt: the smoke read `"verdict": "settled"` with `"windows_settled": 4`, the series
read `"verdict": "all-settled"` with `"boots": 7`, `"settled": 7`, `"ended": 0`, `"other": 0`. No self-end was read,
so nothing was brought to the operator from it. Its artifact was not opened (the exit-witness files are read whole
or not at all); the readings above are the job log's.

## P-128, clause by clause, on this run

- **(a)** `ci.yml` holds 0 download steps (`grep -c download-artifact`: 0; `ci_workflow_makes_no_download_without_a_producer`
  among the 2908 tests the run passed), and the `a11y` job is green with
  `regression-detector: no new violations vs baseline (0/0)` over the baseline in the tree.
- **(b)** exactly the six artifacts (entry 29); the six uploads found 1, 1, 5, 1, 42 and 1 files (`gate-census.md`).
- **(c)** `verdict: green` (entry 28); `6 0` (entry 30); `1 1` (entry 31); `1 1 0` (entry 32); `1 1 3` (entry 33).
- **The pins in the tree at that commit:** the run's `cargo xtask test` passed 2908 tests with them; their red
  readings are in `red-before.md` (an empty coverage report, an empty selection, an absent search root) and
  `mutation-checks.md` (all four, the absent log family among them).

## Limits of this pass

- One run was read, attempt 1. No second attempt exists.
- The run read is the pre-CI commit's. The wrap's chunk commit follows it and carries no source change; its run is
  not this record's.
- `gate-census.md` was written from the six job logs fetched once more to scratch files, by hand, beside entry 30
  (whose own fetch is inside the entry). Those files read `0` for the same four tokens.
- Outside the acceptance's token list, seen on the run: the `lint-test` budget step prints
  `perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log` beside its PASS (the frame arm is
  not required there; `memory` and `snapshot` are, and each read samples).
