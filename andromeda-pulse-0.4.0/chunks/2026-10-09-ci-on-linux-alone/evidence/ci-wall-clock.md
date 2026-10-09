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

_Not yet read. The operator pass fills this section from the jobs entry's recorded output (plan Step 7): the run id,
the sha, the event, the per-job table, the run's wall-clock and its longest job, the sum over the jobs, the number of
checks the commit registered, and the `boot smoke (ubuntu-22.04)` verdict with its job id. It is the FIRST run of the
pre-CI commit, never a second attempt._

Forecasts to hold the reading against, each to be recorded as read and not restated: 7 checks where the base
registered 13; a span inside 1091 to 1855 s with `coverage gate` still the longest job; the sum of job-seconds down by
about the six legs' share, 3120 s of 7221 s on the before-reading.
