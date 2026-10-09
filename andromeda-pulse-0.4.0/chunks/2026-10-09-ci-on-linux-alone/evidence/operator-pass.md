# The operator pass (plan Step 7) — entries 17 to 22

Run on the operator's word, given in this session after /implement's report (the operator, 2026-10-09):

> Disposition for hygiene: in .andromeda/runs/2026-10-09T14-05-04Z-phase/relay-1.md respell the home prefix of my
> path to a tilde - one anchored edit, nothing else in the line - and say so in evidence/operator-pass.md. Then run
> the operator pass, plan.md Step 7 in order: hygiene, the pre-CI commit, the push, the CI read, the three reads of
> the after-run; fill the After table. Start NO skill after it - the wrap waits for my word.

Each entry was driven once by hand; its `run` is written in the spelling the gate tool prints (the home as `~`).

The tree it ran on: the chunk base `0b61bfb` plus the chunk's edits (2 source files, 53 insertions, 88 deletions).
The gate block before it: 16 green, 0 red, 6 operator legs not run; the scope read `clean — changed 2 · listed 2`.

## The respell, before entry 17

- A preview hygiene read at the end of /implement printed `hygiene: refused 1 files — P1 1`, its one row
  `P1 .andromeda/runs/2026-10-09T14-05-04Z-phase/relay-1.md:1 ×1 · home`: the operator's invocation message, kept
  verbatim by the phase run, names the relay file by an absolute path under the home directory.
- On the word quoted above, one anchored edit of that file, line 1: the home prefix of that one path was replaced
  by `~`, so the path now opens `~/dev/projects/additional/pc-overseer/relays/`. Nothing else in the line or the
  file changed (13 bytes shorter). The file is untracked, so no diff against HEAD shows the edit; this record is
  its account.
- The verbatim copy of the same message, `inputs/I2-relay-1.md.txt`, was not touched: it is a manifest-listed copy
  and keeps the path as given (the hygiene line's `1 host paths kept`).
- The respell was made by hand, not by `gate.py respell`: the row's form was `home`, a path outside the
  repository, which that verb does not rewrite.

## Entry 17 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **exit:** 0 · **atoms:** `exit 0` held · `contains hygiene: clean` held → **green**
- **summary line, as printed:**
  `hygiene: clean — read 41 (runs 34 · evidence 3 · inputs 4) · trails 13 not read · copies 2 not read by P1 — 1 host paths kept · binary 0 not read by P1`
- Read at 2026-10-09T14:36:20Z, after the respell. No row was listed.
- Re-read after this record was first written, before the commit (2026-10-09T14:36:45Z): `hygiene: clean — read 42
  (runs 34 · evidence 4 · inputs 4)`; the scope read: `scope: clean — changed 2 · listed 2 · recorded 0`.
- A third read, after that line was added and directly before `git add -A`: the same summary line, `read 42`.

## The pre-CI commit

- `569604be1f07d594d6350b48555b0f9eced619e1` — `chore(2026-10-09-ci-on-linux-alone): operator pre-CI commit`, on
  `build/andromeda-pulse-0.4.0`, parent `0b61bfb` (the chunk base), made 2026-10-09T14:37:10Z. 52 files, 3779
  insertions, 90 deletions: the two source files (`.github/workflows/ci.yml`,
  `pulse-app/tests/quality_gate_workflow.rs`), the chunk folder, the phase and implement run dirs, and the route,
  ledger, friction-log and handoff files the phase had left modified. The bindings file was not in it (identical to
  HEAD).
- The tree read clean after it (0 status lines).
- The records below this line were written after the push, so the pre-CI commit carries this file only as far as
  the third hygiene read.

## Entry 18 — the clean-tree guard and the push

- **run:** `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0`
- **exit:** 0 → **green** (the entry's default atom)
- **as printed:** `0b61bfb..569604b  build/andromeda-pulse-0.4.0 -> build/andromeda-pulse-0.4.0`
- Read back at 2026-10-09T14:37:17Z: `git ls-remote origin build/andromeda-pulse-0.4.0` returns `569604be…19e1`,
  equal to local HEAD. A fast-forward; no force.

## Entry 19 — the CI read on the pushed HEAD

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
- **exit:** 0 · **atoms:** `exit 0` held · `contains verdict: green` held → **green**
- Fired 2026-10-09T14:37:22Z, returned 2026-10-09T15:06:31Z (polled 57 times over 1748 s). One firing; the run was
  on its first attempt (`run_attempt` 1), never re-run.
- **as printed:**
  `569604be1f07 verdict: green · checks 7/7 · wall 1714 s · runs secret-scan#37945547982 completed/success ci#37945548047 …`
  `runs: secret-scan#37945547982 pull_request completed/success · ci#37945548047 pull_request completed/success`
- The row reads 7 checks; the plan predicted 7 where the base registered 13.
- The `ci` run id the three reads below take as `<id>`: `37945548047`.

## Entry 20 — the after-run's jobs (report-only)

- **run:** `gh api "repos/Turbolet85/andromeds-pulse/actions/runs/37945548047/jobs?per_page=100" --jq '.jobs[] | "\(.name) · \(.conclusion) · \(.started_at) · \(.completed_at)"'`
- **exit:** 0 · `expect = []` → **recorded**, nothing asserted. Read at 2026-10-09T15:06:46Z.
- **as printed:**

      coverage gate (line ≥75% / branch ≥70% / function ≥85%) · success · 2026-10-09T14:37:26Z · 2026-10-09T15:06:00Z
      boot smoke (ubuntu-22.04) · success · 2026-10-09T14:37:26Z · 2026-10-09T14:48:24Z
      mcp-server tests (ubuntu-22.04) · success · 2026-10-09T14:37:26Z · 2026-10-09T14:43:13Z
      lint / test (ubuntu-22.04) · success · 2026-10-09T14:37:26Z · 2026-10-09T14:44:56Z
      supply-chain (audit + deny + auditable) · success · 2026-10-09T14:37:26Z · 2026-10-09T14:44:51Z
      a11y (ubuntu-22.04) · success · 2026-10-09T14:37:26Z · 2026-10-09T14:55:34Z

- Six rows, none named for macOS or Windows, no `release build` row. Their seconds are the After table of
  `evidence/ci-wall-clock.md`.
- **The watch's reading:** `boot smoke (ubuntu-22.04)` success, 658 s, job `113870729265`.

## Entry 21 — the after-run's steps

- **run:** `gh api "repos/Turbolet85/andromeds-pulse/actions/runs/37945548047/jobs?per_page=100" --jq '.jobs[] | .name as $j | .steps[] | select(.conclusion != "success") | "\($j) · \(.name) · \(.conclusion)"'`
- **exit:** 0 · **atoms:** `exit 0` held · `no output` held (0 bytes) → **green**
- No step of any of the six jobs reads other than `success`. Step counts per job, from a second read of the same
  endpoint: `lint / test` 37, `a11y` 21, `mcp-server tests` 18, `boot smoke` 20, `supply-chain` 23, `coverage gate`
  22, equal to the before-run's counts on the same jobs (research.md).

## Entry 22 — the after-run's artifacts

- **run:** `gh api "repos/Turbolet85/andromeds-pulse/actions/runs/37945548047/artifacts?per_page=100" --jq '.artifacts[].name'`
- **exit:** 0 · **atoms:** `exit 0` held · `contains` held for each of `logs-boot-Linux`, `logs-perf-samples-Linux`,
  `a11y-violations-Linux`, `playwright-a11y-report-Linux`, `capability-drift-Linux`, `coverage-linux` · `lacks macOS`
  held · `lacks Windows` held → **green**
- **as printed:**

      logs-boot-Linux
      coverage-linux
      playwright-a11y-report-Linux
      a11y-violations-Linux
      logs-perf-samples-Linux
      capability-drift-Linux

- Six names, where the before-run listed 12.

## Where the pass stops

- Entries 17 to 22 are done: five green, one recorded. The After section of `evidence/ci-wall-clock.md` is filled.
- Beside the entries, one read the plan does not list: the a11y job's per-step instants on both runs, because the job
  read 1088 s against 237 s. The finding is in `evidence/ci-wall-clock.md`; no cause is stated.
- This file's records from "The pre-CI commit" down, and the After section of `evidence/ci-wall-clock.md`, were
  written after the push and are uncommitted. No skill was started after the pass; the wrap waits for the
  operator's word.
