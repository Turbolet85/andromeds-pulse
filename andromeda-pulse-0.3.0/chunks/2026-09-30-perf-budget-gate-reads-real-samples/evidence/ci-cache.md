# CI cache record — operator entries #29–#32 on `c6eb395`

Run `ci#36729367693` (secret-scan `#36729367831` green). Round 1 = attempt 1; round 2 = attempt 2 (`gh run rerun`,
same sha). Read from `gh run view --json jobs` and each job's log.

## Verdicts

- #29 round 1: `c6eb395f27e3 verdict: green · checks 13/13 · wall 2149 s`.
- #30 `gh run rerun 36729367693`: exit 0.
- #31 round 2: `verdict: green · checks 13/13` (the tool's `wall 3939 s` spans both attempts). Attempt 2 by job times:
  15:05:48Z → 15:34:06Z = **28 m 18 s**, critical path the coverage gate (28 m 17 s).

## #32 `gh api repos/Turbolet85/andromeds-pulse/actions/cache/usage` (after round 2)

`{"active_caches_size_in_bytes":10605172169,"active_caches_count":8}` — **10 605 172 169 B ≤ 10 737 418 240 B** cap,
under by 132 246 071 B (≈ 1.2 % headroom).

| key | size (B) |
|---|---|
| v0-rust-lint-test-Windows-Windows_NT-x64-15d035cd-769a9503 | 2 675 933 647 |
| v0-rust-lint-test-macOS-Darwin-arm64-9bc487fc-769a9503 | 2 167 617 877 |
| v0-rust-lint-test-Linux-Linux-x64-fb86c71b-769a9503 | 1 801 716 897 |
| v0-rust-boot-Linux-Linux-x64-fb86c71b-769a9503 | 1 726 882 083 |
| v0-rust-release-Windows-Windows_NT-x64-15d035cd-769a9503 | 1 117 671 122 |
| v0-rust-release-macOS-Darwin-arm64-9bc487fc-769a9503 | 911 478 170 |
| v0-rust-coverage-Linux-x64-fb86c71b-769a9503 | 198 154 879 |
| gitleaks-cache-8.24.3-linux-x64 | 5 717 494 |

All on `refs/pull/39/merge`. Per the plan's cache caveat, the lint-test Windows/macOS keys still carry release
dependencies until their next `Cargo.lock`-driven re-save; the headroom above is the reading as it stands.

## Round 2 — every rust-cache step, and the release compile counts

| job | restored key | full match | `Compiling` lines |
|---|---|---|---|
| release build (windows-latest) | release-Windows-…-15d035cd-769a9503 | true | 16 |
| release build (macos-latest) | release-macOS-…-9bc487fc-769a9503 | true | 16 |
| lint / test (windows-latest) | lint-test-Windows-… | true | 32 |
| lint / test (macos-latest) | lint-test-macOS-… | true | 32 |
| lint / test (ubuntu-22.04) | lint-test-Linux-… | true | 23 |
| boot smoke (ubuntu-22.04) | boot-Linux-… | true | 16 |
| supply-chain | boot-Linux-… | true | 16 |
| mcp-server tests | lint-test-Linux-… | true | 16 |
| a11y (ubuntu / windows / macos) | lint-test-{os}-… | true ×3 | 0 ×3 |
| coverage gate | coverage-Linux-… | true | 716 (registry-only key, by design) |

Each release job compiled exactly the 16 workspace crates.

## Release job wall-clock (measured, per the plan's forecast note)

| job | round 1 (cold key) | round 2 (warm) |
|---|---|---|
| release build (windows-latest) | 35 m 48 s — `No cache found`, 606 compiling, key saved | 11 m 45 s |
| release build (macos-latest) | 8 m 58 s — restored the key saved at `ci#36723465727` | 5 m 39 s |

The Windows frame boot step is gone (the phase fallback), so no leg time rides the release job.

## Round 1 — the perf-budget lines on CI

lint / test (ubuntu-22.04), `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot`:

```
perf-budget: graded 120 record(s) across 1 file(s)
perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run
perf-budget: memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS
perf-budget: snapshot p99 0.0 ms <= 500 ms (n=50) PASS
```

boot smoke (ubuntu-22.04), `cargo xtask ci-gates`:

```
ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run
ci-gates: perf-budget: memory NEUTRAL — populated 0 of 1
ci-gates: perf-budget: snapshot NEUTRAL — no metric.snapshot.token_count_ms record
ci-gates: perf-budget NEUTRAL
```
