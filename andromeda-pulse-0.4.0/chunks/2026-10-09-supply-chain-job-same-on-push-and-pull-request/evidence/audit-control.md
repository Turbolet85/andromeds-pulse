# The red control — `cargo audit` turns red on a finding

**Where it ran: on this development host, not on a CI runner.** The operator accepted a local control (inputs#I3,
point 3). What a runner adds is recorded at the foot of this file by the operator pass.

Read at /implement, 2026-10-09T16:38Z.

## The tool

- `cargo audit --version`: `cargo-audit-audit 0.22.2`. The `ubuntu-22.04` runner image lists "Cargo audit 0.22.2"
  (research.md, image `20261004.315.1`).
- The advisory database is the one `cargo audit` fetches itself at each run (`Fetching advisory database from
  https://github.com/RustSec/advisory-db.git`), kept under `~/.cargo/advisory-db` (`CARGO_HOME` is unset on this host).
  It loaded 1296 advisories; the checkout's head after the fetch was `7eebec69c352`, committed 2026-10-09T10:12:02+02:00.

## Three exits, read with the same binary

| Command | Exit | What it printed |
|---|---|---|
| `cargo audit` (the project `Cargo.lock`) | 0 | `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)` · `warning: 10 allowed warnings found` |
| `cargo audit --file …/evidence/audit-control.lock` | 1 | `Scanning …/evidence/audit-control.lock for vulnerabilities (2 crate dependencies)` · `ID:        RUSTSEC-2020-0071` · `error: 1 vulnerability found!` |
| `cargo audit --file …/evidence/no-such-file.lock` (no such file) | 2 | 7 lines of error text, no scan line |

- The control's finding, as printed: crate `time`, version `0.1.43`, "Potential segfault in the time crate",
  2020-11-18, `RUSTSEC-2020-0071`, severity 6.2 (medium), solution "Upgrade to >=0.2.23". One distinct `RUSTSEC-` id.
- The project lockfile's 10 informational ids, distinct: RUSTSEC-2024-0370, -2024-0429, -2024-0436, -2025-0075,
  -2025-0080, -2025-0081, -2025-0098, -2025-0100, -2025-0141, -2026-0221. The set is identical to research.md's
  reading at P3.
- Exit 2 is "could not evaluate". It is not a finding and not a pass; a `run:` step fails on it.

## The control file

`audit-control.lock` is a version-3 lockfile of two packages: a root package `control` depending on `time`, and
`time 0.1.43` from the crates.io registry. It is not named `Cargo.lock`, nothing builds from it, and it is read by
one gate entry.

## What this does not show

- That the runner's `cargo audit` behaves the same. The local binary and the runner image's are the same version;
  the runner's own behaviour on a finding was not exercised, and no advisory was planted in the project lockfile to
  exercise it.
- That the repaired step is green on a push. That is the push run on `main` after the founder's merge.

## From the operator pass

Read 2026-10-09T17:43:46Z from the `supply-chain` job log of the push run on `main` (`ci#37964887106` on `178ebac`,
job `113936654152`):

- `Image: ubuntu-22.04`, `Version: 20261004.315.1`, `Included Software:
  https://github.com/actions/runner-images/blob/ubuntu22/20261004.315/images/ubuntu/Ubuntu2204-Readme.md`. The same
  image version research.md read at P3, the one whose list names "Cargo audit 0.22.2".
- On that runner the repaired step printed `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)` and
  `warning: 10 allowed warnings found`, and ended `success` in 5 s: the same reading as the local exit-0 row above.
- These lines were read with `gh api --allow-escape-sequences …/logs`. The plan's report-only image entry (42)
  printed nothing as written; `operator-pass.md` has the account.
- Still not shown: the runner's `cargo audit` on a finding. The red control above stays local.
