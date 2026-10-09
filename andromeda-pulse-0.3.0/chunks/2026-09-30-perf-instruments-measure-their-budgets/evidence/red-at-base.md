# RED at base — `ea50ca2`

Copied from the P5 baseline runs recorded in `plan.md` `## Test Commands` (the `new = true` entries' `baseline`
keys, run on the untouched tree at phase time). Nothing else was run for RED (plan step 1).

| Entry | Base reading |
|---|---|
| `cargo nextest run --workspace --profile ci -E 'test(/frame_cause/) \| test(/generation_timer/) \| test(/webgpu_adapter/)'` | red — exit 4, `error: no tests to run` (none of the three name fragments exists; bindings unchanged after the run) |
| `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot` | exit 0 but `perf-budget: snapshot p99 0.0 ms <= 500 ms (n=50) PASS` and `perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run` over the untouched producer log (120 records, 1 file) — the formatting-only snapshot sample and the fixed, unevidenced frame cause |
| `ci-gates` over a seeded sample-less data dir | exit 0 but `ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run` |
| `grep -A2 'shared-key: release-' .github/workflows/ci.yml \| grep -c 'cache-on-failure: true'` | red — exit 1, `0` (the release step carries no `cache-on-failure`) |
| `cd pulse-app/ui && npx vitest run src/canvas/adapter-state.test.ts` | red — exit 1, `No test files found` |
| `grep -rh '"target":"ui.webgpu.adapter"' target/perf-frame/ \| grep -c '"outcome":"obtained"'` | red — exit 1, `0` (no such record anywhere) |
| `grep -c record_webgpu_adapter pulse-app/ui/src/bindings/index.ts` | red — exit 1, `0` |

Snapshot timing at base, as measured at the predecessor (`2026-09-30-perf-budget-gate-reads-real-samples`): the graded
`duration_ms` spans `format_markdown` only — 50 × 0 ms against 61–76 ms for a full load + curate + format (debug build).
