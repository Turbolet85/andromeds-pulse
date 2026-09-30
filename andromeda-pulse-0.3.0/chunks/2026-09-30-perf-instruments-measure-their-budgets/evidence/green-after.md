# GREEN after — /implement run `2026-09-30T18-06-28Z`

Readings from the plan's listed entries (gate trail: `.andromeda/runs/2026-09-30T18-06-28Z-implement/`).

## A — the snapshot sample spans the whole generation

Entry 5, the in-process producer (`--profile perf-samples`): `1 test run: 1 passed` (36.8 s).

Entry 6, `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot` (exit 0):

```
perf-budget: graded 120 record(s) across 1 file(s)
perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
perf-budget: memory max 7552000 B <= 512000000 B (n=3, populated 3) PASS
perf-budget: snapshot p99 94.0 ms <= 500 ms (n=50) PASS
perf-budget: PASS
```

| | base `ea50ca2` | after |
|---|---|---|
| snapshot p99 (n=50) | 0.0 ms (formatting only) | **94.0 ms** |
| sample form | whole ms (`as_millis() as u64`) | fractional ms (`f64`) |
| distribution over the 50 samples | 50 × 0 ms | min 62.97 · median 67.99 · max 94.02 ms |

The after-distribution sits on the predecessor's hand measurement of a full load + curate + format (61–76 ms, debug
build), which is the equality the criterion needs: the graded value now spans load → curate → format. The hypothesis
"moves by orders of magnitude and stays well under 500 ms" is measured: from 0 to tens of ms, p99 at 18.8 % of the
budget (debug build; a release build would read lower).

## B — a frame-less log names its cause

Entry 6 (the producer log has no webview): `frame: cannot-evaluate: 0 samples, no adapter record in this log`.

Entry 7, `ci-gates` over a seeded sample-less data dir (exit 0):

```
ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
ci-gates: perf-budget NEUTRAL
```

NEUTRAL, never PASS. The fixed string `no WebGPU adapter in this run` appears in neither.

## C — the release cache saves on failure

Entry 8 prints `1`. Entry 10: `.github/workflows/ci.yml | 1 +` / `1 file changed, 1 insertion(+)`. Entry 9 (no unpinned
`uses:`, no secrets/permissions line added) exit 1, no output.

## Suite

Entry 3 (the new pins by name fragment): `19 tests run: 19 passed, 2466 skipped`. Entry 24 (workspace, default
features): `2485 tests run: 2485 passed, 0 skipped` (2466 at base + 19).
