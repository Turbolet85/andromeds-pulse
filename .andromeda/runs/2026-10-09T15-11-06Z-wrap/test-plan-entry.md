
## 2026-10-09-ci-on-linux-alone — §9 and its restatements: the CI runs on ubuntu-22.04 alone, no matrix, no `release` job
**Section:** §9 CI Integration → Pipeline structure (Lint + tests · Release build · A11y suite · Boot smoke · E2E tests rows) · §9 Matrix builds · §4 Coverage tool · §1 Coverage triggers (`multi-platform-compat`) · §6 Scenario P5 · §3 → Per-chunk gate discipline (the key file `registries/contracts/test-plan/per-chunk-gate-discipline.md`)
**Change:**
- Lint + tests row. Was: "matrix Linux/macOS/Windows", perf steps "on Linux also", Python 3 "on all three runners", "one job per OS". Now: one job on `ubuntu-22.04` (check name `lint / test (ubuntu-22.04)`), pinned by `ci_workflow_runs_on_linux_only`; the three perf steps unconditional; Python 3 on the runner; `lint-test-${{ runner.os }}` resolves to `lint-test-Linux` alone.
- Release build row. Was: the macOS/Windows `release` job, its `release-${{ runner.os }}` key and cache figures. Now: no `release` job; the release-profile builds are `supply-chain`'s `cargo auditable build --workspace --release` and `boot`'s `cargo build --workspace --release --features mcp-server`, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`; the cache readings live in architecture's CI/CD approach.
- A11y suite row: its own `a11y` job on `ubuntu-22.04` alone (was a matrix job on all three OSes).
- Boot smoke row: `ci-gates` runs in this job only; no job runs on macOS or Windows.
- E2E tests row, Parallel cell: n/a (was "matrix per surface (tauri-driver × 3 platforms)"); §6 P5's pointer to a "tauri-driver matrix" trimmed.
- Matrix builds: none (was a fenced three-system `matrix:` block); six jobs, each `runs-on: ubuntu-22.04`, no `strategy`, no `needs:`.
- §4 Coverage tool: runs in the `coverage` job on `ubuntu-22.04` (was "cross-platform on all 3 CI matrix runners").
- §1 `multi-platform-compat`: the "matrix over Windows/macOS/Linux CI runners" does not exist; the Windows and macOS arms have no CI witness and leave with the route entry `Other operating systems retired from the code` (P-113).
- Key file, Process-end witness form: the `cfg(unix)` arms run in CI `lint-test` on `ubuntu-22.04`, with no macOS CI witness (was "CI lint-test Linux/macOS").
**Why:** Chunk 2026-10-09-ci-on-linux-alone removed the Windows and macOS runners from the `ci` workflow. The Release build row keeps its name because the two new self-lint tests cite it.
**Kept:** The dated run measurements in the Lint + tests row. The A11y suite row's `a11y-violations-base` wording: found standing with no producer, not amended, its owner a route matter. The §1 Surfaces row's webview-engine note (the product's systems).
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
