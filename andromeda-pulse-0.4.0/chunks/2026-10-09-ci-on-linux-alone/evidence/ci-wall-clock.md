# CI wall-clock, per job — before and after the Windows and macOS legs leave

Chunk `2026-10-09-ci-on-linux-alone`, plan Step 6. Every figure is read from the runs themselves: each job's
`startedAt` to `completedAt`, in seconds. The before-reading and the eight-run summary were read at take-up (phase,
2026-10-09) and are copied here from `scope.md` and `research.md`; /implement re-read none of them. The after-reading
is the operator pass's.

## Before — `ci#37934330231`

- sha `0b61bfb` (the chunk base) · event `pull_request` · conclusion success · read through `gh run view --json jobs`.

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

- Run wall-clock: 1463 s. Longest job: `coverage gate`, a Linux job that stays.
- Sum over the 12 jobs: 7221 s. The six legs that leave: 3120 s.
- `boot smoke (ubuntu-22.04)`: success, 1106 s (job `113832488437`).

## The eight runs read at take-up

Every commit from the last master flip (`f18c631`) through the chunk base.

| sha | run | event | span | longest job | non-Linux legs | all jobs | boot smoke |
|---|---|---|---|---|---|---|---|
| `0b61bfb` | `ci#37934330231` | pull_request | 1463 | coverage 1463 | 3120 | 7221 | success, 1106 |
| `b3ac58a` | `ci#37924991598` | pull_request | 1754 | coverage 1753 | 3166 | 6961 | failure, 568 |
| `7f99c38` | `ci#37916373451` | pull_request | 1123 | coverage 1123 | 3119 | 6350 | success, 552 |
| `39edd11` | `ci#37914412856` | pull_request | 1091 | coverage 1091 | 3283 | 6574 | success, 654 |
| `60ef43c` | `ci#37907730264` | push | 1923 | lint / test (ubuntu-22.04) 1923 | 8159 | 15140 | success, 1595 |
| `0e45d58` | `ci#37904682919` | pull_request | 1293 | coverage 1293 | 3215 | 6609 | success, 562 |
| `18a872d` | `ci#37731002463` | pull_request | 1763 | coverage 1763 | 3005 | 6888 | success, 490 |
| `f18c631` | `ci#37686027609` | pull_request | 1855 | coverage 1820 | 3368 | 8113 | success, 663 |

- The longest job is a Linux job in 8 of 8, so the removal is not expected to shorten a round by itself.
- Pull-request spans: 1091 to 1855 s across the seven. The longest non-Linux job on them: 626 to 812 s.
- What the removal takes away is runner time: 3005 to 3368 s of 6350 to 8113 s on the pull-request runs, 8159 of
  15140 s on the push run.

## `boot smoke (ubuntu-22.04)` — the verdict of every run the chunk read

| sha | run | verdict | s |
|---|---|---|---|
| `0b61bfb` | `ci#37934330231` | success | 1106 |
| `b3ac58a` | `ci#37924991598` | failure (job `113801670494`) | 568 |
| `7f99c38` | `ci#37916373451` | success | 552 |
| `39edd11` | `ci#37914412856` | success | 654 |
| `60ef43c` | `ci#37907730264` | success | 1595 |
| `0e45d58` | `ci#37904682919` | success | 562 |
| `18a872d` | `ci#37731002463` | success | 490 |
| `f18c631` | `ci#37686027609` | success | 663 |

The watch's readings on equal source (no source file differs between the three commits): `7f99c38` green · `b3ac58a`
red · `0b61bfb` green. The after-run below is the next reading.

## After — the first run of the pre-CI commit

`ci#37945548047`, read in the operator pass at 2026-10-09T15:06:46Z from the jobs entry's output
(`evidence/operator-pass.md`, entry 20).

- sha `569604b` (the pre-CI commit) · event `pull_request` · attempt 1, the first run of that commit · conclusion
  success.

| job | s | the same job on the before-run | job id |
|---|---|---|---|
| coverage gate | 1714 | 1463 | `113870729157` |
| a11y (ubuntu-22.04) | 1088 | 237 | `113870729650` |
| boot smoke (ubuntu-22.04) | 658 | 1106 | `113870729265` |
| lint / test (ubuntu-22.04) | 450 | 426 | `113870729376` |
| supply-chain | 445 | 465 | `113870729512` |
| mcp-server tests (ubuntu-22.04) | 347 | 404 | `113870729321` |

- Run wall-clock: 1714 s (`ci.py`: `wall 1714 s`). Longest job: `coverage gate`.
- Sum over the 6 jobs: 4702 s. The before-run's 12 jobs summed 7221 s; its six Linux jobs alone, 4101 s.
- Checks on the commit: 7 (`checks 7/7`: the six `ci` jobs and `secret-scan#37945547982`).
- `boot smoke (ubuntu-22.04)`: **success**, 658 s, job `113870729265`.
- Step counts per job equal the before-run's: 37, 21, 18, 20, 23, 22.

The forecasts, each as read:

- 7 checks where the base registered 13: read 7.
- A span inside 1091 to 1855 s with `coverage gate` still the longest job: read 1714 s, `coverage gate` the longest.
  The round is 251 s longer than the before-run's 1463 s, inside the spread of the seven pull-request runs.
- The sum of job-seconds down by about the six legs' share, 3120 s: read down 2519 s (7221 to 4702). The six legs'
  3120 s left; the six jobs that stay summed 601 s more than on the before-run.

One job moved far outside its before-reading: `a11y (ubuntu-22.04)`, 237 s to 1088 s. Its per-step instants
(`gh api …/actions/jobs/113870729650`, read 2026-10-09T15:06Z) place the difference in two steps:
`Install Linux system libraries (Tauri + dbus)` took 666 s where the before-run's took 28 s, and
`Install Playwright chromium (a11y harness)` took 266 s where the before-run's took 76 s. Both steps download
packages; this chunk changed neither step's commands (the first lost its always-true condition). The step logs were
not read, so no cause is stated. One run is one reading: whether the a11y job stays slow is not measured here.

With the after-run, the `boot smoke (ubuntu-22.04)` verdicts the chunk read are nine: the eight above and `569604b`
success. On the watch's equal-source series the after-run is not a like-for-like reading in the strict sense: its
commit differs from `0b61bfb` in `ci.yml` and one test file. The boot job's own block of `ci.yml` is unchanged and
a release build compiles no test file; how the wrap counts this reading is the wrap's.
