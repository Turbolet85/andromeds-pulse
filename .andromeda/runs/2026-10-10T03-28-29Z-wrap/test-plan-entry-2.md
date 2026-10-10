
## 2026-10-10-boot-smoke-s-self-end-named-from-a-run — the witness arm's missing shell test, the series' unexercised path, the second shell-out
**Section:** §1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage`; §1 Coverage triggers → performance-budget: WebGPU canvas throughput; §4 Unit Test Strategy → Framework (Rust crates); §9 Pipeline structure → Lint + tests
**Change:**
- The trigger row widened: the sh `boot` verb's exit-witness arm ships with no committed shell-level test (the set arm driven by three local legs and CI, the refusal arm once by hand); the Rust side is pinned (14 reader and decision pins, seven controls on the built library, the member pin). `harness:boot-series`: 16 unit pins, one local leg, one CI run; its `timed-out` path never ran live; its verdict under-reads a boot that ends before ready (`ci#38019133294`: `ended` 2, `other` 4 for seven self-ends). Owed: an assertion per witness-arm branch, the before-ready boot recorded with its exit record and label, a live `timed-out` reading.
- The WebGPU row: the boot job's `ci-gates` frame line was conditioned on the smoke step passing; now on the smoke step and the series step both passing.
- §4 Framework: was "One unit binary shells out to a non-Rust runtime"; now two, the second the `xtask` unit binary building `scripts/exit-witness.c` with `cc` (Linux only; a missing `cc` fails the controls).
- §9 lint-test row: the workspace tests need `cc` on the runner beside Python 3; green on `ci#38019133294`.
**Why:** Each records what this chunk shipped untested or newly required. The before-ready undercount is owned by the route's closing entry, as its first repair.
**Ref:** .andromeda/runs/2026-10-10T03-28-29Z-wrap/
