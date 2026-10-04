# Advisory record — 2026-10-04-supply-chain-advisories-on-wasmtime-resolved

## The database read
- Copy: `$HOME/.cargo/advisory-db` (`CARGO_HOME` is unset on this host, so this is the copy `cargo audit` reads).
- At the after-reading (2026-10-04, this chunk's implement gate run): 1290 security advisories, head
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (2026-10-03T10:14:03+02:00). Worktree status clean; head equal to
  upstream `RustSec/advisory-db` HEAD read by `ls-remote` at the same time. The three advisory files
  `crates/wasmtime/RUSTSEC-2026-0325.md` / `-0326.md` / `-0327.md` are present (the plan's currency-control entry,
  green), so an absent finding is not a DB-age artifact.
- The before-reading (research.md §Measured) and the failing CI job on `5988a5f` (job 111328669434) used the same DB:
  1290 advisories at `ef6173cb`.

## `cargo audit` (the plain pass/fail gate)
| | before (chunk base `5988a5f`, wasmtime 48.0.3) | after (this chunk, wasmtime 48.0.5) |
|---|---|---|
| exit | 1 | 0 |
| vulnerabilities (distinct ids) | RUSTSEC-2026-0325 · RUSTSEC-2026-0326 · RUSTSEC-2026-0327 (all `wasmtime` 48.0.3) | none |
| warnings, unmaintained (distinct ids) | RUSTSEC-2024-0370 · -2024-0436 · -2025-0075 · -2025-0080 · -2025-0081 · -2025-0098 · -2025-0100 · -2025-0141 | set identical to the prior enumeration |
| warnings, unsound (distinct ids) | RUSTSEC-2024-0429 · RUSTSEC-2026-0221 | set identical to the prior enumeration |

After: `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)`, then `warning: 10 allowed warnings
found`, with no vulnerability block. None of the three wasmtime ids appears in the output (0 matching lines).

## `cargo deny check advisories` (observed separately, never folded into the pass/fail invocation)
| | before | after |
|---|---|---|
| exit | 1 | 0 |
| errors (distinct ids) | `error[vulnerability]` RUSTSEC-2026-0325 · RUSTSEC-2026-0326 · RUSTSEC-2026-0327 | none (`advisories ok`) |
| warnings (distinct ids) | `warning[advisory-not-detected]` RUSTSEC-2024-0411 · -0412 · -0413 · -0414 · -0415 · -0416 · -0417 · -0418 · -0419 · -0420 | set identical to the prior enumeration |

The ten `advisory-not-detected` warnings are stale `deny.toml [advisories] ignore` entries (the GTK3 0.18.2 family)
that predate this chunk. They are outside its subject, and `deny.toml` is byte-identical to the chunk base (the plan's
scope guard, green).

## `cargo deny check bans licenses sources` (pass/fail)
Exit 0, `bans ok, licenses ok, sources ok`, before and after. The after-reading prints nine `warning[wildcard]` rows
(workspace path deps of `pulse-app`, `config-watcher`, `corpus`, `snapshot`, `buffer`, `interpretation`, `xtask`,
`mcp-server`, `ui-bridge`) and no `unnecessary-skip`, matching research's before-reading.

## No ignore
- `deny.toml` holds none of the three ids (`grep -cE 'RUSTSEC-2026-032[567]' deny.toml` -> 0).
- No `audit.toml` / `.cargo/audit.toml` exists.
- The CI step `rustsec/audit-check` passes no ignore for them (`.github/workflows/ci.yml` unchanged since the chunk base).
