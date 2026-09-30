
## 2026-09-30-perf-budget-gate-reads-real-samples — the perf-budget gate grades real samples; frame gate on the dev host
**Section:** §1 perf-budget-instruments (frame, snapshot rows) · §5 snapshot metric row · §9 log artifact row · §10 Performance budgets (snapshot, frame, memory rows) · §10 CI gates (perf-budget, snapshot bullets) · §10 load-profile constraints · §11 SLO ban
**Change:**
- Was: the CI perf gate VACUOUS (`ci-gates` fed `perf-slo-check` a boot-smoke log with 0 samples), owner "Perf-budget gate reads real samples". Now: `xtask::perf_budget` (`cargo xtask perf:budget`) grades every `agent-latest.jsonl*` member in-process; FAIL on an arm over budget, a non-numeric graded field, or a `--require`d empty arm; never PASS over empty input. CI lint-test Linux runs the `perf_budget_samples` producer, then `--require memory,snapshot`; `ci-gates` grades with no arm required (NEUTRAL over no samples). The scripts are deleted.
- The p99 rule is ONE: nearest rank, the ⌈0.99·n⌉-th smallest (`(n * 99).div_ceil(100).max(1) - 1`); was a jq `floor(0.99·n)` index and an "assert max" bullet.
- Frame p99 ≤ 33 ms is graded by `perf:frame-sample` on a Windows GPU dev host, not CI; on CI the frame arm prints `frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run`. Was: "release build fails" on frame p99.
- Memory: CI-enforced via the required arm; the gauge is rows × 256 B, not RSS; no RSS gate.
- As measured: the snapshot `duration_ms` times `format_markdown` only, so the 500 ms bounds formatting, not generation; owner "Perf instruments measure what their budgets name".
- §9: `logs-perf-samples-${{ runner.os }}` added; no frame-log artifact.
**Why:** this chunk made the gate able to fail. The hosted Windows runner gave 0 frame samples with the app healthy, so the operator (founder-delegated) moved the frame gate to the dev host rather than retune. The snapshot timer scope is recorded as measured, its fix routed to the named entry. The relay directed one p99 rule.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
