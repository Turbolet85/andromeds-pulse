
## 2026-10-09-ci-on-linux-alone — CI/CD approach: six jobs on Linux alone; the `release` job and both matrices leave
**Section:** §Infrastructure Patterns → CI/CD approach (the key file `registries/contracts/architecture/ci-cd-approach.md`) · §Occupied Resources → xtask CLI surfaces (`check:english-sources`)
**Change:**
- CI/CD approach, the `ci.yml` bullet. Was: seven independent jobs, `lint-test` and `a11y` on a Linux/macOS/Windows matrix, a macOS/Windows `release` job owning `release-{os}` with `cache-on-failure`, three perf steps "on Linux". Now: six jobs (`lint-test` · `mcp-test` · `a11y` · `boot` · `supply-chain` · `coverage`), each on `ubuntu-22.04`, no matrix, no `needs:` edge; the perf steps are unconditional; no CI job compiles or tests on Windows or macOS.
- The workspace release build keeps two CI witnesses, `supply-chain`'s `cargo auditable build --workspace --release` and `boot`'s `cargo build --workspace --release --features mcp-server`, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`; `ci_workflow_runs_on_linux_only` pins that the workflow names no other system.
- Cache keys: `lint-test-{os}` (owned by `lint-test`, restored read-only by `mcp-test` and `a11y`), `boot-Linux` (owned by `boot`, restored read-only by `supply-chain`), `coverage` (registry only). `{os}` resolves to `Linux` alone; no job writes `release-{os}` or a Windows or macOS `lint-test-{os}` key.
- A third dated cache reading beside the two of 2026-09-29/30: 2026-10-09T15:13:15Z, 10 entries, 12 208 662 121 B, which is 1 471 243 881 B over the 10 737 418 240 B cap; six entries (8 531 028 085 B) sit on keys with no writer; the four that stay sum 3 677 634 036 B.
- xtask CLI surfaces: the `check:english-sources` step is wired in `lint-test` on `ubuntu-22.04` (was "on all three OSes").
**Why:** Chunk 2026-10-09-ci-on-linux-alone removed the Windows and macOS runners from the `ci` workflow, the CI clause of P-113. The reading records the cache as measured and names no remedy: the wrap deletes nothing outside the tree, and who owns the six entries is a route matter.
**Kept:** The two dated cache measurements of 2026-09-29/30 with their "a watch" wording; the `release.yml` and `update-channels.yml` bullets (both workflows are unchanged); the dated Windows-runner facts in the xtask entry.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
