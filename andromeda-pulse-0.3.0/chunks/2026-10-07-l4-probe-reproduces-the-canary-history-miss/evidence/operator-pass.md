# The operator pass (plan Step 17) — entries 29, 30, 31

Run on the operator's explicit word, given in the session that swapped the product change to `CX` (2026-10-07,
after that run's report: "Operator pass: go, on my explicit word. Run entry 29 (hygiene), entry 30 (the pre-CI
commit, then the push) and entry 31 (the CI read), then stop before the wrap."). Each entry was driven once by hand;
its `run` is written in the spelling the gate tool prints (the home as `~`).

The tree it ran on: the chunk base `48714f0` plus the chunk's edits, `CX` in the product
(`evidence/remedy-shipped.md`). The gate block before it: 19 green, 0 red, 12 operator legs not run, and the eight
tree-reading entries green again once the last evidence file was written (2026-10-07T18:36:57Z).

## The capture-text read, once more before the commit

- Fired by hand at 2026-10-07T18:38:16Z, the run `evidence/capture-text-read.md` records (three captured prompts, 9
  distinct corpus lines): 0 files, exit 1, both atoms held → **green**. No commit carries a captured line.

## Entry 29 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **exit:** 0 · **atoms:** `exit 0` held · `contains hygiene: clean` held → **green**
- **summary line, as printed:**
  `hygiene: clean — read 98 (runs 61 · evidence 15 · inputs 22) · trails 30 not read · copies 20 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- Read at 2026-10-07T18:38:16Z. No committed capture needed a placeholder: no row was listed.
- Re-read after this record was first written, before the commit (2026-10-07T18:38:31Z): `hygiene: clean — read 99
  (… evidence 16 …)`; the scope read: `scope: clean — changed 4 · listed 4 · recorded 0`.

## The pre-CI commit

- `9bfefb812297bbdea610423a21568b3262bdb7ed` —
  `chore(2026-10-07-l4-probe-reproduces-the-canary-history-miss): operator pre-CI commit`, on
  `chore/migrate-pulse-to-v3`, parent `48714f0` (the chunk base), made 2026-10-07T18:38:39Z. 110 files: the four
  source files, the chunk folder, the two phase and five implement run dirs, and the route, ledger and handoff files
  earlier steps had left modified. The bindings file was not in it (identical to HEAD).
- The tree read clean after it (0 status lines).

## Entry 30 — the clean-tree guard and the push

- **run:** `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`
- **exit:** 0 → **green** (the entry's default atom)
- **as printed:** `48714f0..9bfefb8  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`
- Read back at 2026-10-07T18:38:46Z: `git ls-remote origin chore/migrate-pulse-to-v3` returns `9bfefb81…b7ed`, equal
  to local HEAD. A fast-forward; no force.

## Entry 31 — the CI read on the pushed HEAD

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
- **Verdict of record: green, on the run's second attempt.** Two firings, one per attempt of the same run,
  `ci#37668429742`; `secret-scan#37668429734` was green throughout.

### First firing: red (attempt 1)

- Fired 2026-10-07T18:38:50Z, returned 19:04:46Z (polled 51 times over 1556 s). **exit:** 0 · `contains verdict:
  green` did not hold → **red**.
- **as printed:**
  `9bfefb812297 verdict: red · checks 13/13 · first-fail +1543 s boot smoke (ubuntu-22.04) · runs ci#37668429742 in_progre…`
  `failed 1: boot smoke (ubuntu-22.04) (failure)` · `running 1: oldest coverage gate (line ≥75% / branch ≥70% / function ≥85%) 1548 s`
- **The boot-smoke death, job `112953603888`** (ended 19:04:40Z). Read from the job's log and its `logs-boot-Linux`
  artifact (`11504892957`):
  - The wrapper printed `boot: failed to reach ready state within 10s`, then `app ended: exit 1`, then
    `cleanup: clean`. There is no `boot: ready` line in this log.
  - The app's own log holds 41 records, the first at 19:04:24.407Z and the last at 19:04:24.612Z: tracing up, both
    OTLP receivers bound, the first heartbeat ticks. No `app.exit` record, no `app.panic.fatal` record. One ERROR,
    `corpus.open.error` with `error_kind: KeyringUnavailable`, which the runner has no credential store for.
  - The app's stderr (`boot.log`) holds one AT-SPI warning about the accessibility bus and nothing else.
  - `digest.runtime.boot` recorded the digest assembler disabled in that process (no corpus). The corpus selection
    is the one product code this chunk changes, and it runs only inside the assembler, so it was not reachable in
    the process that died.
- **How this stands against the route entry "The Linux boot smoke is deterministic".** That entry records the app
  ending with exit 1 within 0.7 s after `boot: ready`, twice (`ci#37293411947`, `ci#37585281667`). Here the app
  ended with exit 1 before the wrapper printed `boot: ready`. It is the same job, the same end and the same
  missing record; the order against the ready line differs. Whether it is the same death is not established: the
  cause of none of the three is known. The operator's word before the pass named the death "after boot: ready";
  told of this difference, the operator counted the re-run below as the one allowed for the boot smoke.

### The cancel (the operator's act, not this run's)

- The operator cancelled the run and re-ran its unfinished jobs, at 21:57 local, and told this session so: "run
  37668429742 was cancelled and its unfinished jobs re-run (gh run rerun --failed). Reason, measured: the coverage
  job sat 78 minutes in its step 'Install Linux system libraries (Tauri + dbus)' on the hosted runner (usual
  whole-job time 24 minutes), so the run could never conclude and the boot smoke could not be re-run. The re-run
  also is the one boot-smoke re-run I allowed."
- Re-read from the API before it was relied on: attempt 1 is `completed / cancelled`, updated 19:58:14Z; its coverage
  job `112953603875` shows step 9, "Install Linux system libraries (Tauri + dbus)", from 18:39:41Z to 19:58:07Z
  (78 min 26 s), `cancelled`, and every later step `skipped`. So attempt 1 never ran the coverage measure.
- This run had not re-run anything: a job of a run can only be re-run once the run has concluded, and it had not.

### Second firing: green (attempt 2)

- Fired 2026-10-07T19:58:53Z, returned 20:27:23Z (polled 56 times over 1710 s). **exit:** 0 · **atoms:** `exit 0`
  held · `contains verdict: green` held → **green**.
- **as printed:**
  `9bfefb812297 verdict: green · checks 13/13 · wall 6498 s · runs secret-scan#37668429734 completed/success ci#3766842974…`
  `runs: secret-scan#37668429734 pull_request completed/success · ci#37668429742 pull_request completed/success`
- **Run ids read:** `ci#37668429742`, attempt 2, and `secret-scan#37668429734`, both for
  `9bfefb812297bbdea610423a21568b3262bdb7ed`.
- Attempt 2 ran two jobs anew, both started 19:58:25Z: `boot smoke (ubuntu-22.04)`, job `112988118024`, success at
  20:05:44Z (`boot: ready`, then `"ended": null` and `"verdict": "running-healthy"`, then `cleanup: clean`); and the
  coverage gate, job `112988118325`, success at 20:27:15Z. The other ten jobs carry their attempt-1 success.
- One re-run of the boot smoke, the one the operator allowed. It was not fired a third time.

## Verdict of the pass

Entries 29 and 30 green on their first firing. Entry 31 red on the run's first attempt (the boot smoke, job
`112953603888`) and green on its second, the same sha. The sha Conductor waits on is `9bfefb8`, on a green CI.
The first attempt's red is not read here as this chunk's, on two grounds kept apart: the operator's allowance of
one boot-smoke re-run, applied to it after the difference from the known death was reported; and the measurement
that the changed code was not reachable in the process that died. It was not reproduced on a tree without this
chunk's edits, so the wrap's basis for it is those two and no two-sided probe. The chunk's own commit and the
master flip are the wrap's; this pass made neither.

Process census, read at 2026-10-07T20:27:38Z: no `gh run watch`, `ci.py`, cargo, probe or `llama` process in the
host's process list. The one `gh run watch` this pass started (to wait for attempt 1 to conclude) was stopped by
pid when the operator's re-run made it moot.

Recorded 2026-10-07T20:27:38Z.
