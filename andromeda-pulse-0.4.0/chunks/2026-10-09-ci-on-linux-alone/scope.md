# Scope — CI on Linux alone

**Marker:** `2026-10-09-ci-on-linux-alone` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:11`):** "CI on Linux alone — Windows and macOS legs leave three jobs, the
release-build job whole; one run's wall-clock recorded (P-113)"
**Chunk base:** `0b61bfb` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD: the
operator pre-CI commit moves HEAD before the wrap re-runs them (inputs#I1 item 6).

## Intent
0.4.0 keeps one operating system. Intent R7 expects that "CI has no leg for" another one
(`andromeda-pulse-0.4.0/intent.md:183-190`), and requirement P-113 repeats it (`requirements.md:31`). This is the
first chunk of 0.4.0 and the first of four entries that carry P-113: it takes the Windows and macOS runners out of
the `ci` workflow and records what one run costs, per job, before and after. The founder's word to start code was
given today (inputs#I1 item 1, inputs#I2).

## What the chunk builds
- **The Windows and macOS legs leave `.github/workflows/ci.yml`.** Read at take-up, at `0b61bfb`:
  - `lint-test` runs a three-system matrix, `[ubuntu-22.04, macos-latest, windows-latest]` (`ci.yml:29`). It keeps
    the Linux leg.
  - `a11y` runs the same three-system matrix (`ci.yml:307`). It keeps the Linux leg.
  - `release` runs `[macos-latest, windows-latest]` only (`ci.yml:199`). It has no Linux leg, so it leaves whole
    (`ci.yml:191-242`). These are the entry's "three jobs".
  - The other four jobs already run on `ubuntu-22.04` alone and stay as they are: `mcp-test` (`ci.yml:246`), `boot`
    (`ci.yml:394`), `supply-chain` (`ci.yml:469`), `coverage` (`ci.yml:546`).
  - After the chunk, one run registers 6 checks in `ci` where it registered 12 (two `lint / test`, two `a11y` and two
    `release build` checks leave), plus the one `secret-scan` check: 7 where Setup read 13.
- **What exists in `ci.yml` only because another system ran there leaves with the legs.** Verified at P3
  (`grep -c -i -E 'macos|windows|matrix\.os|runner\.os ==' .github/workflows/ci.yml` reads 20 lines at `0b61bfb`):
  - the step conditions `if: runner.os == 'Linux'` (`ci.yml:60`, `:122`, `:128`, `:132`, `:334`, `:514`, `:579`),
    which are always true once every job runs on Linux. They sit on the system-library install step of four jobs
    (`lint-test`, `a11y`, `supply-chain`, `coverage`) and on the three perf steps of `lint-test`; each step stays
    and becomes unconditional;
  - the comments that describe the other systems' runners: the cache budget's "one per release OS (macOS, Windows)"
    (`ci.yml:18-21`), the note above `release` (`ci.yml:191-192`), the Windows code-page clause (`ci.yml:86-88`), the
    "(Linux only)" step-name suffixes (`ci.yml:121`, `:127`, `:131`);
  - the three matrix blocks with their `fail-fast: false` (`ci.yml:26-29`, `:196-199`, `:304-307`).
  - [premise-corrected: `shell: bash` on the data-dir export step stays. No record says it was written for the
    Windows runner: the plan that added it gives no reason
    (`andromeda-pulse-0.3.0/chunks/2026-09-29-p-025-hue-shift-observable-made-gradable/plan.md:44-45`), and the same
    line stands in every job of all three workflow files, the Linux-only ones included.] On a Linux runner the key
    changes the shell's flags only, and the step holds no pipe.
- **What a Linux run resolves today resolves the same after the chunk.** Verified at P3: the cache keys
  (`lint-test-Linux`, `boot-Linux`, `coverage`) each hold an entry on `refs/heads/main` (the repository's cache
  listing, read 2026-10-09), so the first run after the change restores a warm cache and the after-reading is
  comparable with the before-reading; and the artifact names (`logs-Linux`, `logs-boot-Linux`, `nextest-Linux`,
  `criterion-Linux`, `capability-drift-Linux`, `a11y-violations-Linux`, `playwright-a11y-report-Linux`,
  `logs-perf-samples-Linux`, `coverage-linux`). The names keep their `${{ runner.os }}` template: four assertions of
  `pulse-app/tests/quality_gate_workflow.rs` pin the template's own text (`:290`, `:308`, `:319`, `:327`). The
  handoff's host note and this entry's `WATCH:` read `logs-boot-Linux` by name.
- **The Linux release build keeps a CI witness after `release` leaves.** Verified at P3: `supply-chain` builds
  `cargo auditable build --workspace --release` (`ci.yml:540-542`, its comment "Also the Linux workspace release
  build smoke"), and `boot` builds `cargo build --workspace --release --features mcp-server` (`ci.yml:439-440`). The
  `supply-chain` build runs after the audit, deny and npm steps, so a red at one of them leaves it unmeasured on that
  commit; the witness is the step's own `success`.
- **One run's wall-clock is recorded, per job, before and after, from the runs themselves** (the entry; inputs#I1
  item 5, which quotes the founder's standing direction of 2026-09-29, re-read in this repository at
  `andromeda-pulse-0.3.0/chunks/2026-09-29-ci-wall-time-and-round-trips/scope.md:7`).
  - The before-reading is `ci#37934330231` on `0b61bfb` (pull-request event, green, read at take-up through
    `gh run view --json jobs`, each job's `startedAt` to `completedAt`):

    | job | s |
    |---|---|
    | coverage gate | 1463 |
    | boot smoke (ubuntu-22.04) | 1106 |
    | lint / test (macos-latest) | 804 |
    | lint / test (windows-latest) | 660 |
    | release build (windows-latest) | 654 |
    | supply-chain | 465 |
    | a11y (windows-latest) | 455 |
    | lint / test (ubuntu-22.04) | 426 |
    | mcp-server tests (ubuntu-22.04) | 404 |
    | release build (macos-latest) | 346 |
    | a11y (ubuntu-22.04) | 237 |
    | a11y (macos-latest) | 201 |

    The run's wall-clock is 1463 s and its longest job is `coverage`, a Linux job. Sum over the 12 jobs: 7221 s, of
    which the six legs that leave are 3120 s.
  - The after-reading is the run on this chunk's pre-CI commit, read the same way.
  - The removal shortens no round by itself. Verified at P3 over the eight runs Setup read (each run's jobs through
    `gh run view --json jobs`): the longest job is a Linux job in 8 of 8, `coverage` in the seven pull-request runs
    and `lint / test (ubuntu-22.04)` on the push run. What the removal takes away is runner time, 3005 to 3368 s of
    6350 to 8113 s on the pull-request runs and 8159 of 15140 s on the push run, and the legs' own failure surface.
    The after-reading is expected to show the same wall-clock within run-to-run spread (1091 to 1855 s across the
    seven pull-request runs); a shorter round is a different chunk's work.
- **The records that describe a three-system CI are corrected at the wrap, not here.** Phase amends no spec source.
  Verified at P3: the extracts name the sections, and the plan carries them as expected amendments.
- **What reads `ci.yml` in the tree is kept true.** Verified at P3 (`grep -rn 'ci\.yml'` over the Rust, script and
  workflow files outside the planning folders): two test files and one xtask module.
  - `pulse-app/tests/quality_gate_workflow.rs` holds two tests the change turns red:
    `ci_workflow_release_job_owns_and_saves_its_cache_key` (`:101-112`, its subject is the `release` job) and
    `data_dir_export_precedes_every_consumer` (`:413`, which expects 7 jobs).
  - `pulse-app/tests/a11y_perf_workflow.rs` and `xtask/src/pre_push.rs` read nothing the change removes: the first
    reads step commands and the `a11y` job block, the second the `node-version` major and the first
    `apt-get install` list (`pre_push.rs:527-566`), both unchanged.
- Added at P3: **one self-lint pin that the workflow names no other system**, in the test file that already pins the
  workflow, written before the edit and read red against the workflow as it stands.
- Amended at P5 (validation-1, intent-incomplete): **a second pin, that the Linux release build keeps both
  witnesses** (the `supply-chain` auditable build and the `boot` release build). With `release` gone they are the
  only release-profile builds the workflow runs, three extracts require them to stay, and the helper the pin reads
  through would otherwise lose its one caller with the test that leaves. It passes before and after the edit, so
  its control is a one-shot mutation at implement.
- Read at P3, and not this chunk's to change:
  - 61 conditional-compilation lines in 24 Rust files name Windows or macOS
    (`grep -rn -E 'cfg(_attr)?!?\(.*(windows|macos)'` over `crates`, `pulse-app` and `xtask`). After the chunk no CI
    job compiles their other-system arms. They leave at `Other operating systems retired from the code`
    (`working-route.md:52`) and `Window retired` (`:40`).
  - The corpus credential-store legs clean-skip where no store answers, and a skip is invisible in nextest's counts
    (test-plan amendment `2026-10-04-corpus-key-creation-is-race-free`). Whether they ran on the macOS and Windows
    runners and skip on `ubuntu-22.04` is not measured, so a coverage loss there is neither shown nor excluded.
    Their subject leaves at `Corpus encryption at rest retired` (`working-route.md:31`).
  - The repository cache holds 10 entries, 12,208,662,121 B against a 10 GB cap. Six are the leaving legs' (four on
    `refs/heads/main`, two on `refs/pull/39/merge`), 8,531,028,085 B. Nothing in the tree deletes them; they age
    out, or the operator deletes them.
  - The three pull-request-only baseline downloads (`criterion-Linux-base`, `a11y-violations-base`,
    `coverage-linux-base`; `ci.yml:171-177`, `:360-366`, `:631-637`) have no producer in any workflow
    (`grep -rn -e '-base' .github/workflows/`: the three downloads and their two consumers, no upload), and each is
    `continue-on-error`. The legs' removal changes nothing about that.

## Capability
- **P-113 is advanced, not claimed.** Its acceptance reads "The tree builds, tests and runs on Linux alone; it holds
  no branch, script, link hint or workaround for another operating system; every CI job runs on Linux only and is
  green." Three later entries carry the rest of it: `Pre-push check native on Linux` (`working-route.md:15`),
  `One place on a node` (`:29`) and `Other operating systems retired from the code` (`:52`). This chunk makes the CI
  clause true and leaves the cap pooled.

## Boundaries
- **The supply-chain repair is the next entry's** (`working-route.md:13`, P-120). The same workflow file is open, and
  the `cargo audit` step and its token are not touched here (inputs#I1 item 4).
- **`release.yml` and `update-channels.yml` are not this chunk's.** They leave at `Desktop distribution retired`
  (`working-route.md:42`, P-114, P-127), whose card names the release environment and its secret names for the
  founder. This chunk removes the `release` JOB of `ci.yml`, a build smoke with no secret and no environment
  (`ci.yml:193-242`), and no workflow file. The relay marks the overlap as a hypothesis (inputs#I1 item 4); read at
  take-up, the entry at `:42` lists "bundle workflow, channel publishing, runbooks, manifest test, npm tree, its
  watch, gate", and the `ci.yml` job is none of them.
- **`boot` and the Linux `a11y` leg stay.** They leave at `Window's gates retired` (`working-route.md:38`, P-083).
- **No source change for another system.** The `.ps1` scripts, the `pwsh` spawns, the `wsl.exe` hop, the link hints,
  the linker override and the data-directory names stay where they are; they are the three later P-113 entries'.
- **A boundary widening halts for the founder's own word** (inputs#I1 item 7).
- No run binds 4317 or 4318 without the operator's word (the handoff's host note); the chunk's own edits need none.

## Folded freight (the entry's one block)
- watch: boot smoke on ubuntu-22.04 ended exit 1 within 0.4 s of its last record, after ready, with no process-end
  record; no cause in the application log, whose warnings match the green run on 7f99c38 — run 37924991598 on
  b3ac58a; hypothesis: the window's boot; the job leaves at Window's gates retired (0/3; since
  2026-10-09-0-pending-adaptation-wrap). An observation: no acceptance criterion, no plan task, no gate entry. This
  chunk's wrap is the first to read it, and the `boot smoke (ubuntu-22.04)` verdict is recorded for every CI run the
  chunk reads (inputs#I1 item 3).
  - Readings so far on equal source (no source file differs between the three commits; re-read at take-up): `7f99c38`
    green · `b3ac58a` red · `0b61bfb` green (job `113832488437`, in `ci#37934330231`).

## Second fold source — the CI verdict read at Setup 5a
Every commit from the last master flip (`f18c631`) through HEAD, read through `ci.py conclusion`, 13 checks each.
- `0b61bfbef575` (HEAD): green · wall 1463 s (`ci#37934330231`, `secret-scan#37934330233`).
- `b3ac58a9ef5f`: **red** · `boot smoke (ubuntu-22.04)` alone failed, first fail at +568 s · wall 1754 s
  (`ci#37924991598`, `secret-scan#37924991604`). Re-read at take-up: job `113801670494`, the other 11 jobs success.
- `7f99c3800973`: green · wall 1123 s (`ci#37916373451`, `secret-scan#37916373450`).
- `39edd11529b5`: green · wall 1091 s (`ci#37914412856`, `secret-scan#37914412865`).
- `60ef43c6859c` (the push to `main` that merged PR #39): **red** · `supply-chain (audit + deny + auditable)` alone
  failed, first fail at +47 s · wall 1923 s (`ci#37907730264`, `secret-scan#37907730180`). Re-read at take-up: job
  `113745162405`, its log's one error line `Resource not accessible by integration` on the check-run create call
  (`:660`), the other 11 jobs success.
- `0e45d5883b64`: green · wall 1293 s (`ci#37904682919`, `secret-scan#37904682926`).
- `18a872d0e449`: green · wall 1763 s (`ci#37731002463`, `secret-scan#37731002566`).
- `f18c631bd545` (the last flip): green · wall 1855 s (`ci#37686027609`, with its secret-scan run).

**Disposition of the two reds, on the operator's word (inputs#I1 item 2, inputs#I2).** Neither failing subject is
something this chunk builds, and neither is folded in as work:
- the `60ef43c` red is owned by the route's second entry, `Supply-chain job same on push and pull request`
  (`working-route.md:13`, P-120). It is a push-event defect; this chunk's runs are pull-request runs and witness
  nothing about it.
- the `b3ac58a` red is owned by the `WATCH:` on this entry, folded above as an observation.

## Gate
- none — the entry carries no `BLOCKED-ON`.
